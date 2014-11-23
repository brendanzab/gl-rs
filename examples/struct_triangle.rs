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

#![feature(globs)]
#![feature(phase)]

#[phase(plugin)]
extern crate gl_generator;
extern crate glfw;
extern crate libc;

use glfw::{Context, OpenGlProfileHint, WindowHint};
use std::mem;
use std::ptr;
use std::str;

use self::gl::Gl;
use self::gl::types::*;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod gl {
    use self::types::*;
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "3.2",
        generator: "struct",
    }
}

// Vertex data
static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

fn compile_shader(gl: &Gl, src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl.CreateShader(ty);
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl.ShaderSource(shader, 1, &ptr, ptr::null()));
        gl.CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl.GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(gl: &Gl, vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl.CreateProgram();
    gl.AttachShader(program, vs);
    gl.AttachShader(program, fs);
    gl.LinkProgram(program);
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl.GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl.GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(WindowHint::OpenglProfile(OpenGlProfileHint::Core));

    let (window, _) = glfw.create_window(800, 600, "OpenGL", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    // It is essential to make the context current before calling `gl::load_with`.
    window.make_current();

    // Load the OpenGL function pointers
    let gl = Gl::load_with(|s| window.get_proc_address(s));

    // Create GLSL shaders
    let vs = compile_shader(&gl, VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(&gl, FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(&gl, vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(gl::ARRAY_BUFFER,
                      (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      mem::transmute(&VERTEX_DATA[0]),
                      gl::STATIC_DRAW);

        // Use shader program
        gl.UseProgram(program);
        "out_color".with_c_str(|ptr| gl.BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl.GetAttribLocation(program, ptr));
        gl.EnableVertexAttribArray(pos_attr as GLuint);
        gl.VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                               gl::FALSE as GLboolean, 0, ptr::null());
    }

    while !window.should_close() {
        // Poll events
        glfw.poll_events();

        unsafe {
            // Clear the screen to black
            gl.ClearColor(0.3, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap buffers
        window.swap_buffers();
    }

    unsafe {
        // Cleanup
        gl.DeleteProgram(program);
        gl.DeleteShader(fs);
        gl.DeleteShader(vs);
        gl.DeleteBuffers(1, &vbo);
        gl.DeleteVertexArrays(1, &vao);
    }
}
