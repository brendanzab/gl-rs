# gl-rs

[![Version](https://img.shields.io/crates/v/gl.svg)](https://crates.io/crates/gl)
[![License](https://img.shields.io/crates/l/gl.svg)](https://github.com/brendanzab/gl-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/gl.svg)](https://crates.io/crates/gl)

An OpenGL function pointer loader for the Rust Programming Language.

```toml
[dependencies]
gl = "0.6.0"
```

## Basic usage

You can import the pointer style loader and type aliases like so:

```rust
extern crate gl;
// include the OpenGL type aliases
use gl::types::*;
```

You must load the function pointers into their respective function pointers
using the `load_with` function. You must supply a loader function from your
context library, This is how it would look using [glfw-rs]
(https://github.com/PistonDevelopers/glfw-rs):

```rust
// the supplied function must be of the type:
// `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
// `window` is a glfw::Window
gl::load_with(|s| window.get_proc_address(s) as *const _);

// loading a specific function pointer
gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);
```

Calling a function that has not been loaded will result in a failure like:
`panic!("gl::Viewport was not loaded")`, which avoids a segfault. This feature
does not cause any run time overhead because the failing functions are
assigned only when `load_with` is called.

```rust
// accessing an enum
gl::RED_BITS;

// calling a function
gl::DrawArrays(gl::TRIANGLES, 0, 3);

// functions that take pointers are unsafe
unsafe {  gl::ShaderSource(shader, 1, &c_str, std::ptr::null()) };
```

Each function pointer has an associated boolean value allowing you to
check if a function has been loaded at run time. The function accesses a
corresponding global boolean that is set when `load_with` is called, so there
shouldn't be much overhead.

```rust
if gl::Viewport::is_loaded() {
    // do something...
}
```

## Changelog

### v0.6.0

- Upgrade to `gl_generator` v0.5.0

### v0.5.2

- Update crate metadata

### v0.5.1

- Upgrade `khronos_api` to v1.0.0

### v0.5.0

- Use `glutin` for examples
- Use `raw::c_void` for `GLvoid`
