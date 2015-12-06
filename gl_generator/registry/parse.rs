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

extern crate khronos_api;

use std::collections::hash_map::Entry;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io;
use xml::EventReader as XmlEventReader;
use xml::reader::XmlEvent;

use {Fallbacks, Api, Profile};
use registry::{Binding, Cmd, Enum, GlxOpcode, Registry};

pub fn from_xml<R: io::Read>(src: R, filter: Filter) -> Registry {
    XmlEventReader::new(src).into_iter()
        .map(Result::unwrap)
        .filter_map(ParseEvent::from_xml)
        .parse(filter)
}

#[derive(Debug, PartialEq, Eq)]
enum ParseEvent {
    Start(String, Vec<(String, String)>),
    End(String),
    Text(String),
}

impl ParseEvent {
    fn from_xml(event: XmlEvent) -> Option<ParseEvent> {
        match event {
            XmlEvent::StartDocument { .. } => None,
            XmlEvent::EndDocument => None,
            XmlEvent::StartElement { name, attributes, .. } => {
                let attributes = attributes.into_iter()
                    .map(|att| (att.name.local_name, att.value))
                    .collect();
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

fn api_from_str(src: &str) -> Result<Api, ()> {
    match src {
        "gl" => Ok(Api::Gl),
        "glx" => Ok(Api::Glx),
        "wgl" => Ok(Api::Wgl),
        "egl" => Ok(Api::Egl),
        "glcore" => Ok(Api::GlCore),
        "gles1" => Ok(Api::Gles1),
        "gles2" => Ok(Api::Gles2),
        _     => Err(()),
    }
}

fn profile_from_str(src: &str) -> Result<Profile, ()> {
    match src {
        "core" => Ok(Profile::Core),
        "compatibility" => Ok(Profile::Compatibility),
        _ => Err(()),
    }
}

fn trim_str<'a>(s: &'a str, trim: &str) -> &'a str {
    if s.starts_with(trim) { &s[trim.len()..] } else { s }
}

fn trim_enum_prefix<'a>(ident: &'a str, api: Api) -> &'a str {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => trim_str(ident, "GL_"),
        Api::Glx => trim_str(ident, "GLX_"),
        Api::Wgl =>  trim_str(ident, "WGL_"),
        Api::Egl =>  trim_str(ident, "EGL_"),
    }
}

fn trim_cmd_prefix<'a>(ident: &'a str, api: Api) -> &'a str {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => trim_str(ident, "gl"),
        Api::Glx => trim_str(ident, "glX"),
        Api::Wgl =>  trim_str(ident, "wgl"),
        Api::Egl =>  trim_str(ident, "egl"),
    }
}

fn merge_map(a: &mut HashMap<String, Vec<String>>, b: HashMap<String, Vec<String>>) {
    for (k, v) in b {
        match a.entry(k) {
            Entry::Occupied(mut ent) => { ent.get_mut().extend(v); },
            Entry::Vacant(ent) => { ent.insert(v); }
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
    fn parse(mut self, filter: Filter) -> Registry {
        self.consume_start_element("registry");

        let mut enums = Vec::new();
        let mut cmds = Vec::new();
        let mut features = Vec::new();
        let mut extensions = Vec::new();
        let mut aliases = HashMap::new();

        while let Some(event) = self.next() {
            match event {
                // ignores
                ParseEvent::Text(_) => (),
                ParseEvent::Start(ref name, _) if name == "comment" => self.skip_to_end("comment"),
                ParseEvent::Start(ref name, _) if name == "types" => self.skip_to_end("types"),
                ParseEvent::Start(ref name, _) if name == "groups" => self.skip_to_end("groups"),

                // add enum namespace
                ParseEvent::Start(ref name, _) if name == "enums" => {
                    enums.extend(self.consume_enums(filter.api));
                }

                // add command namespace
                ParseEvent::Start(ref name, _) if name == "commands" => {
                    let (new_cmds, new_aliases) = self.consume_cmds(filter.api);
                    cmds.extend(new_cmds);
                    merge_map(&mut aliases, new_aliases);
                }

                ParseEvent::Start(ref name, ref attributes) if name == "feature" => {
                    debug!("Parsing feature: {:?}", attributes);
                    features.push(Feature::convert(&mut self, &attributes));
                }

                ParseEvent::Start(ref name, _) if name == "extensions" => {
                    loop {
                        match self.next().unwrap() {
                            ParseEvent::Start(ref name, ref attributes) if name == "extension" => {
                                extensions.push(Extension::convert(&mut self, &attributes));
                            }
                            ParseEvent::End(ref name) if name == "extensions" => break,
                            event => panic!("Unexpected message {:?}", event),
                        }
                    }
                }

                // finished building the registry
                ParseEvent::End(ref name) if name == "registry" => break,

                // error handling
                event => panic!("Expected </registry>, found: {:?}", event),
            }
        }

        let mut desired_enums = HashSet::new();
        let mut desired_cmds = HashSet::new();

        // find the features we want
        let mut found_feature = false;
        for feature in &features {
            // XXX: verify that the string comparison with <= actually works as desired
            if feature.api == filter.api && feature.number <= filter.version {
                for require in &feature.requires {
                    desired_enums.extend(require.enums.iter().map(|x| x.clone()));
                    desired_cmds.extend(require.commands.iter().map(|x| x.clone()));
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

        if !found_feature {
            panic!("Did not find version {} in the registry", filter.version);
        }

        for extension in &extensions {
            if filter.extensions.contains(&extension.name) {
                if !extension.supported.contains(&filter.api) {
                    panic!("Requested {}, which doesn't support the {} API", extension.name, filter.api);
                }
                for require in &extension.requires {
                    desired_enums.extend(require.enums.iter().map(|x| x.clone()));
                    desired_cmds.extend(require.commands.iter().map(|x| x.clone()));
                }
            }
        }

        let is_desired_enum = |e: &Enum| {
            desired_enums.contains(&("GL_".to_string() + &e.ident)) ||
            desired_enums.contains(&("WGL_".to_string() + &e.ident)) ||
            desired_enums.contains(&("GLX_".to_string() + &e.ident)) ||
            desired_enums.contains(&("EGL_".to_string() + &e.ident))
        };

        let is_desired_cmd = |c: &Cmd| {
            desired_cmds.contains(&("gl".to_string() + &c.proto.ident)) ||
            desired_cmds.contains(&("wgl".to_string() + &c.proto.ident)) ||
            desired_cmds.contains(&("glX".to_string() + &c.proto.ident)) ||
            desired_cmds.contains(&("egl".to_string() + &c.proto.ident))
        };

        Registry {
            api: filter.api,
            enums: enums.into_iter().filter(is_desired_enum).collect(),
            cmds: cmds.into_iter().filter(is_desired_cmd).collect(),
            aliases: if filter.fallbacks == Fallbacks::None { HashMap::new() } else { aliases },
        }
    }

    fn consume_characters(&mut self) -> String {
        match self.next().unwrap() {
            ParseEvent::Text(ch) => ch,
            event => panic!("Expected characters, found: {:?}", event),
        }
    }

    fn consume_start_element(&mut self, expected_name: &str) -> Vec<(String, String)> {
        match self.next().unwrap() {
            ParseEvent::Start(name, attributes) => {
                if expected_name == name { attributes } else {
                    panic!("Expected <{}>, found: <{}>", expected_name, name)
                }
            }
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

    fn consume_two<'a, T: FromXml, U: FromXml>(&mut self, one: &'a str, two: &'a str, end: &'a str) -> (Vec<T>, Vec<U>) {
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
                event => panic!("Unexpected message {:?}", event) }
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
                    enums.push(
                        Enum {
                            ident:  trim_enum_prefix(&get_attribute(&attributes, "name").unwrap(), api).to_string(),
                            value:  get_attribute(&attributes, "value").unwrap(),
                            alias:  get_attribute(&attributes, "alias"),
                            ty:     get_attribute(&attributes, "type"),
                        }
                    );
                    self.consume_end_element("enum");
                }

                // finished building the namespace
                ParseEvent::End(ref name) if name == "enums" => break,
                // error handling
                event => panic!("Expected </enums>, found: {:?}", event),
            }
        }
        enums
    }

    fn consume_cmds(&mut self, api: Api) -> (Vec<Cmd>, HashMap<String, Vec<String>>) {
        let mut cmds = Vec::new();
        let mut aliases: HashMap<String, Vec<String>> = HashMap::new();
        loop {
            match self.next().unwrap() {
                // add command definition
                ParseEvent::Start(ref name, _) if name == "command" => {
                    let new = self.consume_cmd(api);
                    if let Some(ref v) = new.alias {
                        match aliases.entry(v.clone()) {
                            Entry::Occupied(mut ent) => { ent.get_mut().push(new.proto.ident.clone()); },
                            Entry::Vacant(ent) => { ent.insert(vec![new.proto.ident.clone()]); }
                        }
                    }
                    cmds.push(new);
                }
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
        let mut proto = self.consume_binding("proto");
        proto.ident = trim_cmd_prefix(&proto.ident, api).to_string();

        let mut params = Vec::new();
        let mut alias = None;
        let mut vecequiv = None;
        let mut glx = None;
        loop {
            match self.next().unwrap() {
                ParseEvent::Start(ref name, _) if name == "param" => {
                    params.push(self.consume_binding("param"));
                }
                ParseEvent::Start(ref name, ref attributes) if name == "alias" => {
                    alias = get_attribute(&attributes, "name");
                    alias = alias.map(|t| trim_cmd_prefix(&t, api).to_string());
                    self.consume_end_element("alias");
                }
                ParseEvent::Start(ref name, ref attributes) if name == "vecequiv" => {
                    vecequiv = get_attribute(&attributes, "vecequiv");
                    self.consume_end_element("vecequiv");
                }
                ParseEvent::Start(ref name, ref attributes) if name == "glx" => {
                    glx = Some(GlxOpcode {
                        ty: get_attribute(&attributes, "type").unwrap(),
                        opcode: get_attribute(&attributes, "opcode").unwrap(),
                        name: get_attribute(&attributes, "name"),
                    });
                    self.consume_end_element("glx");
                }
                ParseEvent::End(ref name) if name == "command" => break,
                event => panic!("Expected </command>, found: {:?}", event),
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

    fn consume_binding(&mut self, outside_tag: &str) -> Binding {
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
        let ident = self.consume_characters();
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
            ident: ident,
            ty: ty,
        }
    }
}

impl<T> Parse for T where
    T: Sized + Iterator<Item = ParseEvent>,
{}

fn get_attribute(attribs: &[(String, String)], name: &str) -> Option<String> {
    attribs.iter()
        .find(|attrib| attrib.0 == name)
        .map(|attrib| attrib.1.clone())
}

trait FromXml {
    fn convert<P: Parse>(parser: &mut P, a: &[(String, String)]) -> Self;
}

impl FromXml for Require {
    fn convert<P: Parse>(parser: &mut P, _: &[(String, String)]) -> Require {
        debug!("Doing a FromXml on Require");
        let (enums, commands) = parser.consume_two("enum", "command", "require");
        Require {
            enums: enums,
            commands: commands
        }
    }
}

impl FromXml for Remove {
    fn convert<P: Parse>(parser: &mut P, a: &[(String, String)]) -> Remove {
        debug!("Doing a FromXml on Remove");
        let profile = get_attribute(a, "profile").unwrap();
        let profile = profile_from_str(&profile).unwrap();
        let (enums, commands) = parser.consume_two("enum", "command", "remove");

        Remove {
            profile: profile,
            enums: enums,
            commands: commands
        }
    }
}

impl FromXml for Feature {
    fn convert<P: Parse>(parser: &mut P, a: &[(String, String)]) -> Feature {
        debug!("Doing a FromXml on Feature");
        let api = get_attribute(a, "api").unwrap();
        let api = api_from_str(&api).unwrap();
        let name = get_attribute(a, "name").unwrap();
        let number = get_attribute(a, "number").unwrap();

        debug!("Found api = {}, name = {}, number = {}", api, name, number);

        let (require, remove) = parser.consume_two("require", "remove", "feature");

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
    fn convert<P: Parse>(parser: &mut P, a: &[(String, String)]) -> Extension {
        debug!("Doing a FromXml on Extension");
        let name = get_attribute(a, "name").unwrap();
        let supported = get_attribute(a, "supported").unwrap()
            .split('|')
            .map(api_from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        let mut require = Vec::new();
        loop {
            match parser.next().unwrap() {
                ParseEvent::Start(ref name, ref attributes) if name == "require" => {
                    require.push(FromXml::convert(parser, &attributes));
                }
                ParseEvent::End(ref name) if name == "extension" => break,
                event => panic!("Unexpected message {:?}", event)
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
    fn convert<P: Parse>(_: &mut P, a: &[(String, String)]) -> String {
        get_attribute(a, "name").unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod parse_event {
        mod from_xml {
            use xml::attribute::OwnedAttribute;
            use xml::common::XmlVersion;
            use xml::name::OwnedName;
            use xml::namespace::Namespace;
            use xml::reader::XmlEvent;

            use registry::parse::ParseEvent;

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
                        ("attr1".to_string(), "val1".to_string()),
                        ("attr2".to_string(), "val2".to_string()),
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
