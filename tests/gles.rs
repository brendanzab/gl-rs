#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

mod gl {
    generate_gl_bindings!("gles2", "core", "3.1", "static")
}

#[test]
#[ignore]
fn test() {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    let _: libc::c_uint = gl::CreateProgram();
    gl::CompileShader(5);
}
