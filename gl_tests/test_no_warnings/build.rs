// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
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

extern crate gl_generator;

use gl_generator::*;
use gl_generator::generators::Generator;
use gl_generator::registry::Registry;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::*;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("test_no_warnings.rs")).unwrap();

    // Gl

    let gl_registry = Registry::new(Api::Gl, Fallbacks::All, vec![], "4.5", Profile::Core);

    writeln!(&mut file, "mod gl_global {{").unwrap();
    GlobalGenerator.write(&gl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gl_static {{").unwrap();
    StaticGenerator.write(&gl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gl_struct {{").unwrap();
    StructGenerator.write(&gl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gl_static_struct {{").unwrap();
    StaticStructGenerator.write(&gl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gl_debug_struct {{").unwrap();
    DebugStructGenerator.write(&gl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    // Glx

    let glx_registry = Registry::new(Api::Glx, Fallbacks::All, vec![], "1.4", Profile::Core);

    writeln!(&mut file, "mod glx_global {{").unwrap();
    GlobalGenerator.write(&glx_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod glx_static {{").unwrap();
    StaticGenerator.write(&glx_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod glx_struct {{").unwrap();
    StructGenerator.write(&glx_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod glx_static_struct {{").unwrap();
    StaticStructGenerator.write(&glx_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod glx_debug_struct {{").unwrap();
    DebugStructGenerator.write(&glx_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    // Wgl

    let wgl_registry = Registry::new(Api::Wgl, Fallbacks::All, vec![], "1.0", Profile::Core);

    writeln!(&mut file, "mod wgl_global {{").unwrap();
    GlobalGenerator.write(&wgl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod wgl_static {{").unwrap();
    StaticGenerator.write(&wgl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod wgl_struct {{").unwrap();
    StructGenerator.write(&wgl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod wgl_static_struct {{").unwrap();
    StaticStructGenerator.write(&wgl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod wgl_debug_struct {{").unwrap();
    DebugStructGenerator.write(&wgl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    // Gles1

    let gles1_registry = Registry::new(Api::Gles1, Fallbacks::All, vec![], "1.1", Profile::Core);

    writeln!(&mut file, "mod gles1_global {{").unwrap();
    GlobalGenerator.write(&gles1_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles1_static {{").unwrap();
    StaticGenerator.write(&gles1_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles1_struct {{").unwrap();
    StructGenerator.write(&gles1_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles1_static_struct {{").unwrap();
    StaticStructGenerator.write(&gles1_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles1_debug_struct {{").unwrap();
    DebugStructGenerator.write(&gles1_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    // Gles2

    let gles2_registry = Registry::new(Api::Gles2, Fallbacks::All, vec![], "3.1", Profile::Core);

    writeln!(&mut file, "mod gles2_global {{").unwrap();
    GlobalGenerator.write(&gles2_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles2_static {{").unwrap();
    StaticGenerator.write(&gles2_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles2_struct {{").unwrap();
    StructGenerator.write(&gles2_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles2_static_struct {{").unwrap();
    StaticStructGenerator.write(&gles2_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod gles2_debug_struct {{").unwrap();
    DebugStructGenerator.write(&gles2_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    // Egl

    let egl_registry = Registry::new(Api::Egl, Fallbacks::All, vec![], "1.5", Profile::Core);

    writeln!(&mut file, "mod egl_global {{ {}", build_egl_symbols()).unwrap();
    GlobalGenerator.write(&egl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod egl_static {{ {}", build_egl_symbols()).unwrap();
    StaticGenerator.write(&egl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod egl_struct {{ {}", build_egl_symbols()).unwrap();
    StructGenerator.write(&egl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod egl_static_struct {{ {}", build_egl_symbols()).unwrap();
    StaticStructGenerator.write(&egl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "mod egl_debug_struct {{ {}", build_egl_symbols()).unwrap();
    DebugStructGenerator.write(&egl_registry, &mut file).unwrap();
    writeln!(&mut file, "}}").unwrap();
}

fn build_egl_symbols() -> &'static str {
    "
        #![allow(non_camel_case_types)]

        use std::os::raw;

        pub type khronos_utime_nanoseconds_t = raw::c_int;
        pub type khronos_uint64_t = u64;
        pub type khronos_ssize_t = isize;
        pub type EGLNativeDisplayType = *const raw::c_void;
        pub type EGLNativePixmapType = *const raw::c_void;
        pub type EGLNativeWindowType = *const raw::c_void;
        pub type EGLint = raw::c_int;
        pub type NativeDisplayType = *const raw::c_void;
        pub type NativePixmapType = *const raw::c_void;
        pub type NativeWindowType = *const raw::c_void;
    "
}
