extern crate webidl;
extern crate heck;
extern crate khronos_api;
extern crate serde_xml_rs;
extern crate regex;

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

#[derive(Debug, Deserialize)]
#[serde(rename = "extension")]
struct ExtensionIDL {
    pub name: String,
    pub idl: String,
}

pub enum Exts<'a> {
    Include(&'a [&'a str]),
    Exclude(&'a [&'a str]),
}

impl<'a> Exts<'a> {
    pub const NONE: Exts<'a> = Exts::Include(&[]);
    pub const ALL: Exts<'a> = Exts::Exclude(&[]);

    fn enumerate(&self) -> Vec<ExtensionIDL> {
        use self::serde_xml_rs::deserialize;
        use self::regex::{Regex, RegexBuilder};

        // The Khronos IDL files are... not quite right, so let's fix them up!
        let enum_regex = Regex::new("([(, ])enum\\b").unwrap();
        let missing_semicolon_regex = RegexBuilder::new("^}$").multi_line(true).build().unwrap();
        let shared_callback_regex = Regex::new("\\bAcquireResourcesCallback\\b").unwrap();

        let mut result = Vec::new();
        for &ext_xml in khronos_api::WEBGL_EXT_XML {
            let mut ext: ExtensionIDL = deserialize(ext_xml).unwrap();
            if match self {
                &Exts::Include(names) => names.contains(&&*ext.name),
                &Exts::Exclude(names) => !names.contains(&&*ext.name)
            } {
                ext.idl = enum_regex.replace_all(&ext.idl, "${1}GLenum").into();
                ext.idl = missing_semicolon_regex.replace_all(&ext.idl, "};").into();
                ext.idl = shared_callback_regex.replace_all(&ext.idl, "AcquireSharedResourcesCallback").into();
                result.push(ext);
            }
        }

        result
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub optional: bool
}

impl Type {
    fn optional(&self) -> Self {
        Type {
            name: self.name.clone(),
            optional: true
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Type {
        Type {
            name: s.into(),
            optional: false
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Primitive {
    Bool, I8, U8, I16, U16, I32, U32, I64, U64, F32, F64,
}

impl Primitive {
    pub fn name(self) -> &'static str {
        use self::Primitive::*;
        match self {
            Bool => "bool",
            I8 => "i8", U8 => "u8",
            I16 => "i16", U16 => "u16",
            I32 => "i32", U32 => "u32",
            I64 => "i64", U64 => "u64",
            F32 => "f32", F64 => "f64",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    String,
    ArrayBuffer,
    ArrayBufferView,
    BufferSource,
    CanvasElement,
    TypedArray(Primitive),
    Sequence(Type),
    Union(Vec<Type>),
    Dictionary,
    Interface,
    Enum,
    Typedef(Type),
    Any,
    Object,
    Callback(Vec<Argument>, Option<Type>),
}

impl TypeKind {
    pub fn flatten<'a>(&'a self, registry: &'a Registry) -> &'a TypeKind {
        match self {
            &TypeKind::Typedef(ref t) if !t.optional => {
                registry.resolve_type(&t.name)
            },
            other => other
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Const {
    pub type_: Type,
    pub value: String,
}

#[derive(Debug, Eq, Clone)]
pub struct Argument {
    pub name: String,
    pub optional: bool,
    pub type_: Type,
    pub variadic: bool,
}

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    pub args: Vec<Argument>,
    pub return_type: Option<Type>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attribute {
    pub type_: Type,
    pub setter: bool,
    pub getter: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Member {
    Const(Const),
    Operation(Operation),
    Attribute(Attribute),
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub inherits: Option<String>,
    pub mixins: BTreeSet<String>,
    pub members: BTreeMap<String, Vec<Member>>,
    pub is_hidden: bool,
    pub has_class: bool,
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

fn multimap_insert<K: Ord, V: PartialEq>(m: &mut BTreeMap<K, Vec<V>>, key: K, value: V) {
    let v = m.entry(key).or_insert_with(Vec::new);
    if !v.contains(&value) {
        v.push(value);
    }
}

fn multimap_append<K: Ord + Clone, V: PartialEq>(m: &mut BTreeMap<K, Vec<V>>, other: BTreeMap<K, Vec<V>>) {
    for (k, v) in other {
        for value in v {
            multimap_insert(m, k.clone(), value);
        }
    }
}

impl Interface {
    pub fn collect_members<'a>(&'a self, registry: &'a Registry, options: &VisitOptions) -> BTreeMap<&'a str, Vec<&'a Member>> {
        let mut members = BTreeMap::new();

        // Mixins
        for mixin_name in &self.mixins {
            let mixin = registry.mixins.get(mixin_name).expect(mixin_name);
            if options.visit_mixins {
                multimap_append(&mut members, mixin.collect_members(registry, options));
            }
        }

        // Members
        for (name, ms) in &self.members {
            for member in ms {
                multimap_insert(&mut members, &**name, member);
            }
        }

        members
    }
}

#[derive(Debug)]
pub struct Mixin {
    members: BTreeMap<String, Vec<Member>>,
}

impl Mixin {
    pub fn collect_members<'a>(&'a self, _registry: &'a Registry, _options: &VisitOptions) -> BTreeMap<&'a str, Vec<&'a Member>> {
        let mut members = BTreeMap::new();

        // Members
        for (name, ms) in &self.members {
            for member in ms {
                multimap_insert(&mut members, &**name, member);
            }
        }

        members
    }
}

#[derive(Debug, Default)]
pub struct Registry {
    pub mixins: BTreeMap<String, Mixin>,
    pub interfaces: BTreeMap<String, Interface>,
    pub dictionaries: BTreeMap<String, Dictionary>,
    pub enums: BTreeMap<String, Enum>,
    pub types: BTreeMap<String, TypeKind>,
    pub extensions: BTreeSet<String>,
}

impl Registry {
    pub fn new(api: Api, exts: Exts) -> Registry {
        let mut result = Registry::default();

        for idl_const in api.idl_consts() {
            for def in parse_defs(idl_const) {
                result.load_definition(def);
            }
        }

        for ext in exts.enumerate() {
            result.extensions.insert(ext.name);
            for def in parse_defs(ext.idl.as_bytes()) {
                result.load_definition(def);
            }
        }

        for name in RENDERING_CONTEXTS.into_iter().rev() {
            if let Some(mut iface) = result.interfaces.get(name.1).cloned() {
                iface.rendering_context = None;
                result.interfaces.insert("GLContext".into(), iface);
                break;
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

    pub fn resolve_type(&self, name: &str) -> &TypeKind {
        self.types.get(name).expect(name)
    }

    pub fn write_bindings<W, G>(&self, generator: G, output: &mut W) -> io::Result<()>
        where G: Generator,
              W: io::Write
    {
        generator.write(&self, output)
    }

    fn load_const(&mut self, const_: ast::Const) -> Option<(String, Member)> {
        use self::ast::{ConstType, ConstValue};

        let type_ = ast::Type {
            extended_attributes: Vec::new(),
            kind: match const_.type_ {
                ConstType::Boolean => ast::TypeKind::Boolean,
                ConstType::Byte => ast::TypeKind::Byte,
                ConstType::Octet => ast::TypeKind::Octet,
                ConstType::RestrictedDouble => ast::TypeKind::RestrictedDouble,
                ConstType::UnrestrictedDouble => ast::TypeKind::UnrestrictedDouble,
                ConstType::RestrictedFloat => ast::TypeKind::RestrictedFloat,
                ConstType::UnrestrictedFloat => ast::TypeKind::UnrestrictedFloat,
                ConstType::SignedLong => ast::TypeKind::SignedLong,
                ConstType::UnsignedLong => ast::TypeKind::UnsignedLong,
                ConstType::SignedLongLong => ast::TypeKind::SignedLongLong,
                ConstType::UnsignedLongLong => ast::TypeKind::UnsignedLongLong,
                ConstType::SignedShort => ast::TypeKind::SignedShort,
                ConstType::UnsignedShort => ast::TypeKind::UnsignedShort,
                ConstType::Identifier(s) => ast::TypeKind::Identifier(s),
            },
            nullable: const_.nullable
        };

        Some((const_.name, Member::Const(Const {
            type_: self.load_type(type_),
            value: match const_.value {
                ConstValue::BooleanLiteral(b) => format!("{:?}", b),
                ConstValue::FloatLiteral(v) => format!("{:?}", v),
                ConstValue::IntegerLiteral(v) => format!("{:?}", v),
                ConstValue::Null => "None".into(),
            }
        })))
    }

    fn load_attribute(&mut self, attribute: ast::Attribute) -> Option<(String, Member)> {
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

    fn load_argument(&mut self, argument: ast::Argument) -> Argument {
        let type_ = self.load_type(*argument.type_);
        Argument {
            name: argument.name,
            optional: argument.optional,
            type_,
            variadic: argument.variadic,
        }
    }

    fn load_operation(&mut self, operation: ast::Operation) -> Option<(String, Member)> {
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

    fn load_mixin_member(&mut self, member: ast::MixinMember) -> Option<(String, Member)> {
        use self::ast::MixinMember::*;
        match member {
            Const(c) => self.load_const(c),
            Attribute(a) => self.load_attribute(a),
            Operation(o) => self.load_operation(o),
        }
    }

    fn load_mixin(&mut self, mixin: ast::NonPartialMixin) {
        let mut members = BTreeMap::new();
        for (name, member) in mixin.members.into_iter().flat_map(|m| {
            self.load_mixin_member(m)
        }) {
            multimap_insert(&mut members, name, member);
        }

        self.mixins.insert(mixin.name, Mixin { members });
    }

    fn load_interface_member(&mut self, member: ast::InterfaceMember) -> Option<(String, Member)> {
        use self::ast::InterfaceMember::*;
        match member {
            Const(c) => self.load_const(c),
            Attribute(a) => self.load_attribute(a),
            Operation(o) => self.load_operation(o),
            _ => None
        }
    }

    fn load_interface(&mut self, interface: ast::NonPartialInterface) {
        fn has_attr(attrs: &Vec<Box<ast::ExtendedAttribute>>, name: &str) -> bool {
            use self::ast::ExtendedAttribute::*;
            for attr in attrs {
                if let NoArguments(ref other) = **attr {
                    if let &ast::Other::Identifier(ref n) = other {
                        return n == name;
                    }
                }
            }
            false
        }

        let mut members = BTreeMap::new();
        for (name, member) in interface.members.into_iter().flat_map(|m| {
            self.load_interface_member(m)
        }) {
            multimap_insert(&mut members, name, member);
        }

        let mut result = Interface {
            inherits: interface.inherits,
            mixins: BTreeSet::new(),
            members,
            is_hidden: false,
            has_class: !has_attr(&interface.extended_attributes, "NoInterfaceObject"),
            rendering_context: None,
        };

        for &(context_id, context_interface) in RENDERING_CONTEXTS {
            if context_interface == interface.name {
                result.rendering_context = Some(context_id);
                break;
            }
        }

        self.load_type_kind(&interface.name, TypeKind::Interface);
        self.interfaces.insert(interface.name, result);
    }

    fn load_includes(&mut self, includes: ast::Includes) {
        if let Some(interface) = self.interfaces.get_mut(&includes.includer) {
            interface.mixins.insert(includes.includee);
        }
    }

    // Load a type kind into the registry under a given name, and return
    // a reference to it.
    fn load_type_kind(&mut self, name: &str, type_kind: TypeKind) -> Type {
        if !self.types.contains_key(name) {
            self.types.insert(name.into(), type_kind);
        }
        name.into()
    }

    // Convert an AST type kind into a type kind the generator can understand
    fn load_type_inner(&mut self, kind: ast::TypeKind) -> Type {
        use self::ast::TypeKind::*;

        let mut name = format!("{:?}", kind);

        let type_kind = match kind {
            // Primitives
            Boolean => TypeKind::Primitive(Primitive::Bool),
            Byte => TypeKind::Primitive(Primitive::I8),
            Octet => TypeKind::Primitive(Primitive::U8),
            SignedShort => TypeKind::Primitive(Primitive::I16),
            UnsignedShort => TypeKind::Primitive(Primitive::U16),
            SignedLong => TypeKind::Primitive(Primitive::I32),
            UnsignedLong => TypeKind::Primitive(Primitive::U32),
            SignedLongLong => TypeKind::Primitive(Primitive::I64),
            UnsignedLongLong => TypeKind::Primitive(Primitive::U64),
            RestrictedFloat | UnrestrictedFloat => TypeKind::Primitive(Primitive::F32),
            RestrictedDouble | UnrestrictedDouble => TypeKind::Primitive(Primitive::F64),

            // Strings
            DOMString | USVString => TypeKind::String,
            ByteString => unimplemented!(),

            // TypedArrays
            Int8Array => TypeKind::TypedArray(Primitive::I8),
            Uint8Array => TypeKind::TypedArray(Primitive::U8),
            Int16Array => TypeKind::TypedArray(Primitive::I16),
            Uint16Array => TypeKind::TypedArray(Primitive::U16),
            Int32Array => TypeKind::TypedArray(Primitive::I32),
            Uint32Array => TypeKind::TypedArray(Primitive::U32),
            Float32Array => TypeKind::TypedArray(Primitive::F32),
            Float64Array => TypeKind::TypedArray(Primitive::F64),

            // Sequence
            Sequence(inner) => TypeKind::Sequence(self.load_type(*inner)),

            // Identifier
            Identifier(s) => match &*s {
                "BufferSource" => TypeKind::BufferSource,
                "HTMLCanvasElement" => TypeKind::CanvasElement,
                "ArrayBufferView" => TypeKind::ArrayBufferView,
                other => { return other.into(); }
            },

            // Composite
            Union(inners) => {
                if inners.len() <= 2 {
                    TypeKind::Union(inners.into_iter().map(|inner| {
                        self.load_type(*inner)
                    }).collect())
                } else {
                    TypeKind::Any
                }
            },

            // Misc
            ArrayBuffer => TypeKind::ArrayBuffer,
            Object => TypeKind::Object,
            _ => TypeKind::Any,
        };

        if let TypeKind::Primitive(ref p) = type_kind {
            name = p.name().into();
        }

        self.load_type_kind(&name, type_kind)
    }

    // Convert an AST type into a type the generator can understand
    fn load_type(&mut self, t: ast::Type) -> Type {
        let mut type_ = self.load_type_inner(t.kind);
        if t.nullable {
            type_ = type_.optional();
        }
        type_
    }

    fn load_typedef(&mut self, typedef: ast::Typedef) {
        let type_ = self.load_type(*typedef.type_);
        self.load_type_kind(&typedef.name, TypeKind::Typedef(type_));
    }

    fn load_field(&mut self, field: ast::DictionaryMember) -> Option<(String, Field)> {
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

        self.load_type_kind(&dictionary.name, TypeKind::Dictionary);
        self.dictionaries.insert(dictionary.name, Dictionary {
            inherits: dictionary.inherits,
            fields,
            is_hidden: false,
        });
    }

    fn load_enum(&mut self, enum_: ast::Enum) {
        let variants = enum_.variants.into_iter().collect();
        self.load_type_kind(&enum_.name, TypeKind::Enum);
        self.enums.insert(enum_.name, Enum { variants });
    }

    fn load_callback(&mut self, callback: ast::Callback) {
        use self::ast::ReturnType;
        let args = callback.arguments.into_iter().map(|a| self.load_argument(a)).collect();
        let return_type = match callback.return_type {
            ReturnType::NonVoid(t) => Some(self.load_type(*t)),
            ReturnType::Void => None,
        };
        self.load_type_kind(&callback.name, TypeKind::Callback(args, return_type));
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
            Callback(c) => self.load_callback(c),
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
