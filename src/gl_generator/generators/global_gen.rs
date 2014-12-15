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

#[allow(missing_copy_implementations)]
pub struct GlobalGenerator;

impl super::Generator for GlobalGenerator {
    fn write(&self, registry: &Registry, ns: Ns) -> String {
        let mut result = Vec::new();
        result.push(write_header());
        result.push(write_metaloadfn());
        result.push(write_type_aliases(&ns));
        result.push(write_enums(registry));
        result.push(write_fns(registry));
        result.push(write_fnptr_struct_def());
        result.push(write_ptrs(registry));
        result.push(write_fn_mods(registry, &ns));
        result.push(write_failing_fns(registry));
        result.push(write_load_fn(registry));
        result.connect("\n")
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header() -> String {
    format!(r#"
        mod __gl_imports {{
            extern crate gl_common;
            extern crate libc;
            pub use std::mem;
        }}
    "#)
}

/// Creates the metaloadfn function for fallbacks
fn write_metaloadfn() -> String {
    format!(r#"
        fn metaloadfn(loadfn: |&str| -> *const __gl_imports::libc::c_void,
                      symbol: &str,
                      fallbacks: &[&str]) -> *const __gl_imports::libc::c_void {{
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {{
                for &sym in fallbacks.iter() {{
                    ptr = loadfn(sym);
                    if !ptr.is_null() {{ break; }}
                }}
            }}
            ptr
        }}
    "#)
}

/// Creates a `types` module which contains all the type aliases.
///
/// See also `generators::gen_type_aliases`.
fn write_type_aliases(ns: &Ns) -> String {
    let aliases = super::gen_type_aliases(ns);

    format!(r#"
        #[stable]
        pub mod types {{
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            #![allow(missing_copy_implementations)]

            {aliases}
        }}
    "#, aliases = aliases)
}

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums(registry: &Registry) -> String {
    registry.enum_iter().map(|e| {
        super::gen_enum_item(e, "types::")
    }).collect::<Vec<String>>().connect("\n")
}

/// Creates the functions corresponding to the GL commands.
///
/// The function calls the corresponding function pointer stored in the `storage` module created
///  by `write_ptrs`.
fn write_fns(registry: &Registry) -> String {
    registry.cmd_iter().map(|c| {
        let doc = match registry.aliases.get(&c.proto.ident) {
            Some(v) => format!("/** Fallbacks: {} */", v.connect(", ")),
            None => "".to_string()
        };

        format!(
            "#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] #[unstable] {doc} pub unsafe fn {name}({params}) -> {return_suffix} {{ \
                __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> {return_suffix}>\
                    (storage::{name}.f)({idents}) \
            }}",
            name = c.proto.ident,
            doc = doc,
            params = super::gen_parameters(c, true, true).connect(", "),
            typed_params = super::gen_parameters(c, false, true).connect(", "),
            return_suffix = super::gen_return_type(c),
            idents = super::gen_parameters(c, true, false).connect(", "),
        )
    }).collect::<Vec<String>>().connect("\n")
}

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def() -> String {
    let mut result = Vec::new();

    result.push(format!(r#"
        #[allow(missing_copy_implementations)]
        pub struct FnPtr {{
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::libc::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }}
    "#));

    result.push(format!(r#"
        impl FnPtr {{
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::libc::c_void, failing_fn: *const __gl_imports::libc::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{ f: failing_fn, is_loaded: false }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}
        }}
    "#));

    result.connect("\n")
}

/// Creates a `storage` module which contains a static `FnPtr` per GL command in the registry.
fn write_ptrs(registry: &Registry) -> String {
    let storages = registry.cmd_iter().map(|c| {
        format!(
            "pub static mut {name}: FnPtr = FnPtr {{
                f: failing::{name} as *const libc::c_void,
                is_loaded: false
            }};",
            name = c.proto.ident
        )
    }).collect::<Vec<String>>().connect("\n");

    format!(r##"
        mod storage {{
            #![allow(non_snake_case)]
            use super::__gl_imports::libc;
            use super::failing;
            use super::FnPtr;

            {storages}
        }}
    "##, storages = storages)
}

/// Creates one module for each GL command.
///
/// Each module contains `is_loaded` and `load_with` which interact with the `storage` module
///  created by `write_ptrs`.
fn write_fn_mods(registry: &Registry, ns: &Ns) -> String {
    registry.cmd_iter().map(|c| {
        let fallbacks = match registry.aliases.get(&c.proto.ident) {
            Some(v) => {
                let names = v.iter().map(|name| format!("\"{}\"", super::gen_symbol_name(ns, name.as_slice()))).collect::<Vec<_>>();
                format!("&[{}]", names.connect(", "))
            }, None => "&[]".to_string(),
        };
        let fnname = c.proto.ident.as_slice();
        let symbol = super::gen_symbol_name(ns, c.proto.ident.as_slice());
        let symbol = symbol.as_slice();

        format!(r##"
            #[unstable]
            #[allow(non_snake_case)]
            pub mod {fnname} {{
                use super::{{failing, storage, metaloadfn}};
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {{
                    unsafe {{ storage::{fnname}.is_loaded }}
                }}

                #[allow(dead_code)]
                pub fn load_with(loadfn: |symbol: &str| -> *const super::__gl_imports::libc::c_void) {{
                    unsafe {{
                        storage::{fnname} = FnPtr::new(metaloadfn(loadfn, "{symbol}", {fallbacks}),
                            failing::{fnname} as *const super::__gl_imports::libc::c_void)
                    }}
                }}
            }}
        "##, fnname = fnname, fallbacks = fallbacks, symbol = symbol)
    }).collect::<Vec<String>>().connect("\n")
}

/// Creates a `failing` module which contains one function per GL command.
///
/// These functions are the mocks that are called if the real function could not be loaded.
fn write_failing_fns(registry: &Registry) -> String {
    let functions = registry.cmd_iter().map(|c| {
        (format!(
            "#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            pub extern \"system\" fn {name}({params}) -> {return_suffix} {{ \
                panic!(\"`{name}` was not loaded\") \
            }}",
            name = c.proto.ident,
            params = super::gen_parameters(c, true, true).connect(", "),
            return_suffix = super::gen_return_type(c)
        ))
    }).collect::<Vec<String>>().connect("\n");

    format!(r#"
        mod failing {{
            use super::types;
            use super::__gl_imports;

            {functions}
        }}
    "#, functions = functions)
}

/// Creates the `load_with` function.
///
/// The function calls `load_with` in each module created by `write_fn_mods`.
fn write_load_fn(registry: &Registry) -> String {
    let loadings = registry.cmd_iter().map(|c| {
        let cmd_name = c.proto.ident.as_slice();
        format!("{cmd_name}::load_with(|s| loadfn(s));", cmd_name = cmd_name)
    }).collect::<Vec<String>>().connect("\n");

    let a = format!(r#"
        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load_with(loadfn: |symbol: &str| -> *const __gl_imports::libc::c_void) {{
            {loadings}
        }}
    "#, loadings = loadings);

    let b = format!(r#"
        /// Load each OpenGL symbol using a custom load function.
        ///
        /// ~~~ignore
        /// gl::load(&glfw);
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) {{
            load_with(|name| loader.get_proc_addr(name));
        }}
    "#);

    a + b.as_slice()
}
