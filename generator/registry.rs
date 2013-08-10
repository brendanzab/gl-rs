use std::comm::{Port, stream};
use std::str::eq_slice;
use self::xml::{Characters, StartElement, EndElement};

mod xml;

pub struct Registry {
    enum_nss: ~[EnumNs],
    cmd_nss: ~[CmdNs],
}

impl Registry {
    pub fn from_xml(data: &str) -> Registry {
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

impl<'self> RegistryBuilder {
    fn new(port: Port<xml::ParseResult>) -> RegistryBuilder {
        RegistryBuilder { port: port }
    }

    fn recv(&self) -> xml::Msg {
        match self.port.recv() {
            Ok(msg) => msg,
            Err(err) => fail!("XML error: %s", err.to_str())
        }
    }

    fn parse(data: &str) -> Registry {
        let (port, chan) = stream();
        let builder = RegistryBuilder::new(port);

        xml::parse(data, chan);
        match builder.recv() {
            StartElement(~"registry", _) => builder.consume_registry(),
            msg => fail!("Expected <registry>, found: %?", msg.to_str()),
        }
    }

    fn ignore(&self, name: &str) {
        loop {
            match self.recv() {
                EndElement(ref n) if eq_slice(*n, name) => break,
                _ => (),
            }
        }
    }

    fn consume_registry(&self) -> Registry {
        let mut registry = Registry { enum_nss: ~[], cmd_nss: ~[] };
        loop {
            match self.recv() {
                // ignores
                Characters(_) => (),
                StartElement(~"comment", _) => self.ignore("comment"),
                StartElement(~"types", _) => self.ignore("types"),
                StartElement(~"groups", _) => self.ignore("groups"),
                StartElement(~"feature", _) => self.ignore("feature"),
                StartElement(~"extensions", _) => self.ignore("extensions"),

                // add enum namespace
                StartElement(~"enums", ref atts) => {
                    match atts.find(&~"namespace") {
                        Some(ns) => {
                            registry.enum_nss.push(
                                self.consume_enum_ns(ns.clone())
                            );
                        }
                        _ => fail!("Unexpected enum namespace attributes, found: %?", atts),
                    }
                }
                // add command namespace
                StartElement(~"commands", _) => self.ignore("commands"),

                // finished building the registry
                EndElement(~"registry") => break,

                // error handling
                msg => fail!("Expected </registry>, found: %?", msg.to_str()),
            }
        }
        registry
    }

    fn consume_enum_ns(&self, ns: ~str) -> EnumNs {
        let mut enum_ns = EnumNs { ns: ns, enums: ~[] };
        loop {
            match self.recv() {
                // ignores
                Characters(_) => (),
                StartElement(~"unused", _) => self.ignore("unused"),
                // add enum definition
                StartElement(~"enum", ref atts) => {
                    match (atts.find(&~"name"), atts.find(&~"value")){
                        (Some(ident), Some(value)) => {
                            enum_ns.enums.push(
                                self.consume_enum(ident.clone(), value.clone())
                            )
                        }
                        _ => fail!("Unexpected enum attributes, found: %?", atts),
                    }
                }
                // finished building the namespace
                EndElement(~"enums") => break,
                // error handling
                msg => fail!("Expected </enums>, found: %?", msg.to_str()),
            }
        }
        enum_ns
    }

    fn consume_enum(&self, ident: ~str, value: ~str) -> Enum {
        match self.recv() {
            EndElement(~"enum") => Enum { ident: ident, value: value },
            msg => fail!("Expected </enum>, found: %?", msg.to_str()),
        }
    }

    // TODO: Consume command namespaces
}