#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

// TODO: egl not yet implemented

/*mod egl_static {
    pub type khronos_utime_nanoseconds_t = libc::c_int;
    pub type khronos_uint64_t = libc::uint64_t;
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings!("egl", "core", "1.5", "static")
}

mod egl_struct {
    pub type khronos_utime_nanoseconds_t = libc::c_int;
    pub type khronos_uint64_t = libc::uint64_t;
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings!("egl", "core", "1.5", "struct")
}*/
