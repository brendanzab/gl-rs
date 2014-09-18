use registry::{Enum, Registry, Cmd, Binding, Ns};
use registry::{Gl, Gles1, Gles2, Wgl, Glx, Egl};
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

mod ty;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;

/// Trait for a bindings generator.
pub trait Generator {
    fn write(&self, &ExtCtxt, registry: &Registry, ns: Ns) -> Vec<P<ast::Item>>;
}

/// This function generates a `static name: type = value;` item.
fn gen_enum_item(ecx: &ExtCtxt, enm: &Enum, types_prefix: &str) -> P<ast::Item> {
    use syntax::ext::quote::rt::ExtParseUtils;

    // computing the name of the enum
    // if the original starts with a digit, adding an underscore prefix.
    let ident = if (enm.ident.as_slice().char_at(0)).is_digit() {
        format!("_{}", enm.ident)
    } else {
        enm.ident.clone()
    };

    // computing the type of the enum
    let ty = {
        // some enums have a value of the form `((Type)Value)` ; if this is the case, we need to
        //  replace the type of the enum (which is GLenum by default) by the type in the expression

        // matches `((a)b)`
        let regex = regex!(r"\(\((\w+)\).+\)");

        if (regex).is_match(enm.value.as_slice()) {
            // if the value is like `((Type)Value)`, then the type is `types::Type`
            regex.replace(enm.value.as_slice(), format!("{}$1", types_prefix).as_slice())

        } else if enm.value.as_slice().starts_with("\"") {
            // some values are of the form "Value" ; if this is the case, we use `&'static str`
            //  instead of `GLenum`
            "&'static str".to_string()

        } else {
            // some values are `TRUE` or `FALSE`, in which case we use `GLboolean` instead of
            //  `GLenum`
            match ident.as_slice() {
                "TRUE" | "FALSE" => format!("{}GLboolean", types_prefix),
                _ => match enm.ty {
                    Some(ref s) if s.as_slice() == "ull" => format!("{}GLuint64", types_prefix),
                    _ => format!("{}GLenum", types_prefix)
                }
            }
        }
    };

    // computing the value of the enum
    let value = {
        // similar to the type, some values are `((Type)Value)`
        // replacing "((Type)Value)" by "Value as types::Type"
        let regex = regex!(r"\(\((\w+)\)(.+)\)");
        regex.replace(enm.value.as_slice(), format!("$2 as {}$1", types_prefix).as_slice())
    };

    ecx.parse_item(format!("
        #[stable]
        #[allow(dead_code)]
        pub static {}: {} = {};"
    , ident, ty, value))
}

fn gen_type_aliases(ecx: &ExtCtxt, namespace: &Ns) -> Vec<P<ast::Item>> {
    let mut result = Vec::new();

    match *namespace {
        Gl | Gles1 | Gles2 => {
            result.push_all_move(ty::build_gl_aliases(ecx));
        }
        
        Glx => {
            result.push_all_move(ty::build_gl_aliases(ecx));
            result.push_all_move(ty::build_x_aliases(ecx));
            result.push_all_move(ty::build_glx_aliases(ecx));
        }

        Wgl => {
            result.push_all_move(ty::build_gl_aliases(ecx));
            result.push_all_move(ty::build_win_aliases(ecx));
            result.push_all_move(ty::build_wgl_aliases(ecx));
        }

        Egl => {
            result.push_all_move(ty::build_gl_aliases(ecx));
            result.push_all_move(ty::build_egl_aliases(ecx));
        }
    }

    result
}

/// Generates the list of `Arg`s that a `Cmd` requires.
pub fn gen_parameters(ecx: &ExtCtxt, cmd: &Cmd) -> Vec<ast::Arg> {
    use syntax::ext::build::AstBuilder;
    
    cmd.params.iter()
        .map(|binding| {
            // variable name of the binding
            let ident = match binding.ident.as_slice() {
                "in" => ecx.ident_of("in_"),
                "ref" => ecx.ident_of("ref_"),
                "type" => ecx.ident_of("type_"),
                ident => ecx.ident_of(ident),
            };

            // rust type of the binding
            let ty = ty::to_rust_ty(ecx, binding.ty.as_slice());

            // returning
            // TODO: don't use call_site()?
            ecx.arg(ecx.call_site(), ident, ty)
        })
        .collect()
}

/// Generates the Rust return type of a `Cmd`.
pub fn gen_return_type(ecx: &ExtCtxt, cmd: &Cmd) -> P<ast::Ty> {
    // turn the return type into a Rust type
    let ty = ty::to_rust_ty(ecx, cmd.proto.ty.as_slice());

    // but there is one more step: if the Rust type end with `c_void`, we replace it with `()`
    match ty.node {
        ast::TyPath(ref path, _ ,_)
            if path.segments.last().unwrap().identifier.as_str() == "c_void"
                => return quote_ty!(ecx, ()),
        _ => ()
    };

    ty
}

pub fn gen_symbol_name(ns: &Ns, cmd: &Cmd) -> String {
    (match *ns {
        Gl | Gles1 | Gles2 => "gl",
        Glx => "glX",
        Wgl => "wgl",
        Egl => "egl",
    }).to_string().append(cmd.proto.ident.as_slice())
}
