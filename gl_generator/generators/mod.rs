// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use Api;
use registry::{Enum, Registry, Cmd};
use std::io;

pub mod debug_struct_gen;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;
pub mod static_struct_gen;

/// Trait for a bindings generator.
pub trait Generator {
    /// Builds the GL bindings.
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()> where W: io::Write;
}

pub fn gen_struct_name(api: Api) -> &'static str {
    match api {
        Api::Gl  => "Gl",
        Api::Glx => "Glx",
        Api::Wgl => "Wgl",
        Api::Egl => "Egl",
        Api::GlCore => "GlCore",
        Api::Gles1 => "Gles1",
        Api::Gles2 => "Gles2",
    }
}

/// This function generates a `const name: type = value;` item.
pub fn gen_enum_item<W>(enm: &Enum, types_prefix: &str, dest: &mut W) -> io::Result<()> where W: io::Write {
    // computing the name of the enum
    // if the original starts with a digit, adding an underscore prefix.
    let ident = if (enm.ident.chars().next().unwrap()).is_numeric() {
        format!("_{}", enm.ident)
    } else {
        enm.ident.clone()
    };

    // if the enum has the value of the form `((Type)Value)`, then `val_regexed` contains `(Type, Value)`
    let val_regexed = {
        if enm.value.starts_with("((") && enm.value.ends_with(")") {
            let separator = enm.value.chars().skip(2).position(|c| c == ')').unwrap();
            Some((&enm.value[2 .. separator + 2], enm.value[separator + 3 ..].trim_matches(')')))
        } else {
            None
        }
    };

    // computing the type of the enum
    let ty = {
        // some enums have a value of the form `((Type)Value)` ; if this is the case, we need to
        //  replace the type of the enum (which is GLenum by default) by the type in the expression
        if let Some((ty, _)) = val_regexed {
            // if the value is like `((Type)Value)`, then the type is `types::Type`
            format!("{}{}", types_prefix, ty)

        } else if enm.value.starts_with("\"") {
            // some values are of the form "Value" ; if this is the case, we use `&'static str`
            //  instead of `GLenum`
            "&'static str".to_string()

        } else {
            // some values are `TRUE` or `FALSE`, in which case we use `GLboolean` instead of
            //  `GLenum`
            match &ident[..] {
                "TRUE" | "FALSE" => format!("{}GLboolean", types_prefix),
                _ => match enm.ty {
                    Some(ref s) if &s[..] == "ull" => format!("{}GLuint64", types_prefix),
                    _ => format!("{}GLenum", types_prefix)
                }
            }
        }
    };

    // computing the value of the enum
    let value = {
        // similar to the type, some values are `((Type)Value)`
        // replacing "((Type)Value)" by "Value as types::Type"
        if let Some((ty, val)) = val_regexed {
            format!("{} as {}{}", val, types_prefix, ty)
        } else {
            enm.value.clone()
        }
    };

    writeln!(dest, "\
        #[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const {}: {} = {}; \
    ", ident, ty, value)
}

/// Generates all the type aliases for a namespace.
///
/// Aliases are either `pub type = ...` or `#[repr(C)] pub struct ... { ... }` and contain all the
///  things that we can't obtain from the XML files.
pub fn gen_type_aliases<W>(api: Api, dest: &mut W) -> io::Result<()> where W: io::Write {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => {
            try!(build_gl_aliases(dest));
        }

        Api::Glx => {
            try!(build_gl_aliases(dest));
            try!(build_x_aliases(dest));
            try!(build_glx_aliases(dest));
        }

        Api::Wgl => {
            try!(build_gl_aliases(dest));
            try!(build_win_aliases(dest));
            try!(build_wgl_aliases(dest));
        }

        Api::Egl => {
            try!(build_gl_aliases(dest));
            try!(build_egl_aliases(dest));
        }
    }

    Ok(())
}

/// Generates the list of Rust `Arg`s that a `Cmd` requires.
pub fn gen_parameters(cmd: &Cmd, with_idents: bool, with_types: bool) -> Vec<String> {
    cmd.params.iter()
        .map(|binding| {
            // variable name of the binding
            let ident = match &binding.ident[..] {
                "in" => "in_",
                "ref" => "ref_",
                "type" => "type_",
                ident => ident,
            };

            // returning
            if with_idents && with_types {
                format!("{}: {}", ident, binding.ty)
            } else if with_types {
                format!("{}", binding.ty)
            } else if with_idents {
                format!("{}", ident)
            } else {
                panic!()
            }
        })
        .collect()
}

/// Generates the native symbol name of a `Cmd`.
///
/// Example results: `"glClear"`, `"wglCreateContext"`, etc.
pub fn gen_symbol_name(api: Api, cmd: &str) -> String {
    match api {
        Api::Gl | Api::GlCore | Api::Gles1 | Api::Gles2 => format!("gl{}", cmd),
        Api::Glx => format!("glX{}", cmd),
        Api::Wgl => format!("wgl{}", cmd),
        Api::Egl => format!("egl{}", cmd),
    }
}

pub fn build_gl_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        // Common types from OpenGL 1.1
        "pub type GLenum = super::__gl_imports::raw::c_uint;",
        "pub type GLboolean = super::__gl_imports::raw::c_uchar;",
        "pub type GLbitfield = super::__gl_imports::raw::c_uint;",
        "pub type GLvoid = super::__gl_imports::raw::c_void;",
        "pub type GLbyte = super::__gl_imports::raw::c_char;",
        "pub type GLshort = super::__gl_imports::raw::c_short;",
        "pub type GLint = super::__gl_imports::raw::c_int;",
        "pub type GLclampx = super::__gl_imports::raw::c_int;",
        "pub type GLubyte = super::__gl_imports::raw::c_uchar;",
        "pub type GLushort = super::__gl_imports::raw::c_ushort;",
        "pub type GLuint = super::__gl_imports::raw::c_uint;",
        "pub type GLsizei = super::__gl_imports::raw::c_int;",
        "pub type GLfloat = super::__gl_imports::raw::c_float;",
        "pub type GLclampf = super::__gl_imports::raw::c_float;",
        "pub type GLdouble = super::__gl_imports::raw::c_double;",
        "pub type GLclampd = super::__gl_imports::raw::c_double;",
        "pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;",
        "pub type GLchar = super::__gl_imports::raw::c_char;",
        "pub type GLcharARB = super::__gl_imports::raw::c_char;",

        "#[cfg(target_os = \"macos\")] pub type GLhandleARB = *const super::__gl_imports::raw::c_void;",
        "#[cfg(not(target_os = \"macos\"))] pub type GLhandleARB = super::__gl_imports::raw::c_uint;",

        "pub type GLhalfARB = super::__gl_imports::raw::c_ushort;",
        "pub type GLhalf = super::__gl_imports::raw::c_ushort;",

        // Must be 32 bits
        "pub type GLfixed = GLint;",

        "pub type GLintptr = isize;",
        "pub type GLsizeiptr = isize;",
        "pub type GLint64 = i64;",
        "pub type GLuint64 = u64;",
        "pub type GLintptrARB = isize;",
        "pub type GLsizeiptrARB = isize;",
        "pub type GLint64EXT = i64;",
        "pub type GLuint64EXT = u64;",

        "pub enum __GLsync {}",
        "pub type GLsync = *const __GLsync;",

        // compatible with OpenCL cl_context
        "pub enum _cl_context {}",
        "pub enum _cl_event {}",

        "pub type GLDEBUGPROC = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::raw::c_void);",
        "pub type GLDEBUGPROCARB = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::raw::c_void);",
        "pub type GLDEBUGPROCKHR = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::raw::c_void);",

        // GLES 1 types
        // "pub type GLclampx = i32;",

        // GLES 1/2 types (tagged for GLES 1)
        // "pub type GLbyte = i8;",
        // "pub type GLubyte = u8;",
        // "pub type GLfloat = GLfloat;",
        // "pub type GLclampf = GLfloat;",
        // "pub type GLfixed = i32;",
        // "pub type GLint64 = i64;",
        // "pub type GLuint64 = u64;",
        // "pub type GLintptr = intptr_t;",
        // "pub type GLsizeiptr = ssize_t;",

        // GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
        // "pub type GLbyte = i8;",
        // "pub type GLubyte = u8;",
        // "pub type GLfloat = GLfloat;",
        // "pub type GLclampf = GLfloat;",
        // "pub type GLfixed = i32;",
        // "pub type GLint64 = i64;",
        // "pub type GLuint64 = u64;",
        // "pub type GLint64EXT = i64;",
        // "pub type GLuint64EXT = u64;",
        // "pub type GLintptr = intptr_t;",
        // "pub type GLsizeiptr = ssize_t;",

        // GLES 2 types (none currently)

        // Vendor extension types
        "pub type GLDEBUGPROCAMD = extern \"system\" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::raw::c_void);",
        "pub type GLhalfNV = super::__gl_imports::raw::c_ushort;",
        "pub type GLvdpauSurfaceNV = GLintptr;",
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}

pub fn build_x_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        "pub type XID = super::__gl_imports::raw::c_ulong;",
        "pub type Bool = super::__gl_imports::raw::c_int;",       // Not sure if this is correct...
        "pub enum Display {}",
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}

pub fn build_glx_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        "pub type Font = XID;",
        "pub type Pixmap = XID;",
        "pub enum Visual {}",   // TODO: not sure
        "pub type VisualID = super::__gl_imports::raw::c_ulong;",   // TODO: not sure
        "pub type Window = XID;",
        "pub type GLXFBConfigID = XID;",
        "pub type GLXFBConfig = *const super::__gl_imports::raw::c_void;",
        "pub type GLXContextID = XID;",
        "pub type GLXContext = *const super::__gl_imports::raw::c_void;",
        "pub type GLXPixmap = XID;",
        "pub type GLXDrawable = XID;",
        "pub type GLXWindow = XID;",
        "pub type GLXPbuffer = XID;",
        "pub type __GLXextFuncPtr = extern \"system\" fn();",
        "pub type GLXVideoCaptureDeviceNV = XID;",
        "pub type GLXVideoDeviceNV = super::__gl_imports::raw::c_int;",
        "pub type GLXVideoSourceSGIX = XID;",
        "pub type GLXFBConfigIDSGIX = XID;",
        "pub type GLXFBConfigSGIX = *const super::__gl_imports::raw::c_void;",
        "pub type GLXPbufferSGIX = XID;",

        "
            #[repr(C)]
            pub struct XVisualInfo {
                pub visual: *mut Visual,
                pub visualid: VisualID,
                pub screen: super::__gl_imports::raw::c_int,
                pub depth: super::__gl_imports::raw::c_int,
                pub class: super::__gl_imports::raw::c_int,
                pub red_mask: super::__gl_imports::raw::c_ulong,
                pub green_mask: super::__gl_imports::raw::c_ulong,
                pub blue_mask: super::__gl_imports::raw::c_ulong,
                pub colormap_size: super::__gl_imports::raw::c_int,
                pub bits_per_rgb: super::__gl_imports::raw::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPbufferClobberEvent {
                pub event_type: super::__gl_imports::raw::c_int,          // GLX_DAMAGED or GLX_SAVED
                pub draw_type: super::__gl_imports::raw::c_int,           // GLX_WINDOW or GLX_PBUFFER
                pub serial: super::__gl_imports::raw::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came for SendEvent request
                pub display: *const Display,          // display the event was read from
                pub drawable: GLXDrawable,      // XID of Drawable
                pub buffer_mask: super::__gl_imports::raw::c_uint,        // mask indicating which buffers are affected
                pub aux_buffer: super::__gl_imports::raw::c_uint,         // which aux buffer was affected
                pub x: super::__gl_imports::raw::c_int,
                pub y: super::__gl_imports::raw::c_int,
                pub width: super::__gl_imports::raw::c_int,
                pub height: super::__gl_imports::raw::c_int,
                pub count: super::__gl_imports::raw::c_int,               // if nonzero, at least this many more
            }
        ",

        "
            #[repr(C)]
            pub struct GLXBufferSwapComplete {
                pub type_: super::__gl_imports::raw::c_int,
                pub serial: super::__gl_imports::raw::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came from a SendEvent request
                pub display: *const Display,          // Display the event was read from
                pub drawable: GLXDrawable,      // drawable on which event was requested in event mask
                pub event_type: super::__gl_imports::raw::c_int,
                pub ust: i64,
                pub msc: i64,
                pub sbc: i64,
            }
        ",

        //"
        //    typedef union __GLXEvent {
        //        GLXPbufferClobberEvent glxpbufferclobber;
        //        GLXBufferSwapComplete glxbufferswapcomplete;
        //        long pad[24];
        //    }
        //",

        "
            #[repr(C)]
            pub struct GLXBufferClobberEventSGIX {
                pub type_: super::__gl_imports::raw::c_int,
                pub serial: super::__gl_imports::raw::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came for SendEvent request
                pub display: *const Display,          // display the event was read from
                pub drawable: GLXDrawable,      // i.d. of Drawable
                pub event_type: super::__gl_imports::raw::c_int,          // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX
                pub draw_type: super::__gl_imports::raw::c_int,           // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX
                pub mask: super::__gl_imports::raw::c_uint,               // mask indicating which buffers are affected
                pub x: super::__gl_imports::raw::c_int,
                pub y: super::__gl_imports::raw::c_int,
                pub width: super::__gl_imports::raw::c_int,
                pub height: super::__gl_imports::raw::c_int,
                pub count: super::__gl_imports::raw::c_int,               // if nonzero, at least this many more
            }
        ",

        "
            #[repr(C)]
            pub struct GLXHyperpipeNetworkSGIX {
                pub pipeName: [super::__gl_imports::raw::c_char; 80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub networkId: super::__gl_imports::raw::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXHyperpipeConfigSGIX {
                pub pipeName: [super::__gl_imports::raw::c_char; 80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub channel: super::__gl_imports::raw::c_int,
                pub participationType: super::__gl_imports::raw::c_uint,
                pub timeSlice: super::__gl_imports::raw::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPipeRect {
                pub pipeName: [super::__gl_imports::raw::c_char; 80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub srcXOrigin: super::__gl_imports::raw::c_int,
                pub srcYOrigin: super::__gl_imports::raw::c_int,
                pub srcWidth: super::__gl_imports::raw::c_int,
                pub srcHeight: super::__gl_imports::raw::c_int,
                pub destXOrigin: super::__gl_imports::raw::c_int,
                pub destYOrigin: super::__gl_imports::raw::c_int,
                pub destWidth: super::__gl_imports::raw::c_int,
                pub destHeight: super::__gl_imports::raw::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPipeRectLimits {
                pub pipeName: [super::__gl_imports::raw::c_char; 80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub XOrigin: super::__gl_imports::raw::c_int,
                pub YOrigin: super::__gl_imports::raw::c_int,
                pub maxHeight: super::__gl_imports::raw::c_int,
                pub maxWidth: super::__gl_imports::raw::c_int,
            }
        "
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}

pub fn build_win_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        // From WinNT.h
        "pub type CHAR = super::__gl_imports::raw::c_char;",
        "pub type HANDLE = PVOID;",
        "pub type LONG = super::__gl_imports::raw::c_long;",
        "pub type LPCSTR = *const super::__gl_imports::raw::c_char;",
        "pub type VOID = ();",

        // From Windef.h
        "pub type BOOL = super::__gl_imports::raw::c_int;",
        "pub type BYTE = super::__gl_imports::raw::c_uchar;",
        "pub type COLORREF = DWORD;",
        "pub type FLOAT = super::__gl_imports::raw::c_float;",
        "pub type HDC = HANDLE;",
        "pub type HENHMETAFILE = HANDLE;",
        "pub type HGLRC = *const super::__gl_imports::raw::c_void;",
        "pub type INT = super::__gl_imports::raw::c_int;",
        "pub type PVOID = *const super::__gl_imports::raw::c_void;",
        "pub type LPVOID = *const super::__gl_imports::raw::c_void;",
        "pub type PROC = extern \"system\" fn();",     // Not sure about this one :/
        "
            #[repr(C)]
            pub struct RECT {
                left: LONG,
                top: LONG,
                right: LONG,
                bottom: LONG,
            }
        ",
        "pub type UINT = super::__gl_imports::raw::c_uint;",
        "pub type USHORT = super::__gl_imports::raw::c_ushort;",
        "pub type WORD = super::__gl_imports::raw::c_ushort;",

        // From BaseTsd.h
        "pub type INT32 = i32;",
        "pub type INT64 = i64;",

        // From IntSafe.h
        "pub type DWORD = super::__gl_imports::raw::c_ulong;",

        // From Wingdi.h
        "
            #[repr(C)]
            pub struct POINTFLOAT {
                pub x: FLOAT,
                pub y: FLOAT,
            }
        ",
        "
            #[repr(C)]
            pub struct GLYPHMETRICSFLOAT {
                pub gmfBlackBoxX: FLOAT,
                pub gmfBlackBoxY: FLOAT,
                pub gmfptGlyphOrigin: POINTFLOAT,
                pub gmfCellIncX: FLOAT,
                pub gmfCellIncY: FLOAT,
            }
        ",
        "pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;",
        "
            #[repr(C)]
            pub struct LAYERPLANEDESCRIPTOR {
                pub nSize: WORD,
                pub nVersion: WORD,
                pub dwFlags: DWORD,
                pub iPixelType: BYTE,
                pub cColorBits: BYTE,
                pub cRedBits: BYTE,
                pub cRedShift: BYTE,
                pub cGreenBits: BYTE,
                pub cGreenShift: BYTE,
                pub cBlueBits: BYTE,
                pub cBlueShift: BYTE,
                pub cAlphaBits: BYTE,
                pub cAlphaShift: BYTE,
                pub cAccumBits: BYTE,
                pub cAccumRedBits: BYTE,
                pub cAccumGreenBits: BYTE,
                pub cAccumBlueBits: BYTE,
                pub cAccumAlphaBits: BYTE,
                pub cDepthBits: BYTE,
                pub cStencilBits: BYTE,
                pub cAuxBuffers: BYTE,
                pub iLayerType: BYTE,
                pub bReserved: BYTE,
                pub crTransparent: COLORREF,
            }
        ",
        "
            #[repr(C)]
            pub struct PIXELFORMATDESCRIPTOR {
                pub nSize: WORD,
                pub nVersion: WORD,
                pub dwFlags: DWORD,
                pub iPixelType: BYTE,
                pub cColorBits: BYTE,
                pub cRedBits: BYTE,
                pub cRedShift: BYTE,
                pub cGreenBits: BYTE,
                pub cGreenShift: BYTE,
                pub cBlueBits: BYTE,
                pub cBlueShift: BYTE,
                pub cAlphaBits: BYTE,
                pub cAlphaShift: BYTE,
                pub cAccumBits: BYTE,
                pub cAccumRedBits: BYTE,
                pub cAccumGreenBits: BYTE,
                pub cAccumBlueBits: BYTE,
                pub cAccumAlphaBits: BYTE,
                pub cDepthBits: BYTE,
                pub cStencilBits: BYTE,
                pub cAuxBuffers: BYTE,
                pub iLayerType: BYTE,
                pub bReserved: BYTE,
                pub dwLayerMask: DWORD,
                pub dwVisibleMask: DWORD,
                pub dwDamageMask: DWORD,
            }
        "
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}

pub fn build_wgl_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        // From WinNT.h,
        // #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name
        "pub type HPBUFFERARB = *const super::__gl_imports::raw::c_void;",
        "pub type HPBUFFEREXT = *const super::__gl_imports::raw::c_void;",
        "pub type HVIDEOOUTPUTDEVICENV = *const super::__gl_imports::raw::c_void;",
        "pub type HPVIDEODEV = *const super::__gl_imports::raw::c_void;",
        "pub type HPGPUNV = *const super::__gl_imports::raw::c_void;",
        "pub type HGPUNV = *const super::__gl_imports::raw::c_void;",
        "pub type HVIDEOINPUTDEVICENV = *const super::__gl_imports::raw::c_void;",

        "
            #[repr(C)]
            pub struct _GPU_DEVICE {
                cb: DWORD,
                DeviceName: [CHAR; 32],
                DeviceString: [CHAR; 128],
                Flags: DWORD,
                rcVirtualScreen: RECT,
            }
        ",

        "pub struct GPU_DEVICE(_GPU_DEVICE);",
        "pub struct PGPU_DEVICE(*const _GPU_DEVICE);",
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}

pub fn build_egl_aliases<W>(dest: &mut W) -> io::Result<()> where W: io::Write {
    for l in [
        // platform-specific aliases are unknown
        // IMPORTANT: these are alises to the same level of the bindings
        // the values must be defined by the user
        "pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;",
        "pub type khronos_uint64_t = super::khronos_uint64_t;",
        "pub type khronos_ssize_t = super::khronos_ssize_t;",
        "pub type EGLNativeDisplayType = super::EGLNativeDisplayType;",
        "pub type EGLNativePixmapType = super::EGLNativePixmapType;",
        "pub type EGLNativeWindowType = super::EGLNativeWindowType;",
        "pub type EGLint = super::EGLint;",
        "pub type NativeDisplayType = super::NativeDisplayType;",
        "pub type NativePixmapType = super::NativePixmapType;",
        "pub type NativeWindowType = super::NativeWindowType;",

        // EGL alises
        "pub type Bool = EGLBoolean;",  // TODO: not sure
        "pub type EGLBoolean = super::__gl_imports::raw::c_uint;",
        "pub type EGLenum = super::__gl_imports::raw::c_uint;",
        "pub type EGLAttribKHR = isize;",
        "pub type EGLAttrib = isize;",
        "pub type EGLConfig = *const super::__gl_imports::raw::c_void;",
        "pub type EGLContext = *const super::__gl_imports::raw::c_void;",
        "pub type EGLDeviceEXT = *const super::__gl_imports::raw::c_void;",
        "pub type EGLDisplay = *const super::__gl_imports::raw::c_void;",
        "pub type EGLSurface = *const super::__gl_imports::raw::c_void;",
        "pub type EGLClientBuffer = *const super::__gl_imports::raw::c_void;",
        "pub type __eglMustCastToProperFunctionPointerType = extern \"system\" fn() -> ();",
        "pub type EGLImageKHR = *const super::__gl_imports::raw::c_void;",
        "pub type EGLImage = *const super::__gl_imports::raw::c_void;",
        "pub type EGLSyncKHR = *const super::__gl_imports::raw::c_void;",
        "pub type EGLSync = *const super::__gl_imports::raw::c_void;",
        "pub type EGLTimeKHR = khronos_utime_nanoseconds_t;",
        "pub type EGLTime = khronos_utime_nanoseconds_t;",
        "pub type EGLSyncNV = *const super::__gl_imports::raw::c_void;",
        "pub type EGLTimeNV = khronos_utime_nanoseconds_t;",
        "pub type EGLuint64NV = khronos_utime_nanoseconds_t;",
        "pub type EGLStreamKHR = *const super::__gl_imports::raw::c_void;",
        "pub type EGLuint64KHR = khronos_uint64_t;",
        "pub type EGLNativeFileDescriptorKHR = super::__gl_imports::raw::c_int;",
        "pub type EGLsizeiANDROID = khronos_ssize_t;",
        "pub type EGLSetBlobFuncANDROID = extern \"system\" fn(*const super::__gl_imports::raw::c_void, EGLsizeiANDROID, *const super::__gl_imports::raw::c_void, EGLsizeiANDROID) -> ();",
        "pub type EGLGetBlobFuncANDROID = extern \"system\" fn(*const super::__gl_imports::raw::c_void, EGLsizeiANDROID, *mut super::__gl_imports::raw::c_void, EGLsizeiANDROID) -> EGLsizeiANDROID;",

        "
            #[repr(C)]
            pub struct EGLClientPixmapHI {
                pData: *const super::__gl_imports::raw::c_void,
                iWidth: EGLint,
                iHeight: EGLint,
                iStride: EGLint,
            }
        "
    ].iter() {
        try!(writeln!(dest, "{}", l));
    }

    Ok(())
}
