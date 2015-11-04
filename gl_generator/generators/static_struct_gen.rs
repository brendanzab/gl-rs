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
use std::io;

#[allow(missing_copy_implementations)]
pub struct StaticStructGenerator;

impl super::Generator for StaticStructGenerator {
    fn write<W>(&self, registry: &Registry, ns: Ns, dest: &mut W) -> io::Result<()> where W: io::Write {
        try!(write_header(dest));
        try!(write_type_aliases(&ns, dest));
        try!(write_enums(registry, dest));
        try!(write_struct(&ns, dest));
        try!(write_impl(registry, &ns, dest));
        try!(write_fns(registry, &ns, dest));
        Ok(())
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    writeln!(dest, r#"
        mod __gl_imports {{
            extern crate libc;
            pub use std::mem;
        }}
    "#)
}

/// Creates a `types` module which contains all the type aliases.
///
/// See also `generators::gen_type_aliases`.
fn write_type_aliases<W>(ns: &Ns, dest: &mut W) -> io::Result<()> where W: io::Write {
    try!(writeln!(dest, r#"
        pub mod types {{
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            #![allow(missing_copy_implementations)]
    "#));

    try!(super::gen_type_aliases(ns, dest));

    writeln!(dest, "}}")
}

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums<W>(registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write {
    for e in registry.enum_iter() {
        try!(super::gen_enum_item(e, "types::", dest));
    }

    Ok(())
}

/// Creates a stub structure.
///
/// The name of the struct corresponds to the namespace.
fn write_struct<W>(ns: &Ns, dest: &mut W) -> io::Result<()> where W: io::Write {
    writeln!(dest, "
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[derive(Copy, Clone)]
        pub struct {ns};",
        ns = ns.fmt_struct_name(),
    )
}

/// Creates the `impl` of the structure created by `write_struct`.
fn write_impl<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> io::Result<()> where W: io::Write {
    try!(writeln!(dest,
        "impl {ns} {{
            /// Stub function.
            #[allow(dead_code)]
            pub fn load_with<F>(mut _loadfn: F) -> {ns} where F: FnMut(&str) -> *const __gl_imports::libc::c_void {{
                {ns}
            }}",
        ns = ns.fmt_struct_name(),
    ));

    for c in registry.cmd_iter() {
        try!(writeln!(dest,
            "#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn {name}(&self, {typed_params}) -> {return_suffix} {{
                {name}({idents})
            }}",
            name = c.proto.ident,
            typed_params = super::gen_parameters(c, true, true).join(", "),
            return_suffix = super::gen_return_type(c),
            idents = super::gen_parameters(c, true, false).join(", "),
        ));
    }

    writeln!(dest, "}}")
}

/// io::Writes all functions corresponding to the GL bindings.
///
/// These are foreign functions, they don't have any content.
fn write_fns<W>(registry: &Registry, ns: &Ns, dest: &mut W) -> io::Result<()> where W: io::Write {

    try!(writeln!(dest, "
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(dead_code)]
        extern \"system\" {{"));

    for c in registry.cmd_iter() {
        try!(writeln!(dest,
            "#[link_name=\"{symbol}\"] fn {name}({params}) -> {return_suffix};",
            symbol = super::gen_symbol_name(ns, &c.proto.ident),
            name = c.proto.ident,
            params = super::gen_parameters(c, true, true).join(", "),
            return_suffix = super::gen_return_type(c)
        ));
    }

    writeln!(dest, "}}")
}
