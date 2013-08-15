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

use extra::treemap::TreeSet;
use std::io::file_reader;
use std::os;

use registry::Registry;

pub mod registry;

fn main() {
    let args = os::real_args();
    let path_str = args[1].as_slice();

    // Parse the XML registry.
    let reg = Registry::from_xml(
        file_reader(&Path(path_str))
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
            "--tys" => {
                // Print out a list of all the types used in the registry.
                print_types(&reg);
            }
            _ => {}
        }
    }

    // TODO: Use registry data to generate function loader.
}

fn gl_ty_conv(ty: &str) -> ~str {
    match ty {
        "GLDEBUGPROC"               => ~"GLDEBUGPROC",
        "GLDEBUGPROCAMD"            => ~"GLDEBUGPROCAMD",
        "GLDEBUGPROCARB"            => ~"GLDEBUGPROCARB",
        "GLDEBUGPROCKHR"            => ~"GLDEBUGPROCKHR",
        "GLbitfield"                => ~"GLbitfield",
        "GLboolean"                 => ~"GLboolean",
        "GLbyte"                    => ~"GLbyte",
        "GLclampd"                  => ~"GLclampd",
        "GLclampf"                  => ~"GLclampf",
        "GLclampx"                  => ~"GLclampx",
        "GLdouble"                  => ~"GLdouble",
        "GLeglImageOES"             => ~"GLeglImageOES",
        "GLenum"                    => ~"GLenum",
        "GLfixed"                   => ~"GLfixed",
        "GLfloat"                   => ~"GLfloat",
        "GLhalfNV"                  => ~"GLhalfNV",
        "GLhandleARB"               => ~"GLhandleARB",
        "GLint"                     => ~"GLint",
        "GLint64EXT"                => ~"GLint64EXT",
        "GLintptr"                  => ~"GLintptr",
        "GLintptrARB"               => ~"GLintptrARB",
        "GLshort"                   => ~"GLshort",
        "GLsizei"                   => ~"GLsizei",
        "GLsizeiptr"                => ~"GLsizeiptr",
        "GLsizeiptrARB"             => ~"GLsizeiptrARB",
        "GLsync"                    => ~"GLsync",
        "GLubyte"                   => ~"GLubyte",
        "GLuint"                    => ~"GLuint",
        "GLuint64"                  => ~"GLuint64",
        "GLuint64EXT"               => ~"GLuint64EXT",
        "GLushort"                  => ~"GLushort",
        "GLvdpauSurfaceNV"          => ~"GLvdpauSurfaceNV",
        "GLboolean *"               => ~"*GLboolean",
        "GLchar *"                  => ~"*GLchar",
        "GLcharARB *"               => ~"*GLcharARB",
        "GLdouble *"                => ~"*GLdouble",
        "GLenum *"                  => ~"*GLenum",
        "GLfixed *"                 => ~"*GLfixed",
        "GLfloat *"                 => ~"*GLfloat",
        "GLhandleARB *"             => ~"*GLhandleARB",
        "GLint *"                   => ~"*GLint",
        "GLint64 *"                 => ~"*GLint64",
        "GLint64EXT *"              => ~"*GLint64EXT",
        "GLsizei *"                 => ~"*GLsizei",
        "GLubyte *"                 => ~"*GLubyte",
        "GLuint *"                  => ~"*GLuint",
        "GLuint64 *"                => ~"*GLuint64",
        "GLuint64EXT *"             => ~"*GLuint64EXT",
        "GLushort *"                => ~"*GLushort",
        "GLvoid *"                  => ~"*GLvoid",
        "GLvoid **"                 => ~"**GLvoid",
        "const GLboolean *"         => ~"*GLboolean",
        "const GLbyte *"            => ~"*GLbyte",
        "const GLchar *"            => ~"*GLchar",
        "const GLcharARB *"         => ~"*GLcharARB",
        "const GLclampf *"          => ~"*GLclampf",
        "const GLdouble *"          => ~"*GLdouble",
        "const GLenum *"            => ~"*GLenum",
        "const GLfixed *"           => ~"*GLfixed",
        "const GLfloat *"           => ~"*GLfloat",
        "const GLhalfNV *"          => ~"*GLhalfNV",
        "const GLint *"             => ~"*GLint",
        "const GLint64EXT *"        => ~"*GLint64EXT",
        "const GLintptr *"          => ~"*GLintptr",
        "const GLshort *"           => ~"*GLshort",
        "const GLsizei *"           => ~"*GLsizei",
        "const GLsizeiptr *"        => ~"*GLsizeiptr",
        "const GLubyte *"           => ~"*GLubyte",
        "const GLuint *"            => ~"*GLuint",
        "const GLuint64 *"          => ~"*GLuint64",
        "const GLuint64EXT *"       => ~"*GLuint64EXT",
        "const GLushort *"          => ~"*GLushort",
        "const GLvdpauSurfaceNV *"  => ~"*GLvdpauSurfaceNV",
        "const GLvoid *"            => ~"*GLvoid",
        "const void *"              => ~"*c_void",
        "const GLboolean **"        => ~"**GLboolean",
        "const GLchar **"           => ~"**GLchar",
        "const GLcharARB **"        => ~"**GLcharARB",
        "const GLvoid **"           => ~"**GLvoid",
        "const GLchar *const*"      => ~"**GLchar",
        "const GLvoid *const*"      => ~"**GLvoid",
        "struct _cl_context *"      => ~"*_cl_context",
        "struct _cl_event *"        => ~"*_cl_event",
        "void "                     => ~"c_void",
        "void *"                    => ~"*c_void",
        _                           => fail!("Type conversion not yet implemented for `%s`", ty),
    }
}

fn print_types(reg: &Registry) {
    let mut tys = TreeSet::new();
    for cmds in reg.cmds.iter() {
        for def in cmds.defs.iter() {
            tys.insert(&def.proto.ty);
            for param in def.params.iter() {
                tys.insert(&param.ty);
            }
        }
    }
    for ty in tys.iter() {
        printfln!("        \"%s\"", **ty);
    }
}
