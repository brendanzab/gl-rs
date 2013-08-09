use std::comm::{Port, stream};

mod xml;

pub struct Registry {
    enum_nss: ~[EnumNs],
    cmd_nss: ~[CmdNs],
}

impl Registry {
    pub fn from_xml(data: &str) -> Result<Registry, ~str> {
        RegistryBuilder::parse(data)
    }
}

pub struct EnumNs {
    ns: ~str,
    enums: ~[Enum],
}

pub struct Enum {
    ident: ~str,
    value: ~str,
}

pub struct ReturnType(Option<~str>);

pub struct CmdNs {
    ns: ~str,
    cmds: ~[Cmd],
}

pub struct Cmd {
    ident: ~str,
    ty: ReturnType,
    params: ~[Param]
}

pub struct Param {
    ptype: ~str,
    ident: ~str,
}

struct RegistryBuilder {
    priv port: Port<xml::ParseResult>,
}

macro_rules! ignore(
    ($name:expr) => (
        match self.ignore(~$name) {
            Err(err) => return Err(err), _ => (),
        }
    )
)

impl<'self> RegistryBuilder {
    fn new(port: Port<xml::ParseResult>) -> RegistryBuilder {
        RegistryBuilder { port: port }
    }

    fn recv(&self) -> Result<xml::Msg, ~str> {
        do self.port.recv().chain_err |err| {
            Err(fmt!("XML error: %s", err.to_str()))
        }
    }

    fn parse(data: &str) -> Result<Registry, ~str> {
        let (port, chan) = stream();
        let builder = RegistryBuilder::new(port);

        xml::parse(data, chan);
        match builder.recv() {
            Ok(xml::StartElement(~"registry", _)) => builder.consume_registry(),
            Ok(msg) => Err(fmt!("Expected <registry>, found: %?", msg.to_str())),
            Err(err) => Err(err),
        }
    }

    fn ignore(&self, name: ~str) -> Result<(), ~str> {
        loop {
            match self.recv() {
                Ok(xml::EndElement(ref n)) if *n == name => return Ok(()),
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }
    }

    fn consume_registry(&self) -> Result<Registry, ~str> {
        let mut registry = Registry { enum_nss: ~[], cmd_nss: ~[] };
        loop {
            match self.recv() {
                // ignores
                Ok(xml::Characters(_)) => (),
                Ok(xml::StartElement(~"comment", _)) => ignore!("comment"),
                Ok(xml::StartElement(~"types", _)) => ignore!("types"),
                Ok(xml::StartElement(~"groups", _)) => ignore!("groups"),
                Ok(xml::StartElement(~"feature", _)) => ignore!("feature"),
                Ok(xml::StartElement(~"extensions", _)) => ignore!("extensions"),

                // add enum namespace
                Ok(xml::StartElement(~"enums", ref atts)) => {
                    match atts.find(&~"namespace") {
                        Some(ns) => {
                            match self.consume_enum_ns(ns.clone()) {
                                Ok(enum_ns) => registry.enum_nss.push(enum_ns),
                                Err(err) => return Err(err),
                            }
                        }
                        _ => return Err(fmt!("Unexpected enum namespace attributes, found: %?", atts)),
                    }
                }
                // add command namespace
                Ok(xml::StartElement(~"commands", _)) => ignore!("commands"),

                // finished building the registry
                Ok(xml::EndElement(~"registry")) => return Ok(registry),
                // error handling
                Ok(msg) => return Err(fmt!("Expected </registry>, found: %?", msg.to_str())),
                Err(err) => return Err(err),
            }
        }
    }

    fn consume_enum_ns(&self, ns: ~str) -> Result<EnumNs, ~str> {
        let mut enum_ns = EnumNs { ns: ns, enums: ~[] };
        loop {
            match self.recv() {
                // ignores
                Ok(xml::Characters(_)) => (),
                Ok(xml::StartElement(~"unused", _)) => ignore!("unused"),

                // add enum definition
                Ok(xml::StartElement(~"enum", ref atts)) => {
                    match (atts.find(&~"name"), atts.find(&~"value")){
                        (Some(ident), Some(value)) => {
                            match self.consume_enum(ident.clone(), value.clone()) {
                                Ok(enm) => enum_ns.enums.push(enm),
                                Err(err) => return Err(err),
                            }
                        }
                        _ => return Err(fmt!("Unexpected enum attributes, found: %?", atts)),
                    }
                }

                // finished building the namespace
                Ok(xml::EndElement(~"enums")) => return Ok(enum_ns),
                // error handling
                Ok(msg) => return Err(fmt!("Expected </enums>, found: %?", msg.to_str())),
                Err(err) => return Err(err),
            }
        }
    }

    fn consume_enum(&self, ident: ~str, value: ~str) -> Result<Enum, ~str> {
        match self.recv() {
            Ok(xml::EndElement(~"enum")) => {
                Ok(Enum { ident: ident, value: value })
            }
            // error handling
            Ok(msg) => Err(fmt!("Expected </enum>, found: %?", msg.to_str())),
            Err(err) => Err(err),
        }
    }

    // TODO: Consume command namespaces
}