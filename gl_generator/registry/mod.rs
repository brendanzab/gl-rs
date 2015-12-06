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

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Add;
use std::slice::Iter;

use {Fallbacks, Api, Profile};

mod parse;

pub struct Registry {
    pub api: Api,
    pub enums: Vec<Enum>,
    pub cmds: Vec<Cmd>,
    pub aliases: HashMap<String, Vec<String>>,
}

impl Registry {
    pub fn new<'a, Exts>(api: Api, version: (u8, u8), profile: Profile, fallbacks: Fallbacks, extensions: Exts) -> Registry where
        Exts: AsRef<[&'a str]>,
    {
        let (major, minor) = version;
        let extensions = extensions.as_ref().iter().map(<&str>::to_string).collect();

        let filter = parse::Filter {
            fallbacks: fallbacks,
            extensions: extensions,
            version: format!("{}.{}", major, minor),
            profile: profile,
        };

        let src = match api {
            Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => khronos_api::GL_XML,
            Api::Glx => khronos_api::GLX_XML,
            Api::Wgl => khronos_api::WGL_XML,
            Api::Egl => khronos_api::EGL_XML,
        };

        parse::RegistryParser::parse(src, api, filter)
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
}

pub struct Cmd {
    pub proto: Binding,
    pub params: Vec<Binding>,
    pub alias: Option<String>,
    pub vecequiv: Option<String>,
    pub glx: Option<GlxOpcode>,
}

pub struct GlxOpcode {
    pub ty: String,
    pub opcode: String,
    pub name: Option<String>,
}
