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

// some stuff in here is dead if you don't turn on the unstable feature.
#![allow(dead_code)]

use crate::registry::{Api, Cmd, Enum, Registry};
use std::io;

pub mod global_gen;
pub mod static_gen;
pub mod static_struct_gen;
pub mod struct_gen;

/// Trait for a bindings generator.
///
/// See https://github.com/brendanzab/gl-rs/tree/master/gl_generator#generator-types
pub trait Generator {
    /// Builds the GL bindings.
    fn write(&self, registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>;
}

pub fn gen_struct_name(api: Api) -> &'static str {
    match api {
        Api::Gl => "Gl",
        Api::Glx => "Glx",
        Api::Wgl => "Wgl",
        Api::Egl => "Egl",
        Api::GlCore => "GlCore",
        Api::Gles1 => "Gles1",
        Api::Gles2 => "Gles2",
        Api::Glsc2 => "Glsc2",
    }
}

/// This function generates a `const name: type = value;` item.
pub fn gen_enum_item(enm: &Enum, types_prefix: &str, dest: &mut dyn io::Write) -> io::Result<()>
{
    // TODO: remove the types_prefix thing? Seems silly, doesn't seem user
    // configurable.
    writeln!(
        dest,
        "pub const {ident}: {types_prefix}{ty} = {value}{cast_suffix};",
        ident = enm.ident,
        types_prefix = if enm.ty == "&'static str" {
            ""
        } else {
            types_prefix
        },
        ty = enm.ty,
        value = enm.value,
        cast_suffix = if enm.cast {
            format!(" as {}{}", types_prefix, enm.ty)
        } else {
            String::new()
        },
    )
}

/// Generates all the type aliases for a namespace.
///
/// Aliases are either `pub type = ...` or `#[repr(C)] pub struct ... { ... }`
/// and contain all the things that we can't obtain from the XML files.
pub fn gen_types(api: Api, dest: &mut dyn io::Write) -> io::Result<()>
{
    if let Api::Egl = api {
        writeln!(
            dest,
            "{}",
            include_str!("generators/templates/types/egl.rs")
        )?;
        return Ok(());
    }

    writeln!(dest, "{}", include_str!("generators/templates/types/gl.rs"))?;

    match api {
        Api::Glx => writeln!(
            dest,
            "{}",
            include_str!("generators/templates/types/glx.rs")
        )?,
        Api::Wgl => writeln!(
            dest,
            "{}",
            include_str!("generators/templates/types/wgl.rs")
        )?,
        _ => {},
    }

    Ok(())
}

/// Generates the list of Rust `Arg`s that a `Cmd` requires.
pub fn gen_parameters(cmd: &Cmd, with_idents: bool, with_types: bool) -> Vec<String> {
    cmd.params
        .iter()
        .map(|binding| {
            // returning
            if with_idents && with_types {
                format!("{}: {}", binding.ident, binding.ty)
            } else if with_types {
                binding.ty.to_string()
            } else if with_idents {
                binding.ident.to_string()
            } else {
                panic!()
            }
        })
        .collect()
}

/// Generates the native symbol name of a `Cmd`.
///
/// Example results: `"glClear"`, `"wglCreateContext"`, etc.
pub fn gen_symbol_name(api: Api, cmd: &str) -> String {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 | Api::Glsc2 => format!("gl{}", cmd),
        Api::Glx => format!("glX{}", cmd),
        Api::Wgl => format!("wgl{}", cmd),
        Api::Egl => format!("egl{}", cmd),
    }
}

/// Writes all types into their own sub-module.
pub fn write_type_aliases(registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
{
    writeln!(
        dest,
        r#"pub use types::*;
    pub mod types {{
      use super::*;"#
    )?;

    gen_types(registry.api(), dest)?;

    writeln!(dest, "}}")
}

/// Writes all consts into their own sub-module.
fn write_enums(registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
{
    writeln!(
        dest,
        r#"pub use consts::*;
    pub mod consts {{
      use super::*;"#
    )?;

    for enm in registry.enums() {
        gen_enum_item(enm, "", dest)?;
    }

    writeln!(dest, "}}")
}
