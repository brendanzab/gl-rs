// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern mod sax;

use extra::treemap::TreeSet;
use std::hashmap::HashSet;
use self::sax::*;

pub enum Ns { Gl, Glx, Wgl }

impl FromStr for Ns {
    fn from_str(s: &str) -> Option<Ns> {
        match s {
            "gl"  => Some(Gl),
            "glx" => Some(Glx),
            "wgl" => Some(Wgl),
            _     => None,
        }
    }
}

impl ToStr for Ns {
    fn to_str(&self) -> ~str {
        match *self {
            Gl  => ~"gl",
            Glx => ~"glx",
            Wgl => ~"wgl",
        }
    }
}

fn trim_str<'a>(s: &'a str, trim: &str) -> &'a str {
    if s.starts_with(trim) { s.slice_from(trim.len()) } else { s }
}

fn trim_enum_prefix<'a>(ident: &'a str, ns: Ns) -> &'a str {
    match ns {
        Gl => trim_str(ident, "GL_"),
        Glx => trim_str(ident, "GLX_"),
        Wgl =>  trim_str(ident, "WGL_"),
    }
}

fn trim_cmd_prefix<'a>(ident: &'a str, ns: Ns) -> &'a str {
    match ns {
        Gl => trim_str(ident, "gl"),
        Glx => trim_str(ident, "glx"),
        Wgl =>  trim_str(ident, "wgl"),
    }
}

pub struct Registry {
    groups: ~[Group],
    enums: ~[Enum],
    cmds: ~[Cmd],
    features: ~[Feature],
    extensions: ~[Extension],
}

impl Registry {
    /// Generate a registry from the supplied XML string
    pub fn from_xml(data: &str, ns: Ns, filter: Option<Filter>) -> Registry {
        RegistryBuilder::parse(data, ns, filter)
    }

    /// Returns a set of all the types used in the supplied registry. This is useful
    /// for working out what conversions are needed for the specific registry.
    pub fn get_tys(&self) -> TreeSet<~str> {
        let mut tys = TreeSet::new();
        for def in self.cmds.iter() {
            tys.insert(def.proto.ty.clone());
            for param in def.params.iter() {
                tys.insert(param.ty.clone());
            }
        }
        tys
    }
}

pub struct Group {
    name: ~str,
    enums: ~[~str],
}

pub struct EnumNs {
    namespace: ~str,
    group: Option<~str>,
    ty: Option<~str>,
    start: Option<~str>,
    end: Option<~str>,
    vendor: Option<~str>,
    comment: Option<~str>,
    defs: ~[Enum],
}

pub struct Enum {
    ident: ~str,
    value: ~str,
    alias: Option<~str>,
}

pub struct CmdNs {
    namespace: ~str,
    defs: ~[Cmd],
}

pub struct Binding {
    ident: ~str,
    ty: ~str,
    group: Option<~str>,
}

pub struct Cmd {
    proto: Binding,
    params: ~[Binding],
    is_safe: bool,
    alias: Option<~str>,
    vecequiv: Option<~str>,
    glx: Option<GlxOpcode>,
}

#[deriving(Clone)]
pub struct Feature {
    api: ~str,
    name: ~str,
    number: ~str,
    requires: ~[Require],
    removes: ~[Remove],
}

#[deriving(Clone)]
pub struct Require {
    comment: Option<~str>,
    /// A reference to the earlier types, by name
    enums: ~[~str],
    /// A reference to the earlier types, by name
    commands: ~[~str],
}

#[deriving(Clone)]
pub struct Remove {
    // always core, for now
    profile: ~str,
    comment: ~str,
    /// A reference to the earlier types, by name
    enums: ~[~str],
    /// A reference to the earlier types, by name
    commands: ~[~str],
}

#[deriving(Clone)]
pub struct Extension {
    name: ~str,
    /// which apis this extension is defined for (see Feature.api)
    supported: ~[~str],
    requires: ~[Require],
}

pub struct GlxOpcode {
    ty: ~str,
    opcode: ~str,
    name: Option<~str>,
    comment: Option<~str>,
}

struct RegistryBuilder {
    ns: Ns,
    filter: Option<Filter>,
    port: SaxPort,
}

pub struct Filter {
    extensions: ~[~str],
    profile: ~str,
    version: ~str,
    api: ~str,
}

/// A big, ugly, imperative impl with methods that accumulates a Registry struct
impl<'self> RegistryBuilder {
    fn parse(data: &str, ns: Ns, filter: Option<Filter>) -> Registry {
        RegistryBuilder {
            ns: ns,
            filter: filter,
            port: parse_xml(data),
        }.consume_registry()
    }

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
            match self.recv() {
                EndDocument => fail!("Expected %s, but reached the end of the document.",
                                     event.to_str()),
                ref msg if *msg == event => break,
                _ => (),
            }
        }
    }

    fn consume_registry(&self) -> Registry {
        self.expect_start_element("registry");
        let mut registry = Registry {
            groups: ~[],
            enums: ~[],
            cmds: ~[],
            features: ~[],
            extensions: ~[],
        };

        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) => (),
                StartElement(~"comment", _) => self.skip_until(EndElement(~"comment")),
                StartElement(~"types", _) => self.skip_until(EndElement(~"types")),

                // add groups
                StartElement(~"groups", _) => {
                    loop {
                        match self.recv() {
                            StartElement(~"group", ref atts) => {
                                registry.groups.push(
                                    self.consume_group(atts.get_clone("name"))
                                );
                            }
                            EndElement(~"groups") => break,
                            msg => fail!("Expected </groups>, found: %s", msg.to_str()),
                        }
                    }
                }

                // add enum namespace
                StartElement(~"enums", _) => {
                    registry.enums.extend(&mut self.consume_enums().move_iter())
                }

                // add command namespace
                StartElement(~"commands", _) => {
                    registry.cmds.extend(&mut self.consume_cmds().move_iter());
                }

                StartElement(~"feature", ref atts) => {
                    debug2!("Parsing feature: {:?}", atts);
                    registry.features.push(FromXML::convert(self, atts));
                }

                StartElement(~"extensions", _) => {
                    loop {
                        match self.recv() {
                            StartElement(~"extension", ref atts) => {
                                registry.extensions.push(FromXML::convert(self, atts));
                            }
                            EndElement(~"extensions") => break,
                            msg => fail2!("Unexpected message {}", msg.to_str()),
                        }
                    }
                }

                // finished building the registry
                EndElement(~"registry") => break,

                // error handling
                msg => fail!("Expected </registry>, found: %s", msg.to_str()),
            }
        }

        match self.filter {
            Some(ref filter) => {
                let Registry {
                    groups, enums, cmds, features: feats, extensions: exts
                } = registry;

                let mut desired_enums = HashSet::new();
                let mut desired_cmds = HashSet::new();

                // find the features we want
                let mut found_feat = false;
                for f in feats.iter() {
                    // XXX: verify that the string comparison with <= actually works as desired
                    if f.api == filter.api && f.number <= filter.version {
                        for req in f.requires.iter() {
                            desired_enums.extend(&mut req.enums.iter().map(|x| x.clone()));
                            desired_cmds.extend(&mut req.commands.iter().map(|x| x.clone()));
                        }
                    }
                    if f.number == filter.version {
                        found_feat = true;
                    }
                }

                // remove the things that should be removed
                for f in feats.iter() {
                    // XXX: verify that the string comparison with <= actually works as desired
                    if f.api == filter.api && f.number <= filter.version {
                        for rem in f.removes.iter() {
                            if rem.profile == filter.profile {
                                for enm in rem.enums.iter() {
                                    debug2!("Removing {:?}", enm);
                                    desired_enums.remove(enm);
                                }
                                for cmd in rem.commands.iter() {
                                    debug2!("Removing {:?}", cmd);
                                    desired_cmds.remove(cmd);
                                }
                            }
                        }
                    }
                }

                if !found_feat {
                    fail2!("Did not find version {} in the registry", filter.version);
                }

                for ext in exts.iter() {
                    if filter.extensions.iter().any(|x| x == &ext.name) {
                        if !ext.supported.iter().any(|x| x == &filter.api) {
                            fail2!("Requested {}, which doesn't support the {} API", ext.name, filter.api);
                        }
                        for req in ext.requires.iter() {
                            desired_enums.extend(&mut req.enums.iter().map(|x| x.clone()));
                            desired_cmds.extend(&mut req.commands.iter().map(|x| x.clone()));
                        }
                    }
                }

                Registry {
                    groups: groups,
                    enums: enums.move_iter().filter(|e| desired_enums.contains(&(~"GL_" + e.ident))).to_owned_vec(),
                    cmds: cmds.move_iter().filter(|c| desired_cmds.contains(&(~"gl" + c.proto.ident))).to_owned_vec(),
                    // these aren't important after this step
                    features: ~[],
                    extensions: ~[],
                }
            },
            None => registry
        }
    }

    fn consume_two<'a, T: FromXML, U: FromXML>(&self, one: &'a str, two: &'a str, end: &'a str) -> (~[T], ~[U]) {
        debug2!("consume_two: looking for {:s} and {:s} until {:s}", one, two, end);

        let mut ones = ~[];
        let mut twos = ~[];

        loop {
            match self.recv() {
                StartElement(ref name, ref atts) => {
                    debug2!("Found start element <{:?} {:?}>", name, atts);
                    debug2!("one and two are {:?} and {:?}", one, two);

                    let n = name.clone();

                    if one == n {
                        ones.push(FromXML::convert(self, atts));
                    } else if "type" == n {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if two == n {
                        twos.push(FromXML::convert(self, atts));
                    } else {
                        fail2!("Unexpected element: <{:?} {:?}>", n, atts);
                    }
                },
                EndElement(name) => {
                    debug2!("Found end element </{:?}>", name);

                    if (&[one, two]).iter().any(|&x| x == name) {
                        continue;
                    } else if "type" == name {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if end == name {
                        return (ones, twos);
                    } else {
                        fail2!("Unexpected end element {}", name);
                    }
                },
                msg => fail2!("Unexpected message {}", msg.to_str()) }
        }
    }

    fn consume_group(&self, name: ~str) -> Group {
        let mut enms = ~[];
        loop {
            match self.recv() {
                StartElement(~"enum", ref atts) => {
                    enms.push(atts.get_clone("name"));
                    self.expect_end_element("enum");
                }
                EndElement(~"group") => break,
                msg => fail!("Expected </group>, found: %s", msg.to_str()),
            }
        }
        Group {
            name: name,
            enums: enms,
        }
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
                            ident:  trim_enum_prefix(atts.get("name"), self.ns).to_owned(),
                            value:  atts.get_clone("value"),
                            alias:  atts.find_clone("alias"),
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
        let proto_atts = self.expect_start_element("proto");
        let mut proto = self.consume_binding(proto_atts.find_clone("group"));
        proto.ident = trim_cmd_prefix(proto.ident, self.ns).to_owned();
        self.expect_end_element("proto");

        let mut params = ~[];
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;
        loop {
            match self.recv() {
                StartElement(~"param", ref atts) => {
                    params.push(
                        self.consume_binding(atts.find_clone("group"))
                    );
                    self.expect_end_element("param");
                }
                StartElement(~"alias", ref atts) => {
                    alias = atts.find_clone("alias");
                    self.expect_end_element("alias");
                }
                StartElement(~"vecequiv", ref atts) => {
                    vecequiv = atts.find_clone("vecequiv");
                    self.expect_end_element("vecequiv");
                }
                StartElement(~"glx", ref atts) => {
                    glx = Some(GlxOpcode {
                        ty:      atts.get_clone("type"),
                        opcode:  atts.get_clone("opcode"),
                        name:    atts.find_clone("name"),
                        comment: atts.find_clone("comment"),
                    });
                    self.expect_end_element("glx");
                }
                EndElement(~"command") => break,
                msg => fail!("Expected </command>, found: %s", msg.to_str()),
            }
        }
        let is_safe = params.len() <= 0 || params.iter().all(|p| !p.ty.contains_char('*'));

        Cmd {
            proto: proto,
            params: params,
            is_safe: is_safe,
            alias: alias,
            vecequiv: vecequiv,
            glx: glx,
        }
    }

    fn consume_binding(&self, group: Option<~str>) -> Binding {
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
        // consume identifier
        let ident = self.expect_characters();
        self.expect_end_element("name");
        Binding {
            ident: ident,
            ty: ty,
            group: group,
        }
    }
}

trait FromXML {
    fn convert(r: &RegistryBuilder, a: &sax::Attributes) -> Self;
}

impl FromXML for Require {
    fn convert(r: &RegistryBuilder, a: &sax::Attributes) -> Require {
        debug!("Doing a FromXML on Require");
        let comment = a.find_clone("comment");
        let (enums, commands) = r.consume_two("enum", "command", "require");
        Require {
            comment: comment,
            enums: enums,
            commands: commands
        }
    }
}

impl FromXML for Remove {
    fn convert(r: &RegistryBuilder, a: &sax::Attributes) -> Remove {
        debug!("Doing a FromXML on Remove");
        let profile = a.get_clone("profile");
        let comment = a.get_clone("comment");
        let (enums, commands) = r.consume_two("enum", "command", "remove");

        Remove {
            profile: profile,
            comment: comment,
            enums: enums,
            commands: commands
        }
    }
}

impl FromXML for Feature {
    fn convert(r: &RegistryBuilder, a: &sax::Attributes) -> Feature {
        debug!("Doing a FromXML on Feature");
        let api      = a.get_clone("api");
        let name     = a.get_clone("name");
        let number   = a.get_clone("number");

        debug2!("Found api = {:s}, name = {:s}, number = {:s}", api, name, number);

        let (require, remove) = r.consume_two("require", "remove", "feature");

        Feature {
            api: api,
            name: name,
            number: number,
            requires: require,
            removes: remove
        }
    }
}

impl FromXML for Extension {
    fn convert(r: &RegistryBuilder, a: &sax::Attributes) -> Extension {
        debug!("Doing a FromXML on Extension");
        let name = a.get_clone("name");
        let supported = a.get("supported").split_iter('|').map(|x| x.to_owned()).to_owned_vec();
        let mut require = ~[];
        loop {
            match r.recv() {
                StartElement(~"require", ref atts) => {
                    require.push(FromXML::convert(r, atts));
                }
                EndElement(~"extension") => break,
                msg => fail2!("Unexpected message {}", msg.to_str())
            }
        }

        Extension {
            name: name,
            supported: supported,
            requires: require
        }
    }
}

impl FromXML for ~str {
    fn convert(_: &RegistryBuilder, a: &sax::Attributes) -> ~str {
        a.get_clone("name")
    }
}
