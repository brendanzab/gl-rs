# gl-rs

An OpenGL function pointer loader for the Rust Programming Language.

## Basic usage

The easiest way to use OpenGL is to simply include the `gl` crate.

To do so, add this to your `Cargo.toml`:

```toml
[dependencies.gl]
git = "https://github.com/bjz/gl-rs"
```

You can import the pointer style loader and type aliases like so:

~~~rust
extern crate gl;
// include the OpenGL type aliases
use gl::types::*;
~~~

You must load the function pointers into their respective function pointers
using the `load_with` function. You must supply a loader function from your
context library, This is how it would look using [glfw-rs]
(https://github.com/bjz/glfw-rs):

~~~rust
// the supplied function must be of the type:
// `&fn(symbol: &str) -> Option<extern "C" fn()>`
// `window` is a glfw::Window
gl::load_with(|s| window.get_proc_address(s));

// loading a specific function pointer
gl::Viewport::load_with(|s| window.get_proc_address(s));
~~~

Calling a function that has not been loaded will result in a failure like:
`panic!("gl::Viewport was not loaded")`, which avoids a segfault. This feature
does not cause any run time overhead because the failing functions are
assigned only when `load_with` is called.

~~~rust
// accessing an enum
gl::RED_BITS;

// calling a function
gl::DrawArrays(gl::TRIANGLES, 0, 3);

// functions that take pointers are unsafe
unsafe {  gl::ShaderSource(shader, 1, &c_str, std::ptr::null()) };
~~~

Each function pointer has an associated boolean value allowing you to
check if a function has been loaded at run time. The function accesses a
corresponding global boolean that is set when `load_with` is called, so there
shouldn't be much overhead.

~~~rust
if gl::Viewport::is_loaded() {
    // do something...
}
~~~

## Extra features

The global and struct generators will attempt to use fallbacks functions when
they are available. For example, if `glGenFramebuffers` cannot be loaded it will
also attempt to load `glGenFramebuffersEXT` as a fallback.

## Using gl_generator

If you need a specific version of OpenGL, or you need a different API
(OpenGL ES, EGL, WGL, GLX), or if you need certain extensions, you should use
the `gl_generator` plugin instead.

See [gfx_gl](https://github.com/gfx-rs/gfx_gl) for an example of using a
custom gfx-rs loader for a project.

Add this to your `Cargo.toml`:

~~~toml
[build-dependencies]
gl_generator = "*"

[dependencies]
gl_common = "*"
~~~

Under the `[package]` section, add:

~~~toml
build = "build.rs"
~~~

Create a `build.rs` to pull your specific version/API:

~~~rust
extern crate gl_generator;    // <-- this is your build dependency
extern crate khronos_api;    // included by gl_generator

use std::os;
use std::io::File;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    // This generates bindsings for OpenGL ES v3.1
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    khronos_api::GL_XML,
                                    vec![],
                                    "3.1", "core", &mut file).unwrap();
}
~~~

Then use it like this:

~~~rust
mod gles {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

/*
 * Simple loading example
 */
fn main() {
    // Assuming window is GLFW, initialized, and made current
    gles::load_with(|s| window.get_proc_address(s));
}
~~~

The `build.rs` file will generate all the OpenGL functions in a file named,
`gl_bindings.rs` plus all enumerations, and all types in the `types` submodule.

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
OpenGL 1.3 on Linux, because Windows and Linux are guanteed to provide
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
