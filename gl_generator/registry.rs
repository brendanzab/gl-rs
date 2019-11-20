use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt,
    hash::Hash,
    io,
    ops::{Add, AddAssign},
};

use crate::generators::Generator;

mod parse;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Api {
    Gl,
    Glx,
    Wgl,
    Egl,
    GlCore,
    Gles1,
    Gles2,
    Glsc2,
}

impl fmt::Display for Api {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Api::Gl => write!(fmt, "gl"),
            Api::Glx => write!(fmt, "glx"),
            Api::Wgl => write!(fmt, "wgl"),
            Api::Egl => write!(fmt, "egl"),
            Api::GlCore => write!(fmt, "glcore"),
            Api::Gles1 => write!(fmt, "gles1"),
            Api::Gles2 => write!(fmt, "gles2"),
            Api::Glsc2 => write!(fmt, "glsc2"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Fallbacks {
    Yes,
    No,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Profile {
    Core,
    Compatibility,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum {
    pub ident: String,
    pub value: String,
    pub cast: bool,
    pub alias: Option<String>,
    pub ty: Cow<'static, str>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Binding {
    pub ident: String,
    pub ty: Cow<'static, str>,
    pub group: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub ident: String,
    pub enums_type: Option<String>,
    pub enums: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cmd {
    pub proto: Binding,
    pub params: Vec<Binding>,
    pub alias: Option<String>,
    pub vecequiv: Option<String>,
    pub glx: Option<GlxOpcode>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlxOpcode {
    pub opcode: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registry {
    api: Api,
    version: (u8, u8),
    fallbacks: Fallbacks,
    enums: BTreeSet<Enum>,
    cmds: BTreeSet<Cmd>,
    aliases: BTreeMap<String, Vec<String>>,
    groups: BTreeMap<String, Group>,
    extensions: Vec<String>,
}

impl Registry {
    pub fn api(&self) -> Api {
        self.api
    }
    pub fn version(&self) -> (u8, u8) {
        self.version
    }
    pub fn fallbacks(&self) -> Fallbacks {
        self.fallbacks
    }
    pub fn enums(&self) -> &BTreeSet<Enum> {
        &self.enums
    }
    pub fn cmds(&self) -> &BTreeSet<Cmd> {
        &self.cmds
    }
    pub fn aliases(&self) -> &BTreeMap<String, Vec<String>> {
        &self.aliases
    }
    pub fn groups(&self) -> &BTreeMap<String, Group> {
        &self.groups
    }
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}

impl Registry {
    pub fn new<'a, Exts>(
        api: Api,
        version: (u8, u8),
        profile: Profile,
        fallbacks: Fallbacks,
        extensions: Exts,
    ) -> Registry
    where
        Exts: AsRef<[&'a str]>,
    {
        let (major, minor) = version;
        let extensions = extensions.as_ref().iter().map(<&str>::to_string).collect();

        let filter = parse::Filter {
            api,
            fallbacks,
            extensions,
            version: format!("{}.{}", major, minor),
            profile,
        };

        let src = match api {
            Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 | Api::Glsc2 => khronos_api::GL_XML,
            Api::Glx => khronos_api::GLX_XML,
            Api::Wgl => khronos_api::WGL_XML,
            Api::Egl => khronos_api::EGL_XML,
        };

        let mut registry = parse::from_xml(src, &filter, true);
        registry.version = version;
        registry.fallbacks = fallbacks;
        filter
            .extensions
            .iter()
            .for_each(|s| registry.extensions.push(s.to_string()));
        if filter.extensions.iter().any(|e| e.starts_with("GL_ANGLE_")) {
            registry += parse::from_xml(khronos_api::GL_ANGLE_EXT_XML, &filter, false);
        }
        if filter
            .extensions
            .iter()
            .any(|e| e.starts_with("EGL_ANGLE_"))
        {
            registry += parse::from_xml(khronos_api::EGL_ANGLE_EXT_XML, &filter, false);
        }
        registry
    }

    pub fn write_bindings<W, G>(&self, generator: G, output: &mut W) -> io::Result<()>
    where
        G: Generator,
        W: io::Write,
    {
        generator.write(&self, output)
    }

    /// Returns a set of all the types used in the supplied registry. This is
    /// useful for working out what conversions are needed for the specific
    /// registry.
    pub fn get_tys(&self) -> BTreeSet<&str> {
        let mut tys = BTreeSet::new();
        for def in &self.cmds {
            tys.insert(def.proto.ty.as_ref());
            for param in &def.params {
                tys.insert(param.ty.as_ref());
            }
        }
        tys
    }

    pub fn get_docs_for_cmd(&self, cmd: &Cmd) -> String {
        match self.api {
        Api::Gl => {
            format!("/// See [gl{name}](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/gl{name}.xhtml)", name = cmd.proto.ident)
        },
        Api::Gles2 => {
            if self.version() == (2, 0) {
              format!("/// See [gl{name}](https://www.khronos.org/registry/OpenGL-Refpages/es2.0/html/gl{name}.xhtml)", name = cmd.proto.ident)
            } else if self.version() == (3, 0) {
              format!("/// See [gl{name}](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/gl{name}.xhtml)", name = cmd.proto.ident)
            } else if self.version() == (3, 1) {
              format!("/// See [gl{name}](https://www.khronos.org/registry/OpenGL-Refpages/es3.1/html/gl{name}.xhtml)", name = cmd.proto.ident)
            } else if self.version() == (3, 2) {
              format!("/// See [gl{name}](https://www.khronos.org/registry/OpenGL-Refpages/es3/html/gl{name}.xhtml)", name = cmd.proto.ident)
            } else {
              String::new()
            }
        },
        // TODO: provide docs links for more types of API.
        _ => String::new(),
      }
    }
}

impl Add for Registry {
    type Output = Registry;

    fn add(mut self, other: Registry) -> Registry {
        self += other;
        self
    }
}

impl AddAssign for Registry {
    fn add_assign(&mut self, other: Self) {
        self.enums.extend(other.enums);
        self.cmds.extend(other.cmds);
        self.aliases.extend(other.aliases);
    }
}
