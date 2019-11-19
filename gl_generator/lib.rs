// Based on the `gl_generator` crate, by Brendan Zabarauskas and the gl-rs
// developers (2015). Used under Apache-2.0 license. All new content is also
// available under the Apache-2.0 license.

//! This is a fork of [gl_generator](https://docs.rs/gl_generator).
//!
//! Currently experimental.
//!
//! # Usage
//!
//! First you create a [`Registry`], which describes the GL that you want to
//! use.
//!
//! Next you call `write_bindings`, and specify the [`Generator`], which will
//! determines how you will actually interact with the specified GL API:
//!
//! * [`GlobalGenerator`]: Stores all loaded GL functions in `static mut` or
//!   `AtomicPtr` variables. This allows GL functions to be accessed globally.
//!   All functions are initially not loaded, and you must provide a loader
//!   function which takes `*const c_char` null-terminated strings and produces
//!   `*const c_void` function pointers. Depending on how you're accessing GL
//!   this will be `wglGetProcAddress` or `SDL_GL_GetProcAddress` or similar.
//! * [`StructGenerator`]: Similar to the above, but it stores all loaded GL
//!   functions in a very large struct with one field per function pointer. This
//!   allows you to potentially access more than one GL driver in the same
//!   program, but is otherwise not very helpful.
//!
//! The resulting bindings will depend on `libc` on `not(windows)` systems for
//! all of the C types, but are otherwise completely `no_std` friendly.

#[cfg(feature = "unstable_generator_utils")]
pub mod generators;
#[cfg(not(feature = "unstable_generator_utils"))]
mod generators;

mod registry;

pub use generators::{
  global_gen::GlobalGenerator, struct_gen::StructGenerator,
};

pub use registry::*;
