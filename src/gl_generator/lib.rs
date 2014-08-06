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

#![crate_name = "gl_generator"]
#![comment = "OpenGL function loader generator."]
#![license = "ASL2"]
#![crate_type = "dylib"]

#![feature(phase)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(plugin_registrar)]
#![feature(quote)]

#[phase(plugin, link)]
extern crate log;

extern crate rustc;
extern crate syntax;

use std::path::Path;
use std::io::{File, Reader};

use syntax::parse::token;
use syntax::ast::{ Item, TokenTree };
use syntax::ext::base::{expr_to_string, get_exprs_from_tts, DummyResult, ExtCtxt, MacResult};
use syntax::codemap::Span;

use registry::*;
use static_gen::StaticGenerator;
use struct_gen::StructGenerator;

mod common;
pub mod static_gen;
pub mod struct_gen;
pub mod registry;
pub mod ty;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut ::rustc::plugin::Registry) {
    reg.register_macro("generate_gl_bindings", macro_handler);
}

// this is the object that we will return from the "generate_gl_bindings" macro expansion
struct MacroResult {
    content: Vec<::std::gc::Gc<Item>>
}
impl MacResult for MacroResult {
    fn make_def(&self) -> Option<::syntax::ext::base::MacroDef> { None }
    fn make_expr(&self) -> Option<::std::gc::Gc<::syntax::ast::Expr>> { None }
    fn make_pat(&self) -> Option<::std::gc::Gc<::syntax::ast::Pat>> { None }
    fn make_stmt(&self) -> Option<::std::gc::Gc<::syntax::ast::Stmt>> { None }

    fn make_items(&self) -> Option<::syntax::util::small_vector::SmallVector<::std::gc::Gc<Item>>> {
        Some(::syntax::util::small_vector::SmallVector::many(self.content.clone()))
    }
}

// handler for generate_gl_bindings!
fn macro_handler(ecx: &mut ExtCtxt, span: Span, token_tree: &[TokenTree]) -> Box<MacResult> {
    // getting the arguments from the macro
    let (nsLiteral, api, profile, version, generator, exts) = match parse_macro_arguments(ecx, span.clone(), token_tree) {
        Some(t) => t,
        None => return DummyResult::any(span)
    };

    let ns = match nsLiteral.as_slice() {
        "gl"  => Gl,
        "glx" => {
            ecx.span_err(span, "glx generation unimplemented");
            return DummyResult::any(span)
        },
        "wgl" => {
            ecx.span_err(span, "wgl generation unimplemented");
            return DummyResult::any(span)
        }
        ns => {
            ecx.span_err(span, format!("Unexpected opengl namespace '{}'", ns).as_slice());
            return DummyResult::any(span)
        }
    };

    let file_name = format!("deps/khronos-api/{}.xml", nsLiteral);
    let path = Path::new(file_name.clone());

    let filter = Some(Filter {
        extensions: exts.unwrap_or("".to_string()).as_slice().split(',').map(|s| s.to_string()).collect(),
        profile: profile,
        version: version,
        api: api,
    });

    // generating the registry of all bindings
    let reg = Registry::from_xml(
        match
            match File::open(&path) {
                                Ok(f) => f,
                                Err(err) => {
                                    ecx.span_err(span, format!("{}", err).as_slice());
                                    return DummyResult::any(span)
                                }
                            }.read_to_string()
        {
            Ok(s) => s,
            Err(_) => {
                ecx.span_err(span, "registry source not utf8!");
                return DummyResult::any(span)
            }

        }.as_slice(), ns, filter
    );

    // generating the Rust bindings as a source code into "buffer"
    let mut buffer = ::std::io::MemWriter::new();
    match generator.as_slice() {
        "static" => StaticGenerator::write(&mut buffer, &reg, ns, false),
        "struct" => StructGenerator::write(&mut buffer, &reg, ns, false),
        generator => {
            ecx.span_err(span, format!("unknown generator type: {}", generator).as_slice());
            return DummyResult::any(span)
        },
    }

    // creating a new Rust parser from these bindings
    let content = match String::from_utf8(buffer.unwrap()) {
        Ok(s) => s,
        Err(err) => {
            ecx.span_err(span, format!("{}", err).as_slice());
            return DummyResult::any(span)
        }
    };
    let mut parser = ::syntax::parse::new_parser_from_source_str(ecx.parse_sess(), ecx.cfg(), file_name.to_string(), content);

    // getting all the items defined by the bindings
    let mut items = Vec::new();
    loop {
        match parser.parse_item_with_outer_attributes() {
            None => break,
            Some(i) => items.push(i)
        }
    }
    if !parser.eat(&token::EOF) {
        ecx.span_err(span, "the rust parser failed to compile all the generated bindings (meaning there is a bug in this library!)");
        return DummyResult::any(span)
    }
    box MacroResult { content: items } as Box<MacResult>
}

fn parse_macro_arguments(ecx: &mut ExtCtxt, span: Span, tts: &[syntax::ast::TokenTree]) -> Option<(String, String, String, String, String, Option<String>)> {
    let values = match get_exprs_from_tts(ecx, span, tts) {
        Some(v) => v,
        None => return None
    };

    if values.len() != 5 && values.len() != 6 {
        ecx.span_err(span, format!("expected 5 or 6 arguments but only got {}", values.len()).as_slice());
        return None;
    }

    match (
        expr_to_string(ecx, values[0].clone(), "expected string literal").map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[1].clone(), "expected string literal").map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[2].clone(), "expected string literal").map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[3].clone(), "expected string literal").map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[4].clone(), "expected string literal").map(|e| match e { (s, _) => s.get().to_string() }),
        values.as_slice().get(5).and_then(|e| expr_to_string(ecx, *e, "expected string literal")).map(|e| match e { (s, _) => s.get().to_string() }),
    ) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), f) => Some((a, b, c, d, e, f)),
        _ => None
    }
}
