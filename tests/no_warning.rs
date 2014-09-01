#![feature(phase)]

//! Making sure that no warning is generated by code from generate_gl_bindings!
#![deny(dead_code)]
#![deny(non_snake_case_functions)]
#![deny(uppercase_variables)]
#![deny(unused_variable)]

#[phase(plugin)]
extern crate gl_generator;

mod gl_static {
    generate_gl_bindings!("gl", "core", "4.5", "static")
}

mod gl_struct {
    generate_gl_bindings!("gl", "core", "4.5", "struct")
}

mod wgl_static {
    generate_gl_bindings!("wgl", "core", "1.0", "static")
}

mod wgl_struct {
    generate_gl_bindings!("wgl", "core", "1.0", "struct")
}
