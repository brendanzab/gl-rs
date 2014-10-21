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
//! `gl_generator` is an OpenGL bindings generator plugin. It defines a macro
//!  named `generate_gl_bindings!` which can be used to generate all constants
//!  and functions of a given OpenGL version.
//!
//! ## Example
//!
//! ~~~rust
//! #[phase(plugin)]
//! extern crate gl_generator;
//! extern crate libc;
//!
//! use std::mem;
//! use self::types::*;
//!
//! generate_gl_bindings! {
//!     api: gl,
//!     profile: core,
//!     version: 4.5,
//!     generator: global,
//!     extensions: [
//!         GL_EXT_texture_filter_anisotropic,
//!     ],
//! }
//! ~~~
//!
//! ## Field Syntax
//!
//! Each field can be specified at most once, or not at all. If the field is not
//! specified, then a default value will be used. The syntax for each field is
//! as follows (in [EBNF](http://en.wikipedia.org/wiki/Extended_Backus-Naur_Form)):
//!
//! ~~~ebnf
//! api        = "api:",        ("gl" | "gles1" | "gles2" | "wgl" | "glx" | "egl")
//! profile    = "profile:",    ("core" | "compatibility")
//! version    = "version:",    digit, ".", digit
//! generator  = "generator:",  ("static" | "global" | "struct")
//! extensions = "extensions:", "[", [ [ ident ], { ",", ident } ], [ "," ], "]"
//! ~~~
//!
//! ### Notes
//!
//! - `api`: The API to generate. Defaults to `gl`.
//! - `profile`: Defaults to `core`. `core` will only include all functions
//!    supported by the requested version it self, while `compatibility` will
//!    include all the functions from previous versions as well.
//! - `version`: The requested API version. Defaults to `1.0`.
//! - `generator`: The type of loader to generate. Defaults to `static`.
//! - `extensions`: Extra extensions to include in the bindings. Defaults to `[]`.
//!
//! ## About EGL
//!
//! When you generate bindings for EGL, the following platform-specific types must be declared
//!  *at the same level where you call `generate_gl_bindings`*:
//!
//! - `khronos_utime_nanoseconds_t`
//! - `khronos_uint64_t`
//! - `khronos_ssize_t`
//! - `EGLNativeDisplayType`
//! - `EGLNativePixmapType`
//! - `EGLNativeWindowType`
//! - `EGLint`
//! - `NativeDisplayType`
//! - `NativePixmapType`
//! - `NativeWindowType`
//!


#![crate_name = "gl_generator"]
#![comment = "OpenGL function loader generator."]
#![license = "ASL2"]
#![crate_type = "dylib"]

#![feature(advanced_slice_patterns)]
#![feature(macro_rules)]
#![feature(phase)]
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

use generators::Generator;
use registry::{Registry, Filter, Ns};
use syntax::ast::{TokenTree, TTDelim, TTNonterminal, TTSeq, TTTok};
use syntax::codemap::Span;
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult, MacItems};
use syntax::parse::token;

mod generators;

#[allow(dead_code)]
mod registry;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut ::rustc::plugin::Registry) {
    reg.register_macro("generate_gl_bindings", macro_handler);
}

/// A predicate that is useful for splitting a comma separated list of tokens
fn is_comma(tt: &TokenTree) -> bool {
    match *tt {
        TTTok(_, token::COMMA) => true,
        _ => false,
    }
}

/// Drops a trailing comma if it exists
fn drop_trailing_comma(tts: &[TokenTree]) -> &[TokenTree] {
    match tts {
        [tts.., TTTok(_, token::COMMA)] => tts,
        tts => tts,
    }
}

fn get_span_from_tt(tt: &TokenTree) -> Span {
    match *tt {
        TTTok(span, _) => span,
        TTSeq(span, _, _, _) => span,
        TTNonterminal(span, _) => span,
        TTDelim(ref tts) => get_span_from_tt(
            tts.as_slice().head()
               .expect("Delimetered token trees should contain at least \
                        two elements.")
        ),
    }
}

/// handler for generate_gl_bindings!
fn macro_handler(ecx: &mut ExtCtxt, span: Span, tts: &[TokenTree]) -> Box<MacResult+'static> {
    // Generator options
    let mut api = None::<(Ns, &'static [u8])>;
    let mut profile = None::<String>;
    let mut version = None::<String>;
    let mut generator = None::<Box<Generator>>;
    let mut extensions = None::<Vec<String>>;

    let tts = drop_trailing_comma(tts);

    // Iterate through the comma separated
    for tts in tts.split(is_comma) {
        let mut it = tts.iter();
        let field = match it.next() {
            Some(&TTTok(_, token::IDENT(ref field, _))) => field.as_str(),
            tt => {
                let span = tt.map(get_span_from_tt).unwrap_or(span);
                ecx.span_err(span, "Expected a generator argument name, \
                                    either: `api`, `profile`, `version`, \
                                    `generator`, or `extensions`.");
                return DummyResult::any(span);
            },
        };
        match it.next() {
            Some(&TTTok(_, token::COLON)) => {},
            tt => {
                let span = tt.map(get_span_from_tt).unwrap_or(span);
                ecx.span_err(span, "Expected `:`");
                return DummyResult::any(span);
            },
        }
        match (field, it.collect::<Vec<_>>().as_slice()) {
            ("api", tts) => {
                if api.is_some() {
                    let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                    ecx.span_err(span, "An API was already specified.");
                    return DummyResult::any(span);
                }
                api = Some(match tts {
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "gl"
                        => (registry::Gl, khronos_api::GL_XML),
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "glx"
                        => (registry::Glx, khronos_api::GLX_XML),
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "wgl"
                        => (registry::Wgl, khronos_api::WGL_XML),
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "egl"
                        => (registry::Egl, khronos_api::EGL_XML),
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "gles1"
                        => (registry::Gles1, khronos_api::GL_XML),
                    [&TTTok(_, token::IDENT(ref api, _))] if api.as_str() == "gles2"
                        => (registry::Gles2, khronos_api::GL_XML),
                    [&TTTok(span, token::IDENT(ref api, _))] => {
                        ecx.span_err(span, format!("Unknown API `{}`", api.as_str()).as_slice());
                        return DummyResult::any(span);
                    },
                    _ => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, "Invalid API format, expected \
                                            identifier");
                        return DummyResult::any(span);
                    }
                })
            }
            ("profile", tts) => {
                if profile.is_some() {
                    ecx.span_err(get_span_from_tt(tts[0]),
                                 "A profile was already specified.");
                    return DummyResult::any(span);
                }
                profile = Some(match tts {
                    [&TTTok(_, token::IDENT(ref profile, _))] if profile.as_str() == "core"
                        => "core".to_string(),
                    [&TTTok(_, token::IDENT(ref profile, _))] if profile.as_str() == "compatibility"
                        => "compatibility".to_string(),
                    [&TTTok(_, token::IDENT(ref profile, _))] => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, format!("Unknown profile `{}`",
                                                   profile.as_str()).as_slice());
                        return DummyResult::any(span);
                    },
                    _ => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, "Invalid profile format, expected \
                                            identifier");
                        return DummyResult::any(span);
                    },
                })
            }
            ("version", tts) => {
                if version.is_some() {
                    let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                    ecx.span_err(span, "A version was already specified.");
                    return DummyResult::any(span);
                }
                version = Some(match tts {
                    [&TTTok(_, token::LIT_FLOAT(ref version))] => {
                        version.as_str().to_string()
                    },
                    _ => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, "Invalid version format, expected \
                                            `major.minor`");
                        return DummyResult::any(span);
                    },
                });
            }
            ("generator", tts) => {
                if generator.is_some() {
                    let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                    ecx.span_err(span, "A generator was already specified.");
                    return DummyResult::any(span);
                }
                generator = Some(match tts {
                    [&TTTok(_, token::IDENT(ref gen, _))] if gen.as_str() == "global"
                        => box generators::global_gen::GlobalGenerator as Box<Generator>,
                    [&TTTok(_, token::IDENT(ref gen, _))] if gen.as_str() == "struct"
                        => box generators::struct_gen::StructGenerator as Box<Generator>,
                    [&TTTok(_, token::IDENT(ref gen, _))] if gen.as_str() == "static"
                        => box generators::static_gen::StaticGenerator as Box<Generator>,
                    [&TTTok(span, token::IDENT(ref gen, _))] => {
                        ecx.span_err(span, format!("Unknown generator `{}`",
                                                   gen.as_str()).as_slice());
                        return DummyResult::any(span);
                    },
                    _ => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, "Invalid generator format, expected \
                                            identifier");
                        return DummyResult::any(span);
                    },
                });
            }
            ("extensions", tts) => {
                if extensions.is_some() {
                    let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                    ecx.span_err(span, "The list of extensions were already \
                                        specified.");
                    return DummyResult::any(span);
                }
                extensions = Some(match tts {
                    [&TTDelim(ref tts)] => match tts.as_slice() {
                        [TTTok(_, token::LBRACKET),
                         tts..,
                         TTTok(_, token::RBRACKET)] => {
                            // Drop the trailing comma if it exists
                            let tts = drop_trailing_comma(tts);

                            // Collect the extensions, breaking early if a parse
                            // error occurs.
                            let mut failed = false;
                            let exts = tts.split(is_comma)
                                          .scan((), |_, tts| match tts {
                                [TTTok(_, token::IDENT(ref ext, _))] => {
                                    Some(ext.as_str().to_string())
                                },
                                _ => {
                                    failed = true;
                                    None
                                },
                            }).collect();

                            // Cause an error if there is still some leftover
                            // tokens.
                            if failed {
                                let span = tts.head().map_or(span, get_span_from_tt);
                                ecx.span_err(span, "Invalid extension format, \
                                                    expected identifier");
                                return DummyResult::any(span);
                            } else {
                                exts
                            }
                        },
                        _ => {
                            let span = tts.as_slice().head().map_or(span, get_span_from_tt);
                            ecx.span_err(span, "Expected a comma separated \
                                                list of extension identifiers \
                                                delimited by square brackets: \
                                                `[]`");
                            return DummyResult::any(span);
                        }
                    },
                    _ => {
                        let span = tts.head().map_or(span, |tt| get_span_from_tt(*tt));
                        ecx.span_err(span, "Expected a comma separated list of \
                                            extension identifiers.");
                        return DummyResult::any(span);
                    },
                });
            }
            (field, _) => {
                ecx.span_err(span, format!("Unknown field `{}`", field).as_slice());
                return DummyResult::any(span);
            }
        }
    }

    // Use default values if the parameters have not been set
    let (ns, source) = api.unwrap_or((registry::Gl, khronos_api::GL_XML));
    let extensions = extensions.unwrap_or(vec![]);
    let version = version.unwrap_or("1.0".to_string());
    let generator = generator.unwrap_or(box generators::static_gen::StaticGenerator);
    let profile = profile.unwrap_or("core".to_string());

    // Get generator field values, using default values if they have not been
    // specified
    let filter = Some(Filter {
        api: ns.to_string(),
        extensions: extensions,
        version: version,
        profile: profile,
    });

    // Generate the registry of all bindings
    let registry = {
        use std::io::BufReader;
        use std::task;

        let result = task::try(proc() {
            let reader = BufReader::new(source);
            Registry::from_xml(reader, ns, filter)
        });

        match result {
            Ok(reg) => reg,
            Err(err) => {
                use std::any::{Any, AnyRefExt};
                let err: &Any = &*err;

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
    let items = generator.write(ecx, &registry, ns);

    MacItems::new(items.into_iter())
}
