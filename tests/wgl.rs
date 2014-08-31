#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

mod wgl_static {
    generate_gl_bindings!("wgl", "core", "1.0", "static")
}

mod wgl_struct {
    generate_gl_bindings!("wgl", "core", "1.0", "struct")
}
