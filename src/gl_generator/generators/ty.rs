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

use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

/// Converts a C style type definition to the Rust equivalent
pub fn to_rust_ty(ecx: &ExtCtxt, ty: &str) -> P<ast::Ty> {
    match ty {
        // gl.xml types
        "GLDEBUGPROC"               => quote_ty!(ecx, types::GLDEBUGPROC),
        "GLDEBUGPROCAMD"            => quote_ty!(ecx, types::GLDEBUGPROCAMD),
        "GLDEBUGPROCARB"            => quote_ty!(ecx, types::GLDEBUGPROCARB),
        "GLDEBUGPROCKHR"            => quote_ty!(ecx, types::GLDEBUGPROCKHR),
        "GLbitfield"                => quote_ty!(ecx, types::GLbitfield),
        "GLboolean"                 => quote_ty!(ecx, types::GLboolean),
        "GLbyte"                    => quote_ty!(ecx, types::GLbyte),
        "GLclampd"                  => quote_ty!(ecx, types::GLclampd),
        "GLclampf"                  => quote_ty!(ecx, types::GLclampf),
        "GLclampx"                  => quote_ty!(ecx, types::GLclampx),
        "GLdouble"                  => quote_ty!(ecx, types::GLdouble),
        "GLeglImageOES"             => quote_ty!(ecx, types::GLeglImageOES),
        "GLenum"                    => quote_ty!(ecx, types::GLenum),
        "GLfixed"                   => quote_ty!(ecx, types::GLfixed),
        "GLfloat"                   => quote_ty!(ecx, types::GLfloat),
        "GLhalfNV"                  => quote_ty!(ecx, types::GLhalfNV),
        "GLhandleARB"               => quote_ty!(ecx, types::GLhandleARB),
        "GLint"                     => quote_ty!(ecx, types::GLint),
        "GLint64EXT"                => quote_ty!(ecx, types::GLint64EXT),
        "GLintptr"                  => quote_ty!(ecx, types::GLintptr),
        "GLintptrARB"               => quote_ty!(ecx, types::GLintptrARB),
        "GLshort"                   => quote_ty!(ecx, types::GLshort),
        "GLsizei"                   => quote_ty!(ecx, types::GLsizei),
        "GLsizeiptr"                => quote_ty!(ecx, types::GLsizeiptr),
        "GLsizeiptrARB"             => quote_ty!(ecx, types::GLsizeiptrARB),
        "GLsync"                    => quote_ty!(ecx, types::GLsync),
        "GLubyte"                   => quote_ty!(ecx, types::GLubyte),
        "GLuint"                    => quote_ty!(ecx, types::GLuint),
        "GLuint64"                  => quote_ty!(ecx, types::GLuint64),
        "GLuint64EXT"               => quote_ty!(ecx, types::GLuint64EXT),
        "GLushort"                  => quote_ty!(ecx, types::GLushort),
        "GLvdpauSurfaceNV"          => quote_ty!(ecx, types::GLvdpauSurfaceNV),
        "void "                     => quote_ty!(ecx, __gl_imports::libc::c_void),
        "GLboolean *"               => quote_ty!(ecx, *mut types::GLboolean),
        "GLchar *"                  => quote_ty!(ecx, *mut types::GLchar),
        "GLcharARB *"               => quote_ty!(ecx, *mut types::GLcharARB),
        "GLdouble *"                => quote_ty!(ecx, *mut types::GLdouble),
        "GLenum *"                  => quote_ty!(ecx, *mut types::GLenum),
        "GLfixed *"                 => quote_ty!(ecx, *mut types::GLfixed),
        "GLfloat *"                 => quote_ty!(ecx, *mut types::GLfloat),
        "GLhandleARB *"             => quote_ty!(ecx, *mut types::GLhandleARB),
        "GLint *"                   => quote_ty!(ecx, *mut types::GLint),
        "GLint64 *"                 => quote_ty!(ecx, *mut types::GLint64),
        "GLint64EXT *"              => quote_ty!(ecx, *mut types::GLint64EXT),
        "GLsizei *"                 => quote_ty!(ecx, *mut types::GLsizei),
        "GLubyte *"                 => quote_ty!(ecx, *mut types::GLubyte),
        "GLuint *"                  => quote_ty!(ecx, *mut types::GLuint),
        "GLuint [2]"                => quote_ty!(ecx, *mut [types::GLuint, ..2]),
        "GLuint64 *"                => quote_ty!(ecx, *mut types::GLuint64),
        "GLuint64EXT *"             => quote_ty!(ecx, *mut types::GLuint64EXT),
        "GLushort *"                => quote_ty!(ecx, *mut types::GLushort),
        "GLvoid *"                  => quote_ty!(ecx, *mut types::GLvoid),
        "GLvoid **"                 => quote_ty!(ecx, *const *mut types::GLvoid),
        "void *"                    => quote_ty!(ecx, *mut __gl_imports::libc::c_void),
        "void **"                   => quote_ty!(ecx, *const *mut __gl_imports::libc::c_void),
        "const GLboolean *"         => quote_ty!(ecx, *const types::GLboolean),
        "const GLbyte *"            => quote_ty!(ecx, *const types::GLbyte),
        "const GLchar *"            => quote_ty!(ecx, *const types::GLchar),
        "const GLcharARB *"         => quote_ty!(ecx, *const types::GLcharARB),
        "const GLclampf *"          => quote_ty!(ecx, *const types::GLclampf),
        "const GLdouble *"          => quote_ty!(ecx, *const types::GLdouble),
        "const GLenum *"            => quote_ty!(ecx, *const types::GLenum),
        "const GLfixed *"           => quote_ty!(ecx, *const types::GLfixed),
        "const GLfloat"             => quote_ty!(ecx, types::GLfloat),
        "const GLfloat *"           => quote_ty!(ecx, *const types::GLfloat),
        "const GLhalfNV *"          => quote_ty!(ecx, *const types::GLhalfNV),
        "const GLint *"             => quote_ty!(ecx, *const types::GLint),
        "const GLint64EXT *"        => quote_ty!(ecx, *const types::GLint64EXT),
        "const GLintptr *"          => quote_ty!(ecx, *const types::GLintptr),
        "const GLshort *"           => quote_ty!(ecx, *const types::GLshort),
        "const GLsizei *"           => quote_ty!(ecx, *const types::GLsizei),
        "const GLsizeiptr *"        => quote_ty!(ecx, *const types::GLsizeiptr),
        "const GLubyte *"           => quote_ty!(ecx, *const types::GLubyte),
        "const GLuint *"            => quote_ty!(ecx, *const types::GLuint),
        "const GLuint64 *"          => quote_ty!(ecx, *const types::GLuint64),
        "const GLuint64EXT *"       => quote_ty!(ecx, *const types::GLuint64EXT),
        "const GLushort *"          => quote_ty!(ecx, *const types::GLushort),
        "const GLvdpauSurfaceNV *"  => quote_ty!(ecx, *const types::GLvdpauSurfaceNV),
        "const GLvoid *"            => quote_ty!(ecx, *const types::GLvoid),
        "const void *"              => quote_ty!(ecx, *const __gl_imports::libc::c_void),
        "const void **"             => quote_ty!(ecx, *const *const __gl_imports::libc::c_void),
        "const void *const*"        => quote_ty!(ecx, *const *const __gl_imports::libc::c_void),
        "const GLboolean **"        => quote_ty!(ecx, *const *const types::GLboolean),
        "const GLchar **"           => quote_ty!(ecx, *const *const types::GLchar),
        "const GLcharARB **"        => quote_ty!(ecx, *const *const types::GLcharARB),
        "const GLvoid **"           => quote_ty!(ecx, *const *const types::GLvoid),
        "const GLchar *const*"      => quote_ty!(ecx, *const *const types::GLchar),
        "const GLvoid *const*"      => quote_ty!(ecx, *const *const types::GLvoid),
        "struct _cl_context *"      => quote_ty!(ecx, *const types::_cl_context),
        "struct _cl_event *"        => quote_ty!(ecx, *const types::_cl_event),

        // glx.xml types
        "Bool"                      => quote_ty!(ecx, types::Bool),
        "Colormap"                  => quote_ty!(ecx, types::Colormap),
        "DMbuffer"                  => quote_ty!(ecx, types::DMbuffer),
        "Font"                      => quote_ty!(ecx, types::Font),
        "GLXContext"                => quote_ty!(ecx, types::GLXContext),
        "GLXContextID"              => quote_ty!(ecx, types::GLXContextID),
        "GLXDrawable"               => quote_ty!(ecx, types::GLXDrawable),
        "GLXFBConfig"               => quote_ty!(ecx, types::GLXFBConfig),
        "GLXFBConfigSGIX"           => quote_ty!(ecx, types::GLXFBConfigSGIX),
        "GLXPbuffer"                => quote_ty!(ecx, types::GLXPbuffer),
        "GLXPbufferSGIX"            => quote_ty!(ecx, types::GLXPbufferSGIX),
        "GLXPixmap"                 => quote_ty!(ecx, types::GLXPixmap),
        "GLXVideoCaptureDeviceNV"   => quote_ty!(ecx, types::GLXVideoCaptureDeviceNV),
        "GLXVideoDeviceNV"          => quote_ty!(ecx, types::GLXVideoDeviceNV),
        "GLXVideoSourceSGIX"        => quote_ty!(ecx, types::GLXVideoSourceSGIX),
        "GLXWindow"                 => quote_ty!(ecx, types::GLXWindow),
        // "GLboolean"                 => quote_ty!(ecx, types::GLboolean),
        // "GLenum"                    => quote_ty!(ecx, types::GLenum),
        // "GLint"                     => quote_ty!(ecx, types::GLint),
        // "GLsizei"                   => quote_ty!(ecx, types::GLsizei),
        // "GLuint"                    => quote_ty!(ecx, types::GLuint),
        "Pixmap"                    => quote_ty!(ecx, types::Pixmap),
        //"Status"                    => quote_ty!(ecx, types::Status),
        //"VLNode"                    => quote_ty!(ecx, types::VLNode),
        //"VLPath"                    => quote_ty!(ecx, types::VLPath),
        //"VLServer"                  => quote_ty!(ecx, types::VLServer),
        "Window"                    => quote_ty!(ecx, types::Window),
        "__GLXextFuncPtr"           => quote_ty!(ecx, types::__GLXextFuncPtr),
        "const GLXContext"          => quote_ty!(ecx, const types::GLXContext),
        "float "                    => quote_ty!(ecx, __gl_imports::libc::c_float),
        "int "                      => quote_ty!(ecx, __gl_imports::libc::c_int),
        "int64_t"                   => quote_ty!(ecx, i64),
        "unsigned int "             => quote_ty!(ecx, __gl_imports::libc::c_uint),
        "unsigned long "            => quote_ty!(ecx, __gl_imports::libc::c_ulong),
        // "void "                     => quote_ty!(ecx, __gl_imports::libc::c_void),
        "DMparams *"                => quote_ty!(ecx, *mut types::DMparams),
        "Display *"                 => quote_ty!(ecx, *mut types::Display),
        "GLXFBConfig *"             => quote_ty!(ecx, *mut types::GLXFBConfig),
        "GLXFBConfigSGIX *"         => quote_ty!(ecx, *mut types::GLXFBConfigSGIX),
        "GLXHyperpipeConfigSGIX *"  => quote_ty!(ecx, *mut types::GLXHyperpipeConfigSGIX),
        "GLXHyperpipeNetworkSGIX *" => quote_ty!(ecx, *mut types::GLXHyperpipeNetworkSGIX),
        "GLXVideoCaptureDeviceNV *" => quote_ty!(ecx, *mut types::GLXVideoCaptureDeviceNV),
        "GLXVideoDeviceNV *"        => quote_ty!(ecx, *mut types::GLXVideoDeviceNV),
        // "GLuint *"                  => quote_ty!(ecx, *mut types::GLuint),
        "XVisualInfo *"             => quote_ty!(ecx, *mut types::XVisualInfo),
        // "const GLubyte *"           => quote_ty!(ecx, *GLubyte),
        "const char *"              => quote_ty!(ecx, *const __gl_imports::libc::c_char),
        "const int *"               => quote_ty!(ecx, *const __gl_imports::libc::c_int),
        // "const void *"              => quote_ty!(ecx, *const __gl_imports::libc::c_void),
        "int *"                     => quote_ty!(ecx, *mut __gl_imports::libc::c_int),
        "int32_t *"                 => quote_ty!(ecx, *mut i32),
        "int64_t *"                 => quote_ty!(ecx, *mut i64),
        "long *"                    => quote_ty!(ecx, *mut __gl_imports::libc::c_long),
        "unsigned int *"            => quote_ty!(ecx, *mut __gl_imports::libc::c_uint),
        "unsigned long *"           => quote_ty!(ecx, *mut __gl_imports::libc::c_ulong),
        // "void *"                    => quote_ty!(ecx, *mut __gl_imports::libc::c_void),

        // wgl.xml types
        "BOOL"                      => quote_ty!(ecx, types::BOOL),
        "DWORD"                     => quote_ty!(ecx, types::DWORD),
        "FLOAT"                     => quote_ty!(ecx, types::FLOAT),
        // "GLbitfield"                => quote_ty!(ecx, types::GLbitfield),
        // "GLboolean"                 => quote_ty!(ecx, types::GLboolean),
        // "GLenum"                    => quote_ty!(ecx, types::GLenum),
        // "GLfloat"                   => quote_ty!(ecx, types::GLfloat),
        // "GLint"                     => quote_ty!(ecx, types::GLint),
        // "GLsizei"                   => quote_ty!(ecx, types::GLsizei),
        // "GLuint"                    => quote_ty!(ecx, types::GLuint),
        // "GLushort"                  => quote_ty!(ecx, types::GLushort),
        "HANDLE"                    => quote_ty!(ecx, types::HANDLE),
        "HDC"                       => quote_ty!(ecx, types::HDC),
        "HENHMETAFILE"              => quote_ty!(ecx, types::HENHMETAFILE),
        "HGLRC"                     => quote_ty!(ecx, types::HGLRC),
        "HGPUNV"                    => quote_ty!(ecx, types::HGPUNV),
        "HPBUFFERARB"               => quote_ty!(ecx, types::HPBUFFERARB),
        "HPBUFFEREXT"               => quote_ty!(ecx, types::HPBUFFEREXT),
        "HPVIDEODEV"                => quote_ty!(ecx, types::HPVIDEODEV),
        "HVIDEOINPUTDEVICENV"       => quote_ty!(ecx, types::HVIDEOINPUTDEVICENV),
        "HVIDEOOUTPUTDEVICENV"      => quote_ty!(ecx, types::HVIDEOOUTPUTDEVICENV),
        "INT"                       => quote_ty!(ecx, types::INT),
        "INT64"                     => quote_ty!(ecx, types::INT64),
        "LPCSTR"                    => quote_ty!(ecx, types::LPCSTR),
        "LPGLYPHMETRICSFLOAT"       => quote_ty!(ecx, types::LPGLYPHMETRICSFLOAT),
        "LPVOID"                    => quote_ty!(ecx, types::LPVOID),
        "PGPU_DEVICE"               => quote_ty!(ecx, types::PGPU_DEVICE),
        "PROC"                      => quote_ty!(ecx, types::PROC),
        "UINT"                      => quote_ty!(ecx, types::UINT),
        "VOID"                      => quote_ty!(ecx, types::VOID),
        // "int "                      => quote_ty!(ecx, __gl_imports::libc::c_int),
        // "unsigned int "             => quote_ty!(ecx, __gl_imports::libc::c_uint),
        // "void "                     => quote_ty!(ecx, __gl_imports::libc::c_void),
        "BOOL *"                    => quote_ty!(ecx, *mut types::BOOL),
        "DWORD *"                   => quote_ty!(ecx, *mut types::DWORD),
        "FLOAT *"                   => quote_ty!(ecx, *mut types::FLOAT),
        // "GLuint *"                  => quote_ty!(ecx, *mut types::GLuint),
        "HANDLE *"                  => quote_ty!(ecx, *mut types::HANDLE),
        "HGPUNV *"                  => quote_ty!(ecx, *mut types::HGPUNV),
        "HPVIDEODEV *"              => quote_ty!(ecx, *mut types::HPVIDEODEV),
        "HVIDEOINPUTDEVICENV *"     => quote_ty!(ecx, *mut types::HVIDEOINPUTDEVICENV),
        "HVIDEOOUTPUTDEVICENV *"    => quote_ty!(ecx, *mut types::HVIDEOOUTPUTDEVICENV),
        "INT32 *"                   => quote_ty!(ecx, *mut types::INT32),
        "INT64 *"                   => quote_ty!(ecx, *mut types::INT64),
        "UINT *"                    => quote_ty!(ecx, *mut types::UINT),
        "USHORT *"                  => quote_ty!(ecx, *mut types::USHORT),
        "const COLORREF *"          => quote_ty!(ecx, *const types::COLORREF),
        "const DWORD *"             => quote_ty!(ecx, *const types::DWORD),
        "const FLOAT *"             => quote_ty!(ecx, *const types::FLOAT),
        // "const GLushort *"          => quote_ty!(ecx, *const types::GLushort),
        "const HANDLE *"            => quote_ty!(ecx, *const types::HANDLE),
        "const HGPUNV *"            => quote_ty!(ecx, *const types::HGPUNV),
        "const LAYERPLANEDESCRIPTOR *"  => quote_ty!(ecx, *const types::LAYERPLANEDESCRIPTOR),
        "const LPVOID *"            => quote_ty!(ecx, *const types::LPVOID),
        "const PIXELFORMATDESCRIPTOR *" => quote_ty!(ecx, *const types::IXELFORMATDESCRIPTOR),
        "const USHORT *"            => quote_ty!(ecx, *const types::USHORT),
        // "const char *"              => quote_ty!(ecx, *const __gl_imports::libc::c_char),
        // "const int *"               => quote_ty!(ecx, *const __gl_imports::libc::c_int),
        "float *"                   => quote_ty!(ecx, *mut __gl_imports::libc::c_float),
        // "int *"                     => quote_ty!(ecx, *mut __gl_imports::libc::c_int),
        // "unsigned long *"           => quote_ty!(ecx, *mut __gl_imports::libc::c_ulong),
        // "void *"                    => quote_ty!(ecx, *mut __gl_imports::libc::c_void),

        // elx.xml types
        "khronos_utime_nanoseconds_t"   => quote_ty!(ecx, types::khronos_utime_nanoseconds_t),
        "khronos_uint64_t"          => quote_ty!(ecx, types::khronos_uint64_t),
        "khronos_ssize_t"           => quote_ty!(ecx, types::khronos_ssize_t),
        "EGLNativeDisplayType"      => quote_ty!(ecx, types::EGLNativeDisplayType),
        "EGLNativePixmapType"       => quote_ty!(ecx, types::EGLNativePixmapType),
        "EGLNativeWindowType"       => quote_ty!(ecx, types::EGLNativeWindowType),
        "EGLint"                    => quote_ty!(ecx, types::EGLint),
        "EGLint *"                  => quote_ty!(ecx, *mut types::EGLint),
        "const EGLint *"            => quote_ty!(ecx, *const types::EGLint),
        "NativeDisplayType"         => quote_ty!(ecx, types::NativeDisplayType),
        "NativePixmapType"          => quote_ty!(ecx, types::NativePixmapType),
        "NativeWindowType"          => quote_ty!(ecx, types::NativeWindowType),
        //"Bool"                      => quote_ty!(ecx, types::Bool),
        "EGLBoolean"                => quote_ty!(ecx, types::EGLBoolean),
        "EGLenum"                   => quote_ty!(ecx, types::EGLenum),
        "EGLAttribKHR"              => quote_ty!(ecx, types::EGLAttribKHR),
        "EGLAttrib"                 => quote_ty!(ecx, types::EGLAttrib),
        "EGLAttrib *"               => quote_ty!(ecx, *mut types::EGLAttrib),
        "const EGLAttrib *"         => quote_ty!(ecx, *const types::EGLAttrib),
        "EGLConfig"                 => quote_ty!(ecx, types::EGLConfig),
        "EGLConfig *"               => quote_ty!(ecx, *mut types::EGLConfig),
        "EGLContext"                => quote_ty!(ecx, types::EGLContext),
        "EGLDeviceEXT"              => quote_ty!(ecx, types::EGLDeviceEXT),
        "EGLDisplay"                => quote_ty!(ecx, types::EGLDisplay),
        "EGLSurface"                => quote_ty!(ecx, types::EGLSurface),
        "EGLClientBuffer"               => quote_ty!(ecx, types::EGLClientBuffer),
        "__eglMustCastToProperFunctionPointerType"  => quote_ty!(ecx, types::__eglMustCastToProperFunctionPointerType),
        "EGLImageKHR"               => quote_ty!(ecx, types::EGLImageKHR),
        "EGLImage"                  => quote_ty!(ecx, types::EGLImage),
        "EGLSyncKHR"                => quote_ty!(ecx, types::EGLSyncKHR),
        "EGLSync"                   => quote_ty!(ecx, types::EGLSync),
        "EGLTimeKHR"                => quote_ty!(ecx, types::EGLTimeKHR),
        "EGLTime"                   => quote_ty!(ecx, types::EGLTime),
        "EGLSyncNV"                 => quote_ty!(ecx, types::EGLSyncNV),
        "EGLTimeNV"                 => quote_ty!(ecx, types::EGLTimeNV),
        "EGLuint64NV"               => quote_ty!(ecx, types::EGLuint64NV),
        "EGLStreamKHR"              => quote_ty!(ecx, types::EGLStreamKHR),
        "EGLuint64KHR"              => quote_ty!(ecx, types::EGLuint64KHR),
        "EGLNativeFileDescriptorKHR"    => quote_ty!(ecx, types::EGLNativeFileDescriptorKHR),
        "EGLsizeiANDROID"           => quote_ty!(ecx, types::EGLsizeiANDROID),
        "EGLSetBlobFuncANDROID"     => quote_ty!(ecx, types::EGLSetBlobFuncANDROID),
        "EGLGetBlobFuncANDROID"     => quote_ty!(ecx, types::EGLGetBlobFuncANDROID),
        "EGLClientPixmapHI"         => quote_ty!(ecx, types::EGLClientPixmapHI),

        // failure
        _ => fail!("Type conversion not implemented for `{}`", ty),
    }
}

pub fn build_gl_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        // Common types from OpenGL 1.1
        quote_item!(ecx, pub type GLenum = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type GLboolean = super::__gl_imports::libc::c_uchar;),
        quote_item!(ecx, pub type GLbitfield = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type GLvoid = super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type GLbyte = super::__gl_imports::libc::c_char;),
        quote_item!(ecx, pub type GLshort = super::__gl_imports::libc::c_short;),
        quote_item!(ecx, pub type GLint = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type GLclampx = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type GLubyte = super::__gl_imports::libc::c_uchar;),
        quote_item!(ecx, pub type GLushort = super::__gl_imports::libc::c_ushort;),
        quote_item!(ecx, pub type GLuint = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type GLsizei = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type GLfloat = super::__gl_imports::libc::c_float;),
        quote_item!(ecx, pub type GLclampf = super::__gl_imports::libc::c_float;),
        quote_item!(ecx, pub type GLdouble = super::__gl_imports::libc::c_double;),
        quote_item!(ecx, pub type GLclampd = super::__gl_imports::libc::c_double;),
        quote_item!(ecx, pub type GLeglImageOES = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type GLchar = super::__gl_imports::libc::c_char;),
        quote_item!(ecx, pub type GLcharARB = super::__gl_imports::libc::c_char;),

        quote_item!(ecx, #[cfg(target_os = "macos")] pub type GLhandleARB = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, #[cfg(not(target_os = "macos"))] pub type GLhandleARB = super::__gl_imports::libc::c_uint;),

        quote_item!(ecx, pub type GLhalfARB = super::__gl_imports::libc::c_ushort;),
        quote_item!(ecx, pub type GLhalf = super::__gl_imports::libc::c_ushort;),

        // Must be 32 bits
        quote_item!(ecx, pub type GLfixed = GLint;),

        quote_item!(ecx, pub type GLintptr = super::__gl_imports::libc::ptrdiff_t;),
        quote_item!(ecx, pub type GLsizeiptr = super::__gl_imports::libc::ptrdiff_t;),
        quote_item!(ecx, pub type GLint64 = i64;),
        quote_item!(ecx, pub type GLuint64 = u64;),
        quote_item!(ecx, pub type GLintptrARB = super::__gl_imports::libc::ptrdiff_t;),
        quote_item!(ecx, pub type GLsizeiptrARB = super::__gl_imports::libc::ptrdiff_t;),
        quote_item!(ecx, pub type GLint64EXT = i64;),
        quote_item!(ecx, pub type GLuint64EXT = u64;),

        quote_item!(ecx, #[repr(C)] pub struct __GLsync;),
        quote_item!(ecx, pub type GLsync = *const __GLsync;),

        // compatible with OpenCL cl_context
        quote_item!(ecx, #[repr(C)] pub struct _cl_context;),
        quote_item!(ecx, #[repr(C)] pub struct _cl_event;),

        quote_item!(ecx, pub type GLDEBUGPROC = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);),
        quote_item!(ecx, pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);),
        quote_item!(ecx, pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);),

        // GLES 1 types
        // quote_item!(ecx, pub type GLclampx = i32;),
        
        // GLES 1/2 types (tagged for GLES 1)
        // quote_item!(ecx, pub type GLbyte = i8;),
        // quote_item!(ecx, pub type GLubyte = u8;),
        // quote_item!(ecx, pub type GLfloat = GLfloat;),
        // quote_item!(ecx, pub type GLclampf = GLfloat;),
        // quote_item!(ecx, pub type GLfixed = i32;),
        // quote_item!(ecx, pub type GLint64 = i64;),
        // quote_item!(ecx, pub type GLuint64 = u64;),
        // quote_item!(ecx, pub type GLintptr = intptr_t;),
        // quote_item!(ecx, pub type GLsizeiptr = ssize_t;),
        
        // GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
        // quote_item!(ecx, pub type GLbyte = i8;),
        // quote_item!(ecx, pub type GLubyte = u8;),
        // quote_item!(ecx, pub type GLfloat = GLfloat;),
        // quote_item!(ecx, pub type GLclampf = GLfloat;),
        // quote_item!(ecx, pub type GLfixed = i32;),
        // quote_item!(ecx, pub type GLint64 = i64;),
        // quote_item!(ecx, pub type GLuint64 = u64;),
        // quote_item!(ecx, pub type GLint64EXT = i64;),
        // quote_item!(ecx, pub type GLuint64EXT = u64;),
        // quote_item!(ecx, pub type GLintptr = intptr_t;),
        // quote_item!(ecx, pub type GLsizeiptr = ssize_t;),

        // GLES 2 types (none currently)

        // Vendor extension types
        quote_item!(ecx, pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);),
        quote_item!(ecx, pub type GLhalfNV = super::__gl_imports::libc::c_ushort;),
        quote_item!(ecx, pub type GLvdpauSurfaceNV = GLintptr;)
    ]).into_iter().map(|i| i.unwrap()).collect()
}

pub fn build_x_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        quote_item!(ecx, pub type XID = super::__gl_imports::libc::c_ulong;),
        quote_item!(ecx, pub type Bool = super::__gl_imports::libc::c_int;),       // Not sure if this is correct...
        quote_item!(ecx, #[repr(C)] pub struct Display;)
    ]).into_iter().map(|i| i.unwrap()).collect()
}

pub fn build_glx_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        quote_item!(ecx, pub type Font = XID;),
        quote_item!(ecx, pub type Pixmap = XID;),
        quote_item!(ecx, pub type Visual = ();),   // TODO: not sure
        quote_item!(ecx, pub type VisualID = super::__gl_imports::libc::c_ulong;),   // TODO: not sure
        quote_item!(ecx, pub type Window = XID;),
        quote_item!(ecx, pub type GLXFBConfigID = XID;),
        quote_item!(ecx, pub type GLXFBConfig = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type GLXContextID = XID;),
        quote_item!(ecx, pub type GLXContext = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type GLXPixmap = XID;),
        quote_item!(ecx, pub type GLXDrawable = XID;),
        quote_item!(ecx, pub type GLXWindow = XID;),
        quote_item!(ecx, pub type GLXPbuffer = XID;),
        quote_item!(ecx, pub type __GLXextFuncPtr = extern "system" fn();),
        quote_item!(ecx, pub type GLXVideoCaptureDeviceNV = XID;),
        quote_item!(ecx, pub type GLXVideoDeviceNV = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type GLXVideoSourceSGIX = XID;),
        quote_item!(ecx, pub type GLXFBConfigIDSGIX = XID;),
        quote_item!(ecx, pub type GLXFBConfigSGIX = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type GLXPbufferSGIX = XID;),

        quote_item!(ecx,
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
        ),

        quote_item!(ecx,
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
        ),

        quote_item!(ecx,
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
        ),

        //quote_item!(ecx,
        //    typedef union __GLXEvent {
        //        GLXPbufferClobberEvent glxpbufferclobber;
        //        GLXBufferSwapComplete glxbufferswapcomplete;
        //        long pad[24];
        //    }
        //),

        quote_item!(ecx,
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
        ),

        quote_item!(ecx,
            #[repr(C)]
            pub struct GLXHyperpipeNetworkSGIX {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub networkId: super::__gl_imports::libc::c_int,
            }
        ),

        quote_item!(ecx,
            #[repr(C)]
            pub struct GLXHyperpipeConfigSGIX {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub channel: super::__gl_imports::libc::c_int,
                pub participationType: super::__gl_imports::libc::c_uint,
                pub timeSlice: super::__gl_imports::libc::c_int,
            }
        ),

        quote_item!(ecx,
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
        ),

        quote_item!(ecx,
            #[repr(C)]
            pub struct GLXPipeRectLimits {
                pub pipeName: [super::__gl_imports::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
                pub XOrigin: super::__gl_imports::libc::c_int,
                pub YOrigin: super::__gl_imports::libc::c_int,
                pub maxHeight: super::__gl_imports::libc::c_int,
                pub maxWidth: super::__gl_imports::libc::c_int,
            }
        )
    ]).into_iter().map(|i| i.unwrap()).collect()
}

pub fn build_win_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        // From WinNT.h
        quote_item!(ecx, pub type CHAR = super::__gl_imports::libc::c_char;),
        quote_item!(ecx, pub type HANDLE = PVOID;),
        quote_item!(ecx, pub type LONG = super::__gl_imports::libc::c_long;),
        quote_item!(ecx, pub type LPCSTR = *const super::__gl_imports::libc::c_char;),
        quote_item!(ecx, pub type VOID = super::__gl_imports::libc::c_void;),

        // From Windef.h
        quote_item!(ecx, pub type BOOL = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type BYTE = super::__gl_imports::libc::c_uchar;),
        quote_item!(ecx, pub type COLORREF = DWORD;),
        quote_item!(ecx, pub type FLOAT = super::__gl_imports::libc::c_float;),
        quote_item!(ecx, pub type HDC = HANDLE;),
        quote_item!(ecx, pub type HENHMETAFILE = HANDLE;),
        quote_item!(ecx, pub type HGLRC = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type INT = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type PVOID = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type LPVOID = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type PROC = extern "system" fn();),     // Not sure about this one :/
        quote_item!(ecx,
            #[repr(C)]
            pub struct RECT {
                left: LONG,
                top: LONG,
                right: LONG,
                bottom: LONG,
            }
        ),
        quote_item!(ecx, pub type UINT = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type USHORT = super::__gl_imports::libc::c_ushort;),
        quote_item!(ecx, pub type WORD = super::__gl_imports::libc::c_ushort;),

        // From BaseTsd.h
        quote_item!(ecx, pub type INT32 = i32;),
        quote_item!(ecx, pub type INT64 = i64;),

        // From IntSafe.h
        quote_item!(ecx, pub type DWORD = super::__gl_imports::libc::c_ulong;),

        // From Wingdi.h
        quote_item!(ecx,
            #[repr(C)]
            pub struct POINTFLOAT {
                pub x: FLOAT,
                pub y: FLOAT,
            }
        ),
        quote_item!(ecx,
            #[repr(C)]
            pub struct GLYPHMETRICSFLOAT {
                pub gmfBlackBoxX: FLOAT,
                pub gmfBlackBoxY: FLOAT,
                pub gmfptGlyphOrigin: POINTFLOAT,
                pub gmfCellIncX: FLOAT,
                pub gmfCellIncY: FLOAT,
            }
        ),
        quote_item!(ecx, pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;),
        quote_item!(ecx,
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
        ),
        quote_item!(ecx,
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
        )
    ]).into_iter().map(|i| i.unwrap()).collect()
}

pub fn build_wgl_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        // From WinNT.h,
        // #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name
        quote_item!(ecx, pub type HPBUFFERARB = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HPBUFFEREXT = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HVIDEOOUTPUTDEVICENV = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HPVIDEODEV = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HPGPUNV = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HGPUNV = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type HVIDEOINPUTDEVICENV = *const super::__gl_imports::libc::c_void;),

        quote_item!(ecx,
            #[repr(C)]
            pub struct _GPU_DEVICE {
                cb: DWORD,
                DeviceName: [CHAR, ..32],
                DeviceString: [CHAR, ..128],
                Flags: DWORD,
                rcVirtualScreen: RECT,
            }
        ),

        quote_item!(ecx, pub struct GPU_DEVICE(_GPU_DEVICE);),
        quote_item!(ecx, pub struct PGPU_DEVICE(*const _GPU_DEVICE);),
    ]).into_iter().map(|i| i.unwrap()).collect()
}

pub fn build_egl_aliases(ecx: &ExtCtxt) -> Vec<P<ast::Item>> {
    (vec![
        // platform-specific aliases are unknown
        // IMPORTANT: these are alises to the same level of the bindings
        // the values must be defined by the user
        quote_item!(ecx, pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;),
        quote_item!(ecx, pub type khronos_uint64_t = super::khronos_uint64_t;),
        quote_item!(ecx, pub type khronos_ssize_t = super::khronos_ssize_t;),
        quote_item!(ecx, pub type EGLNativeDisplayType = super::EGLNativeDisplayType;),
        quote_item!(ecx, pub type EGLNativePixmapType = super::EGLNativePixmapType;),
        quote_item!(ecx, pub type EGLNativeWindowType = super::EGLNativeWindowType;),
        quote_item!(ecx, pub type EGLint = super::EGLint;),
        quote_item!(ecx, pub type NativeDisplayType = super::NativeDisplayType;),
        quote_item!(ecx, pub type NativePixmapType = super::NativePixmapType;),
        quote_item!(ecx, pub type NativeWindowType = super::NativeWindowType;),

        // EGL alises
        quote_item!(ecx, pub type Bool = EGLBoolean;),  // TODO: not sure
        quote_item!(ecx, pub type EGLBoolean = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type EGLenum = super::__gl_imports::libc::c_uint;),
        quote_item!(ecx, pub type EGLAttribKHR = super::__gl_imports::libc::intptr_t;),
        quote_item!(ecx, pub type EGLAttrib = super::__gl_imports::libc::intptr_t;),
        quote_item!(ecx, pub type EGLConfig = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLContext = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLDeviceEXT = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLDisplay = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLSurface = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLClientBuffer = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type __eglMustCastToProperFunctionPointerType = extern "system" fn() -> ();),
        quote_item!(ecx, pub type EGLImageKHR = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLImage = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLSyncKHR = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLSync = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLTimeKHR = khronos_utime_nanoseconds_t;),
        quote_item!(ecx, pub type EGLTime = khronos_utime_nanoseconds_t;),
        quote_item!(ecx, pub type EGLSyncNV = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLTimeNV = khronos_utime_nanoseconds_t;),
        quote_item!(ecx, pub type EGLuint64NV = khronos_utime_nanoseconds_t;),
        quote_item!(ecx, pub type EGLStreamKHR = *const super::__gl_imports::libc::c_void;),
        quote_item!(ecx, pub type EGLuint64KHR = khronos_uint64_t;),
        quote_item!(ecx, pub type EGLNativeFileDescriptorKHR = super::__gl_imports::libc::c_int;),
        quote_item!(ecx, pub type EGLsizeiANDROID = khronos_ssize_t;),
        quote_item!(ecx, pub type EGLSetBlobFuncANDROID = extern "system" fn(*const super::__gl_imports::libc::c_void, EGLsizeiANDROID, *const super::__gl_imports::libc::c_void, EGLsizeiANDROID) -> ();),
        quote_item!(ecx, pub type EGLGetBlobFuncANDROID = extern "system" fn(*const super::__gl_imports::libc::c_void, EGLsizeiANDROID, *mut super::__gl_imports::libc::c_void, EGLsizeiANDROID) -> EGLsizeiANDROID;),

        quote_item!(ecx,
            #[repr(C)]
            pub struct EGLClientPixmapHI {
                pData: *const super::__gl_imports::libc::c_void,
                iWidth: EGLint,
                iHeight: EGLint,
                iStride: EGLint,
            }
        )
    ]).into_iter().map(|i| i.unwrap()).collect()
}
