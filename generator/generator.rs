#[link(name = "glgen",
       author = "Brendan Zabarauskas",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];

// Requires libxml2

// This will be used to generate the loader from the [registry xml files]
// (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/)

extern mod extra;

use std::io::file_reader;

use registry::Registry;

pub mod registry;

fn main() {
    // Parse the XML registry. This is currently hardcoded to look for `gl.xml`
    // in the current working directory.
    let reg = Registry::from_xml(
        file_reader(&Path("gl.xml"))
            .expect("Could not find gl.xml")
            .read_c_str()
    );

    // TODO: Use registry data to generate function loader.

    // Print out the registry data for debugging.
    printfln!("%?", reg);
}
