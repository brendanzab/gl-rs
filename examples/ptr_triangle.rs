extern mod glfw;
extern mod gl;

use std::cast::transmute;
use std::ptr::null;
use std::sys::size_of;
use gl::types::*;

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

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

trait GLCustom {
    fn create_shader(&self, src: &str, ty: GLenum) -> GLuint;
    fn create_program(&self, vs: GLuint, fs: GLuint) -> GLuint;
}

impl GLCustom for gl::GL {
fn create_shader(&self, src: &str, ty: GLenum) -> GLuint {
    let shader = self.CreateShader(ty);
    unsafe {
        do src.with_c_str |ptr| {
            self.ShaderSource(shader, 1, &ptr, null());
        }
    }
    self.CompileShader(shader);
    shader
}

fn create_program(&self, vs: GLuint, fs: GLuint) -> GLuint {
    let program = self.CreateProgram();
    self.AttachShader(program, vs);
    self.AttachShader(program, fs);
    self.LinkProgram(program);
    program
}
}

fn main() {
    do glfw::set_error_callback |_, description| {
        printfln!("GLFW Error: %s", description);
    }

    do glfw::start {
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint::context_version(3, 2);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);

        let window = glfw::Window::create(800, 600, "OpenGL", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load the OpenGL function pointers
        let gl = gl::load_with(glfw::get_proc_address);

        // Create GLSL shaders
        let vs = gl.create_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = gl.create_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = gl.create_program(vs, fs);

        let vao = 0;
        let vbo = 0;
        unsafe {
            // Create Vertex Array Object
            gl.GenVertexArrays(1, &vao);
            gl.BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl.GenBuffers(1, &vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                           (VERTEX_DATA.len() * size_of::<GLfloat>()) as GLsizeiptr,
                           transmute(&VERTEX_DATA[0]),
                           gl::STATIC_DRAW);

            // Use shader program
            gl.UseProgram(program);
            do "out_color".with_c_str |ptr| {
                gl.BindFragDataLocation(program, 0, ptr);
            }

            // Specify the layout of the vertex data
            let pos_attr = do "position".with_c_str |ptr| {
                gl.GetAttribLocation(program, ptr)
            };
            gl.EnableVertexAttribArray(pos_attr as GLuint);
            gl.VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean, 0, null());
        }

        while !window.should_close() {
            // Poll events
            glfw::poll_events();

            // Clear the screen to black
            gl.ClearColor(0.3, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl.DrawArrays(gl::TRIANGLES, 0, 3);

            // Swap buffers
            window.swap_buffers();
        }

        // Cleanup
        gl.DeleteProgram(program);
        gl.DeleteShader(fs);
        gl.DeleteShader(vs);
        unsafe { gl.DeleteBuffers(1, &vbo) };
        unsafe { gl.DeleteVertexArrays(1, &vao) };
    }
}
