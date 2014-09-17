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
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ext::quote::rt::ExtParseUtils;
use syntax::ptr::P;

pub struct StructGenerator;

impl super::Generator for StructGenerator {
    fn write(&self, ecx: &ExtCtxt, registry: &Registry, ns: Ns) -> Vec<P<ast::Item>> {
        let mut result = Vec::new();
        result.push(write_header(ecx));
        result.push(write_type_aliases(ecx, &ns));
        result.push_all_move(write_enums(ecx, registry));
        result.push_all_move(write_fnptr_struct_def(ecx));
        result.push(write_failing_fns(ecx, registry));
        result.push(write_struct(ecx, registry, &ns));
        result.push(write_impl(ecx, registry, &ns));
        result
    }
}

fn write_header(ecx: &ExtCtxt) -> P<ast::Item> {
    (quote_item!(ecx,
        mod __gl_imports {
            extern crate libc;
            pub use std::mem;
        }
    )).unwrap()
}

fn write_type_aliases(ecx: &ExtCtxt, ns: &Ns) -> P<ast::Item> {
    let aliases = super::gen_type_aliases(ecx, ns);

    (quote_item!(ecx,
        #[stable]
        pub mod types {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            
            $aliases
        }
    )).unwrap()
}

fn write_enums(ecx: &ExtCtxt, registry: &Registry) -> Vec<P<ast::Item>> {
    registry.enum_iter().map(|e| {
        super::gen_enum_item(ecx, e, "types::")
    }).collect()
}

fn write_fnptr_struct_def(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    vec![
        (quote_item!(ecx,
            #[allow(dead_code)]
            pub struct FnPtr {
                f: *const __gl_imports::libc::c_void,
                is_loaded: bool,
            }
        )).unwrap(),

        (quote_item!(ecx,
            impl FnPtr {
                fn new(ptr: *const __gl_imports::libc::c_void,
                    failing_fn: *const __gl_imports::libc::c_void) -> FnPtr {
                    if ptr.is_null() {
                        FnPtr { f: failing_fn, is_loaded: false }
                    } else {
                        FnPtr { f: ptr, is_loaded: true }
                    }
                }

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded(&self) -> bool {
                    self.is_loaded
                }
            }
        )).unwrap()
    ]
}

fn write_failing_fns(ecx: &ExtCtxt, registry: &Registry) -> P<ast::Item> {
    let fns = registry.cmd_iter().map(|c| {
        ecx.parse_item(format!(r#"
            #[allow(unused_variable)]
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub extern "system" fn {name}({params}){return_suffix} {{
                fail!("`{name}` was not loaded")
            }}
            "#,
            name = c.proto.ident,
            params = super::gen_param_list(c, true),
            return_suffix = super::gen_return_suffix(c)
        ))
    }).collect::<Vec<P<ast::Item>>>();

    (quote_item!(ecx,
        mod failing {
            use super::types;
            use super::__gl_imports;

            $fns
        }
    )).unwrap()
}

fn write_struct(ecx: &ExtCtxt, registry: &Registry, ns: &Ns) -> P<ast::Item> {
    ecx.parse_item(format!("
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[stable]
        pub struct {ns:c} {{
            {ptrs}
        }}",

        ns = *ns,
        ptrs = registry.cmd_iter().map(|c| {
            format!(
                "pub {name}: FnPtr,",
                name = c.proto.ident,
            )
        }).collect::<Vec<String>>().connect("\n")
    ))
}

fn write_impl(ecx: &ExtCtxt, registry: &Registry, ns: &Ns) -> P<ast::Item> {
    ecx.parse_item(format!("
        impl {ns:c} {{
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[unstable]
            #[allow(dead_code)]
            #[allow(unused_variable)]
            pub fn load_with(loadfn: |symbol: &str| -> *const __gl_imports::libc::c_void) -> {ns:c} {{
                {ns:c} {{
                    {loadings}
                }}
            }}

            {modules}
        }}",

        ns = *ns,

        loadings = registry.cmd_iter().map(|c| {
            format!(
                "{name}: FnPtr::new(loadfn(\"{symbol}\"), failing::{name} as *const __gl_imports::libc::c_void),",
                name = c.proto.ident,
                symbol = super::gen_symbol_name(ns, c)
            )
        }).collect::<Vec<String>>().connect("\n"),

        modules = registry.cmd_iter().map(|c| {
            if c.is_safe {
                format!(
                    "#[allow(non_snake_case)] #[allow(unused_variable)] #[allow(dead_code)]
                    #[inline] #[unstable] pub fn {name}(&self, {params}){return_suffix} {{ \
                        unsafe {{ \
                            __gl_imports::mem::transmute::<_, extern \"system\" fn({types}){return_suffix}>\
                                (self.{name}.f)({idents}) \
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
                    #[inline] #[unstable] pub unsafe fn {name}(&self, {typed_params}){return_suffix} {{ \
                        __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) {return_suffix}>\
                            (self.{name}.f)({idents}) \
                    }}",
                    name = c.proto.ident,
                    typed_params = super::gen_param_list(c, true),
                    return_suffix = super::gen_return_suffix(c),
                    idents = super::gen_param_ident_list(c),
                )
            }
        }).collect::<Vec<String>>().connect("\n")
    ))
}
