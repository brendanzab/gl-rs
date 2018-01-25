extern crate webidl;
extern crate heck;
extern crate khronos_api;

use std::{str, fmt, io};
use std::collections::{BTreeMap, BTreeSet};

use self::webidl::ast;
use webgl_generators::Generator;


const HIDDEN_NAMES: &'static [&'static str] = &["WebGLObject", "WebGLContextEventInit"];
const RESERVED_WORDS: &'static [&'static str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "Self", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "abstract", "alignof", "become", "box", "do", "final", "macro", "offsetof",
    "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual", "yield"
];
const RENDERING_CONTEXTS: &'static [(&'static str, &'static str)] = &[
    ("webgl",  "WebGLRenderingContext"),
    ("webgl2", "WebGL2RenderingContext")
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Api {
    WebGl,
    WebGl2,
}

impl Api {
    fn idl_consts(&self) -> &'static [&'static [u8]] {
        match *self {
            Api::WebGl => &[khronos_api::WEBGL_IDL],
            Api::WebGl2 => &[khronos_api::WEBGL_IDL, khronos_api::WEBGL2_IDL],
        }
    }
}

impl fmt::Display for Api {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Api::WebGl => write!(fmt, "webgl"),
            Api::WebGl2 => write!(fmt, "webgl2"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub owned: String,
    pub borrowed: String,
    pub optional: bool
}

impl Type {
    fn optional(&self) -> Self {
        if self.optional {
            self.clone()
        } else {
            Type {
                owned: format!("Option<{}>", self.owned),
                borrowed: format!("Option<{}>", self.borrowed),
                optional: true
            }
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Type {
        Type {
            owned: s.into(),
            borrowed: s.into(),
            optional: false
        }
    }
}

#[derive(Debug)]
pub struct Const {
    pub type_: Type,
    pub value: String,
}

#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub optional: bool,
    pub type_: Type,
    pub variadic: bool,
}

#[derive(Debug)]
pub struct Operation {
    pub args: Vec<Argument>,
    pub return_type: Option<Type>
}

#[derive(Debug)]
pub struct Attribute {
    pub type_: Type,
    pub setter: bool,
    pub getter: bool,
}

#[derive(Debug)]
pub struct Typedef {
    pub type_: Type,
}

#[derive(Debug)]
pub enum Member {
    Const(Const),
    Operation(Operation),
    Attribute(Attribute),
}

#[derive(Debug)]
pub struct Interface {
    pub inherits: Option<String>,
    pub mixins: BTreeSet<String>,
    pub members: BTreeMap<String, Member>,
    pub is_hidden: bool,
    pub rendering_context: Option<&'static str>,
}

#[derive(Debug)]
pub struct Field {
    pub type_: Type,
}

#[derive(Debug)]
pub struct Dictionary {
    pub inherits: Option<String>,
    pub fields: BTreeMap<String, Field>,
    pub is_hidden: bool,
}

#[derive(Debug)]
pub struct Enum {
    pub variants: BTreeSet<String>,
}

#[derive(Debug)]
pub struct VisitOptions {
    pub visit_mixins: bool,
}

impl Default for VisitOptions {
    fn default() -> Self {
        VisitOptions {
            visit_mixins: true,
        }
    }
}

pub fn unreserve<S: Into<String>>(name: S) -> String {
    let mut s = name.into();
    if RESERVED_WORDS.contains(&&*s) {
        s += "_";
    }
    s
}

pub fn snake(name: &str) -> String {
    use self::heck::SnakeCase;
    name.to_snake_case()
}

pub fn shouty_snake(name: &str) -> String {
    use self::heck::ShoutySnakeCase;
    name.to_shouty_snake_case()
}

pub fn camel(name: &str) -> String {
    use self::heck::CamelCase;
    name.to_camel_case()
}

impl Dictionary {
    pub fn collect_fields<'a>(&'a self, registry: &'a Registry) -> BTreeMap<&'a str, &'a Field> {
        let mut fields = BTreeMap::new();

        // Inherits
        if let Some(inherit_name) = self.inherits.as_ref() {
            let inherit = registry.dictionaries.get(inherit_name).expect(inherit_name);
            fields.append(&mut inherit.collect_fields(registry));
        }

        // Fields
        for (name, field) in &self.fields {
            fields.insert(name, field);
        }

        fields
    }
}

impl Interface {
    pub fn collect_members<'a>(&'a self, registry: &'a Registry, options: &VisitOptions) -> BTreeMap<&'a str, &'a Member> {
        let mut members = BTreeMap::new();

        // Mixins
        for mixin_name in &self.mixins {
            let mixin = registry.mixins.get(mixin_name).expect(mixin_name);
            if options.visit_mixins {
                members.append(&mut mixin.collect_members(registry, options));
            }
        }

        // Members
        for (name, member) in &self.members {
            members.insert(name, member);
        }

        members
    }
}

#[derive(Debug)]
pub struct Mixin {
    members: BTreeMap<String, Member>,
}

impl Mixin {
    pub fn collect_members<'a>(&'a self, _registry: &'a Registry, _options: &VisitOptions) -> BTreeMap<&'a str, &'a Member> {
        let mut members = BTreeMap::new();

        // Members
        for (name, member) in &self.members {
            members.insert(&**name, member);
        }

        members
    }
}

#[derive(Debug, Default)]
pub struct Registry {
    pub mixins: BTreeMap<String, Mixin>,
    pub interfaces: BTreeMap<String, Interface>,
    pub typedefs: BTreeMap<String, Typedef>,
    pub dictionaries: BTreeMap<String, Dictionary>,
    pub enums: BTreeMap<String, Enum>,
}

impl Registry {
    pub fn new(api: Api) -> Registry {
        let mut result = Registry::default();

        for idl_const in api.idl_consts() {
            for def in parse_defs(idl_const) {
                result.load_definition(def);
            }
        }

        for &hidden_name in HIDDEN_NAMES {
            if let Some(interface) = result.interfaces.get_mut(hidden_name) {
                interface.is_hidden = true;
            }
            if let Some(dictionary) = result.dictionaries.get_mut(hidden_name) {
                dictionary.is_hidden = true;
            }
        }

        result
    }

    pub fn write_bindings<W, G>(&self, generator: G, output: &mut W) -> io::Result<()>
        where G: Generator,
              W: io::Write
    {
        generator.write(&self, output)
    }

    fn load_const(&self, const_: ast::Const) -> Option<(String, Member)> {
        use self::ast::{ConstType, ConstValue};

        let mut inner_type = match const_.type_ {
            ConstType::Boolean => "bool".into(),
            ConstType::Byte => "i8".into(),
            ConstType::Octet => "u8".into(),
            ConstType::RestrictedDouble | ConstType::UnrestrictedDouble => "f64".into(),
            ConstType::RestrictedFloat | ConstType::UnrestrictedFloat => "f32".into(),
            ConstType::SignedLong => "i32".into(),
            ConstType::UnsignedLong => "u32".into(),
            ConstType::SignedLongLong => "i32".into(),
            ConstType::UnsignedLongLong => "u32".into(),
            ConstType::SignedShort => "i16".into(),
            ConstType::UnsignedShort => "u16".into(),
            ConstType::Identifier(s) => s,
        };

        if const_.nullable {
            inner_type = format!("Option<{}>", inner_type);
        }

        Some((const_.name, Member::Const(Const {
            type_: (*inner_type).into(),
            value: match const_.value {
                ConstValue::BooleanLiteral(b) => format!("{:?}", b),
                ConstValue::FloatLiteral(v) => format!("{:?}", v),
                ConstValue::IntegerLiteral(v) => format!("{:?}", v),
                ConstValue::Null => "None".into(),
            }
        })))
    }

    fn load_attribute(&self, attribute: ast::Attribute) -> Option<(String, Member)> {
        use self::ast::Attribute::*;
        match attribute {
            Regular(a) => {
                let type_ = self.load_type(*a.type_);
                Some((a.name, Member::Attribute(Attribute {
                    type_,
                    setter: !a.read_only,
                    getter: !a.inherits,
                })))
            },
            _ => None
        }
    }

    fn load_argument(&self, argument: ast::Argument) -> Argument {
        let type_ = self.load_type(*argument.type_);
        Argument {
            name: argument.name,
            optional: argument.optional,
            type_,
            variadic: argument.variadic,
        }
    }

    fn load_operation(&self, operation: ast::Operation) -> Option<(String, Member)> {
        use self::ast::Operation::*;
        use self::ast::ReturnType;
        match operation {
            Regular(o) => if let Some(name) = o.name {
                Some((name, Member::Operation(Operation {
                    args: o.arguments.into_iter().map(|a| self.load_argument(a)).collect(),
                    return_type: match o.return_type {
                        ReturnType::NonVoid(t) => Some(self.load_type(*t)),
                        ReturnType::Void => None,
                    }
                })))
            } else { None },
            _ => None
        }
    }

    fn load_mixin_member(&self, member: ast::MixinMember) -> Option<(String, Member)> {
        use self::ast::MixinMember::*;
        match member {
            Const(c) => self.load_const(c),
            Attribute(a) => self.load_attribute(a),
            Operation(o) => self.load_operation(o),
        }
    }

    fn load_mixin(&mut self, mixin: ast::NonPartialMixin) {
        let members = mixin.members.into_iter().flat_map(|m| {
            self.load_mixin_member(m)
        }).collect();

        self.mixins.insert(mixin.name, Mixin { members });
    }

    fn load_interface_member(&self, member: ast::InterfaceMember) -> Option<(String, Member)> {
        use self::ast::InterfaceMember::*;
        match member {
            Const(c) => self.load_const(c),
            Attribute(a) => self.load_attribute(a),
            Operation(o) => self.load_operation(o),
            _ => None
        }
    }

    fn load_interface(&mut self, interface: ast::NonPartialInterface) {
        let members = interface.members.into_iter().flat_map(|m| {
            self.load_interface_member(m)
        }).collect();

        let mut result = Interface {
            inherits: interface.inherits,
            mixins: BTreeSet::new(),
            members,
            is_hidden: false,
            rendering_context: None,
        };

        for &(context_id, context_interface) in RENDERING_CONTEXTS {
            if context_interface == interface.name {
                result.rendering_context = Some(context_id);
                break;
            }
        }

        self.interfaces.insert(interface.name, result);
    }

    fn load_includes(&mut self, includes: ast::Includes) {
        if let Some(interface) = self.interfaces.get_mut(&includes.includer) {
            interface.mixins.insert(includes.includee);
        }
    }

    fn load_type(&self, t: ast::Type) -> Type {
        use self::ast::TypeKind;
        fn obj_type(s: &str) -> Type {
            Type { owned: s.into(), borrowed: format!("&{}", s), optional: s == "Value" }
        }

        let mut type_ = match t.kind {
            TypeKind::Any => obj_type("Value"),
            TypeKind::ArrayBuffer => obj_type("ArrayBuffer"),
            TypeKind::Boolean => "bool".into(),
            TypeKind::Byte => "i8".into(),
            TypeKind::ByteString => Type {
                owned: "Vec<u8>".into(), borrowed: "&[u8]".into(), optional: false
            },
            TypeKind::DOMString | TypeKind::USVString => Type {
                owned: "String".into(), borrowed: "&str".into(), optional: false
            },
            TypeKind::Float32Array => obj_type("Float32Array"),
            TypeKind::Float64Array => obj_type("Float64Array"),
            TypeKind::Identifier(s) => (*s).into(),
            TypeKind::Int16Array => obj_type("Int16Array"),
            TypeKind::Int32Array => obj_type("Int32Array"),
            TypeKind::Int8Array => obj_type("Int8Array"),
            TypeKind::Octet => "u8".into(),
            TypeKind::Object => obj_type("Reference"),
            TypeKind::RestrictedDouble | TypeKind::UnrestrictedDouble => "f64".into(),
            TypeKind::RestrictedFloat | TypeKind::UnrestrictedFloat => "f32".into(),
            TypeKind::Sequence(inner) => {
                let t = self.load_type(*inner);
                Type {
                    owned: format!("Vec<{}>", t.owned),
                    borrowed: format!("&[{}]", t.borrowed),
                    optional: false
                }
            },
            TypeKind::SignedLong => "i32".into(),
            TypeKind::SignedLongLong => "i32".into(),
            TypeKind::SignedShort => "i16".into(),
            TypeKind::Uint16Array => obj_type("Uint16Array"),
            TypeKind::Uint32Array => obj_type("Uint32Array"),
            TypeKind::Uint8Array => obj_type("Uint8Array"),
            TypeKind::UnsignedLong => "u32".into(),
            TypeKind::UnsignedLongLong => "u32".into(),
            TypeKind::UnsignedShort => "u16".into(),
            _ => obj_type("Value")
        };
        if t.nullable {
            type_ = type_.optional();
        }
        type_
    }

    fn load_typedef(&mut self, typedef: ast::Typedef) {
        let type_ = self.load_type(*typedef.type_);
        self.typedefs.insert(typedef.name, Typedef {
            type_,
        });
    }

    fn load_field(&self, field: ast::DictionaryMember) -> Option<(String, Field)> {
        let mut type_ = self.load_type(*field.type_);
        if !field.required {
            type_ = type_.optional();
        }
        Some((field.name, Field {
            type_
        }))
    }

    fn load_dictionary(&mut self, dictionary: ast::NonPartialDictionary) {
        let fields = dictionary.members.into_iter().flat_map(|m| {
            self.load_field(m)
        }).collect();

        self.dictionaries.insert(dictionary.name, Dictionary {
            inherits: dictionary.inherits,
            fields,
            is_hidden: false,
        });
    }

    fn load_enum(&mut self, enum_: ast::Enum) {
        let variants = enum_.variants.into_iter().collect();
        self.enums.insert(enum_.name, Enum { variants });
    }

    fn load_definition(&mut self, def: ast::Definition) {
        use self::ast::Definition::*;
        match def {
            Mixin(ast::Mixin::NonPartial(m)) => self.load_mixin(m),
            Interface(ast::Interface::NonPartial(i)) => self.load_interface(i),
            Includes(i) => self.load_includes(i),
            Typedef(t) => self.load_typedef(t),
            Dictionary(ast::Dictionary::NonPartial(d)) => self.load_dictionary(d),
            Enum(e) => self.load_enum(e),
            _ => {}
        }
    }
}

fn parse_defs(src: &[u8]) -> Vec<ast::Definition> {
    let src = str::from_utf8(src)
        .expect("IDL contained invalid UTF-8");

    let parser = webidl::Parser::new();
    parser.parse_string(src)
        .expect("Failed to parse IDL")
}
