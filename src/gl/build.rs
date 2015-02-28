extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::old_io::File;
use std::old_io::BufferedWriter;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = BufferedWriter::new(File::create(&dest.join("bindings.rs")).unwrap());
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();


    // writing tests files
    if os::getenv("PROFILE").unwrap() == "test" {
        write_test_gen_symbols(&dest);
        write_test_no_warnings(&dest);
    }
}

fn write_test_gen_symbols(dest: &Path) {
    let mut file = BufferedWriter::new(File::create(&dest.join("test_gen_symbols.rs")).unwrap());

    (writeln!(&mut file, "mod gl {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "3.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod glx {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML, vec![], "1.4", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod wgl {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Wgl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::WGL_XML, vec![], "1.0", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod egl {{ {}", build_egl_symbols())).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML, vec![], "1.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();
}

fn write_test_no_warnings(dest: &Path) {
    let mut file = BufferedWriter::new(File::create(&dest.join("test_no_warnings.rs")).unwrap());

    (writeln!(&mut file, "mod gl_global {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gl_static {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gl_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gl_static_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "4.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();



    (writeln!(&mut file, "mod glx_global {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML, vec![], "1.4", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod glx_static {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML, vec![], "1.4", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod glx_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML, vec![], "1.4", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod glx_static_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML, vec![], "1.4", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();



    (writeln!(&mut file, "mod wgl_global {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Wgl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::WGL_XML, vec![], "1.0", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod wgl_static {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Wgl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::WGL_XML, vec![], "1.0", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod wgl_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Wgl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::WGL_XML, vec![], "1.0", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod wgl_static_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Wgl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::WGL_XML, vec![], "1.0", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();



    (writeln!(&mut file, "mod gles1_global {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gles1,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "1.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles1_static {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Gles1,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "1.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles1_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Gles1,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "1.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles1_static_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Gles1,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "1.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();



    (writeln!(&mut file, "mod gles2_global {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "3.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles2_static {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "3.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles2_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "3.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod gles2_static_struct {{")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML, vec![], "3.1", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();



    (writeln!(&mut file, "mod egl_global {{ {}", build_egl_symbols())).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML, vec![], "1.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod egl_static {{ {}", build_egl_symbols())).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML, vec![], "1.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod egl_struct {{ {}", build_egl_symbols())).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML, vec![], "1.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();

    (writeln!(&mut file, "mod egl_static_struct {{ {}", build_egl_symbols())).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticStructGenerator,
                                    gl_generator::registry::Ns::Egl,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::EGL_XML, vec![], "1.5", "core",
                                    &mut file).unwrap();
    (writeln!(&mut file, "}}")).unwrap();


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
