#[link(name = "glgen",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];

// Requires libxml2

// This will be used to generate the loader from the [registry xml files]
// (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/)

extern mod extra;
extern mod rust_curl;

pub mod xml;
pub mod registry;

fn main() {
    let _ = registry::parse_xml(
        registry::downoad_src(
            Path("gl.xml"),
            "https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml",
            true
        )
    );
    // let _ = registry::parse_xml("<hi></hi>".as_bytes());
}
