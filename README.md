_NOTE: this loader does not currently work with the latest version of Rust but will soon. This is because we are waiting for external function pointer calling to be implemented. This should land soon though, so stay tuned!_

# gl-rs

This is an OpenGL function pointer loader for the Rust Programming Language. It was generated using a [modified version of glLoadGen](https://github.com/bjz/glLoadGen) and provides access to functions, types and constants specified in the 4.2 core profile. If you need a compatibility profile or certain extensions not specified here, you can use the loader generator directly by cloning it from its [repository](https://github.com/bjz/glLoadGen).

## Usage

### Importing gl-rs

~~~rust
extern mod gl (name = "gl_core_4_2");
use gl;
use gl::types::*;
~~~

### Initialising the loader

You can load each OpenGL symbol into a new struct by using the `load_with`function. You must supply a loader function from your context library, for example, `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.

This is an example of what it would look like using [glfw-rs](https://github.com/bjz/glfw-rs):

~~~rust
let gl = gl::load_with(glfw::get_proc_address);
~~~

### Checking if a function is loaded

Each function pointer has a boolean value associated with it, allowing you to check if a function is loaded at run time.

~~~rust
if gl.Viewport.is_loaded {
    // ...
}
~~~

### Calling a function

~~~rust
gl.Viewport(0, 0, 600, 480);
~~~

If the function was not loaded when `load_with` was called, the function will fail with an error:

~~~rust
Function glTexStorage1D not initialised
~~~

The failing function was set back when `load_with` was called, so there is no run-time overhead for this functionality.

### Passing the loader to another function

~~~rust
fn render_resize(gl: &gl::GL, width: GLint, height: GLint) {
    gl.Viewport(0, 0, width, height);
}

render_resize(&gl, 600, 480);
~~~