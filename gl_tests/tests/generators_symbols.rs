// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
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

use std::os::raw;

include!(concat!(env!("OUT_DIR"), "/test_gen_symbols.rs"));

#[test]
#[ignore]
fn test_gl() { unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    let _: raw::c_uint = gl::CreateProgram();
    gl::CompileShader(5);

    gl::GetActiveUniformBlockiv(0, 0, gl::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
        std::ptr::null_mut());

    let _: *mut raw::c_void = gl::MapBuffer(0, 0);
}}

#[test]
#[ignore]
fn test_gles() { unsafe {
    gles::Clear(gles::COLOR_BUFFER_BIT);
    let _: raw::c_uint = gles::CreateProgram();
    gles::CompileShader(5);
}}

#[test]
#[ignore]
fn test_glx() { unsafe {
    let _ = glx::GetProcAddress(std::mem::uninitialized());
    glx::SwapBuffers(std::mem::uninitialized(), std::mem::uninitialized());
}}

#[test]
#[ignore]
fn test_wgl() { unsafe {
    let _: wgl::types::HGLRC = wgl::CreateContext(std::mem::uninitialized());
}}

#[test]
#[ignore]
fn test_egl() { unsafe {
    let _ = [
        egl::SURFACE_TYPE, egl::WINDOW_BIT,
        egl::BLUE_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::RED_SIZE, 8,
        egl::NONE
    ];

    let _ = egl::GetDisplay(egl::DEFAULT_DISPLAY);
    egl::Terminate(std::mem::uninitialized());
}}
