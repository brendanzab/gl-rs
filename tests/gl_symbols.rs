//! This file contains a single test that is
//!  compiled but not run, just to ensure that
//!  the GL symbols are defined

extern crate gl;
extern crate libc;

#[test]
#[ignore]
fn test() {
	gl::Clear(gl::COLOR_BUFFER_BIT);
	let _: libc::c_uint = gl::CreateProgram();
	gl::CompileShader(5);

    unsafe {
        gl::GetActiveUniformBlockiv(0, 0, gl::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
            std::ptr::mut_null());
    }
}

