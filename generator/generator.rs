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
            "--ctys" => {
                // Print out a list of all the types used in the registry.
                let tys = get_tys(&reg);
                for ty in tys.iter() {
                    printfln!("\"%s\"", *ty);
                }
            }
            "--rtys" => {
                // Print out a list of all the types used in the registry,
                // converted to rust syntax.
                let tys = get_tys(&reg);
                for ty in tys.iter() {
                    printfln!("\"%s\"", cty_conv(*ty));
                }
            }
            _ => {}
        }
    }

    // TODO: Use registry data to generate function loader.
}

fn cty_conv(ty: &str) -> ~str {
    match ty {
        // gl.xml types
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
        "void "                     => ~"c_void",
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
        "void *"                    => ~"*c_void",
        "const GLboolean **"        => ~"**GLboolean",
        "const GLchar **"           => ~"**GLchar",
        "const GLcharARB **"        => ~"**GLcharARB",
        "const GLvoid **"           => ~"**GLvoid",
        "const GLchar *const*"      => ~"**GLchar",
        "const GLvoid *const*"      => ~"**GLvoid",
        "struct _cl_context *"      => ~"*_cl_context",
        "struct _cl_event *"        => ~"*_cl_event",

        // glx.xml types
        "Bool"                      => ~"Bool",
        "Colormap"                  => ~"Colormap",
        "DMbuffer"                  => ~"DMbuffer",
        "Font"                      => ~"Font",
        "GLXContext"                => ~"GLXContext",
        "GLXContextID"              => ~"GLXContextID",
        "GLXDrawable"               => ~"GLXDrawable",
        "GLXFBConfig"               => ~"GLXFBConfig",
        "GLXFBConfigSGIX"           => ~"GLXFBConfigSGIX",
        "GLXPbuffer"                => ~"GLXPbuffer",
        "GLXPbufferSGIX"            => ~"GLXPbufferSGIX",
        "GLXPixmap"                 => ~"GLXPixmap",
        "GLXVideoCaptureDeviceNV"   => ~"GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV"          => ~"GLXVideoDeviceNV",
        "GLXVideoSourceSGIX"        => ~"GLXVideoSourceSGIX",
        "GLXWindow"                 => ~"GLXWindow",
        // "GLboolean"                 => ~"GLboolean",
        // "GLenum"                    => ~"GLenum",
        // "GLint"                     => ~"GLint",
        // "GLsizei"                   => ~"GLsizei",
        // "GLuint"                    => ~"GLuint",
        "Pixmap"                    => ~"Pixmap",
        "Status"                    => ~"Status",
        "VLNode"                    => ~"VLNode",
        "VLPath"                    => ~"VLPath",
        "VLServer"                  => ~"VLServer",
        "Window"                    => ~"Window",
        "__GLXextFuncPtr"           => ~"__GLXextFuncPtr",
        "const GLXContext"          => ~"const GLXContext",
        "float "                    => ~"c_float",
        "int "                      => ~"c_int",
        "int64_t"                   => ~"i64",
        "unsigned int "             => ~"c_uint",
        "unsigned long "            => ~"c_ulong",
        // "void "                     => ~"c_void",
        "DMparams *"                => ~"*DMparams",
        "Display *"                 => ~"*Display",
        "GLXFBConfig *"             => ~"*GLXFBConfig",
        "GLXFBConfigSGIX *"         => ~"*GLXFBConfigSGIX",
        "GLXHyperpipeConfigSGIX *"  => ~"*GLXHyperpipeConfigSGIX",
        "GLXHyperpipeNetworkSGIX *" => ~"*GLXHyperpipeNetworkSGIX",
        "GLXVideoCaptureDeviceNV *" => ~"*GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV *"        => ~"*GLXVideoDeviceNV",
        // "GLuint *"                  => ~"*GLuint",
        "XVisualInfo *"             => ~"*XVisualInfo",
        // "const GLubyte *"           => ~"*GLubyte",
        "const char *"              => ~"*c_char",
        "const int *"               => ~"*c_int",
        // "const void *"              => ~"*c_void",
        "int *"                     => ~"*c_int",
        "int32_t *"                 => ~"*i32",
        "int64_t *"                 => ~"*i64",
        "long *"                    => ~"*c_long",
        "unsigned int *"            => ~"*c_uint",
        "unsigned long *"           => ~"*c_ulong",
        // "void *"                    => ~"*c_void",

        // wgl.xml types
        "BOOL"                      => ~"BOOL",
        "DWORD"                     => ~"DWORD",
        "FLOAT"                     => ~"FLOAT",
        // "GLbitfield"                => ~"GLbitfield",
        // "GLboolean"                 => ~"GLboolean",
        // "GLenum"                    => ~"GLenum",
        // "GLfloat"                   => ~"GLfloat",
        // "GLint"                     => ~"GLint",
        // "GLsizei"                   => ~"GLsizei",
        // "GLuint"                    => ~"GLuint",
        // "GLushort"                  => ~"GLushort",
        "HANDLE"                    => ~"HANDLE",
        "HDC"                       => ~"HDC",
        "HENHMETAFILE"              => ~"HENHMETAFILE",
        "HGLRC"                     => ~"HGLRC",
        "HGPUNV"                    => ~"HGPUNV",
        "HPBUFFERARB"               => ~"HPBUFFERARB",
        "HPBUFFEREXT"               => ~"HPBUFFEREXT",
        "HPVIDEODEV"                => ~"HPVIDEODEV",
        "HVIDEOINPUTDEVICENV"       => ~"HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV"      => ~"HVIDEOOUTPUTDEVICENV",
        "INT"                       => ~"INT",
        "INT64"                     => ~"INT64",
        "LPCSTR"                    => ~"LPCSTR",
        "LPGLYPHMETRICSFLOAT"       => ~"LPGLYPHMETRICSFLOAT",
        "LPVOID"                    => ~"LPVOID",
        "PGPU_DEVICE"               => ~"PGPU_DEVICE",
        "PROC"                      => ~"PROC",
        "UINT"                      => ~"UINT",
        "VOID"                      => ~"VOID",
        // "int "                      => ~"c_int",
        // "unsigned int "             => ~"c_uint",
        // "void "                     => ~"c_void",
        "BOOL *"                    => ~"*BOOL",
        "DWORD *"                   => ~"*DWORD",
        "FLOAT *"                   => ~"*FLOAT",
        // "GLuint *"                  => ~"*GLuint",
        "HANDLE *"                  => ~"*HANDLE",
        "HGPUNV *"                  => ~"*HGPUNV",
        "HPVIDEODEV *"              => ~"*HPVIDEODEV",
        "HVIDEOINPUTDEVICENV *"     => ~"*HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV *"    => ~"*HVIDEOOUTPUTDEVICENV",
        "INT32 *"                   => ~"*INT32",
        "INT64 *"                   => ~"*INT64",
        "UINT *"                    => ~"*UINT",
        "USHORT *"                  => ~"*USHORT",
        "const COLORREF *"          => ~"*COLORREF",
        "const DWORD *"             => ~"*DWORD",
        "const FLOAT *"             => ~"*FLOAT",
        // "const GLushort *"          => ~"*GLushort",
        "const HANDLE *"            => ~"*HANDLE",
        "const HGPUNV *"            => ~"*HGPUNV",
        "const LAYERPLANEDESCRIPTOR *"  => ~"*LAYERPLANEDESCRIPTOR",
        "const LPVOID *"            => ~"*LPVOID",
        "const PIXELFORMATDESCRIPTOR *" => ~"*PIXELFORMATDESCRIPTOR",
        "const USHORT *"            => ~"*USHORT",
        // "const char *"              => ~"*c_char",
        // "const int *"               => ~"*c_int",
        "float *"                   => ~"*c_float",
        // "int *"                     => ~"*c_int",
        // "unsigned long *"           => ~"*c_ulong",
        // "void *"                    => ~"*c_void",

        // failure
        _ => fail!("Type conversion not implemented for `%s`", ty),
    }
}

fn get_tys(reg: &Registry) -> TreeSet<~str> {
    let mut tys = TreeSet::new();
    for cmds in reg.cmds.iter() {
        for def in cmds.defs.iter() {
            tys.insert(def.proto.ty.clone());
            for param in def.params.iter() {
                tys.insert(param.ty.clone());
            }
        }
    }
    tys
}
