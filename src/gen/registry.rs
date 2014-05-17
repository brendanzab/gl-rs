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

extern crate sax;
extern crate collections;

use self::collections::TreeSet;
use self::collections::HashSet;
use std::fmt;
use std::from_str::FromStr;
use std::slice::Items;

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

impl fmt::Show for Ns {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Gl  => write!(fmt, "gl"),
            Glx => write!(fmt, "glx"),
            Wgl => write!(fmt, "wgl"),
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
    pub groups: Vec<Group>,
    pub enums: Vec<Enum>,
    pub cmds: Vec<Cmd>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
}

impl Registry {
    /// Generate a registry from the supplied XML string
    pub fn from_xml(data: &str, ns: Ns, filter: Option<Filter>) -> Registry {
        RegistryBuilder::parse(data, ns, filter)
    }

    /// Returns a set of all the types used in the supplied registry. This is useful
    /// for working out what conversions are needed for the specific registry.
    pub fn get_tys(&self) -> TreeSet<StrBuf> {
        let mut tys = TreeSet::new();
        for def in self.cmds.iter() {
            tys.insert(def.proto.ty.clone());
            for param in def.params.iter() {
                tys.insert(param.ty.clone());
            }
        }
        tys
    }

    pub fn enum_iter<'a>(&'a self) -> EnumIterator<'a> {
        EnumIterator {
            seen: HashSet::new(),
            iter: self.enums.iter(),
        }
    }

    pub fn cmd_iter<'a>(&'a self) -> CmdIterator<'a> {
        CmdIterator {
            seen: HashSet::new(),
            iter: self.cmds.iter(),
        }
    }
}

pub struct EnumIterator<'a> {
    seen: HashSet<StrBuf>,
    iter: Items<'a, Enum>,
}

impl<'a> Iterator<&'a Enum> for EnumIterator<'a> {
    fn next(&mut self) -> Option<&'a Enum> {
        self.iter.next().and_then(|def| {
            if !self.seen.contains(&def.ident) {
                self.seen.insert(def.ident.clone());
                Some(def)
            } else {
                self.next()
            }
        })
    }
}

pub struct CmdIterator<'a> {
    seen: HashSet<StrBuf>,
    iter: Items<'a, Cmd>,
}

impl<'a> Iterator<&'a Cmd> for CmdIterator<'a> {
    fn next(&mut self) -> Option<&'a Cmd> {
        self.iter.next().and_then(|def| {
            if !self.seen.contains(&def.proto.ident) {
                self.seen.insert(def.proto.ident.clone());
                Some(def)
            } else {
                self.next()
            }
        })
    }
}

pub struct Group {
    pub name: StrBuf,
    pub enums: Vec<StrBuf>,
}

pub struct EnumNs {
    pub namespace: StrBuf,
    pub group: Option<StrBuf>,
    pub ty: Option<StrBuf>,
    pub start: Option<StrBuf>,
    pub end: Option<StrBuf>,
    pub vendor: Option<StrBuf>,
    pub comment: Option<StrBuf>,
    pub defs: Vec<Enum>,
}

pub struct Enum {
    pub ident: StrBuf,
    pub value: StrBuf,
    pub alias: Option<StrBuf>,
    pub ty: Option<StrBuf>,
}

pub struct CmdNs {
    pub namespace: StrBuf,
    pub defs: Vec<Cmd>,
}

pub struct Binding {
    pub ident: StrBuf,
    pub ty: StrBuf,
    pub group: Option<StrBuf>,
}

pub struct Cmd {
    pub proto: Binding,
    pub params: Vec<Binding>,
    pub is_safe: bool,
    pub alias: Option<StrBuf>,
    pub vecequiv: Option<StrBuf>,
    pub glx: Option<GlxOpcode>,
}

#[deriving(Clone)]
pub struct Feature {
    pub api: StrBuf,
    pub name: StrBuf,
    pub number: StrBuf,
    pub requires: Vec<Require>,
    pub removes: Vec<Remove>,
}

#[deriving(Clone)]
pub struct Require {
    pub comment: Option<StrBuf>,
    /// A reference to the earlier types, by name
    pub enums: Vec<StrBuf>,
    /// A reference to the earlier types, by name
    pub commands: Vec<StrBuf>,
}

#[deriving(Clone)]
pub struct Remove {
    // always core, for now
    pub profile: StrBuf,
    pub comment: StrBuf,
    /// A reference to the earlier types, by name
    pub enums: Vec<StrBuf>,
    /// A reference to the earlier types, by name
    pub commands: Vec<StrBuf>,
}

#[deriving(Clone)]
pub struct Extension {
    pub name: StrBuf,
    /// which apis this extension is defined for (see Feature.api)
    pub supported: Vec<StrBuf>,
    pub requires: Vec<Require>,
}

pub struct GlxOpcode {
    pub ty: StrBuf,
    pub opcode: StrBuf,
    pub name: Option<StrBuf>,
    pub comment: Option<StrBuf>,
}

struct RegistryBuilder {
    pub ns: Ns,
    pub filter: Option<Filter>,
    pub port: Receiver<ParseResult>,
}

pub struct Filter {
    pub extensions: Vec<StrBuf>,
    pub profile: StrBuf,
    pub version: StrBuf,
    pub api: StrBuf,
}

/// A big, ugly, imperative impl with methods that accumulates a Registry struct
impl<'a> RegistryBuilder {
    fn parse(data: &str, ns: Ns, filter: Option<Filter>) -> Registry {
        RegistryBuilder {
            ns: ns,
            filter: filter,
            port: parse_str(data),
        }.consume_registry()
    }

    fn recv(&self) -> ParseEvent {
        loop {
            match self.port.recv() {
                Ok(StartDocument) => (),
                Ok(Comment(_)) => (),
                Ok(Characters(ref ch)) if ch.as_slice().is_whitespace() => (),
                Ok(EndDocument) => fail!("The end of the document has been reached"),
                Ok(event) => return event,
                Err(err) => fail!("XML error: {}", err.to_str()),
            }
        }
    }

    fn expect_characters(&self) -> StrBuf {
        match self.recv() {
            Characters(ref ch) => ch.clone(),
            msg => fail!("Expected characters, found: {}", msg.to_str()),
        }
    }

    fn expect_start_element(&self, name: &str) -> Attributes {
        match self.recv() {
            StartElement(ref n, ref atts) if name == n.as_slice() => atts.clone(),
            msg => fail!("Expected <{}>, found: {}", name, msg.to_str()),
        }
    }

    fn expect_end_element(&self, name: &str) {
        match self.recv() {
            EndElement(ref n) if name == n.as_slice() => (),
            msg => fail!("Expected </{}>, found: {}", name, msg.to_str()),
        }
    }

    fn skip_until(&self, event: ParseEvent) {
        loop {
            match self.recv() {
                EndDocument => fail!("Expected {}, but reached the end of the document.",
                                     event.to_str()),
                ref msg if *msg == event => break,
                _ => (),
            }
        }
    }

    fn consume_registry(&self) -> Registry {
        self.expect_start_element("registry");
        let mut registry = Registry {
            groups: Vec::new(),
            enums: Vec::new(),
            cmds: Vec::new(),
            features: Vec::new(),
            extensions: Vec::new(),
        };

        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) => (),
                StartElement(ref s, _) if s.as_slice() == "comment" => self.skip_until(EndElement("comment".to_strbuf())),
                StartElement(ref s, _) if s.as_slice() == "types" => self.skip_until(EndElement("types".to_strbuf())),

                // add groups
                StartElement(ref s, _) if s.as_slice() == "groups" => {
                    loop {
                        match self.recv() {
                            StartElement(ref s, ref atts) if s.as_slice() == "group" => {
                                registry.groups.push(
                                    self.consume_group(atts.get_clone("name"))
                                );
                            }
                            EndElement(ref s) if s.as_slice() == "groups" => break,
                            msg => fail!("Expected </groups>, found: {}", msg.to_str()),
                        }
                    }
                }

                // add enum namespace
                StartElement(ref s, _) if s.as_slice() == "enums" => {
                    registry.enums.extend(self.consume_enums().move_iter());
                }

                // add command namespace
                StartElement(ref s, _) if s.as_slice() == "commands" => {
                    registry.cmds.extend(self.consume_cmds().move_iter());
                }

                StartElement(ref s, ref atts) if s.as_slice() == "feature" => {
                    debug!("Parsing feature: {:?}", atts);
                    registry.features.push(FromXML::convert(self, atts));
                }

                StartElement(ref s, _) if s.as_slice() == "extensions" => {
                    loop {
                        match self.recv() {
                            StartElement(ref s, ref atts) if s.as_slice() == "extension" => {
                                registry.extensions.push(FromXML::convert(self, atts));
                            }
                            EndElement(ref s) if s.as_slice() == "extensions" => break,
                            msg => fail!("Unexpected message {}", msg.to_str()),
                        }
                    }
                }

                // finished building the registry
                EndElement(ref s) if s.as_slice() == "registry" => break,

                // error handling
                msg => fail!("Expected </registry>, found: {}", msg.to_str()),
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
                            desired_enums.extend(req.enums.iter().map(|x| x.clone()));
                            desired_cmds.extend(req.commands.iter().map(|x| x.clone()));
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
                                    debug!("Removing {:?}", enm);
                                    desired_enums.remove(enm);
                                }
                                for cmd in rem.commands.iter() {
                                    debug!("Removing {:?}", cmd);
                                    desired_cmds.remove(cmd);
                                }
                            }
                        }
                    }
                }

                if !found_feat {
                    fail!("Did not find version {} in the registry", filter.version);
                }

                for ext in exts.iter() {
                    if filter.extensions.iter().any(|x| x == &ext.name) {
                        if !ext.supported.iter().any(|x| x == &filter.api) {
                            fail!("Requested {}, which doesn't support the {} API", ext.name, filter.api);
                        }
                        for req in ext.requires.iter() {
                            desired_enums.extend(req.enums.iter().map(|x| x.clone()));
                            desired_cmds.extend(req.commands.iter().map(|x| x.clone()));
                        }
                    }
                }

                Registry {
                    groups: groups,
                    enums: enums.move_iter().filter(|e| desired_enums.contains(&("GL_".to_strbuf().append(e.ident.as_slice())))).collect::<Vec<Enum>>(),
                    cmds: cmds.move_iter().filter(|c| desired_cmds.contains(&("gl".to_strbuf().append(c.proto.ident.as_slice())))).collect::<Vec<Cmd>>(),
                    // these aren't important after this step
                    features: Vec::new(),
                    extensions: Vec::new(),
                }
            },
            None => registry
        }
    }

    fn consume_two<'a, T: FromXML, U: FromXML>(&self, one: &'a str, two: &'a str, end: &'a str) -> (Vec<T>, Vec<U>) {
        debug!("consume_two: looking for {:s} and {:s} until {:s}", one, two, end);

        let mut ones = Vec::new();
        let mut twos = Vec::new();

        loop {
            match self.recv() {
                StartElement(ref name, ref atts) => {
                    debug!("Found start element <{:?} {:?}>", name, atts);
                    debug!("one and two are {:?} and {:?}", one, two);

                    let n = name.clone();

                    if one == n.as_slice() {
                        ones.push(FromXML::convert(self, atts));
                    } else if "type" == n.as_slice() {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if two == n.as_slice() {
                        twos.push(FromXML::convert(self, atts));
                    } else {
                        fail!("Unexpected element: <{:?} {:?}>", n, atts);
                    }
                },
                EndElement(ref name) => {
                    debug!("Found end element </{:?}>", name);

                    if (&[one, two]).iter().any(|&x| x == name.as_slice()) {
                        continue;
                    } else if "type" == name.as_slice() {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if end == name.as_slice() {
                        return (ones, twos);
                    } else {
                        fail!("Unexpected end element {}", name);
                    }
                },
                msg => fail!("Unexpected message {}", msg.to_str()) }
        }
    }

    fn consume_group(&self, name: StrBuf) -> Group {
        let mut enms = Vec::new();
        loop {
            match self.recv() {
                StartElement(ref s, ref atts) if s.as_slice() == "enum" => {
                    enms.push(atts.get_clone("name"));
                    self.expect_end_element("enum");
                }
                EndElement(ref s) if s.as_slice() == "group" => break,
                msg => fail!("Expected </group>, found: {}", msg.to_str()),
            }
        }
        Group {
            name: name,
            enums: enms,
        }
    }

    fn consume_enums(&self) -> Vec<Enum> {
        let mut enums = Vec::new();
        loop {
            match self.recv() {
                // ignores
                Characters(_) | Comment(_) => (),
                StartElement(ref s, _) if s.as_slice() == "unused" => self.skip_until(EndElement("unused".to_strbuf())),

                // add enum definition
                StartElement(ref s, ref atts) if s.as_slice() == "enum" => {
                    enums.push(
                        Enum {
                            ident:  trim_enum_prefix(atts.get("name"), self.ns).to_strbuf(),
                            value:  atts.get_clone("value"),
                            alias:  atts.find_clone("alias"),
                            ty:     atts.find_clone("type"),
                        }
                    );
                    self.expect_end_element("enum");
                }

                // finished building the namespace
                EndElement(ref s) if s.as_slice() == "enums" => break,
                // error handling
                msg => fail!("Expected </enums>, found: {}", msg.to_str()),
            }
        }
        enums
    }

    fn consume_cmds(&self) -> Vec<Cmd> {
        let mut cmds = Vec::new();
        loop {
            match self.recv() {
                // add command definition
                StartElement(ref s, _) if s.as_slice() == "command" => {
                    cmds.push(self.consume_cmd());
                }
                // finished building the namespace
                EndElement(ref s) if s.as_slice() == "commands" => break,
                // error handling
                msg => fail!("Expected </commands>, found: {}", msg.to_str()),
            }
        }
        cmds
    }

    fn consume_cmd(&self) -> Cmd {
        // consume command prototype
        let proto_atts = self.expect_start_element("proto");
        let mut proto = self.consume_binding(proto_atts.find_clone("group"));
        proto.ident = trim_cmd_prefix(proto.ident.as_slice(), self.ns).to_strbuf();
        self.expect_end_element("proto");

        let mut params = Vec::new();
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;
        loop {
            match self.recv() {
                StartElement(ref s, ref atts) if s.as_slice() == "param" => {
                    params.push(
                        self.consume_binding(atts.find_clone("group"))
                    );
                    self.expect_end_element("param");
                }
                StartElement(ref s, ref atts) if s.as_slice() == "alias" => {
                    alias = atts.find_clone("alias");
                    self.expect_end_element("alias");
                }
                StartElement(ref s, ref atts) if s.as_slice() == "vecequiv" => {
                    vecequiv = atts.find_clone("vecequiv");
                    self.expect_end_element("vecequiv");
                }
                StartElement(ref s, ref atts) if s.as_slice() == "glx" => {
                    glx = Some(GlxOpcode {
                        ty:      atts.get_clone("type"),
                        opcode:  atts.get_clone("opcode"),
                        name:    atts.find_clone("name"),
                        comment: atts.find_clone("comment"),
                    });
                    self.expect_end_element("glx");
                }
                EndElement(ref s) if s.as_slice() == "command" => break,
                msg => fail!("Expected </command>, found: {}", msg.to_str()),
            }
        }
        let is_safe = params.len() <= 0 || params.iter().all(|p| !p.ty.as_slice().contains_char('*'));

        Cmd {
            proto: proto,
            params: params,
            is_safe: is_safe,
            alias: alias,
            vecequiv: vecequiv,
            glx: glx,
        }
    }

    fn consume_binding(&self, group: Option<StrBuf>) -> Binding {
        // consume type
        let mut ty = StrBuf::new();
        loop {
            match self.recv() {
                Characters(ch) => ty.push_str(ch.as_slice()),
                StartElement(ref s, _) if s.as_slice() == "ptype" => (),
                EndElement(ref s) if s.as_slice() == "ptype" => (),
                StartElement(ref s, _) if s.as_slice() == "name" => break,
                msg => fail!("Expected binding, found: {}", msg.to_str()),
            }
        }
        // consume identifier
        let ident = self.expect_characters();
        self.expect_end_element("name");
        Binding {
            ident: ident,
            ty: ty.into_strbuf(),
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

        debug!("Found api = {:s}, name = {:s}, number = {:s}", api, name, number);

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
        let supported = a.get("supported").split('|').map(|x| x.to_strbuf()).collect::<Vec<StrBuf>>();
        let mut require = Vec::new();
        loop {
            match r.recv() {
                StartElement(ref s, ref atts) if s.as_slice() == "require" => {
                    require.push(FromXML::convert(r, atts));
                }
                EndElement(ref s) if s.as_slice() == "extension" => break,
                msg => fail!("Unexpected message {}", msg.to_str())
            }
        }

        Extension {
            name: name,
            supported: supported,
            requires: require
        }
    }
}

impl FromXML for StrBuf {
    fn convert(_: &RegistryBuilder, a: &sax::Attributes) -> StrBuf {
        a.get_clone("name")
    }
}
