#[link(name = "glgen",
       author = "Brendan Zabarauskas",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];

//! Requires libxml2
//!
//! This will be used to generate the loader from the [registry xml files]
//! (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/):
//!
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/glx.xml`
//! - `$ wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/wgl.xml`

extern mod extra;

use std::io;
use std::os;

use registry::Registry;

pub mod registry;
pub mod ty_conv;

fn main() {
    let args = os::real_args();
    let path_str = args[1].as_slice();

    // Parse the XML registry.
    let reg = Registry::from_xml(
        io::file_reader(&Path(path_str))
            .expect(fmt!("Could not read %s", path_str))
            .read_c_str()
    );

    // Debug output
    if args.len() > 2 {
        match args[2].as_slice() {
            "--reg" => {
                // Print out the registry data for debugging.
                printfln!("%?", reg);
            }
            "--ctys" => {
                // Print out a list of all the types used in the registry.
                let tys = reg.get_tys();
                for ty in tys.iter() {
                    printfln!("\"%s\"", *ty);
                }
            }
            "--rtys" => {
                // Print out a list of all the types used in the registry,
                // converted to their Rust form.
                let tys = reg.get_tys();
                for ty in tys.iter() {
                    printfln!("\"%s\"", ty_conv::from_cty(*ty));
                }
            }
            _ => {}
        }
    }

    // TODO: Use registry data to generate function loader.
}
