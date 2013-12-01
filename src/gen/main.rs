// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

#[link(name = "glrsgen",
       author = "Brendan Zabarauskas",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];
#[license = "ASL2"];
#[feature(globs)];
#[feature(macro_rules)];

//! Requires libxml2
//!
//! This will be used to generate the loader from the [registry xml files]
//! (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/):
//!
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/glx.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/wgl.xml`

extern mod extra;

use extra::getopts::groups::*;

use std::os;
use std::path::Path;
use std::io;
use std::io::{File, Reader, Writer};
use std::str;

use registry::*;

pub mod registry;
pub mod ty;

fn main() {
    let opts = &[
        optopt("", "namespace", "OpenGL namespace (gl by default)", "gl|glx|wgl"),
        optopt("", "api", "API to generate bindings for (gl by default)", "gl|gles1|gles2"),
        optopt("", "profile", "Profile to generate (core by default)", "core|compatability"),
        optopt("", "version", "Version to generate bindings for (4.3 by default)", ""),
        optmulti("", "extension", "Extension to include", ""),
        optflag("", "full", "Generate API for all profiles, versions and extensions"),
    ];

    let args = match getopts(os::args(), opts) {
        Ok(a) => a,
        Err(x) => fail!("Error: {}\n{}", x.to_err_msg(), usage("generator", opts)),
    };

    let (path, ns) = match args.opt_str("namespace").unwrap_or(~"gl") {
        ~"gl"  => (Path::init("gl.xml"), Gl),
        ~"glx" => fail!("glx generation unimplemented"),
        ~"wgl" => fail!("wgl generation unimplemented"),
        ns     => fail!("Unexpected opengl namespace '{}'", ns)
    };

    let filter = if args.opt_present("full") {
        None
    } else {
        Some(Filter {
            extensions: args.opt_strs("extension"),
            profile: args.opt_str("profile").unwrap_or(~"core"),
            version: args.opt_str("version").unwrap_or(~"4.3"),
            api: args.opt_str("api").unwrap_or(~"gl"),
        })
    };

    let reg = Registry::from_xml(
        str::from_utf8_owned(
            File::open(&path)
                .expect(format!("Could not read {}", path.display()))
                .read_to_end()
            ), ns, filter
    );

    Generator::write(&mut io::stdout(), &reg, ns);
}

static TAB_WIDTH: uint = 4;

struct Generator<'self, W> {
    ns: Ns,
    writer: &'self mut W,
    registry: &'self Registry,
    indent: uint,
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
    format!("{}: {}",
        gen_binding_ident(binding, use_idents),
        ty::to_rust_ty(binding.ty))
}

fn gen_param_list(cmd: &Cmd, use_idents: bool) -> ~str {
    cmd.params.iter()
        .map(|b| gen_binding(b, use_idents))
        .to_owned_vec()
        .connect(", ")
}

fn gen_param_ident_list(cmd: &Cmd) -> ~str {
    cmd.params.iter()
        .map(|b| gen_binding_ident(b, true))
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

impl<'self, W: Writer> Generator<'self, W> {
    fn new<'a>(writer: &'a mut W, registry: &'a Registry, ns: Ns) -> Generator<'a, W> {
        Generator {
            ns: ns,
            writer: writer,
            registry: registry,
            indent: 0,
        }
    }

    fn incr_indent(&mut self) {
        self.indent += 1;
    }

    fn decr_indent(&mut self) {
        if self.indent > 0 { self.indent -= 1 }
    }

    fn write_str(&mut self, s: &str) {
        self.writer.write(s.as_bytes())
    }

    fn write_indent(&mut self) {
        for _ in range(0, TAB_WIDTH * self.indent) {
            self.write_str(" ");
        }
    }

    fn write_line(&mut self, s: &str) {
        self.write_indent();
        self.write_str(s);
        self.write_str("\n");
    }

    fn write_comment(&mut self, s: &str) {
        self.write_indent();
        self.write_str("// ");
        self.write_str(s);
        self.write_str("\n");
    }

    fn write_doc_comment(&mut self, s: &str) {
        self.write_indent();
        self.write_str("/// ");
        self.write_str(s);
        self.write_str("\n");
    }

    fn write_enum(&mut self, enm: &Enum, ty: &str) {
        let ident = if (enm.ident[0] as char).is_digit() {
            "_" + enm.ident
        } else {
            enm.ident.clone()
        };

        let ty = match ident {
            ~"TRUE" | ~"FALSE" => ~"GLboolean",
            _ => ty.to_owned(),
        };

        self.write_line(format!("pub static {}: {} = {};", ident, ty, enm.value))
    }

    fn write_enums(&mut self) {
        for e in self.registry.enum_iter() {
            self.write_enum(e, "GLenum");
        }
    }

    fn write_header(&mut self) {
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
        let ns = self.ns.to_str();
        self.write_line(format!("\\#[link(name = \"{}\",", ns));
        self.write_line("       author = \"Brendan Zabarauskas\",");
        self.write_line("       url = \"https://github.com/bjz/gl-rs\",");
        self.write_line("       vers = \"0.1\")];");
        self.write_line("#[comment = \"An OpenGL function loader.\"];");
        self.write_line("#[license = \"ASL2\"];");
        self.write_line("#[crate_type = \"lib\"];");
        self.write_line("");
        self.write_line("#[feature(macro_rules)];");
        self.write_line("#[feature(globs)];");
        self.write_line("");
        self.write_line("use std::libc::*;");
        self.write_line("use self::types::*;");
    }

    fn write_type_aliases(&mut self) {
        self.write_line("pub mod types {");
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
        self.write_line("    pub fn new(ptr: Option<extern \"system\" fn()>, failing_fn: F) -> FnPtr<F> {");
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
        self.write_line("    (fn $name:ident()) => (pub extern \"system\" fn $name() { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident() -> $ret_ty:ty) => (pub extern \"system\" fn $name() -> $ret_ty { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident($($arg_ty:ty),*)) => (pub extern \"system\" fn $name($(_: $arg_ty),*) { fail!(stringify!($name was not loaded)) });");
        self.write_line("    (fn $name:ident($($arg_ty:ty),*) -> $ret_ty:ty) => (pub extern \"system\" fn $name($(_: $arg_ty),*) -> $ret_ty { fail!(stringify!($name was not loaded)) });");
        self.write_line(")");
        self.write_line("");
        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "failing!(fn {}({}){})",
                c.proto.ident,
                gen_param_ty_list(c),
                gen_return_suffix(c)
            ));
        }
        self.decr_indent();
        self.write_line("}");
    }

    fn write_fns(&mut self) {
        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "\\#[inline] pub {}fn {}({}){} \\{ {}(storage::{}.f)({}){} \\}",
                if c.is_safe { "" } else { "unsafe " },
                c.proto.ident,
                gen_param_list(c, true),
                gen_return_suffix(c),
                if !c.is_safe { "" } else { "unsafe { " },
                c.proto.ident,
                gen_param_ident_list(c),
                if !c.is_safe { "" } else { " }" }
            ));
        }
    }

    fn write_ptrs(&mut self) {
        self.write_line("mod storage {");
        self.incr_indent();
        self.write_line("use std::libc::*;");
        self.write_line("use super::types::*;");
        self.write_line("");
        self.write_line("macro_rules! fn_ptr(");
        self.write_line("    (fn $name:ident()) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"system\" fn()> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident() -> $ret_ty:ty) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"system\" fn() -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident($($arg:ident : $arg_ty:ty),*)) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"system\" fn($($arg: $arg_ty),*)> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line("    (fn $name:ident($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty) => (");
        self.write_line("        pub static mut $name: ::FnPtr<extern \"system\" fn($($arg: $arg_ty),*) -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };");
        self.write_line("    );");
        self.write_line(")");
        self.write_line("");
        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "fn_ptr!(fn {}({}){})",
                c.proto.ident,
                gen_param_list(c, true),
                gen_return_suffix(c)
            ));
        };
        self.decr_indent();
        self.write_line("}");
    }

    fn write_fn_mods(&mut self) {
        self.write_line("macro_rules! fn_mod(");
        self.write_line("    ($name:ident, $sym:expr) => (");
        self.write_line("        pub mod $name {");
        self.write_line("            #[inline]");
        self.write_line("            pub fn is_loaded() -> bool { unsafe { ::storage::$name.is_loaded } }");
        self.write_line("            ");
        self.write_line("            #[inline]");
        self.write_line("            pub fn load_with(loadfn: |symbol: &str| -> Option<extern \"system\" fn()>) {");
        self.write_line("                unsafe { ::storage::$name = ::FnPtr::new(loadfn($sym), ::failing::$name) }");
        self.write_line("            }");
        self.write_line("        }");
        self.write_line("    )");
        self.write_line(")");
        self.write_line("");
        for c in self.registry.cmd_iter() {
            let ns = self.ns;
            self.write_line(format!(
                "fn_mod!({}, \"{}\")",
                c.proto.ident,
                gen_symbol_name(&ns, c)));
        }
    }

    fn write_load_fn(&mut self) {
        self.write_line("/// Load each OpenGL symbol using a custom load function. This allows for the");
        self.write_line("/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.");
        self.write_line("///");
        self.write_line("/// ~~~");
        self.write_line("/// let gl = gl::load_with(glfw::get_proc_address);");
        self.write_line("/// ~~~");
        self.write_line("pub fn load_with(loadfn: |symbol: &str| -> Option<extern \"system\" fn()>) {");
        self.incr_indent();
        for c in self.registry.cmd_iter() {
            self.write_line(format!("{}::load_with(|s| loadfn(s));", c.proto.ident))
        }
        self.decr_indent();
        self.write_line("}");
    }

    fn write(writer: &mut W, registry: &Registry, ns: Ns) {
        let mut gen = Generator::new(writer, registry, ns);

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
