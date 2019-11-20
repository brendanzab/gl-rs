// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
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

#![allow(clippy::cognitive_complexity)]

use super::*;
use log::{debug, warn};
use std::collections::btree_map::Entry;
use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader as XmlEventReader};

pub fn from_xml<R: io::Read>(src: R, filter: &Filter, require_feature: bool) -> Registry {
    XmlEventReader::new(src)
        .into_iter()
        .map(Result::unwrap)
        .filter_map(ParseEvent::from_xml)
        .parse(filter, require_feature)
}

#[derive(Debug, PartialEq, Eq)]
struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    fn new<Key, Value>(key: Key, value: Value) -> Attribute
    where
        Key: ToString,
        Value: ToString,
    {
        Attribute {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<OwnedAttribute> for Attribute {
    fn from(attribute: OwnedAttribute) -> Attribute {
        Attribute::new(attribute.name.local_name, attribute.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseEvent {
    Start(String, Vec<Attribute>),
    End(String),
    Text(String),
}

impl ParseEvent {
    fn from_xml(event: XmlEvent) -> Option<ParseEvent> {
        match event {
            XmlEvent::StartDocument { .. } => None,
            XmlEvent::EndDocument => None,
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let attributes = attributes.into_iter().map(Attribute::from).collect();
                Some(ParseEvent::Start(name.local_name, attributes))
            },
            XmlEvent::EndElement { name } => Some(ParseEvent::End(name.local_name)),
            XmlEvent::Characters(chars) => Some(ParseEvent::Text(chars)),
            XmlEvent::ProcessingInstruction { .. } => None,
            XmlEvent::CData(_) => None,
            XmlEvent::Comment(_) => None,
            XmlEvent::Whitespace(_) => None,
        }
    }
}

fn api_from_str(src: &str) -> Result<Option<Api>, ()> {
    match src {
        "gl" => Ok(Some(Api::Gl)),
        "glx" => Ok(Some(Api::Glx)),
        "wgl" => Ok(Some(Api::Wgl)),
        "egl" => Ok(Some(Api::Egl)),
        "glcore" => Ok(Some(Api::GlCore)),
        "gles1" => Ok(Some(Api::Gles1)),
        "gles2" => Ok(Some(Api::Gles2)),
        "glsc2" => Ok(Some(Api::Glsc2)),
        "disabled" => Ok(None),
        _ => Err(()),
    }
}

fn profile_from_str(src: &str) -> Result<Profile, ()> {
    match src {
        "core" => Ok(Profile::Core),
        "compatibility" => Ok(Profile::Compatibility),
        _ => Err(()),
    }
}

fn underscore_numeric_prefix(src: &str) -> String {
    match src.chars().next() {
        Some(c) if c.is_numeric() => format!("_{}", src),
        Some(_) | None => src.to_string(),
    }
}

fn underscore_keyword(ident: String) -> String {
    match ident.as_ref() {
        "in" => "in_".to_string(),
        "ref" => "ref_".to_string(),
        "type" => "type_".to_string(),
        _ => ident,
    }
}

fn trim_str<'a>(s: &'a str, trim: &str) -> &'a str {
    if s.starts_with(trim) {
        &s[trim.len()..]
    } else {
        s
    }
}

fn trim_enum_prefix(ident: &str, api: Api) -> String {
    let ident = match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 | Api::Glsc2 => trim_str(ident, "GL_"),
        Api::Glx => trim_str(ident, "GLX_"),
        Api::Wgl => trim_str(ident, "WGL_"),
        Api::Egl => trim_str(ident, "EGL_"),
    };
    underscore_numeric_prefix(ident)
}

fn make_enum(ident: String, ty: Option<String>, value: String, alias: Option<String>) -> Enum {
    let (ty, value, cast) = {
        if value.starts_with("((") && value.ends_with(')') {
            // Some enums have a value of the form `'((' type ')' expr ')'`.

            // nothing to see here....
            // just brute forcing some paren matching... (ﾉ ◕ ◡ ◕)ﾉ *:･ﾟ✧
            let working = &value[2..value.len() - 1];
            if let Some((i, _)) = working.match_indices(')').next() {
                let ty = working[..i].to_string();
                let value = working[i + 1..].to_string();

                (Cow::Owned(ty), value, true)
            } else {
                panic!("Unexpected value format: {}", value)
            }
        } else {
            let ty = match ty {
                Some(ref ty) if ty == "u" => "GLuint",
                Some(ref ty) if ty == "ull" => "GLuint64",
                Some(ty) => panic!("Unhandled enum type: {}", ty),
                None if value.starts_with('"') => "&'static str",
                None if ident == "TRUE" || ident == "FALSE" => "GLboolean",
                None => "GLenum",
            };
            (Cow::Borrowed(ty), value, false)
        }
    };

    Enum {
        ident,
        value,
        cast,
        alias,
        ty,
    }
}

fn make_egl_enum(ident: String, ty: Option<String>, value: String, alias: Option<String>) -> Enum {
    let (ty, value, cast) = {
        if value.starts_with("EGL_CAST(") && value.ends_with(')') {
            // Handling "SpecialNumbers" in the egl.xml file
            // The values for these enums has the form `'EGL_CAST(' type ',' expr
            // ')'`.
            let working = &value[9..value.len() - 1];
            if let Some((i, _)) = working.match_indices(',').next() {
                let ty = working[..i].to_string();
                let value = working[i + 1..].to_string();

                (Cow::Owned(ty), value, true)
            } else {
                panic!("Unexpected value format: {}", value)
            }
        } else {
            match value.chars().next() {
                Some('-') | Some('0'..='9') => (),
                _ => panic!("Unexpected value format: {}", value),
            }

            let ty = match ty {
                Some(ref ty) if ty == "ull" => "EGLuint64KHR",
                Some(ty) => panic!("Unhandled enum type: {}", ty),
                None if value.starts_with('-') => "EGLint",
                None if ident == "TRUE" || ident == "FALSE" => "EGLBoolean",
                None => "EGLenum",
            };
            (Cow::Borrowed(ty), value, false)
        }
    };

    Enum {
        ident,
        value,
        cast,
        alias,
        ty,
    }
}

fn trim_cmd_prefix(ident: &str, api: Api) -> &str {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 | Api::Glsc2 => trim_str(ident, "gl"),
        Api::Glx => trim_str(ident, "glX"),
        Api::Wgl => trim_str(ident, "wgl"),
        Api::Egl => trim_str(ident, "egl"),
    }
}

fn merge_map(a: &mut BTreeMap<String, Vec<String>>, b: BTreeMap<String, Vec<String>>) {
    for (k, v) in b {
        match a.entry(k) {
            Entry::Occupied(mut ent) => {
                ent.get_mut().extend(v);
            },
            Entry::Vacant(ent) => {
                ent.insert(v);
            },
        }
    }
}

#[derive(Clone)]
struct Feature {
    pub api: Api,
    pub name: String,
    pub number: String,
    pub requires: Vec<Require>,
    pub removes: Vec<Remove>,
}

#[derive(Clone)]
struct Require {
    /// A reference to the earlier types, by name
    pub enums: Vec<String>,
    /// A reference to the earlier types, by name
    pub commands: Vec<String>,
}

#[derive(Clone)]
struct Remove {
    // always Core, for now
    pub profile: Profile,
    /// A reference to the earlier types, by name
    pub enums: Vec<String>,
    /// A reference to the earlier types, by name
    pub commands: Vec<String>,
}

#[derive(Clone)]
struct Extension {
    pub name: String,
    /// which apis this extension is defined for (see Feature.api)
    pub supported: Vec<Api>,
    pub requires: Vec<Require>,
}

pub struct Filter {
    pub api: Api,
    pub fallbacks: Fallbacks,
    pub extensions: BTreeSet<String>,
    pub profile: Profile,
    pub version: String,
}

trait Parse: Sized + Iterator<Item = ParseEvent> {
    fn parse(mut self, filter: &Filter, require_feature: bool) -> Registry {
        self.consume_start_element("registry");

        let mut enums = Vec::new();
        let mut cmds = Vec::new();
        let mut features = Vec::new();
        let mut extensions = Vec::new();
        let mut aliases = BTreeMap::new();
        let mut groups: BTreeMap<String, Group> = BTreeMap::new();

        while let Some(event) = self.next() {
            match event {
                // ignores
                ParseEvent::Text(_) => (),
                ParseEvent::Start(ref name, _) if name == "comment" => self.skip_to_end("comment"),
                ParseEvent::Start(ref name, _) if name == "types" => self.skip_to_end("types"),

                // add group namespace
                ParseEvent::Start(ref name, _) if name == "groups" => {
                    groups.extend(self.consume_groups(filter.api));
                },

                // add enum namespace
                ParseEvent::Start(ref name, ref attributes) if name == "enums" => {
                    enums.extend(self.consume_enums(filter.api));
                    let enums_group = get_attribute(&attributes, "group");
                    let enums_type = get_attribute(&attributes, "type");
                    if let Some(group) = enums_group.and_then(|name| groups.get_mut(&name)) {
                        group.enums_type = enums_type;
                    }
                },

                // add command namespace
                ParseEvent::Start(ref name, _) if name == "commands" => {
                    let (new_cmds, new_aliases) = self.consume_cmds(filter.api);
                    cmds.extend(new_cmds);
                    merge_map(&mut aliases, new_aliases);
                },

                ParseEvent::Start(ref name, ref attributes) if name == "feature" => {
                    debug!("Parsing feature: {:?}", attributes);
                    features.push(Feature::convert(&mut self, &attributes));
                },

                ParseEvent::Start(ref name, _) if name == "extensions" => loop {
                    match self.next().unwrap() {
                        ParseEvent::Start(ref name, ref attributes) if name == "extension" => {
                            extensions.push(Extension::convert(&mut self, &attributes));
                        },
                        ParseEvent::End(ref name) if name == "extensions" => break,
                        event => panic!("Unexpected message {:?}", event),
                    }
                },

                // finished building the registry
                ParseEvent::End(ref name) if name == "registry" => break,

                // error handling
                event => panic!("Expected </registry>, found: {:?}", event),
            }
        }

        let mut desired_enums = BTreeSet::new();
        let mut desired_cmds = BTreeSet::new();

        // find the features we want
        let mut found_feature = false;
        for feature in &features {
            // XXX: verify that the string comparison with <= actually works as
            // desired
            if feature.api == filter.api && feature.number <= filter.version {
                for require in &feature.requires {
                    desired_enums.extend(require.enums.iter().cloned());
                    desired_cmds.extend(require.commands.iter().cloned());
                }

                for remove in &feature.removes {
                    if remove.profile == filter.profile {
                        for enm in &remove.enums {
                            debug!("Removing {}", enm);
                            desired_enums.remove(enm);
                        }
                        for cmd in &remove.commands {
                            debug!("Removing {}", cmd);
                            desired_cmds.remove(cmd);
                        }
                    }
                }
            }
            if feature.number == filter.version {
                found_feature = true;
            }
        }

        if !found_feature && require_feature {
            panic!("Did not find version {} in the registry", filter.version);
        }

        for extension in &extensions {
            if filter.extensions.contains(&extension.name) {
                if !extension.supported.contains(&filter.api) {
                    panic!(
                        "Requested {}, which doesn't support the {} API",
                        extension.name, filter.api
                    );
                }
                for require in &extension.requires {
                    desired_enums.extend(require.enums.iter().cloned());
                    desired_cmds.extend(require.commands.iter().cloned());
                }
            }
        }

        let is_desired_enum = |e: &Enum| {
            desired_enums.contains(&("GL_".to_string() + &e.ident))
                || desired_enums.contains(&("WGL_".to_string() + &e.ident))
                || desired_enums.contains(&("GLX_".to_string() + &e.ident))
                || desired_enums.contains(&("EGL_".to_string() + &e.ident))
        };

        let is_desired_cmd = |c: &Cmd| {
            desired_cmds.contains(&("gl".to_string() + &c.proto.ident))
                || desired_cmds.contains(&("wgl".to_string() + &c.proto.ident))
                || desired_cmds.contains(&("glX".to_string() + &c.proto.ident))
                || desired_cmds.contains(&("egl".to_string() + &c.proto.ident))
        };

        Registry {
            api: filter.api,
            version: (0, 0),
            fallbacks: Fallbacks::No,
            enums: enums.into_iter().filter(is_desired_enum).collect(),
            cmds: cmds.into_iter().filter(is_desired_cmd).collect(),
            aliases: if filter.fallbacks == Fallbacks::Yes {
                aliases
            } else {
                BTreeMap::new()
            },
            groups,
            extensions: Vec::new(),
        }
    }

    fn consume_characters(&mut self) -> String {
        match self.next().unwrap() {
            ParseEvent::Text(ch) => ch,
            event => panic!("Expected characters, found: {:?}", event),
        }
    }

    fn consume_start_element(&mut self, expected_name: &str) -> Vec<Attribute> {
        match self.next().unwrap() {
            ParseEvent::Start(name, attributes) => {
                if expected_name == name {
                    attributes
                } else {
                    panic!("Expected <{}>, found: <{}>", expected_name, name)
                }
            },
            event => panic!("Expected <{}>, found: {:?}", expected_name, event),
        }
    }

    fn consume_end_element(&mut self, expected_name: &str) {
        match self.next().unwrap() {
            ParseEvent::End(ref name) if expected_name == name => (),
            event => panic!("Expected </{}>, found: {:?}", expected_name, event),
        }
    }

    fn skip_to_end(&mut self, expected_name: &str) {
        loop {
            match self.next().unwrap() {
                ParseEvent::End(ref name) if expected_name == name => break,
                _ => {},
            }
        }
    }

    fn consume_two<'a, T: FromXml, U: FromXml>(
        &mut self,
        one: &'a str,
        two: &'a str,
        end: &'a str,
    ) -> (Vec<T>, Vec<U>) {
        debug!("consume_two: looking for {} and {} until {}", one, two, end);

        let mut ones = Vec::new();
        let mut twos = Vec::new();

        loop {
            match self.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) => {
                    debug!("Found start element <{:?} {:?}>", name, attributes);
                    debug!("one and two are {} and {}", one, two);

                    let n = name.clone();

                    if one == n {
                        ones.push(FromXml::convert(self, &attributes));
                    } else if "type" == n {
                        // XXX: GL1.1 contains types, which we never care about anyway.
                        // Make sure consume_two doesn't get used for things which *do*
                        // care about type.
                        warn!("Ignoring type!");
                        continue;
                    } else if two == n {
                        twos.push(FromXml::convert(self, &attributes));
                    } else {
                        panic!("Unexpected element: <{:?} {:?}>", n, &attributes);
                    }
                },
                ParseEvent::End(ref name) => {
                    debug!("Found end element </{:?}>", name);

                    if one == name || two == name {
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
                        panic!("Unexpected end element {:?}", name);
                    }
                },
                event => panic!("Unexpected message {:?}", event),
            }
        }
    }

    fn consume_enums(&mut self, api: Api) -> Vec<Enum> {
        let mut enums = Vec::new();
        loop {
            match self.next().unwrap() {
                // ignores
                ParseEvent::Text(_) => {},
                ParseEvent::Start(ref name, _) if name == "unused" => self.skip_to_end("unused"),

                // add enum definition
                ParseEvent::Start(ref name, ref attributes) if name == "enum" => {
                    enums.push(self.consume_enum(api, attributes));
                },

                // finished building the namespace
                ParseEvent::End(ref name) if name == "enums" => break,
                // error handling
                event => panic!("Expected </enums>, found: {:?}", event),
            }
        }
        enums
    }

    fn consume_enum(&mut self, api: Api, attributes: &[Attribute]) -> Enum {
        let ident = trim_enum_prefix(&get_attribute(&attributes, "name").unwrap(), api);
        let value = get_attribute(&attributes, "value").unwrap();
        let alias = get_attribute(&attributes, "alias");
        let ty = get_attribute(&attributes, "type");
        self.consume_end_element("enum");

        match api {
            Api::Egl => make_egl_enum(ident, ty, value, alias),
            _ => make_enum(ident, ty, value, alias),
        }
    }

    fn consume_groups(&mut self, api: Api) -> BTreeMap<String, Group> {
        let mut groups = BTreeMap::new();
        loop {
            match self.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) if name == "group" => {
                    let ident = get_attribute(&attributes, "name").unwrap();
                    let group = Group {
                        ident: ident.clone(),
                        enums_type: None,
                        enums: self.consume_group_enums(api),
                    };
                    groups.insert(ident, group);
                },
                ParseEvent::End(ref name) if name == "groups" => break,
                event => panic!("Expected </groups>, found: {:?}", event),
            }
        }
        groups
    }

    fn consume_group_enums(&mut self, api: Api) -> Vec<String> {
        let mut enums = Vec::new();
        loop {
            match self.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) if name == "enum" => {
                    let enum_name = get_attribute(&attributes, "name");
                    enums.push(trim_enum_prefix(&enum_name.unwrap(), api));
                    self.consume_end_element("enum");
                },
                ParseEvent::End(ref name) if name == "group" => break,
                event => panic!("Expected </group>, found: {:?}", event),
            }
        }
        enums
    }

    fn consume_cmds(&mut self, api: Api) -> (Vec<Cmd>, BTreeMap<String, Vec<String>>) {
        let mut cmds = Vec::new();
        let mut aliases: BTreeMap<String, Vec<String>> = BTreeMap::new();
        loop {
            match self.next().unwrap() {
                // add command definition
                ParseEvent::Start(ref name, _) if name == "command" => {
                    let new = self.consume_cmd(api);
                    if let Some(ref v) = new.alias {
                        match aliases.entry(v.clone()) {
                            Entry::Occupied(mut ent) => {
                                ent.get_mut().push(new.proto.ident.clone());
                            },
                            Entry::Vacant(ent) => {
                                ent.insert(vec![new.proto.ident.clone()]);
                            },
                        }
                    }
                    cmds.push(new);
                },
                // finished building the namespace
                ParseEvent::End(ref name) if name == "commands" => break,
                // error handling
                event => panic!("Expected </commands>, found: {:?}", event),
            }
        }
        (cmds, aliases)
    }

    fn consume_cmd(&mut self, api: Api) -> Cmd {
        // consume command prototype
        self.consume_start_element("proto");
        let mut proto = self.consume_binding("proto", &[]);
        proto.ident = trim_cmd_prefix(&proto.ident, api).to_string();

        let mut params = Vec::new();
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;
        loop {
            match self.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) if name == "param" => {
                    params.push(self.consume_binding("param", attributes));
                },
                ParseEvent::Start(ref name, ref attributes) if name == "alias" => {
                    alias = get_attribute(&attributes, "name");
                    alias = alias.map(|t| trim_cmd_prefix(&t, api).to_string());
                    self.consume_end_element("alias");
                },
                ParseEvent::Start(ref name, ref attributes) if name == "vecequiv" => {
                    vecequiv = get_attribute(&attributes, "vecequiv");
                    self.consume_end_element("vecequiv");
                },
                ParseEvent::Start(ref name, ref attributes) if name == "glx" => {
                    glx = Some(GlxOpcode {
                        opcode: get_attribute(&attributes, "opcode").unwrap(),
                        name: get_attribute(&attributes, "name"),
                    });
                    self.consume_end_element("glx");
                },
                ParseEvent::End(ref name) if name == "command" => break,
                event => panic!("Expected </command>, found: {:?}", event),
            }
        }

        Cmd {
            proto,
            params,
            alias,
            vecequiv,
            glx,
        }
    }

    fn consume_binding(&mut self, outside_tag: &str, attributes: &[Attribute]) -> Binding {
        // consume type
        let mut ty = String::new();
        loop {
            match self.next().unwrap() {
                ParseEvent::Text(text) => ty.push_str(&text),
                ParseEvent::Start(ref name, _) if name == "ptype" => (),
                ParseEvent::End(ref name) if name == "ptype" => (),
                ParseEvent::Start(ref name, _) if name == "name" => break,
                event => panic!("Expected binding, found: {:?}", event),
            }
        }

        // consume identifier
        let ident = underscore_keyword(self.consume_characters());
        self.consume_end_element("name");

        // consume the type suffix
        loop {
            match self.next().unwrap() {
                ParseEvent::Text(text) => ty.push_str(&text),
                ParseEvent::End(ref name) if name == outside_tag => break,
                event => panic!("Expected binding, found: {:?}", event),
            }
        }

        Binding {
            ident,
            ty: to_rust_ty(ty),
            group: get_attribute(&attributes, "group"),
        }
    }
}

impl<T> Parse for T where T: Sized + Iterator<Item = ParseEvent> {}

fn get_attribute(attribs: &[Attribute], key: &str) -> Option<String> {
    attribs
        .iter()
        .find(|attrib| attrib.key == key)
        .map(|attrib| attrib.value.clone())
}

trait FromXml {
    fn convert<P: Parse>(parser: &mut P, a: &[Attribute]) -> Self;
}

impl FromXml for Require {
    fn convert<P: Parse>(parser: &mut P, _: &[Attribute]) -> Require {
        debug!("Doing a FromXml on Require");
        let (enums, commands) = parser.consume_two("enum", "command", "require");
        Require { enums, commands }
    }
}

impl FromXml for Remove {
    fn convert<P: Parse>(parser: &mut P, a: &[Attribute]) -> Remove {
        debug!("Doing a FromXml on Remove");
        let profile = get_attribute(a, "profile").unwrap();
        let profile = profile_from_str(&profile).unwrap();
        let (enums, commands) = parser.consume_two("enum", "command", "remove");

        Remove {
            profile,
            enums,
            commands,
        }
    }
}

impl FromXml for Feature {
    fn convert<P: Parse>(parser: &mut P, a: &[Attribute]) -> Feature {
        debug!("Doing a FromXml on Feature");
        let api = get_attribute(a, "api").unwrap();
        let api = api_from_str(&api).unwrap().unwrap();
        let name = get_attribute(a, "name").unwrap();
        let number = get_attribute(a, "number").unwrap();

        debug!("Found api = {}, name = {}, number = {}", api, name, number);

        let (requires, removes) = parser.consume_two("require", "remove", "feature");

        Feature {
            api,
            name,
            number,
            requires,
            removes,
        }
    }
}

impl FromXml for Extension {
    fn convert<P: Parse>(parser: &mut P, a: &[Attribute]) -> Extension {
        debug!("Doing a FromXml on Extension");
        let name = get_attribute(a, "name").unwrap();
        let supported = get_attribute(a, "supported")
            .unwrap()
            .split('|')
            .filter_map(|api| {
                api_from_str(api).unwrap_or_else(|()| panic!("unsupported API `{}`", api))
            })
            .collect::<Vec<_>>();
        let mut requires = Vec::new();
        loop {
            match parser.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) if name == "require" => {
                    requires.push(FromXml::convert(parser, &attributes));
                },
                ParseEvent::End(ref name) if name == "extension" => break,
                event => panic!("Unexpected message {:?}", event),
            }
        }

        Extension {
            name,
            supported,
            requires,
        }
    }
}

impl FromXml for String {
    fn convert<P: Parse>(_: &mut P, a: &[Attribute]) -> String {
        get_attribute(a, "name").unwrap()
    }
}

/// Converts a C style type definition to the Rust equivalent
pub fn to_rust_ty<T: AsRef<str>>(ty: T) -> Cow<'static, str> {
    let ty = match ty.as_ref().trim() {
        // gl.xml types
        "GLDEBUGPROC" => "GLDEBUGPROC",
        "GLDEBUGPROCAMD" => "GLDEBUGPROCAMD",
        "GLDEBUGPROCARB" => "GLDEBUGPROCARB",
        "GLDEBUGPROCKHR" => "GLDEBUGPROCKHR",
        "GLbitfield" => "GLbitfield",
        "GLboolean" => "GLboolean",
        "GLbyte" => "GLbyte",
        "GLclampd" => "GLclampd",
        "GLclampf" => "GLclampf",
        "GLclampx" => "GLclampx",
        "GLdouble" => "GLdouble",
        "GLeglImageOES" => "GLeglImageOES",
        "GLenum" => "GLenum",
        "GLfixed" => "GLfixed",
        "GLfloat" => "GLfloat",
        "GLhalfNV" => "GLhalfNV",
        "GLhandleARB" => "GLhandleARB",
        "GLint" => "GLint",
        "GLint64" => "GLint64",
        "GLint64EXT" => "GLint64EXT",
        "GLintptr" => "GLintptr",
        "GLintptrARB" => "GLintptrARB",
        "GLshort" => "GLshort",
        "GLsizei" => "GLsizei",
        "GLsizeiptr" => "GLsizeiptr",
        "GLsizeiptrARB" => "GLsizeiptrARB",
        "GLsync" => "GLsync",
        "GLubyte" => "GLubyte",
        "GLuint" => "GLuint",
        "GLuint64" => "GLuint64",
        "GLuint64EXT" => "GLuint64EXT",
        "GLushort" => "GLushort",
        "GLvdpauSurfaceNV" => "GLvdpauSurfaceNV",
        "void" => "()",
        "GLboolean *" => "*mut GLboolean",
        "GLchar *" => "*mut GLchar",
        "const GLchar*" => "*const GLchar",
        "GLcharARB *" => "*mut GLcharARB",
        "GLdouble *" => "*mut GLdouble",
        "GLenum *" => "*mut GLenum",
        "GLfixed *" => "*mut GLfixed",
        "GLfloat *" => "*mut GLfloat",
        "GLhandleARB *" => "*mut GLhandleARB",
        "GLint *" => "*mut GLint",
        "GLint64 *" => "*mut GLint64",
        "GLint64EXT *" => "*mut GLint64EXT",
        "GLsizei *" => "*mut GLsizei",
        "GLubyte *" => "*mut GLubyte",
        "GLuint *" => "*mut GLuint",
        "GLuint [2]" => "*mut [GLuint; 2]",
        "GLuint64 *" => "*mut GLuint64",
        "GLuint64EXT *" => "*mut GLuint64EXT",
        "GLushort *" => "*mut GLushort",
        "GLvoid *" => "*mut GLvoid",
        "GLvoid **" => "*const *mut GLvoid",
        "void *" => "*mut c_void",
        "void **" => "*const *mut c_void",
        "const GLboolean *" => "*const GLboolean",
        "const GLbyte *" => "*const GLbyte",
        "const GLchar *" => "*const GLchar",
        "const GLcharARB *" => "*const GLcharARB",
        "const GLclampf *" => "*const GLclampf",
        "const GLdouble *" => "*const GLdouble",
        "const GLenum *" => "*const GLenum",
        "const GLfixed *" => "*const GLfixed",
        "const GLfloat" => "GLfloat",
        "const GLfloat *" => "*const GLfloat",
        "const GLhalfNV *" => "*const GLhalfNV",
        "const GLint *" => "*const GLint",
        "const GLint*" => "*const GLint",
        "const GLint64 *" => "*const GLint64",
        "const GLint64EXT *" => "*const GLint64EXT",
        "const GLintptr *" => "*const GLintptr",
        "const GLshort *" => "*const GLshort",
        "const GLsizei*" | "const GLsizei *" => "*const GLsizei",
        "const GLsizeiptr *" => "*const GLsizeiptr",
        "const GLubyte *" => "*const GLubyte",
        "const GLuint *" => "*const GLuint",
        "const GLuint64 *" => "*const GLuint64",
        "const GLuint64EXT *" => "*const GLuint64EXT",
        "const GLushort *" => "*const GLushort",
        "const GLvdpauSurfaceNV *" => "*const GLvdpauSurfaceNV",
        "const GLvoid *" => "*const GLvoid",
        "const void*" | "const void *" => "*const c_void",
        "const void **" => "*const *const c_void",
        "const void *const*" => "*const *const c_void",
        "const GLboolean **" => "*const *const GLboolean",
        "const GLchar **" => "*const *const GLchar",
        "const GLcharARB **" => "*const *const GLcharARB",
        "const GLvoid **" => "*const *const GLvoid",
        "const GLchar *const*" => "*const *const GLchar",
        "const GLvoid *const*" => "*const *const GLvoid",
        "struct _cl_context *" => "*const _cl_context",
        "struct _cl_event *" => "*const _cl_event",
        "GLuint[2]" => "[Gluint; 2]",

        // glx.xml types
        "Bool" => "Bool",
        "Colormap" => "Colormap",
        "DMbuffer" => "DMbuffer",
        "Font" => "Font",
        "GLXContext" => "GLXContext",
        "GLXContextID" => "GLXContextID",
        "GLXDrawable" => "GLXDrawable",
        "GLXFBConfig" => "GLXFBConfig",
        "GLXFBConfigSGIX" => "GLXFBConfigSGIX",
        "GLXPbuffer" => "GLXPbuffer",
        "GLXPbufferSGIX" => "GLXPbufferSGIX",
        "GLXPixmap" => "GLXPixmap",
        "GLXVideoCaptureDeviceNV" => "GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV" => "GLXVideoDeviceNV",
        "GLXVideoSourceSGIX" => "GLXVideoSourceSGIX",
        "GLXWindow" => "GLXWindow",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        "Pixmap" => "Pixmap",
        "Status" => "Status",
        "VLNode" => "VLNode",
        "VLPath" => "VLPath",
        "VLServer" => "VLServer",
        "Window" => "Window",
        "__GLXextFuncPtr" => "__GLXextFuncPtr",
        "const GLXContext" => "const GLXContext",
        "float" => "c_float",
        "int" => "c_int",
        "int64_t" => "i64",
        "unsigned int" => "c_uint",
        "unsigned long" => "c_ulong",
        // "void "                     => "()",
        "DMparams *" => "*mut DMparams",
        "Display *" => "*mut Display",
        "GLXFBConfig *" => "*mut GLXFBConfig",
        "GLXFBConfigSGIX *" => "*mut GLXFBConfigSGIX",
        "GLXHyperpipeConfigSGIX *" => "*mut GLXHyperpipeConfigSGIX",
        "GLXHyperpipeNetworkSGIX *" => "*mut GLXHyperpipeNetworkSGIX",
        "GLXVideoCaptureDeviceNV *" => "*mut GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV *" => "*mut GLXVideoDeviceNV",
        // "GLuint *"                  => "*mut GLuint",
        "XVisualInfo *" => "*mut XVisualInfo",
        // "const GLubyte *"           => "*GLubyte",
        "const char *" => "*const c_char",
        "const int *" => "*const c_int",
        // "const void *"              => "*const c_void",
        "int *" => "*mut c_int",
        "int32_t *" => "*mut i32",
        "int64_t *" => "*mut i64",
        "long *" => "*mut c_long",
        "unsigned int *" => "*mut c_uint",
        "unsigned long *" => "*mut c_ulong",
        // "void *"                    => "*mut c_void",

        // wgl.xml types
        "BOOL" => "BOOL",
        "DWORD" => "DWORD",
        "FLOAT" => "FLOAT",
        // "GLbitfield"                => "GLbitfield",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLfloat"                   => "GLfloat",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        // "GLushort"                  => "GLushort",
        "HANDLE" => "HANDLE",
        "HDC" => "HDC",
        "HENHMETAFILE" => "HENHMETAFILE",
        "HGLRC" => "HGLRC",
        "HGPUNV" => "HGPUNV",
        "HPBUFFERARB" => "HPBUFFERARB",
        "HPBUFFEREXT" => "HPBUFFEREXT",
        "HPVIDEODEV" => "HPVIDEODEV",
        "HVIDEOINPUTDEVICENV" => "HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV" => "HVIDEOOUTPUTDEVICENV",
        "INT" => "INT",
        "INT64" => "INT64",
        "LPCSTR" => "LPCSTR",
        "LPGLYPHMETRICSFLOAT" => "LPGLYPHMETRICSFLOAT",
        "LPVOID" => "LPVOID",
        "PGPU_DEVICE" => "PGPU_DEVICE",
        "PROC" => "PROC",
        "UINT" => "UINT",
        "VOID" => "VOID",
        // "int "                      => "c_int",
        // "unsigned int "             => "c_uint",
        // "void "                     => "()",
        "BOOL *" => "*mut BOOL",
        "DWORD *" => "*mut DWORD",
        "FLOAT *" => "*mut FLOAT",
        // "GLuint *"                  => "*mut GLuint",
        "HANDLE *" => "*mut HANDLE",
        "HGPUNV *" => "*mut HGPUNV",
        "HPVIDEODEV *" => "*mut HPVIDEODEV",
        "HVIDEOINPUTDEVICENV *" => "*mut HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV *" => "*mut HVIDEOOUTPUTDEVICENV",
        "INT32 *" => "*mut INT32",
        "INT64 *" => "*mut INT64",
        "UINT *" => "*mut UINT",
        "USHORT *" => "*mut USHORT",
        "const COLORREF *" => "*const COLORREF",
        "const DWORD *" => "*const DWORD",
        "const FLOAT *" => "*const FLOAT",
        // "const GLushort *"          => "*const GLushort",
        "const HANDLE *" => "*const HANDLE",
        "const HGPUNV *" => "*const HGPUNV",
        "const LAYERPLANEDESCRIPTOR *" => "*const LAYERPLANEDESCRIPTOR",
        "const LPVOID *" => "*const LPVOID",
        "const PIXELFORMATDESCRIPTOR *" => "*const IXELFORMATDESCRIPTOR",
        "const USHORT *" => "*const USHORT",
        // "const char *"              => "*const c_char",
        // "const int *"               => "*const c_int",
        "float *" => "*mut c_float",
        // "int *"                     => "*mut c_int",
        // "unsigned long *"           => "*mut c_ulong",
        // "void *"                    => "*mut c_void",

        // elx.xml types
        "khronos_utime_nanoseconds_t" => "khronos_utime_nanoseconds_t",
        "khronos_uint64_t" => "khronos_uint64_t",
        "khronos_ssize_t" => "khronos_ssize_t",
        "EGLNativeDisplayType" => "EGLNativeDisplayType",
        "EGLNativePixmapType" => "EGLNativePixmapType",
        "EGLNativeWindowType" => "EGLNativeWindowType",
        "EGLint" => "EGLint",
        "EGLint *" => "*mut EGLint",
        "const EGLint *" => "*const EGLint",
        "NativeDisplayType" => "NativeDisplayType",
        "NativePixmapType" => "NativePixmapType",
        "NativeWindowType" => "NativeWindowType",
        //"Bool"                      => "Bool",
        "EGLBoolean" => "EGLBoolean",
        "EGLenum" => "EGLenum",
        "EGLAttribKHR" => "EGLAttribKHR",
        "EGLAttrib" => "EGLAttrib",
        "EGLAttrib *" => "*mut EGLAttrib",
        "const EGLAttrib *" => "*const EGLAttrib",
        "const EGLattrib *" => "*const EGLAttrib", /* Due to a typo in */
        // khronos_api/
        // api_angle/scripts/
        // egl_angle_ext.xml -
        // see brendanzab/
        // gl-rs#491
        "EGLConfig" => "EGLConfig",
        "EGLConfig *" => "*mut EGLConfig",
        "EGLContext" => "EGLContext",
        "EGLDeviceEXT" => "EGLDeviceEXT",
        "EGLDisplay" => "EGLDisplay",
        "EGLSurface" => "EGLSurface",
        "EGLClientBuffer" => "EGLClientBuffer",
        "__eglMustCastToProperFunctionPointerType" => "__eglMustCastToProperFunctionPointerType",
        "EGLImageKHR" => "EGLImageKHR",
        "EGLImage" => "EGLImage",
        "EGLOutputLayerEXT" => "EGLOutputLayerEXT",
        "EGLOutputPortEXT" => "EGLOutputPortEXT",
        "EGLSyncKHR" => "EGLSyncKHR",
        "EGLSync" => "EGLSync",
        "EGLTimeKHR" => "EGLTimeKHR",
        "EGLTime" => "EGLTime",
        "EGLSyncNV" => "EGLSyncNV",
        "EGLTimeNV" => "EGLTimeNV",
        "EGLuint64NV" => "EGLuint64NV",
        "EGLStreamKHR" => "EGLStreamKHR",
        "EGLuint64KHR" => "EGLuint64KHR",
        "EGLNativeFileDescriptorKHR" => "EGLNativeFileDescriptorKHR",
        "EGLsizeiANDROID" => "EGLsizeiANDROID",
        "EGLSetBlobFuncANDROID" => "EGLSetBlobFuncANDROID",
        "EGLGetBlobFuncANDROID" => "EGLGetBlobFuncANDROID",
        "EGLClientPixmapHI" => "EGLClientPixmapHI",
        "struct EGLClientPixmapHI *" => "*const EGLClientPixmapHI",
        "const EGLAttribKHR *" => "*const EGLAttribKHR",
        "const EGLuint64KHR *" => "*const EGLuint64KHR",
        "EGLAttribKHR *" => "*mut EGLAttribKHR",
        "EGLDeviceEXT *" => "*mut EGLDeviceEXT",
        "EGLNativeDisplayType *" => "*mut EGLNativeDisplayType",
        "EGLNativePixmapType *" => "*mut EGLNativePixmapType",
        "EGLNativeWindowType *" => "*mut EGLNativeWindowType",
        "EGLOutputLayerEXT *" => "*mut EGLOutputLayerEXT",
        "EGLTimeKHR *" => "*mut EGLTimeKHR",
        "EGLOutputPortEXT *" => "*mut EGLOutputPortEXT",
        "EGLuint64KHR *" => "*mut EGLuint64KHR",
        "const struct AHardwareBuffer *" => "*const c_void", /* humm */

        "GLeglClientBufferEXT" => "GLeglClientBufferEXT",
        "GLVULKANPROCNV" => "GLVULKANPROCNV",
        "EGLDEBUGPROCKHR" => "EGLDEBUGPROCKHR",
        "EGLObjectKHR" => "EGLObjectKHR",
        "EGLLabelKHR" => "EGLLabelKHR",
        "EGLnsecsANDROID" => "EGLnsecsANDROID",
        "EGLnsecsANDROID *" => "*mut EGLnsecsANDROID",
        "EGLBoolean *" => "*mut EGLBoolean",

        // failure
        _ => panic!("Type conversion not implemented for `{}`", ty.as_ref()),
    };

    Cow::Borrowed(ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod underscore_numeric_prefix {
        use super::*;

        #[test]
        fn test_numeric_prefix() {
            assert_eq!(parse::underscore_numeric_prefix("3"), "_3");
            assert_eq!(parse::underscore_numeric_prefix("123_FOO"), "_123_FOO");
        }

        #[test]
        fn test_non_numeric_prefix() {
            assert_eq!(parse::underscore_numeric_prefix(""), "");
            assert_eq!(parse::underscore_numeric_prefix("A"), "A");
            assert_eq!(parse::underscore_numeric_prefix("FOO"), "FOO");
        }
    }

    mod underscore_keyword {
        use super::*;

        #[test]
        fn test_keyword() {
            assert_eq!(parse::underscore_keyword("in".to_string()), "in_");
            assert_eq!(parse::underscore_keyword("ref".to_string()), "ref_");
            assert_eq!(parse::underscore_keyword("type".to_string()), "type_");
        }

        #[test]
        fn test_non_keyword() {
            assert_eq!(parse::underscore_keyword("foo".to_string()), "foo");
            assert_eq!(parse::underscore_keyword("bar".to_string()), "bar");
        }
    }

    mod make_enum {
        use super::*;

        #[test]
        fn test_cast_0() {
            let e = parse::make_enum(
                "FOO".to_string(),
                None,
                "((EGLint)-1)".to_string(),
                Some("BAR".to_string()),
            );
            assert_eq!(e.ident, "FOO");
            assert_eq!((&*e.ty, &*e.value), ("EGLint", "-1"));
            assert_eq!(e.alias, Some("BAR".to_string()));
        }

        #[test]
        fn test_cast_1() {
            let e = parse::make_enum(
                "FOO".to_string(),
                None,
                "((EGLint)(-1))".to_string(),
                Some("BAR".to_string()),
            );
            assert_eq!(e.ident, "FOO");
            assert_eq!((&*e.ty, &*e.value), ("EGLint", "(-1)"));
            assert_eq!(e.alias, Some("BAR".to_string()));
        }

        #[test]
        fn test_no_type() {
            let e = parse::make_enum(
                "FOO".to_string(),
                None,
                "value".to_string(),
                Some("BAR".to_string()),
            );
            assert_eq!(e.ident, "FOO");
            assert_eq!(e.value, "value");
            assert_eq!(e.alias, Some("BAR".to_string()));
            assert_eq!(e.ty, "GLenum");
            assert_eq!(e.cast, false);
        }

        #[test]
        fn test_u() {
            let e = parse::make_enum(
                "FOO".to_string(),
                Some("u".to_string()),
                String::new(),
                None,
            );
            assert_eq!(e.ty, "GLuint");
        }

        #[test]
        fn test_ull() {
            let e = parse::make_enum(
                "FOO".to_string(),
                Some("ull".to_string()),
                String::new(),
                None,
            );
            assert_eq!(e.ty, "GLuint64");
        }

        #[test]
        #[should_panic]
        fn test_unknown_type() {
            parse::make_enum(
                "FOO".to_string(),
                Some("blargh".to_string()),
                String::new(),
                None,
            );
        }

        #[test]
        fn test_value_str() {
            let e = parse::make_enum("FOO".to_string(), None, "\"hi\"".to_string(), None);
            assert_eq!(e.ty, "&'static str");
        }

        #[test]
        fn test_ident_true() {
            let e = parse::make_enum("TRUE".to_string(), None, String::new(), None);
            assert_eq!(e.ty, "GLboolean");
        }

        #[test]
        fn test_ident_false() {
            let e = parse::make_enum("FALSE".to_string(), None, String::new(), None);
            assert_eq!(e.ty, "GLboolean");
        }
    }

    mod make_egl_enum {
        use super::*;

        #[test]
        fn test_cast_egl() {
            let e = parse::make_egl_enum(
                "FOO".to_string(),
                None,
                "EGL_CAST(EGLint,-1)".to_string(),
                Some("BAR".to_string()),
            );
            assert_eq!(e.ident, "FOO");
            assert_eq!((&*e.ty, &*e.value), ("EGLint", "-1"));
            assert_eq!(e.alias, Some("BAR".to_string()));
        }

        #[test]
        fn test_ident_true() {
            let e = parse::make_egl_enum("TRUE".to_string(), None, "1234".to_string(), None);
            assert_eq!(e.ty, "EGLBoolean");
        }

        #[test]
        fn test_ident_false() {
            let e = parse::make_egl_enum("FALSE".to_string(), None, "1234".to_string(), None);
            assert_eq!(e.ty, "EGLBoolean");
        }

        #[test]
        fn test_ull() {
            let e = parse::make_egl_enum(
                "FOO".to_string(),
                Some("ull".to_string()),
                "1234".to_string(),
                None,
            );
            assert_eq!(e.ty, "EGLuint64KHR");
        }

        #[test]
        fn test_negative_value() {
            let e = parse::make_egl_enum("FOO".to_string(), None, "-1".to_string(), None);
            assert_eq!(e.ty, "EGLint");
        }

        #[test]
        #[should_panic]
        fn test_unknown_type() {
            parse::make_egl_enum(
                "FOO".to_string(),
                Some("blargh".to_string()),
                String::new(),
                None,
            );
        }

        #[test]
        #[should_panic]
        fn test_unknown_value() {
            parse::make_egl_enum("FOO".to_string(), None, "a".to_string(), None);
        }

        #[test]
        #[should_panic]
        fn test_empty_value() {
            parse::make_egl_enum("FOO".to_string(), None, String::new(), None);
        }
    }

    mod parse_event {
        use super::*;

        mod from_xml {
            use super::*;

            use xml::{
                attribute::OwnedAttribute, common::XmlVersion, name::OwnedName,
                namespace::Namespace, reader::XmlEvent,
            };

            #[test]
            fn test_start_event() {
                let given = XmlEvent::StartElement {
                    name: OwnedName::local("element"),
                    attributes: vec![
                        OwnedAttribute::new(OwnedName::local("attr1"), "val1"),
                        OwnedAttribute::new(OwnedName::local("attr2"), "val2"),
                    ],
                    namespace: Namespace::empty(),
                };
                let expected = ParseEvent::Start(
                    "element".to_string(),
                    vec![
                        Attribute::new("attr1", "val1"),
                        Attribute::new("attr2", "val2"),
                    ],
                );
                assert_eq!(ParseEvent::from_xml(given), Some(expected));
            }

            #[test]
            fn test_end_element() {
                let given = XmlEvent::EndElement {
                    name: OwnedName::local("element"),
                };
                let expected = ParseEvent::End("element".to_string());
                assert_eq!(ParseEvent::from_xml(given), Some(expected));
            }

            #[test]
            fn test_characters() {
                let given = XmlEvent::Characters("text".to_string());
                let expected = ParseEvent::Text("text".to_string());
                assert_eq!(ParseEvent::from_xml(given), Some(expected));
            }

            #[test]
            fn test_start_document() {
                let given = XmlEvent::StartDocument {
                    version: XmlVersion::Version10,
                    encoding: "".to_string(),
                    standalone: None,
                };
                assert_eq!(ParseEvent::from_xml(given), None);
            }

            #[test]
            fn test_end_document() {
                let given = XmlEvent::EndDocument;
                assert_eq!(ParseEvent::from_xml(given), None);
            }

            #[test]
            fn test_processing_instruction() {
                let given = XmlEvent::ProcessingInstruction {
                    name: "".to_string(),
                    data: None,
                };
                assert_eq!(ParseEvent::from_xml(given), None);
            }

            #[test]
            fn test_cdata() {
                let given = XmlEvent::CData("CData".to_string());
                assert_eq!(ParseEvent::from_xml(given), None);
            }

            #[test]
            fn test_comment() {
                let given = XmlEvent::Comment("Comment".to_string());
                assert_eq!(ParseEvent::from_xml(given), None);
            }

            #[test]
            fn test_whitespace() {
                let given = XmlEvent::Whitespace("Whitespace".to_string());
                assert_eq!(ParseEvent::from_xml(given), None);
            }
        }
    }
}
