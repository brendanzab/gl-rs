#![allow(non_camel_case_types)]
#![deny(warnings)]
#![no_std]

// no-std compatible C typedefs, so we don't have to depend
// on std::os - https://crates.io/crates/cty @ 09bc8e384a7f5bd59f305a6f30c087a32633b690
//
// Copyright (c) 2017 Jorge Aparicio
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Type aliases to C types like c_int for use with bindgen
//
// # MSRV
//
// This crate is guaranteed to compile on stable Rust 1.30.0 and up. It *might* compile with older
// versions but that may change in any new patch release.

// AD = Architecture dependent
pub use self::ad::*;
// OD = OS dependent
pub use self::od::*;
// PWD = Pointer Width Dependent
pub use self::pwd::*;

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "asmjs",
          target_arch = "wasm32",
          target_arch = "wasm64",
          target_arch = "powerpc",
          target_arch = "powerpc64",
          target_arch = "s390x",
          target_arch = "riscv32",
          target_arch = "riscv64"))]
mod ad {
    pub type c_char = super::c_uchar;

    pub type c_int = i32;
    pub type c_uint = u32;
}

#[cfg(any(target_arch = "mips",
          target_arch = "mips64",
          target_arch = "sparc64",
          target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "nvptx",
          target_arch = "nvptx64",
          target_arch = "xtensa"))]
mod ad {
    pub type c_char = super::c_schar;

    pub type c_int = i32;
    pub type c_uint = u32;
}

#[cfg(target_arch = "msp430")]
mod ad {
    pub type c_char = super::c_uchar;

    pub type c_int = i16;
    pub type c_uint = u16;
}

// NOTE c_{,u}long definitions come from libc v0.2.3
#[cfg(not(any(windows,
              target_os = "redox",
              target_os = "solaris")))]
mod od {
    #[cfg(any(target_pointer_width = "16",
              target_pointer_width = "32"))]
    pub type c_long = i32;
    #[cfg(any(target_pointer_width = "16",
              target_pointer_width = "32"))]
    pub type c_ulong = u32;

    #[cfg(target_pointer_width = "64")]
    pub type c_long = i64;
    #[cfg(target_pointer_width = "64")]
    pub type c_ulong = u64;
}

#[cfg(windows)]
mod od {
    pub type c_long = i32;
    pub type c_ulong = u32;
}

#[cfg(any(target_os = "redox",
          target_os = "solaris"))]
mod od {
    pub type c_long = i64;
    pub type c_ulong = u64;
}

#[cfg(target_pointer_width = "32")]
mod pwd {}

#[cfg(target_pointer_width = "64")]
mod pwd {}

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_short = i16;
pub type c_longlong = i64;

pub type c_uchar = u8;
pub type c_ushort = u16;
pub type c_ulonglong = u64;

pub type c_float = f32;
pub type c_double = f64;

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type c_void = core::ffi::c_void;
