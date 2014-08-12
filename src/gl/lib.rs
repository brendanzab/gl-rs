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
