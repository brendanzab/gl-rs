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
pub struct StaticStructGenerator;

impl super::Generator for StaticStructGenerator {
    fn write(&self, registry: &Registry, ns: Ns) -> String {
        let mut result = Vec::new();
        result.push(write_header());
        result.push(write_type_aliases(&ns));
        result.push(write_enums(registry));
        result.push(write_struct(&ns));
        result.push(write_impl(registry, &ns));
        result.push(write_fns(registry, &ns));
        result.connect("\n")
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header() -> String {
    format!(r#"
        mod __gl_imports {{
            extern crate libc;
            pub use std::mem;
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

            {aliases}
        }}
    "#, aliases = aliases)
}

/// Writes all the `<enum>` elements at the root of the bindings.
fn write_enums(registry: &Registry) -> String {
    registry.enum_iter().map(|e| {
        super::gen_enum_item(e, "types::")
    }).collect::<Vec<String>>().connect("\n")
}

/// Creates a stub structure.
///
/// The name of the struct corresponds to the namespace.
fn write_struct(ns: &Ns) -> String {
    format!("
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[stable]
        pub struct {ns};
        ",

        ns = ns.fmt_struct_name(),
    )
}

/// Creates the `impl` of the structure created by `write_struct`.
fn write_impl(registry: &Registry, ns: &Ns) -> String {
    format!("
        impl {ns} {{
            /// Stub function.
            #[unstable]
            #[allow(dead_code)]
            pub fn load_with(_loadfn: |symbol: &str| -> *const __gl_imports::libc::c_void) -> {ns} {{
                {ns}
            }}

            {modules}
        }}",

        ns = ns.fmt_struct_name(),

        modules = registry.cmd_iter().map(|c| {
            format!(
                "#[allow(non_snake_case)]
                // #[allow(unused_variables)]
                #[allow(dead_code)]
                #[inline]
                #[unstable]
                pub unsafe fn {name}(&self, {typed_params}) -> {return_suffix} {{
                    {name}({idents})
                }}",
                name = c.proto.ident,
                typed_params = super::gen_parameters(c, true, true).connect(", "),
                return_suffix = super::gen_return_type(c),
                idents = super::gen_parameters(c, true, false).connect(", "),
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}

/// Writes all functions corresponding to the GL bindings.
///
/// These are foreign functions, they don't have any content.
fn write_fns(registry: &Registry, ns: &Ns) -> String {
    let symbols = registry.cmd_iter().map(|c| {
        format!(
            "#[link_name=\"{symbol}\"]
            pub fn {name}({params}) -> {return_suffix};",
            symbol = super::gen_symbol_name(ns, c.proto.ident.as_slice()),
            name = c.proto.ident,
            params = super::gen_parameters(c, true, true).connect(", "),
            return_suffix = super::gen_return_type(c)
        )
    }).collect::<Vec<String>>().connect("\n");

    format!("
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(dead_code)]
        extern \"system\" {{
            {}
        }}
    ", symbols)
}
