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

#![crate_id = "github.com/bjz/gl-rs#gl:0.1"]
#![comment = "An OpenGL function loader."]
#![license = "ASL2"]
#![crate_type = "lib"]

#![feature(macro_rules)]
#![feature(globs)]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::*;
use self::types::*;

pub mod types {
    use libc::*;
    
    // Common types from OpenGL 1.1
    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLclampx = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;
    pub type GLeglImageOES = *c_void;
    pub type GLchar = c_char;
    pub type GLcharARB = c_char;
    
    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = c_uint;
    
    pub type GLhalfARB = c_ushort;
    pub type GLhalf = c_ushort;
    
    // Must be 32 bits
    pub type GLfixed = GLint;
    
    pub type GLintptr = ptrdiff_t;
    pub type GLsizeiptr = ptrdiff_t;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = ptrdiff_t;
    pub type GLsizeiptrARB = ptrdiff_t;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;
    
    pub struct __GLsync;
    pub type GLsync = *__GLsync;
    
    // compatible with OpenCL cl_context
    pub struct _cl_context;
    pub struct _cl_event;
    
    pub type GLDEBUGPROC = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    
    // Vendor extension types
    pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLhalfNV = c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;
}

pub static DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub static STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub static COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub static CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub static CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub static CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub static MAP_READ_BIT: GLenum = 0x0001;
pub static MAP_WRITE_BIT: GLenum = 0x0002;
pub static MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub static MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub static MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub static MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
pub static SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub static FALSE: GLboolean = 0;
pub static NO_ERROR: GLenum = 0;
pub static ZERO: GLenum = 0;
pub static NONE: GLenum = 0;
pub static TRUE: GLboolean = 1;
pub static ONE: GLenum = 1;
pub static INVALID_INDEX: GLenum = 0xFFFFFFFF;
pub static TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub static POINTS: GLenum = 0x0000;
pub static LINES: GLenum = 0x0001;
pub static LINE_LOOP: GLenum = 0x0002;
pub static LINE_STRIP: GLenum = 0x0003;
pub static TRIANGLES: GLenum = 0x0004;
pub static TRIANGLE_STRIP: GLenum = 0x0005;
pub static TRIANGLE_FAN: GLenum = 0x0006;
pub static LINES_ADJACENCY: GLenum = 0x000A;
pub static LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub static TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub static TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub static NEVER: GLenum = 0x0200;
pub static LESS: GLenum = 0x0201;
pub static EQUAL: GLenum = 0x0202;
pub static LEQUAL: GLenum = 0x0203;
pub static GREATER: GLenum = 0x0204;
pub static NOTEQUAL: GLenum = 0x0205;
pub static GEQUAL: GLenum = 0x0206;
pub static ALWAYS: GLenum = 0x0207;
pub static SRC_COLOR: GLenum = 0x0300;
pub static ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub static SRC_ALPHA: GLenum = 0x0302;
pub static ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub static DST_ALPHA: GLenum = 0x0304;
pub static ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub static DST_COLOR: GLenum = 0x0306;
pub static ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub static SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub static FRONT_LEFT: GLenum = 0x0400;
pub static FRONT_RIGHT: GLenum = 0x0401;
pub static BACK_LEFT: GLenum = 0x0402;
pub static BACK_RIGHT: GLenum = 0x0403;
pub static FRONT: GLenum = 0x0404;
pub static BACK: GLenum = 0x0405;
pub static LEFT: GLenum = 0x0406;
pub static RIGHT: GLenum = 0x0407;
pub static FRONT_AND_BACK: GLenum = 0x0408;
pub static INVALID_ENUM: GLenum = 0x0500;
pub static INVALID_VALUE: GLenum = 0x0501;
pub static INVALID_OPERATION: GLenum = 0x0502;
pub static OUT_OF_MEMORY: GLenum = 0x0505;
pub static INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub static CW: GLenum = 0x0900;
pub static CCW: GLenum = 0x0901;
pub static POINT_SIZE: GLenum = 0x0B11;
pub static POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static LINE_SMOOTH: GLenum = 0x0B20;
pub static LINE_WIDTH: GLenum = 0x0B21;
pub static LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static POLYGON_MODE: GLenum = 0x0B40;
pub static POLYGON_SMOOTH: GLenum = 0x0B41;
pub static CULL_FACE: GLenum = 0x0B44;
pub static CULL_FACE_MODE: GLenum = 0x0B45;
pub static FRONT_FACE: GLenum = 0x0B46;
pub static DEPTH_RANGE: GLenum = 0x0B70;
pub static DEPTH_TEST: GLenum = 0x0B71;
pub static DEPTH_WRITEMASK: GLenum = 0x0B72;
pub static DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub static DEPTH_FUNC: GLenum = 0x0B74;
pub static STENCIL_TEST: GLenum = 0x0B90;
pub static STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub static STENCIL_FUNC: GLenum = 0x0B92;
pub static STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub static STENCIL_FAIL: GLenum = 0x0B94;
pub static STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub static STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub static STENCIL_REF: GLenum = 0x0B97;
pub static STENCIL_WRITEMASK: GLenum = 0x0B98;
pub static VIEWPORT: GLenum = 0x0BA2;
pub static DITHER: GLenum = 0x0BD0;
pub static BLEND_DST: GLenum = 0x0BE0;
pub static BLEND_SRC: GLenum = 0x0BE1;
pub static BLEND: GLenum = 0x0BE2;
pub static LOGIC_OP_MODE: GLenum = 0x0BF0;
pub static COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub static DRAW_BUFFER: GLenum = 0x0C01;
pub static READ_BUFFER: GLenum = 0x0C02;
pub static SCISSOR_BOX: GLenum = 0x0C10;
pub static SCISSOR_TEST: GLenum = 0x0C11;
pub static COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub static COLOR_WRITEMASK: GLenum = 0x0C23;
pub static DOUBLEBUFFER: GLenum = 0x0C32;
pub static STEREO: GLenum = 0x0C33;
pub static LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub static POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub static UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub static UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub static UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub static UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub static UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub static UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub static PACK_SWAP_BYTES: GLenum = 0x0D00;
pub static PACK_LSB_FIRST: GLenum = 0x0D01;
pub static PACK_ROW_LENGTH: GLenum = 0x0D02;
pub static PACK_SKIP_ROWS: GLenum = 0x0D03;
pub static PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub static PACK_ALIGNMENT: GLenum = 0x0D05;
pub static MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub static MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub static MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub static SUBPIXEL_BITS: GLenum = 0x0D50;
pub static TEXTURE_1D: GLenum = 0x0DE0;
pub static TEXTURE_2D: GLenum = 0x0DE1;
pub static TEXTURE_WIDTH: GLenum = 0x1000;
pub static TEXTURE_HEIGHT: GLenum = 0x1001;
pub static TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub static TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub static DONT_CARE: GLenum = 0x1100;
pub static FASTEST: GLenum = 0x1101;
pub static NICEST: GLenum = 0x1102;
pub static BYTE: GLenum = 0x1400;
pub static UNSIGNED_BYTE: GLenum = 0x1401;
pub static SHORT: GLenum = 0x1402;
pub static UNSIGNED_SHORT: GLenum = 0x1403;
pub static INT: GLenum = 0x1404;
pub static UNSIGNED_INT: GLenum = 0x1405;
pub static FLOAT: GLenum = 0x1406;
pub static DOUBLE: GLenum = 0x140A;
pub static HALF_FLOAT: GLenum = 0x140B;
pub static CLEAR: GLenum = 0x1500;
pub static AND: GLenum = 0x1501;
pub static AND_REVERSE: GLenum = 0x1502;
pub static COPY: GLenum = 0x1503;
pub static AND_INVERTED: GLenum = 0x1504;
pub static NOOP: GLenum = 0x1505;
pub static XOR: GLenum = 0x1506;
pub static OR: GLenum = 0x1507;
pub static NOR: GLenum = 0x1508;
pub static EQUIV: GLenum = 0x1509;
pub static INVERT: GLenum = 0x150A;
pub static OR_REVERSE: GLenum = 0x150B;
pub static COPY_INVERTED: GLenum = 0x150C;
pub static OR_INVERTED: GLenum = 0x150D;
pub static NAND: GLenum = 0x150E;
pub static SET: GLenum = 0x150F;
pub static TEXTURE: GLenum = 0x1702;
pub static COLOR: GLenum = 0x1800;
pub static DEPTH: GLenum = 0x1801;
pub static STENCIL: GLenum = 0x1802;
pub static STENCIL_INDEX: GLenum = 0x1901;
pub static DEPTH_COMPONENT: GLenum = 0x1902;
pub static RED: GLenum = 0x1903;
pub static GREEN: GLenum = 0x1904;
pub static BLUE: GLenum = 0x1905;
pub static ALPHA: GLenum = 0x1906;
pub static RGB: GLenum = 0x1907;
pub static RGBA: GLenum = 0x1908;
pub static POINT: GLenum = 0x1B00;
pub static LINE: GLenum = 0x1B01;
pub static FILL: GLenum = 0x1B02;
pub static KEEP: GLenum = 0x1E00;
pub static REPLACE: GLenum = 0x1E01;
pub static INCR: GLenum = 0x1E02;
pub static DECR: GLenum = 0x1E03;
pub static VENDOR: GLenum = 0x1F00;
pub static RENDERER: GLenum = 0x1F01;
pub static VERSION: GLenum = 0x1F02;
pub static EXTENSIONS: GLenum = 0x1F03;
pub static NEAREST: GLenum = 0x2600;
pub static LINEAR: GLenum = 0x2601;
pub static NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub static LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub static NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub static LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub static TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub static TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub static TEXTURE_WRAP_S: GLenum = 0x2802;
pub static TEXTURE_WRAP_T: GLenum = 0x2803;
pub static REPEAT: GLenum = 0x2901;
pub static POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub static POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub static POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub static R3_G3_B2: GLenum = 0x2A10;
pub static CLIP_DISTANCE0: GLenum = 0x3000;
pub static CLIP_DISTANCE1: GLenum = 0x3001;
pub static CLIP_DISTANCE2: GLenum = 0x3002;
pub static CLIP_DISTANCE3: GLenum = 0x3003;
pub static CLIP_DISTANCE4: GLenum = 0x3004;
pub static CLIP_DISTANCE5: GLenum = 0x3005;
pub static CLIP_DISTANCE6: GLenum = 0x3006;
pub static CLIP_DISTANCE7: GLenum = 0x3007;
pub static CONSTANT_COLOR: GLenum = 0x8001;
pub static ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub static CONSTANT_ALPHA: GLenum = 0x8003;
pub static ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub static FUNC_ADD: GLenum = 0x8006;
pub static MIN: GLenum = 0x8007;
pub static MAX: GLenum = 0x8008;
pub static BLEND_EQUATION_RGB: GLenum = 0x8009;
pub static FUNC_SUBTRACT: GLenum = 0x800A;
pub static FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub static UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub static UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub static UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub static UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub static UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub static POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub static POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub static RGB4: GLenum = 0x804F;
pub static RGB5: GLenum = 0x8050;
pub static RGB8: GLenum = 0x8051;
pub static RGB10: GLenum = 0x8052;
pub static RGB12: GLenum = 0x8053;
pub static RGB16: GLenum = 0x8054;
pub static RGBA2: GLenum = 0x8055;
pub static RGBA4: GLenum = 0x8056;
pub static RGB5_A1: GLenum = 0x8057;
pub static RGBA8: GLenum = 0x8058;
pub static RGB10_A2: GLenum = 0x8059;
pub static RGBA12: GLenum = 0x805A;
pub static RGBA16: GLenum = 0x805B;
pub static TEXTURE_RED_SIZE: GLenum = 0x805C;
pub static TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub static TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub static TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub static PROXY_TEXTURE_1D: GLenum = 0x8063;
pub static PROXY_TEXTURE_2D: GLenum = 0x8064;
pub static TEXTURE_BINDING_1D: GLenum = 0x8068;
pub static TEXTURE_BINDING_2D: GLenum = 0x8069;
pub static TEXTURE_BINDING_3D: GLenum = 0x806A;
pub static PACK_SKIP_IMAGES: GLenum = 0x806B;
pub static PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub static UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub static UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub static TEXTURE_3D: GLenum = 0x806F;
pub static PROXY_TEXTURE_3D: GLenum = 0x8070;
pub static TEXTURE_DEPTH: GLenum = 0x8071;
pub static TEXTURE_WRAP_R: GLenum = 0x8072;
pub static MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub static MULTISAMPLE: GLenum = 0x809D;
pub static SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub static SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub static SAMPLE_COVERAGE: GLenum = 0x80A0;
pub static SAMPLE_BUFFERS: GLenum = 0x80A8;
pub static SAMPLES: GLenum = 0x80A9;
pub static SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub static SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub static BLEND_DST_RGB: GLenum = 0x80C8;
pub static BLEND_SRC_RGB: GLenum = 0x80C9;
pub static BLEND_DST_ALPHA: GLenum = 0x80CA;
pub static BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub static BGR: GLenum = 0x80E0;
pub static BGRA: GLenum = 0x80E1;
pub static MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub static MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub static POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub static CLAMP_TO_BORDER: GLenum = 0x812D;
pub static CLAMP_TO_EDGE: GLenum = 0x812F;
pub static TEXTURE_MIN_LOD: GLenum = 0x813A;
pub static TEXTURE_MAX_LOD: GLenum = 0x813B;
pub static TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub static TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub static DEPTH_COMPONENT16: GLenum = 0x81A5;
pub static DEPTH_COMPONENT24: GLenum = 0x81A6;
pub static DEPTH_COMPONENT32: GLenum = 0x81A7;
pub static FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub static FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub static FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub static FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub static FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub static FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub static FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub static FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub static FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub static FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub static DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub static MAJOR_VERSION: GLenum = 0x821B;
pub static MINOR_VERSION: GLenum = 0x821C;
pub static NUM_EXTENSIONS: GLenum = 0x821D;
pub static CONTEXT_FLAGS: GLenum = 0x821E;
pub static INDEX: GLenum = 0x8222;
pub static COMPRESSED_RED: GLenum = 0x8225;
pub static COMPRESSED_RG: GLenum = 0x8226;
pub static RG: GLenum = 0x8227;
pub static RG_INTEGER: GLenum = 0x8228;
pub static R8: GLenum = 0x8229;
pub static R16: GLenum = 0x822A;
pub static RG8: GLenum = 0x822B;
pub static RG16: GLenum = 0x822C;
pub static R16F: GLenum = 0x822D;
pub static R32F: GLenum = 0x822E;
pub static RG16F: GLenum = 0x822F;
pub static RG32F: GLenum = 0x8230;
pub static R8I: GLenum = 0x8231;
pub static R8UI: GLenum = 0x8232;
pub static R16I: GLenum = 0x8233;
pub static R16UI: GLenum = 0x8234;
pub static R32I: GLenum = 0x8235;
pub static R32UI: GLenum = 0x8236;
pub static RG8I: GLenum = 0x8237;
pub static RG8UI: GLenum = 0x8238;
pub static RG16I: GLenum = 0x8239;
pub static RG16UI: GLenum = 0x823A;
pub static RG32I: GLenum = 0x823B;
pub static RG32UI: GLenum = 0x823C;
pub static UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub static UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub static UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub static UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub static UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub static UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub static UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub static MIRRORED_REPEAT: GLenum = 0x8370;
pub static ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub static TEXTURE0: GLenum = 0x84C0;
pub static TEXTURE1: GLenum = 0x84C1;
pub static TEXTURE2: GLenum = 0x84C2;
pub static TEXTURE3: GLenum = 0x84C3;
pub static TEXTURE4: GLenum = 0x84C4;
pub static TEXTURE5: GLenum = 0x84C5;
pub static TEXTURE6: GLenum = 0x84C6;
pub static TEXTURE7: GLenum = 0x84C7;
pub static TEXTURE8: GLenum = 0x84C8;
pub static TEXTURE9: GLenum = 0x84C9;
pub static TEXTURE10: GLenum = 0x84CA;
pub static TEXTURE11: GLenum = 0x84CB;
pub static TEXTURE12: GLenum = 0x84CC;
pub static TEXTURE13: GLenum = 0x84CD;
pub static TEXTURE14: GLenum = 0x84CE;
pub static TEXTURE15: GLenum = 0x84CF;
pub static TEXTURE16: GLenum = 0x84D0;
pub static TEXTURE17: GLenum = 0x84D1;
pub static TEXTURE18: GLenum = 0x84D2;
pub static TEXTURE19: GLenum = 0x84D3;
pub static TEXTURE20: GLenum = 0x84D4;
pub static TEXTURE21: GLenum = 0x84D5;
pub static TEXTURE22: GLenum = 0x84D6;
pub static TEXTURE23: GLenum = 0x84D7;
pub static TEXTURE24: GLenum = 0x84D8;
pub static TEXTURE25: GLenum = 0x84D9;
pub static TEXTURE26: GLenum = 0x84DA;
pub static TEXTURE27: GLenum = 0x84DB;
pub static TEXTURE28: GLenum = 0x84DC;
pub static TEXTURE29: GLenum = 0x84DD;
pub static TEXTURE30: GLenum = 0x84DE;
pub static TEXTURE31: GLenum = 0x84DF;
pub static ACTIVE_TEXTURE: GLenum = 0x84E0;
pub static MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub static COMPRESSED_RGB: GLenum = 0x84ED;
pub static COMPRESSED_RGBA: GLenum = 0x84EE;
pub static TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub static TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub static TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub static PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub static MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub static DEPTH_STENCIL: GLenum = 0x84F9;
pub static UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub static MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub static TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub static INCR_WRAP: GLenum = 0x8507;
pub static DECR_WRAP: GLenum = 0x8508;
pub static TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub static TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub static TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub static TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub static TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub static TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub static PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub static MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub static SRC1_ALPHA: GLenum = 0x8589;
pub static VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub static VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub static VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub static VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub static VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub static CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub static VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub static DEPTH_CLAMP: GLenum = 0x864F;
pub static TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub static TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub static NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub static COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub static BUFFER_SIZE: GLenum = 0x8764;
pub static BUFFER_USAGE: GLenum = 0x8765;
pub static STENCIL_BACK_FUNC: GLenum = 0x8800;
pub static STENCIL_BACK_FAIL: GLenum = 0x8801;
pub static STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub static STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub static RGBA32F: GLenum = 0x8814;
pub static RGB32F: GLenum = 0x8815;
pub static RGBA16F: GLenum = 0x881A;
pub static RGB16F: GLenum = 0x881B;
pub static MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub static DRAW_BUFFER0: GLenum = 0x8825;
pub static DRAW_BUFFER1: GLenum = 0x8826;
pub static DRAW_BUFFER2: GLenum = 0x8827;
pub static DRAW_BUFFER3: GLenum = 0x8828;
pub static DRAW_BUFFER4: GLenum = 0x8829;
pub static DRAW_BUFFER5: GLenum = 0x882A;
pub static DRAW_BUFFER6: GLenum = 0x882B;
pub static DRAW_BUFFER7: GLenum = 0x882C;
pub static DRAW_BUFFER8: GLenum = 0x882D;
pub static DRAW_BUFFER9: GLenum = 0x882E;
pub static DRAW_BUFFER10: GLenum = 0x882F;
pub static DRAW_BUFFER11: GLenum = 0x8830;
pub static DRAW_BUFFER12: GLenum = 0x8831;
pub static DRAW_BUFFER13: GLenum = 0x8832;
pub static DRAW_BUFFER14: GLenum = 0x8833;
pub static DRAW_BUFFER15: GLenum = 0x8834;
pub static BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub static TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub static TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub static TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub static COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub static TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub static QUERY_COUNTER_BITS: GLenum = 0x8864;
pub static CURRENT_QUERY: GLenum = 0x8865;
pub static QUERY_RESULT: GLenum = 0x8866;
pub static QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub static MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub static VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub static MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub static ARRAY_BUFFER: GLenum = 0x8892;
pub static ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub static ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub static ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub static VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub static READ_ONLY: GLenum = 0x88B8;
pub static WRITE_ONLY: GLenum = 0x88B9;
pub static READ_WRITE: GLenum = 0x88BA;
pub static BUFFER_ACCESS: GLenum = 0x88BB;
pub static BUFFER_MAPPED: GLenum = 0x88BC;
pub static BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub static STREAM_DRAW: GLenum = 0x88E0;
pub static STREAM_READ: GLenum = 0x88E1;
pub static STREAM_COPY: GLenum = 0x88E2;
pub static STATIC_DRAW: GLenum = 0x88E4;
pub static STATIC_READ: GLenum = 0x88E5;
pub static STATIC_COPY: GLenum = 0x88E6;
pub static DYNAMIC_DRAW: GLenum = 0x88E8;
pub static DYNAMIC_READ: GLenum = 0x88E9;
pub static DYNAMIC_COPY: GLenum = 0x88EA;
pub static PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub static PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub static PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub static PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub static DEPTH24_STENCIL8: GLenum = 0x88F0;
pub static TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub static VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub static MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub static MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub static MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub static SAMPLES_PASSED: GLenum = 0x8914;
pub static GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub static GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub static GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub static CLAMP_READ_COLOR: GLenum = 0x891C;
pub static FIXED_ONLY: GLenum = 0x891D;
pub static UNIFORM_BUFFER: GLenum = 0x8A11;
pub static UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub static UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub static UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub static MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub static MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub static MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub static MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub static MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub static MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub static MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub static UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub static ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub static ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub static UNIFORM_TYPE: GLenum = 0x8A37;
pub static UNIFORM_SIZE: GLenum = 0x8A38;
pub static UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub static UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub static UNIFORM_OFFSET: GLenum = 0x8A3B;
pub static UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub static UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub static UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub static UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub static UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub static UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub static UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub static UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub static UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub static UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub static FRAGMENT_SHADER: GLenum = 0x8B30;
pub static VERTEX_SHADER: GLenum = 0x8B31;
pub static MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub static MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub static MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub static MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub static MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub static MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub static SHADER_TYPE: GLenum = 0x8B4F;
pub static FLOAT_VEC2: GLenum = 0x8B50;
pub static FLOAT_VEC3: GLenum = 0x8B51;
pub static FLOAT_VEC4: GLenum = 0x8B52;
pub static INT_VEC2: GLenum = 0x8B53;
pub static INT_VEC3: GLenum = 0x8B54;
pub static INT_VEC4: GLenum = 0x8B55;
pub static BOOL: GLenum = 0x8B56;
pub static BOOL_VEC2: GLenum = 0x8B57;
pub static BOOL_VEC3: GLenum = 0x8B58;
pub static BOOL_VEC4: GLenum = 0x8B59;
pub static FLOAT_MAT2: GLenum = 0x8B5A;
pub static FLOAT_MAT3: GLenum = 0x8B5B;
pub static FLOAT_MAT4: GLenum = 0x8B5C;
pub static SAMPLER_1D: GLenum = 0x8B5D;
pub static SAMPLER_2D: GLenum = 0x8B5E;
pub static SAMPLER_3D: GLenum = 0x8B5F;
pub static SAMPLER_CUBE: GLenum = 0x8B60;
pub static SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub static SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub static SAMPLER_2D_RECT: GLenum = 0x8B63;
pub static SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub static FLOAT_MAT2x3: GLenum = 0x8B65;
pub static FLOAT_MAT2x4: GLenum = 0x8B66;
pub static FLOAT_MAT3x2: GLenum = 0x8B67;
pub static FLOAT_MAT3x4: GLenum = 0x8B68;
pub static FLOAT_MAT4x2: GLenum = 0x8B69;
pub static FLOAT_MAT4x3: GLenum = 0x8B6A;
pub static DELETE_STATUS: GLenum = 0x8B80;
pub static COMPILE_STATUS: GLenum = 0x8B81;
pub static LINK_STATUS: GLenum = 0x8B82;
pub static VALIDATE_STATUS: GLenum = 0x8B83;
pub static INFO_LOG_LENGTH: GLenum = 0x8B84;
pub static ATTACHED_SHADERS: GLenum = 0x8B85;
pub static ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub static ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub static SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub static ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub static ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub static FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub static SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub static CURRENT_PROGRAM: GLenum = 0x8B8D;
pub static TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub static TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub static TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub static TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub static TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub static UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub static TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub static PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub static TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub static PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub static TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub static TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub static MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub static TEXTURE_BUFFER: GLenum = 0x8C2A;
pub static MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub static TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub static TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub static R11F_G11F_B10F: GLenum = 0x8C3A;
pub static UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub static RGB9_E5: GLenum = 0x8C3D;
pub static UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub static TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub static SRGB: GLenum = 0x8C40;
pub static SRGB8: GLenum = 0x8C41;
pub static SRGB_ALPHA: GLenum = 0x8C42;
pub static SRGB8_ALPHA8: GLenum = 0x8C43;
pub static COMPRESSED_SRGB: GLenum = 0x8C48;
pub static COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub static TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub static TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub static MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub static TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub static TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub static TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub static PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub static TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub static RASTERIZER_DISCARD: GLenum = 0x8C89;
pub static MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub static MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub static INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub static SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub static TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub static TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub static POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub static LOWER_LEFT: GLenum = 0x8CA1;
pub static UPPER_LEFT: GLenum = 0x8CA2;
pub static STENCIL_BACK_REF: GLenum = 0x8CA3;
pub static STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub static STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub static DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub static READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub static DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub static READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub static RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub static DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub static DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub static FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub static FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub static FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub static FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub static FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub static FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub static FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub static FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub static MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub static COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub static COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub static COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub static COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub static COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub static COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub static COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub static COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub static COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub static COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub static COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub static COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub static COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub static COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub static COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub static COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub static DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub static STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub static FRAMEBUFFER: GLenum = 0x8D40;
pub static RENDERBUFFER: GLenum = 0x8D41;
pub static RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub static RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub static RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub static STENCIL_INDEX1: GLenum = 0x8D46;
pub static STENCIL_INDEX4: GLenum = 0x8D47;
pub static STENCIL_INDEX8: GLenum = 0x8D48;
pub static STENCIL_INDEX16: GLenum = 0x8D49;
pub static RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub static RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub static RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub static RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub static RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub static RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub static FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub static MAX_SAMPLES: GLenum = 0x8D57;
pub static RGBA32UI: GLenum = 0x8D70;
pub static RGB32UI: GLenum = 0x8D71;
pub static RGBA16UI: GLenum = 0x8D76;
pub static RGB16UI: GLenum = 0x8D77;
pub static RGBA8UI: GLenum = 0x8D7C;
pub static RGB8UI: GLenum = 0x8D7D;
pub static RGBA32I: GLenum = 0x8D82;
pub static RGB32I: GLenum = 0x8D83;
pub static RGBA16I: GLenum = 0x8D88;
pub static RGB16I: GLenum = 0x8D89;
pub static RGBA8I: GLenum = 0x8D8E;
pub static RGB8I: GLenum = 0x8D8F;
pub static RED_INTEGER: GLenum = 0x8D94;
pub static GREEN_INTEGER: GLenum = 0x8D95;
pub static BLUE_INTEGER: GLenum = 0x8D96;
pub static RGB_INTEGER: GLenum = 0x8D98;
pub static RGBA_INTEGER: GLenum = 0x8D99;
pub static BGR_INTEGER: GLenum = 0x8D9A;
pub static BGRA_INTEGER: GLenum = 0x8D9B;
pub static FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub static FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub static FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub static FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub static COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub static COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub static COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub static COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub static SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub static SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub static SAMPLER_BUFFER: GLenum = 0x8DC2;
pub static SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub static SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub static SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub static UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub static UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub static UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub static INT_SAMPLER_1D: GLenum = 0x8DC9;
pub static INT_SAMPLER_2D: GLenum = 0x8DCA;
pub static INT_SAMPLER_3D: GLenum = 0x8DCB;
pub static INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub static INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub static INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub static INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub static INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub static UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub static UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub static UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub static UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub static UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub static UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub static UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub static UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub static GEOMETRY_SHADER: GLenum = 0x8DD9;
pub static MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub static MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub static MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub static QUERY_WAIT: GLenum = 0x8E13;
pub static QUERY_NO_WAIT: GLenum = 0x8E14;
pub static QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub static QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub static QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub static FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub static LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub static PROVOKING_VERTEX: GLenum = 0x8E4F;
pub static SAMPLE_POSITION: GLenum = 0x8E50;
pub static SAMPLE_MASK: GLenum = 0x8E51;
pub static SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub static MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub static COPY_READ_BUFFER: GLenum = 0x8F36;
pub static COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub static R8_SNORM: GLenum = 0x8F94;
pub static RG8_SNORM: GLenum = 0x8F95;
pub static RGB8_SNORM: GLenum = 0x8F96;
pub static RGBA8_SNORM: GLenum = 0x8F97;
pub static R16_SNORM: GLenum = 0x8F98;
pub static RG16_SNORM: GLenum = 0x8F99;
pub static RGB16_SNORM: GLenum = 0x8F9A;
pub static RGBA16_SNORM: GLenum = 0x8F9B;
pub static SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub static PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub static PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub static TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub static PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub static TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub static PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub static TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub static TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub static TEXTURE_SAMPLES: GLenum = 0x9106;
pub static TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub static SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub static INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub static UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub static SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub static INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub static UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub static MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub static MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub static MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub static MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub static OBJECT_TYPE: GLenum = 0x9112;
pub static SYNC_CONDITION: GLenum = 0x9113;
pub static SYNC_STATUS: GLenum = 0x9114;
pub static SYNC_FLAGS: GLenum = 0x9115;
pub static SYNC_FENCE: GLenum = 0x9116;
pub static SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub static UNSIGNALED: GLenum = 0x9118;
pub static SIGNALED: GLenum = 0x9119;
pub static ALREADY_SIGNALED: GLenum = 0x911A;
pub static TIMEOUT_EXPIRED: GLenum = 0x911B;
pub static CONDITION_SATISFIED: GLenum = 0x911C;
pub static WAIT_FAILED: GLenum = 0x911D;
pub static BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub static BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub static BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub static MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub static MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub static MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub static MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub static CONTEXT_PROFILE_MASK: GLenum = 0x9126;

#[inline] pub fn ActiveTexture(texture: GLenum) { unsafe { (storage::ActiveTexture.f)(texture) } }
#[inline] pub fn AttachShader(program: GLuint, shader: GLuint) { unsafe { (storage::AttachShader.f)(program, shader) } }
#[inline] pub fn BeginConditionalRender(id: GLuint, mode: GLenum) { unsafe { (storage::BeginConditionalRender.f)(id, mode) } }
#[inline] pub fn BeginQuery(target: GLenum, id: GLuint) { unsafe { (storage::BeginQuery.f)(target, id) } }
#[inline] pub fn BeginTransformFeedback(primitiveMode: GLenum) { unsafe { (storage::BeginTransformFeedback.f)(primitiveMode) } }
#[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar) { (storage::BindAttribLocation.f)(program, index, name) }
#[inline] pub fn BindBuffer(target: GLenum, buffer: GLuint) { unsafe { (storage::BindBuffer.f)(target, buffer) } }
#[inline] pub fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) { unsafe { (storage::BindBufferBase.f)(target, index, buffer) } }
#[inline] pub fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { unsafe { (storage::BindBufferRange.f)(target, index, buffer, offset, size) } }
#[inline] pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *GLchar) { (storage::BindFragDataLocation.f)(program, color, name) }
#[inline] pub fn BindFramebuffer(target: GLenum, framebuffer: GLuint) { unsafe { (storage::BindFramebuffer.f)(target, framebuffer) } }
#[inline] pub fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) { unsafe { (storage::BindRenderbuffer.f)(target, renderbuffer) } }
#[inline] pub fn BindTexture(target: GLenum, texture: GLuint) { unsafe { (storage::BindTexture.f)(target, texture) } }
#[inline] pub fn BindVertexArray(array: GLuint) { unsafe { (storage::BindVertexArray.f)(array) } }
#[inline] pub fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::BlendColor.f)(red, green, blue, alpha) } }
#[inline] pub fn BlendEquation(mode: GLenum) { unsafe { (storage::BlendEquation.f)(mode) } }
#[inline] pub fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) { unsafe { (storage::BlendEquationSeparate.f)(modeRGB, modeAlpha) } }
#[inline] pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum) { unsafe { (storage::BlendFunc.f)(sfactor, dfactor) } }
#[inline] pub fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { unsafe { (storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) } }
#[inline] pub fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { unsafe { (storage::BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } }
#[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *c_void, usage: GLenum) { (storage::BufferData.f)(target, size, data, usage) }
#[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *c_void) { (storage::BufferSubData.f)(target, offset, size, data) }
#[inline] pub fn CheckFramebufferStatus(target: GLenum) -> GLenum { unsafe { (storage::CheckFramebufferStatus.f)(target) } }
#[inline] pub fn ClampColor(target: GLenum, clamp: GLenum) { unsafe { (storage::ClampColor.f)(target, clamp) } }
#[inline] pub fn Clear(mask: GLbitfield) { unsafe { (storage::Clear.f)(mask) } }
#[inline] pub fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { unsafe { (storage::ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) } }
#[inline] pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *GLfloat) { (storage::ClearBufferfv.f)(buffer, drawbuffer, value) }
#[inline] pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *GLint) { (storage::ClearBufferiv.f)(buffer, drawbuffer, value) }
#[inline] pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *GLuint) { (storage::ClearBufferuiv.f)(buffer, drawbuffer, value) }
#[inline] pub fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::ClearColor.f)(red, green, blue, alpha) } }
#[inline] pub fn ClearDepth(depth: GLdouble) { unsafe { (storage::ClearDepth.f)(depth) } }
#[inline] pub fn ClearStencil(s: GLint) { unsafe { (storage::ClearStencil.f)(s) } }
#[inline] pub fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { unsafe { (storage::ClientWaitSync.f)(sync, flags, timeout) } }
#[inline] pub fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { unsafe { (storage::ColorMask.f)(red, green, blue, alpha) } }
#[inline] pub fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) { unsafe { (storage::ColorMaski.f)(index, r, g, b, a) } }
#[inline] pub fn CompileShader(shader: GLuint) { unsafe { (storage::CompileShader.f)(shader) } }
#[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[inline] pub fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { unsafe { (storage::CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) } }
#[inline] pub fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) } }
#[inline] pub fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) } }
#[inline] pub fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { unsafe { (storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) } }
#[inline] pub fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) } }
#[inline] pub fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) } }
#[inline] pub fn CreateProgram() -> GLuint { unsafe { (storage::CreateProgram.f)() } }
#[inline] pub fn CreateShader(type_: GLenum) -> GLuint { unsafe { (storage::CreateShader.f)(type_) } }
#[inline] pub fn CullFace(mode: GLenum) { unsafe { (storage::CullFace.f)(mode) } }
#[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *GLuint) { (storage::DeleteBuffers.f)(n, buffers) }
#[inline] pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *GLuint) { (storage::DeleteFramebuffers.f)(n, framebuffers) }
#[inline] pub fn DeleteProgram(program: GLuint) { unsafe { (storage::DeleteProgram.f)(program) } }
#[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *GLuint) { (storage::DeleteQueries.f)(n, ids) }
#[inline] pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *GLuint) { (storage::DeleteRenderbuffers.f)(n, renderbuffers) }
#[inline] pub fn DeleteShader(shader: GLuint) { unsafe { (storage::DeleteShader.f)(shader) } }
#[inline] pub fn DeleteSync(sync: GLsync) { unsafe { (storage::DeleteSync.f)(sync) } }
#[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *GLuint) { (storage::DeleteTextures.f)(n, textures) }
#[inline] pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *GLuint) { (storage::DeleteVertexArrays.f)(n, arrays) }
#[inline] pub fn DepthFunc(func: GLenum) { unsafe { (storage::DepthFunc.f)(func) } }
#[inline] pub fn DepthMask(flag: GLboolean) { unsafe { (storage::DepthMask.f)(flag) } }
#[inline] pub fn DepthRange(near: GLdouble, far: GLdouble) { unsafe { (storage::DepthRange.f)(near, far) } }
#[inline] pub fn DetachShader(program: GLuint, shader: GLuint) { unsafe { (storage::DetachShader.f)(program, shader) } }
#[inline] pub fn Disable(cap: GLenum) { unsafe { (storage::Disable.f)(cap) } }
#[inline] pub fn DisableVertexAttribArray(index: GLuint) { unsafe { (storage::DisableVertexAttribArray.f)(index) } }
#[inline] pub fn Disablei(target: GLenum, index: GLuint) { unsafe { (storage::Disablei.f)(target, index) } }
#[inline] pub fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) { unsafe { (storage::DrawArrays.f)(mode, first, count) } }
#[inline] pub fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) { unsafe { (storage::DrawArraysInstanced.f)(mode, first, count, instancecount) } }
#[inline] pub fn DrawBuffer(mode: GLenum) { unsafe { (storage::DrawBuffer.f)(mode) } }
#[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *GLenum) { (storage::DrawBuffers.f)(n, bufs) }
#[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void) { (storage::DrawElements.f)(mode, count, type_, indices) }
#[inline] pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, basevertex: GLint) { (storage::DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
#[inline] pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei) { (storage::DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
#[inline] pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, basevertex: GLint) { (storage::DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
#[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void) { (storage::DrawRangeElements.f)(mode, start, end, count, type_, indices) }
#[inline] pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void, basevertex: GLint) { (storage::DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
#[inline] pub fn Enable(cap: GLenum) { unsafe { (storage::Enable.f)(cap) } }
#[inline] pub fn EnableVertexAttribArray(index: GLuint) { unsafe { (storage::EnableVertexAttribArray.f)(index) } }
#[inline] pub fn Enablei(target: GLenum, index: GLuint) { unsafe { (storage::Enablei.f)(target, index) } }
#[inline] pub fn EndConditionalRender() { unsafe { (storage::EndConditionalRender.f)() } }
#[inline] pub fn EndQuery(target: GLenum) { unsafe { (storage::EndQuery.f)(target) } }
#[inline] pub fn EndTransformFeedback() { unsafe { (storage::EndTransformFeedback.f)() } }
#[inline] pub fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { unsafe { (storage::FenceSync.f)(condition, flags) } }
#[inline] pub fn Finish() { unsafe { (storage::Finish.f)() } }
#[inline] pub fn Flush() { unsafe { (storage::Flush.f)() } }
#[inline] pub fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) { unsafe { (storage::FlushMappedBufferRange.f)(target, offset, length) } }
#[inline] pub fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { unsafe { (storage::FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) } }
#[inline] pub fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture.f)(target, attachment, texture, level) } }
#[inline] pub fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture1D.f)(target, attachment, textarget, texture, level) } }
#[inline] pub fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture2D.f)(target, attachment, textarget, texture, level) } }
#[inline] pub fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) { unsafe { (storage::FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) } }
#[inline] pub fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { unsafe { (storage::FramebufferTextureLayer.f)(target, attachment, texture, level, layer) } }
#[inline] pub fn FrontFace(mode: GLenum) { unsafe { (storage::FrontFace.f)(mode) } }
#[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) { (storage::GenBuffers.f)(n, buffers) }
#[inline] pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) { (storage::GenFramebuffers.f)(n, framebuffers) }
#[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) { (storage::GenQueries.f)(n, ids) }
#[inline] pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) { (storage::GenRenderbuffers.f)(n, renderbuffers) }
#[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) { (storage::GenTextures.f)(n, textures) }
#[inline] pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) { (storage::GenVertexArrays.f)(n, arrays) }
#[inline] pub fn GenerateMipmap(target: GLenum) { unsafe { (storage::GenerateMipmap.f)(target) } }
#[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[inline] pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) { (storage::GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
#[inline] pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
#[inline] pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) { (storage::GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
#[inline] pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *mut GLint) { (storage::GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
#[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) { (storage::GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetAttribLocation.f)(program, name) }
#[inline] pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) { (storage::GetBooleani_v.f)(target, index, data) }
#[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) { (storage::GetBooleanv.f)(pname, data) }
#[inline] pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) { (storage::GetBufferParameteri64v.f)(target, pname, params) }
#[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetBufferParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut c_void) { (storage::GetBufferPointerv.f)(target, pname, params) }
#[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) { (storage::GetBufferSubData.f)(target, offset, size, data) }
#[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) { (storage::GetCompressedTexImage.f)(target, level, img) }
#[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) { (storage::GetDoublev.f)(pname, data) }
#[inline] pub fn GetError() -> GLenum { unsafe { (storage::GetError.f)() } }
#[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) { (storage::GetFloatv.f)(pname, data) }
#[inline] pub unsafe fn GetFragDataLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetFragDataLocation.f)(program, name) }
#[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[inline] pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) { (storage::GetInteger64i_v.f)(target, index, data) }
#[inline] pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) { (storage::GetInteger64v.f)(pname, data) }
#[inline] pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) { (storage::GetIntegeri_v.f)(target, index, data) }
#[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) { (storage::GetIntegerv.f)(pname, data) }
#[inline] pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) { (storage::GetMultisamplefv.f)(pname, index, val) }
#[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetProgramiv.f)(program, pname, params) }
#[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetQueryObjectiv.f)(id, pname, params) }
#[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetQueryObjectuiv.f)(id, pname, params) }
#[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetQueryiv.f)(target, pname, params) }
#[inline] pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetRenderbufferParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) { (storage::GetShaderSource.f)(shader, bufSize, length, source) }
#[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetShaderiv.f)(shader, pname, params) }
#[inline] pub fn GetString(name: GLenum) -> *GLubyte { unsafe { (storage::GetString.f)(name) } }
#[inline] pub fn GetStringi(name: GLenum, index: GLuint) -> *GLubyte { unsafe { (storage::GetStringi.f)(name, index) } }
#[inline] pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) { (storage::GetSynciv.f)(sync, pname, bufSize, length, values) }
#[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) { (storage::GetTexImage.f)(target, level, format, type_, pixels) }
#[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) { (storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
#[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) { (storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
#[inline] pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexParameterIiv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) { (storage::GetTexParameterIuiv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetTexParameterfv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) { (storage::GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
#[inline] pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *GLchar) -> GLuint { (storage::GetUniformBlockIndex.f)(program, uniformBlockName) }
#[inline] pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: **GLchar, uniformIndices: *mut GLuint) { (storage::GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
#[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetUniformLocation.f)(program, name) }
#[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) { (storage::GetUniformfv.f)(program, location, params) }
#[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) { (storage::GetUniformiv.f)(program, location, params) }
#[inline] pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) { (storage::GetUniformuiv.f)(program, location, params) }
#[inline] pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetVertexAttribIiv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetVertexAttribIuiv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut c_void) { (storage::GetVertexAttribPointerv.f)(index, pname, pointer) }
#[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { (storage::GetVertexAttribdv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) { (storage::GetVertexAttribfv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetVertexAttribiv.f)(index, pname, params) }
#[inline] pub fn Hint(target: GLenum, mode: GLenum) { unsafe { (storage::Hint.f)(target, mode) } }
#[inline] pub fn IsBuffer(buffer: GLuint) -> GLboolean { unsafe { (storage::IsBuffer.f)(buffer) } }
#[inline] pub fn IsEnabled(cap: GLenum) -> GLboolean { unsafe { (storage::IsEnabled.f)(cap) } }
#[inline] pub fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { unsafe { (storage::IsEnabledi.f)(target, index) } }
#[inline] pub fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { unsafe { (storage::IsFramebuffer.f)(framebuffer) } }
#[inline] pub fn IsProgram(program: GLuint) -> GLboolean { unsafe { (storage::IsProgram.f)(program) } }
#[inline] pub fn IsQuery(id: GLuint) -> GLboolean { unsafe { (storage::IsQuery.f)(id) } }
#[inline] pub fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { unsafe { (storage::IsRenderbuffer.f)(renderbuffer) } }
#[inline] pub fn IsShader(shader: GLuint) -> GLboolean { unsafe { (storage::IsShader.f)(shader) } }
#[inline] pub fn IsSync(sync: GLsync) -> GLboolean { unsafe { (storage::IsSync.f)(sync) } }
#[inline] pub fn IsTexture(texture: GLuint) -> GLboolean { unsafe { (storage::IsTexture.f)(texture) } }
#[inline] pub fn IsVertexArray(array: GLuint) -> GLboolean { unsafe { (storage::IsVertexArray.f)(array) } }
#[inline] pub fn LineWidth(width: GLfloat) { unsafe { (storage::LineWidth.f)(width) } }
#[inline] pub fn LinkProgram(program: GLuint) { unsafe { (storage::LinkProgram.f)(program) } }
#[inline] pub fn LogicOp(opcode: GLenum) { unsafe { (storage::LogicOp.f)(opcode) } }
#[inline] pub fn MapBuffer(target: GLenum, access: GLenum) -> *c_void { unsafe { (storage::MapBuffer.f)(target, access) } }
#[inline] pub fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *c_void { unsafe { (storage::MapBufferRange.f)(target, offset, length, access) } }
#[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei) { (storage::MultiDrawArrays.f)(mode, first, count, drawcount) }
#[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei) { (storage::MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
#[inline] pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei, basevertex: *GLint) { (storage::MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
#[inline] pub fn PixelStoref(pname: GLenum, param: GLfloat) { unsafe { (storage::PixelStoref.f)(pname, param) } }
#[inline] pub fn PixelStorei(pname: GLenum, param: GLint) { unsafe { (storage::PixelStorei.f)(pname, param) } }
#[inline] pub fn PointParameterf(pname: GLenum, param: GLfloat) { unsafe { (storage::PointParameterf.f)(pname, param) } }
#[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *GLfloat) { (storage::PointParameterfv.f)(pname, params) }
#[inline] pub fn PointParameteri(pname: GLenum, param: GLint) { unsafe { (storage::PointParameteri.f)(pname, param) } }
#[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *GLint) { (storage::PointParameteriv.f)(pname, params) }
#[inline] pub fn PointSize(size: GLfloat) { unsafe { (storage::PointSize.f)(size) } }
#[inline] pub fn PolygonMode(face: GLenum, mode: GLenum) { unsafe { (storage::PolygonMode.f)(face, mode) } }
#[inline] pub fn PolygonOffset(factor: GLfloat, units: GLfloat) { unsafe { (storage::PolygonOffset.f)(factor, units) } }
#[inline] pub fn PrimitiveRestartIndex(index: GLuint) { unsafe { (storage::PrimitiveRestartIndex.f)(index) } }
#[inline] pub fn ProvokingVertex(mode: GLenum) { unsafe { (storage::ProvokingVertex.f)(mode) } }
#[inline] pub fn ReadBuffer(mode: GLenum) { unsafe { (storage::ReadBuffer.f)(mode) } }
#[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) { (storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[inline] pub fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) { unsafe { (storage::RenderbufferStorage.f)(target, internalformat, width, height) } }
#[inline] pub fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { unsafe { (storage::RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) } }
#[inline] pub fn SampleCoverage(value: GLfloat, invert: GLboolean) { unsafe { (storage::SampleCoverage.f)(value, invert) } }
#[inline] pub fn SampleMaski(index: GLuint, mask: GLbitfield) { unsafe { (storage::SampleMaski.f)(index, mask) } }
#[inline] pub fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Scissor.f)(x, y, width, height) } }
#[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint) { (storage::ShaderSource.f)(shader, count, string, length) }
#[inline] pub fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFunc.f)(func, ref_, mask) } }
#[inline] pub fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFuncSeparate.f)(face, func, ref_, mask) } }
#[inline] pub fn StencilMask(mask: GLuint) { unsafe { (storage::StencilMask.f)(mask) } }
#[inline] pub fn StencilMaskSeparate(face: GLenum, mask: GLuint) { unsafe { (storage::StencilMaskSeparate.f)(face, mask) } }
#[inline] pub fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) { unsafe { (storage::StencilOp.f)(fail, zfail, zpass) } }
#[inline] pub fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { unsafe { (storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass) } }
#[inline] pub fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) { unsafe { (storage::TexBuffer.f)(target, internalformat, buffer) } }
#[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[inline] pub fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) } }
#[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
#[inline] pub fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) } }
#[inline] pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexParameterIiv.f)(target, pname, params) }
#[inline] pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *GLuint) { (storage::TexParameterIuiv.f)(target, pname, params) }
#[inline] pub fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::TexParameterf.f)(target, pname, param) } }
#[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat) { (storage::TexParameterfv.f)(target, pname, params) }
#[inline] pub fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::TexParameteri.f)(target, pname, param) } }
#[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[inline] pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: **GLchar, bufferMode: GLenum) { (storage::TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
#[inline] pub fn Uniform1f(location: GLint, v0: GLfloat) { unsafe { (storage::Uniform1f.f)(location, v0) } }
#[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform1fv.f)(location, count, value) }
#[inline] pub fn Uniform1i(location: GLint, v0: GLint) { unsafe { (storage::Uniform1i.f)(location, v0) } }
#[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform1iv.f)(location, count, value) }
#[inline] pub fn Uniform1ui(location: GLint, v0: GLuint) { unsafe { (storage::Uniform1ui.f)(location, v0) } }
#[inline] pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform1uiv.f)(location, count, value) }
#[inline] pub fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) { unsafe { (storage::Uniform2f.f)(location, v0, v1) } }
#[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform2fv.f)(location, count, value) }
#[inline] pub fn Uniform2i(location: GLint, v0: GLint, v1: GLint) { unsafe { (storage::Uniform2i.f)(location, v0, v1) } }
#[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform2iv.f)(location, count, value) }
#[inline] pub fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) { unsafe { (storage::Uniform2ui.f)(location, v0, v1) } }
#[inline] pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform2uiv.f)(location, count, value) }
#[inline] pub fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { unsafe { (storage::Uniform3f.f)(location, v0, v1, v2) } }
#[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform3fv.f)(location, count, value) }
#[inline] pub fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) { unsafe { (storage::Uniform3i.f)(location, v0, v1, v2) } }
#[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform3iv.f)(location, count, value) }
#[inline] pub fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { unsafe { (storage::Uniform3ui.f)(location, v0, v1, v2) } }
#[inline] pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform3uiv.f)(location, count, value) }
#[inline] pub fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { unsafe { (storage::Uniform4f.f)(location, v0, v1, v2, v3) } }
#[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform4fv.f)(location, count, value) }
#[inline] pub fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { unsafe { (storage::Uniform4i.f)(location, v0, v1, v2, v3) } }
#[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform4iv.f)(location, count, value) }
#[inline] pub fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { unsafe { (storage::Uniform4ui.f)(location, v0, v1, v2, v3) } }
#[inline] pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform4uiv.f)(location, count, value) }
#[inline] pub fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) { unsafe { (storage::UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) } }
#[inline] pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2x3fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2x4fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3x2fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3x4fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4x2fv.f)(location, count, transpose, value) }
#[inline] pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4x3fv.f)(location, count, transpose, value) }
#[inline] pub fn UnmapBuffer(target: GLenum) -> GLboolean { unsafe { (storage::UnmapBuffer.f)(target) } }
#[inline] pub fn UseProgram(program: GLuint) { unsafe { (storage::UseProgram.f)(program) } }
#[inline] pub fn ValidateProgram(program: GLuint) { unsafe { (storage::ValidateProgram.f)(program) } }
#[inline] pub fn VertexAttrib1d(index: GLuint, x: GLdouble) { unsafe { (storage::VertexAttrib1d.f)(index, x) } }
#[inline] pub unsafe fn VertexAttrib1dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib1dv.f)(index, v) }
#[inline] pub fn VertexAttrib1f(index: GLuint, x: GLfloat) { unsafe { (storage::VertexAttrib1f.f)(index, x) } }
#[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib1fv.f)(index, v) }
#[inline] pub fn VertexAttrib1s(index: GLuint, x: GLshort) { unsafe { (storage::VertexAttrib1s.f)(index, x) } }
#[inline] pub unsafe fn VertexAttrib1sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib1sv.f)(index, v) }
#[inline] pub fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) { unsafe { (storage::VertexAttrib2d.f)(index, x, y) } }
#[inline] pub unsafe fn VertexAttrib2dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib2dv.f)(index, v) }
#[inline] pub fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) { unsafe { (storage::VertexAttrib2f.f)(index, x, y) } }
#[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib2fv.f)(index, v) }
#[inline] pub fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) { unsafe { (storage::VertexAttrib2s.f)(index, x, y) } }
#[inline] pub unsafe fn VertexAttrib2sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib2sv.f)(index, v) }
#[inline] pub fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::VertexAttrib3d.f)(index, x, y, z) } }
#[inline] pub unsafe fn VertexAttrib3dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib3dv.f)(index, v) }
#[inline] pub fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::VertexAttrib3f.f)(index, x, y, z) } }
#[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib3fv.f)(index, v) }
#[inline] pub fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) { unsafe { (storage::VertexAttrib3s.f)(index, x, y, z) } }
#[inline] pub unsafe fn VertexAttrib3sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib3sv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *GLbyte) { (storage::VertexAttrib4Nbv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *GLint) { (storage::VertexAttrib4Niv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *GLshort) { (storage::VertexAttrib4Nsv.f)(index, v) }
#[inline] pub fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) { unsafe { (storage::VertexAttrib4Nub.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *GLubyte) { (storage::VertexAttrib4Nubv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *GLuint) { (storage::VertexAttrib4Nuiv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *GLushort) { (storage::VertexAttrib4Nusv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4bv(index: GLuint, v: *GLbyte) { (storage::VertexAttrib4bv.f)(index, v) }
#[inline] pub fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::VertexAttrib4d.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttrib4dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib4dv.f)(index, v) }
#[inline] pub fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { unsafe { (storage::VertexAttrib4f.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib4fv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4iv(index: GLuint, v: *GLint) { (storage::VertexAttrib4iv.f)(index, v) }
#[inline] pub fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { unsafe { (storage::VertexAttrib4s.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttrib4sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib4sv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *GLubyte) { (storage::VertexAttrib4ubv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *GLuint) { (storage::VertexAttrib4uiv.f)(index, v) }
#[inline] pub unsafe fn VertexAttrib4usv(index: GLuint, v: *GLushort) { (storage::VertexAttrib4usv.f)(index, v) }
#[inline] pub fn VertexAttribI1i(index: GLuint, x: GLint) { unsafe { (storage::VertexAttribI1i.f)(index, x) } }
#[inline] pub unsafe fn VertexAttribI1iv(index: GLuint, v: *GLint) { (storage::VertexAttribI1iv.f)(index, v) }
#[inline] pub fn VertexAttribI1ui(index: GLuint, x: GLuint) { unsafe { (storage::VertexAttribI1ui.f)(index, x) } }
#[inline] pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI1uiv.f)(index, v) }
#[inline] pub fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) { unsafe { (storage::VertexAttribI2i.f)(index, x, y) } }
#[inline] pub unsafe fn VertexAttribI2iv(index: GLuint, v: *GLint) { (storage::VertexAttribI2iv.f)(index, v) }
#[inline] pub fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) { unsafe { (storage::VertexAttribI2ui.f)(index, x, y) } }
#[inline] pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI2uiv.f)(index, v) }
#[inline] pub fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) { unsafe { (storage::VertexAttribI3i.f)(index, x, y, z) } }
#[inline] pub unsafe fn VertexAttribI3iv(index: GLuint, v: *GLint) { (storage::VertexAttribI3iv.f)(index, v) }
#[inline] pub fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) { unsafe { (storage::VertexAttribI3ui.f)(index, x, y, z) } }
#[inline] pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI3uiv.f)(index, v) }
#[inline] pub unsafe fn VertexAttribI4bv(index: GLuint, v: *GLbyte) { (storage::VertexAttribI4bv.f)(index, v) }
#[inline] pub fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) { unsafe { (storage::VertexAttribI4i.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttribI4iv(index: GLuint, v: *GLint) { (storage::VertexAttribI4iv.f)(index, v) }
#[inline] pub unsafe fn VertexAttribI4sv(index: GLuint, v: *GLshort) { (storage::VertexAttribI4sv.f)(index, v) }
#[inline] pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *GLubyte) { (storage::VertexAttribI4ubv.f)(index, v) }
#[inline] pub fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) { unsafe { (storage::VertexAttribI4ui.f)(index, x, y, z, w) } }
#[inline] pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI4uiv.f)(index, v) }
#[inline] pub unsafe fn VertexAttribI4usv(index: GLuint, v: *GLushort) { (storage::VertexAttribI4usv.f)(index, v) }
#[inline] pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
#[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *c_void) { (storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[inline] pub fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Viewport.f)(x, y, width, height) } }
#[inline] pub fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) { unsafe { (storage::WaitSync.f)(sync, flags, timeout) } }

pub struct FnPtr<F> { f: F, is_loaded: bool }

impl<F> FnPtr<F> {
    pub fn new(ptr: Option<extern "system" fn()>, failing_fn: F) -> FnPtr<F> {
        use std::cast::transmute;
        match ptr {
            std::option::Some(p) => FnPtr { f: unsafe { transmute(p) }, is_loaded: true },
            None => FnPtr { f: failing_fn, is_loaded: false },
        }
    }
}

mod storage {
    use libc::*;
    use super::types::*;
    
    macro_rules! fn_ptr(
        (fn $name:ident()) => (
            pub static mut $name: ::FnPtr<extern "system" fn()> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident() -> $ret_ty:ty) => (
            pub static mut $name: ::FnPtr<extern "system" fn() -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident($($arg:ident : $arg_ty:ty),*)) => (
            pub static mut $name: ::FnPtr<extern "system" fn($($arg: $arg_ty),*)> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty) => (
            pub static mut $name: ::FnPtr<extern "system" fn($($arg: $arg_ty),*) -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
    )
    
    fn_ptr!(fn ActiveTexture(texture: GLenum))
    fn_ptr!(fn AttachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn BeginConditionalRender(id: GLuint, mode: GLenum))
    fn_ptr!(fn BeginQuery(target: GLenum, id: GLuint))
    fn_ptr!(fn BeginTransformFeedback(primitiveMode: GLenum))
    fn_ptr!(fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar))
    fn_ptr!(fn BindBuffer(target: GLenum, buffer: GLuint))
    fn_ptr!(fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint))
    fn_ptr!(fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr))
    fn_ptr!(fn BindFragDataLocation(program: GLuint, color: GLuint, name: *GLchar))
    fn_ptr!(fn BindFramebuffer(target: GLenum, framebuffer: GLuint))
    fn_ptr!(fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint))
    fn_ptr!(fn BindTexture(target: GLenum, texture: GLuint))
    fn_ptr!(fn BindVertexArray(array: GLuint))
    fn_ptr!(fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn BlendEquation(mode: GLenum))
    fn_ptr!(fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum))
    fn_ptr!(fn BlendFunc(sfactor: GLenum, dfactor: GLenum))
    fn_ptr!(fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum))
    fn_ptr!(fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum))
    fn_ptr!(fn BufferData(target: GLenum, size: GLsizeiptr, data: *c_void, usage: GLenum))
    fn_ptr!(fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *c_void))
    fn_ptr!(fn CheckFramebufferStatus(target: GLenum) -> GLenum)
    fn_ptr!(fn ClampColor(target: GLenum, clamp: GLenum))
    fn_ptr!(fn Clear(mask: GLbitfield))
    fn_ptr!(fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint))
    fn_ptr!(fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *GLfloat))
    fn_ptr!(fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *GLint))
    fn_ptr!(fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *GLuint))
    fn_ptr!(fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn ClearDepth(depth: GLdouble))
    fn_ptr!(fn ClearStencil(s: GLint))
    fn_ptr!(fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum)
    fn_ptr!(fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean))
    fn_ptr!(fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean))
    fn_ptr!(fn CompileShader(shader: GLuint))
    fn_ptr!(fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr))
    fn_ptr!(fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei))
    fn_ptr!(fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CreateProgram() -> GLuint)
    fn_ptr!(fn CreateShader(type_: GLenum) -> GLuint)
    fn_ptr!(fn CullFace(mode: GLenum))
    fn_ptr!(fn DeleteBuffers(n: GLsizei, buffers: *GLuint))
    fn_ptr!(fn DeleteFramebuffers(n: GLsizei, framebuffers: *GLuint))
    fn_ptr!(fn DeleteProgram(program: GLuint))
    fn_ptr!(fn DeleteQueries(n: GLsizei, ids: *GLuint))
    fn_ptr!(fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *GLuint))
    fn_ptr!(fn DeleteShader(shader: GLuint))
    fn_ptr!(fn DeleteSync(sync: GLsync))
    fn_ptr!(fn DeleteTextures(n: GLsizei, textures: *GLuint))
    fn_ptr!(fn DeleteVertexArrays(n: GLsizei, arrays: *GLuint))
    fn_ptr!(fn DepthFunc(func: GLenum))
    fn_ptr!(fn DepthMask(flag: GLboolean))
    fn_ptr!(fn DepthRange(near: GLdouble, far: GLdouble))
    fn_ptr!(fn DetachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn Disable(cap: GLenum))
    fn_ptr!(fn DisableVertexAttribArray(index: GLuint))
    fn_ptr!(fn Disablei(target: GLenum, index: GLuint))
    fn_ptr!(fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei))
    fn_ptr!(fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei))
    fn_ptr!(fn DrawBuffer(mode: GLenum))
    fn_ptr!(fn DrawBuffers(n: GLsizei, bufs: *GLenum))
    fn_ptr!(fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void))
    fn_ptr!(fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, basevertex: GLint))
    fn_ptr!(fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei))
    fn_ptr!(fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, basevertex: GLint))
    fn_ptr!(fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void))
    fn_ptr!(fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void, basevertex: GLint))
    fn_ptr!(fn Enable(cap: GLenum))
    fn_ptr!(fn EnableVertexAttribArray(index: GLuint))
    fn_ptr!(fn Enablei(target: GLenum, index: GLuint))
    fn_ptr!(fn EndConditionalRender())
    fn_ptr!(fn EndQuery(target: GLenum))
    fn_ptr!(fn EndTransformFeedback())
    fn_ptr!(fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync)
    fn_ptr!(fn Finish())
    fn_ptr!(fn Flush())
    fn_ptr!(fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr))
    fn_ptr!(fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint))
    fn_ptr!(fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint))
    fn_ptr!(fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint))
    fn_ptr!(fn FrontFace(mode: GLenum))
    fn_ptr!(fn GenBuffers(n: GLsizei, buffers: *mut GLuint))
    fn_ptr!(fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint))
    fn_ptr!(fn GenQueries(n: GLsizei, ids: *mut GLuint))
    fn_ptr!(fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint))
    fn_ptr!(fn GenTextures(n: GLsizei, textures: *mut GLuint))
    fn_ptr!(fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint))
    fn_ptr!(fn GenerateMipmap(target: GLenum))
    fn_ptr!(fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar))
    fn_ptr!(fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar))
    fn_ptr!(fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint))
    fn_ptr!(fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean))
    fn_ptr!(fn GetBooleanv(pname: GLenum, data: *mut GLboolean))
    fn_ptr!(fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64))
    fn_ptr!(fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut c_void))
    fn_ptr!(fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void))
    fn_ptr!(fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void))
    fn_ptr!(fn GetDoublev(pname: GLenum, data: *mut GLdouble))
    fn_ptr!(fn GetError() -> GLenum)
    fn_ptr!(fn GetFloatv(pname: GLenum, data: *mut GLfloat))
    fn_ptr!(fn GetFragDataLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64))
    fn_ptr!(fn GetInteger64v(pname: GLenum, data: *mut GLint64))
    fn_ptr!(fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint))
    fn_ptr!(fn GetIntegerv(pname: GLenum, data: *mut GLint))
    fn_ptr!(fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat))
    fn_ptr!(fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar))
    fn_ptr!(fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetString(name: GLenum) -> *GLubyte)
    fn_ptr!(fn GetStringi(name: GLenum, index: GLuint) -> *GLubyte)
    fn_ptr!(fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint))
    fn_ptr!(fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void))
    fn_ptr!(fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *GLchar) -> GLuint)
    fn_ptr!(fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: **GLchar, uniformIndices: *mut GLuint))
    fn_ptr!(fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat))
    fn_ptr!(fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint))
    fn_ptr!(fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint))
    fn_ptr!(fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut c_void))
    fn_ptr!(fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn Hint(target: GLenum, mode: GLenum))
    fn_ptr!(fn IsBuffer(buffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsEnabled(cap: GLenum) -> GLboolean)
    fn_ptr!(fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean)
    fn_ptr!(fn IsFramebuffer(framebuffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsProgram(program: GLuint) -> GLboolean)
    fn_ptr!(fn IsQuery(id: GLuint) -> GLboolean)
    fn_ptr!(fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsShader(shader: GLuint) -> GLboolean)
    fn_ptr!(fn IsSync(sync: GLsync) -> GLboolean)
    fn_ptr!(fn IsTexture(texture: GLuint) -> GLboolean)
    fn_ptr!(fn IsVertexArray(array: GLuint) -> GLboolean)
    fn_ptr!(fn LineWidth(width: GLfloat))
    fn_ptr!(fn LinkProgram(program: GLuint))
    fn_ptr!(fn LogicOp(opcode: GLenum))
    fn_ptr!(fn MapBuffer(target: GLenum, access: GLenum) -> *c_void)
    fn_ptr!(fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *c_void)
    fn_ptr!(fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei))
    fn_ptr!(fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei))
    fn_ptr!(fn MultiDrawElementsBaseVertex(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei, basevertex: *GLint))
    fn_ptr!(fn PixelStoref(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PixelStorei(pname: GLenum, param: GLint))
    fn_ptr!(fn PointParameterf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PointParameterfv(pname: GLenum, params: *GLfloat))
    fn_ptr!(fn PointParameteri(pname: GLenum, param: GLint))
    fn_ptr!(fn PointParameteriv(pname: GLenum, params: *GLint))
    fn_ptr!(fn PointSize(size: GLfloat))
    fn_ptr!(fn PolygonMode(face: GLenum, mode: GLenum))
    fn_ptr!(fn PolygonOffset(factor: GLfloat, units: GLfloat))
    fn_ptr!(fn PrimitiveRestartIndex(index: GLuint))
    fn_ptr!(fn ProvokingVertex(mode: GLenum))
    fn_ptr!(fn ReadBuffer(mode: GLenum))
    fn_ptr!(fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void))
    fn_ptr!(fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei))
    fn_ptr!(fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei))
    fn_ptr!(fn SampleCoverage(value: GLfloat, invert: GLboolean))
    fn_ptr!(fn SampleMaski(index: GLuint, mask: GLbitfield))
    fn_ptr!(fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint))
    fn_ptr!(fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilMask(mask: GLuint))
    fn_ptr!(fn StencilMaskSeparate(face: GLenum, mask: GLuint))
    fn_ptr!(fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum))
    fn_ptr!(fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum))
    fn_ptr!(fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint))
    fn_ptr!(fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexParameterIiv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *GLuint))
    fn_ptr!(fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn TexParameteri(target: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: **GLchar, bufferMode: GLenum))
    fn_ptr!(fn Uniform1f(location: GLint, v0: GLfloat))
    fn_ptr!(fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform1i(location: GLint, v0: GLint))
    fn_ptr!(fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform1ui(location: GLint, v0: GLuint))
    fn_ptr!(fn Uniform1uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat))
    fn_ptr!(fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform2i(location: GLint, v0: GLint, v1: GLint))
    fn_ptr!(fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint))
    fn_ptr!(fn Uniform2uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat))
    fn_ptr!(fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint))
    fn_ptr!(fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint))
    fn_ptr!(fn Uniform3uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat))
    fn_ptr!(fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint))
    fn_ptr!(fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint))
    fn_ptr!(fn Uniform4uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint))
    fn_ptr!(fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UnmapBuffer(target: GLenum) -> GLboolean)
    fn_ptr!(fn UseProgram(program: GLuint))
    fn_ptr!(fn ValidateProgram(program: GLuint))
    fn_ptr!(fn VertexAttrib1d(index: GLuint, x: GLdouble))
    fn_ptr!(fn VertexAttrib1dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib1f(index: GLuint, x: GLfloat))
    fn_ptr!(fn VertexAttrib1fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib1s(index: GLuint, x: GLshort))
    fn_ptr!(fn VertexAttrib1sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble))
    fn_ptr!(fn VertexAttrib2dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat))
    fn_ptr!(fn VertexAttrib2fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort))
    fn_ptr!(fn VertexAttrib2sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn VertexAttrib3dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn VertexAttrib3fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort))
    fn_ptr!(fn VertexAttrib3sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4Nbv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttrib4Niv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttrib4Nsv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte))
    fn_ptr!(fn VertexAttrib4Nubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttrib4Nuiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttrib4Nusv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttrib4bv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn VertexAttrib4dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat))
    fn_ptr!(fn VertexAttrib4fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib4iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort))
    fn_ptr!(fn VertexAttrib4sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4ubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttrib4uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttrib4usv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttribI1i(index: GLuint, x: GLint))
    fn_ptr!(fn VertexAttribI1iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI1ui(index: GLuint, x: GLuint))
    fn_ptr!(fn VertexAttribI1uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint))
    fn_ptr!(fn VertexAttribI2iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint))
    fn_ptr!(fn VertexAttribI2uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint))
    fn_ptr!(fn VertexAttribI3iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint))
    fn_ptr!(fn VertexAttribI3uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI4bv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint))
    fn_ptr!(fn VertexAttribI4iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI4sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttribI4ubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint))
    fn_ptr!(fn VertexAttribI4uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI4usv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64))
}

macro_rules! fn_mod(
    ($name:ident, $sym:expr) => (
        pub mod $name {
            #[inline]
            pub fn is_loaded() -> bool { unsafe { ::storage::$name.is_loaded } }
            
            #[inline]
            pub fn load_with(loadfn: |symbol: &str| -> Option<extern "system" fn()>) {
                unsafe { ::storage::$name = ::FnPtr::new(loadfn($sym), ::failing::$name) }
            }
        }
    )
)

fn_mod!(ActiveTexture, "glActiveTexture")
fn_mod!(AttachShader, "glAttachShader")
fn_mod!(BeginConditionalRender, "glBeginConditionalRender")
fn_mod!(BeginQuery, "glBeginQuery")
fn_mod!(BeginTransformFeedback, "glBeginTransformFeedback")
fn_mod!(BindAttribLocation, "glBindAttribLocation")
fn_mod!(BindBuffer, "glBindBuffer")
fn_mod!(BindBufferBase, "glBindBufferBase")
fn_mod!(BindBufferRange, "glBindBufferRange")
fn_mod!(BindFragDataLocation, "glBindFragDataLocation")
fn_mod!(BindFramebuffer, "glBindFramebuffer")
fn_mod!(BindRenderbuffer, "glBindRenderbuffer")
fn_mod!(BindTexture, "glBindTexture")
fn_mod!(BindVertexArray, "glBindVertexArray")
fn_mod!(BlendColor, "glBlendColor")
fn_mod!(BlendEquation, "glBlendEquation")
fn_mod!(BlendEquationSeparate, "glBlendEquationSeparate")
fn_mod!(BlendFunc, "glBlendFunc")
fn_mod!(BlendFuncSeparate, "glBlendFuncSeparate")
fn_mod!(BlitFramebuffer, "glBlitFramebuffer")
fn_mod!(BufferData, "glBufferData")
fn_mod!(BufferSubData, "glBufferSubData")
fn_mod!(CheckFramebufferStatus, "glCheckFramebufferStatus")
fn_mod!(ClampColor, "glClampColor")
fn_mod!(Clear, "glClear")
fn_mod!(ClearBufferfi, "glClearBufferfi")
fn_mod!(ClearBufferfv, "glClearBufferfv")
fn_mod!(ClearBufferiv, "glClearBufferiv")
fn_mod!(ClearBufferuiv, "glClearBufferuiv")
fn_mod!(ClearColor, "glClearColor")
fn_mod!(ClearDepth, "glClearDepth")
fn_mod!(ClearStencil, "glClearStencil")
fn_mod!(ClientWaitSync, "glClientWaitSync")
fn_mod!(ColorMask, "glColorMask")
fn_mod!(ColorMaski, "glColorMaski")
fn_mod!(CompileShader, "glCompileShader")
fn_mod!(CompressedTexImage1D, "glCompressedTexImage1D")
fn_mod!(CompressedTexImage2D, "glCompressedTexImage2D")
fn_mod!(CompressedTexImage3D, "glCompressedTexImage3D")
fn_mod!(CompressedTexSubImage1D, "glCompressedTexSubImage1D")
fn_mod!(CompressedTexSubImage2D, "glCompressedTexSubImage2D")
fn_mod!(CompressedTexSubImage3D, "glCompressedTexSubImage3D")
fn_mod!(CopyBufferSubData, "glCopyBufferSubData")
fn_mod!(CopyTexImage1D, "glCopyTexImage1D")
fn_mod!(CopyTexImage2D, "glCopyTexImage2D")
fn_mod!(CopyTexSubImage1D, "glCopyTexSubImage1D")
fn_mod!(CopyTexSubImage2D, "glCopyTexSubImage2D")
fn_mod!(CopyTexSubImage3D, "glCopyTexSubImage3D")
fn_mod!(CreateProgram, "glCreateProgram")
fn_mod!(CreateShader, "glCreateShader")
fn_mod!(CullFace, "glCullFace")
fn_mod!(DeleteBuffers, "glDeleteBuffers")
fn_mod!(DeleteFramebuffers, "glDeleteFramebuffers")
fn_mod!(DeleteProgram, "glDeleteProgram")
fn_mod!(DeleteQueries, "glDeleteQueries")
fn_mod!(DeleteRenderbuffers, "glDeleteRenderbuffers")
fn_mod!(DeleteShader, "glDeleteShader")
fn_mod!(DeleteSync, "glDeleteSync")
fn_mod!(DeleteTextures, "glDeleteTextures")
fn_mod!(DeleteVertexArrays, "glDeleteVertexArrays")
fn_mod!(DepthFunc, "glDepthFunc")
fn_mod!(DepthMask, "glDepthMask")
fn_mod!(DepthRange, "glDepthRange")
fn_mod!(DetachShader, "glDetachShader")
fn_mod!(Disable, "glDisable")
fn_mod!(DisableVertexAttribArray, "glDisableVertexAttribArray")
fn_mod!(Disablei, "glDisablei")
fn_mod!(DrawArrays, "glDrawArrays")
fn_mod!(DrawArraysInstanced, "glDrawArraysInstanced")
fn_mod!(DrawBuffer, "glDrawBuffer")
fn_mod!(DrawBuffers, "glDrawBuffers")
fn_mod!(DrawElements, "glDrawElements")
fn_mod!(DrawElementsBaseVertex, "glDrawElementsBaseVertex")
fn_mod!(DrawElementsInstanced, "glDrawElementsInstanced")
fn_mod!(DrawElementsInstancedBaseVertex, "glDrawElementsInstancedBaseVertex")
fn_mod!(DrawRangeElements, "glDrawRangeElements")
fn_mod!(DrawRangeElementsBaseVertex, "glDrawRangeElementsBaseVertex")
fn_mod!(Enable, "glEnable")
fn_mod!(EnableVertexAttribArray, "glEnableVertexAttribArray")
fn_mod!(Enablei, "glEnablei")
fn_mod!(EndConditionalRender, "glEndConditionalRender")
fn_mod!(EndQuery, "glEndQuery")
fn_mod!(EndTransformFeedback, "glEndTransformFeedback")
fn_mod!(FenceSync, "glFenceSync")
fn_mod!(Finish, "glFinish")
fn_mod!(Flush, "glFlush")
fn_mod!(FlushMappedBufferRange, "glFlushMappedBufferRange")
fn_mod!(FramebufferRenderbuffer, "glFramebufferRenderbuffer")
fn_mod!(FramebufferTexture, "glFramebufferTexture")
fn_mod!(FramebufferTexture1D, "glFramebufferTexture1D")
fn_mod!(FramebufferTexture2D, "glFramebufferTexture2D")
fn_mod!(FramebufferTexture3D, "glFramebufferTexture3D")
fn_mod!(FramebufferTextureLayer, "glFramebufferTextureLayer")
fn_mod!(FrontFace, "glFrontFace")
fn_mod!(GenBuffers, "glGenBuffers")
fn_mod!(GenFramebuffers, "glGenFramebuffers")
fn_mod!(GenQueries, "glGenQueries")
fn_mod!(GenRenderbuffers, "glGenRenderbuffers")
fn_mod!(GenTextures, "glGenTextures")
fn_mod!(GenVertexArrays, "glGenVertexArrays")
fn_mod!(GenerateMipmap, "glGenerateMipmap")
fn_mod!(GetActiveAttrib, "glGetActiveAttrib")
fn_mod!(GetActiveUniform, "glGetActiveUniform")
fn_mod!(GetActiveUniformBlockName, "glGetActiveUniformBlockName")
fn_mod!(GetActiveUniformBlockiv, "glGetActiveUniformBlockiv")
fn_mod!(GetActiveUniformName, "glGetActiveUniformName")
fn_mod!(GetActiveUniformsiv, "glGetActiveUniformsiv")
fn_mod!(GetAttachedShaders, "glGetAttachedShaders")
fn_mod!(GetAttribLocation, "glGetAttribLocation")
fn_mod!(GetBooleani_v, "glGetBooleani_v")
fn_mod!(GetBooleanv, "glGetBooleanv")
fn_mod!(GetBufferParameteri64v, "glGetBufferParameteri64v")
fn_mod!(GetBufferParameteriv, "glGetBufferParameteriv")
fn_mod!(GetBufferPointerv, "glGetBufferPointerv")
fn_mod!(GetBufferSubData, "glGetBufferSubData")
fn_mod!(GetCompressedTexImage, "glGetCompressedTexImage")
fn_mod!(GetDoublev, "glGetDoublev")
fn_mod!(GetError, "glGetError")
fn_mod!(GetFloatv, "glGetFloatv")
fn_mod!(GetFragDataLocation, "glGetFragDataLocation")
fn_mod!(GetFramebufferAttachmentParameteriv, "glGetFramebufferAttachmentParameteriv")
fn_mod!(GetInteger64i_v, "glGetInteger64i_v")
fn_mod!(GetInteger64v, "glGetInteger64v")
fn_mod!(GetIntegeri_v, "glGetIntegeri_v")
fn_mod!(GetIntegerv, "glGetIntegerv")
fn_mod!(GetMultisamplefv, "glGetMultisamplefv")
fn_mod!(GetProgramInfoLog, "glGetProgramInfoLog")
fn_mod!(GetProgramiv, "glGetProgramiv")
fn_mod!(GetQueryObjectiv, "glGetQueryObjectiv")
fn_mod!(GetQueryObjectuiv, "glGetQueryObjectuiv")
fn_mod!(GetQueryiv, "glGetQueryiv")
fn_mod!(GetRenderbufferParameteriv, "glGetRenderbufferParameteriv")
fn_mod!(GetShaderInfoLog, "glGetShaderInfoLog")
fn_mod!(GetShaderSource, "glGetShaderSource")
fn_mod!(GetShaderiv, "glGetShaderiv")
fn_mod!(GetString, "glGetString")
fn_mod!(GetStringi, "glGetStringi")
fn_mod!(GetSynciv, "glGetSynciv")
fn_mod!(GetTexImage, "glGetTexImage")
fn_mod!(GetTexLevelParameterfv, "glGetTexLevelParameterfv")
fn_mod!(GetTexLevelParameteriv, "glGetTexLevelParameteriv")
fn_mod!(GetTexParameterIiv, "glGetTexParameterIiv")
fn_mod!(GetTexParameterIuiv, "glGetTexParameterIuiv")
fn_mod!(GetTexParameterfv, "glGetTexParameterfv")
fn_mod!(GetTexParameteriv, "glGetTexParameteriv")
fn_mod!(GetTransformFeedbackVarying, "glGetTransformFeedbackVarying")
fn_mod!(GetUniformBlockIndex, "glGetUniformBlockIndex")
fn_mod!(GetUniformIndices, "glGetUniformIndices")
fn_mod!(GetUniformLocation, "glGetUniformLocation")
fn_mod!(GetUniformfv, "glGetUniformfv")
fn_mod!(GetUniformiv, "glGetUniformiv")
fn_mod!(GetUniformuiv, "glGetUniformuiv")
fn_mod!(GetVertexAttribIiv, "glGetVertexAttribIiv")
fn_mod!(GetVertexAttribIuiv, "glGetVertexAttribIuiv")
fn_mod!(GetVertexAttribPointerv, "glGetVertexAttribPointerv")
fn_mod!(GetVertexAttribdv, "glGetVertexAttribdv")
fn_mod!(GetVertexAttribfv, "glGetVertexAttribfv")
fn_mod!(GetVertexAttribiv, "glGetVertexAttribiv")
fn_mod!(Hint, "glHint")
fn_mod!(IsBuffer, "glIsBuffer")
fn_mod!(IsEnabled, "glIsEnabled")
fn_mod!(IsEnabledi, "glIsEnabledi")
fn_mod!(IsFramebuffer, "glIsFramebuffer")
fn_mod!(IsProgram, "glIsProgram")
fn_mod!(IsQuery, "glIsQuery")
fn_mod!(IsRenderbuffer, "glIsRenderbuffer")
fn_mod!(IsShader, "glIsShader")
fn_mod!(IsSync, "glIsSync")
fn_mod!(IsTexture, "glIsTexture")
fn_mod!(IsVertexArray, "glIsVertexArray")
fn_mod!(LineWidth, "glLineWidth")
fn_mod!(LinkProgram, "glLinkProgram")
fn_mod!(LogicOp, "glLogicOp")
fn_mod!(MapBuffer, "glMapBuffer")
fn_mod!(MapBufferRange, "glMapBufferRange")
fn_mod!(MultiDrawArrays, "glMultiDrawArrays")
fn_mod!(MultiDrawElements, "glMultiDrawElements")
fn_mod!(MultiDrawElementsBaseVertex, "glMultiDrawElementsBaseVertex")
fn_mod!(PixelStoref, "glPixelStoref")
fn_mod!(PixelStorei, "glPixelStorei")
fn_mod!(PointParameterf, "glPointParameterf")
fn_mod!(PointParameterfv, "glPointParameterfv")
fn_mod!(PointParameteri, "glPointParameteri")
fn_mod!(PointParameteriv, "glPointParameteriv")
fn_mod!(PointSize, "glPointSize")
fn_mod!(PolygonMode, "glPolygonMode")
fn_mod!(PolygonOffset, "glPolygonOffset")
fn_mod!(PrimitiveRestartIndex, "glPrimitiveRestartIndex")
fn_mod!(ProvokingVertex, "glProvokingVertex")
fn_mod!(ReadBuffer, "glReadBuffer")
fn_mod!(ReadPixels, "glReadPixels")
fn_mod!(RenderbufferStorage, "glRenderbufferStorage")
fn_mod!(RenderbufferStorageMultisample, "glRenderbufferStorageMultisample")
fn_mod!(SampleCoverage, "glSampleCoverage")
fn_mod!(SampleMaski, "glSampleMaski")
fn_mod!(Scissor, "glScissor")
fn_mod!(ShaderSource, "glShaderSource")
fn_mod!(StencilFunc, "glStencilFunc")
fn_mod!(StencilFuncSeparate, "glStencilFuncSeparate")
fn_mod!(StencilMask, "glStencilMask")
fn_mod!(StencilMaskSeparate, "glStencilMaskSeparate")
fn_mod!(StencilOp, "glStencilOp")
fn_mod!(StencilOpSeparate, "glStencilOpSeparate")
fn_mod!(TexBuffer, "glTexBuffer")
fn_mod!(TexImage1D, "glTexImage1D")
fn_mod!(TexImage2D, "glTexImage2D")
fn_mod!(TexImage2DMultisample, "glTexImage2DMultisample")
fn_mod!(TexImage3D, "glTexImage3D")
fn_mod!(TexImage3DMultisample, "glTexImage3DMultisample")
fn_mod!(TexParameterIiv, "glTexParameterIiv")
fn_mod!(TexParameterIuiv, "glTexParameterIuiv")
fn_mod!(TexParameterf, "glTexParameterf")
fn_mod!(TexParameterfv, "glTexParameterfv")
fn_mod!(TexParameteri, "glTexParameteri")
fn_mod!(TexParameteriv, "glTexParameteriv")
fn_mod!(TexSubImage1D, "glTexSubImage1D")
fn_mod!(TexSubImage2D, "glTexSubImage2D")
fn_mod!(TexSubImage3D, "glTexSubImage3D")
fn_mod!(TransformFeedbackVaryings, "glTransformFeedbackVaryings")
fn_mod!(Uniform1f, "glUniform1f")
fn_mod!(Uniform1fv, "glUniform1fv")
fn_mod!(Uniform1i, "glUniform1i")
fn_mod!(Uniform1iv, "glUniform1iv")
fn_mod!(Uniform1ui, "glUniform1ui")
fn_mod!(Uniform1uiv, "glUniform1uiv")
fn_mod!(Uniform2f, "glUniform2f")
fn_mod!(Uniform2fv, "glUniform2fv")
fn_mod!(Uniform2i, "glUniform2i")
fn_mod!(Uniform2iv, "glUniform2iv")
fn_mod!(Uniform2ui, "glUniform2ui")
fn_mod!(Uniform2uiv, "glUniform2uiv")
fn_mod!(Uniform3f, "glUniform3f")
fn_mod!(Uniform3fv, "glUniform3fv")
fn_mod!(Uniform3i, "glUniform3i")
fn_mod!(Uniform3iv, "glUniform3iv")
fn_mod!(Uniform3ui, "glUniform3ui")
fn_mod!(Uniform3uiv, "glUniform3uiv")
fn_mod!(Uniform4f, "glUniform4f")
fn_mod!(Uniform4fv, "glUniform4fv")
fn_mod!(Uniform4i, "glUniform4i")
fn_mod!(Uniform4iv, "glUniform4iv")
fn_mod!(Uniform4ui, "glUniform4ui")
fn_mod!(Uniform4uiv, "glUniform4uiv")
fn_mod!(UniformBlockBinding, "glUniformBlockBinding")
fn_mod!(UniformMatrix2fv, "glUniformMatrix2fv")
fn_mod!(UniformMatrix2x3fv, "glUniformMatrix2x3fv")
fn_mod!(UniformMatrix2x4fv, "glUniformMatrix2x4fv")
fn_mod!(UniformMatrix3fv, "glUniformMatrix3fv")
fn_mod!(UniformMatrix3x2fv, "glUniformMatrix3x2fv")
fn_mod!(UniformMatrix3x4fv, "glUniformMatrix3x4fv")
fn_mod!(UniformMatrix4fv, "glUniformMatrix4fv")
fn_mod!(UniformMatrix4x2fv, "glUniformMatrix4x2fv")
fn_mod!(UniformMatrix4x3fv, "glUniformMatrix4x3fv")
fn_mod!(UnmapBuffer, "glUnmapBuffer")
fn_mod!(UseProgram, "glUseProgram")
fn_mod!(ValidateProgram, "glValidateProgram")
fn_mod!(VertexAttrib1d, "glVertexAttrib1d")
fn_mod!(VertexAttrib1dv, "glVertexAttrib1dv")
fn_mod!(VertexAttrib1f, "glVertexAttrib1f")
fn_mod!(VertexAttrib1fv, "glVertexAttrib1fv")
fn_mod!(VertexAttrib1s, "glVertexAttrib1s")
fn_mod!(VertexAttrib1sv, "glVertexAttrib1sv")
fn_mod!(VertexAttrib2d, "glVertexAttrib2d")
fn_mod!(VertexAttrib2dv, "glVertexAttrib2dv")
fn_mod!(VertexAttrib2f, "glVertexAttrib2f")
fn_mod!(VertexAttrib2fv, "glVertexAttrib2fv")
fn_mod!(VertexAttrib2s, "glVertexAttrib2s")
fn_mod!(VertexAttrib2sv, "glVertexAttrib2sv")
fn_mod!(VertexAttrib3d, "glVertexAttrib3d")
fn_mod!(VertexAttrib3dv, "glVertexAttrib3dv")
fn_mod!(VertexAttrib3f, "glVertexAttrib3f")
fn_mod!(VertexAttrib3fv, "glVertexAttrib3fv")
fn_mod!(VertexAttrib3s, "glVertexAttrib3s")
fn_mod!(VertexAttrib3sv, "glVertexAttrib3sv")
fn_mod!(VertexAttrib4Nbv, "glVertexAttrib4Nbv")
fn_mod!(VertexAttrib4Niv, "glVertexAttrib4Niv")
fn_mod!(VertexAttrib4Nsv, "glVertexAttrib4Nsv")
fn_mod!(VertexAttrib4Nub, "glVertexAttrib4Nub")
fn_mod!(VertexAttrib4Nubv, "glVertexAttrib4Nubv")
fn_mod!(VertexAttrib4Nuiv, "glVertexAttrib4Nuiv")
fn_mod!(VertexAttrib4Nusv, "glVertexAttrib4Nusv")
fn_mod!(VertexAttrib4bv, "glVertexAttrib4bv")
fn_mod!(VertexAttrib4d, "glVertexAttrib4d")
fn_mod!(VertexAttrib4dv, "glVertexAttrib4dv")
fn_mod!(VertexAttrib4f, "glVertexAttrib4f")
fn_mod!(VertexAttrib4fv, "glVertexAttrib4fv")
fn_mod!(VertexAttrib4iv, "glVertexAttrib4iv")
fn_mod!(VertexAttrib4s, "glVertexAttrib4s")
fn_mod!(VertexAttrib4sv, "glVertexAttrib4sv")
fn_mod!(VertexAttrib4ubv, "glVertexAttrib4ubv")
fn_mod!(VertexAttrib4uiv, "glVertexAttrib4uiv")
fn_mod!(VertexAttrib4usv, "glVertexAttrib4usv")
fn_mod!(VertexAttribI1i, "glVertexAttribI1i")
fn_mod!(VertexAttribI1iv, "glVertexAttribI1iv")
fn_mod!(VertexAttribI1ui, "glVertexAttribI1ui")
fn_mod!(VertexAttribI1uiv, "glVertexAttribI1uiv")
fn_mod!(VertexAttribI2i, "glVertexAttribI2i")
fn_mod!(VertexAttribI2iv, "glVertexAttribI2iv")
fn_mod!(VertexAttribI2ui, "glVertexAttribI2ui")
fn_mod!(VertexAttribI2uiv, "glVertexAttribI2uiv")
fn_mod!(VertexAttribI3i, "glVertexAttribI3i")
fn_mod!(VertexAttribI3iv, "glVertexAttribI3iv")
fn_mod!(VertexAttribI3ui, "glVertexAttribI3ui")
fn_mod!(VertexAttribI3uiv, "glVertexAttribI3uiv")
fn_mod!(VertexAttribI4bv, "glVertexAttribI4bv")
fn_mod!(VertexAttribI4i, "glVertexAttribI4i")
fn_mod!(VertexAttribI4iv, "glVertexAttribI4iv")
fn_mod!(VertexAttribI4sv, "glVertexAttribI4sv")
fn_mod!(VertexAttribI4ubv, "glVertexAttribI4ubv")
fn_mod!(VertexAttribI4ui, "glVertexAttribI4ui")
fn_mod!(VertexAttribI4uiv, "glVertexAttribI4uiv")
fn_mod!(VertexAttribI4usv, "glVertexAttribI4usv")
fn_mod!(VertexAttribIPointer, "glVertexAttribIPointer")
fn_mod!(VertexAttribPointer, "glVertexAttribPointer")
fn_mod!(Viewport, "glViewport")
fn_mod!(WaitSync, "glWaitSync")

mod failing {
    use libc::*;
    use super::types::*;
    
    macro_rules! failing(
        (fn $name:ident()) => (pub extern "system" fn $name() { fail!(stringify!($name was not loaded)) });
        (fn $name:ident() -> $ret_ty:ty) => (pub extern "system" fn $name() -> $ret_ty { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*)) => (pub extern "system" fn $name($(_: $arg_ty),*) { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*) -> $ret_ty:ty) => (pub extern "system" fn $name($(_: $arg_ty),*) -> $ret_ty { fail!(stringify!($name was not loaded)) });
    )
    
    failing!(fn ActiveTexture(GLenum))
    failing!(fn AttachShader(GLuint, GLuint))
    failing!(fn BeginConditionalRender(GLuint, GLenum))
    failing!(fn BeginQuery(GLenum, GLuint))
    failing!(fn BeginTransformFeedback(GLenum))
    failing!(fn BindAttribLocation(GLuint, GLuint, *GLchar))
    failing!(fn BindBuffer(GLenum, GLuint))
    failing!(fn BindBufferBase(GLenum, GLuint, GLuint))
    failing!(fn BindBufferRange(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr))
    failing!(fn BindFragDataLocation(GLuint, GLuint, *GLchar))
    failing!(fn BindFramebuffer(GLenum, GLuint))
    failing!(fn BindRenderbuffer(GLenum, GLuint))
    failing!(fn BindTexture(GLenum, GLuint))
    failing!(fn BindVertexArray(GLuint))
    failing!(fn BlendColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn BlendEquation(GLenum))
    failing!(fn BlendEquationSeparate(GLenum, GLenum))
    failing!(fn BlendFunc(GLenum, GLenum))
    failing!(fn BlendFuncSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn BlitFramebuffer(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum))
    failing!(fn BufferData(GLenum, GLsizeiptr, *c_void, GLenum))
    failing!(fn BufferSubData(GLenum, GLintptr, GLsizeiptr, *c_void))
    failing!(fn CheckFramebufferStatus(GLenum) -> GLenum)
    failing!(fn ClampColor(GLenum, GLenum))
    failing!(fn Clear(GLbitfield))
    failing!(fn ClearBufferfi(GLenum, GLint, GLfloat, GLint))
    failing!(fn ClearBufferfv(GLenum, GLint, *GLfloat))
    failing!(fn ClearBufferiv(GLenum, GLint, *GLint))
    failing!(fn ClearBufferuiv(GLenum, GLint, *GLuint))
    failing!(fn ClearColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ClearDepth(GLdouble))
    failing!(fn ClearStencil(GLint))
    failing!(fn ClientWaitSync(GLsync, GLbitfield, GLuint64) -> GLenum)
    failing!(fn ColorMask(GLboolean, GLboolean, GLboolean, GLboolean))
    failing!(fn ColorMaski(GLuint, GLboolean, GLboolean, GLboolean, GLboolean))
    failing!(fn CompileShader(GLuint))
    failing!(fn CompressedTexImage1D(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexImage2D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexImage3D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CopyBufferSubData(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr))
    failing!(fn CopyTexImage1D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint))
    failing!(fn CopyTexImage2D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint))
    failing!(fn CopyTexSubImage1D(GLenum, GLint, GLint, GLint, GLint, GLsizei))
    failing!(fn CopyTexSubImage2D(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CopyTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CreateProgram() -> GLuint)
    failing!(fn CreateShader(GLenum) -> GLuint)
    failing!(fn CullFace(GLenum))
    failing!(fn DeleteBuffers(GLsizei, *GLuint))
    failing!(fn DeleteFramebuffers(GLsizei, *GLuint))
    failing!(fn DeleteProgram(GLuint))
    failing!(fn DeleteQueries(GLsizei, *GLuint))
    failing!(fn DeleteRenderbuffers(GLsizei, *GLuint))
    failing!(fn DeleteShader(GLuint))
    failing!(fn DeleteSync(GLsync))
    failing!(fn DeleteTextures(GLsizei, *GLuint))
    failing!(fn DeleteVertexArrays(GLsizei, *GLuint))
    failing!(fn DepthFunc(GLenum))
    failing!(fn DepthMask(GLboolean))
    failing!(fn DepthRange(GLdouble, GLdouble))
    failing!(fn DetachShader(GLuint, GLuint))
    failing!(fn Disable(GLenum))
    failing!(fn DisableVertexAttribArray(GLuint))
    failing!(fn Disablei(GLenum, GLuint))
    failing!(fn DrawArrays(GLenum, GLint, GLsizei))
    failing!(fn DrawArraysInstanced(GLenum, GLint, GLsizei, GLsizei))
    failing!(fn DrawBuffer(GLenum))
    failing!(fn DrawBuffers(GLsizei, *GLenum))
    failing!(fn DrawElements(GLenum, GLsizei, GLenum, *c_void))
    failing!(fn DrawElementsBaseVertex(GLenum, GLsizei, GLenum, *c_void, GLint))
    failing!(fn DrawElementsInstanced(GLenum, GLsizei, GLenum, *c_void, GLsizei))
    failing!(fn DrawElementsInstancedBaseVertex(GLenum, GLsizei, GLenum, *c_void, GLsizei, GLint))
    failing!(fn DrawRangeElements(GLenum, GLuint, GLuint, GLsizei, GLenum, *c_void))
    failing!(fn DrawRangeElementsBaseVertex(GLenum, GLuint, GLuint, GLsizei, GLenum, *c_void, GLint))
    failing!(fn Enable(GLenum))
    failing!(fn EnableVertexAttribArray(GLuint))
    failing!(fn Enablei(GLenum, GLuint))
    failing!(fn EndConditionalRender())
    failing!(fn EndQuery(GLenum))
    failing!(fn EndTransformFeedback())
    failing!(fn FenceSync(GLenum, GLbitfield) -> GLsync)
    failing!(fn Finish())
    failing!(fn Flush())
    failing!(fn FlushMappedBufferRange(GLenum, GLintptr, GLsizeiptr))
    failing!(fn FramebufferRenderbuffer(GLenum, GLenum, GLenum, GLuint))
    failing!(fn FramebufferTexture(GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture1D(GLenum, GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture2D(GLenum, GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture3D(GLenum, GLenum, GLenum, GLuint, GLint, GLint))
    failing!(fn FramebufferTextureLayer(GLenum, GLenum, GLuint, GLint, GLint))
    failing!(fn FrontFace(GLenum))
    failing!(fn GenBuffers(GLsizei, *mut GLuint))
    failing!(fn GenFramebuffers(GLsizei, *mut GLuint))
    failing!(fn GenQueries(GLsizei, *mut GLuint))
    failing!(fn GenRenderbuffers(GLsizei, *mut GLuint))
    failing!(fn GenTextures(GLsizei, *mut GLuint))
    failing!(fn GenVertexArrays(GLsizei, *mut GLuint))
    failing!(fn GenerateMipmap(GLenum))
    failing!(fn GetActiveAttrib(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetActiveUniform(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetActiveUniformBlockName(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveUniformBlockiv(GLuint, GLuint, GLenum, *mut GLint))
    failing!(fn GetActiveUniformName(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveUniformsiv(GLuint, GLsizei, *GLuint, GLenum, *mut GLint))
    failing!(fn GetAttachedShaders(GLuint, GLsizei, *mut GLsizei, *mut GLuint))
    failing!(fn GetAttribLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetBooleani_v(GLenum, GLuint, *mut GLboolean))
    failing!(fn GetBooleanv(GLenum, *mut GLboolean))
    failing!(fn GetBufferParameteri64v(GLenum, GLenum, *mut GLint64))
    failing!(fn GetBufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetBufferPointerv(GLenum, GLenum, **mut c_void))
    failing!(fn GetBufferSubData(GLenum, GLintptr, GLsizeiptr, *mut c_void))
    failing!(fn GetCompressedTexImage(GLenum, GLint, *mut c_void))
    failing!(fn GetDoublev(GLenum, *mut GLdouble))
    failing!(fn GetError() -> GLenum)
    failing!(fn GetFloatv(GLenum, *mut GLfloat))
    failing!(fn GetFragDataLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetFramebufferAttachmentParameteriv(GLenum, GLenum, GLenum, *mut GLint))
    failing!(fn GetInteger64i_v(GLenum, GLuint, *mut GLint64))
    failing!(fn GetInteger64v(GLenum, *mut GLint64))
    failing!(fn GetIntegeri_v(GLenum, GLuint, *mut GLint))
    failing!(fn GetIntegerv(GLenum, *mut GLint))
    failing!(fn GetMultisamplefv(GLenum, GLuint, *mut GLfloat))
    failing!(fn GetProgramInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjectiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjectuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetQueryiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetRenderbufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetShaderInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderSource(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetString(GLenum) -> *GLubyte)
    failing!(fn GetStringi(GLenum, GLuint) -> *GLubyte)
    failing!(fn GetSynciv(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint))
    failing!(fn GetTexImage(GLenum, GLint, GLenum, GLenum, *mut c_void))
    failing!(fn GetTexLevelParameterfv(GLenum, GLint, GLenum, *mut GLfloat))
    failing!(fn GetTexLevelParameteriv(GLenum, GLint, GLenum, *mut GLint))
    failing!(fn GetTexParameterIiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTexParameterIuiv(GLenum, GLenum, *mut GLuint))
    failing!(fn GetTexParameterfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetTexParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTransformFeedbackVarying(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar))
    failing!(fn GetUniformBlockIndex(GLuint, *GLchar) -> GLuint)
    failing!(fn GetUniformIndices(GLuint, GLsizei, **GLchar, *mut GLuint))
    failing!(fn GetUniformLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetUniformfv(GLuint, GLint, *mut GLfloat))
    failing!(fn GetUniformiv(GLuint, GLint, *mut GLint))
    failing!(fn GetUniformuiv(GLuint, GLint, *mut GLuint))
    failing!(fn GetVertexAttribIiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetVertexAttribIuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetVertexAttribPointerv(GLuint, GLenum, **mut c_void))
    failing!(fn GetVertexAttribdv(GLuint, GLenum, *mut GLdouble))
    failing!(fn GetVertexAttribfv(GLuint, GLenum, *mut GLfloat))
    failing!(fn GetVertexAttribiv(GLuint, GLenum, *mut GLint))
    failing!(fn Hint(GLenum, GLenum))
    failing!(fn IsBuffer(GLuint) -> GLboolean)
    failing!(fn IsEnabled(GLenum) -> GLboolean)
    failing!(fn IsEnabledi(GLenum, GLuint) -> GLboolean)
    failing!(fn IsFramebuffer(GLuint) -> GLboolean)
    failing!(fn IsProgram(GLuint) -> GLboolean)
    failing!(fn IsQuery(GLuint) -> GLboolean)
    failing!(fn IsRenderbuffer(GLuint) -> GLboolean)
    failing!(fn IsShader(GLuint) -> GLboolean)
    failing!(fn IsSync(GLsync) -> GLboolean)
    failing!(fn IsTexture(GLuint) -> GLboolean)
    failing!(fn IsVertexArray(GLuint) -> GLboolean)
    failing!(fn LineWidth(GLfloat))
    failing!(fn LinkProgram(GLuint))
    failing!(fn LogicOp(GLenum))
    failing!(fn MapBuffer(GLenum, GLenum) -> *c_void)
    failing!(fn MapBufferRange(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *c_void)
    failing!(fn MultiDrawArrays(GLenum, *GLint, *GLsizei, GLsizei))
    failing!(fn MultiDrawElements(GLenum, *GLsizei, GLenum, **c_void, GLsizei))
    failing!(fn MultiDrawElementsBaseVertex(GLenum, *GLsizei, GLenum, **c_void, GLsizei, *GLint))
    failing!(fn PixelStoref(GLenum, GLfloat))
    failing!(fn PixelStorei(GLenum, GLint))
    failing!(fn PointParameterf(GLenum, GLfloat))
    failing!(fn PointParameterfv(GLenum, *GLfloat))
    failing!(fn PointParameteri(GLenum, GLint))
    failing!(fn PointParameteriv(GLenum, *GLint))
    failing!(fn PointSize(GLfloat))
    failing!(fn PolygonMode(GLenum, GLenum))
    failing!(fn PolygonOffset(GLfloat, GLfloat))
    failing!(fn PrimitiveRestartIndex(GLuint))
    failing!(fn ProvokingVertex(GLenum))
    failing!(fn ReadBuffer(GLenum))
    failing!(fn ReadPixels(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void))
    failing!(fn RenderbufferStorage(GLenum, GLenum, GLsizei, GLsizei))
    failing!(fn RenderbufferStorageMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei))
    failing!(fn SampleCoverage(GLfloat, GLboolean))
    failing!(fn SampleMaski(GLuint, GLbitfield))
    failing!(fn Scissor(GLint, GLint, GLsizei, GLsizei))
    failing!(fn ShaderSource(GLuint, GLsizei, **GLchar, *GLint))
    failing!(fn StencilFunc(GLenum, GLint, GLuint))
    failing!(fn StencilFuncSeparate(GLenum, GLenum, GLint, GLuint))
    failing!(fn StencilMask(GLuint))
    failing!(fn StencilMaskSeparate(GLenum, GLuint))
    failing!(fn StencilOp(GLenum, GLenum, GLenum))
    failing!(fn StencilOpSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn TexBuffer(GLenum, GLenum, GLuint))
    failing!(fn TexImage1D(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexImage2D(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexImage2DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean))
    failing!(fn TexImage3D(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexImage3DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean))
    failing!(fn TexParameterIiv(GLenum, GLenum, *GLint))
    failing!(fn TexParameterIuiv(GLenum, GLenum, *GLuint))
    failing!(fn TexParameterf(GLenum, GLenum, GLfloat))
    failing!(fn TexParameterfv(GLenum, GLenum, *GLfloat))
    failing!(fn TexParameteri(GLenum, GLenum, GLint))
    failing!(fn TexParameteriv(GLenum, GLenum, *GLint))
    failing!(fn TexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn TexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn TexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn TransformFeedbackVaryings(GLuint, GLsizei, **GLchar, GLenum))
    failing!(fn Uniform1f(GLint, GLfloat))
    failing!(fn Uniform1fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform1i(GLint, GLint))
    failing!(fn Uniform1iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform1ui(GLint, GLuint))
    failing!(fn Uniform1uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform2f(GLint, GLfloat, GLfloat))
    failing!(fn Uniform2fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform2i(GLint, GLint, GLint))
    failing!(fn Uniform2iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform2ui(GLint, GLuint, GLuint))
    failing!(fn Uniform2uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform3f(GLint, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform3fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform3i(GLint, GLint, GLint, GLint))
    failing!(fn Uniform3iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform3ui(GLint, GLuint, GLuint, GLuint))
    failing!(fn Uniform3uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform4f(GLint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform4fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform4i(GLint, GLint, GLint, GLint, GLint))
    failing!(fn Uniform4iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform4ui(GLint, GLuint, GLuint, GLuint, GLuint))
    failing!(fn Uniform4uiv(GLint, GLsizei, *GLuint))
    failing!(fn UniformBlockBinding(GLuint, GLuint, GLuint))
    failing!(fn UniformMatrix2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix2x3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix2x4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3x2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3x4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4x2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4x3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UnmapBuffer(GLenum) -> GLboolean)
    failing!(fn UseProgram(GLuint))
    failing!(fn ValidateProgram(GLuint))
    failing!(fn VertexAttrib1d(GLuint, GLdouble))
    failing!(fn VertexAttrib1dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib1f(GLuint, GLfloat))
    failing!(fn VertexAttrib1fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib1s(GLuint, GLshort))
    failing!(fn VertexAttrib1sv(GLuint, *GLshort))
    failing!(fn VertexAttrib2d(GLuint, GLdouble, GLdouble))
    failing!(fn VertexAttrib2dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib2f(GLuint, GLfloat, GLfloat))
    failing!(fn VertexAttrib2fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib2s(GLuint, GLshort, GLshort))
    failing!(fn VertexAttrib2sv(GLuint, *GLshort))
    failing!(fn VertexAttrib3d(GLuint, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttrib3dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib3f(GLuint, GLfloat, GLfloat, GLfloat))
    failing!(fn VertexAttrib3fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib3s(GLuint, GLshort, GLshort, GLshort))
    failing!(fn VertexAttrib3sv(GLuint, *GLshort))
    failing!(fn VertexAttrib4Nbv(GLuint, *GLbyte))
    failing!(fn VertexAttrib4Niv(GLuint, *GLint))
    failing!(fn VertexAttrib4Nsv(GLuint, *GLshort))
    failing!(fn VertexAttrib4Nub(GLuint, GLubyte, GLubyte, GLubyte, GLubyte))
    failing!(fn VertexAttrib4Nubv(GLuint, *GLubyte))
    failing!(fn VertexAttrib4Nuiv(GLuint, *GLuint))
    failing!(fn VertexAttrib4Nusv(GLuint, *GLushort))
    failing!(fn VertexAttrib4bv(GLuint, *GLbyte))
    failing!(fn VertexAttrib4d(GLuint, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttrib4dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib4f(GLuint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn VertexAttrib4fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib4iv(GLuint, *GLint))
    failing!(fn VertexAttrib4s(GLuint, GLshort, GLshort, GLshort, GLshort))
    failing!(fn VertexAttrib4sv(GLuint, *GLshort))
    failing!(fn VertexAttrib4ubv(GLuint, *GLubyte))
    failing!(fn VertexAttrib4uiv(GLuint, *GLuint))
    failing!(fn VertexAttrib4usv(GLuint, *GLushort))
    failing!(fn VertexAttribI1i(GLuint, GLint))
    failing!(fn VertexAttribI1iv(GLuint, *GLint))
    failing!(fn VertexAttribI1ui(GLuint, GLuint))
    failing!(fn VertexAttribI1uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI2i(GLuint, GLint, GLint))
    failing!(fn VertexAttribI2iv(GLuint, *GLint))
    failing!(fn VertexAttribI2ui(GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI2uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI3i(GLuint, GLint, GLint, GLint))
    failing!(fn VertexAttribI3iv(GLuint, *GLint))
    failing!(fn VertexAttribI3ui(GLuint, GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI3uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI4bv(GLuint, *GLbyte))
    failing!(fn VertexAttribI4i(GLuint, GLint, GLint, GLint, GLint))
    failing!(fn VertexAttribI4iv(GLuint, *GLint))
    failing!(fn VertexAttribI4sv(GLuint, *GLshort))
    failing!(fn VertexAttribI4ubv(GLuint, *GLubyte))
    failing!(fn VertexAttribI4ui(GLuint, GLuint, GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI4uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI4usv(GLuint, *GLushort))
    failing!(fn VertexAttribIPointer(GLuint, GLint, GLenum, GLsizei, *c_void))
    failing!(fn VertexAttribPointer(GLuint, GLint, GLenum, GLboolean, GLsizei, *c_void))
    failing!(fn Viewport(GLint, GLint, GLsizei, GLsizei))
    failing!(fn WaitSync(GLsync, GLbitfield, GLuint64))
}

/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
///
/// ~~~
/// let gl = gl::load_with(glfw::get_proc_address);
/// ~~~
pub fn load_with(loadfn: |symbol: &str| -> Option<extern "system" fn()>) {
    ActiveTexture::load_with(|s| loadfn(s));
    AttachShader::load_with(|s| loadfn(s));
    BeginConditionalRender::load_with(|s| loadfn(s));
    BeginQuery::load_with(|s| loadfn(s));
    BeginTransformFeedback::load_with(|s| loadfn(s));
    BindAttribLocation::load_with(|s| loadfn(s));
    BindBuffer::load_with(|s| loadfn(s));
    BindBufferBase::load_with(|s| loadfn(s));
    BindBufferRange::load_with(|s| loadfn(s));
    BindFragDataLocation::load_with(|s| loadfn(s));
    BindFramebuffer::load_with(|s| loadfn(s));
    BindRenderbuffer::load_with(|s| loadfn(s));
    BindTexture::load_with(|s| loadfn(s));
    BindVertexArray::load_with(|s| loadfn(s));
    BlendColor::load_with(|s| loadfn(s));
    BlendEquation::load_with(|s| loadfn(s));
    BlendEquationSeparate::load_with(|s| loadfn(s));
    BlendFunc::load_with(|s| loadfn(s));
    BlendFuncSeparate::load_with(|s| loadfn(s));
    BlitFramebuffer::load_with(|s| loadfn(s));
    BufferData::load_with(|s| loadfn(s));
    BufferSubData::load_with(|s| loadfn(s));
    CheckFramebufferStatus::load_with(|s| loadfn(s));
    ClampColor::load_with(|s| loadfn(s));
    Clear::load_with(|s| loadfn(s));
    ClearBufferfi::load_with(|s| loadfn(s));
    ClearBufferfv::load_with(|s| loadfn(s));
    ClearBufferiv::load_with(|s| loadfn(s));
    ClearBufferuiv::load_with(|s| loadfn(s));
    ClearColor::load_with(|s| loadfn(s));
    ClearDepth::load_with(|s| loadfn(s));
    ClearStencil::load_with(|s| loadfn(s));
    ClientWaitSync::load_with(|s| loadfn(s));
    ColorMask::load_with(|s| loadfn(s));
    ColorMaski::load_with(|s| loadfn(s));
    CompileShader::load_with(|s| loadfn(s));
    CompressedTexImage1D::load_with(|s| loadfn(s));
    CompressedTexImage2D::load_with(|s| loadfn(s));
    CompressedTexImage3D::load_with(|s| loadfn(s));
    CompressedTexSubImage1D::load_with(|s| loadfn(s));
    CompressedTexSubImage2D::load_with(|s| loadfn(s));
    CompressedTexSubImage3D::load_with(|s| loadfn(s));
    CopyBufferSubData::load_with(|s| loadfn(s));
    CopyTexImage1D::load_with(|s| loadfn(s));
    CopyTexImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage1D::load_with(|s| loadfn(s));
    CopyTexSubImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage3D::load_with(|s| loadfn(s));
    CreateProgram::load_with(|s| loadfn(s));
    CreateShader::load_with(|s| loadfn(s));
    CullFace::load_with(|s| loadfn(s));
    DeleteBuffers::load_with(|s| loadfn(s));
    DeleteFramebuffers::load_with(|s| loadfn(s));
    DeleteProgram::load_with(|s| loadfn(s));
    DeleteQueries::load_with(|s| loadfn(s));
    DeleteRenderbuffers::load_with(|s| loadfn(s));
    DeleteShader::load_with(|s| loadfn(s));
    DeleteSync::load_with(|s| loadfn(s));
    DeleteTextures::load_with(|s| loadfn(s));
    DeleteVertexArrays::load_with(|s| loadfn(s));
    DepthFunc::load_with(|s| loadfn(s));
    DepthMask::load_with(|s| loadfn(s));
    DepthRange::load_with(|s| loadfn(s));
    DetachShader::load_with(|s| loadfn(s));
    Disable::load_with(|s| loadfn(s));
    DisableVertexAttribArray::load_with(|s| loadfn(s));
    Disablei::load_with(|s| loadfn(s));
    DrawArrays::load_with(|s| loadfn(s));
    DrawArraysInstanced::load_with(|s| loadfn(s));
    DrawBuffer::load_with(|s| loadfn(s));
    DrawBuffers::load_with(|s| loadfn(s));
    DrawElements::load_with(|s| loadfn(s));
    DrawElementsBaseVertex::load_with(|s| loadfn(s));
    DrawElementsInstanced::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseVertex::load_with(|s| loadfn(s));
    DrawRangeElements::load_with(|s| loadfn(s));
    DrawRangeElementsBaseVertex::load_with(|s| loadfn(s));
    Enable::load_with(|s| loadfn(s));
    EnableVertexAttribArray::load_with(|s| loadfn(s));
    Enablei::load_with(|s| loadfn(s));
    EndConditionalRender::load_with(|s| loadfn(s));
    EndQuery::load_with(|s| loadfn(s));
    EndTransformFeedback::load_with(|s| loadfn(s));
    FenceSync::load_with(|s| loadfn(s));
    Finish::load_with(|s| loadfn(s));
    Flush::load_with(|s| loadfn(s));
    FlushMappedBufferRange::load_with(|s| loadfn(s));
    FramebufferRenderbuffer::load_with(|s| loadfn(s));
    FramebufferTexture::load_with(|s| loadfn(s));
    FramebufferTexture1D::load_with(|s| loadfn(s));
    FramebufferTexture2D::load_with(|s| loadfn(s));
    FramebufferTexture3D::load_with(|s| loadfn(s));
    FramebufferTextureLayer::load_with(|s| loadfn(s));
    FrontFace::load_with(|s| loadfn(s));
    GenBuffers::load_with(|s| loadfn(s));
    GenFramebuffers::load_with(|s| loadfn(s));
    GenQueries::load_with(|s| loadfn(s));
    GenRenderbuffers::load_with(|s| loadfn(s));
    GenTextures::load_with(|s| loadfn(s));
    GenVertexArrays::load_with(|s| loadfn(s));
    GenerateMipmap::load_with(|s| loadfn(s));
    GetActiveAttrib::load_with(|s| loadfn(s));
    GetActiveUniform::load_with(|s| loadfn(s));
    GetActiveUniformBlockName::load_with(|s| loadfn(s));
    GetActiveUniformBlockiv::load_with(|s| loadfn(s));
    GetActiveUniformName::load_with(|s| loadfn(s));
    GetActiveUniformsiv::load_with(|s| loadfn(s));
    GetAttachedShaders::load_with(|s| loadfn(s));
    GetAttribLocation::load_with(|s| loadfn(s));
    GetBooleani_v::load_with(|s| loadfn(s));
    GetBooleanv::load_with(|s| loadfn(s));
    GetBufferParameteri64v::load_with(|s| loadfn(s));
    GetBufferParameteriv::load_with(|s| loadfn(s));
    GetBufferPointerv::load_with(|s| loadfn(s));
    GetBufferSubData::load_with(|s| loadfn(s));
    GetCompressedTexImage::load_with(|s| loadfn(s));
    GetDoublev::load_with(|s| loadfn(s));
    GetError::load_with(|s| loadfn(s));
    GetFloatv::load_with(|s| loadfn(s));
    GetFragDataLocation::load_with(|s| loadfn(s));
    GetFramebufferAttachmentParameteriv::load_with(|s| loadfn(s));
    GetInteger64i_v::load_with(|s| loadfn(s));
    GetInteger64v::load_with(|s| loadfn(s));
    GetIntegeri_v::load_with(|s| loadfn(s));
    GetIntegerv::load_with(|s| loadfn(s));
    GetMultisamplefv::load_with(|s| loadfn(s));
    GetProgramInfoLog::load_with(|s| loadfn(s));
    GetProgramiv::load_with(|s| loadfn(s));
    GetQueryObjectiv::load_with(|s| loadfn(s));
    GetQueryObjectuiv::load_with(|s| loadfn(s));
    GetQueryiv::load_with(|s| loadfn(s));
    GetRenderbufferParameteriv::load_with(|s| loadfn(s));
    GetShaderInfoLog::load_with(|s| loadfn(s));
    GetShaderSource::load_with(|s| loadfn(s));
    GetShaderiv::load_with(|s| loadfn(s));
    GetString::load_with(|s| loadfn(s));
    GetStringi::load_with(|s| loadfn(s));
    GetSynciv::load_with(|s| loadfn(s));
    GetTexImage::load_with(|s| loadfn(s));
    GetTexLevelParameterfv::load_with(|s| loadfn(s));
    GetTexLevelParameteriv::load_with(|s| loadfn(s));
    GetTexParameterIiv::load_with(|s| loadfn(s));
    GetTexParameterIuiv::load_with(|s| loadfn(s));
    GetTexParameterfv::load_with(|s| loadfn(s));
    GetTexParameteriv::load_with(|s| loadfn(s));
    GetTransformFeedbackVarying::load_with(|s| loadfn(s));
    GetUniformBlockIndex::load_with(|s| loadfn(s));
    GetUniformIndices::load_with(|s| loadfn(s));
    GetUniformLocation::load_with(|s| loadfn(s));
    GetUniformfv::load_with(|s| loadfn(s));
    GetUniformiv::load_with(|s| loadfn(s));
    GetUniformuiv::load_with(|s| loadfn(s));
    GetVertexAttribIiv::load_with(|s| loadfn(s));
    GetVertexAttribIuiv::load_with(|s| loadfn(s));
    GetVertexAttribPointerv::load_with(|s| loadfn(s));
    GetVertexAttribdv::load_with(|s| loadfn(s));
    GetVertexAttribfv::load_with(|s| loadfn(s));
    GetVertexAttribiv::load_with(|s| loadfn(s));
    Hint::load_with(|s| loadfn(s));
    IsBuffer::load_with(|s| loadfn(s));
    IsEnabled::load_with(|s| loadfn(s));
    IsEnabledi::load_with(|s| loadfn(s));
    IsFramebuffer::load_with(|s| loadfn(s));
    IsProgram::load_with(|s| loadfn(s));
    IsQuery::load_with(|s| loadfn(s));
    IsRenderbuffer::load_with(|s| loadfn(s));
    IsShader::load_with(|s| loadfn(s));
    IsSync::load_with(|s| loadfn(s));
    IsTexture::load_with(|s| loadfn(s));
    IsVertexArray::load_with(|s| loadfn(s));
    LineWidth::load_with(|s| loadfn(s));
    LinkProgram::load_with(|s| loadfn(s));
    LogicOp::load_with(|s| loadfn(s));
    MapBuffer::load_with(|s| loadfn(s));
    MapBufferRange::load_with(|s| loadfn(s));
    MultiDrawArrays::load_with(|s| loadfn(s));
    MultiDrawElements::load_with(|s| loadfn(s));
    MultiDrawElementsBaseVertex::load_with(|s| loadfn(s));
    PixelStoref::load_with(|s| loadfn(s));
    PixelStorei::load_with(|s| loadfn(s));
    PointParameterf::load_with(|s| loadfn(s));
    PointParameterfv::load_with(|s| loadfn(s));
    PointParameteri::load_with(|s| loadfn(s));
    PointParameteriv::load_with(|s| loadfn(s));
    PointSize::load_with(|s| loadfn(s));
    PolygonMode::load_with(|s| loadfn(s));
    PolygonOffset::load_with(|s| loadfn(s));
    PrimitiveRestartIndex::load_with(|s| loadfn(s));
    ProvokingVertex::load_with(|s| loadfn(s));
    ReadBuffer::load_with(|s| loadfn(s));
    ReadPixels::load_with(|s| loadfn(s));
    RenderbufferStorage::load_with(|s| loadfn(s));
    RenderbufferStorageMultisample::load_with(|s| loadfn(s));
    SampleCoverage::load_with(|s| loadfn(s));
    SampleMaski::load_with(|s| loadfn(s));
    Scissor::load_with(|s| loadfn(s));
    ShaderSource::load_with(|s| loadfn(s));
    StencilFunc::load_with(|s| loadfn(s));
    StencilFuncSeparate::load_with(|s| loadfn(s));
    StencilMask::load_with(|s| loadfn(s));
    StencilMaskSeparate::load_with(|s| loadfn(s));
    StencilOp::load_with(|s| loadfn(s));
    StencilOpSeparate::load_with(|s| loadfn(s));
    TexBuffer::load_with(|s| loadfn(s));
    TexImage1D::load_with(|s| loadfn(s));
    TexImage2D::load_with(|s| loadfn(s));
    TexImage2DMultisample::load_with(|s| loadfn(s));
    TexImage3D::load_with(|s| loadfn(s));
    TexImage3DMultisample::load_with(|s| loadfn(s));
    TexParameterIiv::load_with(|s| loadfn(s));
    TexParameterIuiv::load_with(|s| loadfn(s));
    TexParameterf::load_with(|s| loadfn(s));
    TexParameterfv::load_with(|s| loadfn(s));
    TexParameteri::load_with(|s| loadfn(s));
    TexParameteriv::load_with(|s| loadfn(s));
    TexSubImage1D::load_with(|s| loadfn(s));
    TexSubImage2D::load_with(|s| loadfn(s));
    TexSubImage3D::load_with(|s| loadfn(s));
    TransformFeedbackVaryings::load_with(|s| loadfn(s));
    Uniform1f::load_with(|s| loadfn(s));
    Uniform1fv::load_with(|s| loadfn(s));
    Uniform1i::load_with(|s| loadfn(s));
    Uniform1iv::load_with(|s| loadfn(s));
    Uniform1ui::load_with(|s| loadfn(s));
    Uniform1uiv::load_with(|s| loadfn(s));
    Uniform2f::load_with(|s| loadfn(s));
    Uniform2fv::load_with(|s| loadfn(s));
    Uniform2i::load_with(|s| loadfn(s));
    Uniform2iv::load_with(|s| loadfn(s));
    Uniform2ui::load_with(|s| loadfn(s));
    Uniform2uiv::load_with(|s| loadfn(s));
    Uniform3f::load_with(|s| loadfn(s));
    Uniform3fv::load_with(|s| loadfn(s));
    Uniform3i::load_with(|s| loadfn(s));
    Uniform3iv::load_with(|s| loadfn(s));
    Uniform3ui::load_with(|s| loadfn(s));
    Uniform3uiv::load_with(|s| loadfn(s));
    Uniform4f::load_with(|s| loadfn(s));
    Uniform4fv::load_with(|s| loadfn(s));
    Uniform4i::load_with(|s| loadfn(s));
    Uniform4iv::load_with(|s| loadfn(s));
    Uniform4ui::load_with(|s| loadfn(s));
    Uniform4uiv::load_with(|s| loadfn(s));
    UniformBlockBinding::load_with(|s| loadfn(s));
    UniformMatrix2fv::load_with(|s| loadfn(s));
    UniformMatrix2x3fv::load_with(|s| loadfn(s));
    UniformMatrix2x4fv::load_with(|s| loadfn(s));
    UniformMatrix3fv::load_with(|s| loadfn(s));
    UniformMatrix3x2fv::load_with(|s| loadfn(s));
    UniformMatrix3x4fv::load_with(|s| loadfn(s));
    UniformMatrix4fv::load_with(|s| loadfn(s));
    UniformMatrix4x2fv::load_with(|s| loadfn(s));
    UniformMatrix4x3fv::load_with(|s| loadfn(s));
    UnmapBuffer::load_with(|s| loadfn(s));
    UseProgram::load_with(|s| loadfn(s));
    ValidateProgram::load_with(|s| loadfn(s));
    VertexAttrib1d::load_with(|s| loadfn(s));
    VertexAttrib1dv::load_with(|s| loadfn(s));
    VertexAttrib1f::load_with(|s| loadfn(s));
    VertexAttrib1fv::load_with(|s| loadfn(s));
    VertexAttrib1s::load_with(|s| loadfn(s));
    VertexAttrib1sv::load_with(|s| loadfn(s));
    VertexAttrib2d::load_with(|s| loadfn(s));
    VertexAttrib2dv::load_with(|s| loadfn(s));
    VertexAttrib2f::load_with(|s| loadfn(s));
    VertexAttrib2fv::load_with(|s| loadfn(s));
    VertexAttrib2s::load_with(|s| loadfn(s));
    VertexAttrib2sv::load_with(|s| loadfn(s));
    VertexAttrib3d::load_with(|s| loadfn(s));
    VertexAttrib3dv::load_with(|s| loadfn(s));
    VertexAttrib3f::load_with(|s| loadfn(s));
    VertexAttrib3fv::load_with(|s| loadfn(s));
    VertexAttrib3s::load_with(|s| loadfn(s));
    VertexAttrib3sv::load_with(|s| loadfn(s));
    VertexAttrib4Nbv::load_with(|s| loadfn(s));
    VertexAttrib4Niv::load_with(|s| loadfn(s));
    VertexAttrib4Nsv::load_with(|s| loadfn(s));
    VertexAttrib4Nub::load_with(|s| loadfn(s));
    VertexAttrib4Nubv::load_with(|s| loadfn(s));
    VertexAttrib4Nuiv::load_with(|s| loadfn(s));
    VertexAttrib4Nusv::load_with(|s| loadfn(s));
    VertexAttrib4bv::load_with(|s| loadfn(s));
    VertexAttrib4d::load_with(|s| loadfn(s));
    VertexAttrib4dv::load_with(|s| loadfn(s));
    VertexAttrib4f::load_with(|s| loadfn(s));
    VertexAttrib4fv::load_with(|s| loadfn(s));
    VertexAttrib4iv::load_with(|s| loadfn(s));
    VertexAttrib4s::load_with(|s| loadfn(s));
    VertexAttrib4sv::load_with(|s| loadfn(s));
    VertexAttrib4ubv::load_with(|s| loadfn(s));
    VertexAttrib4uiv::load_with(|s| loadfn(s));
    VertexAttrib4usv::load_with(|s| loadfn(s));
    VertexAttribI1i::load_with(|s| loadfn(s));
    VertexAttribI1iv::load_with(|s| loadfn(s));
    VertexAttribI1ui::load_with(|s| loadfn(s));
    VertexAttribI1uiv::load_with(|s| loadfn(s));
    VertexAttribI2i::load_with(|s| loadfn(s));
    VertexAttribI2iv::load_with(|s| loadfn(s));
    VertexAttribI2ui::load_with(|s| loadfn(s));
    VertexAttribI2uiv::load_with(|s| loadfn(s));
    VertexAttribI3i::load_with(|s| loadfn(s));
    VertexAttribI3iv::load_with(|s| loadfn(s));
    VertexAttribI3ui::load_with(|s| loadfn(s));
    VertexAttribI3uiv::load_with(|s| loadfn(s));
    VertexAttribI4bv::load_with(|s| loadfn(s));
    VertexAttribI4i::load_with(|s| loadfn(s));
    VertexAttribI4iv::load_with(|s| loadfn(s));
    VertexAttribI4sv::load_with(|s| loadfn(s));
    VertexAttribI4ubv::load_with(|s| loadfn(s));
    VertexAttribI4ui::load_with(|s| loadfn(s));
    VertexAttribI4uiv::load_with(|s| loadfn(s));
    VertexAttribI4usv::load_with(|s| loadfn(s));
    VertexAttribIPointer::load_with(|s| loadfn(s));
    VertexAttribPointer::load_with(|s| loadfn(s));
    Viewport::load_with(|s| loadfn(s));
    WaitSync::load_with(|s| loadfn(s));
}

