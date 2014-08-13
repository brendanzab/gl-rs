#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;

generate_gl_bindings!("gl", "core", "4.5", "struct")
