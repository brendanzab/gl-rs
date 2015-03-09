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
pub struct StructGenerator;

impl super::Generator for StructGenerator {
    fn write<W>(&self, registry: &Registry, ns: Ns, dest: &mut W) -> IoResult<()> where W: Writer {
        try!(write_header(dest));
        try!(write_type_aliases(&ns, dest));
        try!(write_enums(registry, dest));
        try!(write_fnptr_struct_def(dest));
        try!(write_panicking_fns(registry, &ns, dest));
        try!(write_struct(registry, &ns, dest));
        try!(write_impl(registry, &ns, dest));
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
            pub use std::marker::Send;
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

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def<W>(dest: &mut W) -> IoResult<()> where W: Writer {
    try!(writeln!(dest, r#"
        #[allow(dead_code)]
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
            fn new(ptr: *const __gl_imports::libc::c_void,
                panicking_fn: *const __gl_imports::libc::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{ f: panicking_fn, is_loaded: false }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {{
                self.is_loaded
            }}
        }}
    "#)
}

/// Creates a `panicking` module which contains one function per GL command.
///
/// These functions are the mocks that are called if the real function could not be loaded.
fn write_panicking_fns<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    try!(writeln!(dest,
        "mod panicking {{
            use super::types;
            use super::__gl_imports;

            #[inline(never)]
            fn missing_fn_panic() -> ! {{
                panic!(\"{ns} function was not loaded\")
            }}
        ", ns = ns));

    for c in registry.cmd_iter() {
        try!(writeln!(dest,
            "#[allow(non_snake_case, unused_variables, dead_code)] \
            pub extern \"system\" fn {name}({params}) -> {return_suffix} {{ missing_fn_panic() }}",
            name = c.proto.ident,
            params = super::gen_parameters(c, true, true).connect(", "),
            return_suffix = super::gen_return_type(c)
        ))
    }

    writeln!(dest, "}}")
}

/// Creates a structure which stores all the `FnPtr` of the bindings.
///
/// The name of the struct corresponds to the namespace.
fn write_struct<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    writeln!(dest, "
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[stable]
        pub struct {ns} {{
            {ptrs}
        }}",

        ns = ns.fmt_struct_name(),
        ptrs = registry.cmd_iter().map(|c| {
            let fallbacks = match registry.aliases.get(&c.proto.ident) {
                Some(v) => v.clone(),
                None => Vec::new(),
            };
            format!(
                "{doc} pub {name}: FnPtr,",
                doc = if fallbacks.len() == 0 { "".to_string() } else { format!("/** Fallbacks: {} */", fallbacks.connect(", ")) },
                name = c.proto.ident,
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}

/// Creates the `impl` of the structure created by `write_struct`.
fn write_impl<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> IoResult<()> where W: Writer {
    writeln!(dest, "
        impl {ns} {{
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[unstable]
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> {ns} where F: FnMut(&str) -> *const __gl_imports::libc::c_void {{
                let mut metaloadfn = |symbol: &str, symbols: &[&str]| {{
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {{
                        for &sym in symbols.iter() {{
                            ptr = loadfn(sym);
                            if !ptr.is_null() {{ break; }}
                        }}
                    }}
                    ptr
                }};
                {ns} {{
                    {loadings}
                }}
            }}

            /// Load each OpenGL symbol using a custom load function.
            ///
            /// ~~~ignore
            /// let gl = Gl::load(&glfw);
            /// ~~~
            #[unstable]
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) -> {ns} {{
                {ns}::load_with(|name| loader.get_proc_addr(name))
            }}

            {modules}
        }}

        unsafe impl __gl_imports::Send for {ns} {{}}",

        ns = ns.fmt_struct_name(),

        loadings = registry.cmd_iter().map(|c| {
            let fallbacks = registry.aliases.get(&c.proto.ident);
            format!(
                "{name}: FnPtr::new(metaloadfn(\"{symbol}\", {fb}), panicking::{name} as *const __gl_imports::libc::c_void),",
                name = c.proto.ident,
                symbol = super::gen_symbol_name(ns, c.proto.ident.as_slice()),
                fb = match fallbacks {
                    Some(fallbacks) => format!("&[{}]", fallbacks.iter().map(|name| format!("\"{}\"", super::gen_symbol_name(ns, name.as_slice()))).collect::<Vec<_>>().connect(", ")),
                    None => format!("&[]")
                },
            )
        }).collect::<Vec<String>>().connect("\n"),

        modules = registry.cmd_iter().map(|c| {
            format!(
                "#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
                #[inline] #[unstable] pub unsafe fn {name}(&self, {params}) -> {return_suffix} {{ \
                    __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> {return_suffix}>\
                        (self.{name}.f)({idents}) \
                }}",
                name = c.proto.ident,
                params = super::gen_parameters(c, true, true).connect(", "),
                typed_params = super::gen_parameters(c, false, true).connect(", "),
                return_suffix = super::gen_return_type(c),
                idents = super::gen_parameters(c, true, false).connect(", "),
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}
