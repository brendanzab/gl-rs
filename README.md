# gl-rs

An OpenGL function pointer loader for the Rust Programming Language.

## Compilation

To compile the library, examples, and documentation:

~~~
make
~~~

### Custom loader configuration

The following variables can be customised when compiling the library:

~~~make
GL_NAMESPACE        ?= gl
GL_API              ?= gl
GL_PROFILE          ?= core
GL_VERSION          ?= 4.3
GL_EXTENSIONS       ?=
GL_GENERATOR        ?=
GL_FULL             ?=
GL_XML              ?= ./deps/khronos-api/$(GL_API).xml
~~~

For example:

~~~
GL_VERSION=2.1 GL_PROFILE=compatability make lib
~~~

### Compiling with Rust-Empty

The following command compiles to the folder 'target/cpu-vendor-os/lib':

`make gen-lib && make -f rust-empty.mk`

For more information see [Rust-Empty](https://github.com/bvssvni/rust-empty)

## Usage

You can import the pointer style loader and type aliases like so:

~~~rust
extern crate gl;
// include the OpenGL type aliases
use gl::types::*;
~~~

You can load the function pointers into their respective function pointers
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
`fail!("gl::Viewport was not loaded")`, which aviods a segfault. This feature
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

## Todo

- Make the generator work properly GLX and WGL.
