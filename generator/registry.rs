
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
    group: Option<~str>,
    ty: Option<~str>,
    start: Option<~str>,
    end: Option<~str>,
    vendor: Option<~str>,
    comment: Option<~str>,
    enums: ~[Enum],
}

pub struct Enum {
    ident: ~str,
    value: ~str,
}

pub struct CmdNs {
    namespace: ~str,
    cmds: ~[Cmd],
}

pub struct Binding {
    ident: ~str,
    ty: ~str,
}

pub struct Cmd {
    proto: Binding,
    params: ~[Binding],
    alias: Option<~str>,
    vecequiv: Option<~str>,
    glx: Option<GlxOpcode>,
}

pub struct GlxOpcode {
    ty: ~str,
    opcode: ~str,
    name: Option<~str>,
    comment: Option<~str>,
}

struct RegistryBuilder {
    priv port: SaxPort,
}

impl<'self> RegistryBuilder {
    fn recv(&self) -> ParseEvent {
        loop {
            match self.port.recv() {
                Ok(StartDocument) => (),
                Ok(Comment(_)) => (),
                Ok(Characters(ref ch)) if ch.is_whitespace() => (),
                Ok(EndDocument) => fail!("The end of the document has been reached"),
                Ok(event) => return event,
                Err(err) => fail!("XML error: %s", err.to_str()),
            }
        }
    }

    fn parse(data: &str) -> Registry {
        RegistryBuilder {
            port: parse_xml(data)
        }.consume_registry()
    }

    fn expect_characters(&self) -> ~str {
        match self.recv() {
            Characters(ref ch) => ch.clone(),
            msg => fail!("Expected characters, found: %s", msg.to_str()),
        }
    }

    fn expect_start_element(&self, name: &str) -> Attributes {
        match self.recv() {
            StartElement(ref n, ref atts) if name == *n => atts.clone(),
            msg => fail!("Expected <%s>, found: %s", name, msg.to_str()),
        }
    }

    fn expect_end_element(&self, name: &str) {
        match self.recv() {
            EndElement(ref n) if name == *n => (),
            msg => fail!("Expected </%s>, found: %s", name, msg.to_str()),
        }
    }

    fn skip_until(&self, event: ParseEvent) {
        loop {
            let msg = self.recv();
            if msg == event || msg == EndDocument { break }
        }
    }

    fn consume_registry(&self) -> Registry {
        self.expect_start_element("registry");
        let mut registry = Registry { enum_nss: ~[], cmd_nss: ~[] };
        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) => (),
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
                            group:      atts.find_copy(&~"group"),
                            ty:         atts.find_copy(&~"type"),
                            start:      atts.find_copy(&~"start"),
                            end:        atts.find_copy(&~"end"),
                            vendor:     atts.find_copy(&~"vendor"),
                            comment:    atts.find_copy(&~"comment"),
                            enums:      self.consume_enums(),
                        }
                    )
                }

                // add command namespace
                StartElement(~"commands", ref atts) => {
                    registry.cmd_nss.push(
                        CmdNs {
                            namespace:  atts.get_copy(&~"namespace"),
                            cmds:       self.consume_cmds(),
                        }
                    );
                }

                // finished building the registry
                EndElement(~"registry") => break,

                // error handling
                msg => fail!("Expected </registry>, found: %s", msg.to_str()),
            }
        }
        registry
    }

    fn consume_enums(&self) -> ~[Enum] {
        let mut enums = ~[];
        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) => (),
                StartElement(~"unused", _) => self.skip_until(EndElement(~"unused")),

                // add enum definition
                StartElement(~"enum", ref atts) => {
                    enums.push(
                        Enum {
                            ident:  atts.get_copy(&~"name"),
                            value:  atts.get_copy(&~"value"),
                        }
                    );
                    self.expect_end_element("enum");
                }

                // finished building the namespace
                EndElement(~"enums") => break,
                // error handling
                msg => fail!("Expected </enums>, found: %s", msg.to_str()),
            }
        }
        enums
    }

    fn consume_cmds(&self) -> ~[Cmd] {
        let mut cmds = ~[];
        loop {
            match self.recv() {
                // add command definition
                StartElement(~"command", _) => {
                    cmds.push(self.consume_cmd());
                }
                // finished building the namespace
                EndElement(~"commands") => break,
                // error handling
                msg => fail!("Expected </commands>, found: %s", msg.to_str()),
            }
        }
        cmds
    }

    fn consume_cmd(&self) -> Cmd {
        // consume command prototype
        self.expect_start_element("proto");
        let proto = self.consume_binding();
        self.expect_end_element("proto");

        let mut params = ~[];
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;

        loop {
            match self.recv() {
                StartElement(~"param", _) => {
                    params.push(
                        self.consume_binding()
                    );
                    self.expect_end_element("param");
                }
                StartElement(~"alias", ref atts) => {
                    alias = atts.find_copy(&~"alias");
                    self.expect_end_element("alias");
                }
                StartElement(~"vecequiv", ref atts) => {
                    vecequiv = atts.find_copy(&~"vecequiv");
                    self.expect_end_element("vecequiv");
                }
                StartElement(~"glx", ref atts) => {
                    glx = Some(GlxOpcode {
                        ty:      atts.get_copy(&~"type"),
                        opcode:  atts.get_copy(&~"opcode"),
                        name:    atts.find_copy(&~"name"),
                        comment: atts.find_copy(&~"comment"),
                    });
                    self.expect_end_element("glx");
                }
                EndElement(~"command") => break,
                msg => fail!("Expected </command>, found: %s", msg.to_str()),
            }
        }
        Cmd {
            proto: proto,
            params: params,
            alias: alias,
            vecequiv: vecequiv,
            glx: glx,
        }
    }

    fn consume_binding(&self) -> Binding {
        // consume type
        let mut ty = ~"";
        loop {
            match self.recv() {
                Characters(ch) => ty.push_str(ch),
                StartElement(~"ptype", _) => (),
                EndElement(~"ptype") => (),
                StartElement(~"name", _) => break,
                msg => fail!("Expected binding, found: %s", msg.to_str()),
            }
        }
        let ident = self.expect_characters();
        // consume identifier
        self.expect_end_element("name");
        Binding {
            ident: ident,
            ty: ty,
        }
    }
}
