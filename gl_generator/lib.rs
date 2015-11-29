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
//! `gl_generator` is an OpenGL bindings generator. It defines a function
//!  named `generate_bindings` which can be used to generate all constants
//!  and functions of a given OpenGL version.
//!
//! ## Example
//!
//! In `build.rs`:
//!
//! ```no_run
//! extern crate gl_generator;
//!
//! use gl_generator::{Fallbacks, GlobalGenerator, Api};
//! use std::env;
//! use std::fs::File;
//! use std::path::Path;
//!
//! fn main() {
//!     let dest = env::var("OUT_DIR").unwrap();
//!     let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
//!
//!     gl_generator::generate_bindings(GlobalGenerator, Api::Gl, Fallbacks::All,
//!                                     vec![], "4.5", "core", &mut file).unwrap();
//! }
//! ```
//!
//! In your project:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
//! ```
//!
//! ## Arguments
//!
//! Each field can be specified at most once, or not at all. If the field is not
//! specified, then a default value will be used.
//!
//! - `api`: The API to generate. Can be either `"gl"`, `"gles1"`, `"gles2"`,
//!   `"wgl"`, `"glx"`, `"egl"`. Defaults to `"gl"`.
//! - `profile`: Can be either `"core"` or `"compatibility"`. Defaults to
//!   `"core"`. `"core"` will only include all functions supported by the
//!   requested version it self, while `"compatibility"` will include all the
//!   functions from previous versions as well.
//! - `version`: The requested API version. This is usually in the form
//!   `"major.minor"`. Defaults to `"1.0"`
//! - `generator`: The type of loader to generate. Can be either `"static"`,
//!   `"global"`, or `"struct"`. Defaults to `"static"`.
//! - `extensions`: Extra extensions to include in the bindings. These are
//!   specified as a list of strings. Defaults to `[]`.
//!
//! ## About EGL
//!
//! When you generate bindings for EGL, the following platform-specific types must be declared
//!  *at the same level where you include the bindings*:
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

extern crate khronos_api;
#[macro_use]
extern crate log;

use generators::Generator;
use registry::{Registry, Filter};

use std::io;

pub use registry::{Fallbacks, Api};
pub use generators::debug_struct_gen::DebugStructGenerator;
pub use generators::global_gen::GlobalGenerator;
pub use generators::static_gen::StaticGenerator;
pub use generators::static_struct_gen::StaticStructGenerator;
pub use generators::struct_gen::StructGenerator;

pub mod generators;

#[allow(dead_code)]
pub mod registry;

/// Public function that generates Rust source code.
pub fn generate_bindings<G, W>(generator: G, api: registry::Api, fallbacks: Fallbacks,
                               extensions: Vec<String>, version: &str, profile: &str,
                               dest: &mut W) -> io::Result<()> where G: Generator, W: io::Write
{
    // Get generator field values, using default values if they have not been
    // specified
    let filter = Filter {
        api: api,
        fallbacks: fallbacks,
        extensions: extensions,
        version: version.to_string(),
        profile: profile.to_string(),
    };

    // Generate the registry of all bindings
    let registry = {
        use std::io::BufReader;

        let src = match api {
            Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => khronos_api::GL_XML,
            Api::Glx => khronos_api::GLX_XML,
            Api::Wgl => khronos_api::WGL_XML,
            Api::Egl => khronos_api::EGL_XML,
        };

        let reader = BufReader::new(src);
        Registry::from_xml(reader, api, filter)
    };

    generator.write(&registry, dest)
}
