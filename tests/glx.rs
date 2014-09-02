#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

mod glx_static {
    generate_gl_bindings!("glx", "core", "1.4", "static")
}

mod glx_struct {
    generate_gl_bindings!("glx", "core", "1.4", "struct")
}
