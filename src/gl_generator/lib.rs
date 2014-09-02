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

//! # gl_generator
//!
//! `gl_generator` is an OpenGL bindings generator plugin. It defines a macro named
//!  `generate_gl_bindings!` which can be used to generate all constants and functions of a
//!  given OpenGL version.
//!
//! ## Example
//!
//! ```rust
//! #[phase(plugin)]
//! extern crate gl_generator;
//! extern crate libc;
//! 
//! use std::mem;
//! use self::types::*;
//!
//! generate_gl_bindings!("gl", "core", "4.5", "static", [ "GL_EXT_texture_filter_anisotropic" ])
//! ```
//!
//! ## Parameters
//!
//! * API: Can be `gl`, `gles1`, `gles2`, `wgl`, `glx`, `egl`.
//! * Profile: Can be `core` or `compatibility`. `core` will only include all functions supported
//!    by the requested version it self, while `compatibility` will include all the functions from
//!    previous versions as well.
//! * Version: The requested version of OpenGL, WGL, GLX or EGL in the format `x.x`.
//! * Generator: Can be `static` or `struct`.
//! * Extensions (optional): An array of extensions to include in the bindings.
//! 
//! ## About EGL
//!
//! When you generate bindings for EGL, the following platform-specific types must be declared
//!  *at the same level where you call `generate_gl_bindings`*:
//!
//! - khronos_utime_nanoseconds_t
//! - khronos_uint64_t
//! - khronos_ssize_t
//! - EGLNativeDisplayType
//! - EGLNativePixmapType
//! - EGLNativeWindowType
//! - EGLint
//! - NativeDisplayType
//! - NativePixmapType
//! - NativeWindowType
//!


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

#[phase(plugin)]
extern crate regex_macros;

extern crate khronos_api;
extern crate regex;
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
fn macro_handler(ecx: &mut ExtCtxt, span: Span, token_tree: &[TokenTree]) -> Box<MacResult+'static> {
    // getting the arguments from the macro
    let (api, profile, version, generator, extensions) = match parse_macro_arguments(ecx, span.clone(), token_tree) {
        Some(t) => t,
        None => return DummyResult::any(span)
    };

    let (ns, source) = match api.as_slice() {
        "gl"  => (Gl, khronos_api::GL_XML),
        "glx" => (Glx, khronos_api::GLX_XML),
        "wgl" => (Wgl, khronos_api::WGL_XML),
        "egl" => (Egl, khronos_api::EGL_XML),
        "gles1"  => (Gles1, khronos_api::GL_XML),
        "gles2"  => (Gles2, khronos_api::GL_XML),
        ns => {
            ecx.span_err(span, format!("Unexpected opengl namespace '{}'", ns).as_slice());
            return DummyResult::any(span)
        }
    };

    let filter = Some(Filter {
        extensions: extensions,
        profile: profile,
        version: version,
        api: api,
    });

    // generating the registry of all bindings
    let reg = {
        use std::io::BufReader;
        use std::task;

        let result = task::try(proc() {
            let reader = BufReader::new(source.as_bytes());
            Registry::from_xml(reader, ns, filter)
        });

        match result {
            Ok(reg) => reg,
            Err(err) => {
                use std::any::{Any, AnyRefExt};
                let err: &Any = err;

                match err {
                    err if err.is::<String>() => {
                        ecx.span_err(span, "error while parsing the registry");
                        ecx.span_err(span, err.downcast_ref::<String>().unwrap().as_slice());
                    },
                    err if err.is::<&'static str>() => {
                        ecx.span_err(span, "error while parsing the registry");
                        ecx.span_err(span, err.downcast_ref::<&'static str>().unwrap().as_slice());
                    },
                    _ => {
                        ecx.span_err(span, "unknown error while parsing the registry");
                    }
                }

                return DummyResult::any(span);
            }
        }
    };

    // generating the Rust bindings as a source code into "buffer"
    let buffer = {
        use std::io::MemWriter;
        use std::task;

        // calling the generator
        let result = match generator.as_slice() {
            "static" => task::try(proc() {
                let mut buffer = MemWriter::new();
                StaticGenerator::write(&mut buffer, &reg, ns);
                buffer
            }),

            "struct" => task::try(proc() {
                let mut buffer = MemWriter::new();
                StructGenerator::write(&mut buffer, &reg, ns);
                buffer
            }),

            generator => {
                ecx.span_err(span, format!("unknown generator type: {}", generator).as_slice());
                return DummyResult::any(span);
            },
        };

        // processing the result
        match result {
            Ok(buffer) => buffer.unwrap(),
            Err(err) => {
                use std::any::{Any, AnyRefExt};
                let err: &Any = err;

                match err {
                    err if err.is::<String>() => {
                        ecx.span_err(span, "error while generating the bindings");
                        ecx.span_err(span, err.downcast_ref::<String>().unwrap().as_slice());
                    },
                    err if err.is::<&'static str>() => {
                        ecx.span_err(span, "error while generating the bindings");
                        ecx.span_err(span, err.downcast_ref::<&'static str>().unwrap().as_slice());
                    },
                    _ => {
                        ecx.span_err(span, "unknown error while generating the bindings");
                    }
                }

                return DummyResult::any(span);
            }
        }
    };

    // creating a new Rust parser from these bindings
    let content = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(err) => {
            ecx.span_err(span, format!("{}", err).as_slice());
            return DummyResult::any(span)
        }
    };
    let mut parser = ::syntax::parse::new_parser_from_source_str(ecx.parse_sess(), ecx.cfg(),
        Path::new(ecx.codemap().span_to_filename(span)).display().to_string(), content);

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

fn parse_macro_arguments(ecx: &mut ExtCtxt, span: Span, tts: &[syntax::ast::TokenTree])
                        -> Option<(String, String, String, String, Vec<String>)>
{
    // getting parameters list
    let values = match get_exprs_from_tts(ecx, span, tts) {
        Some(v) => v,
        None => return None
    };

    if values.len() != 4 && values.len() != 5 {
        ecx.span_err(span, format!("expected 4 or 5 arguments but got {}", values.len())
            .as_slice());
        return None;
    }

    // computing the extensions (last parameter)
    let extensions: Vec<String> = match values.as_slice().get(4) {
        None => Vec::new(),
        Some(vector) => {
            use syntax::ast::ExprVec;

            match vector.node {
                // only [ ... ] is accepted
                ExprVec(ref list) => {
                    // turning each element into a string
                    let mut result = Vec::new();
                    for element in list.iter() {
                        match expr_to_string(ecx, element.clone(), "expected string literal") {
                            Some((s, _)) => result.push(s.get().to_string()),
                            None => return None
                        }
                    }
                    result
                },
                _ => {
                    ecx.span_err(span, format!("last argument must be a vector").as_slice());
                    return None;
                }
            }
        }
    };

    // computing other parameters
    match (
        expr_to_string(ecx, values[0].clone(), "expected string literal")
            .map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[1].clone(), "expected string literal")
            .map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[2].clone(), "expected string literal")
            .map(|e| match e { (s, _) => s.get().to_string() }),
        expr_to_string(ecx, values[3].clone(), "expected string literal")
            .map(|e| match e { (s, _) => s.get().to_string() })
    ) {
        (Some(a), Some(b), Some(c), Some(d)) => Some((a, b, c, d, extensions)),
        _ => None
    }
}
