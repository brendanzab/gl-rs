#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

mod gl {
    generate_gl_bindings!("gl", "core", "4.5", "global")
}

mod gles {
    generate_gl_bindings!("gles2", "core", "3.1", "global")
}

mod glx {
    generate_gl_bindings!("glx", "core", "1.4", "global")
}

mod wgl {
    generate_gl_bindings!("wgl", "core", "1.0", "global")
}

mod egl {
    #![allow(non_camel_case_types)]

    use libc;

    pub type khronos_utime_nanoseconds_t = libc::c_int;
    pub type khronos_uint64_t = libc::uint64_t;
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings!("egl", "core", "1.5", "global")
}

#[test]
#[ignore]
fn test_gl() {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    let _: libc::c_uint = gl::CreateProgram();
    gl::CompileShader(5);

    unsafe {
        gl::GetActiveUniformBlockiv(0, 0, gl::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
            std::ptr::null_mut());
    }
}

#[test]
#[ignore]
fn test_gles() {
    gles::Clear(gles::COLOR_BUFFER_BIT);
    let _: libc::c_uint = gles::CreateProgram();
    gles::CompileShader(5);
}

#[test]
#[ignore]
fn test_glx() {
    let _ = unsafe { glx::GetProcAddress(std::mem::uninitialized()) };
    unsafe { glx::SwapBuffers(std::mem::uninitialized(), std::mem::uninitialized()) };
}

#[test]
#[ignore]
fn test_wgl() {
    let _: wgl::types::HGLRC = unsafe { wgl::CreateContext(std::mem::uninitialized()) };
}

#[test]
#[ignore]
fn test_egl() {
    let _ = [
        egl::SURFACE_TYPE, egl::WINDOW_BIT,
        egl::BLUE_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::RED_SIZE, 8,
        egl::NONE
    ];

    let _ = egl::GetDisplay(egl::DEFAULT_DISPLAY);
    egl::Terminate(unsafe { std::mem::uninitialized() });
}
