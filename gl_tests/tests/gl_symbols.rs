//! This test ensures that the GL symbols are defined and that fallback works correctly.

extern crate gl_tests;
extern crate libc;

#[test]
#[ignore]
fn symbols_exist() { unsafe {
	gl_tests::Clear(gl_tests::COLOR_BUFFER_BIT);
	let _: libc::c_uint = gl_tests::CreateProgram();
	gl_tests::CompileShader(5);

    gl_tests::GetActiveUniformBlockiv(0, 0, gl_tests::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
        std::ptr::null_mut());
} }

#[test]
fn fallback_works() {
    fn loader(name: &str) -> *const libc::c_void {
        match name {
            "glGenFramebuffers" => 0 as *const libc::c_void,
            "glGenFramebuffersEXT" => 42 as *const libc::c_void,
            name => panic!("test tried to load {} unexpectedly!", name)
        }
    };

    gl_tests::GenFramebuffers::load_with(loader);
    assert!(gl_tests::GenFramebuffers::is_loaded());
}
