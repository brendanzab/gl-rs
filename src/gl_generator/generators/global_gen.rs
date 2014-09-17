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

#![experimental]

use registry::{Registry, Ns};
use super::ty;

pub struct GlobalGenerator;

impl super::Generator for GlobalGenerator {
    fn write<W: Writer>(&self, writer: &mut W, registry: &Registry, ns: Ns) {
        writeln!(writer, "{}", write_header());
        writeln!(writer, "{}", write_type_aliases(&ns));
        writeln!(writer, "{}", write_enums(registry));
        writeln!(writer, "{}", write_fns(registry));
        writeln!(writer, "{}", write_fnptr_struct_def());
        writeln!(writer, "{}", write_ptrs(registry));
        writeln!(writer, "{}", write_fn_mods(registry, &ns));
        writeln!(writer, "{}", write_failing_fns(registry));
        writeln!(writer, "{}", write_load_fn(registry));
    }
}

fn write_header() -> String {
    format!(
        "mod __gl_imports {{
            extern crate libc;
            pub use std::mem;
        }}"
    )
}

fn write_type_aliases(ns: &Ns) -> String {
    format!(
        "#[stable]
        pub mod types {{
            {}
        }}
        ",

        super::gen_type_aliases(ns)
    )
}

fn write_enums(registry: &Registry) -> String {
    registry.enum_iter().map(|e| {
        super::gen_enum_item(e, "types::")
    }).collect::<Vec<String>>().connect("\n")
}

fn write_fns(registry: &Registry) -> String {
    registry.cmd_iter().map(|c| {
        if c.is_safe {
            format!(
                "#[allow(non_snake_case)] #[allow(unused_variable)] #[allow(dead_code)]
                #[inline] #[unstable] pub fn {name}({params}){return_suffix} {{ \
                    unsafe {{ \
                        __gl_imports::mem::transmute::<_, extern \"system\" fn({types}){return_suffix}>\
                            (storage::{name}.f)({idents}) \
                    }} \
                }}",
                name = c.proto.ident,
                params = super::gen_param_list(c, true),
                types = super::gen_param_ty_list(c),
                return_suffix = super::gen_return_suffix(c),
                idents = super::gen_param_ident_list(c),
            )
        } else {
            format!(
                "#[allow(non_snake_case)] #[allow(unused_variable)] #[allow(dead_code)]
                #[inline] #[unstable] pub unsafe fn {name}({typed_params}){return_suffix} {{ \
                    __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) {return_suffix}>\
                        (storage::{name}.f)({idents}) \
                }}",
                name = c.proto.ident,
                typed_params = super::gen_param_list(c, true),
                return_suffix = super::gen_return_suffix(c),
                idents = super::gen_param_ident_list(c),
            )
        }
    }).collect::<Vec<String>>().connect("\n")
}

fn write_fnptr_struct_def() -> String {
    format!(
        "pub struct FnPtr {{
            f: *const __gl_imports::libc::c_void,
            is_loaded: bool,
        }}

        impl FnPtr {{
            pub fn new(ptr: *const __gl_imports::libc::c_void, failing_fn: *const __gl_imports::libc::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{ f: failing_fn, is_loaded: false }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}
        }}"
    )
}

fn write_ptrs(registry: &Registry) -> String {
    format!(
        "mod storage {{
            #![allow(non_snake_case)]
            use super::__gl_imports::libc;
            use super::failing;
            use super::FnPtr;

            {storages}
        }}",

        storages = registry.cmd_iter().map(|c| {
            format!(
                "pub static mut {name}: FnPtr = FnPtr {{ \
                    f: failing::{name} as *const libc::c_void, \
                    is_loaded: false \
                }};",
                name = c.proto.ident,
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}

fn write_fn_mods(registry: &Registry, ns: &Ns) -> String {
    registry.cmd_iter().map(|c| {
        format!(
            "#[unstable]
            #[allow(non_snake_case)]
            pub mod {0} {{
                use super::{{failing, storage}};
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {{
                    unsafe {{ storage::{0}.is_loaded }}
                }}

                #[allow(dead_code)]
                pub fn load_with(loadfn: |symbol: &str| -> *const super::__gl_imports::libc::c_void) {{
                    unsafe {{
                        storage::{0} = FnPtr::new(loadfn(\"{1}\"),
                            failing::{0} as *const super::__gl_imports::libc::c_void)
                    }}
                }}
            }}",
            c.proto.ident,
            super::gen_symbol_name(ns, c)
        )
    }).collect::<Vec<String>>().connect("\n")

    // TODO: this is a reliquate from an old code, I have no idea what it does
    // for c in self.registry.cmd_iter() {
    //     self.write_line(format!(
    //         "pub mod {name} {{ \
    //             #[inline] \
    //             pub fn is_loaded() -> bool {{ \
    //                 unsafe {{ ::storage::{name}.is_loaded }} \
    //             }} \
    //         }}",
    //         name = c.proto.ident,
    //     ).as_slice());
    // }
}

fn write_failing_fns(registry: &Registry) -> String {
    format!(
        "mod failing {{
            use super::types;
            use super::__gl_imports;

            {functions}
        }}",

        functions = registry.cmd_iter().map(|c| {
            format!(
                "#[allow(non_snake_case)] #[allow(unused_variable)] #[allow(dead_code)]
                pub extern \"system\" fn {name}({params}){return_suffix} {{ \
                    fail!(\"`{name}` was not loaded\") \
                }}",
                name = c.proto.ident,
                params = super::gen_param_list(c, true),
                return_suffix = super::gen_return_suffix(c)
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}

fn write_load_fn(registry: &Registry) -> String {
    format!(
        "/// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load_with(loadfn: |symbol: &str| -> *const __gl_imports::libc::c_void) {{
            {exprs}
        }}",

        exprs = registry.cmd_iter().map(|c| {
            format!("{}::load_with(|s| loadfn(s));", c.proto.ident)
        }).collect::<Vec<String>>().connect("\n")
    )
}
