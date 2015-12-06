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

use Api;
use registry::{Enum, Registry, Cmd};
use std::io;

pub mod ty;
pub mod debug_struct_gen;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;
pub mod static_struct_gen;

/// Trait for a bindings generator.
pub trait Generator {
    /// Builds the GL bindings.
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write;
}

pub fn gen_struct_name(api: Api) -> &'static str {
    match api {
        Api::Gl  => "Gl",
        Api::Glx => "Glx",
        Api::Wgl => "Wgl",
        Api::Egl => "Egl",
        Api::GlCore => "GlCore",
        Api::Gles1 => "Gles1",
        Api::Gles2 => "Gles2",
    }
}

/// This function generates a `const name: type = value;` item.
pub fn gen_enum_item<W>(enm: &Enum, types_prefix: &str, dest: &mut W) -> io::Result<()> where W: io::Write {
    // computing the name of the enum
    // if the original starts with a digit, adding an underscore prefix.
    let ident = if (enm.ident.chars().next().unwrap()).is_numeric() {
        format!("_{}", enm.ident)
    } else {
        enm.ident.clone()
    };

    // if the enum has the value of the form `((Type)Value)`, then `val_regexed` contains `(Type, Value)`
    let val_regexed = {
        if enm.value.starts_with("((") && enm.value.ends_with(")") {
            let separator = enm.value.chars().skip(2).position(|c| c == ')').unwrap();
            Some((&enm.value[2 .. separator + 2], enm.value[separator + 3 ..].trim_matches(')')))
        } else {
            None
        }
    };

    // computing the type of the enum
    let ty = {
        // some enums have a value of the form `((Type)Value)` ; if this is the case, we need to
        //  replace the type of the enum (which is GLenum by default) by the type in the expression
        if let Some((ty, _)) = val_regexed {
            // if the value is like `((Type)Value)`, then the type is `types::Type`
            format!("{}{}", types_prefix, ty)

        } else if enm.value.starts_with("\"") {
            // some values are of the form "Value" ; if this is the case, we use `&'static str`
            //  instead of `GLenum`
            "&'static str".to_string()

        } else {
            // some values are `TRUE` or `FALSE`, in which case we use `GLboolean` instead of
            //  `GLenum`
            match &ident[..] {
                "TRUE" | "FALSE" => format!("{}GLboolean", types_prefix),
                _ => match enm.ty {
                    Some(ref s) if &s[..] == "ull" => format!("{}GLuint64", types_prefix),
                    _ => format!("{}GLenum", types_prefix)
                }
            }
        }
    };

    // computing the value of the enum
    let value = {
        // similar to the type, some values are `((Type)Value)`
        // replacing "((Type)Value)" by "Value as types::Type"
        if let Some((ty, val)) = val_regexed {
            format!("{} as {}{}", val, types_prefix, ty)
        } else {
            enm.value.clone()
        }
    };

    writeln!(dest, "\
        #[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const {}: {} = {}; \
    ", ident, ty, value)
}

/// Generates all the type aliases for a namespace.
///
/// Aliases are either `pub type = ...` or `#[repr(C)] pub struct ... { ... }` and contain all the
///  things that we can't obtain from the XML files.
pub fn gen_type_aliases<W>(api: Api, dest: &mut W) -> io::Result<()> where W: io::Write {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => {
            try!(ty::build_gl_aliases(dest));
        }

        Api::Glx => {
            try!(ty::build_gl_aliases(dest));
            try!(ty::build_x_aliases(dest));
            try!(ty::build_glx_aliases(dest));
        }

        Api::Wgl => {
            try!(ty::build_gl_aliases(dest));
            try!(ty::build_win_aliases(dest));
            try!(ty::build_wgl_aliases(dest));
        }

        Api::Egl => {
            try!(ty::build_gl_aliases(dest));
            try!(ty::build_egl_aliases(dest));
        }
    }

    Ok(())
}

/// Generates the list of Rust `Arg`s that a `Cmd` requires.
pub fn gen_parameters(cmd: &Cmd, with_idents: bool, with_types: bool) -> Vec<String> {
    cmd.params.iter()
        .map(|binding| {
            // variable name of the binding
            let ident = match &binding.ident[..] {
                "in" => "in_",
                "ref" => "ref_",
                "type" => "type_",
                ident => ident,
            };

            // rust type of the binding
            let ty = ty::to_rust_ty(&binding.ty[..]);

            // returning
            if with_idents && with_types {
                format!("{}: {}", ident, ty)
            } else if with_types {
                format!("{}", ty)
            } else if with_idents {
                format!("{}", ident)
            } else {
                panic!()
            }
        })
        .collect()
}

/// Generates the Rust return type of a `Cmd`.
pub fn gen_return_type(cmd: &Cmd) -> String {
    // turn the return type into a Rust type
    let ty = ty::to_rust_ty(&cmd.proto.ty);
    ty.to_string()
}

/// Generates the native symbol name of a `Cmd`.
///
/// Example results: `"glClear"`, `"wglCreateContext"`, etc.
pub fn gen_symbol_name(api: Api, cmd: &str) -> String {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => format!("gl{}", cmd),
        Api::Glx => format!("glX{}", cmd),
        Api::Wgl => format!("wgl{}", cmd),
        Api::Egl => format!("egl{}", cmd),
    }
}
