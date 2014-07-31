// Copyright 2013-2014 The gl-rs developers. For a full listing of the authors,
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

use registry::*;
use ty;
use common;
use std::io::Writer;

static TAB_WIDTH: uint = 4;

pub struct StructGenerator<'a, W> {
    ns: Ns,
    writer: &'a mut W,
    registry: &'a Registry,
    indent: uint,
}

impl<'a, W: Writer> StructGenerator<'a, W> {
    fn new<'a>(writer: &'a mut W, registry: &'a Registry, ns: Ns) -> StructGenerator<'a, W> {
        StructGenerator {
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

    #[allow(unused_must_use)]
    fn write_str(&mut self, s: &str) {
        self.writer.write(s.as_bytes());
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

    fn write_enum(&mut self, enm: &Enum) {
        let ident = if (enm.ident.as_slice().char_at(0)).is_digit() {
            format!("_{}", enm.ident)
        } else {
            enm.ident.clone()
        };

        let ty = match ident.as_slice() {
            "TRUE" | "FALSE" => "GLboolean",
            _ => match enm.ty {
                Some(ref s) if s.as_slice() == "ull" => "GLuint64",
                _ => "GLenum"
            }
        };

        self.write_line(format!("pub static {}: {} = {};", ident, ty, enm.value).as_slice())
    }

    fn write_enums(&mut self) {
        for e in self.registry.enum_iter() {
            self.write_enum(e);
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
        let ns = self.ns.to_string();
        self.write_line(format!("#![crate_name = \"{}\"]", ns).as_slice());
        self.write_line("#![comment = \"An OpenGL function loader.\"]");
        self.write_line("#![license = \"ASL2\"]");
        self.write_line("#![crate_type = \"lib\"]");
        self.write_line("");
        self.write_line("#![feature(macro_rules)]");
        self.write_line("#![feature(globs)]");
        self.write_line("#![allow(non_camel_case_types)]");
        self.write_line("#![allow(non_snake_case_functions)]");
        self.write_line("");
        self.write_line("extern crate libc;");
        self.write_line("");
        self.write_line("use std::mem;");
        self.write_line("");
        self.write_line("use self::types::*;");
    }

    fn write_type_aliases(&mut self) {
        self.write_line("pub mod types {");
        self.incr_indent();
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
        self.write_line("pub struct FnPtr {");
        self.write_line("    f: *const ::libc::c_void,");
        self.write_line("    is_loaded: bool,");
        self.write_line("}");
        self.write_line("");
        self.write_line("impl FnPtr {");
        self.write_line("    fn new(ptr: *const ::libc::c_void, failing_fn: *const ::libc::c_void) -> FnPtr {");
        self.write_line("        if ptr.is_null() {");
        self.write_line("            FnPtr { f: failing_fn, is_loaded: false }");
        self.write_line("        } else {");
        self.write_line("            FnPtr { f: ptr, is_loaded: true }");
        self.write_line("        }");
        self.write_line("    }");
        self.write_line("");
        self.write_line("    #[inline]");
        self.write_line("    pub fn is_loaded(&self) -> bool {");
        self.write_line("        self.is_loaded");
        self.write_line("    }");
        self.write_line("}");
    }

    fn write_failing_fns(&mut self) {
        self.write_line("mod failing {");
        self.incr_indent();
        self.write_line("use super::types::*;");
        self.write_line("");
        for c in self.registry.cmd_iter() {
            self.write_line("#[allow(unused_variable)]");
            self.write_line(format!(
                "pub extern \"system\" fn {name}({params}){return_suffix} {{ \
                    fail!(\"`{name}` was not loaded\") \
                }}",
                name = c.proto.ident,
                params = common::gen_param_list(c, true),
                return_suffix = common::gen_return_suffix(c)
            ).as_slice());
        }
        self.decr_indent();
        self.write_line("}");
    }

    fn write_struct(&mut self) {
        let ns = self.ns;
        self.write_line("#[allow(uppercase_variables)]");
        self.write_line(format!("pub struct {:c} {{", ns).as_slice());
        self.incr_indent();
        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "{name}: FnPtr,",
                name = c.proto.ident,
            ).as_slice());
        }
        self.decr_indent();
        self.write_line("}");
    }

    fn write_impl(&mut self) {
        let ns = self.ns;
        self.write_line(format!("impl {:c} {{", ns).as_slice());
        self.incr_indent();
        let ns = self.ns;
        self.write_line("/// Load each OpenGL symbol using a custom load function. This allows for the");
        self.write_line("/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.");
        self.write_line("///");
        self.write_line("/// ~~~rust");
        self.write_line("/// let gl = Gl::load_with(|s| glfw.get_proc_address(s));");
        self.write_line("/// ~~~");
        self.write_line(format!(
            "pub fn load_with(loadfn: |symbol: &str| -> *const ::libc::c_void) -> {:c} {{", ns
        ).as_slice());
        self.incr_indent();
        self.write_line(format!("{:c} {{", ns).as_slice());
        self.incr_indent();
        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "{name}: FnPtr::new(loadfn(\"{symbol}\"), failing::{name} as *const ::libc::c_void),",
                name = c.proto.ident,
                symbol = common::gen_symbol_name(&ns, c)
            ).as_slice());
        }
        self.decr_indent();
        self.write_line("}");
        self.decr_indent();
        self.write_line("}");
        self.write_line("");
        for c in self.registry.cmd_iter() {
            self.write_line(
                if c.is_safe {
                    format!(
                        "#[inline] pub fn {name}(&self, {params}){return_suffix} {{ \
                            unsafe {{ \
                                mem::transmute::<_, extern \"system\" fn({types}){return_suffix}>\
                                    (self.{name}.f)({idents}) \
                            }} \
                        }}",
                        name = c.proto.ident,
                        params = common::gen_param_list(c, true),
                        types = common::gen_param_ty_list(c),
                        return_suffix = common::gen_return_suffix(c),
                        idents = common::gen_param_ident_list(c),
                    )
                } else {
                    format!(
                        "#[inline] pub unsafe fn {name}(&self, {typed_params}){return_suffix} {{ \
                            mem::transmute::<_, extern \"system\" fn({typed_params}) {return_suffix}>\
                                (self.{name}.f)({idents}) \
                        }}",
                        name = c.proto.ident,
                        typed_params = common::gen_param_list(c, true),
                        return_suffix = common::gen_return_suffix(c),
                        idents = common::gen_param_ident_list(c),
                    )
                }.as_slice()
            );
        }
        self.decr_indent();
        self.write_line("}");
    }

    pub fn write(writer: &mut W, registry: &Registry, ns: Ns, write_header: bool) {
        let mut gen = StructGenerator::new(writer, registry, ns);

        if write_header {
            // header with licence, metadata and imports
            gen.write_header();
            gen.write_line("");
        }

        // type aliases
        gen.write_type_aliases();
        gen.write_line("");

        // enums definitions
        gen.write_enums();
        gen.write_line("");

        // FnPtr struct def
        gen.write_fnptr_struct_def();
        gen.write_line("");

        // failing functions to assign to the function pointers
        gen.write_failing_fns();
        gen.write_line("");

        // struct for storing function pointers
        gen.write_struct();
        gen.write_line("");

        // opengl functions as methods on the struct
        gen.write_impl();
        gen.write_line("");
    }
}
