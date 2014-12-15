extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::io::File;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");

    let mut file = File::create(&dest.join("bindings.rs")).unwrap();
    (write!(&mut file, "{}", bindings)).unwrap();


    // writing tests files
    // FIXME (https://github.com/rust-lang/cargo/issues/1058): only build the tests file if
    //                                                         we run "cargo test"
    //if os::getenv("PROFILE").unwrap() == "test" {
        write_test_gen_symbols(&dest);
        write_test_no_warnings(&dest);
    //}
}

fn write_test_gen_symbols(dest: &Path) {
    let mut file = File::create(&dest.join("test_gen_symbols.rs")).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");
    (write!(&mut file, "mod gl {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gles2,
                                                   khronos_api::GL_XML, vec![], "3.1", "core");
    (write!(&mut file, "mod gles {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Glx,
                                                   khronos_api::GLX_XML, vec![], "1.4", "core");
    (write!(&mut file, "mod glx {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Wgl,
                                                   khronos_api::WGL_XML, vec![], "1.0", "core");
    (write!(&mut file, "mod wgl {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Egl,
                                                   khronos_api::EGL_XML, vec![], "1.5", "core");
    (write!(&mut file, "mod egl {{ {} {} }}", build_egl_symbols(), bindings)).unwrap();
}

fn write_test_no_warnings(dest: &Path) {
    let mut file = File::create(&dest.join("test_no_warnings.rs")).unwrap();



    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");
    (write!(&mut file, "mod gl_global {{ {} }}", bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");
    (write!(&mut file, "mod gl_static {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");
    (write!(&mut file, "mod gl_struct {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML, vec![], "4.5", "core");
    (write!(&mut file, "mod gl_static_struct {{ {} }}", bindings)).unwrap();
    


    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Glx,
                                                   khronos_api::GLX_XML, vec![], "1.4", "core");
    (write!(&mut file, "mod glx_global {{ {} }}", bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Glx,
                                                   khronos_api::GLX_XML, vec![], "1.4", "core");
    (write!(&mut file, "mod glx_static {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Glx,
                                                   khronos_api::GLX_XML, vec![], "1.4", "core");
    (write!(&mut file, "mod glx_struct {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Glx,
                                                   khronos_api::GLX_XML, vec![], "1.4", "core");
    (write!(&mut file, "mod glx_static_struct {{ {} }}", bindings)).unwrap();
    


    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Wgl,
                                                   khronos_api::WGL_XML, vec![], "1.0", "core");
    (write!(&mut file, "mod wgl_global {{ {} }}", bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Wgl,
                                                   khronos_api::WGL_XML, vec![], "1.0", "core");
    (write!(&mut file, "mod wgl_static {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Wgl,
                                                   khronos_api::WGL_XML, vec![], "1.0", "core");
    (write!(&mut file, "mod wgl_struct {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Wgl,
                                                   khronos_api::WGL_XML, vec![], "1.0", "core");
    (write!(&mut file, "mod wgl_static_struct {{ {} }}", bindings)).unwrap();
    


    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gles1,
                                                   khronos_api::GL_XML, vec![], "1.1", "core");
    (write!(&mut file, "mod gles1_global {{ {} }}", bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Gles1,
                                                   khronos_api::GL_XML, vec![], "1.1", "core");
    (write!(&mut file, "mod gles1_static {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Gles1,
                                                   khronos_api::GL_XML, vec![], "1.1", "core");
    (write!(&mut file, "mod gles1_struct {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Gles1,
                                                   khronos_api::GL_XML, vec![], "1.1", "core");
    (write!(&mut file, "mod gles1_static_struct {{ {} }}", bindings)).unwrap();
    


    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Gles2,
                                                   khronos_api::GL_XML, vec![], "3.1", "core");
    (write!(&mut file, "mod gles2_global {{ {} }}", bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Gles2,
                                                   khronos_api::GL_XML, vec![], "3.1", "core");
    (write!(&mut file, "mod gles2_static {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Gles2,
                                                   khronos_api::GL_XML, vec![], "3.1", "core");
    (write!(&mut file, "mod gles2_struct {{ {} }}", bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Gles2,
                                                   khronos_api::GL_XML, vec![], "3.1", "core");
    (write!(&mut file, "mod gles2_static_struct {{ {} }}", bindings)).unwrap();
    


    let bindings = gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                                   gl_generator::registry::Ns::Egl,
                                                   khronos_api::EGL_XML, vec![], "1.5", "core");
    (write!(&mut file, "mod egl_global {{ {} {} }}", build_egl_symbols(), bindings)).unwrap();

    let bindings = gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                                   gl_generator::registry::Ns::Egl,
                                                   khronos_api::EGL_XML, vec![], "1.5", "core");
    (write!(&mut file, "mod egl_static {{ {} {} }}", build_egl_symbols(), bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Egl,
                                                   khronos_api::EGL_XML, vec![], "1.5", "core");
    (write!(&mut file, "mod egl_struct {{ {} {} }}", build_egl_symbols(), bindings)).unwrap();
    
    let bindings = gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                                   gl_generator::registry::Ns::Egl,
                                                   khronos_api::EGL_XML, vec![], "1.5", "core");
    (write!(&mut file, "mod egl_static_struct {{ {} {} }}", build_egl_symbols(), bindings)).unwrap();
    

}

fn build_egl_symbols() -> &'static str {
    "
        #![allow(non_camel_case_types)]

        use libc;

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
    "
}
