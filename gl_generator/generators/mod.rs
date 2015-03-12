use registry::{Enum, Registry, Cmd, Ns};
use std::io;

mod ty;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;
pub mod static_struct_gen;

/// Trait for a bindings generator.
pub trait Generator {
    /// Builds the GL bindings.
    fn write<W>(&self, registry: &Registry, ns: Ns, dest: &mut W) -> io::Result<()> where W: io::Write;
}

/// This function generates a `const name: type = value;` item.
fn gen_enum_item<W>(enm: &Enum, types_prefix: &str, dest: &mut W) -> io::Result<()> where W: io::Write {
    // computing the name of the enum
    // if the original starts with a digit, adding an underscore prefix.
    let ident = if (enm.ident.char_at(0)).is_numeric() {
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
        #[stable]
        #[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const {}: {} = {}; \
    ", ident, ty, value)
}

/// Generates all the type aliases for a namespace.
///
/// Aliases are either `pub type = ...` or `#[repr(C)] pub struct ... { ... }` and contain all the
///  things that we can't obtain from the XML files.
pub fn gen_type_aliases<W>(namespace: &Ns, dest: &mut W) -> io::Result<()> where W: io::Write {
    match *namespace {
        Ns::Gl | Ns::Gles1 | Ns::Gles2 => {
            try!(ty::build_gl_aliases(dest));
        }

        Ns::Glx => {
            try!(ty::build_gl_aliases(dest));
            try!(ty::build_x_aliases(dest));
            try!(ty::build_glx_aliases(dest));
        }

        Ns::Wgl => {
            try!(ty::build_gl_aliases(dest));
            try!(ty::build_win_aliases(dest));
            try!(ty::build_wgl_aliases(dest));
        }

        Ns::Egl => {
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

    // ... but there is one more step: if the Rust type is `c_void`, we replace it with `()`
    if ty == "__gl_imports::libc::c_void" {
        return "()".to_string();
    }

    ty.to_string()
}

/// Generates the native symbol name of a `Cmd`.
///
/// Example results: `"glClear"`, `"wglCreateContext"`, etc.
pub fn gen_symbol_name(ns: &Ns, cmd: &str) -> String {
    (match *ns {
        Ns::Gl | Ns::Gles1 | Ns::Gles2 => "gl",
        Ns::Glx => "glX",
        Ns::Wgl => "wgl",
        Ns::Egl => "egl",
    }).to_string() + cmd
}
