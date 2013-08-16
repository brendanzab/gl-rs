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
use std::util;

use registry::Registry;

pub mod registry;
pub mod ty_conv;

fn main() {
    match os::real_args() {
        [_, ref path, ..args] => {
            // Parse the XML registry.
            let reg = Registry::from_xml(
                io::file_reader(&Path(path.as_slice()))
                    .expect(fmt!("Could not read %s", *path))
                    .read_c_str()
            );
            parse_args(args, &reg);
        }
        [_] => println("Error: expected a path to an XML file."),
        [] => util::unreachable(),
    }
    // TODO: Use registry data to generate function loader.
}

fn parse_args(args: &[~str], reg: &Registry) {
    match args {
        [~"--reg", ..tl] => {
            print_registry(reg);
            parse_args(tl, reg);
        }
        [~"--ctys", ..tl] => {
            print_ctys(reg);
            parse_args(tl, reg);
        }
        [~"--rtys", ..tl] => {
            print_rtys(reg);
            parse_args(tl, reg);
        }
        [ref flag] => printfln!("Error: unexpected argument `%s`.", *flag),
        [] => (),
    }
}

/// Print out the registry data for debugging.
fn print_registry(reg: &Registry) {
    printfln!("%?", reg);
}

/// Print out a list of all the types used in the registry.
fn print_ctys(reg: &Registry) {
    let tys = reg.get_tys();
    for ty in tys.iter() {
        printfln!("\"%s\"", *ty);
    }
}

/// Print out a list of all the types used in the registry, converted to their
/// Rust declaration syntax.
fn print_rtys(reg: &Registry) {
    let tys = reg.get_tys();
    for ty in tys.iter() {
        printfln!("\"%s\"", ty_conv::from_cty(*ty));
    }
}
