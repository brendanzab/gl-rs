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

use registry::{Registry, Ns};
use std::old_io::IoResult;

#[allow(missing_copy_implementations)]
pub struct GlobalGenerator;

impl super::Generator for GlobalGenerator {
    fn write<W>(&self, registry: &Registry, ns: Ns, dest: &mut W) -> IoResult<()> where W: Writer {
        try!(write_header(dest));
        try!(write_metaloadfn(dest));
        try!(write_type_aliases(&ns, dest));
        try!(write_enums(registry, dest));
        try!(write_fns(registry, dest));
        try!(write_fnptr_struct_def(dest));
        try!(write_ptrs(registry, dest));
        try!(write_fn_mods(registry, &ns, dest));
        try!(write_panicking_fns(registry, &ns, dest));
        try!(write_load_fn(registry, dest));
        Ok(())
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header<W>(dest: &mut W) -> IoResult<()> where W: Writer {
    writeln!(dest, r#"
        mod __gl_imports {{
            extern crate gl_common;
            extern crate libc;
            pub use std::mem;
        }}
    "#)
}

/// Creates the metaloadfn function for fallbacks
fn write_metaloadfn<W>(dest: &mut W) -> IoResult<()> where W: Writer {
    writeln!(dest, r#"
        fn metaloadfn<F>(mut loadfn: F,
                         symbol: &str,
                         fallbacks: &[&str]) -> *const __gl_imports::libc::c_void
                         where F: FnMut(&str) -> *const __gl_imports::libc::c_void {{
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
fn write_type_aliases<W>(ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    try!(writeln!(dest, r#"
        #[stable]
        pub mod types {{
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            #![allow(missing_copy_implementations)]
    "#));

    try!(super::gen_type_aliases(ns, dest));

    writeln!(dest, "
        }}
    ")
}

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums<W>(registry: &Registry, dest: &mut W) -> IoResult<()> where W: Writer {
    for e in registry.enum_iter() {
        try!(super::gen_enum_item(e, "types::", dest));
    }

    Ok(())
}

/// Creates the functions corresponding to the GL commands.
///
/// The function calls the corresponding function pointer stored in the `storage` module created
///  by `write_ptrs`.
fn write_fns<W>(registry: &Registry, dest: &mut W) -> IoResult<()> where W: Writer {
    for c in registry.cmd_iter() {
        let doc = match registry.aliases.get(&c.proto.ident) {
            Some(v) => format!("/** Fallbacks: {} */", v.connect(", ")),
            None => "".to_string()
        };

        try!(writeln!(dest,
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
        ));
    }

    Ok(())
}

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def<W>(dest: &mut W) -> IoResult<()> where W: Writer {
    try!(writeln!(dest, r#"
        #[allow(missing_copy_implementations)]
        pub struct FnPtr {{
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::libc::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }}
    "#));

    writeln!(dest, r#"
        impl FnPtr {{
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::libc::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{ f: missing_fn_panic as *const __gl_imports::libc::c_void, is_loaded: false }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}
        }}
    "#)
}

/// Creates a `storage` module which contains a static `FnPtr` per GL command in the registry.
fn write_ptrs<W>(registry: &Registry, dest: &mut W) -> IoResult<()> where W: Writer {
    let storages = registry.cmd_iter().map(|c| {
        format!(
            "pub static mut {name}: FnPtr = FnPtr {{
                f: super::missing_fn_panic as *const libc::c_void,
                is_loaded: false
            }};",
            name = c.proto.ident
        )
    }).collect::<Vec<String>>().connect("\n");

    writeln!(dest, r##"
        mod storage {{
            #![allow(non_snake_case)]
            use super::__gl_imports::libc;
            use super::FnPtr;

            {storages}
        }}
    "##, storages = storages)
}

/// Creates one module for each GL command.
///
/// Each module contains `is_loaded` and `load_with` which interact with the `storage` module
///  created by `write_ptrs`.
fn write_fn_mods<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    for c in registry.cmd_iter() {
        let fallbacks = match registry.aliases.get(&c.proto.ident) {
            Some(v) => {
                let names = v.iter().map(|name| format!("\"{}\"", super::gen_symbol_name(ns, name.as_slice()))).collect::<Vec<_>>();
                format!("&[{}]", names.connect(", "))
            }, None => "&[]".to_string(),
        };
        let fnname = c.proto.ident.as_slice();
        let symbol = super::gen_symbol_name(ns, c.proto.ident.as_slice());
        let symbol = symbol.as_slice();

        try!(writeln!(dest, r##"
            #[unstable]
            #[allow(non_snake_case)]
            pub mod {fnname} {{
                use super::{{storage, metaloadfn}};
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {{
                    unsafe {{ storage::{fnname}.is_loaded }}
                }}

                #[allow(dead_code)]
                pub fn load_with<F>(loadfn: F) where F: FnMut(&str) -> *const super::__gl_imports::libc::c_void {{
                    unsafe {{
                        storage::{fnname} = FnPtr::new(metaloadfn(loadfn, "{symbol}", {fallbacks}))
                    }}
                }}
            }}
        "##, fnname = fnname, fallbacks = fallbacks, symbol = symbol));
    }

    Ok(())
}

/// Creates a `missing_fn_panic` function.
///
/// This function is the mock that is called if the real function could not be called.
fn write_panicking_fns<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    writeln!(dest,
        "
        #[inline(never)]
        fn missing_fn_panic() -> ! {{
            panic!(\"{ns} function was not loaded\")
        }}
        ", ns = ns)
}

/// Creates the `load_with` function.
///
/// The function calls `load_with` in each module created by `write_fn_mods`.
fn write_load_fn<W>(registry: &Registry, dest: &mut W) -> IoResult<()> where W: Writer {
    let loadings = registry.cmd_iter().map(|c| {
        let cmd_name = c.proto.ident.as_slice();
        format!("{cmd_name}::load_with(|s| loadfn(s));", cmd_name = cmd_name)
    }).collect::<Vec<String>>().connect("\n");

    try!(writeln!(dest, r#"
        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[unstable]
        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const __gl_imports::libc::c_void {{
            {loadings}
        }}
    "#, loadings = loadings));

    writeln!(dest, r#"
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
    "#)
}
