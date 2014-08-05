// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the \"License\
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Usage
//! 
//! You can import the pointer style loader and type aliases like so:
//! 
//! ~~~rust
//! extern crate gl;
//! // include the OpenGL type aliases
//! use gl::types::*;
//! ~~~
//! 
//! You can load the function pointers into their respective function pointers
//! using the `load_with` function. You must supply a loader function from your
//! context library, This is how it would look using [glfw-rs]
//! (https://github.com/bjz/glfw-rs):
//! 
//! ~~~rust
//! // the supplied function must be of the type:
//! // `&fn(symbol: &str) -> Option<extern "C" fn()>`
//! gl::load_with(|s| glfw.get_proc_address(s));
//! 
//! // loading a specific function pointer
//! gl::Viewport::load_with(|s| glfw.get_proc_address(s));
//! ~~~
//! 
//! Calling a function that has not been loaded will result in a failure like:
//! `fail!("gl::Viewport was not loaded")`, which aviods a segfault. This feature
//! does not cause any run time overhead because the failing functions are
//! assigned only when `load_with` is called.
//! 
//! ~~~rust
//! // accessing an enum
//! gl::RED_BITS;
//! 
//! // calling a function
//! gl::DrawArrays(gl::TRIANGLES, 0, 3);
//! 
//! // functions that take pointers are unsafe
//! unsafe { gl::ShaderSource(shader, 1, &c_str, std::ptr::null()) };
//! ~~~
//! 
//! Each function pointer has an associated boolean value allowing you to
//! check if a function has been loaded at run time. The function accesses a
//! corresponding global boolean that is set when `load_with` is called, so there
//! shouldn't be much overhead.
//! 
//! ~~~rust
//! if gl::Viewport::is_loaded() {
//!     // do something...
//! }
//! ~~~
//!

#![crate_name = "gl"]
#![comment = "An OpenGL function loader."]
#![license = "ASL2"]
#![crate_type = "lib"]

#![feature(macro_rules)]
#![feature(globs)]
#![feature(phase)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]
#![allow(unused_variable)]

#[phase(plugin)]
extern crate gl_generator;
extern crate libc;

use std::mem;
use self::types::*;

generate_gl_bindings!("gl", "core", "4.5", "static", [ "GL_EXT_texture_filter_anisotropic" ])
