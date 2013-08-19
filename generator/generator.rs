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

use std::hashmap::HashMap;
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
            PtrGenerator::write(std::io::stdout(), reg, ns, 4);
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

struct Generator<'self> {
    ns: Ns,
    writer: @Writer,
    registry: &'self Registry,
    indent: uint,
    tab_width: uint,
}

impl<'self> Generator<'self> {
    fn new<'a>(writer: @Writer, reg: &'a Registry, ns: Ns, tab_width: uint) -> Generator<'a> {
        Generator {
            ns: ns,
            writer: writer,
            registry: &'a *reg,
            indent: 0,
            tab_width: tab_width,
        }
    }

    fn incr_indent(&mut self) {
        self.indent += 1;
    }

    fn decr_indent(&mut self) {
        if self.indent > 0 { self.indent -= 1 }
    }

    fn write_indent(&self) {
        do (self.tab_width * self.indent).times {
            self.writer.write_char(' ');
        }
    }

    fn write_line(&self, s: &str) {
        self.write_indent();
        self.writer.write_line(s);
    }

    fn write_comment(&self, s: &str) {
        self.write_line("// " + s);
    }

    fn write_doc_comment(&self, s: &str) {
        self.write_line("/// " + s);
    }

    fn for_enums(&self, fn_ns: &fn(&EnumNs), fn_unseen: &fn(&EnumNs, &Enum), fn_seen: &fn(&EnumNs, &Enum)) {
        let mut seen = HashMap::new();
        for ns in self.registry.enums.iter() {
            fn_ns(ns);
            for def in ns.defs.iter() {
                match seen.find(&def.ident) {
                    Some(seen_ns) => { fn_seen(*seen_ns, def); loop; }
                    None => fn_unseen(ns, def),
                }
                seen.insert(def.ident.clone(), ns);
            }
        }
    }

    fn gen_enum_ident(enm: &Enum) -> ~str {
        if (enm.ident[0] as char).is_digit() {
            "_" + enm.ident
        } else {
            enm.ident.clone()
        }
    }

    fn write_enum(&self, enm: &Enum, ty: &str) {
        self.write_line(
            fmt!("pub static %s: %s = %s;",
                Generator::gen_enum_ident(enm),
                ty, enm.value)
        )
    }

    fn write_enums(&self) {
        self.for_enums(
            |_| (),
            |_, e| self.write_enum(e, "GLenum"),
            |_, _| ()
        )
    }

    fn for_cmds(&self, fn_ns: &fn(&CmdNs), fn_unseen: &fn(&CmdNs, &Cmd), fn_seen: &fn(&CmdNs, &Cmd)) {
        let mut seen = HashMap::new();
        for ns in self.registry.cmds.iter() {
            fn_ns(ns);
            for def in ns.defs.iter() {
                match seen.find(&def.proto.ident) {
                    Some(seen_ns) => { fn_seen(*seen_ns, def); loop; }
                    None => fn_unseen(ns, def),
                }
                seen.insert(def.proto.ident.clone(), ns);
            }
        }
    }

    fn gen_binding_ident(binding: &Binding, use_idents: bool) -> ~str {
        if use_idents {
            match binding.ident {
                ~"in" => ~"in_",
                ~"ref" => ~"ref_",
                ~"type" => ~"type_",
                ref ident => ident.clone(),
            }
        } else { ~"_" }
    }

    fn gen_binding(binding: &Binding, use_idents: bool) -> ~str {
        fmt!("%s: %s",
            Generator::gen_binding_ident(binding, use_idents),
            ty::to_rust_ty(binding.ty))
    }

    fn gen_param_list(cmd: &Cmd, use_idents: bool) -> ~str {
        cmd.params.iter()
            .map(|b| Generator::gen_binding(b, use_idents))
            .to_owned_vec()
            .connect(", ")
    }

    fn gen_param_call_list(cmd: &Cmd) -> ~str {
        cmd.params.iter()
            .map(|b| Generator::gen_binding_ident(b, true))
            .to_owned_vec()
            .connect(", ")
    }

    fn gen_return_suffix(cmd: &Cmd) -> ~str {
        ty::to_return_suffix(ty::to_rust_ty(cmd.proto.ty))
    }

    fn write_header(&mut self) {
        // license
        self.write_comment("Copyright 2013 The gl-rs developers. For a full listing of the authors,");
        self.write_comment("refer to the AUTHORS file at the top-level directory of this distribution.");
        self.write_comment("");
        self.write_comment("Licensed under the Apache License, Version 2.0 (the \"License\");");
        self.write_comment("you may not use this file except in compliance with the License.");
        self.write_comment("You may obtain a copy of the License at");
        self.write_comment("");
        self.write_comment("    http://www.apache.org/licenses/LICENSE-2.0");
        self.write_comment("");
        self.write_comment("Unless required by applicable law or agreed to in writing, software");
        self.write_comment("distributed under the License is distributed on an \"AS IS\" BASIS,");
        self.write_comment("WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.");
        self.write_comment("See the License for the specific language governing permissions and");
        self.write_comment("limitations under the License.");
        self.write_line("");

        // crate metadata
        self.write_line(fmt!("#[link(name = \"%s\",", match self.ns { Gl => "gl", Glx => "glx", Wgl => "wgl" }));
        self.write_line("       author = \"Brendan Zabarauskas\",");
        self.write_line("       url = \"https://github.com/bjz/gl-rs\",");
        self.write_line("       vers = \"0.1\")];");
        self.write_line("#[comment = \"An OpenGL function loader.\"];");
        self.write_line("#[license = \"ASL2\"];");
        self.write_line("#[crate_type = \"lib\"];");
        self.write_line("");

        // imports
        self.write_line("use std::libc::*;");
        self.write_line("use self::types::*;");
        self.write_line("");

        // type aliases
        self.write_line("mod types {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("");
        match self.ns {
            Gl => {
                for alias in ty::GL_ALIASES.iter() { self.write_line(*alias) }
            }
            Glx => {
                for alias in ty::X_ALIASES.iter() { self.write_line(*alias) }
                for alias in ty::GLX_ALIASES.iter() { self.write_line(*alias) }
            }
            Wgl => {
                for alias in ty::WIN_ALIASES.iter() { self.write_line(*alias) }
                for alias in ty::WGL_ALIASES.iter() { self.write_line(*alias) }
            }
        }
        self.decr_indent();
        self.write_line("}");
        self.write_line("");

        // enums
        self.write_enums();
    }
}

struct PtrGenerator<'self>(Generator<'self>);

impl<'self> PtrGenerator<'self> {
    fn write_fns(&self) {
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(
                fmt!("#[inline] pub %sfn %s(%s)%s { %sstorage::%s(%s)%s }",
                    if c.is_safe { "" } else { "unsafe " },
                    c.proto.ident,
                    Generator::gen_param_list(c, true),
                    Generator::gen_return_suffix(c),
                    if !c.is_safe { "" } else { "unsafe { " },
                    c.proto.ident,
                    Generator::gen_param_call_list(c),
                    if !c.is_safe { "" } else { " }" })
            ),
            |_, _| ()
        );
    }

    fn write_ptrs(&mut self) {
        self.write_line("mod storage {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("use super::types::*;");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(
                fmt!("pub static mut %s: extern \"C\" fn(%s)%s = super::failing::%s;",
                    c.proto.ident,
                    Generator::gen_param_list(c, true),
                    Generator::gen_return_suffix(c),
                    c.proto.ident)
            ),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    fn write_is_loaded_mods(&self) {
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!("pub mod %s { pub static mut is_loaded: bool = false; }", c.proto.ident)),
            |_, _| ()
        );
    }

    fn write_failing_fns(&mut self) {
        self.write_line("mod failing {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("use super::types::*;");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(
                fmt!("pub extern \"C\" fn %s(%s)%s { fail!(\"%s was not loaded\") }",
                    c.proto.ident,
                    Generator::gen_param_list(c, false),
                    Generator::gen_return_suffix(c),
                    c.proto.ident)
            ),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    fn write_load_fn(&mut self) {
        self.write_line("/// Load each OpenGL symbol using a custom load function. This allows for the");
        self.write_line("/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.");
        self.write_line("///");
        self.write_line("/// ~~~");
        self.write_line("/// let gl = gl::load_with(glfw::get_proc_address);");
        self.write_line("/// ~~~");
        self.write_line("pub fn load_with(loadfn: &fn(symbol: &str) -> *c_void) {");
        self.incr_indent();
        self.write_line("use std::cast::transmute;");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(
                fmt!(
                    "match loadfn(\"%s\") { \
                        ptr if !ptr.is_null() => unsafe { storage::%s = transmute(ptr); %s::is_loaded = true; }, \
                        _ => unsafe { %s::is_loaded = false; } \
                    }",
                    c.proto.ident,
                    c.proto.ident,
                    c.proto.ident,
                    c.proto.ident
                )
            ),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    fn write(writer: @Writer, reg: &Registry, ns: Ns, tab_width: uint) {
        let mut gen = PtrGenerator(
            Generator::new(writer, reg, ns, tab_width)
        );

        gen.write_header();
        gen.write_line("");

        // safe and unsafe OpenGl functions
        gen.write_fns();
        gen.write_line("");

        // static muts for storing function pointers
        gen.write_ptrs();
        gen.write_line("");

        // static muts for storing the status of the function pointers
        gen.write_is_loaded_mods();
        gen.write_line("");

        // failing functions to assign to the function pointers
        gen.write_failing_fns();
        gen.write_line("");

        // loader function
        gen.write_load_fn();
        gen.write_line("");
    }
}

pub mod gen_struct {
    use registry::*;

    pub fn write_loader(_writer: @Writer, _reg: &Registry, _ns: Ns) {
        // gen::write_header(writer, reg, ns);
        // writer.write_line("");
    }
}
