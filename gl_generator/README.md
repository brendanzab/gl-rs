# gl_generator

[![Version](https://img.shields.io/crates/v/gl_generator.svg)](https://crates.io/crates/gl_generator)
[![License](https://img.shields.io/crates/l/gl_generator.svg)](https://github.com/bjz/gl-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/gl_generator.svg)](https://crates.io/crates/gl_generator)

Code generators for creating bindings to the Khronos OpenGL APIs.

## Usage

If you need a specific version of OpenGL, or you need a different API
(OpenGL ES, EGL, WGL, GLX), or if you need certain extensions, you should use
the `gl_generator` plugin instead.

See [gfx_gl](https://github.com/gfx-rs/gfx_gl) for an example of using a
custom gfx-rs loader for a project.

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
gl_generator = "0.4.2"
khronos_api = "1.0.0"
```

Under the `[package]` section, add:

```toml
build = "build.rs"
```

Create a `build.rs` to pull your specific version/API:

```rust
extern crate gl_generator;
extern crate khronos_api;

use gl_generator::{Fallbacks, GlobalGenerator, Ns};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir);

    let mut file = BufWriter::new(File::create(&dest.join("bindings.rs")).unwrap());
    gl_generator::generate_bindings(GlobalGenerator, Ns::Gl, Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
}
```

Then use it like this:

```rust
mod gles {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/*
 * Simple loading example
 */
fn main() {
    // Assuming window is GLFW, initialized, and made current
    gles::load_with(|s| window.get_proc_address(s));
}
```

The `build.rs` file will generate all the OpenGL functions in a file named,
`bindings.rs` plus all enumerations, and all types in the `types` submodule.

### Arguments

- The type of loader to generate. Can be
  `gl_generator::StaticGenerator`, `gl_generator::StaticStructGenerator`,
  `gl_generator::StructGenerator`, or `gl_generator::GlobalGenerator`.
- The API to generate. Can be `Gl`, `Gles1`, `Gles2`
  (GLES 2 or 3), `Wgl`, `Glx`, `Egl`.
- The file which contains the bindings to parse. Can be `GL_XML` (for GL
  and GL ES), `GLX_XML`, `WGL_XML`, `EGL_XML`.
- Extra extensions to include in the bindings. These are
  specified as a list of strings.
- The requested API version. This is usually in the form
  `"major.minor"`.
- The GL profile. Can be either `"core"` or `"compatibility"`. `"core"` will
  only include all functions supported by the
  requested version it self, while `"compatibility"` will include all the
  functions from previous versions as well.
- The file to save the generated bindings to.

## Generator types

### Global generator

The global generator is the one used by default by the `gl` crate. See above
for more details.

### Struct generator

The struct generator is a cleaner alternative to the global generator.

The main difference is that you must call `gl::Gl::load_with` instead of
`gl::load_with`, and this functions returns a struct of type `Gl`. The OpenGL
functions are not static functions but member functions in this `Gl` struct.
This is important when the GL functions are associated with the current
context, as is true on Windows.

The enumerations and types are still static and available in a similar way as
in the global generator.

### Static generator

The static generator generates plain old bindings. You don't need to load the
functions.

This generator should only be used only if the platform you are compiling for
is guaranteed to support the requested API. Otherwise you will get a
compilation error.
For example, you can use it for WGL and OpenGL 1.1 on Windows or GLX and
OpenGL 1.3 on Linux, because Windows and Linux are guaranteed to provide
implementations for these APIs.

You will need to manually provide the linkage. For example to use WGL or
OpenGL 1.1 on Windows, you will need to add
`#[link="OpenGL32.lib"] extern {}` somewhere in your code.

### Custom Generators

The `gl_generator` crate is extensible. This is a niche feature useful only in
very rare cases. To create a custom generator, [create a new plugin
crate](http://doc.rust-lang.org/guide-plugin.html#syntax-extensions) which
depends on `gl_generator`. Then, implement the `gl_generator::Generator` trait
and in your plugin registrar, register a function which calls
`gl_generator::generate_bindings` with your custom generator and its name.

## Extra features

The global and struct generators will attempt to use fallbacks functions when
they are available. For example, if `glGenFramebuffers` cannot be loaded it will
also attempt to load `glGenFramebuffersEXT` as a fallback.

## Changelog

### v0.4.2

- Update crate metadata

### v0.4.1

- Upgrade `khronos_api` to v1.0.0

### v0.4.0

- Upgrade `xml-rs` to v0.2.2
- Use `raw::c_void` for `GLvoid`
- Remove `registry::{Group, EnumNs, CmdNs}`
- Remove `groups` field from `registry::Registry`
- Remove `is_safe` field from `registry::Cmd`
- Remove `comment` field from `registry::{Require, Remove, GlxOpcode}`
- Downgrade `khronos_api` to be a dev-dependency
