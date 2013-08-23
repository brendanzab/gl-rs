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
            PtrGenerator::write(std::io::stdout(), reg, ns);
        }
        [~"struct", .._] => {
            StructGenerator::write(std::io::stdout(), reg, ns);
        }
        [ref flag] => {
            printfln!("Error: unexpected argument `%s`.", *flag);
        }
        [] => (),
    }
}

static TAB_WIDTH: uint = 4;

struct Generator<'self> {
    ns: Ns,
    writer: @Writer,
    registry: &'self Registry,
    indent: uint,
}

impl<'self> Generator<'self> {
    fn new<'a>(writer: @Writer, reg: &'a Registry, ns: Ns) -> Generator<'a> {
        Generator {
            ns: ns,
            writer: writer,
            registry: &'a *reg,
            indent: 0,
        }
    }

    fn incr_indent(&mut self) {
        self.indent += 1;
    }

    fn decr_indent(&mut self) {
        if self.indent > 0 { self.indent -= 1 }
    }

    fn write_indent(&self) {
        do (TAB_WIDTH * self.indent).times {
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
        self.write_line(fmt!(
            "pub static %s: %s = %s;",
            Generator::gen_enum_ident(enm),
            ty, enm.value
        ))
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

    fn gen_binding_ident<'a>(binding: &'a Binding, use_idents: bool) -> &'a str {
        if use_idents {
            match binding.ident.as_slice() {
                "in" => &'a "in_",
                "ref" => &'a "ref_",
                "type" => &'a "type_",
                ident => ident,
            }
        } else {
            &'a "_"
        }
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

    fn gen_param_ident_list(cmd: &Cmd) -> ~str {
        cmd.params.iter()
            .map(|b| Generator::gen_binding_ident(b, true))
            .to_owned_vec()
            .connect(", ")
    }

    fn gen_param_ty_list(cmd: &Cmd) -> ~str {
        cmd.params.iter()
            .map(|b| ty::to_rust_ty(b.ty))
            .to_owned_vec()
            .connect(", ")
    }

    fn gen_return_suffix(cmd: &Cmd) -> ~str {
        ty::to_return_suffix(ty::to_rust_ty(cmd.proto.ty))
    }

    fn gen_symbol_name(ns: &Ns, cmd: &Cmd) -> ~str {
        (match *ns {
            Gl => "gl",
            Glx => "glx",
            Wgl => "wgl",
        }) + cmd.proto.ident
    }

    fn write_header(&self) {
        self.write_line("// Copyright 2013 The gl-rs developers. For a full listing of the authors,");
        self.write_line("// refer to the AUTHORS file at the top-level directory of this distribution.");
        self.write_line("// ");
        self.write_line("// Licensed under the Apache License, Version 2.0 (the \"License\");");
        self.write_line("// you may not use this file except in compliance with the License.");
        self.write_line("// You may obtain a copy of the License at");
        self.write_line("// ");
        self.write_line("//     http://www.apache.org/licenses/LICENSE-2.0");
        self.write_line("// ");
        self.write_line("// Unless required by applicable law or agreed to in writing, software");
        self.write_line("// distributed under the License is distributed on an \"AS IS\" BASIS,");
        self.write_line("// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.");
        self.write_line("// See the License for the specific language governing permissions and");
        self.write_line("// limitations under the License.");
        self.write_line("");
        self.write_line(fmt!("#[link(name = \"%s\",", match self.ns { Gl => "gl", Glx => "glx", Wgl => "wgl" }));
        self.write_line("       author = \"Brendan Zabarauskas\",");
        self.write_line("       url = \"https://github.com/bjz/gl-rs\",");
        self.write_line("       vers = \"0.1\")];");
        self.write_line("#[comment = \"An OpenGL function loader.\"];");
        self.write_line("#[license = \"ASL2\"];");
        self.write_line("#[crate_type = \"lib\"];");
        self.write_line("");
        self.write_line("use std::libc::*;");
        self.write_line("use self::types::*;");
    }

    fn write_type_aliases(&mut self) {
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
    }

    fn write_fnptr_struct_def(&mut self) {
        self.write_line("pub struct FnPtr<F> { f: F, is_loaded: bool }");
        self.write_line("");
        self.write_line("impl<F> FnPtr<F> {");
        self.write_line("    pub fn new(ptr: Option<extern \"C\" fn()>, failing_fn: F) -> FnPtr<F> {");
        self.write_line("        use std::cast::transmute;");
        self.write_line("        match ptr {");
        self.write_line("            Some(p) => FnPtr { f: unsafe { transmute(p) }, is_loaded: true },");
        self.write_line("            None => FnPtr { f: failing_fn, is_loaded: false },");
        self.write_line("        }");
        self.write_line("    }");
        self.write_line("}");
    }

    fn write_failing_fns(&mut self) {
        self.write_line("mod failing {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("use super::types::*;");
        self.write_line("");
        self.write_line("macro_rules! failing(");
        self.write_line("    (fn $name:ident()) => (pub extern \"C\" fn $name() { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident() -> $ret_ty:ty) => (pub extern \"C\" fn $name() -> $ret_ty { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident($($arg_ty:ty),*)) => (pub extern \"C\" fn $name($(_: $arg_ty),*) { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident($($arg_ty:ty),*) -> $ret_ty:ty) => (pub extern \"C\" fn $name($(_: $arg_ty),*) -> $ret_ty { fail!(stringify!($name was not loaded)) });");
        self.write_line(")");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "failing!(fn %s(%s)%s)",
                c.proto.ident,
                Generator::gen_param_ty_list(c),
                Generator::gen_return_suffix(c)
            )),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }
}

struct PtrGenerator<'self>(Generator<'self>);

impl<'self> PtrGenerator<'self> {
    fn new<'a>(writer: @Writer, reg: &'a Registry, ns: Ns) -> PtrGenerator<'a> {
        PtrGenerator(
            Generator::new(writer, reg, ns)
        )
    }

    fn write_fns(&self) {
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "#[fixed_stack_segment] #[inline] pub %sfn %s(%s)%s { %s(storage::%s.f)(%s)%s }",
                if c.is_safe { "" } else { "unsafe " },
                c.proto.ident,
                Generator::gen_param_list(c, true),
                Generator::gen_return_suffix(c),
                if !c.is_safe { "" } else { "unsafe { " },
                c.proto.ident,
                Generator::gen_param_ident_list(c),
                if !c.is_safe { "" } else { " }" }
            )),
            |_, _| ()
        );
    }

    fn write_ptrs(&mut self) {
        self.write_line("mod storage {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("use super::types::*;");
        self.write_line("");
        self.write_line("macro_rules! fn_ptr(");
        self.write_line("    (fn $name:ident()) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"C\" fn() = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident() -> $ret_ty:ty) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"C\" fn() -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident($($arg:ident : $arg_ty:ty),*)) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"C\" fn($($arg: $arg_ty),*)> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"C\" fn($($arg: $arg_ty),*) -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line(")");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "fn_ptr!(fn %s(%s)%s)",
                c.proto.ident,
                Generator::gen_param_list(c, true),
                Generator::gen_return_suffix(c)
            )),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    fn write_fn_mods(&self) {
        self.write_line("macro_rules! fn_mod(");
        self.write_line("    ($name:ident, $sym:expr) => (");
        self.write_line("        pub mod $name {");
        self.write_line("            #[inline]");
        self.write_line("            pub fn is_loaded() -> bool { unsafe { ::storage::$name.is_loaded } }");
        self.write_line("            ");
        self.write_line("            #[inline]");
        self.write_line("            pub fn load_with(loadfn: &fn(symbol: &str) -> Option<extern \"C\" fn()>) {");
        self.write_line("                unsafe { ::storage::$name = ::FnPtr::new(loadfn($sym), ::failing::$name) }");
        self.write_line("            }");
        self.write_line("        }");
        self.write_line("    )");
        self.write_line(")");
        self.write_line("");
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "fn_mod!(%s, \"%s\")",
                c.proto.ident,
                Generator::gen_symbol_name(&self.ns, c))),
            |_, _| ()
        );
    }

    fn write_load_fn(&mut self) {
        self.write_line("/// Load each OpenGL symbol using a custom load function. This allows for the");
        self.write_line("/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.");
        self.write_line("///");
        self.write_line("/// ~~~");
        self.write_line("/// let gl = gl::load_with(glfw::get_proc_address);");
        self.write_line("/// ~~~");
        self.write_line("pub fn load_with(loadfn: &fn(symbol: &str) -> Option<extern \"C\" fn()>) {");
        self.write_line("");
        self.incr_indent();
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!("%s::load_with(|s| loadfn(s));", c.proto.ident)),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    fn write(writer: @Writer, reg: &Registry, ns: Ns) {
        let mut gen = PtrGenerator::new(writer, reg, ns);

        // header with licence, metadata and imports
        gen.write_header();
        gen.write_line("");

        // type aliases
        gen.write_type_aliases();
        gen.write_line("");

        // enums definitions
        gen.write_enums();
        gen.write_line("");

        // safe and unsafe OpenGl functions
        gen.write_fns();
        gen.write_line("");

        // FnPtr struct def
        gen.write_fnptr_struct_def();
        gen.write_line("");

        // static muts for storing function pointers
        gen.write_ptrs();
        gen.write_line("");

        // functions for querying the status of individual function pointers
        gen.write_fn_mods();
        gen.write_line("");

        // failing functions to assign to the function pointers
        gen.write_failing_fns();
        gen.write_line("");

        // loader function
        gen.write_load_fn();
        gen.write_line("");
    }
}

struct StructGenerator<'self>(Generator<'self>);

impl<'self> StructGenerator<'self> {
    fn new<'a>(writer: @Writer, reg: &'a Registry, ns: Ns) -> StructGenerator<'a> {
        StructGenerator(
            Generator::new(writer, reg, ns)
        )
    }

    fn gen_struct_name(ns: &Ns) -> &'static str {
        match *ns {
            Gl => "GL",
            Glx => "GLX",
            Wgl => "WGL",
        }
    }

    fn write_struct_def(&mut self) {
        self.write_line(fmt!("pub struct %s {", StructGenerator::gen_struct_name(&self.ns)));
        self.incr_indent();
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "%s: FnPtr<extern \"C\" fn(%s)%s>,",
                c.proto.ident,
                Generator::gen_param_list(c, true),
                Generator::gen_return_suffix(c)
            )),
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
        self.write_line(fmt!(
            "pub fn load_with(loadfn: &fn(symbol: &str) -> Option<extern \"C\" fn()>) -> ~%s {",
            StructGenerator::gen_struct_name(&self.ns)
        ));
        self.write_line("");
        self.write_line(fmt!("~%s {", StructGenerator::gen_struct_name(&self.ns)));
        self.incr_indent();
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "%s: FnPtr::new(loadfn(\"%s\"), failing::%s),",
                c.proto.ident,
                Generator::gen_symbol_name(&self.ns, c),
                c.proto.ident
            )),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
        self.decr_indent();
        self.write_line("}");
    }

    fn write_wrapper_fns(&mut self) {
        self.write_line(fmt!("impl %s {", StructGenerator::gen_struct_name(&self.ns)));
        self.incr_indent();
        self.for_cmds(
            |_| (),
            |_, c| self.write_line(fmt!(
                "#[fixed_stack_segment] #[inline] pub %sfn %s(%s%s)%s { %s(self.%s.f)(%s)%s }",
                if c.is_safe { "" } else { "unsafe " },
                c.proto.ident,
                if c.params.len() > 0 { "&self, " } else { "&self" },
                Generator::gen_param_list(c, true),
                Generator::gen_return_suffix(c),
                if !c.is_safe { "" } else { "unsafe { " },
                c.proto.ident,
                Generator::gen_param_ident_list(c),
                if !c.is_safe { "" } else { " }" }
            )),
            |_, _| ()
        );
        self.decr_indent();
        self.write_line("}");
    }

    pub fn write(writer: @Writer, reg: &Registry, ns: Ns) {
        let mut gen = StructGenerator::new(writer, reg, ns);

        // header with licence, metadata and imports
        gen.write_header();
        gen.write_line("");

        // type aliases
        gen.write_type_aliases();
        gen.write_line("");

        // enums definitions
        gen.write_enums();
        gen.write_line("");

        // FnPtr struct def
        gen.write_fnptr_struct_def();
        gen.write_line("");

        // pointer storage struct definition
        gen.write_struct_def();
        gen.write_line("");

        // failing fallback functions
        gen.write_failing_fns();
        gen.write_line("");

        // loader function
        gen.write_load_fn();
        gen.write_line("");

        // function wrappers
        gen.write_wrapper_fns();
        gen.write_line("");
    }
}
