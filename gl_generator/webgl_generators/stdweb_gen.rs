use std::io;

use webgl_registry::*;

#[allow(missing_copy_implementations)]
pub struct StdwebGenerator;

fn write_header<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    writeln!(dest, r#"
// {registry:?}

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use self::stdweb::{{Reference, Value}};
use self::stdweb::private::*;
use self::stdweb::unstable::*;

type ConversionError = <Reference as TryFrom<Value>>::Error;
type HTMLCanvasElement = stdweb::web::html_element::CanvasElement;

// TODO: Implement stronger types for these
type ArrayBufferView = Reference;
type BufferSource = Reference;

fn custom_error(s: &str) -> ConversionError {{
    use self::serde::ser::Error;
    stdweb::serde::ConversionError::custom(s).into()
}}
    "#, registry=registry)?;
    Ok(())
}

impl super::Generator for StdwebGenerator {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
        where W: io::Write
    {
        write_header(registry, dest)?;
        write_typedefs(registry, dest)?;
        write_enums(registry, dest)?;
        write_dictionaries(registry, dest)?;
        write_interfaces(registry, dest)?;
        Ok(())
    }
}

fn write_typedefs<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    for (name, typedef) in &registry.typedefs {
        write_typedef(name, typedef, registry, dest)?;
    }
    Ok(())
}

fn write_typedef<W>(name: &str, typedef: &Typedef, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    writeln!(dest, r#"pub type {name} = {type_};"#, name=name, type_=typedef.type_.owned)?;
    Ok(())
}

fn write_enums<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    for (name, enum_) in &registry.enums {
        write_enum(name, enum_, registry, dest)?;
    }
    Ok(())
}

fn write_enum<W>(name: &str, enum_: &Enum, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    write!(dest, r#"
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum {name} {{
    "#, name=name)?;

    for variant in &enum_.variants {
        writeln!(dest, r#"
    #[serde(rename = "{raw_variant}")]
    {variant},"#,
        variant=camel(&variant),
        raw_variant=variant)?;
    }

    writeln!(dest, r#"
}}
js_deserializable!({name});
js_serializable!({name});
    "#, name=name)?;
    Ok(())
}

fn write_dictionaries<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    for (name, dictionary) in &registry.dictionaries {
        write_dictionary(name, dictionary, registry, dest)?;
    }
    Ok(())
}

fn write_dictionary<W>(name: &str, dictionary: &Dictionary, registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    if dictionary.is_hidden {
        return Ok(());
    }

    write!(dest, r#"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {name} {{
    "#, name=name)?;

    for (name, field) in dictionary.collect_fields(registry) {
        write_field(name, field, registry, dest)?;
    }

    writeln!(dest, r#"
}}
js_deserializable!({name});
js_serializable!({name});
    "#, name=name)?;
    Ok(())
}

fn write_field<W>(name: &str, field: &Field, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    let mut serde_attrs = Vec::new();
    let field_name = unreserve(snake(name));
    if field_name != name {
        serde_attrs.push(format!(r#"rename = "{}""#, name));
    }
    if field.type_.owned.starts_with("Option<") {
        serde_attrs.push(r#"default"#.into());
        serde_attrs.push(r#"skip_serializing_if = "Option::is_none""#.into());
    }

    if !serde_attrs.is_empty() {
        write!(dest, r#"
    #[serde({})]"#, serde_attrs.join(", "))?;
    }

    writeln!(dest, r#"
    {name}: {type_},"#, name=field_name, type_=field.type_.owned)?;

    Ok(())
}

fn write_interfaces<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    for (name, interface) in &registry.interfaces {
        write_interface(name, interface, registry, dest)?;
    }
    Ok(())
}

fn write_interface<W>(name: &str, interface: &Interface, registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    if interface.is_hidden {
        return Ok(());
    }

    write!(dest, r#"
#[derive(Debug, Clone)]
pub struct {name}(Reference);

impl FromReferenceUnchecked for {name} {{
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {{
        {name}(reference)
    }}
}}

impl FromReference for {name} {{
    #[inline]
    fn from_reference(reference: Reference) -> Option<Self> {{
        if {{
            __js_raw_asm!(
                "return (Module.STDWEB.acquire_js_reference( $0 ) instanceof \"{name}\") | 0;",
                reference.as_raw()
            ) == 1
        }} {{
            Some({name}(reference))
        }} else {{
            None
        }}
    }}
}}

impl AsRef<Reference> for {name} {{
    #[inline]
    fn as_ref(&self) -> &Reference {{
        &self.0
    }}
}}

impl From<{name}> for Reference {{
    #[inline]
    fn from(value: {name}) -> Self {{
        value.0
    }}
}}

impl TryFrom<{name}> for Reference {{
    type Error = Void;

    #[inline]
    fn try_from(value: {name}) -> Result<Self, Self::Error> {{
        Ok(value.0)
    }}
}}

impl<R: TryInto<Reference>> TryFrom<R> for {name}
    where <R as TryInto<Reference>>::Error: Into<ConversionError>
{{
    type Error = ConversionError;

    #[inline]
    fn try_from(value: R) -> Result<Self, Self::Error> {{
        value.try_into()
            .map_err(|error| error.into())
            .and_then(|reference: Reference| {{
                reference.downcast()
                    .ok_or_else(|| custom_error("reference is of a different type"))
            }})
    }}
}}

impl JsSerializable for {name} {{
    #[inline]
    fn into_js<'a>(&'a self, arena: &'a PreallocatedArena) -> SerializedValue<'a> {{
        self.0.into_js(arena)
    }}

    #[inline]
    fn memory_required(&self) -> usize {{
        Reference::memory_required(&self.0)
    }}
}}

__js_serializable_boilerplate!(() ({name}) ());

impl {name} {{
    "#, name=name)?;

    for (name, member) in interface.collect_members(registry, &VisitOptions::default()) {
        match member {
            &Member::Const(ref const_) => {
                write_const(name, const_, registry, dest)?;
            },
            &Member::Attribute(ref attribute) => {
                write_attribute(name, attribute, registry, dest)?;
            },
            &Member::Operation(ref operation) => {
                write_operation(name, operation, registry, dest)?;
            }
        }
    }

    writeln!(dest, r#"
}}
    "#)?;
    Ok(())
}

fn write_const<W>(name: &str, const_: &Const, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    write!(dest, r#"
    pub const {name}: {type_} = {value};"#,
    name=shouty_snake(name),
    type_=const_.type_.owned,
    value=const_.value
    )?;
    Ok(())
}

fn write_attribute<W>(name: &str, attribute: &Attribute, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    if attribute.getter {
        write!(dest, r#"
    pub fn {name}(&self) -> {type_} {{
        (js! {{ return @{{self}}.{raw_name}; }} ).try_into().unwrap()
    }}
        "#,
        name=unreserve(snake(name)),
        raw_name=name,
        type_=attribute.type_.owned,
        )?;
    }
    if attribute.setter {
        write!(dest, r#"
    pub fn set_{name}(&self, value: {type_}) {{
        js!( @{{self}}.{raw_name} = @{{value}}; );
    }}
        "#,
        name=snake(name),
        raw_name=name,
        type_=attribute.type_.borrowed,
        )?;
    }
    Ok(())
}

fn write_operation<W>(name: &str, operation: &Operation, _registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    let mut args: String = operation.args.iter().map(|a| {
        format!("{}: {}, ", unreserve(snake(&a.name)), a.type_.borrowed)
    }).collect();
    args.pop(); args.pop();
    let mut js_args: String = operation.args.iter().map(|a| {
        format!("@{{{}}}, ", unreserve(snake(&a.name)))
    }).collect();
    js_args.pop(); js_args.pop();

    if let Some(return_type) = operation.return_type.as_ref() {
        let conversion_type = if return_type.owned.starts_with("Option<") {
            "ok()"
        } else {
            "unwrap()"
        };

        write!(dest, r#"

    pub fn {name}(&self, {args}) -> {return_type} {{
        (js! {{ return @{{self}}.{raw_name}({js_args}); }} ).try_into().{conversion_type}
    }}"#,
        name=unreserve(snake(name)),
        raw_name=name,
        return_type=return_type.owned,
        args=args,
        js_args=js_args,
        conversion_type=conversion_type
        )?;
    } else {
        write!(dest, r#"

    pub fn {name}(&self, {args}) {{
        js!( @{{self}}.{raw_name}({js_args}); );
    }}"#,
        name=unreserve(snake(name)),
        raw_name=name,
        args=args,
        js_args=js_args
        )?;
    }
    Ok(())
}
