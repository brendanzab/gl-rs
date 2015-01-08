#![feature(plugin)]

#[plugin]
extern crate gl_generator;

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/test_gen_symbols.rs"));

#[test]
#[ignore]
fn test_gl() { unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    let _: libc::c_uint = gl::CreateProgram();
    gl::CompileShader(5);

    gl::GetActiveUniformBlockiv(0, 0, gl::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
        std::ptr::null_mut());

    let _: *mut libc::c_void = gl::MapBuffer(0, 0);
}}

#[test]
#[ignore]
fn test_gles() { unsafe {
    gles::Clear(gles::COLOR_BUFFER_BIT);
    let _: libc::c_uint = gles::CreateProgram();
    gles::CompileShader(5);
}}

#[test]
#[ignore]
fn test_glx() { unsafe {
    let _ = glx::GetProcAddress(std::mem::uninitialized());
    glx::SwapBuffers(std::mem::uninitialized(), std::mem::uninitialized());
}}

#[test]
#[ignore]
fn test_wgl() { unsafe {
    let _: wgl::types::HGLRC = wgl::CreateContext(std::mem::uninitialized());
}}

#[test]
#[ignore]
fn test_egl() { unsafe {
    let _ = [
        egl::SURFACE_TYPE, egl::WINDOW_BIT,
        egl::BLUE_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::RED_SIZE, 8,
        egl::NONE
    ];

    let _ = egl::GetDisplay(egl::DEFAULT_DISPLAY);
    egl::Terminate(std::mem::uninitialized());
}}
