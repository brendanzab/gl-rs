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

pub struct StaticGenerator;

impl super::Generator for StaticGenerator {
    #[allow(unused_must_use)]
    fn write<W: Writer>(&self, writer: &mut W, registry: &Registry, ns: Ns) {
        writeln!(writer, "{}", write_header());
        writeln!(writer, "{}", write_type_aliases(&ns));
        writeln!(writer, "{}", write_enums(registry));
        writeln!(writer, "{}", write_fns(registry, &ns));
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

fn write_fns(registry: &Registry, ns: &Ns) -> String {
    format!(
        "#[allow(non_snake_case)]
        #[allow(unused_variable)]
        #[allow(dead_code)]

        extern \"system\" {{
            {symbols}
        }}",

        symbols = registry.cmd_iter().map(|c| {
            format!(
                "#[link_name=\"{symbol}\"]
                pub fn {name}({params}){return_suffix};",
                symbol = super::gen_symbol_name(ns, c),
                name = c.proto.ident,
                params = super::gen_param_list(c, true),
                return_suffix = super::gen_return_suffix(c)
            )
        }).collect::<Vec<String>>().connect("\n")
    )
}
