//! This crates contains the sources of the official OpenGL repository.

/// Content of the official `gl.xml` file.
pub static GL_XML: &'static str = include_str!("../../../deps/khronos-api/gl.xml");

/// Content of the official `egl.xml` file.
pub static EGL_XML: &'static str = include_str!("../../../deps/khronos-api/egl.xml");

/// Content of the official `wgl.xml` file.
pub static WGL_XML: &'static str = include_str!("../../../deps/khronos-api/wgl.xml");

/// Content of the official `glx.xml` file.
pub static GLX_XML: &'static str = include_str!("../../../deps/khronos-api/glx.xml");
