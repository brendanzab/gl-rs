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

use registry::*;
use super::ty;
use std::io::Writer;

pub struct StaticGenerator<'a, W: 'a> {
    ns: Ns,
    writer: &'a mut W,
    registry: &'a Registry,
}

impl<'a, W: Writer> StaticGenerator<'a, W> {
    fn new<'a>(writer: &'a mut W, registry: &'a Registry, ns: Ns) -> StaticGenerator<'a, W> {
        StaticGenerator {
            ns: ns,
            writer: writer,
            registry: registry,
        }
    }

    #[allow(unused_must_use)]
    fn write_str(&mut self, s: &str) {
        self.writer.write(s.as_bytes());
    }

    fn write_line(&mut self, s: &str) {
        self.write_str(s);
        self.write_str("\n");
    }

    fn write_enum(&mut self, enm: &Enum) {
        self.write_line(super::gen_enum_item(enm, "types::").as_slice());
    }

    fn write_enums(&mut self) {
        for e in self.registry.enum_iter() {
            self.write_enum(e);
        }
    }

    fn write_header(&mut self) {
        self.write_line("mod __gl_imports {");
        self.write_line("    extern crate libc;");
        self.write_line("    pub use std::mem;");
        self.write_line("}");
    }

    fn write_type_aliases(&mut self) {
        self.write_line("#[stable]");
        self.write_line("pub mod types {");
        let aliases = super::gen_type_aliases(&self.ns);
        self.write_line(aliases.as_slice());
        self.write_line("}");
    }

    fn write_fns(&mut self) {
        let ns = self.ns;

        self.write_line("#[allow(non_snake_case)] #[allow(unused_variable)] #[allow(dead_code)]");
        self.write_line("extern \"system\" {");

        for c in self.registry.cmd_iter() {
            self.write_line(format!(
                "#[link_name=\"{symbol}\"]
                pub fn {name}({params}){return_suffix};",
                symbol = super::gen_symbol_name(&ns, c),
                name = c.proto.ident,
                params = super::gen_param_list(c, true),
                return_suffix = super::gen_return_suffix(c)
            ).as_slice());
        }
        
        self.write_line("}");
    }

    pub fn write(writer: &mut W, registry: &Registry, ns: Ns) {
        let mut gen = StaticGenerator::new(writer, registry, ns);

        // header with imports
        gen.write_header();
        gen.write_line("");

        // type aliases
        gen.write_type_aliases();
        gen.write_line("");

        // enums definitions
        gen.write_enums();
        gen.write_line("");

        // safe and unsafe OpenGl functions
        gen.write_fns();
        gen.write_line("");
    }
}
