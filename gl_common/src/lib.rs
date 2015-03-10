/*!

Defines everything used by `gl_generator`.

*/

extern crate libc;

pub trait GlFunctionsSource {
    fn get_proc_addr(&self, &str) -> *const libc::c_void;
}
