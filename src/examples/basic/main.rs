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

extern crate native;
extern crate glfw = "glfw-rs";
extern crate gl;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    glfw::set_error_callback(~ErrorContext);

    glfw::start(proc() {
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint::context_version(3, 2);
        glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
        glfw::window_hint::opengl_forward_compat(true);

        let window = glfw::Window::create(800, 600, "OpenGL", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load the OpenGL function pointers
        gl::load_with(glfw::get_proc_address);

        while !window.should_close() {
            // Poll events
            glfw::poll_events();

            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Swap buffers
            window.swap_buffers();
        }
    });
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}
