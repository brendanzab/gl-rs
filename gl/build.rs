extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = BufWriter::new(File::create(&dest.join("bindings.rs")).unwrap());
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
}
