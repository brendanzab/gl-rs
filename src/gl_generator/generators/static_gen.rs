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

pub struct StaticGenerator;

impl super::Generator for StaticGenerator {
    fn write(&self, ecx: &ExtCtxt, registry: &Registry, ns: Ns) -> Vec<P<ast::Item>> {
        let mut result = Vec::new();
        result.push(write_header(ecx));
        result.push(write_type_aliases(ecx, &ns));
        result.push_all_move(write_enums(ecx, registry));
        result.push(write_fns(ecx, registry, &ns));
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
    let aliases = super::gen_type_aliases(ns);

    ecx.parse_item(format!("
        #[stable]
        pub mod types {{
            {}
        }}
    ", aliases))
}

fn write_enums(ecx: &ExtCtxt, registry: &Registry) -> Vec<P<ast::Item>> {
    registry.enum_iter().map(|e| {
        ecx.parse_item(super::gen_enum_item(e, "types::"))
    }).collect()
}

fn write_fns(ecx: &ExtCtxt, registry: &Registry, ns: &Ns) -> P<ast::Item> {
    let symbols = registry.cmd_iter().map(|c| {
        format!(
            "#[link_name=\"{symbol}\"]
            pub fn {name}({params}){return_suffix};",
            symbol = super::gen_symbol_name(ns, c),
            name = c.proto.ident,
            params = super::gen_param_list(c, true),
            return_suffix = super::gen_return_suffix(c)
        )
    }).collect::<Vec<String>>().connect("\n");

    ecx.parse_item(format!("
        #[allow(non_snake_case)]
        #[allow(unused_variable)]
        #[allow(dead_code)]
        extern \"system\" {{
            {}
        }}
    ", symbols))
}
