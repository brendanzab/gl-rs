// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

#![experimental]

/// Converts a C style type definition to the Rust equivalent
pub fn to_rust_ty(ty: &str) -> &'static str {
    match ty {
        // gl.xml types
        "GLDEBUGPROC"               => "types::GLDEBUGPROC",
        "GLDEBUGPROCAMD"            => "types::GLDEBUGPROCAMD",
        "GLDEBUGPROCARB"            => "types::GLDEBUGPROCARB",
        "GLDEBUGPROCKHR"            => "types::GLDEBUGPROCKHR",
        "GLbitfield"                => "types::GLbitfield",
        "GLboolean"                 => "types::GLboolean",
        "GLbyte"                    => "types::GLbyte",
        "GLclampd"                  => "types::GLclampd",
        "GLclampf"                  => "types::GLclampf",
        "GLclampx"                  => "types::GLclampx",
        "GLdouble"                  => "types::GLdouble",
        "GLeglImageOES"             => "types::GLeglImageOES",
        "GLenum"                    => "types::GLenum",
        "GLfixed"                   => "types::GLfixed",
        "GLfloat"                   => "types::GLfloat",
        "GLhalfNV"                  => "types::GLhalfNV",
        "GLhandleARB"               => "types::GLhandleARB",
        "GLint"                     => "types::GLint",
        "GLint64EXT"                => "types::GLint64EXT",
        "GLintptr"                  => "types::GLintptr",
        "GLintptrARB"               => "types::GLintptrARB",
        "GLshort"                   => "types::GLshort",
        "GLsizei"                   => "types::GLsizei",
        "GLsizeiptr"                => "types::GLsizeiptr",
        "GLsizeiptrARB"             => "types::GLsizeiptrARB",
        "GLsync"                    => "types::GLsync",
        "GLubyte"                   => "types::GLubyte",
        "GLuint"                    => "types::GLuint",
        "GLuint64"                  => "types::GLuint64",
        "GLuint64EXT"               => "types::GLuint64EXT",
        "GLushort"                  => "types::GLushort",
        "GLvdpauSurfaceNV"          => "types::GLvdpauSurfaceNV",
        "void "                     => "__gl_imports::libc::c_void",
        "GLboolean *"               => "*mut types::GLboolean",
        "GLchar *"                  => "*mut types::GLchar",
        "GLcharARB *"               => "*mut types::GLcharARB",
        "GLdouble *"                => "*mut types::GLdouble",
        "GLenum *"                  => "*mut types::GLenum",
        "GLfixed *"                 => "*mut types::GLfixed",
        "GLfloat *"                 => "*mut types::GLfloat",
        "GLhandleARB *"             => "*mut types::GLhandleARB",
        "GLint *"                   => "*mut types::GLint",
        "GLint64 *"                 => "*mut types::GLint64",
        "GLint64EXT *"              => "*mut types::GLint64EXT",
        "GLsizei *"                 => "*mut types::GLsizei",
        "GLubyte *"                 => "*mut types::GLubyte",
        "GLuint *"                  => "*mut types::GLuint",
        "GLuint [2]"                => "*mut [types::GLuint, ..2]",
        "GLuint64 *"                => "*mut types::GLuint64",
        "GLuint64EXT *"             => "*mut types::GLuint64EXT",
        "GLushort *"                => "*mut types::GLushort",
        "GLvoid *"                  => "*mut types::GLvoid",
        "GLvoid **"                 => "*const *mut types::GLvoid",
        "void *"                    => "*mut __gl_imports::libc::c_void",
        "void **"                   => "*const *mut __gl_imports::libc::c_void",
        "const GLboolean *"         => "*const types::GLboolean",
        "const GLbyte *"            => "*const types::GLbyte",
        "const GLchar *"            => "*const types::GLchar",
        "const GLcharARB *"         => "*const types::GLcharARB",
        "const GLclampf *"          => "*const types::GLclampf",
        "const GLdouble *"          => "*const types::GLdouble",
        "const GLenum *"            => "*const types::GLenum",
        "const GLfixed *"           => "*const types::GLfixed",
        "const GLfloat"             => "types::GLfloat",
        "const GLfloat *"           => "*const types::GLfloat",
        "const GLhalfNV *"          => "*const types::GLhalfNV",
        "const GLint *"             => "*const types::GLint",
        "const GLint64EXT *"        => "*const types::GLint64EXT",
        "const GLintptr *"          => "*const types::GLintptr",
        "const GLshort *"           => "*const types::GLshort",
        "const GLsizei *"           => "*const types::GLsizei",
        "const GLsizeiptr *"        => "*const types::GLsizeiptr",
        "const GLubyte *"           => "*const types::GLubyte",
        "const GLuint *"            => "*const types::GLuint",
        "const GLuint64 *"          => "*const types::GLuint64",
        "const GLuint64EXT *"       => "*const types::GLuint64EXT",
        "const GLushort *"          => "*const types::GLushort",
        "const GLvdpauSurfaceNV *"  => "*const types::GLvdpauSurfaceNV",
        "const GLvoid *"            => "*const types::GLvoid",
        "const void *"              => "*const __gl_imports::libc::c_void",
        "const void **"             => "*const *const __gl_imports::libc::c_void",
        "const void *const*"        => "*const *const __gl_imports::libc::c_void",
        "const GLboolean **"        => "*const *const types::GLboolean",
        "const GLchar **"           => "*const *const types::GLchar",
        "const GLcharARB **"        => "*const *const types::GLcharARB",
        "const GLvoid **"           => "*const *const types::GLvoid",
        "const GLchar *const*"      => "*const *const types::GLchar",
        "const GLvoid *const*"      => "*const *const types::GLvoid",
        "struct _cl_context *"      => "*const types::_cl_context",
        "struct _cl_event *"        => "*const types::_cl_event",

        // glx.xml types
        "Bool"                      => "types::Bool",
        "Colormap"                  => "types::Colormap",
        "DMbuffer"                  => "types::DMbuffer",
        "Font"                      => "types::Font",
        "GLXContext"                => "types::GLXContext",
        "GLXContextID"              => "types::GLXContextID",
        "GLXDrawable"               => "types::GLXDrawable",
        "GLXFBConfig"               => "types::GLXFBConfig",
        "GLXFBConfigSGIX"           => "types::GLXFBConfigSGIX",
        "GLXPbuffer"                => "types::GLXPbuffer",
        "GLXPbufferSGIX"            => "types::GLXPbufferSGIX",
        "GLXPixmap"                 => "types::GLXPixmap",
        "GLXVideoCaptureDeviceNV"   => "types::GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV"          => "types::GLXVideoDeviceNV",
        "GLXVideoSourceSGIX"        => "types::GLXVideoSourceSGIX",
        "GLXWindow"                 => "types::GLXWindow",
        // "GLboolean"                 => "types::GLboolean",
        // "GLenum"                    => "types::GLenum",
        // "GLint"                     => "types::GLint",
        // "GLsizei"                   => "types::GLsizei",
        // "GLuint"                    => "types::GLuint",
        "Pixmap"                    => "types::Pixmap",
        //"Status"                    => "types::Status",
        //"VLNode"                    => "types::VLNode",
        //"VLPath"                    => "types::VLPath",
        //"VLServer"                  => "types::VLServer",
        "Window"                    => "types::Window",
        "__GLXextFuncPtr"           => "types::__GLXextFuncPtr",
        "const GLXContext"          => "const types::GLXContext",
        "float "                    => "__gl_imports::libc::c_float",
        "int "                      => "__gl_imports::libc::c_int",
        "int64_t"                   => "i64",
        "unsigned int "             => "__gl_imports::libc::c_uint",
        "unsigned long "            => "__gl_imports::libc::c_ulong",
        // "void "                     => "__gl_imports::libc::c_void",
        "DMparams *"                => "*mut types::DMparams",
        "Display *"                 => "*mut types::Display",
        "GLXFBConfig *"             => "*mut types::GLXFBConfig",
        "GLXFBConfigSGIX *"         => "*mut types::GLXFBConfigSGIX",
        "GLXHyperpipeConfigSGIX *"  => "*mut types::GLXHyperpipeConfigSGIX",
        "GLXHyperpipeNetworkSGIX *" => "*mut types::GLXHyperpipeNetworkSGIX",
        "GLXVideoCaptureDeviceNV *" => "*mut types::GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV *"        => "*mut types::GLXVideoDeviceNV",
        // "GLuint *"                  => "*mut types::GLuint",
        "XVisualInfo *"             => "*mut types::XVisualInfo",
        // "const GLubyte *"           => "*GLubyte",
        "const char *"              => "*const __gl_imports::libc::c_char",
        "const int *"               => "*const __gl_imports::libc::c_int",
        // "const void *"              => "*const __gl_imports::libc::c_void",
        "int *"                     => "*mut __gl_imports::libc::c_int",
        "int32_t *"                 => "*mut i32",
        "int64_t *"                 => "*mut i64",
        "long *"                    => "*mut __gl_imports::libc::c_long",
        "unsigned int *"            => "*mut __gl_imports::libc::c_uint",
        "unsigned long *"           => "*mut __gl_imports::libc::c_ulong",
        // "void *"                    => "*mut __gl_imports::libc::c_void",

        // wgl.xml types
        "BOOL"                      => "types::BOOL",
        "DWORD"                     => "types::DWORD",
        "FLOAT"                     => "types::FLOAT",
        // "GLbitfield"                => "types::GLbitfield",
        // "GLboolean"                 => "types::GLboolean",
        // "GLenum"                    => "types::GLenum",
        // "GLfloat"                   => "types::GLfloat",
        // "GLint"                     => "types::GLint",
        // "GLsizei"                   => "types::GLsizei",
        // "GLuint"                    => "types::GLuint",
        // "GLushort"                  => "types::GLushort",
        "HANDLE"                    => "types::HANDLE",
        "HDC"                       => "types::HDC",
        "HENHMETAFILE"              => "types::HENHMETAFILE",
        "HGLRC"                     => "types::HGLRC",
        "HGPUNV"                    => "types::HGPUNV",
        "HPBUFFERARB"               => "types::HPBUFFERARB",
        "HPBUFFEREXT"               => "types::HPBUFFEREXT",
        "HPVIDEODEV"                => "types::HPVIDEODEV",
        "HVIDEOINPUTDEVICENV"       => "types::HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV"      => "types::HVIDEOOUTPUTDEVICENV",
        "INT"                       => "types::INT",
        "INT64"                     => "types::INT64",
        "LPCSTR"                    => "types::LPCSTR",
        "LPGLYPHMETRICSFLOAT"       => "types::LPGLYPHMETRICSFLOAT",
        "LPVOID"                    => "types::LPVOID",
        "PGPU_DEVICE"               => "types::PGPU_DEVICE",
        "PROC"                      => "types::PROC",
        "UINT"                      => "types::UINT",
        "VOID"                      => "types::VOID",
        // "int "                      => "__gl_imports::libc::c_int",
        // "unsigned int "             => "__gl_imports::libc::c_uint",
        // "void "                     => "__gl_imports::libc::c_void",
        "BOOL *"                    => "*mut types::BOOL",
        "DWORD *"                   => "*mut types::DWORD",
        "FLOAT *"                   => "*mut types::FLOAT",
        // "GLuint *"                  => "*mut types::GLuint",
        "HANDLE *"                  => "*mut types::HANDLE",
        "HGPUNV *"                  => "*mut types::HGPUNV",
        "HPVIDEODEV *"              => "*mut types::HPVIDEODEV",
        "HVIDEOINPUTDEVICENV *"     => "*mut types::HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV *"    => "*mut types::HVIDEOOUTPUTDEVICENV",
        "INT32 *"                   => "*mut types::INT32",
        "INT64 *"                   => "*mut types::INT64",
        "UINT *"                    => "*mut types::UINT",
        "USHORT *"                  => "*mut types::USHORT",
        "const COLORREF *"          => "*const types::COLORREF",
        "const DWORD *"             => "*const types::DWORD",
        "const FLOAT *"             => "*const types::FLOAT",
        // "const GLushort *"          => "*const types::GLushort",
        "const HANDLE *"            => "*const types::HANDLE",
        "const HGPUNV *"            => "*const types::HGPUNV",
        "const LAYERPLANEDESCRIPTOR *"  => "*const types::LAYERPLANEDESCRIPTOR",
        "const LPVOID *"            => "*const types::LPVOID",
        "const PIXELFORMATDESCRIPTOR *" => "*const types::IXELFORMATDESCRIPTOR",
        "const USHORT *"            => "*const types::USHORT",
        // "const char *"              => "*const __gl_imports::libc::c_char",
        // "const int *"               => "*const __gl_imports::libc::c_int",
        "float *"                   => "*mut __gl_imports::libc::c_float",
        // "int *"                     => "*mut __gl_imports::libc::c_int",
        // "unsigned long *"           => "*mut __gl_imports::libc::c_ulong",
        // "void *"                    => "*mut __gl_imports::libc::c_void",

        // elx.xml types
        "khronos_utime_nanoseconds_t"   => "types::khronos_utime_nanoseconds_t",
        "khronos_uint64_t"          => "types::khronos_uint64_t",
        "khronos_ssize_t"           => "types::khronos_ssize_t",
        "EGLNativeDisplayType"      => "types::EGLNativeDisplayType",
        "EGLNativePixmapType"       => "types::EGLNativePixmapType",
        "EGLNativeWindowType"       => "types::EGLNativeWindowType",
        "EGLint"                    => "types::EGLint",
        "EGLint *"                  => "*mut types::EGLint",
        "const EGLint *"            => "*const types::EGLint",
        "NativeDisplayType"         => "types::NativeDisplayType",
        "NativePixmapType"          => "types::NativePixmapType",
        "NativeWindowType"          => "types::NativeWindowType",
        //"Bool"                      => "types::Bool",
        "EGLBoolean"                => "types::EGLBoolean",
        "EGLenum"                   => "types::EGLenum",
        "EGLAttribKHR"              => "types::EGLAttribKHR",
        "EGLAttrib"                 => "types::EGLAttrib",
        "EGLAttrib *"               => "*mut types::EGLAttrib",
        "const EGLAttrib *"         => "*const types::EGLAttrib",
        "EGLConfig"                 => "types::EGLConfig",
        "EGLConfig *"               => "*mut types::EGLConfig",
        "EGLContext"                => "types::EGLContext",
        "EGLDeviceEXT"              => "types::EGLDeviceEXT",
        "EGLDisplay"                => "types::EGLDisplay",
        "EGLSurface"                => "types::EGLSurface",
        "EGLClientBuffer"               => "types::EGLClientBuffer",
        "__eglMustCastToProperFunctionPointerType"  => "types::__eglMustCastToProperFunctionPointerType",
        "EGLImageKHR"               => "types::EGLImageKHR",
        "EGLImage"                  => "types::EGLImage",
        "EGLSyncKHR"                => "types::EGLSyncKHR",
        "EGLSync"                   => "types::EGLSync",
        "EGLTimeKHR"                => "types::EGLTimeKHR",
        "EGLTime"                   => "types::EGLTime",
        "EGLSyncNV"                 => "types::EGLSyncNV",
        "EGLTimeNV"                 => "types::EGLTimeNV",
        "EGLuint64NV"               => "types::EGLuint64NV",
        "EGLStreamKHR"              => "types::EGLStreamKHR",
        "EGLuint64KHR"              => "types::EGLuint64KHR",
        "EGLNativeFileDescriptorKHR"    => "types::EGLNativeFileDescriptorKHR",
        "EGLsizeiANDROID"           => "types::EGLsizeiANDROID",
        "EGLSetBlobFuncANDROID"     => "types::EGLSetBlobFuncANDROID",
        "EGLGetBlobFuncANDROID"     => "types::EGLGetBlobFuncANDROID",
        "EGLClientPixmapHI"         => "types::EGLClientPixmapHI",

        // failure
        _ => panic!("Type conversion not implemented for `{}`", ty),
    }
}

pub fn build_gl_aliases() -> String {
    [
        // Common types from OpenGL 1.1
        "pub type GLenum = super::__gl_imports::libc::c_uint;",
        "pub type GLboolean = super::__gl_imports::libc::c_uchar;",
        "pub type GLbitfield = super::__gl_imports::libc::c_uint;",
        "pub type GLvoid = super::__gl_imports::libc::c_void;",
        "pub type GLbyte = super::__gl_imports::libc::c_char;",
        "pub type GLshort = super::__gl_imports::libc::c_short;",
        "pub type GLint = super::__gl_imports::libc::c_int;",
        "pub type GLclampx = super::__gl_imports::libc::c_int;",
        "pub type GLubyte = super::__gl_imports::libc::c_uchar;",
        "pub type GLushort = super::__gl_imports::libc::c_ushort;",
        "pub type GLuint = super::__gl_imports::libc::c_uint;",
        "pub type GLsizei = super::__gl_imports::libc::c_int;",
        "pub type GLfloat = super::__gl_imports::libc::c_float;",
        "pub type GLclampf = super::__gl_imports::libc::c_float;",
        "pub type GLdouble = super::__gl_imports::libc::c_double;",
        "pub type GLclampd = super::__gl_imports::libc::c_double;",
        "pub type GLeglImageOES = *const super::__gl_imports::libc::c_void;",
        "pub type GLchar = super::__gl_imports::libc::c_char;",
        "pub type GLcharARB = super::__gl_imports::libc::c_char;",

        "#[cfg(target_os = \"macos\")] pub type GLhandleARB = *const super::__gl_imports::libc::c_void;",
        "#[cfg(not(target_os = \"macos\"))] pub type GLhandleARB = super::__gl_imports::libc::c_uint;",

        "pub type GLhalfARB = super::__gl_imports::libc::c_ushort;",
        "pub type GLhalf = super::__gl_imports::libc::c_ushort;",

        // Must be 32 bits
        "pub type GLfixed = GLint;",

        "pub type GLintptr = super::__gl_imports::libc::ptrdiff_t;",
        "pub type GLsizeiptr = super::__gl_imports::libc::ptrdiff_t;",
        "pub type GLint64 = i64;",
        "pub type GLuint64 = u64;",
        "pub type GLintptrARB = super::__gl_imports::libc::ptrdiff_t;",
        "pub type GLsizeiptrARB = super::__gl_imports::libc::ptrdiff_t;",
        "pub type GLint64EXT = i64;",
        "pub type GLuint64EXT = u64;",

        "#[repr(C)] pub struct __GLsync;",
        "pub type GLsync = *const __GLsync;",

        // compatible with OpenCL cl_context
        "#[repr(C)] pub struct _cl_context;",
        "#[repr(C)] pub struct _cl_event;",

        "pub type GLDEBUGPROC = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);",
        "pub type GLDEBUGPROCARB = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);",
        "pub type GLDEBUGPROCKHR = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);",

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
        "pub type GLDEBUGPROCAMD = extern \"system\" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);",
        "pub type GLhalfNV = super::__gl_imports::libc::c_ushort;",
        "pub type GLvdpauSurfaceNV = GLintptr;",
    ].connect("\n")
}

pub fn build_x_aliases() -> String {
    [
        "pub type XID = super::__gl_imports::libc::c_ulong;",
        "pub type Bool = super::__gl_imports::libc::c_int;",       // Not sure if this is correct...
        "#[repr(C)] pub struct Display;",
    ].connect("\n")
}

pub fn build_glx_aliases() -> String {
    [
        "pub type Font = XID;",
        "pub type Pixmap = XID;",
        "pub type Visual = ();",   // TODO: not sure
        "pub type VisualID = super::__gl_imports::libc::c_ulong;",   // TODO: not sure
        "pub type Window = XID;",
        "pub type GLXFBConfigID = XID;",
        "pub type GLXFBConfig = *const super::__gl_imports::libc::c_void;",
        "pub type GLXContextID = XID;",
        "pub type GLXContext = *const super::__gl_imports::libc::c_void;",
        "pub type GLXPixmap = XID;",
        "pub type GLXDrawable = XID;",
        "pub type GLXWindow = XID;",
        "pub type GLXPbuffer = XID;",
        "pub type __GLXextFuncPtr = extern \"system\" fn();",
        "pub type GLXVideoCaptureDeviceNV = XID;",
        "pub type GLXVideoDeviceNV = super::__gl_imports::libc::c_int;",
        "pub type GLXVideoSourceSGIX = XID;",
        "pub type GLXFBConfigIDSGIX = XID;",
        "pub type GLXFBConfigSGIX = *const super::__gl_imports::libc::c_void;",
        "pub type GLXPbufferSGIX = XID;",

        "
            #[repr(C)]
            pub struct XVisualInfo {
                pub visual: *mut Visual,
                pub visualid: VisualID,
                pub screen: super::__gl_imports::libc::c_int,
                pub depth: super::__gl_imports::libc::c_int,
                pub class: super::__gl_imports::libc::c_int,
                pub red_mask: super::__gl_imports::libc::c_ulong,
                pub green_mask: super::__gl_imports::libc::c_ulong,
                pub blue_mask: super::__gl_imports::libc::c_ulong,
                pub colormap_size: super::__gl_imports::libc::c_int,
                pub bits_per_rgb: super::__gl_imports::libc::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPbufferClobberEvent {
                pub event_type: super::__gl_imports::libc::c_int,          // GLX_DAMAGED or GLX_SAVED
                pub draw_type: super::__gl_imports::libc::c_int,           // GLX_WINDOW or GLX_PBUFFER
                pub serial: super::__gl_imports::libc::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came for SendEvent request
                pub display: *const Display,          // display the event was read from
                pub drawable: GLXDrawable,      // XID of Drawable
                pub buffer_mask: super::__gl_imports::libc::c_uint,        // mask indicating which buffers are affected
                pub aux_buffer: super::__gl_imports::libc::c_uint,         // which aux buffer was affected
                pub x: super::__gl_imports::libc::c_int,
                pub y: super::__gl_imports::libc::c_int,
                pub width: super::__gl_imports::libc::c_int,
                pub height: super::__gl_imports::libc::c_int,
                pub count: super::__gl_imports::libc::c_int,               // if nonzero, at least this many more
            }
        ",

        "
            #[repr(C)]
            pub struct GLXBufferSwapComplete {
                pub type_: super::__gl_imports::libc::c_int,
                pub serial: super::__gl_imports::libc::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came from a SendEvent request
                pub display: *const Display,          // Display the event was read from
                pub drawable: GLXDrawable,      // drawable on which event was requested in event mask
                pub event_type: super::__gl_imports::libc::c_int,
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
                pub type_: super::__gl_imports::libc::c_int,
                pub serial: super::__gl_imports::libc::c_ulong,            // # of last request processed by server
                pub send_event: Bool,           // true if this came for SendEvent request
                pub display: *const Display,          // display the event was read from
                pub drawable: GLXDrawable,      // i.d. of Drawable
                pub event_type: super::__gl_imports::libc::c_int,          // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX
                pub draw_type: super::__gl_imports::libc::c_int,           // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX
                pub mask: super::__gl_imports::libc::c_uint,               // mask indicating which buffers are affected
                pub x: super::__gl_imports::libc::c_int,
                pub y: super::__gl_imports::libc::c_int,
                pub width: super::__gl_imports::libc::c_int,
                pub height: super::__gl_imports::libc::c_int,
                pub count: super::__gl_imports::libc::c_int,               // if nonzero, at least this many more
            }
        ",

        "
            #[repr(C)]
            pub struct GLXHyperpipeNetworkSGIX {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub networkId: super::__gl_imports::libc::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXHyperpipeConfigSGIX {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub channel: super::__gl_imports::libc::c_int,
                pub participationType: super::__gl_imports::libc::c_uint,
                pub timeSlice: super::__gl_imports::libc::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPipeRect {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub srcXOrigin: super::__gl_imports::libc::c_int,
                pub srcYOrigin: super::__gl_imports::libc::c_int,
                pub srcWidth: super::__gl_imports::libc::c_int,
                pub srcHeight: super::__gl_imports::libc::c_int,
                pub destXOrigin: super::__gl_imports::libc::c_int,
                pub destYOrigin: super::__gl_imports::libc::c_int,
                pub destWidth: super::__gl_imports::libc::c_int,
                pub destHeight: super::__gl_imports::libc::c_int,
            }
        ",

        "
            #[repr(C)]
            pub struct GLXPipeRectLimits {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub XOrigin: super::__gl_imports::libc::c_int,
                pub YOrigin: super::__gl_imports::libc::c_int,
                pub maxHeight: super::__gl_imports::libc::c_int,
                pub maxWidth: super::__gl_imports::libc::c_int,
            }
        "
    ].connect("\n")
}

pub fn build_win_aliases() -> String {
    [
        // From WinNT.h
        "pub type CHAR = super::__gl_imports::libc::c_char;",
        "pub type HANDLE = PVOID;",
        "pub type LONG = super::__gl_imports::libc::c_long;",
        "pub type LPCSTR = *const super::__gl_imports::libc::c_char;",
        "pub type VOID = super::__gl_imports::libc::c_void;",

        // From Windef.h
        "pub type BOOL = super::__gl_imports::libc::c_int;",
        "pub type BYTE = super::__gl_imports::libc::c_uchar;",
        "pub type COLORREF = DWORD;",
        "pub type FLOAT = super::__gl_imports::libc::c_float;",
        "pub type HDC = HANDLE;",
        "pub type HENHMETAFILE = HANDLE;",
        "pub type HGLRC = *const super::__gl_imports::libc::c_void;",
        "pub type INT = super::__gl_imports::libc::c_int;",
        "pub type PVOID = *const super::__gl_imports::libc::c_void;",
        "pub type LPVOID = *const super::__gl_imports::libc::c_void;",
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
        "pub type UINT = super::__gl_imports::libc::c_uint;",
        "pub type USHORT = super::__gl_imports::libc::c_ushort;",
        "pub type WORD = super::__gl_imports::libc::c_ushort;",

        // From BaseTsd.h
        "pub type INT32 = i32;",
        "pub type INT64 = i64;",

        // From IntSafe.h
        "pub type DWORD = super::__gl_imports::libc::c_ulong;",

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
    ].connect("\n")
}

pub fn build_wgl_aliases() -> String {
    [
        // From WinNT.h,
        // #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name
        "pub type HPBUFFERARB = *const super::__gl_imports::libc::c_void;",
        "pub type HPBUFFEREXT = *const super::__gl_imports::libc::c_void;",
        "pub type HVIDEOOUTPUTDEVICENV = *const super::__gl_imports::libc::c_void;",
        "pub type HPVIDEODEV = *const super::__gl_imports::libc::c_void;",
        "pub type HPGPUNV = *const super::__gl_imports::libc::c_void;",
        "pub type HGPUNV = *const super::__gl_imports::libc::c_void;",
        "pub type HVIDEOINPUTDEVICENV = *const super::__gl_imports::libc::c_void;",

        "
            #[repr(C)]
            pub struct _GPU_DEVICE {
                cb: DWORD,
                DeviceName: [CHAR, ..32],
                DeviceString: [CHAR, ..128],
                Flags: DWORD,
                rcVirtualScreen: RECT,
            }
        ",

        "pub struct GPU_DEVICE(_GPU_DEVICE);",
        "pub struct PGPU_DEVICE(*const _GPU_DEVICE);",
    ].connect("\n")
}

pub fn build_egl_aliases() -> String {
    [
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
        "pub type EGLBoolean = super::__gl_imports::libc::c_uint;",
        "pub type EGLenum = super::__gl_imports::libc::c_uint;",
        "pub type EGLAttribKHR = super::__gl_imports::libc::intptr_t;",
        "pub type EGLAttrib = super::__gl_imports::libc::intptr_t;",
        "pub type EGLConfig = *const super::__gl_imports::libc::c_void;",
        "pub type EGLContext = *const super::__gl_imports::libc::c_void;",
        "pub type EGLDeviceEXT = *const super::__gl_imports::libc::c_void;",
        "pub type EGLDisplay = *const super::__gl_imports::libc::c_void;",
        "pub type EGLSurface = *const super::__gl_imports::libc::c_void;",
        "pub type EGLClientBuffer = *const super::__gl_imports::libc::c_void;",
        "pub type __eglMustCastToProperFunctionPointerType = extern \"system\" fn() -> ();",
        "pub type EGLImageKHR = *const super::__gl_imports::libc::c_void;",
        "pub type EGLImage = *const super::__gl_imports::libc::c_void;",
        "pub type EGLSyncKHR = *const super::__gl_imports::libc::c_void;",
        "pub type EGLSync = *const super::__gl_imports::libc::c_void;",
        "pub type EGLTimeKHR = khronos_utime_nanoseconds_t;",
        "pub type EGLTime = khronos_utime_nanoseconds_t;",
        "pub type EGLSyncNV = *const super::__gl_imports::libc::c_void;",
        "pub type EGLTimeNV = khronos_utime_nanoseconds_t;",
        "pub type EGLuint64NV = khronos_utime_nanoseconds_t;",
        "pub type EGLStreamKHR = *const super::__gl_imports::libc::c_void;",
        "pub type EGLuint64KHR = khronos_uint64_t;",
        "pub type EGLNativeFileDescriptorKHR = super::__gl_imports::libc::c_int;",
        "pub type EGLsizeiANDROID = khronos_ssize_t;",
        "pub type EGLSetBlobFuncANDROID = extern \"system\" fn(*const super::__gl_imports::libc::c_void, EGLsizeiANDROID, *const super::__gl_imports::libc::c_void, EGLsizeiANDROID) -> ();",
        "pub type EGLGetBlobFuncANDROID = extern \"system\" fn(*const super::__gl_imports::libc::c_void, EGLsizeiANDROID, *mut super::__gl_imports::libc::c_void, EGLsizeiANDROID) -> EGLsizeiANDROID;",

        "
            #[repr(C)]
            pub struct EGLClientPixmapHI {
                pData: *const super::__gl_imports::libc::c_void,
                iWidth: EGLint,
                iHeight: EGLint,
                iStride: EGLint,
            }
        "
    ].connect("\n")
}
