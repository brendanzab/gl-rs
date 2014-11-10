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

pub struct GlobalGenerator;

impl super::Generator for GlobalGenerator {
    fn write(&self, ecx: &ExtCtxt, registry: &Registry, ns: Ns) -> Vec<P<ast::Item>> {
        let mut result = Vec::new();
        result.push(write_header(ecx));
        result.push(write_metaloadfn(ecx));
        result.push(write_type_aliases(ecx, &ns));
        result.extend(write_enums(ecx, registry).into_iter());
        result.extend(write_fns(ecx, registry).into_iter());
        result.extend(write_fnptr_struct_def(ecx).into_iter());
        result.push(write_ptrs(ecx, registry));
        result.extend(write_fn_mods(ecx, registry, &ns).into_iter());
        result.push(write_failing_fns(ecx, registry));
        result.extend(write_load_fn(ecx, registry).into_iter());
        result
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header(ecx: &ExtCtxt) -> P<ast::Item> {
    (quote_item!(ecx,
        mod __gl_imports {
            extern crate gl_common;
            extern crate libc;
            pub use std::mem;
        }
    )).unwrap()
}

/// Creates the metaloadfn function for fallbacks
fn write_metaloadfn(ecx: &ExtCtxt) -> P<ast::Item> {
    (quote_item!(ecx,
        fn metaloadfn(loadfn: |&str| -> *const __gl_imports::libc::c_void,
                      symbol: &str,
                      fallbacks: &[&str]) -> *const __gl_imports::libc::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in fallbacks.iter() {
                    ptr = loadfn(sym);
                    if !ptr.is_null() { break; }
                }
            }
            ptr
        }
    )).unwrap()
}

/// Creates a `types` module which contains all the type aliases.
///
/// See also `generators::gen_type_aliases`.
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

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums(ecx: &ExtCtxt, registry: &Registry) -> Vec<P<ast::Item>> {
    registry.enum_iter().map(|e| {
        super::gen_enum_item(ecx, e, "types::")
    }).collect()
}

/// Creates the functions corresponding to the GL commands.
///
/// The function calls the corresponding function pointer stored in the `storage` module created
///  by `write_ptrs`.
fn write_fns(ecx: &ExtCtxt, registry: &Registry) -> Vec<P<ast::Item>> {
    registry.cmd_iter().map(|c| {
        use syntax::ext::quote::rt::ToSource;

        let doc = match registry.aliases.get(&c.proto.ident) {
            Some(v) => format!("/** Fallbacks: {} */", v.connect(", ")),
            None => "".to_string()
        };

        ecx.parse_item(
            format!(
                "#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
                #[inline] #[unstable] {doc} pub unsafe fn {name}({typed_params}) -> {return_suffix} {{ \
                    __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> {return_suffix}>\
                        (storage::{name}.f)({idents}) \
                }}",
                name = c.proto.ident,
                doc = doc,
                typed_params = super::gen_parameters(ecx, c).into_iter().map(|p| p.to_source()).collect::<Vec<String>>().connect(", "),
                return_suffix = super::gen_return_type(ecx, c).to_source(),
                idents = super::gen_parameters(ecx, c).into_iter().map(|p| p.pat.to_source()).collect::<Vec<String>>().connect(", "),
            )
        )
    }).collect()
}

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    let mut result = Vec::new();
    
    result.push((quote_item!(ecx,
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::libc::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }
    )).unwrap());

    result.push((quote_item!(ecx,
        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::libc::c_void, failing_fn: *const __gl_imports::libc::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr { f: failing_fn, is_loaded: false }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }
        }
    )).unwrap());

    result
}

/// Creates a `storage` module which contains a static `FnPtr` per GL command in the registry.
fn write_ptrs(ecx: &ExtCtxt, registry: &Registry) -> P<ast::Item> {
    let storages = registry.cmd_iter().map(|c| {
        ecx.parse_item(format!(
            "pub static mut {name}: FnPtr = FnPtr {{
                f: failing::{name} as *const libc::c_void,
                is_loaded: false
            }};",
            name = c.proto.ident
        ))
    }).collect::<Vec<P<ast::Item>>>();

    (quote_item!(ecx,
        mod storage {
            #![allow(non_snake_case)]
            use super::__gl_imports::libc;
            use super::failing;
            use super::FnPtr;

            $storages
        }
    )).unwrap()
}

/// Creates one module for each GL command.
///
/// Each module contains `is_loaded` and `load_with` which interact with the `storage` module
///  created by `write_ptrs`.
fn write_fn_mods(ecx: &ExtCtxt, registry: &Registry, ns: &Ns) -> Vec<P<ast::Item>> {
    registry.cmd_iter().map(|c| {
        let fallbacks = match registry.aliases.get(&c.proto.ident) {
            Some(v) => {
                let names = v.iter().map(|name| format!("\"{}\"", super::gen_symbol_name(ns, name.as_slice()))).collect::<Vec<_>>();
                format!("[{}]", names.connect(", "))
            }, None => "[]".to_string(),
        };
        let fallbacks = ecx.parse_expr(fallbacks);
        let fnname = ecx.ident_of(c.proto.ident.as_slice());
        let symbol = super::gen_symbol_name(ns, c.proto.ident.as_slice());
        let symbol = symbol.as_slice();

        (quote_item!(ecx,
            #[unstable]
            #[allow(non_snake_case)]
            pub mod $fnname {
                use super::{failing, storage, metaloadfn};
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::$fnname.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with(loadfn: |symbol: &str| -> *const super::__gl_imports::libc::c_void) {
                    unsafe {
                        storage::$fnname = FnPtr::new(metaloadfn(loadfn, $symbol, $fallbacks),
                            failing::$fnname as *const super::__gl_imports::libc::c_void)
                    }
                }
            }
        )).unwrap()
    }).collect()

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

/// Creates a `failing` module which contains one function per GL command.
///
/// These functions are the mocks that are called if the real function could not be loaded.
fn write_failing_fns(ecx: &ExtCtxt, registry: &Registry) -> P<ast::Item> {
    use syntax::ext::quote::rt::ToSource;

    let functions = registry.cmd_iter().map(|c| {
        ecx.parse_item(format!(
            "#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            pub extern \"system\" fn {name}({params}) -> {return_suffix} {{ \
                panic!(\"`{name}` was not loaded\") \
            }}",
            name = c.proto.ident,
            params = super::gen_parameters(ecx, c).into_iter().map(|p| p.to_source()).collect::<Vec<String>>().connect(", "),
            return_suffix = super::gen_return_type(ecx, c).to_source()
        ))
    }).collect::<Vec<P<ast::Item>>>();

    (quote_item!(ecx,
        mod failing {
            use super::types;
            use super::__gl_imports;

            $functions
        }
    )).unwrap()
}

/// Creates the `load_with` function.
///
/// The function calls `load_with` in each module created by `write_fn_mods`.
fn write_load_fn(ecx: &ExtCtxt, registry: &Registry) -> Vec<P<ast::Item>> {
    let loadings = registry.cmd_iter().map(|c| {
        let cmd_name = ecx.ident_of(c.proto.ident.as_slice());
        quote_stmt!(ecx, $cmd_name::load_with(|s| loadfn(s));)
    }).collect::<Vec<P<ast::Stmt>>>();

    let a = (quote_item!(ecx,
        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load_with(loadfn: |symbol: &str| -> *const __gl_imports::libc::c_void) {
            $loadings
        }
    )).unwrap();

    let b = (quote_item!(ecx,
        /// Load each OpenGL symbol using a custom load function.
        ///
        /// ~~~ignore
        /// gl::load(&glfw);
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) {
            load_with(|name| loader.get_proc_addr(name));
        }
    )).unwrap();

    vec![a, b]
}
