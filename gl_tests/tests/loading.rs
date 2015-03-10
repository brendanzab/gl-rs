extern crate libc;
extern crate gl_tests;
extern crate gl_common;

struct X;

impl gl_common::GlFunctionsSource for X {
    fn get_proc_addr(&self, _: &str) -> *const libc::c_void {
        std::ptr::null()
    }
}

#[test]
fn main() {
    gl_tests::load(&X);
}
