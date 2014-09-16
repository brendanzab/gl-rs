use registry::*;

mod ty;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;

/// This function generates a `static name: type = value;` item.
fn gen_enum_item(enm: &Enum, types_prefix: &str) -> String {
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
        regex.replace(enm.value.as_slice(), format!("$1 as {}$0", types_prefix).as_slice())
    };

    format!("
        #[stable]
        #[allow(dead_code)]
        pub static {}: {} = {};"
    , ident, ty, value)
}

pub fn gen_binding_ident(binding: &Binding, use_idents: bool) -> String {
    // FIXME: use &'a str when https://github.com/mozilla/rust/issues/11869 is
    // fixed
    if use_idents {
        match binding.ident.as_slice() {
            "in" => "in_".to_string(),
            "ref" => "ref_".to_string(),
            "type" => "type_".to_string(),
            ident => ident.to_string(),
        }
    } else {
        "_".to_string()
    }
}

pub fn gen_binding(binding: &Binding, use_idents: bool) -> String {
    format!("{}: {}",
        gen_binding_ident(binding, use_idents),
        ty::to_rust_ty(binding.ty.as_slice()))
}

pub fn gen_param_list(cmd: &Cmd, use_idents: bool) -> String {
    cmd.params.iter()
        .map(|b| gen_binding(b, use_idents))
        .collect::<Vec<String>>()
        .connect(", ")
}

pub fn gen_param_ident_list(cmd: &Cmd) -> String {
    cmd.params.iter()
        .map(|b| gen_binding_ident(b, true))
        .collect::<Vec<String>>()
        .connect(", ")
}

pub fn gen_param_ty_list(cmd: &Cmd) -> String {
    cmd.params.iter()
        .map(|b| ty::to_rust_ty(b.ty.as_slice()))
        .collect::<Vec<&str>>()
        .connect(", ")
}

pub fn gen_return_suffix(cmd: &Cmd) -> String {
    ty::to_return_suffix(ty::to_rust_ty(cmd.proto.ty.as_slice()))
}

pub fn gen_symbol_name(ns: &Ns, cmd: &Cmd) -> String {
    (match *ns {
        Gl | Gles1 | Gles2 => "gl",
        Glx => "glX",
        Wgl => "wgl",
        Egl => "egl",
    }).to_string().append(cmd.proto.ident.as_slice())
}
