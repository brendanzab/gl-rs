extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::old_io::File;
use std::old_io::BufferedWriter;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = BufferedWriter::new(File::create(&dest.join("bindings.rs")).unwrap());
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
}
