//! This crates contains the sources of the official OpenGL repository.

/// Content of the official `gl.xml` file.
pub const GL_XML: &'static [u8] = include_bytes!("../api/gl.xml");

/// Content of the official `egl.xml` file.
pub const EGL_XML: &'static [u8] = include_bytes!("../api/egl.xml");

/// Content of the official `wgl.xml` file.
pub const WGL_XML: &'static [u8] = include_bytes!("../api/wgl.xml");

/// Content of the official `glx.xml` file.
pub const GLX_XML: &'static [u8] = include_bytes!("../api/glx.xml");
