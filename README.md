# gl-rs

An OpenGL function pointer loader for the Rust Programming Language.

## Compilation

~~~
rustpkg build --opt-level=3 gl
~~~

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
gl::load_with(glfw::get_proc_address);

// loading a specific function pointer
gl::Viewport::load_with(glfw::get_proc_address);
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

## Generating the loader

The loader in `gl.rs` is generated using the [XML API Registry]
(http://www.opengl.org/discussion_boards/showthread.php/181927-New-XML-based-API-Registry-released?p=1251775).
If you would like to generate the loader yourself, please refer to the README
in the `src/gen` directory.

## Todo

- Make the generator work properly GLX and WGL.
