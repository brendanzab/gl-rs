
extern mod sax;

use self::sax::*;

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
    namespace: ~str,
    enums: ~[Enum],
    group: Option<~str>,
    ty: Option<~str>,
    start: Option<~str>,
    end: Option<~str>,
    vendor: Option<~str>,
    comment: Option<~str>,
}

pub struct Enum {
    ident: ~str,
    value: ~str,
}

pub struct ReturnType(Option<~str>);

pub struct CmdNs {
    namespace: ~str,
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
    priv port: SaxPort,
}

impl<'self> RegistryBuilder {
    fn recv(&self) -> ParseEvent {
        match self.port.recv() {
            Ok(msg) => msg,
            Err(err) => fail!("XML error: %s", err.to_str())
        }
    }

    fn parse(data: &str) -> Registry {
        let builder = RegistryBuilder {
            port: parse_xml(data)
        };
        match builder.recv() {
            StartDocument => (),
            msg => fail!("Expected start of document, found: %?", msg.to_str()),
        }
        match builder.recv() {
            StartElement(~"registry", _) => builder.consume_registry(),
            msg => fail!("Expected <registry>, found: %?", msg.to_str()),
        }
    }

    fn skip_until(&self, event: ParseEvent) {
        loop {
            let msg = self.recv();
            if msg == event || msg == EndDocument { break }
        }
    }

    fn consume_registry(&self) -> Registry {
        let mut registry = Registry { enum_nss: ~[], cmd_nss: ~[] };
        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) | IgnorableWhitespace(_) => (),
                StartElement(~"comment", _) => self.skip_until(EndElement(~"comment")),
                StartElement(~"types", _) => self.skip_until(EndElement(~"types")),
                StartElement(~"groups", _) => self.skip_until(EndElement(~"groups")),
                StartElement(~"feature", _) => self.skip_until(EndElement(~"feature")),
                StartElement(~"extensions", _) => self.skip_until(EndElement(~"extensions")),

                // add enum namespace
                StartElement(~"enums", ref atts) => {
                    registry.enum_nss.push(
                        EnumNs {
                            namespace:  atts.get_copy(&~"namespace"),
                            enums:      self.consume_enums(),
                            group:      atts.find_copy(&~"group"),
                            ty:         atts.find_copy(&~"type"),
                            start:      atts.find_copy(&~"start"),
                            end:        atts.find_copy(&~"end"),
                            vendor:     atts.find_copy(&~"vendor"),
                            comment:    atts.find_copy(&~"comment"),
                        }
                    )
                }
                // add command namespace
                StartElement(~"commands", _) => self.skip_until(EndElement(~"commands")),

                // finished building the registry
                EndElement(~"registry") => break,

                // error handling
                msg => fail!("Expected </registry>, found: %?", msg.to_str()),
            }
        }
        registry
    }

    fn consume_enums(&self) -> ~[Enum] {
        let mut enums = ~[];
        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) | IgnorableWhitespace(_) => (),
                StartElement(~"unused", _) => self.skip_until(EndElement(~"unused")),
                // add enum definition
                StartElement(~"enum", ref atts) => {
                    enums.push(
                        Enum {
                            ident:  atts.get_copy(&~"name"),
                            value:  atts.get_copy(&~"value"),
                        }
                    );
                    match self.recv() {
                        EndElement(~"enum") => (),
                        msg => fail!("Expected </enum>, found: %?", msg.to_str()),
                    }
                }
                // finished building the namespace
                EndElement(~"enums") => break,
                // error handling
                msg => fail!("Expected </enums>, found: %?", msg.to_str()),
            }
        }
        enums
    }

    // TODO: Consume command namespaces
}