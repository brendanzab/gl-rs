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
gl::load_with(|s| glfw.get_proc_address(s));

// loading a specific function pointer
gl::Viewport::load_with(|s| glfw.get_proc_address(s));
~~~

Calling a function that has not been loaded will result in a failure like:
`fail!("gl::Viewport was not loaded")`, which avoids a segfault. This feature
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

## Using gl_generator

If you need a specific version of OpenGL, or you need a different API
(OpenGL ES, EGL, WGL, GLX), or if you need certain extensions, you should use
the `gl_generator` plugin instead.

See [gfx_gl](https://github.com/gfx-rs/gfx_gl) for an example of using a
custom gfx-rs loader for a project.

Add this to your `Cargo.toml`:

~~~toml
[dependencies.gl_generator]
git = "https://github.com/bjz/gl-rs"
~~~

Then use it like this:

~~~rust
#[phase(plugin)]
extern crate gl_generator;
extern crate libc;

use std::mem;
use self::types::*;

generate_gl_bindings! {
    api: gl,
    profile: core,
    version: 4.5,
    generator: global,
    extensions: [
        GL_EXT_texture_filter_anisotropic,
    ],
}
~~~

The `generate_gl_bindings` macro will generate all the OpenGL functions,
plus all enumerations, plus all types in the `types` submodule.

### Field Syntax

Each field can be specified at most once, or not at all. If the field is not
specified, then a default value will be used. The syntax for each field is
as follows (in [EBNF](http://en.wikipedia.org/wiki/Extended_Backus-Naur_Form)):

~~~ebnf
api        = "api:",        ("gl" | "gles1" | "gles2" | "wgl" | "glx" | "egl")
profile    = "profile:",    ("core" | "compatibility")
version    = "version:",    digit, ".", digit
generator  = "generator:",  ("static" | "global" | "struct")
extensions = "extensions:", "[", [ [ ident ], { ",", ident } ], [ "," ], "]"
~~~

#### Notes

- `api`: The API to generate. Defaults to `gl`.
- `profile`: Defaults to `core`. `core` will only include all functions
   supported by the requested version it self, while `compatibility` will
   include all the functions from previous versions as well.
- `version`: The requested API version. Defaults to `1.0`.
- `generator`: The type of loader to generate. Defaults to `static`.
- `extensions`: Extra extensions to include in the bindings. Defaults to `[]`.

### Global generator

The global generator is the one used by default by the `gl` crate. See above
for more details.

### Struct generator

The struct generator is a cleaner alternative to the global generator.

The main difference is that you must call `gl::Gl::load_with` instead of
`gl::load_with`, and this functions returns a struct of type `Gl`. The OpenGL
functions are not static functions but member functions in this `Gl` struct.

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
