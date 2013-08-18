#[link(name = "glgen",
       author = "Brendan Zabarauskas",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];

//! Requires libxml2
//!
//! This will be used to generate the loader from the [registry xml files]
//! (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/):
//!
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/glx.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/wgl.xml`

extern mod extra;

use std::io;
use std::os;
use std::util;

use registry::*;

pub mod registry;
pub mod ty;

fn main() {
    match os::real_args() {
        [_, ref ns_str, ..args] => {
            let (path, ns) = match *ns_str {
                ~"gl" => (~"gl.xml", registry::Gl),
                ~"glx" => (~"glx.xml", registry::Glx),
                ~"wgl" => (~"wgl.xml", registry::Wgl),
                _ => fail!("Unexpected opengl namespace '%s'. Expected one of: gl, glx, or wgl.", *ns_str),
            };
            // Parse the XML registry.
            let reg = Registry::from_xml(
                io::file_reader(&Path(path))
                    .expect(fmt!("Could not read %s", path))
                    .read_c_str(),
                ns
            );
            parse_args(args, &reg, ns);
        }
        [_] => fail!("Error: expected an opengl namespace. Expected one of: gl, glx, or wgl."),
        [] => util::unreachable(),
    }
    // TODO: Use registry data to generate function loader.
}

fn parse_args(args: &[~str], reg: &Registry, ns: Ns) {
    match args {
        [~"ptr", .._] => {
            gen_ptr::write_loader(std::io::stdout(), reg, ns);
        }
        [~"struct", .._] => {
            gen_struct::write_loader(std::io::stdout(), reg, ns);
        }
        [ref flag] => {
            printfln!("Error: unexpected argument `%s`.", *flag);
        }
        [] => (),
    }
}

/// Print out the registry data for debugging.
fn print_registry(reg: &Registry) {
    printfln!("%?", reg);
}

/// Print out a list of all the types used in the registry.
fn print_ctys(reg: &Registry) {
    let tys = reg.get_tys();
    for ty in tys.iter() {
        printfln!("\"%s\"", *ty);
    }
}

/// Print out a list of all the types used in the registry, converted to their
/// Rust declaration syntax.
fn print_rtys(reg: &Registry) {
    let tys = reg.get_tys();
    for ty in tys.iter() {
        printfln!("\"%s\"", ty::to_rust_ty(*ty));
    }
}

pub mod gen {
    use super::ty;
    use registry::*;

    pub fn tab_str(n: uint) -> ~str {
        "    ".repeat(n)
    }

    pub fn enum_ident_str(enm: &Enum) -> ~str {
        if (enm.ident[0] as char).is_digit() {
            "_" + enm.ident
        } else {
            enm.ident.clone()
        }
    }

    pub fn enum_str(enm: &Enum, ty: &str) -> ~str {
        fmt!("pub static %s: %s = %s;",
            enum_ident_str(enm),
            ty, enm.value)
    }

    pub fn binding_ident_str(binding: &Binding, use_idents: bool) -> ~str {
        if use_idents {
            match binding.ident {
                ~"in" => ~"in_",
                ~"ref" => ~"ref_",
                ~"type" => ~"type_",
                ref ident => ident.clone(),
            }
        } else { ~"_" }
    }

    pub fn binding_str(binding: &Binding, use_idents: bool) -> ~str {
        fmt!("%s: %s",
            binding_ident_str(binding, use_idents),
            ty::to_rust_ty(binding.ty))
    }

    pub fn param_list_str(cmd: &Cmd, use_idents: bool) -> ~str {
        cmd.params.iter()
            .map(|b| binding_str(b, use_idents))
            .to_owned_vec()
            .connect(", ")
    }

    pub fn return_suffix_str(cmd: &Cmd) -> ~str {
        ty::to_return_suffix(
            ty::to_rust_ty(cmd.proto.ty)
        )
    }

    pub fn write_header(writer: @Writer, reg: &Registry, ns: Ns) {
        writer.write_line("use std::libc::*;");
        writer.write_line("use self::types::*;");
        writer.write_line("");
        writer.write_line("mod types {");
        writer.write_line(tab_str(1) + "use std::libc::*;");
        match ns {
            Gl => {
                for alias in ty::GL_ALIASES.iter()
                    { writer.write_line(tab_str(1) + *alias) }
            }
            Glx => {
                for alias in ty::X_ALIASES.iter()
                    { writer.write_line(tab_str(1) + *alias) }
                for alias in ty::GLX_ALIASES.iter()
                    { writer.write_line(tab_str(1) + *alias) }
            }
            Wgl => {
                for alias in ty::WIN_ALIASES.iter()
                    { writer.write_line(tab_str(1) + *alias) }
                for alias in ty::WGL_ALIASES.iter()
                    { writer.write_line(tab_str(1) + *alias) }
            }
        }
        writer.write_line("}");
        writer.write_line("");
        for ns in reg.enums.iter() {
            for def in ns.defs.iter() {
                writer.write_line(enum_str(def, "GLenum"));
            }
        }
    }
}

pub mod gen_ptr {
    use super::gen;
    use registry::*;

    pub fn static_mut_str(cmd: &Cmd) -> ~str {
        fmt!("pub static mut %s: extern \"C\" fn(%s)%s = failing::%s;",
            cmd.proto.ident,
            gen::param_list_str(cmd, true),
            gen::return_suffix_str(cmd),
            cmd.proto.ident)
    }

    pub fn is_loaded_str(cmd: &Cmd) -> ~str {
        fmt!("pub mod %s { pub static mut is_loaded: bool = false; }",
            cmd.proto.ident)
    }

    pub fn failing_fn_str(cmd: &Cmd) -> ~str {
        fmt!("extern \"C\" fn %s(%s)%s { fail!(\"%s was not loaded\") }",
            cmd.proto.ident,
            gen::param_list_str(cmd, false),
            gen::return_suffix_str(cmd),
            cmd.proto.ident)
    }

    pub fn load_statement_str(cmd: &Cmd) -> ~str {
        fmt!("match loadfn(\"%s\") { ptr if !ptr.is_null() => unsafe { %s = transmute(ptr); %s::is_loaded = true; }, _ => unsafe { %s::is_loaded = false; } }",
            cmd.proto.ident,
            cmd.proto.ident,
            cmd.proto.ident,
            cmd.proto.ident)
    }

    pub fn write_loader(writer: @Writer, reg: &Registry, ns: Ns) {
        gen::write_header(writer, reg, ns);
        writer.write_line("");

        // static muts for storing function pointers
        for def in reg.cmds[0].defs.iter() {
            writer.write_line(static_mut_str(def));
        }
        writer.write_line("");

        // static muts for storing the status of the function pointers
        for def in reg.cmds[0].defs.iter() {
            writer.write_line(is_loaded_str(def));
        }
        writer.write_line("");

        // failing functions to assign to the function pointers
        writer.write_line("mod failing {");
        for def in reg.cmds[0].defs.iter() {
            writer.write_line(gen::tab_str(1) + failing_fn_str(def));
        }
        writer.write_line("}");
        writer.write_line("");

        // loader function
        writer.write_line("/// Load each OpenGL symbol using a custom load function. This allows for the");
        writer.write_line("/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.");
        writer.write_line("///");
        writer.write_line("/// ~~~");
        writer.write_line("/// let gl = gl::load_with(glfw::get_proc_address);");
        writer.write_line("/// ~~~");
        writer.write_line("pub fn load_with(loadfn: &fn(symbol: &str) -> *c_void) {");
        writer.write_line("    use std::cast::transmute;");
        writer.write_line("");
        for def in reg.cmds[0].defs.iter() {
            writer.write_line(gen::tab_str(1) + load_statement_str(def));
        }
        writer.write_line("}");
    }
}

pub mod gen_struct {
    use super::gen;
    use registry::*;

    pub fn write_loader(writer: @Writer, reg: &Registry, ns: Ns) {
        gen::write_header(writer, reg, ns);
        writer.write_line("");
    }
}
