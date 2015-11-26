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

extern crate xml;

use std::collections::hash_map::Entry;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Add;
use std::fmt;
use std::str::FromStr;
use std::slice::Iter;
use std::io;

use self::xml::EventReader as XmlEventReader;
use self::xml::attribute::OwnedAttribute;
use self::xml::reader::XmlEvent;

#[derive(Copy, Clone)]
pub enum Ns { Gl, Glx, Wgl, Egl, Gles1, Gles2 }

pub enum Fallbacks { All, None }

impl Ns {
    pub fn fmt_struct_name(&self) -> &str {
        match *self {
            Ns::Gl  => "Gl",
            Ns::Glx => "Glx",
            Ns::Wgl => "Wgl",
            Ns::Egl => "Egl",
            Ns::Gles1 => "Gles1",
            Ns::Gles2 => "Gles2",
        }
    }
}

impl FromStr for Ns {
    type Err = ();
    fn from_str(s: &str) -> Result<Ns, ()> {
        match s {
            "gl"  => Ok(Ns::Gl),
            "glx" => Ok(Ns::Glx),
            "wgl" => Ok(Ns::Wgl),
            "egl" => Ok(Ns::Egl),
            "gles1" => Ok(Ns::Gles1),
            "gles2" => Ok(Ns::Gles2),
            _     => Err(()),
        }
    }
}

impl fmt::Display for Ns {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ns::Gl  => write!(fmt, "gl"),
            Ns::Glx => write!(fmt, "glx"),
            Ns::Wgl => write!(fmt, "wgl"),
            Ns::Egl => write!(fmt, "egl"),
            Ns::Gles1 => write!(fmt, "gles1"),
            Ns::Gles2 => write!(fmt, "gles2"),
        }
    }
}

fn trim_str<'a>(s: &'a str, trim: &str) -> &'a str {
    if s.starts_with(trim) { &s[trim.len()..] } else { s }
}

fn trim_enum_prefix<'a>(ident: &'a str, ns: Ns) -> &'a str {
    match ns {
        Ns::Gl | Ns::Gles1 | Ns::Gles2 => trim_str(ident, "GL_"),
        Ns::Glx => trim_str(ident, "GLX_"),
        Ns::Wgl =>  trim_str(ident, "WGL_"),
        Ns::Egl =>  trim_str(ident, "EGL_"),
    }
}

fn trim_cmd_prefix<'a>(ident: &'a str, ns: Ns) -> &'a str {
    match ns {
        Ns::Gl | Ns::Gles1 | Ns::Gles2 => trim_str(ident, "gl"),
        Ns::Glx => trim_str(ident, "glX"),
        Ns::Wgl =>  trim_str(ident, "wgl"),
        Ns::Egl =>  trim_str(ident, "egl"),
    }
}

fn merge_map(a: &mut HashMap<String, Vec<String>>, b: HashMap<String, Vec<String>>) {
    for (k, v) in b.into_iter() {
        match a.entry(k) {
            Entry::Occupied(mut ent) => { ent.get_mut().extend(v.into_iter()); },
            Entry::Vacant(ent) => { ent.insert(v); }
        }
    }
}

pub struct Registry {
    pub enums: Vec<Enum>,
    pub cmds: Vec<Cmd>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
    pub aliases: HashMap<String, Vec<String>>,
}

impl Registry {
    /// Generate a registry from the supplied XML string
    pub fn from_xml<R: io::Read>(data: R, ns: Ns, filter: Option<Filter>) -> Registry {
        use std::io::BufReader;
        let data = BufReader::new(data);

        RegistryBuilder {
            ns: ns,
            filter: filter,
            reader: XmlEventReader::new(data),
        }.consume_registry()
    }

    /// Returns a set of all the types used in the supplied registry. This is useful
    /// for working out what conversions are needed for the specific registry.
    pub fn get_tys(&self) -> BTreeSet<String> {
        let mut tys = BTreeSet::new();
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

impl Add for Registry {
    type Output = Registry;

    fn add(mut self, other: Registry) -> Registry {
        self.enums.extend(other.enums.into_iter());
        self.cmds.extend(other.cmds.into_iter());
        self.features.extend(other.features.into_iter());
        self.extensions.extend(other.extensions.into_iter());
        self.aliases.extend(other.aliases.into_iter());
        self
    }
}

pub struct EnumIterator<'a> {
    seen: HashSet<String>,
    iter: Iter<'a, Enum>,
}

impl<'a> Iterator for EnumIterator<'a> {
    type Item = &'a Enum;

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
    seen: HashSet<String>,
    iter: Iter<'a, Cmd>,
}

impl<'a> Iterator for CmdIterator<'a> {
    type Item = &'a Cmd;

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

pub struct Enum {
    pub ident: String,
    pub value: String,
    pub alias: Option<String>,
    pub ty: Option<String>,
}

pub struct Binding {
    pub ident: String,
    pub ty: String,
    pub group: Option<String>,
}

pub struct Cmd {
    pub proto: Binding,
    pub params: Vec<Binding>,
    /// True if this command doesn't take any pointers.
    ///
    /// Unused by the built-in generators.
    pub is_safe: bool,
    pub alias: Option<String>,
    pub vecequiv: Option<String>,
    pub glx: Option<GlxOpcode>,
}

#[derive(Clone)]
pub struct Feature {
    pub api: String,
    pub name: String,
    pub number: String,
    pub requires: Vec<Require>,
    pub removes: Vec<Remove>,
}

#[derive(Clone)]
pub struct Require {
    pub comment: Option<String>,
    /// A reference to the earlier types, by name
    pub enums: Vec<String>,
    /// A reference to the earlier types, by name
    pub commands: Vec<String>,
}

#[derive(Clone)]
pub struct Remove {
    // always core, for now
    pub profile: String,
    pub comment: String,
    /// A reference to the earlier types, by name
    pub enums: Vec<String>,
    /// A reference to the earlier types, by name
    pub commands: Vec<String>,
}

#[derive(Clone)]
pub struct Extension {
    pub name: String,
    /// which apis this extension is defined for (see Feature.api)
    pub supported: Vec<String>,
    pub requires: Vec<Require>,
}

pub struct GlxOpcode {
    pub ty: String,
    pub opcode: String,
    pub name: Option<String>,
    pub comment: Option<String>,
}

struct RegistryBuilder<R: io::Read> {
    pub ns: Ns,
    pub filter: Option<Filter>,
    pub reader: XmlEventReader<R>,
}

pub struct Filter {
    pub fallbacks: Fallbacks,
    pub extensions: Vec<String>,
    pub profile: String,
    pub version: String,
    pub api: String,
}

/// A big, ugly, imperative impl with methods that accumulates a Registry struct
impl<R: io::Read> RegistryBuilder<R> {
    fn next(&mut self) -> XmlEvent {
        loop {
            let event = self.reader.next();
            match event.unwrap() {
                XmlEvent::StartDocument { .. } => (),
                XmlEvent::EndDocument => panic!("The end of the document has been reached"),
                XmlEvent::Comment(_) => (),
                XmlEvent::Whitespace(_) => (),
                event => return event,
            }
        }
    }

    fn expect_characters(&mut self) -> String {
        match self.next() {
            XmlEvent::Characters(ch) => ch,
            msg => panic!("Expected characters, found: {:?}", msg),
        }
    }

    fn expect_start_element(&mut self, n: &str) -> Vec<OwnedAttribute> {
        match self.next() {
            XmlEvent::StartElement { name, attributes, .. } => {
                if n == name.local_name { attributes } else {
                    panic!("Expected <{}>, found: <{}>", n, name.local_name)
                }
            }
            msg => panic!("Expected <{}>, found: {:?}", n, msg),
        }
    }

    fn expect_end_element(&mut self, n: &str) {
        match self.next() {
            XmlEvent::EndElement { ref name } if n == name.local_name => (),
            msg => panic!("Expected </{}>, found: {:?}", n, msg),
        }
    }

    fn skip_until(&mut self, event: XmlEvent) {
        loop {
            match self.next() {
                XmlEvent::EndDocument => panic!("Expected {:?}, but reached the end of the document.", event),
                ref msg if *msg == event => break,
                _ => (),
            }
        }
    }

    fn consume_registry(&mut self) -> Registry {
        self.expect_start_element("registry");
        let mut registry = Registry {
            enums: Vec::new(),
            cmds: Vec::new(),
            features: Vec::new(),
            extensions: Vec::new(),
            aliases: HashMap::new(),
        };

        loop {
            match self.next() {
                // ignores
                XmlEvent::Characters(_) | XmlEvent::Comment(_) => (),
                XmlEvent::StartElement{ref name, ..}
                    if name.local_name == "comment" =>
                        self.skip_until(XmlEvent::EndElement { name: name.clone() }),
                XmlEvent::StartElement{ref name, ..}
                    if name.local_name == "types" =>
                        self.skip_until(XmlEvent::EndElement { name: name.clone() }),
                XmlEvent::StartElement{ref name, ..}
                    if name.local_name == "groups" =>
                        self.skip_until(XmlEvent::EndElement { name: name.clone() }),

                // add enum namespace
                XmlEvent::StartElement{ref name, ..} if name.local_name == "enums" => {
                    registry.enums.extend(self.consume_enums().into_iter());
                }

                // add command namespace
                XmlEvent::StartElement{ref name, ..} if name.local_name == "commands" => {
                    let (cmds, aliases) = self.consume_cmds();
                    registry.cmds.extend(cmds.into_iter());
                    merge_map(&mut registry.aliases, aliases);
                }

                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "feature" => {
                    debug!("Parsing feature: {:?}", attributes);
                    registry.features.push(FromXml::convert(self, &attributes));
                }

                XmlEvent::StartElement{ref name, ..} if name.local_name == "extensions" => {
                    loop {
                        match self.next() {
                            XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "extension" => {
                                registry.extensions.push(FromXml::convert(self, &attributes));
                            }
                            XmlEvent::EndElement{ref name} if name.local_name == "extensions" => break,
                            msg => panic!("Unexpected message {:?}", msg),
                        }
                    }
                }

                // finished building the registry
                XmlEvent::EndElement{ref name} if name.local_name == "registry" => break,

                // error handling
                msg => panic!("Expected </registry>, found: {:?}", msg),
            }
        }

        match self.filter {
            Some(ref filter) => {
                let Registry { enums, cmds, aliases, features: feats, extensions: exts } = registry;
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
                                    debug!("Removing {}", enm);
                                    desired_enums.remove(enm);
                                }
                                for cmd in rem.commands.iter() {
                                    debug!("Removing {}", cmd);
                                    desired_cmds.remove(cmd);
                                }
                            }
                        }
                    }
                }

                if !found_feat {
                    panic!("Did not find version {} in the registry", filter.version);
                }

                for ext in exts.iter() {
                    if filter.extensions.iter().any(|x| x == &ext.name) {
                        if !ext.supported.iter().any(|x| x == &filter.api) {
                            panic!("Requested {}, which doesn't support the {} API", ext.name, filter.api);
                        }
                        for req in ext.requires.iter() {
                            desired_enums.extend(req.enums.iter().map(|x| x.clone()));
                            desired_cmds.extend(req.commands.iter().map(|x| x.clone()));
                        }
                    }
                }

                let aliases = if let &Filter { fallbacks: Fallbacks::None, ..} = filter { HashMap::new() } else { aliases };

                Registry {
                    enums: enums.into_iter().filter(|e| {
                            desired_enums.contains(&("GL_".to_string() + &e.ident)) ||
                            desired_enums.contains(&("WGL_".to_string() + &e.ident)) ||
                            desired_enums.contains(&("GLX_".to_string() + &e.ident)) ||
                            desired_enums.contains(&("EGL_".to_string() + &e.ident))
                        }).collect::<Vec<Enum>>(),
                    cmds: cmds.into_iter().filter(|c| {
                            desired_cmds.contains(&("gl".to_string() + &c.proto.ident)) ||
                            desired_cmds.contains(&("wgl".to_string() + &c.proto.ident)) ||
                            desired_cmds.contains(&("glX".to_string() + &c.proto.ident)) ||
                            desired_cmds.contains(&("egl".to_string() + &c.proto.ident))
                        }).collect::<Vec<Cmd>>(),
                    // these aren't important after this step
                    features: Vec::new(),
                    extensions: Vec::new(),
                    aliases: aliases,
                }
            },
            None => registry
        }
    }

    fn consume_two<'a, T: FromXml, U: FromXml>(&mut self, one: &'a str, two: &'a str, end: &'a str) -> (Vec<T>, Vec<U>) {
        debug!("consume_two: looking for {} and {} until {}", one, two, end);

        let mut ones = Vec::new();
        let mut twos = Vec::new();

        loop {
            match self.next() {
                XmlEvent::StartElement{ref name, ref attributes, ..} => {
                    debug!("Found start element <{:?} {:?}>", name, attributes);
                    debug!("one and two are {} and {}", one, two);

                    let n = name.clone();

                    if one == n.local_name {
                        ones.push(FromXml::convert(self, &attributes));
                    } else if "type" == n.local_name {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if two == n.local_name {
                        twos.push(FromXml::convert(self, &attributes));
                    } else {
                        panic!("Unexpected element: <{:?} {:?}>", n, &attributes);
                    }
                },
                XmlEvent::EndElement{ref name} => {
                    debug!("Found end element </{:?}>", name);

                    if (&[one, two]).iter().any(|&x| x == name.local_name) {
                        continue;
                    } else if "type" == name.local_name {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if end == name.local_name {
                        return (ones, twos);
                    } else {
                        panic!("Unexpected end element {:?}", name.local_name);
                    }
                },
                msg => panic!("Unexpected message {:?}", msg) }
        }
    }

    fn consume_enums(&mut self) -> Vec<Enum> {
        let mut enums = Vec::new();
        loop {
            match self.next() {
                // ignores
                XmlEvent::Characters(_) | XmlEvent::Comment(_) => (),
                XmlEvent::StartElement{ref name, ..} if name.local_name == "unused" =>
                    self.skip_until(XmlEvent::EndElement{name: name.clone()}),

                // add enum definition
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "enum" => {
                    enums.push(
                        Enum {
                            ident:  trim_enum_prefix(&get_attribute(&attributes, "name").unwrap(), self.ns).to_string(),
                            value:  get_attribute(&attributes, "value").unwrap(),
                            alias:  get_attribute(&attributes, "alias"),
                            ty:     get_attribute(&attributes, "type"),
                        }
                    );
                    self.expect_end_element("enum");
                }

                // finished building the namespace
                XmlEvent::EndElement{ref name} if name.local_name == "enums" => break,
                // error handling
                msg => panic!("Expected </enums>, found: {:?}", msg),
            }
        }
        enums
    }

    fn consume_cmds(&mut self) -> (Vec<Cmd>, HashMap<String, Vec<String>>) {
        let mut cmds = Vec::new();
        let mut aliases: HashMap<String, Vec<String>> = HashMap::new();
        loop {
            match self.next() {
                // add command definition
                XmlEvent::StartElement{ref name, ..} if name.local_name == "command" => {
                    let new = self.consume_cmd();
                    match new.alias {
                        Some(ref v) => {
                            match aliases.entry(v.clone()) {
                                Entry::Occupied(mut ent) => { ent.get_mut().push(new.proto.ident.clone()); },
                                Entry::Vacant(ent) => { ent.insert(vec![new.proto.ident.clone()]); }
                            }
                        },
                        None => { }
                    }
                    cmds.push(new);
                }
                // finished building the namespace
                XmlEvent::EndElement{ref name} if name.local_name == "commands" => break,
                // error handling
                msg => panic!("Expected </commands>, found: {:?}", msg),
            }
        }
        (cmds, aliases)
    }

    fn consume_cmd(&mut self) -> Cmd {
        // consume command prototype
        let proto_attr = self.expect_start_element("proto");
        let mut proto = self.consume_binding("proto", get_attribute(&proto_attr, "group"));
        proto.ident = trim_cmd_prefix(&proto.ident, self.ns).to_string();

        let mut params = Vec::new();
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;
        loop {
            match self.next() {
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "param" => {
                    params.push(
                        self.consume_binding("param", get_attribute(&attributes, "group"))
                    );
                }
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "alias" => {
                    alias = get_attribute(&attributes, "name");
                    alias = alias.map(|t| trim_cmd_prefix(&t, self.ns).to_string());
                    self.expect_end_element("alias");
                }
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "vecequiv" => {
                    vecequiv = get_attribute(&attributes, "vecequiv");
                    self.expect_end_element("vecequiv");
                }
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "glx" => {
                    glx = Some(GlxOpcode {
                        ty:      get_attribute(&attributes, "type").unwrap(),
                        opcode:  get_attribute(&attributes, "opcode").unwrap(),
                        name:    get_attribute(&attributes, "name"),
                        comment: get_attribute(&attributes, "comment"),
                    });
                    self.expect_end_element("glx");
                }
                XmlEvent::EndElement{ref name} if name.local_name == "command" => break,
                msg => panic!("Expected </command>, found: {:?}", msg),
            }
        }
        let is_safe = params.len() <= 0 || params.iter().all(|p| !p.ty.contains('*'));

        Cmd {
            proto: proto,
            params: params,
            is_safe: is_safe,
            alias: alias,
            vecequiv: vecequiv,
            glx: glx,
        }
    }

    fn consume_binding(&mut self, outside_tag: &str, group: Option<String>) -> Binding {
        // consume type
        let mut ty = String::new();
        loop {
            match self.next() {
                XmlEvent::Characters(ch) => ty.push_str(&ch),
                XmlEvent::StartElement{ref name, ..} if name.local_name == "ptype" => (),
                XmlEvent::EndElement{ref name} if name.local_name == "ptype" => (),
                XmlEvent::StartElement{ref name, ..} if name.local_name == "name" => break,
                msg => panic!("Expected binding, found: {:?}", msg),
            }
        }

        // consume identifier
        let ident = self.expect_characters();
        self.expect_end_element("name");

        // consume the type suffix
        loop {
            match self.next() {
                XmlEvent::Characters(ch) => ty.push_str(&ch),
                XmlEvent::EndElement{ref name} if name.local_name == outside_tag => break,
                msg => panic!("Expected binding, found: {:?}", msg),
            }
        }

        Binding {
            ident: ident,
            ty: ty,
            group: group,
        }
    }
}

fn get_attribute(a: &[OwnedAttribute], name: &str) -> Option<String> {
    a.iter().find(|a| a.name.local_name == name).map(|e| e.value.clone())
}

trait FromXml {
    fn convert<R: io::Read>(r: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> Self;
}

impl FromXml for Require {
    fn convert<R: io::Read>(r: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> Require {
        debug!("Doing a FromXml on Require");
        let comment = get_attribute(a, "comment");
        let (enums, commands) = r.consume_two("enum", "command", "require");
        Require {
            comment: comment,
            enums: enums,
            commands: commands
        }
    }
}

impl FromXml for Remove {
    fn convert<R: io::Read>(r: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> Remove {
        debug!("Doing a FromXml on Remove");
        let profile = get_attribute(a, "profile").unwrap();
        let comment = get_attribute(a, "comment").unwrap();
        let (enums, commands) = r.consume_two("enum", "command", "remove");

        Remove {
            profile: profile,
            comment: comment,
            enums: enums,
            commands: commands
        }
    }
}

impl FromXml for Feature {
    fn convert<R: io::Read>(r: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> Feature {
        debug!("Doing a FromXml on Feature");
        let api      = get_attribute(a, "api").unwrap();
        let name     = get_attribute(a, "name").unwrap();
        let number   = get_attribute(a, "number").unwrap();

        debug!("Found api = {}, name = {}, number = {}", api, name, number);

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

impl FromXml for Extension {
    fn convert<R: io::Read>(r: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> Extension {
        debug!("Doing a FromXml on Extension");
        let name = get_attribute(a, "name").unwrap();
        let supported = get_attribute(a, "supported").unwrap().split('|').map(|x| x.to_string()).collect::<Vec<String>>();
        let mut require = Vec::new();
        loop {
            match r.next() {
                XmlEvent::StartElement{ref name, ref attributes, ..} if name.local_name == "require" => {
                    require.push(FromXml::convert(r, &attributes));
                }
                XmlEvent::EndElement{ref name} if name.local_name == "extension" => break,
                msg => panic!("Unexpected message {:?}", msg)
            }
        }

        Extension {
            name: name,
            supported: supported,
            requires: require
        }
    }
}

impl FromXml for String {
    fn convert<R: io::Read>(_: &mut RegistryBuilder<R>, a: &[OwnedAttribute]) -> String {
        get_attribute(a, "name").unwrap()
    }
}
