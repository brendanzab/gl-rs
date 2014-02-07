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

#[crate_id = "github.com/bjz/gl-rs#gl:0.1"];
#[comment = "An OpenGL function loader."];
#[license = "ASL2"];
#[crate_type = "lib"];

#[feature(macro_rules)];
#[feature(globs)];

use std::libc::*;
use self::types::*;

pub mod types {
    use std::libc::*;
    
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

pub static CURRENT_BIT: GLenum = 0x00000001;
pub static POINT_BIT: GLenum = 0x00000002;
pub static LINE_BIT: GLenum = 0x00000004;
pub static POLYGON_BIT: GLenum = 0x00000008;
pub static POLYGON_STIPPLE_BIT: GLenum = 0x00000010;
pub static PIXEL_MODE_BIT: GLenum = 0x00000020;
pub static LIGHTING_BIT: GLenum = 0x00000040;
pub static FOG_BIT: GLenum = 0x00000080;
pub static DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub static ACCUM_BUFFER_BIT: GLenum = 0x00000200;
pub static STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub static VIEWPORT_BIT: GLenum = 0x00000800;
pub static TRANSFORM_BIT: GLenum = 0x00001000;
pub static ENABLE_BIT: GLenum = 0x00002000;
pub static COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub static HINT_BIT: GLenum = 0x00008000;
pub static EVAL_BIT: GLenum = 0x00010000;
pub static LIST_BIT: GLenum = 0x00020000;
pub static TEXTURE_BIT: GLenum = 0x00040000;
pub static SCISSOR_BIT: GLenum = 0x00080000;
pub static MULTISAMPLE_BIT: GLenum = 0x20000000;
pub static ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
pub static CLIENT_PIXEL_STORE_BIT: GLenum = 0x00000001;
pub static CLIENT_VERTEX_ARRAY_BIT: GLenum = 0x00000002;
pub static CLIENT_ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
pub static FALSE: GLboolean = 0;
pub static NO_ERROR: GLenum = 0;
pub static ZERO: GLenum = 0;
pub static NONE: GLenum = 0;
pub static TRUE: GLboolean = 1;
pub static ONE: GLenum = 1;
pub static POINTS: GLenum = 0x0000;
pub static LINES: GLenum = 0x0001;
pub static LINE_LOOP: GLenum = 0x0002;
pub static LINE_STRIP: GLenum = 0x0003;
pub static TRIANGLES: GLenum = 0x0004;
pub static TRIANGLE_STRIP: GLenum = 0x0005;
pub static TRIANGLE_FAN: GLenum = 0x0006;
pub static QUADS: GLenum = 0x0007;
pub static QUAD_STRIP: GLenum = 0x0008;
pub static POLYGON: GLenum = 0x0009;
pub static ACCUM: GLenum = 0x0100;
pub static LOAD: GLenum = 0x0101;
pub static RETURN: GLenum = 0x0102;
pub static MULT: GLenum = 0x0103;
pub static ADD: GLenum = 0x0104;
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
pub static AUX0: GLenum = 0x0409;
pub static AUX1: GLenum = 0x040A;
pub static AUX2: GLenum = 0x040B;
pub static AUX3: GLenum = 0x040C;
pub static INVALID_ENUM: GLenum = 0x0500;
pub static INVALID_VALUE: GLenum = 0x0501;
pub static INVALID_OPERATION: GLenum = 0x0502;
pub static STACK_OVERFLOW: GLenum = 0x0503;
pub static STACK_UNDERFLOW: GLenum = 0x0504;
pub static OUT_OF_MEMORY: GLenum = 0x0505;
pub static _2D: GLenum = 0x0600;
pub static _3D: GLenum = 0x0601;
pub static _3D_COLOR: GLenum = 0x0602;
pub static _3D_COLOR_TEXTURE: GLenum = 0x0603;
pub static _4D_COLOR_TEXTURE: GLenum = 0x0604;
pub static PASS_THROUGH_TOKEN: GLenum = 0x0700;
pub static POINT_TOKEN: GLenum = 0x0701;
pub static LINE_TOKEN: GLenum = 0x0702;
pub static POLYGON_TOKEN: GLenum = 0x0703;
pub static BITMAP_TOKEN: GLenum = 0x0704;
pub static DRAW_PIXEL_TOKEN: GLenum = 0x0705;
pub static COPY_PIXEL_TOKEN: GLenum = 0x0706;
pub static LINE_RESET_TOKEN: GLenum = 0x0707;
pub static EXP: GLenum = 0x0800;
pub static EXP2: GLenum = 0x0801;
pub static CW: GLenum = 0x0900;
pub static CCW: GLenum = 0x0901;
pub static COEFF: GLenum = 0x0A00;
pub static ORDER: GLenum = 0x0A01;
pub static DOMAIN: GLenum = 0x0A02;
pub static CURRENT_COLOR: GLenum = 0x0B00;
pub static CURRENT_INDEX: GLenum = 0x0B01;
pub static CURRENT_NORMAL: GLenum = 0x0B02;
pub static CURRENT_TEXTURE_COORDS: GLenum = 0x0B03;
pub static CURRENT_RASTER_COLOR: GLenum = 0x0B04;
pub static CURRENT_RASTER_INDEX: GLenum = 0x0B05;
pub static CURRENT_RASTER_TEXTURE_COORDS: GLenum = 0x0B06;
pub static CURRENT_RASTER_POSITION: GLenum = 0x0B07;
pub static CURRENT_RASTER_POSITION_VALID: GLenum = 0x0B08;
pub static CURRENT_RASTER_DISTANCE: GLenum = 0x0B09;
pub static POINT_SMOOTH: GLenum = 0x0B10;
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
pub static LINE_STIPPLE: GLenum = 0x0B24;
pub static LINE_STIPPLE_PATTERN: GLenum = 0x0B25;
pub static LINE_STIPPLE_REPEAT: GLenum = 0x0B26;
pub static LIST_MODE: GLenum = 0x0B30;
pub static MAX_LIST_NESTING: GLenum = 0x0B31;
pub static LIST_BASE: GLenum = 0x0B32;
pub static LIST_INDEX: GLenum = 0x0B33;
pub static POLYGON_MODE: GLenum = 0x0B40;
pub static POLYGON_SMOOTH: GLenum = 0x0B41;
pub static POLYGON_STIPPLE: GLenum = 0x0B42;
pub static EDGE_FLAG: GLenum = 0x0B43;
pub static CULL_FACE: GLenum = 0x0B44;
pub static CULL_FACE_MODE: GLenum = 0x0B45;
pub static FRONT_FACE: GLenum = 0x0B46;
pub static LIGHTING: GLenum = 0x0B50;
pub static LIGHT_MODEL_LOCAL_VIEWER: GLenum = 0x0B51;
pub static LIGHT_MODEL_TWO_SIDE: GLenum = 0x0B52;
pub static LIGHT_MODEL_AMBIENT: GLenum = 0x0B53;
pub static SHADE_MODEL: GLenum = 0x0B54;
pub static COLOR_MATERIAL_FACE: GLenum = 0x0B55;
pub static COLOR_MATERIAL_PARAMETER: GLenum = 0x0B56;
pub static COLOR_MATERIAL: GLenum = 0x0B57;
pub static FOG: GLenum = 0x0B60;
pub static FOG_INDEX: GLenum = 0x0B61;
pub static FOG_DENSITY: GLenum = 0x0B62;
pub static FOG_START: GLenum = 0x0B63;
pub static FOG_END: GLenum = 0x0B64;
pub static FOG_MODE: GLenum = 0x0B65;
pub static FOG_COLOR: GLenum = 0x0B66;
pub static DEPTH_RANGE: GLenum = 0x0B70;
pub static DEPTH_TEST: GLenum = 0x0B71;
pub static DEPTH_WRITEMASK: GLenum = 0x0B72;
pub static DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub static DEPTH_FUNC: GLenum = 0x0B74;
pub static ACCUM_CLEAR_VALUE: GLenum = 0x0B80;
pub static STENCIL_TEST: GLenum = 0x0B90;
pub static STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub static STENCIL_FUNC: GLenum = 0x0B92;
pub static STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub static STENCIL_FAIL: GLenum = 0x0B94;
pub static STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub static STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub static STENCIL_REF: GLenum = 0x0B97;
pub static STENCIL_WRITEMASK: GLenum = 0x0B98;
pub static MATRIX_MODE: GLenum = 0x0BA0;
pub static NORMALIZE: GLenum = 0x0BA1;
pub static VIEWPORT: GLenum = 0x0BA2;
pub static MODELVIEW_STACK_DEPTH: GLenum = 0x0BA3;
pub static PROJECTION_STACK_DEPTH: GLenum = 0x0BA4;
pub static TEXTURE_STACK_DEPTH: GLenum = 0x0BA5;
pub static MODELVIEW_MATRIX: GLenum = 0x0BA6;
pub static PROJECTION_MATRIX: GLenum = 0x0BA7;
pub static TEXTURE_MATRIX: GLenum = 0x0BA8;
pub static ATTRIB_STACK_DEPTH: GLenum = 0x0BB0;
pub static CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0BB1;
pub static ALPHA_TEST: GLenum = 0x0BC0;
pub static ALPHA_TEST_FUNC: GLenum = 0x0BC1;
pub static ALPHA_TEST_REF: GLenum = 0x0BC2;
pub static DITHER: GLenum = 0x0BD0;
pub static BLEND_DST: GLenum = 0x0BE0;
pub static BLEND_SRC: GLenum = 0x0BE1;
pub static BLEND: GLenum = 0x0BE2;
pub static LOGIC_OP_MODE: GLenum = 0x0BF0;
pub static INDEX_LOGIC_OP: GLenum = 0x0BF1;
pub static LOGIC_OP: GLenum = 0x0BF1;
pub static COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub static AUX_BUFFERS: GLenum = 0x0C00;
pub static DRAW_BUFFER: GLenum = 0x0C01;
pub static READ_BUFFER: GLenum = 0x0C02;
pub static SCISSOR_BOX: GLenum = 0x0C10;
pub static SCISSOR_TEST: GLenum = 0x0C11;
pub static INDEX_CLEAR_VALUE: GLenum = 0x0C20;
pub static INDEX_WRITEMASK: GLenum = 0x0C21;
pub static COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub static COLOR_WRITEMASK: GLenum = 0x0C23;
pub static INDEX_MODE: GLenum = 0x0C30;
pub static RGBA_MODE: GLenum = 0x0C31;
pub static DOUBLEBUFFER: GLenum = 0x0C32;
pub static STEREO: GLenum = 0x0C33;
pub static RENDER_MODE: GLenum = 0x0C40;
pub static PERSPECTIVE_CORRECTION_HINT: GLenum = 0x0C50;
pub static POINT_SMOOTH_HINT: GLenum = 0x0C51;
pub static LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub static POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub static FOG_HINT: GLenum = 0x0C54;
pub static TEXTURE_GEN_S: GLenum = 0x0C60;
pub static TEXTURE_GEN_T: GLenum = 0x0C61;
pub static TEXTURE_GEN_R: GLenum = 0x0C62;
pub static TEXTURE_GEN_Q: GLenum = 0x0C63;
pub static PIXEL_MAP_I_TO_I: GLenum = 0x0C70;
pub static PIXEL_MAP_S_TO_S: GLenum = 0x0C71;
pub static PIXEL_MAP_I_TO_R: GLenum = 0x0C72;
pub static PIXEL_MAP_I_TO_G: GLenum = 0x0C73;
pub static PIXEL_MAP_I_TO_B: GLenum = 0x0C74;
pub static PIXEL_MAP_I_TO_A: GLenum = 0x0C75;
pub static PIXEL_MAP_R_TO_R: GLenum = 0x0C76;
pub static PIXEL_MAP_G_TO_G: GLenum = 0x0C77;
pub static PIXEL_MAP_B_TO_B: GLenum = 0x0C78;
pub static PIXEL_MAP_A_TO_A: GLenum = 0x0C79;
pub static PIXEL_MAP_I_TO_I_SIZE: GLenum = 0x0CB0;
pub static PIXEL_MAP_S_TO_S_SIZE: GLenum = 0x0CB1;
pub static PIXEL_MAP_I_TO_R_SIZE: GLenum = 0x0CB2;
pub static PIXEL_MAP_I_TO_G_SIZE: GLenum = 0x0CB3;
pub static PIXEL_MAP_I_TO_B_SIZE: GLenum = 0x0CB4;
pub static PIXEL_MAP_I_TO_A_SIZE: GLenum = 0x0CB5;
pub static PIXEL_MAP_R_TO_R_SIZE: GLenum = 0x0CB6;
pub static PIXEL_MAP_G_TO_G_SIZE: GLenum = 0x0CB7;
pub static PIXEL_MAP_B_TO_B_SIZE: GLenum = 0x0CB8;
pub static PIXEL_MAP_A_TO_A_SIZE: GLenum = 0x0CB9;
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
pub static MAP_COLOR: GLenum = 0x0D10;
pub static MAP_STENCIL: GLenum = 0x0D11;
pub static INDEX_SHIFT: GLenum = 0x0D12;
pub static INDEX_OFFSET: GLenum = 0x0D13;
pub static RED_SCALE: GLenum = 0x0D14;
pub static RED_BIAS: GLenum = 0x0D15;
pub static ZOOM_X: GLenum = 0x0D16;
pub static ZOOM_Y: GLenum = 0x0D17;
pub static GREEN_SCALE: GLenum = 0x0D18;
pub static GREEN_BIAS: GLenum = 0x0D19;
pub static BLUE_SCALE: GLenum = 0x0D1A;
pub static BLUE_BIAS: GLenum = 0x0D1B;
pub static ALPHA_SCALE: GLenum = 0x0D1C;
pub static ALPHA_BIAS: GLenum = 0x0D1D;
pub static DEPTH_SCALE: GLenum = 0x0D1E;
pub static DEPTH_BIAS: GLenum = 0x0D1F;
pub static MAX_EVAL_ORDER: GLenum = 0x0D30;
pub static MAX_LIGHTS: GLenum = 0x0D31;
pub static MAX_CLIP_PLANES: GLenum = 0x0D32;
pub static MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub static MAX_PIXEL_MAP_TABLE: GLenum = 0x0D34;
pub static MAX_ATTRIB_STACK_DEPTH: GLenum = 0x0D35;
pub static MAX_MODELVIEW_STACK_DEPTH: GLenum = 0x0D36;
pub static MAX_NAME_STACK_DEPTH: GLenum = 0x0D37;
pub static MAX_PROJECTION_STACK_DEPTH: GLenum = 0x0D38;
pub static MAX_TEXTURE_STACK_DEPTH: GLenum = 0x0D39;
pub static MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub static MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0D3B;
pub static SUBPIXEL_BITS: GLenum = 0x0D50;
pub static INDEX_BITS: GLenum = 0x0D51;
pub static RED_BITS: GLenum = 0x0D52;
pub static GREEN_BITS: GLenum = 0x0D53;
pub static BLUE_BITS: GLenum = 0x0D54;
pub static ALPHA_BITS: GLenum = 0x0D55;
pub static DEPTH_BITS: GLenum = 0x0D56;
pub static STENCIL_BITS: GLenum = 0x0D57;
pub static ACCUM_RED_BITS: GLenum = 0x0D58;
pub static ACCUM_GREEN_BITS: GLenum = 0x0D59;
pub static ACCUM_BLUE_BITS: GLenum = 0x0D5A;
pub static ACCUM_ALPHA_BITS: GLenum = 0x0D5B;
pub static NAME_STACK_DEPTH: GLenum = 0x0D70;
pub static AUTO_NORMAL: GLenum = 0x0D80;
pub static MAP1_COLOR_4: GLenum = 0x0D90;
pub static MAP1_INDEX: GLenum = 0x0D91;
pub static MAP1_NORMAL: GLenum = 0x0D92;
pub static MAP1_TEXTURE_COORD_1: GLenum = 0x0D93;
pub static MAP1_TEXTURE_COORD_2: GLenum = 0x0D94;
pub static MAP1_TEXTURE_COORD_3: GLenum = 0x0D95;
pub static MAP1_TEXTURE_COORD_4: GLenum = 0x0D96;
pub static MAP1_VERTEX_3: GLenum = 0x0D97;
pub static MAP1_VERTEX_4: GLenum = 0x0D98;
pub static MAP2_COLOR_4: GLenum = 0x0DB0;
pub static MAP2_INDEX: GLenum = 0x0DB1;
pub static MAP2_NORMAL: GLenum = 0x0DB2;
pub static MAP2_TEXTURE_COORD_1: GLenum = 0x0DB3;
pub static MAP2_TEXTURE_COORD_2: GLenum = 0x0DB4;
pub static MAP2_TEXTURE_COORD_3: GLenum = 0x0DB5;
pub static MAP2_TEXTURE_COORD_4: GLenum = 0x0DB6;
pub static MAP2_VERTEX_3: GLenum = 0x0DB7;
pub static MAP2_VERTEX_4: GLenum = 0x0DB8;
pub static MAP1_GRID_DOMAIN: GLenum = 0x0DD0;
pub static MAP1_GRID_SEGMENTS: GLenum = 0x0DD1;
pub static MAP2_GRID_DOMAIN: GLenum = 0x0DD2;
pub static MAP2_GRID_SEGMENTS: GLenum = 0x0DD3;
pub static TEXTURE_1D: GLenum = 0x0DE0;
pub static TEXTURE_2D: GLenum = 0x0DE1;
pub static FEEDBACK_BUFFER_POINTER: GLenum = 0x0DF0;
pub static FEEDBACK_BUFFER_SIZE: GLenum = 0x0DF1;
pub static FEEDBACK_BUFFER_TYPE: GLenum = 0x0DF2;
pub static SELECTION_BUFFER_POINTER: GLenum = 0x0DF3;
pub static SELECTION_BUFFER_SIZE: GLenum = 0x0DF4;
pub static TEXTURE_WIDTH: GLenum = 0x1000;
pub static TEXTURE_HEIGHT: GLenum = 0x1001;
pub static TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub static TEXTURE_COMPONENTS: GLenum = 0x1003;
pub static TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub static TEXTURE_BORDER: GLenum = 0x1005;
pub static DONT_CARE: GLenum = 0x1100;
pub static FASTEST: GLenum = 0x1101;
pub static NICEST: GLenum = 0x1102;
pub static AMBIENT: GLenum = 0x1200;
pub static DIFFUSE: GLenum = 0x1201;
pub static SPECULAR: GLenum = 0x1202;
pub static POSITION: GLenum = 0x1203;
pub static SPOT_DIRECTION: GLenum = 0x1204;
pub static SPOT_EXPONENT: GLenum = 0x1205;
pub static SPOT_CUTOFF: GLenum = 0x1206;
pub static CONSTANT_ATTENUATION: GLenum = 0x1207;
pub static LINEAR_ATTENUATION: GLenum = 0x1208;
pub static QUADRATIC_ATTENUATION: GLenum = 0x1209;
pub static COMPILE: GLenum = 0x1300;
pub static COMPILE_AND_EXECUTE: GLenum = 0x1301;
pub static BYTE: GLenum = 0x1400;
pub static UNSIGNED_BYTE: GLenum = 0x1401;
pub static SHORT: GLenum = 0x1402;
pub static UNSIGNED_SHORT: GLenum = 0x1403;
pub static INT: GLenum = 0x1404;
pub static UNSIGNED_INT: GLenum = 0x1405;
pub static FLOAT: GLenum = 0x1406;
pub static _2_BYTES: GLenum = 0x1407;
pub static _3_BYTES: GLenum = 0x1408;
pub static _4_BYTES: GLenum = 0x1409;
pub static DOUBLE: GLenum = 0x140A;
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
pub static EMISSION: GLenum = 0x1600;
pub static SHININESS: GLenum = 0x1601;
pub static AMBIENT_AND_DIFFUSE: GLenum = 0x1602;
pub static COLOR_INDEXES: GLenum = 0x1603;
pub static MODELVIEW: GLenum = 0x1700;
pub static PROJECTION: GLenum = 0x1701;
pub static TEXTURE: GLenum = 0x1702;
pub static COLOR: GLenum = 0x1800;
pub static DEPTH: GLenum = 0x1801;
pub static STENCIL: GLenum = 0x1802;
pub static COLOR_INDEX: GLenum = 0x1900;
pub static STENCIL_INDEX: GLenum = 0x1901;
pub static DEPTH_COMPONENT: GLenum = 0x1902;
pub static RED: GLenum = 0x1903;
pub static GREEN: GLenum = 0x1904;
pub static BLUE: GLenum = 0x1905;
pub static ALPHA: GLenum = 0x1906;
pub static RGB: GLenum = 0x1907;
pub static RGBA: GLenum = 0x1908;
pub static LUMINANCE: GLenum = 0x1909;
pub static LUMINANCE_ALPHA: GLenum = 0x190A;
pub static BITMAP: GLenum = 0x1A00;
pub static POINT: GLenum = 0x1B00;
pub static LINE: GLenum = 0x1B01;
pub static FILL: GLenum = 0x1B02;
pub static RENDER: GLenum = 0x1C00;
pub static FEEDBACK: GLenum = 0x1C01;
pub static SELECT: GLenum = 0x1C02;
pub static FLAT: GLenum = 0x1D00;
pub static SMOOTH: GLenum = 0x1D01;
pub static KEEP: GLenum = 0x1E00;
pub static REPLACE: GLenum = 0x1E01;
pub static INCR: GLenum = 0x1E02;
pub static DECR: GLenum = 0x1E03;
pub static VENDOR: GLenum = 0x1F00;
pub static RENDERER: GLenum = 0x1F01;
pub static VERSION: GLenum = 0x1F02;
pub static EXTENSIONS: GLenum = 0x1F03;
pub static S: GLenum = 0x2000;
pub static T: GLenum = 0x2001;
pub static R: GLenum = 0x2002;
pub static Q: GLenum = 0x2003;
pub static MODULATE: GLenum = 0x2100;
pub static DECAL: GLenum = 0x2101;
pub static TEXTURE_ENV_MODE: GLenum = 0x2200;
pub static TEXTURE_ENV_COLOR: GLenum = 0x2201;
pub static TEXTURE_ENV: GLenum = 0x2300;
pub static EYE_LINEAR: GLenum = 0x2400;
pub static OBJECT_LINEAR: GLenum = 0x2401;
pub static SPHERE_MAP: GLenum = 0x2402;
pub static TEXTURE_GEN_MODE: GLenum = 0x2500;
pub static OBJECT_PLANE: GLenum = 0x2501;
pub static EYE_PLANE: GLenum = 0x2502;
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
pub static CLAMP: GLenum = 0x2900;
pub static REPEAT: GLenum = 0x2901;
pub static POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub static POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub static POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub static R3_G3_B2: GLenum = 0x2A10;
pub static V2F: GLenum = 0x2A20;
pub static V3F: GLenum = 0x2A21;
pub static C4UB_V2F: GLenum = 0x2A22;
pub static C4UB_V3F: GLenum = 0x2A23;
pub static C3F_V3F: GLenum = 0x2A24;
pub static N3F_V3F: GLenum = 0x2A25;
pub static C4F_N3F_V3F: GLenum = 0x2A26;
pub static T2F_V3F: GLenum = 0x2A27;
pub static T4F_V4F: GLenum = 0x2A28;
pub static T2F_C4UB_V3F: GLenum = 0x2A29;
pub static T2F_C3F_V3F: GLenum = 0x2A2A;
pub static T2F_N3F_V3F: GLenum = 0x2A2B;
pub static T2F_C4F_N3F_V3F: GLenum = 0x2A2C;
pub static T4F_C4F_N3F_V4F: GLenum = 0x2A2D;
pub static CLIP_PLANE0: GLenum = 0x3000;
pub static CLIP_PLANE1: GLenum = 0x3001;
pub static CLIP_PLANE2: GLenum = 0x3002;
pub static CLIP_PLANE3: GLenum = 0x3003;
pub static CLIP_PLANE4: GLenum = 0x3004;
pub static CLIP_PLANE5: GLenum = 0x3005;
pub static LIGHT0: GLenum = 0x4000;
pub static LIGHT1: GLenum = 0x4001;
pub static LIGHT2: GLenum = 0x4002;
pub static LIGHT3: GLenum = 0x4003;
pub static LIGHT4: GLenum = 0x4004;
pub static LIGHT5: GLenum = 0x4005;
pub static LIGHT6: GLenum = 0x4006;
pub static LIGHT7: GLenum = 0x4007;
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
pub static RESCALE_NORMAL: GLenum = 0x803A;
pub static ALPHA4: GLenum = 0x803B;
pub static ALPHA8: GLenum = 0x803C;
pub static ALPHA12: GLenum = 0x803D;
pub static ALPHA16: GLenum = 0x803E;
pub static LUMINANCE4: GLenum = 0x803F;
pub static LUMINANCE8: GLenum = 0x8040;
pub static LUMINANCE12: GLenum = 0x8041;
pub static LUMINANCE16: GLenum = 0x8042;
pub static LUMINANCE4_ALPHA4: GLenum = 0x8043;
pub static LUMINANCE6_ALPHA2: GLenum = 0x8044;
pub static LUMINANCE8_ALPHA8: GLenum = 0x8045;
pub static LUMINANCE12_ALPHA4: GLenum = 0x8046;
pub static LUMINANCE12_ALPHA12: GLenum = 0x8047;
pub static LUMINANCE16_ALPHA16: GLenum = 0x8048;
pub static INTENSITY: GLenum = 0x8049;
pub static INTENSITY4: GLenum = 0x804A;
pub static INTENSITY8: GLenum = 0x804B;
pub static INTENSITY12: GLenum = 0x804C;
pub static INTENSITY16: GLenum = 0x804D;
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
pub static TEXTURE_LUMINANCE_SIZE: GLenum = 0x8060;
pub static TEXTURE_INTENSITY_SIZE: GLenum = 0x8061;
pub static PROXY_TEXTURE_1D: GLenum = 0x8063;
pub static PROXY_TEXTURE_2D: GLenum = 0x8064;
pub static TEXTURE_PRIORITY: GLenum = 0x8066;
pub static TEXTURE_RESIDENT: GLenum = 0x8067;
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
pub static VERTEX_ARRAY: GLenum = 0x8074;
pub static NORMAL_ARRAY: GLenum = 0x8075;
pub static COLOR_ARRAY: GLenum = 0x8076;
pub static INDEX_ARRAY: GLenum = 0x8077;
pub static TEXTURE_COORD_ARRAY: GLenum = 0x8078;
pub static EDGE_FLAG_ARRAY: GLenum = 0x8079;
pub static VERTEX_ARRAY_SIZE: GLenum = 0x807A;
pub static VERTEX_ARRAY_TYPE: GLenum = 0x807B;
pub static VERTEX_ARRAY_STRIDE: GLenum = 0x807C;
pub static NORMAL_ARRAY_TYPE: GLenum = 0x807E;
pub static NORMAL_ARRAY_STRIDE: GLenum = 0x807F;
pub static COLOR_ARRAY_SIZE: GLenum = 0x8081;
pub static COLOR_ARRAY_TYPE: GLenum = 0x8082;
pub static COLOR_ARRAY_STRIDE: GLenum = 0x8083;
pub static INDEX_ARRAY_TYPE: GLenum = 0x8085;
pub static INDEX_ARRAY_STRIDE: GLenum = 0x8086;
pub static TEXTURE_COORD_ARRAY_SIZE: GLenum = 0x8088;
pub static TEXTURE_COORD_ARRAY_TYPE: GLenum = 0x8089;
pub static TEXTURE_COORD_ARRAY_STRIDE: GLenum = 0x808A;
pub static EDGE_FLAG_ARRAY_STRIDE: GLenum = 0x808C;
pub static VERTEX_ARRAY_POINTER: GLenum = 0x808E;
pub static NORMAL_ARRAY_POINTER: GLenum = 0x808F;
pub static COLOR_ARRAY_POINTER: GLenum = 0x8090;
pub static INDEX_ARRAY_POINTER: GLenum = 0x8091;
pub static TEXTURE_COORD_ARRAY_POINTER: GLenum = 0x8092;
pub static EDGE_FLAG_ARRAY_POINTER: GLenum = 0x8093;
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
pub static POINT_SIZE_MIN: GLenum = 0x8126;
pub static POINT_SIZE_MAX: GLenum = 0x8127;
pub static POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub static POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
pub static CLAMP_TO_BORDER: GLenum = 0x812D;
pub static CLAMP_TO_EDGE: GLenum = 0x812F;
pub static TEXTURE_MIN_LOD: GLenum = 0x813A;
pub static TEXTURE_MAX_LOD: GLenum = 0x813B;
pub static TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub static TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub static GENERATE_MIPMAP: GLenum = 0x8191;
pub static GENERATE_MIPMAP_HINT: GLenum = 0x8192;
pub static DEPTH_COMPONENT16: GLenum = 0x81A5;
pub static DEPTH_COMPONENT24: GLenum = 0x81A6;
pub static DEPTH_COMPONENT32: GLenum = 0x81A7;
pub static LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
pub static SINGLE_COLOR: GLenum = 0x81F9;
pub static SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
pub static UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub static UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub static UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub static UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub static UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub static UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub static UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub static MIRRORED_REPEAT: GLenum = 0x8370;
pub static FOG_COORDINATE_SOURCE: GLenum = 0x8450;
pub static FOG_COORD_SRC: GLenum = 0x8450;
pub static FOG_COORDINATE: GLenum = 0x8451;
pub static FOG_COORD: GLenum = 0x8451;
pub static FRAGMENT_DEPTH: GLenum = 0x8452;
pub static CURRENT_FOG_COORDINATE: GLenum = 0x8453;
pub static CURRENT_FOG_COORD: GLenum = 0x8453;
pub static FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
pub static FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
pub static FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
pub static FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
pub static FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
pub static FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
pub static FOG_COORDINATE_ARRAY: GLenum = 0x8457;
pub static FOG_COORD_ARRAY: GLenum = 0x8457;
pub static COLOR_SUM: GLenum = 0x8458;
pub static CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
pub static SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
pub static SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
pub static SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
pub static SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
pub static SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
pub static CURRENT_RASTER_SECONDARY_COLOR: GLenum = 0x845F;
pub static ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
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
pub static CLIENT_ACTIVE_TEXTURE: GLenum = 0x84E1;
pub static MAX_TEXTURE_UNITS: GLenum = 0x84E2;
pub static TRANSPOSE_MODELVIEW_MATRIX: GLenum = 0x84E3;
pub static TRANSPOSE_PROJECTION_MATRIX: GLenum = 0x84E4;
pub static TRANSPOSE_TEXTURE_MATRIX: GLenum = 0x84E5;
pub static TRANSPOSE_COLOR_MATRIX: GLenum = 0x84E6;
pub static SUBTRACT: GLenum = 0x84E7;
pub static COMPRESSED_ALPHA: GLenum = 0x84E9;
pub static COMPRESSED_LUMINANCE: GLenum = 0x84EA;
pub static COMPRESSED_LUMINANCE_ALPHA: GLenum = 0x84EB;
pub static COMPRESSED_INTENSITY: GLenum = 0x84EC;
pub static COMPRESSED_RGB: GLenum = 0x84ED;
pub static COMPRESSED_RGBA: GLenum = 0x84EE;
pub static TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub static MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub static TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
pub static TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub static INCR_WRAP: GLenum = 0x8507;
pub static DECR_WRAP: GLenum = 0x8508;
pub static NORMAL_MAP: GLenum = 0x8511;
pub static REFLECTION_MAP: GLenum = 0x8512;
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
pub static COMBINE: GLenum = 0x8570;
pub static COMBINE_RGB: GLenum = 0x8571;
pub static COMBINE_ALPHA: GLenum = 0x8572;
pub static RGB_SCALE: GLenum = 0x8573;
pub static ADD_SIGNED: GLenum = 0x8574;
pub static INTERPOLATE: GLenum = 0x8575;
pub static CONSTANT: GLenum = 0x8576;
pub static PRIMARY_COLOR: GLenum = 0x8577;
pub static PREVIOUS: GLenum = 0x8578;
pub static SOURCE0_RGB: GLenum = 0x8580;
pub static SRC0_RGB: GLenum = 0x8580;
pub static SOURCE1_RGB: GLenum = 0x8581;
pub static SRC1_RGB: GLenum = 0x8581;
pub static SOURCE2_RGB: GLenum = 0x8582;
pub static SRC2_RGB: GLenum = 0x8582;
pub static SOURCE0_ALPHA: GLenum = 0x8588;
pub static SRC0_ALPHA: GLenum = 0x8588;
pub static SOURCE1_ALPHA: GLenum = 0x8589;
pub static SRC1_ALPHA: GLenum = 0x8589;
pub static SOURCE2_ALPHA: GLenum = 0x858A;
pub static SRC2_ALPHA: GLenum = 0x858A;
pub static OPERAND0_RGB: GLenum = 0x8590;
pub static OPERAND1_RGB: GLenum = 0x8591;
pub static OPERAND2_RGB: GLenum = 0x8592;
pub static OPERAND0_ALPHA: GLenum = 0x8598;
pub static OPERAND1_ALPHA: GLenum = 0x8599;
pub static OPERAND2_ALPHA: GLenum = 0x859A;
pub static VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub static VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub static VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub static VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub static CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub static VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static VERTEX_PROGRAM_TWO_SIDE: GLenum = 0x8643;
pub static VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub static TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub static TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub static NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub static COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub static DOT3_RGB: GLenum = 0x86AE;
pub static DOT3_RGBA: GLenum = 0x86AF;
pub static BUFFER_SIZE: GLenum = 0x8764;
pub static BUFFER_USAGE: GLenum = 0x8765;
pub static STENCIL_BACK_FUNC: GLenum = 0x8800;
pub static STENCIL_BACK_FAIL: GLenum = 0x8801;
pub static STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub static STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
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
pub static DEPTH_TEXTURE_MODE: GLenum = 0x884B;
pub static TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub static TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub static COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
pub static POINT_SPRITE: GLenum = 0x8861;
pub static COORD_REPLACE: GLenum = 0x8862;
pub static QUERY_COUNTER_BITS: GLenum = 0x8864;
pub static CURRENT_QUERY: GLenum = 0x8865;
pub static QUERY_RESULT: GLenum = 0x8866;
pub static QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub static MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub static VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub static MAX_TEXTURE_COORDS: GLenum = 0x8871;
pub static MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub static ARRAY_BUFFER: GLenum = 0x8892;
pub static ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub static ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub static ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub static VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
pub static NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
pub static COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
pub static INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
pub static TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
pub static EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
pub static SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
pub static FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub static FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub static WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
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
pub static SAMPLES_PASSED: GLenum = 0x8914;
pub static FRAGMENT_SHADER: GLenum = 0x8B30;
pub static VERTEX_SHADER: GLenum = 0x8B31;
pub static MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub static MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub static MAX_VARYING_FLOATS: GLenum = 0x8B4B;
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
pub static SRGB: GLenum = 0x8C40;
pub static SRGB8: GLenum = 0x8C41;
pub static SRGB_ALPHA: GLenum = 0x8C42;
pub static SRGB8_ALPHA8: GLenum = 0x8C43;
pub static SLUMINANCE_ALPHA: GLenum = 0x8C44;
pub static SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
pub static SLUMINANCE: GLenum = 0x8C46;
pub static SLUMINANCE8: GLenum = 0x8C47;
pub static COMPRESSED_SRGB: GLenum = 0x8C48;
pub static COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub static COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
pub static COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;
pub static POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub static LOWER_LEFT: GLenum = 0x8CA1;
pub static UPPER_LEFT: GLenum = 0x8CA2;
pub static STENCIL_BACK_REF: GLenum = 0x8CA3;
pub static STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub static STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;

#[inline] pub fn Accum(op: GLenum, value: GLfloat) { unsafe { (storage::Accum.f)(op, value) } }
#[inline] pub fn ActiveTexture(texture: GLenum) { unsafe { (storage::ActiveTexture.f)(texture) } }
#[inline] pub fn AlphaFunc(func: GLenum, ref_: GLfloat) { unsafe { (storage::AlphaFunc.f)(func, ref_) } }
#[inline] pub unsafe fn AreTexturesResident(n: GLsizei, textures: *GLuint, residences: *mut GLboolean) -> GLboolean { (storage::AreTexturesResident.f)(n, textures, residences) }
#[inline] pub fn ArrayElement(i: GLint) { unsafe { (storage::ArrayElement.f)(i) } }
#[inline] pub fn AttachShader(program: GLuint, shader: GLuint) { unsafe { (storage::AttachShader.f)(program, shader) } }
#[inline] pub fn Begin(mode: GLenum) { unsafe { (storage::Begin.f)(mode) } }
#[inline] pub fn BeginQuery(target: GLenum, id: GLuint) { unsafe { (storage::BeginQuery.f)(target, id) } }
#[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar) { (storage::BindAttribLocation.f)(program, index, name) }
#[inline] pub fn BindBuffer(target: GLenum, buffer: GLuint) { unsafe { (storage::BindBuffer.f)(target, buffer) } }
#[inline] pub fn BindTexture(target: GLenum, texture: GLuint) { unsafe { (storage::BindTexture.f)(target, texture) } }
#[inline] pub unsafe fn Bitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *GLubyte) { (storage::Bitmap.f)(width, height, xorig, yorig, xmove, ymove, bitmap) }
#[inline] pub fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::BlendColor.f)(red, green, blue, alpha) } }
#[inline] pub fn BlendEquation(mode: GLenum) { unsafe { (storage::BlendEquation.f)(mode) } }
#[inline] pub fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) { unsafe { (storage::BlendEquationSeparate.f)(modeRGB, modeAlpha) } }
#[inline] pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum) { unsafe { (storage::BlendFunc.f)(sfactor, dfactor) } }
#[inline] pub fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { unsafe { (storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) } }
#[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *c_void, usage: GLenum) { (storage::BufferData.f)(target, size, data, usage) }
#[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *c_void) { (storage::BufferSubData.f)(target, offset, size, data) }
#[inline] pub fn CallList(list: GLuint) { unsafe { (storage::CallList.f)(list) } }
#[inline] pub unsafe fn CallLists(n: GLsizei, type_: GLenum, lists: *c_void) { (storage::CallLists.f)(n, type_, lists) }
#[inline] pub fn Clear(mask: GLbitfield) { unsafe { (storage::Clear.f)(mask) } }
#[inline] pub fn ClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::ClearAccum.f)(red, green, blue, alpha) } }
#[inline] pub fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::ClearColor.f)(red, green, blue, alpha) } }
#[inline] pub fn ClearDepth(depth: GLdouble) { unsafe { (storage::ClearDepth.f)(depth) } }
#[inline] pub fn ClearIndex(c: GLfloat) { unsafe { (storage::ClearIndex.f)(c) } }
#[inline] pub fn ClearStencil(s: GLint) { unsafe { (storage::ClearStencil.f)(s) } }
#[inline] pub fn ClientActiveTexture(texture: GLenum) { unsafe { (storage::ClientActiveTexture.f)(texture) } }
#[inline] pub unsafe fn ClipPlane(plane: GLenum, equation: *GLdouble) { (storage::ClipPlane.f)(plane, equation) }
#[inline] pub fn Color3b(red: GLbyte, green: GLbyte, blue: GLbyte) { unsafe { (storage::Color3b.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3bv(v: *GLbyte) { (storage::Color3bv.f)(v) }
#[inline] pub fn Color3d(red: GLdouble, green: GLdouble, blue: GLdouble) { unsafe { (storage::Color3d.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3dv(v: *GLdouble) { (storage::Color3dv.f)(v) }
#[inline] pub fn Color3f(red: GLfloat, green: GLfloat, blue: GLfloat) { unsafe { (storage::Color3f.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3fv(v: *GLfloat) { (storage::Color3fv.f)(v) }
#[inline] pub fn Color3i(red: GLint, green: GLint, blue: GLint) { unsafe { (storage::Color3i.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3iv(v: *GLint) { (storage::Color3iv.f)(v) }
#[inline] pub fn Color3s(red: GLshort, green: GLshort, blue: GLshort) { unsafe { (storage::Color3s.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3sv(v: *GLshort) { (storage::Color3sv.f)(v) }
#[inline] pub fn Color3ub(red: GLubyte, green: GLubyte, blue: GLubyte) { unsafe { (storage::Color3ub.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3ubv(v: *GLubyte) { (storage::Color3ubv.f)(v) }
#[inline] pub fn Color3ui(red: GLuint, green: GLuint, blue: GLuint) { unsafe { (storage::Color3ui.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3uiv(v: *GLuint) { (storage::Color3uiv.f)(v) }
#[inline] pub fn Color3us(red: GLushort, green: GLushort, blue: GLushort) { unsafe { (storage::Color3us.f)(red, green, blue) } }
#[inline] pub unsafe fn Color3usv(v: *GLushort) { (storage::Color3usv.f)(v) }
#[inline] pub fn Color4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) { unsafe { (storage::Color4b.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4bv(v: *GLbyte) { (storage::Color4bv.f)(v) }
#[inline] pub fn Color4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble) { unsafe { (storage::Color4d.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4dv(v: *GLdouble) { (storage::Color4dv.f)(v) }
#[inline] pub fn Color4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::Color4f.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4fv(v: *GLfloat) { (storage::Color4fv.f)(v) }
#[inline] pub fn Color4i(red: GLint, green: GLint, blue: GLint, alpha: GLint) { unsafe { (storage::Color4i.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4iv(v: *GLint) { (storage::Color4iv.f)(v) }
#[inline] pub fn Color4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) { unsafe { (storage::Color4s.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4sv(v: *GLshort) { (storage::Color4sv.f)(v) }
#[inline] pub fn Color4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) { unsafe { (storage::Color4ub.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4ubv(v: *GLubyte) { (storage::Color4ubv.f)(v) }
#[inline] pub fn Color4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) { unsafe { (storage::Color4ui.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4uiv(v: *GLuint) { (storage::Color4uiv.f)(v) }
#[inline] pub fn Color4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) { unsafe { (storage::Color4us.f)(red, green, blue, alpha) } }
#[inline] pub unsafe fn Color4usv(v: *GLushort) { (storage::Color4usv.f)(v) }
#[inline] pub fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { unsafe { (storage::ColorMask.f)(red, green, blue, alpha) } }
#[inline] pub fn ColorMaterial(face: GLenum, mode: GLenum) { unsafe { (storage::ColorMaterial.f)(face, mode) } }
#[inline] pub unsafe fn ColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::ColorPointer.f)(size, type_, stride, pointer) }
#[inline] pub fn CompileShader(shader: GLuint) { unsafe { (storage::CompileShader.f)(shader) } }
#[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void) { (storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[inline] pub fn CopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum) { unsafe { (storage::CopyPixels.f)(x, y, width, height, type_) } }
#[inline] pub fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) } }
#[inline] pub fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) } }
#[inline] pub fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { unsafe { (storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) } }
#[inline] pub fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) } }
#[inline] pub fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) } }
#[inline] pub fn CreateProgram() -> GLuint { unsafe { (storage::CreateProgram.f)() } }
#[inline] pub fn CreateShader(type_: GLenum) -> GLuint { unsafe { (storage::CreateShader.f)(type_) } }
#[inline] pub fn CullFace(mode: GLenum) { unsafe { (storage::CullFace.f)(mode) } }
#[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *GLuint) { (storage::DeleteBuffers.f)(n, buffers) }
#[inline] pub fn DeleteLists(list: GLuint, range: GLsizei) { unsafe { (storage::DeleteLists.f)(list, range) } }
#[inline] pub fn DeleteProgram(program: GLuint) { unsafe { (storage::DeleteProgram.f)(program) } }
#[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *GLuint) { (storage::DeleteQueries.f)(n, ids) }
#[inline] pub fn DeleteShader(shader: GLuint) { unsafe { (storage::DeleteShader.f)(shader) } }
#[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *GLuint) { (storage::DeleteTextures.f)(n, textures) }
#[inline] pub fn DepthFunc(func: GLenum) { unsafe { (storage::DepthFunc.f)(func) } }
#[inline] pub fn DepthMask(flag: GLboolean) { unsafe { (storage::DepthMask.f)(flag) } }
#[inline] pub fn DepthRange(near: GLdouble, far: GLdouble) { unsafe { (storage::DepthRange.f)(near, far) } }
#[inline] pub fn DetachShader(program: GLuint, shader: GLuint) { unsafe { (storage::DetachShader.f)(program, shader) } }
#[inline] pub fn Disable(cap: GLenum) { unsafe { (storage::Disable.f)(cap) } }
#[inline] pub fn DisableClientState(array: GLenum) { unsafe { (storage::DisableClientState.f)(array) } }
#[inline] pub fn DisableVertexAttribArray(index: GLuint) { unsafe { (storage::DisableVertexAttribArray.f)(index) } }
#[inline] pub fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) { unsafe { (storage::DrawArrays.f)(mode, first, count) } }
#[inline] pub fn DrawBuffer(mode: GLenum) { unsafe { (storage::DrawBuffer.f)(mode) } }
#[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *GLenum) { (storage::DrawBuffers.f)(n, bufs) }
#[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void) { (storage::DrawElements.f)(mode, count, type_, indices) }
#[inline] pub unsafe fn DrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::DrawPixels.f)(width, height, format, type_, pixels) }
#[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void) { (storage::DrawRangeElements.f)(mode, start, end, count, type_, indices) }
#[inline] pub fn EdgeFlag(flag: GLboolean) { unsafe { (storage::EdgeFlag.f)(flag) } }
#[inline] pub unsafe fn EdgeFlagPointer(stride: GLsizei, pointer: *c_void) { (storage::EdgeFlagPointer.f)(stride, pointer) }
#[inline] pub unsafe fn EdgeFlagv(flag: *GLboolean) { (storage::EdgeFlagv.f)(flag) }
#[inline] pub fn Enable(cap: GLenum) { unsafe { (storage::Enable.f)(cap) } }
#[inline] pub fn EnableClientState(array: GLenum) { unsafe { (storage::EnableClientState.f)(array) } }
#[inline] pub fn EnableVertexAttribArray(index: GLuint) { unsafe { (storage::EnableVertexAttribArray.f)(index) } }
#[inline] pub fn End() { unsafe { (storage::End.f)() } }
#[inline] pub fn EndList() { unsafe { (storage::EndList.f)() } }
#[inline] pub fn EndQuery(target: GLenum) { unsafe { (storage::EndQuery.f)(target) } }
#[inline] pub fn EvalCoord1d(u: GLdouble) { unsafe { (storage::EvalCoord1d.f)(u) } }
#[inline] pub unsafe fn EvalCoord1dv(u: *GLdouble) { (storage::EvalCoord1dv.f)(u) }
#[inline] pub fn EvalCoord1f(u: GLfloat) { unsafe { (storage::EvalCoord1f.f)(u) } }
#[inline] pub unsafe fn EvalCoord1fv(u: *GLfloat) { (storage::EvalCoord1fv.f)(u) }
#[inline] pub fn EvalCoord2d(u: GLdouble, v: GLdouble) { unsafe { (storage::EvalCoord2d.f)(u, v) } }
#[inline] pub unsafe fn EvalCoord2dv(u: *GLdouble) { (storage::EvalCoord2dv.f)(u) }
#[inline] pub fn EvalCoord2f(u: GLfloat, v: GLfloat) { unsafe { (storage::EvalCoord2f.f)(u, v) } }
#[inline] pub unsafe fn EvalCoord2fv(u: *GLfloat) { (storage::EvalCoord2fv.f)(u) }
#[inline] pub fn EvalMesh1(mode: GLenum, i1: GLint, i2: GLint) { unsafe { (storage::EvalMesh1.f)(mode, i1, i2) } }
#[inline] pub fn EvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) { unsafe { (storage::EvalMesh2.f)(mode, i1, i2, j1, j2) } }
#[inline] pub fn EvalPoint1(i: GLint) { unsafe { (storage::EvalPoint1.f)(i) } }
#[inline] pub fn EvalPoint2(i: GLint, j: GLint) { unsafe { (storage::EvalPoint2.f)(i, j) } }
#[inline] pub unsafe fn FeedbackBuffer(size: GLsizei, type_: GLenum, buffer: *mut GLfloat) { (storage::FeedbackBuffer.f)(size, type_, buffer) }
#[inline] pub fn Finish() { unsafe { (storage::Finish.f)() } }
#[inline] pub fn Flush() { unsafe { (storage::Flush.f)() } }
#[inline] pub unsafe fn FogCoordPointer(type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::FogCoordPointer.f)(type_, stride, pointer) }
#[inline] pub fn FogCoordd(coord: GLdouble) { unsafe { (storage::FogCoordd.f)(coord) } }
#[inline] pub unsafe fn FogCoorddv(coord: *GLdouble) { (storage::FogCoorddv.f)(coord) }
#[inline] pub fn FogCoordf(coord: GLfloat) { unsafe { (storage::FogCoordf.f)(coord) } }
#[inline] pub unsafe fn FogCoordfv(coord: *GLfloat) { (storage::FogCoordfv.f)(coord) }
#[inline] pub fn Fogf(pname: GLenum, param: GLfloat) { unsafe { (storage::Fogf.f)(pname, param) } }
#[inline] pub unsafe fn Fogfv(pname: GLenum, params: *GLfloat) { (storage::Fogfv.f)(pname, params) }
#[inline] pub fn Fogi(pname: GLenum, param: GLint) { unsafe { (storage::Fogi.f)(pname, param) } }
#[inline] pub unsafe fn Fogiv(pname: GLenum, params: *GLint) { (storage::Fogiv.f)(pname, params) }
#[inline] pub fn FrontFace(mode: GLenum) { unsafe { (storage::FrontFace.f)(mode) } }
#[inline] pub fn Frustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) { unsafe { (storage::Frustum.f)(left, right, bottom, top, zNear, zFar) } }
#[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) { (storage::GenBuffers.f)(n, buffers) }
#[inline] pub fn GenLists(range: GLsizei) -> GLuint { unsafe { (storage::GenLists.f)(range) } }
#[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) { (storage::GenQueries.f)(n, ids) }
#[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) { (storage::GenTextures.f)(n, textures) }
#[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) { (storage::GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetAttribLocation.f)(program, name) }
#[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) { (storage::GetBooleanv.f)(pname, data) }
#[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetBufferParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut c_void) { (storage::GetBufferPointerv.f)(target, pname, params) }
#[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) { (storage::GetBufferSubData.f)(target, offset, size, data) }
#[inline] pub unsafe fn GetClipPlane(plane: GLenum, equation: *mut GLdouble) { (storage::GetClipPlane.f)(plane, equation) }
#[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) { (storage::GetCompressedTexImage.f)(target, level, img) }
#[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) { (storage::GetDoublev.f)(pname, data) }
#[inline] pub fn GetError() -> GLenum { unsafe { (storage::GetError.f)() } }
#[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) { (storage::GetFloatv.f)(pname, data) }
#[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) { (storage::GetIntegerv.f)(pname, data) }
#[inline] pub unsafe fn GetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetLightfv.f)(light, pname, params) }
#[inline] pub unsafe fn GetLightiv(light: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetLightiv.f)(light, pname, params) }
#[inline] pub unsafe fn GetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble) { (storage::GetMapdv.f)(target, query, v) }
#[inline] pub unsafe fn GetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat) { (storage::GetMapfv.f)(target, query, v) }
#[inline] pub unsafe fn GetMapiv(target: GLenum, query: GLenum, v: *mut GLint) { (storage::GetMapiv.f)(target, query, v) }
#[inline] pub unsafe fn GetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetMaterialfv.f)(face, pname, params) }
#[inline] pub unsafe fn GetMaterialiv(face: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetMaterialiv.f)(face, pname, params) }
#[inline] pub unsafe fn GetPixelMapfv(map: GLenum, values: *mut GLfloat) { (storage::GetPixelMapfv.f)(map, values) }
#[inline] pub unsafe fn GetPixelMapuiv(map: GLenum, values: *mut GLuint) { (storage::GetPixelMapuiv.f)(map, values) }
#[inline] pub unsafe fn GetPixelMapusv(map: GLenum, values: *mut GLushort) { (storage::GetPixelMapusv.f)(map, values) }
#[inline] pub unsafe fn GetPointerv(pname: GLenum, params: **mut c_void) { (storage::GetPointerv.f)(pname, params) }
#[inline] pub unsafe fn GetPolygonStipple(mask: *mut GLubyte) { (storage::GetPolygonStipple.f)(mask) }
#[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetProgramiv.f)(program, pname, params) }
#[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetQueryObjectiv.f)(id, pname, params) }
#[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetQueryObjectuiv.f)(id, pname, params) }
#[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetQueryiv.f)(target, pname, params) }
#[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) { (storage::GetShaderSource.f)(shader, bufSize, length, source) }
#[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetShaderiv.f)(shader, pname, params) }
#[inline] pub fn GetString(name: GLenum) -> *GLubyte { unsafe { (storage::GetString.f)(name) } }
#[inline] pub unsafe fn GetTexEnvfv(target: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetTexEnvfv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexEnviv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexEnviv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble) { (storage::GetTexGendv.f)(coord, pname, params) }
#[inline] pub unsafe fn GetTexGenfv(coord: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetTexGenfv.f)(coord, pname, params) }
#[inline] pub unsafe fn GetTexGeniv(coord: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexGeniv.f)(coord, pname, params) }
#[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) { (storage::GetTexImage.f)(target, level, format, type_, pixels) }
#[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) { (storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
#[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) { (storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
#[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetTexParameterfv.f)(target, pname, params) }
#[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetUniformLocation.f)(program, name) }
#[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) { (storage::GetUniformfv.f)(program, location, params) }
#[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) { (storage::GetUniformiv.f)(program, location, params) }
#[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut c_void) { (storage::GetVertexAttribPointerv.f)(index, pname, pointer) }
#[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { (storage::GetVertexAttribdv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) { (storage::GetVertexAttribfv.f)(index, pname, params) }
#[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetVertexAttribiv.f)(index, pname, params) }
#[inline] pub fn Hint(target: GLenum, mode: GLenum) { unsafe { (storage::Hint.f)(target, mode) } }
#[inline] pub fn IndexMask(mask: GLuint) { unsafe { (storage::IndexMask.f)(mask) } }
#[inline] pub unsafe fn IndexPointer(type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::IndexPointer.f)(type_, stride, pointer) }
#[inline] pub fn Indexd(c: GLdouble) { unsafe { (storage::Indexd.f)(c) } }
#[inline] pub unsafe fn Indexdv(c: *GLdouble) { (storage::Indexdv.f)(c) }
#[inline] pub fn Indexf(c: GLfloat) { unsafe { (storage::Indexf.f)(c) } }
#[inline] pub unsafe fn Indexfv(c: *GLfloat) { (storage::Indexfv.f)(c) }
#[inline] pub fn Indexi(c: GLint) { unsafe { (storage::Indexi.f)(c) } }
#[inline] pub unsafe fn Indexiv(c: *GLint) { (storage::Indexiv.f)(c) }
#[inline] pub fn Indexs(c: GLshort) { unsafe { (storage::Indexs.f)(c) } }
#[inline] pub unsafe fn Indexsv(c: *GLshort) { (storage::Indexsv.f)(c) }
#[inline] pub fn Indexub(c: GLubyte) { unsafe { (storage::Indexub.f)(c) } }
#[inline] pub unsafe fn Indexubv(c: *GLubyte) { (storage::Indexubv.f)(c) }
#[inline] pub fn InitNames() { unsafe { (storage::InitNames.f)() } }
#[inline] pub unsafe fn InterleavedArrays(format: GLenum, stride: GLsizei, pointer: *c_void) { (storage::InterleavedArrays.f)(format, stride, pointer) }
#[inline] pub fn IsBuffer(buffer: GLuint) -> GLboolean { unsafe { (storage::IsBuffer.f)(buffer) } }
#[inline] pub fn IsEnabled(cap: GLenum) -> GLboolean { unsafe { (storage::IsEnabled.f)(cap) } }
#[inline] pub fn IsList(list: GLuint) -> GLboolean { unsafe { (storage::IsList.f)(list) } }
#[inline] pub fn IsProgram(program: GLuint) -> GLboolean { unsafe { (storage::IsProgram.f)(program) } }
#[inline] pub fn IsQuery(id: GLuint) -> GLboolean { unsafe { (storage::IsQuery.f)(id) } }
#[inline] pub fn IsShader(shader: GLuint) -> GLboolean { unsafe { (storage::IsShader.f)(shader) } }
#[inline] pub fn IsTexture(texture: GLuint) -> GLboolean { unsafe { (storage::IsTexture.f)(texture) } }
#[inline] pub fn LightModelf(pname: GLenum, param: GLfloat) { unsafe { (storage::LightModelf.f)(pname, param) } }
#[inline] pub unsafe fn LightModelfv(pname: GLenum, params: *GLfloat) { (storage::LightModelfv.f)(pname, params) }
#[inline] pub fn LightModeli(pname: GLenum, param: GLint) { unsafe { (storage::LightModeli.f)(pname, param) } }
#[inline] pub unsafe fn LightModeliv(pname: GLenum, params: *GLint) { (storage::LightModeliv.f)(pname, params) }
#[inline] pub fn Lightf(light: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::Lightf.f)(light, pname, param) } }
#[inline] pub unsafe fn Lightfv(light: GLenum, pname: GLenum, params: *GLfloat) { (storage::Lightfv.f)(light, pname, params) }
#[inline] pub fn Lighti(light: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::Lighti.f)(light, pname, param) } }
#[inline] pub unsafe fn Lightiv(light: GLenum, pname: GLenum, params: *GLint) { (storage::Lightiv.f)(light, pname, params) }
#[inline] pub fn LineStipple(factor: GLint, pattern: GLushort) { unsafe { (storage::LineStipple.f)(factor, pattern) } }
#[inline] pub fn LineWidth(width: GLfloat) { unsafe { (storage::LineWidth.f)(width) } }
#[inline] pub fn LinkProgram(program: GLuint) { unsafe { (storage::LinkProgram.f)(program) } }
#[inline] pub fn ListBase(base: GLuint) { unsafe { (storage::ListBase.f)(base) } }
#[inline] pub fn LoadIdentity() { unsafe { (storage::LoadIdentity.f)() } }
#[inline] pub unsafe fn LoadMatrixd(m: *GLdouble) { (storage::LoadMatrixd.f)(m) }
#[inline] pub unsafe fn LoadMatrixf(m: *GLfloat) { (storage::LoadMatrixf.f)(m) }
#[inline] pub fn LoadName(name: GLuint) { unsafe { (storage::LoadName.f)(name) } }
#[inline] pub unsafe fn LoadTransposeMatrixd(m: *GLdouble) { (storage::LoadTransposeMatrixd.f)(m) }
#[inline] pub unsafe fn LoadTransposeMatrixf(m: *GLfloat) { (storage::LoadTransposeMatrixf.f)(m) }
#[inline] pub fn LogicOp(opcode: GLenum) { unsafe { (storage::LogicOp.f)(opcode) } }
#[inline] pub unsafe fn Map1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *GLdouble) { (storage::Map1d.f)(target, u1, u2, stride, order, points) }
#[inline] pub unsafe fn Map1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *GLfloat) { (storage::Map1f.f)(target, u1, u2, stride, order, points) }
#[inline] pub unsafe fn Map2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *GLdouble) { (storage::Map2d.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[inline] pub unsafe fn Map2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *GLfloat) { (storage::Map2f.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[inline] pub fn MapBuffer(target: GLenum, access: GLenum) -> *c_void { unsafe { (storage::MapBuffer.f)(target, access) } }
#[inline] pub fn MapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble) { unsafe { (storage::MapGrid1d.f)(un, u1, u2) } }
#[inline] pub fn MapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat) { unsafe { (storage::MapGrid1f.f)(un, u1, u2) } }
#[inline] pub fn MapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble) { unsafe { (storage::MapGrid2d.f)(un, u1, u2, vn, v1, v2) } }
#[inline] pub fn MapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat) { unsafe { (storage::MapGrid2f.f)(un, u1, u2, vn, v1, v2) } }
#[inline] pub fn Materialf(face: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::Materialf.f)(face, pname, param) } }
#[inline] pub unsafe fn Materialfv(face: GLenum, pname: GLenum, params: *GLfloat) { (storage::Materialfv.f)(face, pname, params) }
#[inline] pub fn Materiali(face: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::Materiali.f)(face, pname, param) } }
#[inline] pub unsafe fn Materialiv(face: GLenum, pname: GLenum, params: *GLint) { (storage::Materialiv.f)(face, pname, params) }
#[inline] pub fn MatrixMode(mode: GLenum) { unsafe { (storage::MatrixMode.f)(mode) } }
#[inline] pub unsafe fn MultMatrixd(m: *GLdouble) { (storage::MultMatrixd.f)(m) }
#[inline] pub unsafe fn MultMatrixf(m: *GLfloat) { (storage::MultMatrixf.f)(m) }
#[inline] pub unsafe fn MultTransposeMatrixd(m: *GLdouble) { (storage::MultTransposeMatrixd.f)(m) }
#[inline] pub unsafe fn MultTransposeMatrixf(m: *GLfloat) { (storage::MultTransposeMatrixf.f)(m) }
#[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei) { (storage::MultiDrawArrays.f)(mode, first, count, drawcount) }
#[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei) { (storage::MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
#[inline] pub fn MultiTexCoord1d(target: GLenum, s: GLdouble) { unsafe { (storage::MultiTexCoord1d.f)(target, s) } }
#[inline] pub unsafe fn MultiTexCoord1dv(target: GLenum, v: *GLdouble) { (storage::MultiTexCoord1dv.f)(target, v) }
#[inline] pub fn MultiTexCoord1f(target: GLenum, s: GLfloat) { unsafe { (storage::MultiTexCoord1f.f)(target, s) } }
#[inline] pub unsafe fn MultiTexCoord1fv(target: GLenum, v: *GLfloat) { (storage::MultiTexCoord1fv.f)(target, v) }
#[inline] pub fn MultiTexCoord1i(target: GLenum, s: GLint) { unsafe { (storage::MultiTexCoord1i.f)(target, s) } }
#[inline] pub unsafe fn MultiTexCoord1iv(target: GLenum, v: *GLint) { (storage::MultiTexCoord1iv.f)(target, v) }
#[inline] pub fn MultiTexCoord1s(target: GLenum, s: GLshort) { unsafe { (storage::MultiTexCoord1s.f)(target, s) } }
#[inline] pub unsafe fn MultiTexCoord1sv(target: GLenum, v: *GLshort) { (storage::MultiTexCoord1sv.f)(target, v) }
#[inline] pub fn MultiTexCoord2d(target: GLenum, s: GLdouble, t: GLdouble) { unsafe { (storage::MultiTexCoord2d.f)(target, s, t) } }
#[inline] pub unsafe fn MultiTexCoord2dv(target: GLenum, v: *GLdouble) { (storage::MultiTexCoord2dv.f)(target, v) }
#[inline] pub fn MultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat) { unsafe { (storage::MultiTexCoord2f.f)(target, s, t) } }
#[inline] pub unsafe fn MultiTexCoord2fv(target: GLenum, v: *GLfloat) { (storage::MultiTexCoord2fv.f)(target, v) }
#[inline] pub fn MultiTexCoord2i(target: GLenum, s: GLint, t: GLint) { unsafe { (storage::MultiTexCoord2i.f)(target, s, t) } }
#[inline] pub unsafe fn MultiTexCoord2iv(target: GLenum, v: *GLint) { (storage::MultiTexCoord2iv.f)(target, v) }
#[inline] pub fn MultiTexCoord2s(target: GLenum, s: GLshort, t: GLshort) { unsafe { (storage::MultiTexCoord2s.f)(target, s, t) } }
#[inline] pub unsafe fn MultiTexCoord2sv(target: GLenum, v: *GLshort) { (storage::MultiTexCoord2sv.f)(target, v) }
#[inline] pub fn MultiTexCoord3d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) { unsafe { (storage::MultiTexCoord3d.f)(target, s, t, r) } }
#[inline] pub unsafe fn MultiTexCoord3dv(target: GLenum, v: *GLdouble) { (storage::MultiTexCoord3dv.f)(target, v) }
#[inline] pub fn MultiTexCoord3f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) { unsafe { (storage::MultiTexCoord3f.f)(target, s, t, r) } }
#[inline] pub unsafe fn MultiTexCoord3fv(target: GLenum, v: *GLfloat) { (storage::MultiTexCoord3fv.f)(target, v) }
#[inline] pub fn MultiTexCoord3i(target: GLenum, s: GLint, t: GLint, r: GLint) { unsafe { (storage::MultiTexCoord3i.f)(target, s, t, r) } }
#[inline] pub unsafe fn MultiTexCoord3iv(target: GLenum, v: *GLint) { (storage::MultiTexCoord3iv.f)(target, v) }
#[inline] pub fn MultiTexCoord3s(target: GLenum, s: GLshort, t: GLshort, r: GLshort) { unsafe { (storage::MultiTexCoord3s.f)(target, s, t, r) } }
#[inline] pub unsafe fn MultiTexCoord3sv(target: GLenum, v: *GLshort) { (storage::MultiTexCoord3sv.f)(target, v) }
#[inline] pub fn MultiTexCoord4d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) { unsafe { (storage::MultiTexCoord4d.f)(target, s, t, r, q) } }
#[inline] pub unsafe fn MultiTexCoord4dv(target: GLenum, v: *GLdouble) { (storage::MultiTexCoord4dv.f)(target, v) }
#[inline] pub fn MultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) { unsafe { (storage::MultiTexCoord4f.f)(target, s, t, r, q) } }
#[inline] pub unsafe fn MultiTexCoord4fv(target: GLenum, v: *GLfloat) { (storage::MultiTexCoord4fv.f)(target, v) }
#[inline] pub fn MultiTexCoord4i(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) { unsafe { (storage::MultiTexCoord4i.f)(target, s, t, r, q) } }
#[inline] pub unsafe fn MultiTexCoord4iv(target: GLenum, v: *GLint) { (storage::MultiTexCoord4iv.f)(target, v) }
#[inline] pub fn MultiTexCoord4s(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) { unsafe { (storage::MultiTexCoord4s.f)(target, s, t, r, q) } }
#[inline] pub unsafe fn MultiTexCoord4sv(target: GLenum, v: *GLshort) { (storage::MultiTexCoord4sv.f)(target, v) }
#[inline] pub fn NewList(list: GLuint, mode: GLenum) { unsafe { (storage::NewList.f)(list, mode) } }
#[inline] pub fn Normal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte) { unsafe { (storage::Normal3b.f)(nx, ny, nz) } }
#[inline] pub unsafe fn Normal3bv(v: *GLbyte) { (storage::Normal3bv.f)(v) }
#[inline] pub fn Normal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble) { unsafe { (storage::Normal3d.f)(nx, ny, nz) } }
#[inline] pub unsafe fn Normal3dv(v: *GLdouble) { (storage::Normal3dv.f)(v) }
#[inline] pub fn Normal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat) { unsafe { (storage::Normal3f.f)(nx, ny, nz) } }
#[inline] pub unsafe fn Normal3fv(v: *GLfloat) { (storage::Normal3fv.f)(v) }
#[inline] pub fn Normal3i(nx: GLint, ny: GLint, nz: GLint) { unsafe { (storage::Normal3i.f)(nx, ny, nz) } }
#[inline] pub unsafe fn Normal3iv(v: *GLint) { (storage::Normal3iv.f)(v) }
#[inline] pub fn Normal3s(nx: GLshort, ny: GLshort, nz: GLshort) { unsafe { (storage::Normal3s.f)(nx, ny, nz) } }
#[inline] pub unsafe fn Normal3sv(v: *GLshort) { (storage::Normal3sv.f)(v) }
#[inline] pub unsafe fn NormalPointer(type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::NormalPointer.f)(type_, stride, pointer) }
#[inline] pub fn Ortho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) { unsafe { (storage::Ortho.f)(left, right, bottom, top, zNear, zFar) } }
#[inline] pub fn PassThrough(token: GLfloat) { unsafe { (storage::PassThrough.f)(token) } }
#[inline] pub unsafe fn PixelMapfv(map: GLenum, mapsize: GLsizei, values: *GLfloat) { (storage::PixelMapfv.f)(map, mapsize, values) }
#[inline] pub unsafe fn PixelMapuiv(map: GLenum, mapsize: GLsizei, values: *GLuint) { (storage::PixelMapuiv.f)(map, mapsize, values) }
#[inline] pub unsafe fn PixelMapusv(map: GLenum, mapsize: GLsizei, values: *GLushort) { (storage::PixelMapusv.f)(map, mapsize, values) }
#[inline] pub fn PixelStoref(pname: GLenum, param: GLfloat) { unsafe { (storage::PixelStoref.f)(pname, param) } }
#[inline] pub fn PixelStorei(pname: GLenum, param: GLint) { unsafe { (storage::PixelStorei.f)(pname, param) } }
#[inline] pub fn PixelTransferf(pname: GLenum, param: GLfloat) { unsafe { (storage::PixelTransferf.f)(pname, param) } }
#[inline] pub fn PixelTransferi(pname: GLenum, param: GLint) { unsafe { (storage::PixelTransferi.f)(pname, param) } }
#[inline] pub fn PixelZoom(xfactor: GLfloat, yfactor: GLfloat) { unsafe { (storage::PixelZoom.f)(xfactor, yfactor) } }
#[inline] pub fn PointParameterf(pname: GLenum, param: GLfloat) { unsafe { (storage::PointParameterf.f)(pname, param) } }
#[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *GLfloat) { (storage::PointParameterfv.f)(pname, params) }
#[inline] pub fn PointParameteri(pname: GLenum, param: GLint) { unsafe { (storage::PointParameteri.f)(pname, param) } }
#[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *GLint) { (storage::PointParameteriv.f)(pname, params) }
#[inline] pub fn PointSize(size: GLfloat) { unsafe { (storage::PointSize.f)(size) } }
#[inline] pub fn PolygonMode(face: GLenum, mode: GLenum) { unsafe { (storage::PolygonMode.f)(face, mode) } }
#[inline] pub fn PolygonOffset(factor: GLfloat, units: GLfloat) { unsafe { (storage::PolygonOffset.f)(factor, units) } }
#[inline] pub unsafe fn PolygonStipple(mask: *GLubyte) { (storage::PolygonStipple.f)(mask) }
#[inline] pub fn PopAttrib() { unsafe { (storage::PopAttrib.f)() } }
#[inline] pub fn PopClientAttrib() { unsafe { (storage::PopClientAttrib.f)() } }
#[inline] pub fn PopMatrix() { unsafe { (storage::PopMatrix.f)() } }
#[inline] pub fn PopName() { unsafe { (storage::PopName.f)() } }
#[inline] pub unsafe fn PrioritizeTextures(n: GLsizei, textures: *GLuint, priorities: *GLfloat) { (storage::PrioritizeTextures.f)(n, textures, priorities) }
#[inline] pub fn PushAttrib(mask: GLbitfield) { unsafe { (storage::PushAttrib.f)(mask) } }
#[inline] pub fn PushClientAttrib(mask: GLbitfield) { unsafe { (storage::PushClientAttrib.f)(mask) } }
#[inline] pub fn PushMatrix() { unsafe { (storage::PushMatrix.f)() } }
#[inline] pub fn PushName(name: GLuint) { unsafe { (storage::PushName.f)(name) } }
#[inline] pub fn RasterPos2d(x: GLdouble, y: GLdouble) { unsafe { (storage::RasterPos2d.f)(x, y) } }
#[inline] pub unsafe fn RasterPos2dv(v: *GLdouble) { (storage::RasterPos2dv.f)(v) }
#[inline] pub fn RasterPos2f(x: GLfloat, y: GLfloat) { unsafe { (storage::RasterPos2f.f)(x, y) } }
#[inline] pub unsafe fn RasterPos2fv(v: *GLfloat) { (storage::RasterPos2fv.f)(v) }
#[inline] pub fn RasterPos2i(x: GLint, y: GLint) { unsafe { (storage::RasterPos2i.f)(x, y) } }
#[inline] pub unsafe fn RasterPos2iv(v: *GLint) { (storage::RasterPos2iv.f)(v) }
#[inline] pub fn RasterPos2s(x: GLshort, y: GLshort) { unsafe { (storage::RasterPos2s.f)(x, y) } }
#[inline] pub unsafe fn RasterPos2sv(v: *GLshort) { (storage::RasterPos2sv.f)(v) }
#[inline] pub fn RasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::RasterPos3d.f)(x, y, z) } }
#[inline] pub unsafe fn RasterPos3dv(v: *GLdouble) { (storage::RasterPos3dv.f)(v) }
#[inline] pub fn RasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::RasterPos3f.f)(x, y, z) } }
#[inline] pub unsafe fn RasterPos3fv(v: *GLfloat) { (storage::RasterPos3fv.f)(v) }
#[inline] pub fn RasterPos3i(x: GLint, y: GLint, z: GLint) { unsafe { (storage::RasterPos3i.f)(x, y, z) } }
#[inline] pub unsafe fn RasterPos3iv(v: *GLint) { (storage::RasterPos3iv.f)(v) }
#[inline] pub fn RasterPos3s(x: GLshort, y: GLshort, z: GLshort) { unsafe { (storage::RasterPos3s.f)(x, y, z) } }
#[inline] pub unsafe fn RasterPos3sv(v: *GLshort) { (storage::RasterPos3sv.f)(v) }
#[inline] pub fn RasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::RasterPos4d.f)(x, y, z, w) } }
#[inline] pub unsafe fn RasterPos4dv(v: *GLdouble) { (storage::RasterPos4dv.f)(v) }
#[inline] pub fn RasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { unsafe { (storage::RasterPos4f.f)(x, y, z, w) } }
#[inline] pub unsafe fn RasterPos4fv(v: *GLfloat) { (storage::RasterPos4fv.f)(v) }
#[inline] pub fn RasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint) { unsafe { (storage::RasterPos4i.f)(x, y, z, w) } }
#[inline] pub unsafe fn RasterPos4iv(v: *GLint) { (storage::RasterPos4iv.f)(v) }
#[inline] pub fn RasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) { unsafe { (storage::RasterPos4s.f)(x, y, z, w) } }
#[inline] pub unsafe fn RasterPos4sv(v: *GLshort) { (storage::RasterPos4sv.f)(v) }
#[inline] pub fn ReadBuffer(mode: GLenum) { unsafe { (storage::ReadBuffer.f)(mode) } }
#[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) { (storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[inline] pub fn Rectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble) { unsafe { (storage::Rectd.f)(x1, y1, x2, y2) } }
#[inline] pub unsafe fn Rectdv(v1: *GLdouble, v2: *GLdouble) { (storage::Rectdv.f)(v1, v2) }
#[inline] pub fn Rectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) { unsafe { (storage::Rectf.f)(x1, y1, x2, y2) } }
#[inline] pub unsafe fn Rectfv(v1: *GLfloat, v2: *GLfloat) { (storage::Rectfv.f)(v1, v2) }
#[inline] pub fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) { unsafe { (storage::Recti.f)(x1, y1, x2, y2) } }
#[inline] pub unsafe fn Rectiv(v1: *GLint, v2: *GLint) { (storage::Rectiv.f)(v1, v2) }
#[inline] pub fn Rects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) { unsafe { (storage::Rects.f)(x1, y1, x2, y2) } }
#[inline] pub unsafe fn Rectsv(v1: *GLshort, v2: *GLshort) { (storage::Rectsv.f)(v1, v2) }
#[inline] pub fn RenderMode(mode: GLenum) -> GLint { unsafe { (storage::RenderMode.f)(mode) } }
#[inline] pub fn Rotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::Rotated.f)(angle, x, y, z) } }
#[inline] pub fn Rotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::Rotatef.f)(angle, x, y, z) } }
#[inline] pub fn SampleCoverage(value: GLfloat, invert: GLboolean) { unsafe { (storage::SampleCoverage.f)(value, invert) } }
#[inline] pub fn Scaled(x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::Scaled.f)(x, y, z) } }
#[inline] pub fn Scalef(x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::Scalef.f)(x, y, z) } }
#[inline] pub fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Scissor.f)(x, y, width, height) } }
#[inline] pub fn SecondaryColor3b(red: GLbyte, green: GLbyte, blue: GLbyte) { unsafe { (storage::SecondaryColor3b.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3bv(v: *GLbyte) { (storage::SecondaryColor3bv.f)(v) }
#[inline] pub fn SecondaryColor3d(red: GLdouble, green: GLdouble, blue: GLdouble) { unsafe { (storage::SecondaryColor3d.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3dv(v: *GLdouble) { (storage::SecondaryColor3dv.f)(v) }
#[inline] pub fn SecondaryColor3f(red: GLfloat, green: GLfloat, blue: GLfloat) { unsafe { (storage::SecondaryColor3f.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3fv(v: *GLfloat) { (storage::SecondaryColor3fv.f)(v) }
#[inline] pub fn SecondaryColor3i(red: GLint, green: GLint, blue: GLint) { unsafe { (storage::SecondaryColor3i.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3iv(v: *GLint) { (storage::SecondaryColor3iv.f)(v) }
#[inline] pub fn SecondaryColor3s(red: GLshort, green: GLshort, blue: GLshort) { unsafe { (storage::SecondaryColor3s.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3sv(v: *GLshort) { (storage::SecondaryColor3sv.f)(v) }
#[inline] pub fn SecondaryColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte) { unsafe { (storage::SecondaryColor3ub.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3ubv(v: *GLubyte) { (storage::SecondaryColor3ubv.f)(v) }
#[inline] pub fn SecondaryColor3ui(red: GLuint, green: GLuint, blue: GLuint) { unsafe { (storage::SecondaryColor3ui.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3uiv(v: *GLuint) { (storage::SecondaryColor3uiv.f)(v) }
#[inline] pub fn SecondaryColor3us(red: GLushort, green: GLushort, blue: GLushort) { unsafe { (storage::SecondaryColor3us.f)(red, green, blue) } }
#[inline] pub unsafe fn SecondaryColor3usv(v: *GLushort) { (storage::SecondaryColor3usv.f)(v) }
#[inline] pub unsafe fn SecondaryColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::SecondaryColorPointer.f)(size, type_, stride, pointer) }
#[inline] pub unsafe fn SelectBuffer(size: GLsizei, buffer: *mut GLuint) { (storage::SelectBuffer.f)(size, buffer) }
#[inline] pub fn ShadeModel(mode: GLenum) { unsafe { (storage::ShadeModel.f)(mode) } }
#[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint) { (storage::ShaderSource.f)(shader, count, string, length) }
#[inline] pub fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFunc.f)(func, ref_, mask) } }
#[inline] pub fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFuncSeparate.f)(face, func, ref_, mask) } }
#[inline] pub fn StencilMask(mask: GLuint) { unsafe { (storage::StencilMask.f)(mask) } }
#[inline] pub fn StencilMaskSeparate(face: GLenum, mask: GLuint) { unsafe { (storage::StencilMaskSeparate.f)(face, mask) } }
#[inline] pub fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) { unsafe { (storage::StencilOp.f)(fail, zfail, zpass) } }
#[inline] pub fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { unsafe { (storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass) } }
#[inline] pub fn TexCoord1d(s: GLdouble) { unsafe { (storage::TexCoord1d.f)(s) } }
#[inline] pub unsafe fn TexCoord1dv(v: *GLdouble) { (storage::TexCoord1dv.f)(v) }
#[inline] pub fn TexCoord1f(s: GLfloat) { unsafe { (storage::TexCoord1f.f)(s) } }
#[inline] pub unsafe fn TexCoord1fv(v: *GLfloat) { (storage::TexCoord1fv.f)(v) }
#[inline] pub fn TexCoord1i(s: GLint) { unsafe { (storage::TexCoord1i.f)(s) } }
#[inline] pub unsafe fn TexCoord1iv(v: *GLint) { (storage::TexCoord1iv.f)(v) }
#[inline] pub fn TexCoord1s(s: GLshort) { unsafe { (storage::TexCoord1s.f)(s) } }
#[inline] pub unsafe fn TexCoord1sv(v: *GLshort) { (storage::TexCoord1sv.f)(v) }
#[inline] pub fn TexCoord2d(s: GLdouble, t: GLdouble) { unsafe { (storage::TexCoord2d.f)(s, t) } }
#[inline] pub unsafe fn TexCoord2dv(v: *GLdouble) { (storage::TexCoord2dv.f)(v) }
#[inline] pub fn TexCoord2f(s: GLfloat, t: GLfloat) { unsafe { (storage::TexCoord2f.f)(s, t) } }
#[inline] pub unsafe fn TexCoord2fv(v: *GLfloat) { (storage::TexCoord2fv.f)(v) }
#[inline] pub fn TexCoord2i(s: GLint, t: GLint) { unsafe { (storage::TexCoord2i.f)(s, t) } }
#[inline] pub unsafe fn TexCoord2iv(v: *GLint) { (storage::TexCoord2iv.f)(v) }
#[inline] pub fn TexCoord2s(s: GLshort, t: GLshort) { unsafe { (storage::TexCoord2s.f)(s, t) } }
#[inline] pub unsafe fn TexCoord2sv(v: *GLshort) { (storage::TexCoord2sv.f)(v) }
#[inline] pub fn TexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble) { unsafe { (storage::TexCoord3d.f)(s, t, r) } }
#[inline] pub unsafe fn TexCoord3dv(v: *GLdouble) { (storage::TexCoord3dv.f)(v) }
#[inline] pub fn TexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat) { unsafe { (storage::TexCoord3f.f)(s, t, r) } }
#[inline] pub unsafe fn TexCoord3fv(v: *GLfloat) { (storage::TexCoord3fv.f)(v) }
#[inline] pub fn TexCoord3i(s: GLint, t: GLint, r: GLint) { unsafe { (storage::TexCoord3i.f)(s, t, r) } }
#[inline] pub unsafe fn TexCoord3iv(v: *GLint) { (storage::TexCoord3iv.f)(v) }
#[inline] pub fn TexCoord3s(s: GLshort, t: GLshort, r: GLshort) { unsafe { (storage::TexCoord3s.f)(s, t, r) } }
#[inline] pub unsafe fn TexCoord3sv(v: *GLshort) { (storage::TexCoord3sv.f)(v) }
#[inline] pub fn TexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) { unsafe { (storage::TexCoord4d.f)(s, t, r, q) } }
#[inline] pub unsafe fn TexCoord4dv(v: *GLdouble) { (storage::TexCoord4dv.f)(v) }
#[inline] pub fn TexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) { unsafe { (storage::TexCoord4f.f)(s, t, r, q) } }
#[inline] pub unsafe fn TexCoord4fv(v: *GLfloat) { (storage::TexCoord4fv.f)(v) }
#[inline] pub fn TexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint) { unsafe { (storage::TexCoord4i.f)(s, t, r, q) } }
#[inline] pub unsafe fn TexCoord4iv(v: *GLint) { (storage::TexCoord4iv.f)(v) }
#[inline] pub fn TexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort) { unsafe { (storage::TexCoord4s.f)(s, t, r, q) } }
#[inline] pub unsafe fn TexCoord4sv(v: *GLshort) { (storage::TexCoord4sv.f)(v) }
#[inline] pub unsafe fn TexCoordPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::TexCoordPointer.f)(size, type_, stride, pointer) }
#[inline] pub fn TexEnvf(target: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::TexEnvf.f)(target, pname, param) } }
#[inline] pub unsafe fn TexEnvfv(target: GLenum, pname: GLenum, params: *GLfloat) { (storage::TexEnvfv.f)(target, pname, params) }
#[inline] pub fn TexEnvi(target: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::TexEnvi.f)(target, pname, param) } }
#[inline] pub unsafe fn TexEnviv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexEnviv.f)(target, pname, params) }
#[inline] pub fn TexGend(coord: GLenum, pname: GLenum, param: GLdouble) { unsafe { (storage::TexGend.f)(coord, pname, param) } }
#[inline] pub unsafe fn TexGendv(coord: GLenum, pname: GLenum, params: *GLdouble) { (storage::TexGendv.f)(coord, pname, params) }
#[inline] pub fn TexGenf(coord: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::TexGenf.f)(coord, pname, param) } }
#[inline] pub unsafe fn TexGenfv(coord: GLenum, pname: GLenum, params: *GLfloat) { (storage::TexGenfv.f)(coord, pname, params) }
#[inline] pub fn TexGeni(coord: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::TexGeni.f)(coord, pname, param) } }
#[inline] pub unsafe fn TexGeniv(coord: GLenum, pname: GLenum, params: *GLint) { (storage::TexGeniv.f)(coord, pname, params) }
#[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
#[inline] pub fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::TexParameterf.f)(target, pname, param) } }
#[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat) { (storage::TexParameterfv.f)(target, pname, params) }
#[inline] pub fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::TexParameteri.f)(target, pname, param) } }
#[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexParameteriv.f)(target, pname, params) }
#[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void) { (storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[inline] pub fn Translated(x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::Translated.f)(x, y, z) } }
#[inline] pub fn Translatef(x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::Translatef.f)(x, y, z) } }
#[inline] pub fn Uniform1f(location: GLint, v0: GLfloat) { unsafe { (storage::Uniform1f.f)(location, v0) } }
#[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform1fv.f)(location, count, value) }
#[inline] pub fn Uniform1i(location: GLint, v0: GLint) { unsafe { (storage::Uniform1i.f)(location, v0) } }
#[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform1iv.f)(location, count, value) }
#[inline] pub fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) { unsafe { (storage::Uniform2f.f)(location, v0, v1) } }
#[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform2fv.f)(location, count, value) }
#[inline] pub fn Uniform2i(location: GLint, v0: GLint, v1: GLint) { unsafe { (storage::Uniform2i.f)(location, v0, v1) } }
#[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform2iv.f)(location, count, value) }
#[inline] pub fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { unsafe { (storage::Uniform3f.f)(location, v0, v1, v2) } }
#[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform3fv.f)(location, count, value) }
#[inline] pub fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) { unsafe { (storage::Uniform3i.f)(location, v0, v1, v2) } }
#[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform3iv.f)(location, count, value) }
#[inline] pub fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { unsafe { (storage::Uniform4f.f)(location, v0, v1, v2, v3) } }
#[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform4fv.f)(location, count, value) }
#[inline] pub fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { unsafe { (storage::Uniform4i.f)(location, v0, v1, v2, v3) } }
#[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform4iv.f)(location, count, value) }
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
#[inline] pub fn Vertex2d(x: GLdouble, y: GLdouble) { unsafe { (storage::Vertex2d.f)(x, y) } }
#[inline] pub unsafe fn Vertex2dv(v: *GLdouble) { (storage::Vertex2dv.f)(v) }
#[inline] pub fn Vertex2f(x: GLfloat, y: GLfloat) { unsafe { (storage::Vertex2f.f)(x, y) } }
#[inline] pub unsafe fn Vertex2fv(v: *GLfloat) { (storage::Vertex2fv.f)(v) }
#[inline] pub fn Vertex2i(x: GLint, y: GLint) { unsafe { (storage::Vertex2i.f)(x, y) } }
#[inline] pub unsafe fn Vertex2iv(v: *GLint) { (storage::Vertex2iv.f)(v) }
#[inline] pub fn Vertex2s(x: GLshort, y: GLshort) { unsafe { (storage::Vertex2s.f)(x, y) } }
#[inline] pub unsafe fn Vertex2sv(v: *GLshort) { (storage::Vertex2sv.f)(v) }
#[inline] pub fn Vertex3d(x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::Vertex3d.f)(x, y, z) } }
#[inline] pub unsafe fn Vertex3dv(v: *GLdouble) { (storage::Vertex3dv.f)(v) }
#[inline] pub fn Vertex3f(x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::Vertex3f.f)(x, y, z) } }
#[inline] pub unsafe fn Vertex3fv(v: *GLfloat) { (storage::Vertex3fv.f)(v) }
#[inline] pub fn Vertex3i(x: GLint, y: GLint, z: GLint) { unsafe { (storage::Vertex3i.f)(x, y, z) } }
#[inline] pub unsafe fn Vertex3iv(v: *GLint) { (storage::Vertex3iv.f)(v) }
#[inline] pub fn Vertex3s(x: GLshort, y: GLshort, z: GLshort) { unsafe { (storage::Vertex3s.f)(x, y, z) } }
#[inline] pub unsafe fn Vertex3sv(v: *GLshort) { (storage::Vertex3sv.f)(v) }
#[inline] pub fn Vertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::Vertex4d.f)(x, y, z, w) } }
#[inline] pub unsafe fn Vertex4dv(v: *GLdouble) { (storage::Vertex4dv.f)(v) }
#[inline] pub fn Vertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { unsafe { (storage::Vertex4f.f)(x, y, z, w) } }
#[inline] pub unsafe fn Vertex4fv(v: *GLfloat) { (storage::Vertex4fv.f)(v) }
#[inline] pub fn Vertex4i(x: GLint, y: GLint, z: GLint, w: GLint) { unsafe { (storage::Vertex4i.f)(x, y, z, w) } }
#[inline] pub unsafe fn Vertex4iv(v: *GLint) { (storage::Vertex4iv.f)(v) }
#[inline] pub fn Vertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) { unsafe { (storage::Vertex4s.f)(x, y, z, w) } }
#[inline] pub unsafe fn Vertex4sv(v: *GLshort) { (storage::Vertex4sv.f)(v) }
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
#[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *c_void) { (storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[inline] pub unsafe fn VertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void) { (storage::VertexPointer.f)(size, type_, stride, pointer) }
#[inline] pub fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Viewport.f)(x, y, width, height) } }
#[inline] pub fn WindowPos2d(x: GLdouble, y: GLdouble) { unsafe { (storage::WindowPos2d.f)(x, y) } }
#[inline] pub unsafe fn WindowPos2dv(v: *GLdouble) { (storage::WindowPos2dv.f)(v) }
#[inline] pub fn WindowPos2f(x: GLfloat, y: GLfloat) { unsafe { (storage::WindowPos2f.f)(x, y) } }
#[inline] pub unsafe fn WindowPos2fv(v: *GLfloat) { (storage::WindowPos2fv.f)(v) }
#[inline] pub fn WindowPos2i(x: GLint, y: GLint) { unsafe { (storage::WindowPos2i.f)(x, y) } }
#[inline] pub unsafe fn WindowPos2iv(v: *GLint) { (storage::WindowPos2iv.f)(v) }
#[inline] pub fn WindowPos2s(x: GLshort, y: GLshort) { unsafe { (storage::WindowPos2s.f)(x, y) } }
#[inline] pub unsafe fn WindowPos2sv(v: *GLshort) { (storage::WindowPos2sv.f)(v) }
#[inline] pub fn WindowPos3d(x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::WindowPos3d.f)(x, y, z) } }
#[inline] pub unsafe fn WindowPos3dv(v: *GLdouble) { (storage::WindowPos3dv.f)(v) }
#[inline] pub fn WindowPos3f(x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::WindowPos3f.f)(x, y, z) } }
#[inline] pub unsafe fn WindowPos3fv(v: *GLfloat) { (storage::WindowPos3fv.f)(v) }
#[inline] pub fn WindowPos3i(x: GLint, y: GLint, z: GLint) { unsafe { (storage::WindowPos3i.f)(x, y, z) } }
#[inline] pub unsafe fn WindowPos3iv(v: *GLint) { (storage::WindowPos3iv.f)(v) }
#[inline] pub fn WindowPos3s(x: GLshort, y: GLshort, z: GLshort) { unsafe { (storage::WindowPos3s.f)(x, y, z) } }
#[inline] pub unsafe fn WindowPos3sv(v: *GLshort) { (storage::WindowPos3sv.f)(v) }

pub struct FnPtr<F> { f: F, is_loaded: bool }

impl<F> FnPtr<F> {
    pub fn new(ptr: Option<extern "system" fn()>, failing_fn: F) -> FnPtr<F> {
        use std::cast::transmute;
        match ptr {
            Some(p) => FnPtr { f: unsafe { transmute(p) }, is_loaded: true },
            None => FnPtr { f: failing_fn, is_loaded: false },
        }
    }
}

mod storage {
    use std::libc::*;
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
    
    fn_ptr!(fn Accum(op: GLenum, value: GLfloat))
    fn_ptr!(fn ActiveTexture(texture: GLenum))
    fn_ptr!(fn AlphaFunc(func: GLenum, ref_: GLfloat))
    fn_ptr!(fn AreTexturesResident(n: GLsizei, textures: *GLuint, residences: *mut GLboolean) -> GLboolean)
    fn_ptr!(fn ArrayElement(i: GLint))
    fn_ptr!(fn AttachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn Begin(mode: GLenum))
    fn_ptr!(fn BeginQuery(target: GLenum, id: GLuint))
    fn_ptr!(fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar))
    fn_ptr!(fn BindBuffer(target: GLenum, buffer: GLuint))
    fn_ptr!(fn BindTexture(target: GLenum, texture: GLuint))
    fn_ptr!(fn Bitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *GLubyte))
    fn_ptr!(fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn BlendEquation(mode: GLenum))
    fn_ptr!(fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum))
    fn_ptr!(fn BlendFunc(sfactor: GLenum, dfactor: GLenum))
    fn_ptr!(fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum))
    fn_ptr!(fn BufferData(target: GLenum, size: GLsizeiptr, data: *c_void, usage: GLenum))
    fn_ptr!(fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *c_void))
    fn_ptr!(fn CallList(list: GLuint))
    fn_ptr!(fn CallLists(n: GLsizei, type_: GLenum, lists: *c_void))
    fn_ptr!(fn Clear(mask: GLbitfield))
    fn_ptr!(fn ClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn ClearDepth(depth: GLdouble))
    fn_ptr!(fn ClearIndex(c: GLfloat))
    fn_ptr!(fn ClearStencil(s: GLint))
    fn_ptr!(fn ClientActiveTexture(texture: GLenum))
    fn_ptr!(fn ClipPlane(plane: GLenum, equation: *GLdouble))
    fn_ptr!(fn Color3b(red: GLbyte, green: GLbyte, blue: GLbyte))
    fn_ptr!(fn Color3bv(v: *GLbyte))
    fn_ptr!(fn Color3d(red: GLdouble, green: GLdouble, blue: GLdouble))
    fn_ptr!(fn Color3dv(v: *GLdouble))
    fn_ptr!(fn Color3f(red: GLfloat, green: GLfloat, blue: GLfloat))
    fn_ptr!(fn Color3fv(v: *GLfloat))
    fn_ptr!(fn Color3i(red: GLint, green: GLint, blue: GLint))
    fn_ptr!(fn Color3iv(v: *GLint))
    fn_ptr!(fn Color3s(red: GLshort, green: GLshort, blue: GLshort))
    fn_ptr!(fn Color3sv(v: *GLshort))
    fn_ptr!(fn Color3ub(red: GLubyte, green: GLubyte, blue: GLubyte))
    fn_ptr!(fn Color3ubv(v: *GLubyte))
    fn_ptr!(fn Color3ui(red: GLuint, green: GLuint, blue: GLuint))
    fn_ptr!(fn Color3uiv(v: *GLuint))
    fn_ptr!(fn Color3us(red: GLushort, green: GLushort, blue: GLushort))
    fn_ptr!(fn Color3usv(v: *GLushort))
    fn_ptr!(fn Color4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte))
    fn_ptr!(fn Color4bv(v: *GLbyte))
    fn_ptr!(fn Color4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble))
    fn_ptr!(fn Color4dv(v: *GLdouble))
    fn_ptr!(fn Color4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn Color4fv(v: *GLfloat))
    fn_ptr!(fn Color4i(red: GLint, green: GLint, blue: GLint, alpha: GLint))
    fn_ptr!(fn Color4iv(v: *GLint))
    fn_ptr!(fn Color4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort))
    fn_ptr!(fn Color4sv(v: *GLshort))
    fn_ptr!(fn Color4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte))
    fn_ptr!(fn Color4ubv(v: *GLubyte))
    fn_ptr!(fn Color4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint))
    fn_ptr!(fn Color4uiv(v: *GLuint))
    fn_ptr!(fn Color4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort))
    fn_ptr!(fn Color4usv(v: *GLushort))
    fn_ptr!(fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean))
    fn_ptr!(fn ColorMaterial(face: GLenum, mode: GLenum))
    fn_ptr!(fn ColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn CompileShader(shader: GLuint))
    fn_ptr!(fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *c_void))
    fn_ptr!(fn CopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum))
    fn_ptr!(fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei))
    fn_ptr!(fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CreateProgram() -> GLuint)
    fn_ptr!(fn CreateShader(type_: GLenum) -> GLuint)
    fn_ptr!(fn CullFace(mode: GLenum))
    fn_ptr!(fn DeleteBuffers(n: GLsizei, buffers: *GLuint))
    fn_ptr!(fn DeleteLists(list: GLuint, range: GLsizei))
    fn_ptr!(fn DeleteProgram(program: GLuint))
    fn_ptr!(fn DeleteQueries(n: GLsizei, ids: *GLuint))
    fn_ptr!(fn DeleteShader(shader: GLuint))
    fn_ptr!(fn DeleteTextures(n: GLsizei, textures: *GLuint))
    fn_ptr!(fn DepthFunc(func: GLenum))
    fn_ptr!(fn DepthMask(flag: GLboolean))
    fn_ptr!(fn DepthRange(near: GLdouble, far: GLdouble))
    fn_ptr!(fn DetachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn Disable(cap: GLenum))
    fn_ptr!(fn DisableClientState(array: GLenum))
    fn_ptr!(fn DisableVertexAttribArray(index: GLuint))
    fn_ptr!(fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei))
    fn_ptr!(fn DrawBuffer(mode: GLenum))
    fn_ptr!(fn DrawBuffers(n: GLsizei, bufs: *GLenum))
    fn_ptr!(fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void))
    fn_ptr!(fn DrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *c_void))
    fn_ptr!(fn EdgeFlag(flag: GLboolean))
    fn_ptr!(fn EdgeFlagPointer(stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn EdgeFlagv(flag: *GLboolean))
    fn_ptr!(fn Enable(cap: GLenum))
    fn_ptr!(fn EnableClientState(array: GLenum))
    fn_ptr!(fn EnableVertexAttribArray(index: GLuint))
    fn_ptr!(fn End())
    fn_ptr!(fn EndList())
    fn_ptr!(fn EndQuery(target: GLenum))
    fn_ptr!(fn EvalCoord1d(u: GLdouble))
    fn_ptr!(fn EvalCoord1dv(u: *GLdouble))
    fn_ptr!(fn EvalCoord1f(u: GLfloat))
    fn_ptr!(fn EvalCoord1fv(u: *GLfloat))
    fn_ptr!(fn EvalCoord2d(u: GLdouble, v: GLdouble))
    fn_ptr!(fn EvalCoord2dv(u: *GLdouble))
    fn_ptr!(fn EvalCoord2f(u: GLfloat, v: GLfloat))
    fn_ptr!(fn EvalCoord2fv(u: *GLfloat))
    fn_ptr!(fn EvalMesh1(mode: GLenum, i1: GLint, i2: GLint))
    fn_ptr!(fn EvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint))
    fn_ptr!(fn EvalPoint1(i: GLint))
    fn_ptr!(fn EvalPoint2(i: GLint, j: GLint))
    fn_ptr!(fn FeedbackBuffer(size: GLsizei, type_: GLenum, buffer: *mut GLfloat))
    fn_ptr!(fn Finish())
    fn_ptr!(fn Flush())
    fn_ptr!(fn FogCoordPointer(type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn FogCoordd(coord: GLdouble))
    fn_ptr!(fn FogCoorddv(coord: *GLdouble))
    fn_ptr!(fn FogCoordf(coord: GLfloat))
    fn_ptr!(fn FogCoordfv(coord: *GLfloat))
    fn_ptr!(fn Fogf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn Fogfv(pname: GLenum, params: *GLfloat))
    fn_ptr!(fn Fogi(pname: GLenum, param: GLint))
    fn_ptr!(fn Fogiv(pname: GLenum, params: *GLint))
    fn_ptr!(fn FrontFace(mode: GLenum))
    fn_ptr!(fn Frustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble))
    fn_ptr!(fn GenBuffers(n: GLsizei, buffers: *mut GLuint))
    fn_ptr!(fn GenLists(range: GLsizei) -> GLuint)
    fn_ptr!(fn GenQueries(n: GLsizei, ids: *mut GLuint))
    fn_ptr!(fn GenTextures(n: GLsizei, textures: *mut GLuint))
    fn_ptr!(fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint))
    fn_ptr!(fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetBooleanv(pname: GLenum, data: *mut GLboolean))
    fn_ptr!(fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut c_void))
    fn_ptr!(fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void))
    fn_ptr!(fn GetClipPlane(plane: GLenum, equation: *mut GLdouble))
    fn_ptr!(fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void))
    fn_ptr!(fn GetDoublev(pname: GLenum, data: *mut GLdouble))
    fn_ptr!(fn GetError() -> GLenum)
    fn_ptr!(fn GetFloatv(pname: GLenum, data: *mut GLfloat))
    fn_ptr!(fn GetIntegerv(pname: GLenum, data: *mut GLint))
    fn_ptr!(fn GetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetLightiv(light: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble))
    fn_ptr!(fn GetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat))
    fn_ptr!(fn GetMapiv(target: GLenum, query: GLenum, v: *mut GLint))
    fn_ptr!(fn GetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetMaterialiv(face: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetPixelMapfv(map: GLenum, values: *mut GLfloat))
    fn_ptr!(fn GetPixelMapuiv(map: GLenum, values: *mut GLuint))
    fn_ptr!(fn GetPixelMapusv(map: GLenum, values: *mut GLushort))
    fn_ptr!(fn GetPointerv(pname: GLenum, params: **mut c_void))
    fn_ptr!(fn GetPolygonStipple(mask: *mut GLubyte))
    fn_ptr!(fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar))
    fn_ptr!(fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetString(name: GLenum) -> *GLubyte)
    fn_ptr!(fn GetTexEnvfv(target: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexEnviv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetTexGenfv(coord: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexGeniv(coord: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void))
    fn_ptr!(fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat))
    fn_ptr!(fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint))
    fn_ptr!(fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut c_void))
    fn_ptr!(fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn Hint(target: GLenum, mode: GLenum))
    fn_ptr!(fn IndexMask(mask: GLuint))
    fn_ptr!(fn IndexPointer(type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn Indexd(c: GLdouble))
    fn_ptr!(fn Indexdv(c: *GLdouble))
    fn_ptr!(fn Indexf(c: GLfloat))
    fn_ptr!(fn Indexfv(c: *GLfloat))
    fn_ptr!(fn Indexi(c: GLint))
    fn_ptr!(fn Indexiv(c: *GLint))
    fn_ptr!(fn Indexs(c: GLshort))
    fn_ptr!(fn Indexsv(c: *GLshort))
    fn_ptr!(fn Indexub(c: GLubyte))
    fn_ptr!(fn Indexubv(c: *GLubyte))
    fn_ptr!(fn InitNames())
    fn_ptr!(fn InterleavedArrays(format: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn IsBuffer(buffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsEnabled(cap: GLenum) -> GLboolean)
    fn_ptr!(fn IsList(list: GLuint) -> GLboolean)
    fn_ptr!(fn IsProgram(program: GLuint) -> GLboolean)
    fn_ptr!(fn IsQuery(id: GLuint) -> GLboolean)
    fn_ptr!(fn IsShader(shader: GLuint) -> GLboolean)
    fn_ptr!(fn IsTexture(texture: GLuint) -> GLboolean)
    fn_ptr!(fn LightModelf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn LightModelfv(pname: GLenum, params: *GLfloat))
    fn_ptr!(fn LightModeli(pname: GLenum, param: GLint))
    fn_ptr!(fn LightModeliv(pname: GLenum, params: *GLint))
    fn_ptr!(fn Lightf(light: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn Lightfv(light: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn Lighti(light: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn Lightiv(light: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn LineStipple(factor: GLint, pattern: GLushort))
    fn_ptr!(fn LineWidth(width: GLfloat))
    fn_ptr!(fn LinkProgram(program: GLuint))
    fn_ptr!(fn ListBase(base: GLuint))
    fn_ptr!(fn LoadIdentity())
    fn_ptr!(fn LoadMatrixd(m: *GLdouble))
    fn_ptr!(fn LoadMatrixf(m: *GLfloat))
    fn_ptr!(fn LoadName(name: GLuint))
    fn_ptr!(fn LoadTransposeMatrixd(m: *GLdouble))
    fn_ptr!(fn LoadTransposeMatrixf(m: *GLfloat))
    fn_ptr!(fn LogicOp(opcode: GLenum))
    fn_ptr!(fn Map1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *GLdouble))
    fn_ptr!(fn Map1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *GLfloat))
    fn_ptr!(fn Map2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *GLdouble))
    fn_ptr!(fn Map2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *GLfloat))
    fn_ptr!(fn MapBuffer(target: GLenum, access: GLenum) -> *c_void)
    fn_ptr!(fn MapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble))
    fn_ptr!(fn MapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat))
    fn_ptr!(fn MapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble))
    fn_ptr!(fn MapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat))
    fn_ptr!(fn Materialf(face: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn Materialfv(face: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn Materiali(face: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn Materialiv(face: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn MatrixMode(mode: GLenum))
    fn_ptr!(fn MultMatrixd(m: *GLdouble))
    fn_ptr!(fn MultMatrixf(m: *GLfloat))
    fn_ptr!(fn MultTransposeMatrixd(m: *GLdouble))
    fn_ptr!(fn MultTransposeMatrixf(m: *GLfloat))
    fn_ptr!(fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei))
    fn_ptr!(fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **c_void, drawcount: GLsizei))
    fn_ptr!(fn MultiTexCoord1d(target: GLenum, s: GLdouble))
    fn_ptr!(fn MultiTexCoord1dv(target: GLenum, v: *GLdouble))
    fn_ptr!(fn MultiTexCoord1f(target: GLenum, s: GLfloat))
    fn_ptr!(fn MultiTexCoord1fv(target: GLenum, v: *GLfloat))
    fn_ptr!(fn MultiTexCoord1i(target: GLenum, s: GLint))
    fn_ptr!(fn MultiTexCoord1iv(target: GLenum, v: *GLint))
    fn_ptr!(fn MultiTexCoord1s(target: GLenum, s: GLshort))
    fn_ptr!(fn MultiTexCoord1sv(target: GLenum, v: *GLshort))
    fn_ptr!(fn MultiTexCoord2d(target: GLenum, s: GLdouble, t: GLdouble))
    fn_ptr!(fn MultiTexCoord2dv(target: GLenum, v: *GLdouble))
    fn_ptr!(fn MultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat))
    fn_ptr!(fn MultiTexCoord2fv(target: GLenum, v: *GLfloat))
    fn_ptr!(fn MultiTexCoord2i(target: GLenum, s: GLint, t: GLint))
    fn_ptr!(fn MultiTexCoord2iv(target: GLenum, v: *GLint))
    fn_ptr!(fn MultiTexCoord2s(target: GLenum, s: GLshort, t: GLshort))
    fn_ptr!(fn MultiTexCoord2sv(target: GLenum, v: *GLshort))
    fn_ptr!(fn MultiTexCoord3d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble))
    fn_ptr!(fn MultiTexCoord3dv(target: GLenum, v: *GLdouble))
    fn_ptr!(fn MultiTexCoord3f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat))
    fn_ptr!(fn MultiTexCoord3fv(target: GLenum, v: *GLfloat))
    fn_ptr!(fn MultiTexCoord3i(target: GLenum, s: GLint, t: GLint, r: GLint))
    fn_ptr!(fn MultiTexCoord3iv(target: GLenum, v: *GLint))
    fn_ptr!(fn MultiTexCoord3s(target: GLenum, s: GLshort, t: GLshort, r: GLshort))
    fn_ptr!(fn MultiTexCoord3sv(target: GLenum, v: *GLshort))
    fn_ptr!(fn MultiTexCoord4d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble))
    fn_ptr!(fn MultiTexCoord4dv(target: GLenum, v: *GLdouble))
    fn_ptr!(fn MultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat))
    fn_ptr!(fn MultiTexCoord4fv(target: GLenum, v: *GLfloat))
    fn_ptr!(fn MultiTexCoord4i(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint))
    fn_ptr!(fn MultiTexCoord4iv(target: GLenum, v: *GLint))
    fn_ptr!(fn MultiTexCoord4s(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort))
    fn_ptr!(fn MultiTexCoord4sv(target: GLenum, v: *GLshort))
    fn_ptr!(fn NewList(list: GLuint, mode: GLenum))
    fn_ptr!(fn Normal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte))
    fn_ptr!(fn Normal3bv(v: *GLbyte))
    fn_ptr!(fn Normal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble))
    fn_ptr!(fn Normal3dv(v: *GLdouble))
    fn_ptr!(fn Normal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat))
    fn_ptr!(fn Normal3fv(v: *GLfloat))
    fn_ptr!(fn Normal3i(nx: GLint, ny: GLint, nz: GLint))
    fn_ptr!(fn Normal3iv(v: *GLint))
    fn_ptr!(fn Normal3s(nx: GLshort, ny: GLshort, nz: GLshort))
    fn_ptr!(fn Normal3sv(v: *GLshort))
    fn_ptr!(fn NormalPointer(type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn Ortho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble))
    fn_ptr!(fn PassThrough(token: GLfloat))
    fn_ptr!(fn PixelMapfv(map: GLenum, mapsize: GLsizei, values: *GLfloat))
    fn_ptr!(fn PixelMapuiv(map: GLenum, mapsize: GLsizei, values: *GLuint))
    fn_ptr!(fn PixelMapusv(map: GLenum, mapsize: GLsizei, values: *GLushort))
    fn_ptr!(fn PixelStoref(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PixelStorei(pname: GLenum, param: GLint))
    fn_ptr!(fn PixelTransferf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PixelTransferi(pname: GLenum, param: GLint))
    fn_ptr!(fn PixelZoom(xfactor: GLfloat, yfactor: GLfloat))
    fn_ptr!(fn PointParameterf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PointParameterfv(pname: GLenum, params: *GLfloat))
    fn_ptr!(fn PointParameteri(pname: GLenum, param: GLint))
    fn_ptr!(fn PointParameteriv(pname: GLenum, params: *GLint))
    fn_ptr!(fn PointSize(size: GLfloat))
    fn_ptr!(fn PolygonMode(face: GLenum, mode: GLenum))
    fn_ptr!(fn PolygonOffset(factor: GLfloat, units: GLfloat))
    fn_ptr!(fn PolygonStipple(mask: *GLubyte))
    fn_ptr!(fn PopAttrib())
    fn_ptr!(fn PopClientAttrib())
    fn_ptr!(fn PopMatrix())
    fn_ptr!(fn PopName())
    fn_ptr!(fn PrioritizeTextures(n: GLsizei, textures: *GLuint, priorities: *GLfloat))
    fn_ptr!(fn PushAttrib(mask: GLbitfield))
    fn_ptr!(fn PushClientAttrib(mask: GLbitfield))
    fn_ptr!(fn PushMatrix())
    fn_ptr!(fn PushName(name: GLuint))
    fn_ptr!(fn RasterPos2d(x: GLdouble, y: GLdouble))
    fn_ptr!(fn RasterPos2dv(v: *GLdouble))
    fn_ptr!(fn RasterPos2f(x: GLfloat, y: GLfloat))
    fn_ptr!(fn RasterPos2fv(v: *GLfloat))
    fn_ptr!(fn RasterPos2i(x: GLint, y: GLint))
    fn_ptr!(fn RasterPos2iv(v: *GLint))
    fn_ptr!(fn RasterPos2s(x: GLshort, y: GLshort))
    fn_ptr!(fn RasterPos2sv(v: *GLshort))
    fn_ptr!(fn RasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn RasterPos3dv(v: *GLdouble))
    fn_ptr!(fn RasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn RasterPos3fv(v: *GLfloat))
    fn_ptr!(fn RasterPos3i(x: GLint, y: GLint, z: GLint))
    fn_ptr!(fn RasterPos3iv(v: *GLint))
    fn_ptr!(fn RasterPos3s(x: GLshort, y: GLshort, z: GLshort))
    fn_ptr!(fn RasterPos3sv(v: *GLshort))
    fn_ptr!(fn RasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn RasterPos4dv(v: *GLdouble))
    fn_ptr!(fn RasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat))
    fn_ptr!(fn RasterPos4fv(v: *GLfloat))
    fn_ptr!(fn RasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint))
    fn_ptr!(fn RasterPos4iv(v: *GLint))
    fn_ptr!(fn RasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort))
    fn_ptr!(fn RasterPos4sv(v: *GLshort))
    fn_ptr!(fn ReadBuffer(mode: GLenum))
    fn_ptr!(fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void))
    fn_ptr!(fn Rectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble))
    fn_ptr!(fn Rectdv(v1: *GLdouble, v2: *GLdouble))
    fn_ptr!(fn Rectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat))
    fn_ptr!(fn Rectfv(v1: *GLfloat, v2: *GLfloat))
    fn_ptr!(fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint))
    fn_ptr!(fn Rectiv(v1: *GLint, v2: *GLint))
    fn_ptr!(fn Rects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort))
    fn_ptr!(fn Rectsv(v1: *GLshort, v2: *GLshort))
    fn_ptr!(fn RenderMode(mode: GLenum) -> GLint)
    fn_ptr!(fn Rotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn Rotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn SampleCoverage(value: GLfloat, invert: GLboolean))
    fn_ptr!(fn Scaled(x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn Scalef(x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn SecondaryColor3b(red: GLbyte, green: GLbyte, blue: GLbyte))
    fn_ptr!(fn SecondaryColor3bv(v: *GLbyte))
    fn_ptr!(fn SecondaryColor3d(red: GLdouble, green: GLdouble, blue: GLdouble))
    fn_ptr!(fn SecondaryColor3dv(v: *GLdouble))
    fn_ptr!(fn SecondaryColor3f(red: GLfloat, green: GLfloat, blue: GLfloat))
    fn_ptr!(fn SecondaryColor3fv(v: *GLfloat))
    fn_ptr!(fn SecondaryColor3i(red: GLint, green: GLint, blue: GLint))
    fn_ptr!(fn SecondaryColor3iv(v: *GLint))
    fn_ptr!(fn SecondaryColor3s(red: GLshort, green: GLshort, blue: GLshort))
    fn_ptr!(fn SecondaryColor3sv(v: *GLshort))
    fn_ptr!(fn SecondaryColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte))
    fn_ptr!(fn SecondaryColor3ubv(v: *GLubyte))
    fn_ptr!(fn SecondaryColor3ui(red: GLuint, green: GLuint, blue: GLuint))
    fn_ptr!(fn SecondaryColor3uiv(v: *GLuint))
    fn_ptr!(fn SecondaryColor3us(red: GLushort, green: GLushort, blue: GLushort))
    fn_ptr!(fn SecondaryColor3usv(v: *GLushort))
    fn_ptr!(fn SecondaryColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn SelectBuffer(size: GLsizei, buffer: *mut GLuint))
    fn_ptr!(fn ShadeModel(mode: GLenum))
    fn_ptr!(fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint))
    fn_ptr!(fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilMask(mask: GLuint))
    fn_ptr!(fn StencilMaskSeparate(face: GLenum, mask: GLuint))
    fn_ptr!(fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum))
    fn_ptr!(fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum))
    fn_ptr!(fn TexCoord1d(s: GLdouble))
    fn_ptr!(fn TexCoord1dv(v: *GLdouble))
    fn_ptr!(fn TexCoord1f(s: GLfloat))
    fn_ptr!(fn TexCoord1fv(v: *GLfloat))
    fn_ptr!(fn TexCoord1i(s: GLint))
    fn_ptr!(fn TexCoord1iv(v: *GLint))
    fn_ptr!(fn TexCoord1s(s: GLshort))
    fn_ptr!(fn TexCoord1sv(v: *GLshort))
    fn_ptr!(fn TexCoord2d(s: GLdouble, t: GLdouble))
    fn_ptr!(fn TexCoord2dv(v: *GLdouble))
    fn_ptr!(fn TexCoord2f(s: GLfloat, t: GLfloat))
    fn_ptr!(fn TexCoord2fv(v: *GLfloat))
    fn_ptr!(fn TexCoord2i(s: GLint, t: GLint))
    fn_ptr!(fn TexCoord2iv(v: *GLint))
    fn_ptr!(fn TexCoord2s(s: GLshort, t: GLshort))
    fn_ptr!(fn TexCoord2sv(v: *GLshort))
    fn_ptr!(fn TexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble))
    fn_ptr!(fn TexCoord3dv(v: *GLdouble))
    fn_ptr!(fn TexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat))
    fn_ptr!(fn TexCoord3fv(v: *GLfloat))
    fn_ptr!(fn TexCoord3i(s: GLint, t: GLint, r: GLint))
    fn_ptr!(fn TexCoord3iv(v: *GLint))
    fn_ptr!(fn TexCoord3s(s: GLshort, t: GLshort, r: GLshort))
    fn_ptr!(fn TexCoord3sv(v: *GLshort))
    fn_ptr!(fn TexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble))
    fn_ptr!(fn TexCoord4dv(v: *GLdouble))
    fn_ptr!(fn TexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat))
    fn_ptr!(fn TexCoord4fv(v: *GLfloat))
    fn_ptr!(fn TexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint))
    fn_ptr!(fn TexCoord4iv(v: *GLint))
    fn_ptr!(fn TexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort))
    fn_ptr!(fn TexCoord4sv(v: *GLshort))
    fn_ptr!(fn TexCoordPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn TexEnvf(target: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn TexEnvfv(target: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn TexEnvi(target: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn TexEnviv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexGend(coord: GLenum, pname: GLenum, param: GLdouble))
    fn_ptr!(fn TexGendv(coord: GLenum, pname: GLenum, params: *GLdouble))
    fn_ptr!(fn TexGenf(coord: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn TexGenfv(coord: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn TexGeni(coord: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn TexGeniv(coord: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn TexParameteri(target: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *c_void))
    fn_ptr!(fn Translated(x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn Translatef(x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn Uniform1f(location: GLint, v0: GLfloat))
    fn_ptr!(fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform1i(location: GLint, v0: GLint))
    fn_ptr!(fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat))
    fn_ptr!(fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform2i(location: GLint, v0: GLint, v1: GLint))
    fn_ptr!(fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat))
    fn_ptr!(fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint))
    fn_ptr!(fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat))
    fn_ptr!(fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint))
    fn_ptr!(fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint))
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
    fn_ptr!(fn Vertex2d(x: GLdouble, y: GLdouble))
    fn_ptr!(fn Vertex2dv(v: *GLdouble))
    fn_ptr!(fn Vertex2f(x: GLfloat, y: GLfloat))
    fn_ptr!(fn Vertex2fv(v: *GLfloat))
    fn_ptr!(fn Vertex2i(x: GLint, y: GLint))
    fn_ptr!(fn Vertex2iv(v: *GLint))
    fn_ptr!(fn Vertex2s(x: GLshort, y: GLshort))
    fn_ptr!(fn Vertex2sv(v: *GLshort))
    fn_ptr!(fn Vertex3d(x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn Vertex3dv(v: *GLdouble))
    fn_ptr!(fn Vertex3f(x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn Vertex3fv(v: *GLfloat))
    fn_ptr!(fn Vertex3i(x: GLint, y: GLint, z: GLint))
    fn_ptr!(fn Vertex3iv(v: *GLint))
    fn_ptr!(fn Vertex3s(x: GLshort, y: GLshort, z: GLshort))
    fn_ptr!(fn Vertex3sv(v: *GLshort))
    fn_ptr!(fn Vertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn Vertex4dv(v: *GLdouble))
    fn_ptr!(fn Vertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat))
    fn_ptr!(fn Vertex4fv(v: *GLfloat))
    fn_ptr!(fn Vertex4i(x: GLint, y: GLint, z: GLint, w: GLint))
    fn_ptr!(fn Vertex4iv(v: *GLint))
    fn_ptr!(fn Vertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort))
    fn_ptr!(fn Vertex4sv(v: *GLshort))
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
    fn_ptr!(fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn VertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *c_void))
    fn_ptr!(fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn WindowPos2d(x: GLdouble, y: GLdouble))
    fn_ptr!(fn WindowPos2dv(v: *GLdouble))
    fn_ptr!(fn WindowPos2f(x: GLfloat, y: GLfloat))
    fn_ptr!(fn WindowPos2fv(v: *GLfloat))
    fn_ptr!(fn WindowPos2i(x: GLint, y: GLint))
    fn_ptr!(fn WindowPos2iv(v: *GLint))
    fn_ptr!(fn WindowPos2s(x: GLshort, y: GLshort))
    fn_ptr!(fn WindowPos2sv(v: *GLshort))
    fn_ptr!(fn WindowPos3d(x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn WindowPos3dv(v: *GLdouble))
    fn_ptr!(fn WindowPos3f(x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn WindowPos3fv(v: *GLfloat))
    fn_ptr!(fn WindowPos3i(x: GLint, y: GLint, z: GLint))
    fn_ptr!(fn WindowPos3iv(v: *GLint))
    fn_ptr!(fn WindowPos3s(x: GLshort, y: GLshort, z: GLshort))
    fn_ptr!(fn WindowPos3sv(v: *GLshort))
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

fn_mod!(Accum, "glAccum")
fn_mod!(ActiveTexture, "glActiveTexture")
fn_mod!(AlphaFunc, "glAlphaFunc")
fn_mod!(AreTexturesResident, "glAreTexturesResident")
fn_mod!(ArrayElement, "glArrayElement")
fn_mod!(AttachShader, "glAttachShader")
fn_mod!(Begin, "glBegin")
fn_mod!(BeginQuery, "glBeginQuery")
fn_mod!(BindAttribLocation, "glBindAttribLocation")
fn_mod!(BindBuffer, "glBindBuffer")
fn_mod!(BindTexture, "glBindTexture")
fn_mod!(Bitmap, "glBitmap")
fn_mod!(BlendColor, "glBlendColor")
fn_mod!(BlendEquation, "glBlendEquation")
fn_mod!(BlendEquationSeparate, "glBlendEquationSeparate")
fn_mod!(BlendFunc, "glBlendFunc")
fn_mod!(BlendFuncSeparate, "glBlendFuncSeparate")
fn_mod!(BufferData, "glBufferData")
fn_mod!(BufferSubData, "glBufferSubData")
fn_mod!(CallList, "glCallList")
fn_mod!(CallLists, "glCallLists")
fn_mod!(Clear, "glClear")
fn_mod!(ClearAccum, "glClearAccum")
fn_mod!(ClearColor, "glClearColor")
fn_mod!(ClearDepth, "glClearDepth")
fn_mod!(ClearIndex, "glClearIndex")
fn_mod!(ClearStencil, "glClearStencil")
fn_mod!(ClientActiveTexture, "glClientActiveTexture")
fn_mod!(ClipPlane, "glClipPlane")
fn_mod!(Color3b, "glColor3b")
fn_mod!(Color3bv, "glColor3bv")
fn_mod!(Color3d, "glColor3d")
fn_mod!(Color3dv, "glColor3dv")
fn_mod!(Color3f, "glColor3f")
fn_mod!(Color3fv, "glColor3fv")
fn_mod!(Color3i, "glColor3i")
fn_mod!(Color3iv, "glColor3iv")
fn_mod!(Color3s, "glColor3s")
fn_mod!(Color3sv, "glColor3sv")
fn_mod!(Color3ub, "glColor3ub")
fn_mod!(Color3ubv, "glColor3ubv")
fn_mod!(Color3ui, "glColor3ui")
fn_mod!(Color3uiv, "glColor3uiv")
fn_mod!(Color3us, "glColor3us")
fn_mod!(Color3usv, "glColor3usv")
fn_mod!(Color4b, "glColor4b")
fn_mod!(Color4bv, "glColor4bv")
fn_mod!(Color4d, "glColor4d")
fn_mod!(Color4dv, "glColor4dv")
fn_mod!(Color4f, "glColor4f")
fn_mod!(Color4fv, "glColor4fv")
fn_mod!(Color4i, "glColor4i")
fn_mod!(Color4iv, "glColor4iv")
fn_mod!(Color4s, "glColor4s")
fn_mod!(Color4sv, "glColor4sv")
fn_mod!(Color4ub, "glColor4ub")
fn_mod!(Color4ubv, "glColor4ubv")
fn_mod!(Color4ui, "glColor4ui")
fn_mod!(Color4uiv, "glColor4uiv")
fn_mod!(Color4us, "glColor4us")
fn_mod!(Color4usv, "glColor4usv")
fn_mod!(ColorMask, "glColorMask")
fn_mod!(ColorMaterial, "glColorMaterial")
fn_mod!(ColorPointer, "glColorPointer")
fn_mod!(CompileShader, "glCompileShader")
fn_mod!(CompressedTexImage1D, "glCompressedTexImage1D")
fn_mod!(CompressedTexImage2D, "glCompressedTexImage2D")
fn_mod!(CompressedTexImage3D, "glCompressedTexImage3D")
fn_mod!(CompressedTexSubImage1D, "glCompressedTexSubImage1D")
fn_mod!(CompressedTexSubImage2D, "glCompressedTexSubImage2D")
fn_mod!(CompressedTexSubImage3D, "glCompressedTexSubImage3D")
fn_mod!(CopyPixels, "glCopyPixels")
fn_mod!(CopyTexImage1D, "glCopyTexImage1D")
fn_mod!(CopyTexImage2D, "glCopyTexImage2D")
fn_mod!(CopyTexSubImage1D, "glCopyTexSubImage1D")
fn_mod!(CopyTexSubImage2D, "glCopyTexSubImage2D")
fn_mod!(CopyTexSubImage3D, "glCopyTexSubImage3D")
fn_mod!(CreateProgram, "glCreateProgram")
fn_mod!(CreateShader, "glCreateShader")
fn_mod!(CullFace, "glCullFace")
fn_mod!(DeleteBuffers, "glDeleteBuffers")
fn_mod!(DeleteLists, "glDeleteLists")
fn_mod!(DeleteProgram, "glDeleteProgram")
fn_mod!(DeleteQueries, "glDeleteQueries")
fn_mod!(DeleteShader, "glDeleteShader")
fn_mod!(DeleteTextures, "glDeleteTextures")
fn_mod!(DepthFunc, "glDepthFunc")
fn_mod!(DepthMask, "glDepthMask")
fn_mod!(DepthRange, "glDepthRange")
fn_mod!(DetachShader, "glDetachShader")
fn_mod!(Disable, "glDisable")
fn_mod!(DisableClientState, "glDisableClientState")
fn_mod!(DisableVertexAttribArray, "glDisableVertexAttribArray")
fn_mod!(DrawArrays, "glDrawArrays")
fn_mod!(DrawBuffer, "glDrawBuffer")
fn_mod!(DrawBuffers, "glDrawBuffers")
fn_mod!(DrawElements, "glDrawElements")
fn_mod!(DrawPixels, "glDrawPixels")
fn_mod!(DrawRangeElements, "glDrawRangeElements")
fn_mod!(EdgeFlag, "glEdgeFlag")
fn_mod!(EdgeFlagPointer, "glEdgeFlagPointer")
fn_mod!(EdgeFlagv, "glEdgeFlagv")
fn_mod!(Enable, "glEnable")
fn_mod!(EnableClientState, "glEnableClientState")
fn_mod!(EnableVertexAttribArray, "glEnableVertexAttribArray")
fn_mod!(End, "glEnd")
fn_mod!(EndList, "glEndList")
fn_mod!(EndQuery, "glEndQuery")
fn_mod!(EvalCoord1d, "glEvalCoord1d")
fn_mod!(EvalCoord1dv, "glEvalCoord1dv")
fn_mod!(EvalCoord1f, "glEvalCoord1f")
fn_mod!(EvalCoord1fv, "glEvalCoord1fv")
fn_mod!(EvalCoord2d, "glEvalCoord2d")
fn_mod!(EvalCoord2dv, "glEvalCoord2dv")
fn_mod!(EvalCoord2f, "glEvalCoord2f")
fn_mod!(EvalCoord2fv, "glEvalCoord2fv")
fn_mod!(EvalMesh1, "glEvalMesh1")
fn_mod!(EvalMesh2, "glEvalMesh2")
fn_mod!(EvalPoint1, "glEvalPoint1")
fn_mod!(EvalPoint2, "glEvalPoint2")
fn_mod!(FeedbackBuffer, "glFeedbackBuffer")
fn_mod!(Finish, "glFinish")
fn_mod!(Flush, "glFlush")
fn_mod!(FogCoordPointer, "glFogCoordPointer")
fn_mod!(FogCoordd, "glFogCoordd")
fn_mod!(FogCoorddv, "glFogCoorddv")
fn_mod!(FogCoordf, "glFogCoordf")
fn_mod!(FogCoordfv, "glFogCoordfv")
fn_mod!(Fogf, "glFogf")
fn_mod!(Fogfv, "glFogfv")
fn_mod!(Fogi, "glFogi")
fn_mod!(Fogiv, "glFogiv")
fn_mod!(FrontFace, "glFrontFace")
fn_mod!(Frustum, "glFrustum")
fn_mod!(GenBuffers, "glGenBuffers")
fn_mod!(GenLists, "glGenLists")
fn_mod!(GenQueries, "glGenQueries")
fn_mod!(GenTextures, "glGenTextures")
fn_mod!(GetActiveAttrib, "glGetActiveAttrib")
fn_mod!(GetActiveUniform, "glGetActiveUniform")
fn_mod!(GetAttachedShaders, "glGetAttachedShaders")
fn_mod!(GetAttribLocation, "glGetAttribLocation")
fn_mod!(GetBooleanv, "glGetBooleanv")
fn_mod!(GetBufferParameteriv, "glGetBufferParameteriv")
fn_mod!(GetBufferPointerv, "glGetBufferPointerv")
fn_mod!(GetBufferSubData, "glGetBufferSubData")
fn_mod!(GetClipPlane, "glGetClipPlane")
fn_mod!(GetCompressedTexImage, "glGetCompressedTexImage")
fn_mod!(GetDoublev, "glGetDoublev")
fn_mod!(GetError, "glGetError")
fn_mod!(GetFloatv, "glGetFloatv")
fn_mod!(GetIntegerv, "glGetIntegerv")
fn_mod!(GetLightfv, "glGetLightfv")
fn_mod!(GetLightiv, "glGetLightiv")
fn_mod!(GetMapdv, "glGetMapdv")
fn_mod!(GetMapfv, "glGetMapfv")
fn_mod!(GetMapiv, "glGetMapiv")
fn_mod!(GetMaterialfv, "glGetMaterialfv")
fn_mod!(GetMaterialiv, "glGetMaterialiv")
fn_mod!(GetPixelMapfv, "glGetPixelMapfv")
fn_mod!(GetPixelMapuiv, "glGetPixelMapuiv")
fn_mod!(GetPixelMapusv, "glGetPixelMapusv")
fn_mod!(GetPointerv, "glGetPointerv")
fn_mod!(GetPolygonStipple, "glGetPolygonStipple")
fn_mod!(GetProgramInfoLog, "glGetProgramInfoLog")
fn_mod!(GetProgramiv, "glGetProgramiv")
fn_mod!(GetQueryObjectiv, "glGetQueryObjectiv")
fn_mod!(GetQueryObjectuiv, "glGetQueryObjectuiv")
fn_mod!(GetQueryiv, "glGetQueryiv")
fn_mod!(GetShaderInfoLog, "glGetShaderInfoLog")
fn_mod!(GetShaderSource, "glGetShaderSource")
fn_mod!(GetShaderiv, "glGetShaderiv")
fn_mod!(GetString, "glGetString")
fn_mod!(GetTexEnvfv, "glGetTexEnvfv")
fn_mod!(GetTexEnviv, "glGetTexEnviv")
fn_mod!(GetTexGendv, "glGetTexGendv")
fn_mod!(GetTexGenfv, "glGetTexGenfv")
fn_mod!(GetTexGeniv, "glGetTexGeniv")
fn_mod!(GetTexImage, "glGetTexImage")
fn_mod!(GetTexLevelParameterfv, "glGetTexLevelParameterfv")
fn_mod!(GetTexLevelParameteriv, "glGetTexLevelParameteriv")
fn_mod!(GetTexParameterfv, "glGetTexParameterfv")
fn_mod!(GetTexParameteriv, "glGetTexParameteriv")
fn_mod!(GetUniformLocation, "glGetUniformLocation")
fn_mod!(GetUniformfv, "glGetUniformfv")
fn_mod!(GetUniformiv, "glGetUniformiv")
fn_mod!(GetVertexAttribPointerv, "glGetVertexAttribPointerv")
fn_mod!(GetVertexAttribdv, "glGetVertexAttribdv")
fn_mod!(GetVertexAttribfv, "glGetVertexAttribfv")
fn_mod!(GetVertexAttribiv, "glGetVertexAttribiv")
fn_mod!(Hint, "glHint")
fn_mod!(IndexMask, "glIndexMask")
fn_mod!(IndexPointer, "glIndexPointer")
fn_mod!(Indexd, "glIndexd")
fn_mod!(Indexdv, "glIndexdv")
fn_mod!(Indexf, "glIndexf")
fn_mod!(Indexfv, "glIndexfv")
fn_mod!(Indexi, "glIndexi")
fn_mod!(Indexiv, "glIndexiv")
fn_mod!(Indexs, "glIndexs")
fn_mod!(Indexsv, "glIndexsv")
fn_mod!(Indexub, "glIndexub")
fn_mod!(Indexubv, "glIndexubv")
fn_mod!(InitNames, "glInitNames")
fn_mod!(InterleavedArrays, "glInterleavedArrays")
fn_mod!(IsBuffer, "glIsBuffer")
fn_mod!(IsEnabled, "glIsEnabled")
fn_mod!(IsList, "glIsList")
fn_mod!(IsProgram, "glIsProgram")
fn_mod!(IsQuery, "glIsQuery")
fn_mod!(IsShader, "glIsShader")
fn_mod!(IsTexture, "glIsTexture")
fn_mod!(LightModelf, "glLightModelf")
fn_mod!(LightModelfv, "glLightModelfv")
fn_mod!(LightModeli, "glLightModeli")
fn_mod!(LightModeliv, "glLightModeliv")
fn_mod!(Lightf, "glLightf")
fn_mod!(Lightfv, "glLightfv")
fn_mod!(Lighti, "glLighti")
fn_mod!(Lightiv, "glLightiv")
fn_mod!(LineStipple, "glLineStipple")
fn_mod!(LineWidth, "glLineWidth")
fn_mod!(LinkProgram, "glLinkProgram")
fn_mod!(ListBase, "glListBase")
fn_mod!(LoadIdentity, "glLoadIdentity")
fn_mod!(LoadMatrixd, "glLoadMatrixd")
fn_mod!(LoadMatrixf, "glLoadMatrixf")
fn_mod!(LoadName, "glLoadName")
fn_mod!(LoadTransposeMatrixd, "glLoadTransposeMatrixd")
fn_mod!(LoadTransposeMatrixf, "glLoadTransposeMatrixf")
fn_mod!(LogicOp, "glLogicOp")
fn_mod!(Map1d, "glMap1d")
fn_mod!(Map1f, "glMap1f")
fn_mod!(Map2d, "glMap2d")
fn_mod!(Map2f, "glMap2f")
fn_mod!(MapBuffer, "glMapBuffer")
fn_mod!(MapGrid1d, "glMapGrid1d")
fn_mod!(MapGrid1f, "glMapGrid1f")
fn_mod!(MapGrid2d, "glMapGrid2d")
fn_mod!(MapGrid2f, "glMapGrid2f")
fn_mod!(Materialf, "glMaterialf")
fn_mod!(Materialfv, "glMaterialfv")
fn_mod!(Materiali, "glMateriali")
fn_mod!(Materialiv, "glMaterialiv")
fn_mod!(MatrixMode, "glMatrixMode")
fn_mod!(MultMatrixd, "glMultMatrixd")
fn_mod!(MultMatrixf, "glMultMatrixf")
fn_mod!(MultTransposeMatrixd, "glMultTransposeMatrixd")
fn_mod!(MultTransposeMatrixf, "glMultTransposeMatrixf")
fn_mod!(MultiDrawArrays, "glMultiDrawArrays")
fn_mod!(MultiDrawElements, "glMultiDrawElements")
fn_mod!(MultiTexCoord1d, "glMultiTexCoord1d")
fn_mod!(MultiTexCoord1dv, "glMultiTexCoord1dv")
fn_mod!(MultiTexCoord1f, "glMultiTexCoord1f")
fn_mod!(MultiTexCoord1fv, "glMultiTexCoord1fv")
fn_mod!(MultiTexCoord1i, "glMultiTexCoord1i")
fn_mod!(MultiTexCoord1iv, "glMultiTexCoord1iv")
fn_mod!(MultiTexCoord1s, "glMultiTexCoord1s")
fn_mod!(MultiTexCoord1sv, "glMultiTexCoord1sv")
fn_mod!(MultiTexCoord2d, "glMultiTexCoord2d")
fn_mod!(MultiTexCoord2dv, "glMultiTexCoord2dv")
fn_mod!(MultiTexCoord2f, "glMultiTexCoord2f")
fn_mod!(MultiTexCoord2fv, "glMultiTexCoord2fv")
fn_mod!(MultiTexCoord2i, "glMultiTexCoord2i")
fn_mod!(MultiTexCoord2iv, "glMultiTexCoord2iv")
fn_mod!(MultiTexCoord2s, "glMultiTexCoord2s")
fn_mod!(MultiTexCoord2sv, "glMultiTexCoord2sv")
fn_mod!(MultiTexCoord3d, "glMultiTexCoord3d")
fn_mod!(MultiTexCoord3dv, "glMultiTexCoord3dv")
fn_mod!(MultiTexCoord3f, "glMultiTexCoord3f")
fn_mod!(MultiTexCoord3fv, "glMultiTexCoord3fv")
fn_mod!(MultiTexCoord3i, "glMultiTexCoord3i")
fn_mod!(MultiTexCoord3iv, "glMultiTexCoord3iv")
fn_mod!(MultiTexCoord3s, "glMultiTexCoord3s")
fn_mod!(MultiTexCoord3sv, "glMultiTexCoord3sv")
fn_mod!(MultiTexCoord4d, "glMultiTexCoord4d")
fn_mod!(MultiTexCoord4dv, "glMultiTexCoord4dv")
fn_mod!(MultiTexCoord4f, "glMultiTexCoord4f")
fn_mod!(MultiTexCoord4fv, "glMultiTexCoord4fv")
fn_mod!(MultiTexCoord4i, "glMultiTexCoord4i")
fn_mod!(MultiTexCoord4iv, "glMultiTexCoord4iv")
fn_mod!(MultiTexCoord4s, "glMultiTexCoord4s")
fn_mod!(MultiTexCoord4sv, "glMultiTexCoord4sv")
fn_mod!(NewList, "glNewList")
fn_mod!(Normal3b, "glNormal3b")
fn_mod!(Normal3bv, "glNormal3bv")
fn_mod!(Normal3d, "glNormal3d")
fn_mod!(Normal3dv, "glNormal3dv")
fn_mod!(Normal3f, "glNormal3f")
fn_mod!(Normal3fv, "glNormal3fv")
fn_mod!(Normal3i, "glNormal3i")
fn_mod!(Normal3iv, "glNormal3iv")
fn_mod!(Normal3s, "glNormal3s")
fn_mod!(Normal3sv, "glNormal3sv")
fn_mod!(NormalPointer, "glNormalPointer")
fn_mod!(Ortho, "glOrtho")
fn_mod!(PassThrough, "glPassThrough")
fn_mod!(PixelMapfv, "glPixelMapfv")
fn_mod!(PixelMapuiv, "glPixelMapuiv")
fn_mod!(PixelMapusv, "glPixelMapusv")
fn_mod!(PixelStoref, "glPixelStoref")
fn_mod!(PixelStorei, "glPixelStorei")
fn_mod!(PixelTransferf, "glPixelTransferf")
fn_mod!(PixelTransferi, "glPixelTransferi")
fn_mod!(PixelZoom, "glPixelZoom")
fn_mod!(PointParameterf, "glPointParameterf")
fn_mod!(PointParameterfv, "glPointParameterfv")
fn_mod!(PointParameteri, "glPointParameteri")
fn_mod!(PointParameteriv, "glPointParameteriv")
fn_mod!(PointSize, "glPointSize")
fn_mod!(PolygonMode, "glPolygonMode")
fn_mod!(PolygonOffset, "glPolygonOffset")
fn_mod!(PolygonStipple, "glPolygonStipple")
fn_mod!(PopAttrib, "glPopAttrib")
fn_mod!(PopClientAttrib, "glPopClientAttrib")
fn_mod!(PopMatrix, "glPopMatrix")
fn_mod!(PopName, "glPopName")
fn_mod!(PrioritizeTextures, "glPrioritizeTextures")
fn_mod!(PushAttrib, "glPushAttrib")
fn_mod!(PushClientAttrib, "glPushClientAttrib")
fn_mod!(PushMatrix, "glPushMatrix")
fn_mod!(PushName, "glPushName")
fn_mod!(RasterPos2d, "glRasterPos2d")
fn_mod!(RasterPos2dv, "glRasterPos2dv")
fn_mod!(RasterPos2f, "glRasterPos2f")
fn_mod!(RasterPos2fv, "glRasterPos2fv")
fn_mod!(RasterPos2i, "glRasterPos2i")
fn_mod!(RasterPos2iv, "glRasterPos2iv")
fn_mod!(RasterPos2s, "glRasterPos2s")
fn_mod!(RasterPos2sv, "glRasterPos2sv")
fn_mod!(RasterPos3d, "glRasterPos3d")
fn_mod!(RasterPos3dv, "glRasterPos3dv")
fn_mod!(RasterPos3f, "glRasterPos3f")
fn_mod!(RasterPos3fv, "glRasterPos3fv")
fn_mod!(RasterPos3i, "glRasterPos3i")
fn_mod!(RasterPos3iv, "glRasterPos3iv")
fn_mod!(RasterPos3s, "glRasterPos3s")
fn_mod!(RasterPos3sv, "glRasterPos3sv")
fn_mod!(RasterPos4d, "glRasterPos4d")
fn_mod!(RasterPos4dv, "glRasterPos4dv")
fn_mod!(RasterPos4f, "glRasterPos4f")
fn_mod!(RasterPos4fv, "glRasterPos4fv")
fn_mod!(RasterPos4i, "glRasterPos4i")
fn_mod!(RasterPos4iv, "glRasterPos4iv")
fn_mod!(RasterPos4s, "glRasterPos4s")
fn_mod!(RasterPos4sv, "glRasterPos4sv")
fn_mod!(ReadBuffer, "glReadBuffer")
fn_mod!(ReadPixels, "glReadPixels")
fn_mod!(Rectd, "glRectd")
fn_mod!(Rectdv, "glRectdv")
fn_mod!(Rectf, "glRectf")
fn_mod!(Rectfv, "glRectfv")
fn_mod!(Recti, "glRecti")
fn_mod!(Rectiv, "glRectiv")
fn_mod!(Rects, "glRects")
fn_mod!(Rectsv, "glRectsv")
fn_mod!(RenderMode, "glRenderMode")
fn_mod!(Rotated, "glRotated")
fn_mod!(Rotatef, "glRotatef")
fn_mod!(SampleCoverage, "glSampleCoverage")
fn_mod!(Scaled, "glScaled")
fn_mod!(Scalef, "glScalef")
fn_mod!(Scissor, "glScissor")
fn_mod!(SecondaryColor3b, "glSecondaryColor3b")
fn_mod!(SecondaryColor3bv, "glSecondaryColor3bv")
fn_mod!(SecondaryColor3d, "glSecondaryColor3d")
fn_mod!(SecondaryColor3dv, "glSecondaryColor3dv")
fn_mod!(SecondaryColor3f, "glSecondaryColor3f")
fn_mod!(SecondaryColor3fv, "glSecondaryColor3fv")
fn_mod!(SecondaryColor3i, "glSecondaryColor3i")
fn_mod!(SecondaryColor3iv, "glSecondaryColor3iv")
fn_mod!(SecondaryColor3s, "glSecondaryColor3s")
fn_mod!(SecondaryColor3sv, "glSecondaryColor3sv")
fn_mod!(SecondaryColor3ub, "glSecondaryColor3ub")
fn_mod!(SecondaryColor3ubv, "glSecondaryColor3ubv")
fn_mod!(SecondaryColor3ui, "glSecondaryColor3ui")
fn_mod!(SecondaryColor3uiv, "glSecondaryColor3uiv")
fn_mod!(SecondaryColor3us, "glSecondaryColor3us")
fn_mod!(SecondaryColor3usv, "glSecondaryColor3usv")
fn_mod!(SecondaryColorPointer, "glSecondaryColorPointer")
fn_mod!(SelectBuffer, "glSelectBuffer")
fn_mod!(ShadeModel, "glShadeModel")
fn_mod!(ShaderSource, "glShaderSource")
fn_mod!(StencilFunc, "glStencilFunc")
fn_mod!(StencilFuncSeparate, "glStencilFuncSeparate")
fn_mod!(StencilMask, "glStencilMask")
fn_mod!(StencilMaskSeparate, "glStencilMaskSeparate")
fn_mod!(StencilOp, "glStencilOp")
fn_mod!(StencilOpSeparate, "glStencilOpSeparate")
fn_mod!(TexCoord1d, "glTexCoord1d")
fn_mod!(TexCoord1dv, "glTexCoord1dv")
fn_mod!(TexCoord1f, "glTexCoord1f")
fn_mod!(TexCoord1fv, "glTexCoord1fv")
fn_mod!(TexCoord1i, "glTexCoord1i")
fn_mod!(TexCoord1iv, "glTexCoord1iv")
fn_mod!(TexCoord1s, "glTexCoord1s")
fn_mod!(TexCoord1sv, "glTexCoord1sv")
fn_mod!(TexCoord2d, "glTexCoord2d")
fn_mod!(TexCoord2dv, "glTexCoord2dv")
fn_mod!(TexCoord2f, "glTexCoord2f")
fn_mod!(TexCoord2fv, "glTexCoord2fv")
fn_mod!(TexCoord2i, "glTexCoord2i")
fn_mod!(TexCoord2iv, "glTexCoord2iv")
fn_mod!(TexCoord2s, "glTexCoord2s")
fn_mod!(TexCoord2sv, "glTexCoord2sv")
fn_mod!(TexCoord3d, "glTexCoord3d")
fn_mod!(TexCoord3dv, "glTexCoord3dv")
fn_mod!(TexCoord3f, "glTexCoord3f")
fn_mod!(TexCoord3fv, "glTexCoord3fv")
fn_mod!(TexCoord3i, "glTexCoord3i")
fn_mod!(TexCoord3iv, "glTexCoord3iv")
fn_mod!(TexCoord3s, "glTexCoord3s")
fn_mod!(TexCoord3sv, "glTexCoord3sv")
fn_mod!(TexCoord4d, "glTexCoord4d")
fn_mod!(TexCoord4dv, "glTexCoord4dv")
fn_mod!(TexCoord4f, "glTexCoord4f")
fn_mod!(TexCoord4fv, "glTexCoord4fv")
fn_mod!(TexCoord4i, "glTexCoord4i")
fn_mod!(TexCoord4iv, "glTexCoord4iv")
fn_mod!(TexCoord4s, "glTexCoord4s")
fn_mod!(TexCoord4sv, "glTexCoord4sv")
fn_mod!(TexCoordPointer, "glTexCoordPointer")
fn_mod!(TexEnvf, "glTexEnvf")
fn_mod!(TexEnvfv, "glTexEnvfv")
fn_mod!(TexEnvi, "glTexEnvi")
fn_mod!(TexEnviv, "glTexEnviv")
fn_mod!(TexGend, "glTexGend")
fn_mod!(TexGendv, "glTexGendv")
fn_mod!(TexGenf, "glTexGenf")
fn_mod!(TexGenfv, "glTexGenfv")
fn_mod!(TexGeni, "glTexGeni")
fn_mod!(TexGeniv, "glTexGeniv")
fn_mod!(TexImage1D, "glTexImage1D")
fn_mod!(TexImage2D, "glTexImage2D")
fn_mod!(TexImage3D, "glTexImage3D")
fn_mod!(TexParameterf, "glTexParameterf")
fn_mod!(TexParameterfv, "glTexParameterfv")
fn_mod!(TexParameteri, "glTexParameteri")
fn_mod!(TexParameteriv, "glTexParameteriv")
fn_mod!(TexSubImage1D, "glTexSubImage1D")
fn_mod!(TexSubImage2D, "glTexSubImage2D")
fn_mod!(TexSubImage3D, "glTexSubImage3D")
fn_mod!(Translated, "glTranslated")
fn_mod!(Translatef, "glTranslatef")
fn_mod!(Uniform1f, "glUniform1f")
fn_mod!(Uniform1fv, "glUniform1fv")
fn_mod!(Uniform1i, "glUniform1i")
fn_mod!(Uniform1iv, "glUniform1iv")
fn_mod!(Uniform2f, "glUniform2f")
fn_mod!(Uniform2fv, "glUniform2fv")
fn_mod!(Uniform2i, "glUniform2i")
fn_mod!(Uniform2iv, "glUniform2iv")
fn_mod!(Uniform3f, "glUniform3f")
fn_mod!(Uniform3fv, "glUniform3fv")
fn_mod!(Uniform3i, "glUniform3i")
fn_mod!(Uniform3iv, "glUniform3iv")
fn_mod!(Uniform4f, "glUniform4f")
fn_mod!(Uniform4fv, "glUniform4fv")
fn_mod!(Uniform4i, "glUniform4i")
fn_mod!(Uniform4iv, "glUniform4iv")
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
fn_mod!(Vertex2d, "glVertex2d")
fn_mod!(Vertex2dv, "glVertex2dv")
fn_mod!(Vertex2f, "glVertex2f")
fn_mod!(Vertex2fv, "glVertex2fv")
fn_mod!(Vertex2i, "glVertex2i")
fn_mod!(Vertex2iv, "glVertex2iv")
fn_mod!(Vertex2s, "glVertex2s")
fn_mod!(Vertex2sv, "glVertex2sv")
fn_mod!(Vertex3d, "glVertex3d")
fn_mod!(Vertex3dv, "glVertex3dv")
fn_mod!(Vertex3f, "glVertex3f")
fn_mod!(Vertex3fv, "glVertex3fv")
fn_mod!(Vertex3i, "glVertex3i")
fn_mod!(Vertex3iv, "glVertex3iv")
fn_mod!(Vertex3s, "glVertex3s")
fn_mod!(Vertex3sv, "glVertex3sv")
fn_mod!(Vertex4d, "glVertex4d")
fn_mod!(Vertex4dv, "glVertex4dv")
fn_mod!(Vertex4f, "glVertex4f")
fn_mod!(Vertex4fv, "glVertex4fv")
fn_mod!(Vertex4i, "glVertex4i")
fn_mod!(Vertex4iv, "glVertex4iv")
fn_mod!(Vertex4s, "glVertex4s")
fn_mod!(Vertex4sv, "glVertex4sv")
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
fn_mod!(VertexAttribPointer, "glVertexAttribPointer")
fn_mod!(VertexPointer, "glVertexPointer")
fn_mod!(Viewport, "glViewport")
fn_mod!(WindowPos2d, "glWindowPos2d")
fn_mod!(WindowPos2dv, "glWindowPos2dv")
fn_mod!(WindowPos2f, "glWindowPos2f")
fn_mod!(WindowPos2fv, "glWindowPos2fv")
fn_mod!(WindowPos2i, "glWindowPos2i")
fn_mod!(WindowPos2iv, "glWindowPos2iv")
fn_mod!(WindowPos2s, "glWindowPos2s")
fn_mod!(WindowPos2sv, "glWindowPos2sv")
fn_mod!(WindowPos3d, "glWindowPos3d")
fn_mod!(WindowPos3dv, "glWindowPos3dv")
fn_mod!(WindowPos3f, "glWindowPos3f")
fn_mod!(WindowPos3fv, "glWindowPos3fv")
fn_mod!(WindowPos3i, "glWindowPos3i")
fn_mod!(WindowPos3iv, "glWindowPos3iv")
fn_mod!(WindowPos3s, "glWindowPos3s")
fn_mod!(WindowPos3sv, "glWindowPos3sv")

mod failing {
    use std::libc::*;
    use super::types::*;
    
    macro_rules! failing(
        (fn $name:ident()) => (pub extern "system" fn $name() { fail!(stringify!($name was not loaded)) });
        (fn $name:ident() -> $ret_ty:ty) => (pub extern "system" fn $name() -> $ret_ty { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*)) => (pub extern "system" fn $name($(_: $arg_ty),*) { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*) -> $ret_ty:ty) => (pub extern "system" fn $name($(_: $arg_ty),*) -> $ret_ty { fail!(stringify!($name was not loaded)) });
    )
    
    failing!(fn Accum(GLenum, GLfloat))
    failing!(fn ActiveTexture(GLenum))
    failing!(fn AlphaFunc(GLenum, GLfloat))
    failing!(fn AreTexturesResident(GLsizei, *GLuint, *mut GLboolean) -> GLboolean)
    failing!(fn ArrayElement(GLint))
    failing!(fn AttachShader(GLuint, GLuint))
    failing!(fn Begin(GLenum))
    failing!(fn BeginQuery(GLenum, GLuint))
    failing!(fn BindAttribLocation(GLuint, GLuint, *GLchar))
    failing!(fn BindBuffer(GLenum, GLuint))
    failing!(fn BindTexture(GLenum, GLuint))
    failing!(fn Bitmap(GLsizei, GLsizei, GLfloat, GLfloat, GLfloat, GLfloat, *GLubyte))
    failing!(fn BlendColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn BlendEquation(GLenum))
    failing!(fn BlendEquationSeparate(GLenum, GLenum))
    failing!(fn BlendFunc(GLenum, GLenum))
    failing!(fn BlendFuncSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn BufferData(GLenum, GLsizeiptr, *c_void, GLenum))
    failing!(fn BufferSubData(GLenum, GLintptr, GLsizeiptr, *c_void))
    failing!(fn CallList(GLuint))
    failing!(fn CallLists(GLsizei, GLenum, *c_void))
    failing!(fn Clear(GLbitfield))
    failing!(fn ClearAccum(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ClearColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ClearDepth(GLdouble))
    failing!(fn ClearIndex(GLfloat))
    failing!(fn ClearStencil(GLint))
    failing!(fn ClientActiveTexture(GLenum))
    failing!(fn ClipPlane(GLenum, *GLdouble))
    failing!(fn Color3b(GLbyte, GLbyte, GLbyte))
    failing!(fn Color3bv(*GLbyte))
    failing!(fn Color3d(GLdouble, GLdouble, GLdouble))
    failing!(fn Color3dv(*GLdouble))
    failing!(fn Color3f(GLfloat, GLfloat, GLfloat))
    failing!(fn Color3fv(*GLfloat))
    failing!(fn Color3i(GLint, GLint, GLint))
    failing!(fn Color3iv(*GLint))
    failing!(fn Color3s(GLshort, GLshort, GLshort))
    failing!(fn Color3sv(*GLshort))
    failing!(fn Color3ub(GLubyte, GLubyte, GLubyte))
    failing!(fn Color3ubv(*GLubyte))
    failing!(fn Color3ui(GLuint, GLuint, GLuint))
    failing!(fn Color3uiv(*GLuint))
    failing!(fn Color3us(GLushort, GLushort, GLushort))
    failing!(fn Color3usv(*GLushort))
    failing!(fn Color4b(GLbyte, GLbyte, GLbyte, GLbyte))
    failing!(fn Color4bv(*GLbyte))
    failing!(fn Color4d(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn Color4dv(*GLdouble))
    failing!(fn Color4f(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Color4fv(*GLfloat))
    failing!(fn Color4i(GLint, GLint, GLint, GLint))
    failing!(fn Color4iv(*GLint))
    failing!(fn Color4s(GLshort, GLshort, GLshort, GLshort))
    failing!(fn Color4sv(*GLshort))
    failing!(fn Color4ub(GLubyte, GLubyte, GLubyte, GLubyte))
    failing!(fn Color4ubv(*GLubyte))
    failing!(fn Color4ui(GLuint, GLuint, GLuint, GLuint))
    failing!(fn Color4uiv(*GLuint))
    failing!(fn Color4us(GLushort, GLushort, GLushort, GLushort))
    failing!(fn Color4usv(*GLushort))
    failing!(fn ColorMask(GLboolean, GLboolean, GLboolean, GLboolean))
    failing!(fn ColorMaterial(GLenum, GLenum))
    failing!(fn ColorPointer(GLint, GLenum, GLsizei, *c_void))
    failing!(fn CompileShader(GLuint))
    failing!(fn CompressedTexImage1D(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexImage2D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexImage3D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CompressedTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *c_void))
    failing!(fn CopyPixels(GLint, GLint, GLsizei, GLsizei, GLenum))
    failing!(fn CopyTexImage1D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint))
    failing!(fn CopyTexImage2D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint))
    failing!(fn CopyTexSubImage1D(GLenum, GLint, GLint, GLint, GLint, GLsizei))
    failing!(fn CopyTexSubImage2D(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CopyTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CreateProgram() -> GLuint)
    failing!(fn CreateShader(GLenum) -> GLuint)
    failing!(fn CullFace(GLenum))
    failing!(fn DeleteBuffers(GLsizei, *GLuint))
    failing!(fn DeleteLists(GLuint, GLsizei))
    failing!(fn DeleteProgram(GLuint))
    failing!(fn DeleteQueries(GLsizei, *GLuint))
    failing!(fn DeleteShader(GLuint))
    failing!(fn DeleteTextures(GLsizei, *GLuint))
    failing!(fn DepthFunc(GLenum))
    failing!(fn DepthMask(GLboolean))
    failing!(fn DepthRange(GLdouble, GLdouble))
    failing!(fn DetachShader(GLuint, GLuint))
    failing!(fn Disable(GLenum))
    failing!(fn DisableClientState(GLenum))
    failing!(fn DisableVertexAttribArray(GLuint))
    failing!(fn DrawArrays(GLenum, GLint, GLsizei))
    failing!(fn DrawBuffer(GLenum))
    failing!(fn DrawBuffers(GLsizei, *GLenum))
    failing!(fn DrawElements(GLenum, GLsizei, GLenum, *c_void))
    failing!(fn DrawPixels(GLsizei, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn DrawRangeElements(GLenum, GLuint, GLuint, GLsizei, GLenum, *c_void))
    failing!(fn EdgeFlag(GLboolean))
    failing!(fn EdgeFlagPointer(GLsizei, *c_void))
    failing!(fn EdgeFlagv(*GLboolean))
    failing!(fn Enable(GLenum))
    failing!(fn EnableClientState(GLenum))
    failing!(fn EnableVertexAttribArray(GLuint))
    failing!(fn End())
    failing!(fn EndList())
    failing!(fn EndQuery(GLenum))
    failing!(fn EvalCoord1d(GLdouble))
    failing!(fn EvalCoord1dv(*GLdouble))
    failing!(fn EvalCoord1f(GLfloat))
    failing!(fn EvalCoord1fv(*GLfloat))
    failing!(fn EvalCoord2d(GLdouble, GLdouble))
    failing!(fn EvalCoord2dv(*GLdouble))
    failing!(fn EvalCoord2f(GLfloat, GLfloat))
    failing!(fn EvalCoord2fv(*GLfloat))
    failing!(fn EvalMesh1(GLenum, GLint, GLint))
    failing!(fn EvalMesh2(GLenum, GLint, GLint, GLint, GLint))
    failing!(fn EvalPoint1(GLint))
    failing!(fn EvalPoint2(GLint, GLint))
    failing!(fn FeedbackBuffer(GLsizei, GLenum, *mut GLfloat))
    failing!(fn Finish())
    failing!(fn Flush())
    failing!(fn FogCoordPointer(GLenum, GLsizei, *c_void))
    failing!(fn FogCoordd(GLdouble))
    failing!(fn FogCoorddv(*GLdouble))
    failing!(fn FogCoordf(GLfloat))
    failing!(fn FogCoordfv(*GLfloat))
    failing!(fn Fogf(GLenum, GLfloat))
    failing!(fn Fogfv(GLenum, *GLfloat))
    failing!(fn Fogi(GLenum, GLint))
    failing!(fn Fogiv(GLenum, *GLint))
    failing!(fn FrontFace(GLenum))
    failing!(fn Frustum(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn GenBuffers(GLsizei, *mut GLuint))
    failing!(fn GenLists(GLsizei) -> GLuint)
    failing!(fn GenQueries(GLsizei, *mut GLuint))
    failing!(fn GenTextures(GLsizei, *mut GLuint))
    failing!(fn GetActiveAttrib(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetActiveUniform(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetAttachedShaders(GLuint, GLsizei, *mut GLsizei, *mut GLuint))
    failing!(fn GetAttribLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetBooleanv(GLenum, *mut GLboolean))
    failing!(fn GetBufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetBufferPointerv(GLenum, GLenum, **mut c_void))
    failing!(fn GetBufferSubData(GLenum, GLintptr, GLsizeiptr, *mut c_void))
    failing!(fn GetClipPlane(GLenum, *mut GLdouble))
    failing!(fn GetCompressedTexImage(GLenum, GLint, *mut c_void))
    failing!(fn GetDoublev(GLenum, *mut GLdouble))
    failing!(fn GetError() -> GLenum)
    failing!(fn GetFloatv(GLenum, *mut GLfloat))
    failing!(fn GetIntegerv(GLenum, *mut GLint))
    failing!(fn GetLightfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetLightiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetMapdv(GLenum, GLenum, *mut GLdouble))
    failing!(fn GetMapfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetMapiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetMaterialfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetMaterialiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetPixelMapfv(GLenum, *mut GLfloat))
    failing!(fn GetPixelMapuiv(GLenum, *mut GLuint))
    failing!(fn GetPixelMapusv(GLenum, *mut GLushort))
    failing!(fn GetPointerv(GLenum, **mut c_void))
    failing!(fn GetPolygonStipple(*mut GLubyte))
    failing!(fn GetProgramInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjectiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjectuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetQueryiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetShaderInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderSource(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetString(GLenum) -> *GLubyte)
    failing!(fn GetTexEnvfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetTexEnviv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTexGendv(GLenum, GLenum, *mut GLdouble))
    failing!(fn GetTexGenfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetTexGeniv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTexImage(GLenum, GLint, GLenum, GLenum, *mut c_void))
    failing!(fn GetTexLevelParameterfv(GLenum, GLint, GLenum, *mut GLfloat))
    failing!(fn GetTexLevelParameteriv(GLenum, GLint, GLenum, *mut GLint))
    failing!(fn GetTexParameterfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetTexParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetUniformLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetUniformfv(GLuint, GLint, *mut GLfloat))
    failing!(fn GetUniformiv(GLuint, GLint, *mut GLint))
    failing!(fn GetVertexAttribPointerv(GLuint, GLenum, **mut c_void))
    failing!(fn GetVertexAttribdv(GLuint, GLenum, *mut GLdouble))
    failing!(fn GetVertexAttribfv(GLuint, GLenum, *mut GLfloat))
    failing!(fn GetVertexAttribiv(GLuint, GLenum, *mut GLint))
    failing!(fn Hint(GLenum, GLenum))
    failing!(fn IndexMask(GLuint))
    failing!(fn IndexPointer(GLenum, GLsizei, *c_void))
    failing!(fn Indexd(GLdouble))
    failing!(fn Indexdv(*GLdouble))
    failing!(fn Indexf(GLfloat))
    failing!(fn Indexfv(*GLfloat))
    failing!(fn Indexi(GLint))
    failing!(fn Indexiv(*GLint))
    failing!(fn Indexs(GLshort))
    failing!(fn Indexsv(*GLshort))
    failing!(fn Indexub(GLubyte))
    failing!(fn Indexubv(*GLubyte))
    failing!(fn InitNames())
    failing!(fn InterleavedArrays(GLenum, GLsizei, *c_void))
    failing!(fn IsBuffer(GLuint) -> GLboolean)
    failing!(fn IsEnabled(GLenum) -> GLboolean)
    failing!(fn IsList(GLuint) -> GLboolean)
    failing!(fn IsProgram(GLuint) -> GLboolean)
    failing!(fn IsQuery(GLuint) -> GLboolean)
    failing!(fn IsShader(GLuint) -> GLboolean)
    failing!(fn IsTexture(GLuint) -> GLboolean)
    failing!(fn LightModelf(GLenum, GLfloat))
    failing!(fn LightModelfv(GLenum, *GLfloat))
    failing!(fn LightModeli(GLenum, GLint))
    failing!(fn LightModeliv(GLenum, *GLint))
    failing!(fn Lightf(GLenum, GLenum, GLfloat))
    failing!(fn Lightfv(GLenum, GLenum, *GLfloat))
    failing!(fn Lighti(GLenum, GLenum, GLint))
    failing!(fn Lightiv(GLenum, GLenum, *GLint))
    failing!(fn LineStipple(GLint, GLushort))
    failing!(fn LineWidth(GLfloat))
    failing!(fn LinkProgram(GLuint))
    failing!(fn ListBase(GLuint))
    failing!(fn LoadIdentity())
    failing!(fn LoadMatrixd(*GLdouble))
    failing!(fn LoadMatrixf(*GLfloat))
    failing!(fn LoadName(GLuint))
    failing!(fn LoadTransposeMatrixd(*GLdouble))
    failing!(fn LoadTransposeMatrixf(*GLfloat))
    failing!(fn LogicOp(GLenum))
    failing!(fn Map1d(GLenum, GLdouble, GLdouble, GLint, GLint, *GLdouble))
    failing!(fn Map1f(GLenum, GLfloat, GLfloat, GLint, GLint, *GLfloat))
    failing!(fn Map2d(GLenum, GLdouble, GLdouble, GLint, GLint, GLdouble, GLdouble, GLint, GLint, *GLdouble))
    failing!(fn Map2f(GLenum, GLfloat, GLfloat, GLint, GLint, GLfloat, GLfloat, GLint, GLint, *GLfloat))
    failing!(fn MapBuffer(GLenum, GLenum) -> *c_void)
    failing!(fn MapGrid1d(GLint, GLdouble, GLdouble))
    failing!(fn MapGrid1f(GLint, GLfloat, GLfloat))
    failing!(fn MapGrid2d(GLint, GLdouble, GLdouble, GLint, GLdouble, GLdouble))
    failing!(fn MapGrid2f(GLint, GLfloat, GLfloat, GLint, GLfloat, GLfloat))
    failing!(fn Materialf(GLenum, GLenum, GLfloat))
    failing!(fn Materialfv(GLenum, GLenum, *GLfloat))
    failing!(fn Materiali(GLenum, GLenum, GLint))
    failing!(fn Materialiv(GLenum, GLenum, *GLint))
    failing!(fn MatrixMode(GLenum))
    failing!(fn MultMatrixd(*GLdouble))
    failing!(fn MultMatrixf(*GLfloat))
    failing!(fn MultTransposeMatrixd(*GLdouble))
    failing!(fn MultTransposeMatrixf(*GLfloat))
    failing!(fn MultiDrawArrays(GLenum, *GLint, *GLsizei, GLsizei))
    failing!(fn MultiDrawElements(GLenum, *GLsizei, GLenum, **c_void, GLsizei))
    failing!(fn MultiTexCoord1d(GLenum, GLdouble))
    failing!(fn MultiTexCoord1dv(GLenum, *GLdouble))
    failing!(fn MultiTexCoord1f(GLenum, GLfloat))
    failing!(fn MultiTexCoord1fv(GLenum, *GLfloat))
    failing!(fn MultiTexCoord1i(GLenum, GLint))
    failing!(fn MultiTexCoord1iv(GLenum, *GLint))
    failing!(fn MultiTexCoord1s(GLenum, GLshort))
    failing!(fn MultiTexCoord1sv(GLenum, *GLshort))
    failing!(fn MultiTexCoord2d(GLenum, GLdouble, GLdouble))
    failing!(fn MultiTexCoord2dv(GLenum, *GLdouble))
    failing!(fn MultiTexCoord2f(GLenum, GLfloat, GLfloat))
    failing!(fn MultiTexCoord2fv(GLenum, *GLfloat))
    failing!(fn MultiTexCoord2i(GLenum, GLint, GLint))
    failing!(fn MultiTexCoord2iv(GLenum, *GLint))
    failing!(fn MultiTexCoord2s(GLenum, GLshort, GLshort))
    failing!(fn MultiTexCoord2sv(GLenum, *GLshort))
    failing!(fn MultiTexCoord3d(GLenum, GLdouble, GLdouble, GLdouble))
    failing!(fn MultiTexCoord3dv(GLenum, *GLdouble))
    failing!(fn MultiTexCoord3f(GLenum, GLfloat, GLfloat, GLfloat))
    failing!(fn MultiTexCoord3fv(GLenum, *GLfloat))
    failing!(fn MultiTexCoord3i(GLenum, GLint, GLint, GLint))
    failing!(fn MultiTexCoord3iv(GLenum, *GLint))
    failing!(fn MultiTexCoord3s(GLenum, GLshort, GLshort, GLshort))
    failing!(fn MultiTexCoord3sv(GLenum, *GLshort))
    failing!(fn MultiTexCoord4d(GLenum, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn MultiTexCoord4dv(GLenum, *GLdouble))
    failing!(fn MultiTexCoord4f(GLenum, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn MultiTexCoord4fv(GLenum, *GLfloat))
    failing!(fn MultiTexCoord4i(GLenum, GLint, GLint, GLint, GLint))
    failing!(fn MultiTexCoord4iv(GLenum, *GLint))
    failing!(fn MultiTexCoord4s(GLenum, GLshort, GLshort, GLshort, GLshort))
    failing!(fn MultiTexCoord4sv(GLenum, *GLshort))
    failing!(fn NewList(GLuint, GLenum))
    failing!(fn Normal3b(GLbyte, GLbyte, GLbyte))
    failing!(fn Normal3bv(*GLbyte))
    failing!(fn Normal3d(GLdouble, GLdouble, GLdouble))
    failing!(fn Normal3dv(*GLdouble))
    failing!(fn Normal3f(GLfloat, GLfloat, GLfloat))
    failing!(fn Normal3fv(*GLfloat))
    failing!(fn Normal3i(GLint, GLint, GLint))
    failing!(fn Normal3iv(*GLint))
    failing!(fn Normal3s(GLshort, GLshort, GLshort))
    failing!(fn Normal3sv(*GLshort))
    failing!(fn NormalPointer(GLenum, GLsizei, *c_void))
    failing!(fn Ortho(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn PassThrough(GLfloat))
    failing!(fn PixelMapfv(GLenum, GLsizei, *GLfloat))
    failing!(fn PixelMapuiv(GLenum, GLsizei, *GLuint))
    failing!(fn PixelMapusv(GLenum, GLsizei, *GLushort))
    failing!(fn PixelStoref(GLenum, GLfloat))
    failing!(fn PixelStorei(GLenum, GLint))
    failing!(fn PixelTransferf(GLenum, GLfloat))
    failing!(fn PixelTransferi(GLenum, GLint))
    failing!(fn PixelZoom(GLfloat, GLfloat))
    failing!(fn PointParameterf(GLenum, GLfloat))
    failing!(fn PointParameterfv(GLenum, *GLfloat))
    failing!(fn PointParameteri(GLenum, GLint))
    failing!(fn PointParameteriv(GLenum, *GLint))
    failing!(fn PointSize(GLfloat))
    failing!(fn PolygonMode(GLenum, GLenum))
    failing!(fn PolygonOffset(GLfloat, GLfloat))
    failing!(fn PolygonStipple(*GLubyte))
    failing!(fn PopAttrib())
    failing!(fn PopClientAttrib())
    failing!(fn PopMatrix())
    failing!(fn PopName())
    failing!(fn PrioritizeTextures(GLsizei, *GLuint, *GLfloat))
    failing!(fn PushAttrib(GLbitfield))
    failing!(fn PushClientAttrib(GLbitfield))
    failing!(fn PushMatrix())
    failing!(fn PushName(GLuint))
    failing!(fn RasterPos2d(GLdouble, GLdouble))
    failing!(fn RasterPos2dv(*GLdouble))
    failing!(fn RasterPos2f(GLfloat, GLfloat))
    failing!(fn RasterPos2fv(*GLfloat))
    failing!(fn RasterPos2i(GLint, GLint))
    failing!(fn RasterPos2iv(*GLint))
    failing!(fn RasterPos2s(GLshort, GLshort))
    failing!(fn RasterPos2sv(*GLshort))
    failing!(fn RasterPos3d(GLdouble, GLdouble, GLdouble))
    failing!(fn RasterPos3dv(*GLdouble))
    failing!(fn RasterPos3f(GLfloat, GLfloat, GLfloat))
    failing!(fn RasterPos3fv(*GLfloat))
    failing!(fn RasterPos3i(GLint, GLint, GLint))
    failing!(fn RasterPos3iv(*GLint))
    failing!(fn RasterPos3s(GLshort, GLshort, GLshort))
    failing!(fn RasterPos3sv(*GLshort))
    failing!(fn RasterPos4d(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn RasterPos4dv(*GLdouble))
    failing!(fn RasterPos4f(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn RasterPos4fv(*GLfloat))
    failing!(fn RasterPos4i(GLint, GLint, GLint, GLint))
    failing!(fn RasterPos4iv(*GLint))
    failing!(fn RasterPos4s(GLshort, GLshort, GLshort, GLshort))
    failing!(fn RasterPos4sv(*GLshort))
    failing!(fn ReadBuffer(GLenum))
    failing!(fn ReadPixels(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void))
    failing!(fn Rectd(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn Rectdv(*GLdouble, *GLdouble))
    failing!(fn Rectf(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Rectfv(*GLfloat, *GLfloat))
    failing!(fn Recti(GLint, GLint, GLint, GLint))
    failing!(fn Rectiv(*GLint, *GLint))
    failing!(fn Rects(GLshort, GLshort, GLshort, GLshort))
    failing!(fn Rectsv(*GLshort, *GLshort))
    failing!(fn RenderMode(GLenum) -> GLint)
    failing!(fn Rotated(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn Rotatef(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn SampleCoverage(GLfloat, GLboolean))
    failing!(fn Scaled(GLdouble, GLdouble, GLdouble))
    failing!(fn Scalef(GLfloat, GLfloat, GLfloat))
    failing!(fn Scissor(GLint, GLint, GLsizei, GLsizei))
    failing!(fn SecondaryColor3b(GLbyte, GLbyte, GLbyte))
    failing!(fn SecondaryColor3bv(*GLbyte))
    failing!(fn SecondaryColor3d(GLdouble, GLdouble, GLdouble))
    failing!(fn SecondaryColor3dv(*GLdouble))
    failing!(fn SecondaryColor3f(GLfloat, GLfloat, GLfloat))
    failing!(fn SecondaryColor3fv(*GLfloat))
    failing!(fn SecondaryColor3i(GLint, GLint, GLint))
    failing!(fn SecondaryColor3iv(*GLint))
    failing!(fn SecondaryColor3s(GLshort, GLshort, GLshort))
    failing!(fn SecondaryColor3sv(*GLshort))
    failing!(fn SecondaryColor3ub(GLubyte, GLubyte, GLubyte))
    failing!(fn SecondaryColor3ubv(*GLubyte))
    failing!(fn SecondaryColor3ui(GLuint, GLuint, GLuint))
    failing!(fn SecondaryColor3uiv(*GLuint))
    failing!(fn SecondaryColor3us(GLushort, GLushort, GLushort))
    failing!(fn SecondaryColor3usv(*GLushort))
    failing!(fn SecondaryColorPointer(GLint, GLenum, GLsizei, *c_void))
    failing!(fn SelectBuffer(GLsizei, *mut GLuint))
    failing!(fn ShadeModel(GLenum))
    failing!(fn ShaderSource(GLuint, GLsizei, **GLchar, *GLint))
    failing!(fn StencilFunc(GLenum, GLint, GLuint))
    failing!(fn StencilFuncSeparate(GLenum, GLenum, GLint, GLuint))
    failing!(fn StencilMask(GLuint))
    failing!(fn StencilMaskSeparate(GLenum, GLuint))
    failing!(fn StencilOp(GLenum, GLenum, GLenum))
    failing!(fn StencilOpSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn TexCoord1d(GLdouble))
    failing!(fn TexCoord1dv(*GLdouble))
    failing!(fn TexCoord1f(GLfloat))
    failing!(fn TexCoord1fv(*GLfloat))
    failing!(fn TexCoord1i(GLint))
    failing!(fn TexCoord1iv(*GLint))
    failing!(fn TexCoord1s(GLshort))
    failing!(fn TexCoord1sv(*GLshort))
    failing!(fn TexCoord2d(GLdouble, GLdouble))
    failing!(fn TexCoord2dv(*GLdouble))
    failing!(fn TexCoord2f(GLfloat, GLfloat))
    failing!(fn TexCoord2fv(*GLfloat))
    failing!(fn TexCoord2i(GLint, GLint))
    failing!(fn TexCoord2iv(*GLint))
    failing!(fn TexCoord2s(GLshort, GLshort))
    failing!(fn TexCoord2sv(*GLshort))
    failing!(fn TexCoord3d(GLdouble, GLdouble, GLdouble))
    failing!(fn TexCoord3dv(*GLdouble))
    failing!(fn TexCoord3f(GLfloat, GLfloat, GLfloat))
    failing!(fn TexCoord3fv(*GLfloat))
    failing!(fn TexCoord3i(GLint, GLint, GLint))
    failing!(fn TexCoord3iv(*GLint))
    failing!(fn TexCoord3s(GLshort, GLshort, GLshort))
    failing!(fn TexCoord3sv(*GLshort))
    failing!(fn TexCoord4d(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn TexCoord4dv(*GLdouble))
    failing!(fn TexCoord4f(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn TexCoord4fv(*GLfloat))
    failing!(fn TexCoord4i(GLint, GLint, GLint, GLint))
    failing!(fn TexCoord4iv(*GLint))
    failing!(fn TexCoord4s(GLshort, GLshort, GLshort, GLshort))
    failing!(fn TexCoord4sv(*GLshort))
    failing!(fn TexCoordPointer(GLint, GLenum, GLsizei, *c_void))
    failing!(fn TexEnvf(GLenum, GLenum, GLfloat))
    failing!(fn TexEnvfv(GLenum, GLenum, *GLfloat))
    failing!(fn TexEnvi(GLenum, GLenum, GLint))
    failing!(fn TexEnviv(GLenum, GLenum, *GLint))
    failing!(fn TexGend(GLenum, GLenum, GLdouble))
    failing!(fn TexGendv(GLenum, GLenum, *GLdouble))
    failing!(fn TexGenf(GLenum, GLenum, GLfloat))
    failing!(fn TexGenfv(GLenum, GLenum, *GLfloat))
    failing!(fn TexGeni(GLenum, GLenum, GLint))
    failing!(fn TexGeniv(GLenum, GLenum, *GLint))
    failing!(fn TexImage1D(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexImage2D(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexImage3D(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *c_void))
    failing!(fn TexParameterf(GLenum, GLenum, GLfloat))
    failing!(fn TexParameterfv(GLenum, GLenum, *GLfloat))
    failing!(fn TexParameteri(GLenum, GLenum, GLint))
    failing!(fn TexParameteriv(GLenum, GLenum, *GLint))
    failing!(fn TexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn TexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn TexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *c_void))
    failing!(fn Translated(GLdouble, GLdouble, GLdouble))
    failing!(fn Translatef(GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform1f(GLint, GLfloat))
    failing!(fn Uniform1fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform1i(GLint, GLint))
    failing!(fn Uniform1iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform2f(GLint, GLfloat, GLfloat))
    failing!(fn Uniform2fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform2i(GLint, GLint, GLint))
    failing!(fn Uniform2iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform3f(GLint, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform3fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform3i(GLint, GLint, GLint, GLint))
    failing!(fn Uniform3iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform4f(GLint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform4fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform4i(GLint, GLint, GLint, GLint, GLint))
    failing!(fn Uniform4iv(GLint, GLsizei, *GLint))
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
    failing!(fn Vertex2d(GLdouble, GLdouble))
    failing!(fn Vertex2dv(*GLdouble))
    failing!(fn Vertex2f(GLfloat, GLfloat))
    failing!(fn Vertex2fv(*GLfloat))
    failing!(fn Vertex2i(GLint, GLint))
    failing!(fn Vertex2iv(*GLint))
    failing!(fn Vertex2s(GLshort, GLshort))
    failing!(fn Vertex2sv(*GLshort))
    failing!(fn Vertex3d(GLdouble, GLdouble, GLdouble))
    failing!(fn Vertex3dv(*GLdouble))
    failing!(fn Vertex3f(GLfloat, GLfloat, GLfloat))
    failing!(fn Vertex3fv(*GLfloat))
    failing!(fn Vertex3i(GLint, GLint, GLint))
    failing!(fn Vertex3iv(*GLint))
    failing!(fn Vertex3s(GLshort, GLshort, GLshort))
    failing!(fn Vertex3sv(*GLshort))
    failing!(fn Vertex4d(GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn Vertex4dv(*GLdouble))
    failing!(fn Vertex4f(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Vertex4fv(*GLfloat))
    failing!(fn Vertex4i(GLint, GLint, GLint, GLint))
    failing!(fn Vertex4iv(*GLint))
    failing!(fn Vertex4s(GLshort, GLshort, GLshort, GLshort))
    failing!(fn Vertex4sv(*GLshort))
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
    failing!(fn VertexAttribPointer(GLuint, GLint, GLenum, GLboolean, GLsizei, *c_void))
    failing!(fn VertexPointer(GLint, GLenum, GLsizei, *c_void))
    failing!(fn Viewport(GLint, GLint, GLsizei, GLsizei))
    failing!(fn WindowPos2d(GLdouble, GLdouble))
    failing!(fn WindowPos2dv(*GLdouble))
    failing!(fn WindowPos2f(GLfloat, GLfloat))
    failing!(fn WindowPos2fv(*GLfloat))
    failing!(fn WindowPos2i(GLint, GLint))
    failing!(fn WindowPos2iv(*GLint))
    failing!(fn WindowPos2s(GLshort, GLshort))
    failing!(fn WindowPos2sv(*GLshort))
    failing!(fn WindowPos3d(GLdouble, GLdouble, GLdouble))
    failing!(fn WindowPos3dv(*GLdouble))
    failing!(fn WindowPos3f(GLfloat, GLfloat, GLfloat))
    failing!(fn WindowPos3fv(*GLfloat))
    failing!(fn WindowPos3i(GLint, GLint, GLint))
    failing!(fn WindowPos3iv(*GLint))
    failing!(fn WindowPos3s(GLshort, GLshort, GLshort))
    failing!(fn WindowPos3sv(*GLshort))
}

/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
///
/// ~~~
/// let gl = gl::load_with(glfw::get_proc_address);
/// ~~~
pub fn load_with(loadfn: |symbol: &str| -> Option<extern "system" fn()>) {
    Accum::load_with(|s| loadfn(s));
    ActiveTexture::load_with(|s| loadfn(s));
    AlphaFunc::load_with(|s| loadfn(s));
    AreTexturesResident::load_with(|s| loadfn(s));
    ArrayElement::load_with(|s| loadfn(s));
    AttachShader::load_with(|s| loadfn(s));
    Begin::load_with(|s| loadfn(s));
    BeginQuery::load_with(|s| loadfn(s));
    BindAttribLocation::load_with(|s| loadfn(s));
    BindBuffer::load_with(|s| loadfn(s));
    BindTexture::load_with(|s| loadfn(s));
    Bitmap::load_with(|s| loadfn(s));
    BlendColor::load_with(|s| loadfn(s));
    BlendEquation::load_with(|s| loadfn(s));
    BlendEquationSeparate::load_with(|s| loadfn(s));
    BlendFunc::load_with(|s| loadfn(s));
    BlendFuncSeparate::load_with(|s| loadfn(s));
    BufferData::load_with(|s| loadfn(s));
    BufferSubData::load_with(|s| loadfn(s));
    CallList::load_with(|s| loadfn(s));
    CallLists::load_with(|s| loadfn(s));
    Clear::load_with(|s| loadfn(s));
    ClearAccum::load_with(|s| loadfn(s));
    ClearColor::load_with(|s| loadfn(s));
    ClearDepth::load_with(|s| loadfn(s));
    ClearIndex::load_with(|s| loadfn(s));
    ClearStencil::load_with(|s| loadfn(s));
    ClientActiveTexture::load_with(|s| loadfn(s));
    ClipPlane::load_with(|s| loadfn(s));
    Color3b::load_with(|s| loadfn(s));
    Color3bv::load_with(|s| loadfn(s));
    Color3d::load_with(|s| loadfn(s));
    Color3dv::load_with(|s| loadfn(s));
    Color3f::load_with(|s| loadfn(s));
    Color3fv::load_with(|s| loadfn(s));
    Color3i::load_with(|s| loadfn(s));
    Color3iv::load_with(|s| loadfn(s));
    Color3s::load_with(|s| loadfn(s));
    Color3sv::load_with(|s| loadfn(s));
    Color3ub::load_with(|s| loadfn(s));
    Color3ubv::load_with(|s| loadfn(s));
    Color3ui::load_with(|s| loadfn(s));
    Color3uiv::load_with(|s| loadfn(s));
    Color3us::load_with(|s| loadfn(s));
    Color3usv::load_with(|s| loadfn(s));
    Color4b::load_with(|s| loadfn(s));
    Color4bv::load_with(|s| loadfn(s));
    Color4d::load_with(|s| loadfn(s));
    Color4dv::load_with(|s| loadfn(s));
    Color4f::load_with(|s| loadfn(s));
    Color4fv::load_with(|s| loadfn(s));
    Color4i::load_with(|s| loadfn(s));
    Color4iv::load_with(|s| loadfn(s));
    Color4s::load_with(|s| loadfn(s));
    Color4sv::load_with(|s| loadfn(s));
    Color4ub::load_with(|s| loadfn(s));
    Color4ubv::load_with(|s| loadfn(s));
    Color4ui::load_with(|s| loadfn(s));
    Color4uiv::load_with(|s| loadfn(s));
    Color4us::load_with(|s| loadfn(s));
    Color4usv::load_with(|s| loadfn(s));
    ColorMask::load_with(|s| loadfn(s));
    ColorMaterial::load_with(|s| loadfn(s));
    ColorPointer::load_with(|s| loadfn(s));
    CompileShader::load_with(|s| loadfn(s));
    CompressedTexImage1D::load_with(|s| loadfn(s));
    CompressedTexImage2D::load_with(|s| loadfn(s));
    CompressedTexImage3D::load_with(|s| loadfn(s));
    CompressedTexSubImage1D::load_with(|s| loadfn(s));
    CompressedTexSubImage2D::load_with(|s| loadfn(s));
    CompressedTexSubImage3D::load_with(|s| loadfn(s));
    CopyPixels::load_with(|s| loadfn(s));
    CopyTexImage1D::load_with(|s| loadfn(s));
    CopyTexImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage1D::load_with(|s| loadfn(s));
    CopyTexSubImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage3D::load_with(|s| loadfn(s));
    CreateProgram::load_with(|s| loadfn(s));
    CreateShader::load_with(|s| loadfn(s));
    CullFace::load_with(|s| loadfn(s));
    DeleteBuffers::load_with(|s| loadfn(s));
    DeleteLists::load_with(|s| loadfn(s));
    DeleteProgram::load_with(|s| loadfn(s));
    DeleteQueries::load_with(|s| loadfn(s));
    DeleteShader::load_with(|s| loadfn(s));
    DeleteTextures::load_with(|s| loadfn(s));
    DepthFunc::load_with(|s| loadfn(s));
    DepthMask::load_with(|s| loadfn(s));
    DepthRange::load_with(|s| loadfn(s));
    DetachShader::load_with(|s| loadfn(s));
    Disable::load_with(|s| loadfn(s));
    DisableClientState::load_with(|s| loadfn(s));
    DisableVertexAttribArray::load_with(|s| loadfn(s));
    DrawArrays::load_with(|s| loadfn(s));
    DrawBuffer::load_with(|s| loadfn(s));
    DrawBuffers::load_with(|s| loadfn(s));
    DrawElements::load_with(|s| loadfn(s));
    DrawPixels::load_with(|s| loadfn(s));
    DrawRangeElements::load_with(|s| loadfn(s));
    EdgeFlag::load_with(|s| loadfn(s));
    EdgeFlagPointer::load_with(|s| loadfn(s));
    EdgeFlagv::load_with(|s| loadfn(s));
    Enable::load_with(|s| loadfn(s));
    EnableClientState::load_with(|s| loadfn(s));
    EnableVertexAttribArray::load_with(|s| loadfn(s));
    End::load_with(|s| loadfn(s));
    EndList::load_with(|s| loadfn(s));
    EndQuery::load_with(|s| loadfn(s));
    EvalCoord1d::load_with(|s| loadfn(s));
    EvalCoord1dv::load_with(|s| loadfn(s));
    EvalCoord1f::load_with(|s| loadfn(s));
    EvalCoord1fv::load_with(|s| loadfn(s));
    EvalCoord2d::load_with(|s| loadfn(s));
    EvalCoord2dv::load_with(|s| loadfn(s));
    EvalCoord2f::load_with(|s| loadfn(s));
    EvalCoord2fv::load_with(|s| loadfn(s));
    EvalMesh1::load_with(|s| loadfn(s));
    EvalMesh2::load_with(|s| loadfn(s));
    EvalPoint1::load_with(|s| loadfn(s));
    EvalPoint2::load_with(|s| loadfn(s));
    FeedbackBuffer::load_with(|s| loadfn(s));
    Finish::load_with(|s| loadfn(s));
    Flush::load_with(|s| loadfn(s));
    FogCoordPointer::load_with(|s| loadfn(s));
    FogCoordd::load_with(|s| loadfn(s));
    FogCoorddv::load_with(|s| loadfn(s));
    FogCoordf::load_with(|s| loadfn(s));
    FogCoordfv::load_with(|s| loadfn(s));
    Fogf::load_with(|s| loadfn(s));
    Fogfv::load_with(|s| loadfn(s));
    Fogi::load_with(|s| loadfn(s));
    Fogiv::load_with(|s| loadfn(s));
    FrontFace::load_with(|s| loadfn(s));
    Frustum::load_with(|s| loadfn(s));
    GenBuffers::load_with(|s| loadfn(s));
    GenLists::load_with(|s| loadfn(s));
    GenQueries::load_with(|s| loadfn(s));
    GenTextures::load_with(|s| loadfn(s));
    GetActiveAttrib::load_with(|s| loadfn(s));
    GetActiveUniform::load_with(|s| loadfn(s));
    GetAttachedShaders::load_with(|s| loadfn(s));
    GetAttribLocation::load_with(|s| loadfn(s));
    GetBooleanv::load_with(|s| loadfn(s));
    GetBufferParameteriv::load_with(|s| loadfn(s));
    GetBufferPointerv::load_with(|s| loadfn(s));
    GetBufferSubData::load_with(|s| loadfn(s));
    GetClipPlane::load_with(|s| loadfn(s));
    GetCompressedTexImage::load_with(|s| loadfn(s));
    GetDoublev::load_with(|s| loadfn(s));
    GetError::load_with(|s| loadfn(s));
    GetFloatv::load_with(|s| loadfn(s));
    GetIntegerv::load_with(|s| loadfn(s));
    GetLightfv::load_with(|s| loadfn(s));
    GetLightiv::load_with(|s| loadfn(s));
    GetMapdv::load_with(|s| loadfn(s));
    GetMapfv::load_with(|s| loadfn(s));
    GetMapiv::load_with(|s| loadfn(s));
    GetMaterialfv::load_with(|s| loadfn(s));
    GetMaterialiv::load_with(|s| loadfn(s));
    GetPixelMapfv::load_with(|s| loadfn(s));
    GetPixelMapuiv::load_with(|s| loadfn(s));
    GetPixelMapusv::load_with(|s| loadfn(s));
    GetPointerv::load_with(|s| loadfn(s));
    GetPolygonStipple::load_with(|s| loadfn(s));
    GetProgramInfoLog::load_with(|s| loadfn(s));
    GetProgramiv::load_with(|s| loadfn(s));
    GetQueryObjectiv::load_with(|s| loadfn(s));
    GetQueryObjectuiv::load_with(|s| loadfn(s));
    GetQueryiv::load_with(|s| loadfn(s));
    GetShaderInfoLog::load_with(|s| loadfn(s));
    GetShaderSource::load_with(|s| loadfn(s));
    GetShaderiv::load_with(|s| loadfn(s));
    GetString::load_with(|s| loadfn(s));
    GetTexEnvfv::load_with(|s| loadfn(s));
    GetTexEnviv::load_with(|s| loadfn(s));
    GetTexGendv::load_with(|s| loadfn(s));
    GetTexGenfv::load_with(|s| loadfn(s));
    GetTexGeniv::load_with(|s| loadfn(s));
    GetTexImage::load_with(|s| loadfn(s));
    GetTexLevelParameterfv::load_with(|s| loadfn(s));
    GetTexLevelParameteriv::load_with(|s| loadfn(s));
    GetTexParameterfv::load_with(|s| loadfn(s));
    GetTexParameteriv::load_with(|s| loadfn(s));
    GetUniformLocation::load_with(|s| loadfn(s));
    GetUniformfv::load_with(|s| loadfn(s));
    GetUniformiv::load_with(|s| loadfn(s));
    GetVertexAttribPointerv::load_with(|s| loadfn(s));
    GetVertexAttribdv::load_with(|s| loadfn(s));
    GetVertexAttribfv::load_with(|s| loadfn(s));
    GetVertexAttribiv::load_with(|s| loadfn(s));
    Hint::load_with(|s| loadfn(s));
    IndexMask::load_with(|s| loadfn(s));
    IndexPointer::load_with(|s| loadfn(s));
    Indexd::load_with(|s| loadfn(s));
    Indexdv::load_with(|s| loadfn(s));
    Indexf::load_with(|s| loadfn(s));
    Indexfv::load_with(|s| loadfn(s));
    Indexi::load_with(|s| loadfn(s));
    Indexiv::load_with(|s| loadfn(s));
    Indexs::load_with(|s| loadfn(s));
    Indexsv::load_with(|s| loadfn(s));
    Indexub::load_with(|s| loadfn(s));
    Indexubv::load_with(|s| loadfn(s));
    InitNames::load_with(|s| loadfn(s));
    InterleavedArrays::load_with(|s| loadfn(s));
    IsBuffer::load_with(|s| loadfn(s));
    IsEnabled::load_with(|s| loadfn(s));
    IsList::load_with(|s| loadfn(s));
    IsProgram::load_with(|s| loadfn(s));
    IsQuery::load_with(|s| loadfn(s));
    IsShader::load_with(|s| loadfn(s));
    IsTexture::load_with(|s| loadfn(s));
    LightModelf::load_with(|s| loadfn(s));
    LightModelfv::load_with(|s| loadfn(s));
    LightModeli::load_with(|s| loadfn(s));
    LightModeliv::load_with(|s| loadfn(s));
    Lightf::load_with(|s| loadfn(s));
    Lightfv::load_with(|s| loadfn(s));
    Lighti::load_with(|s| loadfn(s));
    Lightiv::load_with(|s| loadfn(s));
    LineStipple::load_with(|s| loadfn(s));
    LineWidth::load_with(|s| loadfn(s));
    LinkProgram::load_with(|s| loadfn(s));
    ListBase::load_with(|s| loadfn(s));
    LoadIdentity::load_with(|s| loadfn(s));
    LoadMatrixd::load_with(|s| loadfn(s));
    LoadMatrixf::load_with(|s| loadfn(s));
    LoadName::load_with(|s| loadfn(s));
    LoadTransposeMatrixd::load_with(|s| loadfn(s));
    LoadTransposeMatrixf::load_with(|s| loadfn(s));
    LogicOp::load_with(|s| loadfn(s));
    Map1d::load_with(|s| loadfn(s));
    Map1f::load_with(|s| loadfn(s));
    Map2d::load_with(|s| loadfn(s));
    Map2f::load_with(|s| loadfn(s));
    MapBuffer::load_with(|s| loadfn(s));
    MapGrid1d::load_with(|s| loadfn(s));
    MapGrid1f::load_with(|s| loadfn(s));
    MapGrid2d::load_with(|s| loadfn(s));
    MapGrid2f::load_with(|s| loadfn(s));
    Materialf::load_with(|s| loadfn(s));
    Materialfv::load_with(|s| loadfn(s));
    Materiali::load_with(|s| loadfn(s));
    Materialiv::load_with(|s| loadfn(s));
    MatrixMode::load_with(|s| loadfn(s));
    MultMatrixd::load_with(|s| loadfn(s));
    MultMatrixf::load_with(|s| loadfn(s));
    MultTransposeMatrixd::load_with(|s| loadfn(s));
    MultTransposeMatrixf::load_with(|s| loadfn(s));
    MultiDrawArrays::load_with(|s| loadfn(s));
    MultiDrawElements::load_with(|s| loadfn(s));
    MultiTexCoord1d::load_with(|s| loadfn(s));
    MultiTexCoord1dv::load_with(|s| loadfn(s));
    MultiTexCoord1f::load_with(|s| loadfn(s));
    MultiTexCoord1fv::load_with(|s| loadfn(s));
    MultiTexCoord1i::load_with(|s| loadfn(s));
    MultiTexCoord1iv::load_with(|s| loadfn(s));
    MultiTexCoord1s::load_with(|s| loadfn(s));
    MultiTexCoord1sv::load_with(|s| loadfn(s));
    MultiTexCoord2d::load_with(|s| loadfn(s));
    MultiTexCoord2dv::load_with(|s| loadfn(s));
    MultiTexCoord2f::load_with(|s| loadfn(s));
    MultiTexCoord2fv::load_with(|s| loadfn(s));
    MultiTexCoord2i::load_with(|s| loadfn(s));
    MultiTexCoord2iv::load_with(|s| loadfn(s));
    MultiTexCoord2s::load_with(|s| loadfn(s));
    MultiTexCoord2sv::load_with(|s| loadfn(s));
    MultiTexCoord3d::load_with(|s| loadfn(s));
    MultiTexCoord3dv::load_with(|s| loadfn(s));
    MultiTexCoord3f::load_with(|s| loadfn(s));
    MultiTexCoord3fv::load_with(|s| loadfn(s));
    MultiTexCoord3i::load_with(|s| loadfn(s));
    MultiTexCoord3iv::load_with(|s| loadfn(s));
    MultiTexCoord3s::load_with(|s| loadfn(s));
    MultiTexCoord3sv::load_with(|s| loadfn(s));
    MultiTexCoord4d::load_with(|s| loadfn(s));
    MultiTexCoord4dv::load_with(|s| loadfn(s));
    MultiTexCoord4f::load_with(|s| loadfn(s));
    MultiTexCoord4fv::load_with(|s| loadfn(s));
    MultiTexCoord4i::load_with(|s| loadfn(s));
    MultiTexCoord4iv::load_with(|s| loadfn(s));
    MultiTexCoord4s::load_with(|s| loadfn(s));
    MultiTexCoord4sv::load_with(|s| loadfn(s));
    NewList::load_with(|s| loadfn(s));
    Normal3b::load_with(|s| loadfn(s));
    Normal3bv::load_with(|s| loadfn(s));
    Normal3d::load_with(|s| loadfn(s));
    Normal3dv::load_with(|s| loadfn(s));
    Normal3f::load_with(|s| loadfn(s));
    Normal3fv::load_with(|s| loadfn(s));
    Normal3i::load_with(|s| loadfn(s));
    Normal3iv::load_with(|s| loadfn(s));
    Normal3s::load_with(|s| loadfn(s));
    Normal3sv::load_with(|s| loadfn(s));
    NormalPointer::load_with(|s| loadfn(s));
    Ortho::load_with(|s| loadfn(s));
    PassThrough::load_with(|s| loadfn(s));
    PixelMapfv::load_with(|s| loadfn(s));
    PixelMapuiv::load_with(|s| loadfn(s));
    PixelMapusv::load_with(|s| loadfn(s));
    PixelStoref::load_with(|s| loadfn(s));
    PixelStorei::load_with(|s| loadfn(s));
    PixelTransferf::load_with(|s| loadfn(s));
    PixelTransferi::load_with(|s| loadfn(s));
    PixelZoom::load_with(|s| loadfn(s));
    PointParameterf::load_with(|s| loadfn(s));
    PointParameterfv::load_with(|s| loadfn(s));
    PointParameteri::load_with(|s| loadfn(s));
    PointParameteriv::load_with(|s| loadfn(s));
    PointSize::load_with(|s| loadfn(s));
    PolygonMode::load_with(|s| loadfn(s));
    PolygonOffset::load_with(|s| loadfn(s));
    PolygonStipple::load_with(|s| loadfn(s));
    PopAttrib::load_with(|s| loadfn(s));
    PopClientAttrib::load_with(|s| loadfn(s));
    PopMatrix::load_with(|s| loadfn(s));
    PopName::load_with(|s| loadfn(s));
    PrioritizeTextures::load_with(|s| loadfn(s));
    PushAttrib::load_with(|s| loadfn(s));
    PushClientAttrib::load_with(|s| loadfn(s));
    PushMatrix::load_with(|s| loadfn(s));
    PushName::load_with(|s| loadfn(s));
    RasterPos2d::load_with(|s| loadfn(s));
    RasterPos2dv::load_with(|s| loadfn(s));
    RasterPos2f::load_with(|s| loadfn(s));
    RasterPos2fv::load_with(|s| loadfn(s));
    RasterPos2i::load_with(|s| loadfn(s));
    RasterPos2iv::load_with(|s| loadfn(s));
    RasterPos2s::load_with(|s| loadfn(s));
    RasterPos2sv::load_with(|s| loadfn(s));
    RasterPos3d::load_with(|s| loadfn(s));
    RasterPos3dv::load_with(|s| loadfn(s));
    RasterPos3f::load_with(|s| loadfn(s));
    RasterPos3fv::load_with(|s| loadfn(s));
    RasterPos3i::load_with(|s| loadfn(s));
    RasterPos3iv::load_with(|s| loadfn(s));
    RasterPos3s::load_with(|s| loadfn(s));
    RasterPos3sv::load_with(|s| loadfn(s));
    RasterPos4d::load_with(|s| loadfn(s));
    RasterPos4dv::load_with(|s| loadfn(s));
    RasterPos4f::load_with(|s| loadfn(s));
    RasterPos4fv::load_with(|s| loadfn(s));
    RasterPos4i::load_with(|s| loadfn(s));
    RasterPos4iv::load_with(|s| loadfn(s));
    RasterPos4s::load_with(|s| loadfn(s));
    RasterPos4sv::load_with(|s| loadfn(s));
    ReadBuffer::load_with(|s| loadfn(s));
    ReadPixels::load_with(|s| loadfn(s));
    Rectd::load_with(|s| loadfn(s));
    Rectdv::load_with(|s| loadfn(s));
    Rectf::load_with(|s| loadfn(s));
    Rectfv::load_with(|s| loadfn(s));
    Recti::load_with(|s| loadfn(s));
    Rectiv::load_with(|s| loadfn(s));
    Rects::load_with(|s| loadfn(s));
    Rectsv::load_with(|s| loadfn(s));
    RenderMode::load_with(|s| loadfn(s));
    Rotated::load_with(|s| loadfn(s));
    Rotatef::load_with(|s| loadfn(s));
    SampleCoverage::load_with(|s| loadfn(s));
    Scaled::load_with(|s| loadfn(s));
    Scalef::load_with(|s| loadfn(s));
    Scissor::load_with(|s| loadfn(s));
    SecondaryColor3b::load_with(|s| loadfn(s));
    SecondaryColor3bv::load_with(|s| loadfn(s));
    SecondaryColor3d::load_with(|s| loadfn(s));
    SecondaryColor3dv::load_with(|s| loadfn(s));
    SecondaryColor3f::load_with(|s| loadfn(s));
    SecondaryColor3fv::load_with(|s| loadfn(s));
    SecondaryColor3i::load_with(|s| loadfn(s));
    SecondaryColor3iv::load_with(|s| loadfn(s));
    SecondaryColor3s::load_with(|s| loadfn(s));
    SecondaryColor3sv::load_with(|s| loadfn(s));
    SecondaryColor3ub::load_with(|s| loadfn(s));
    SecondaryColor3ubv::load_with(|s| loadfn(s));
    SecondaryColor3ui::load_with(|s| loadfn(s));
    SecondaryColor3uiv::load_with(|s| loadfn(s));
    SecondaryColor3us::load_with(|s| loadfn(s));
    SecondaryColor3usv::load_with(|s| loadfn(s));
    SecondaryColorPointer::load_with(|s| loadfn(s));
    SelectBuffer::load_with(|s| loadfn(s));
    ShadeModel::load_with(|s| loadfn(s));
    ShaderSource::load_with(|s| loadfn(s));
    StencilFunc::load_with(|s| loadfn(s));
    StencilFuncSeparate::load_with(|s| loadfn(s));
    StencilMask::load_with(|s| loadfn(s));
    StencilMaskSeparate::load_with(|s| loadfn(s));
    StencilOp::load_with(|s| loadfn(s));
    StencilOpSeparate::load_with(|s| loadfn(s));
    TexCoord1d::load_with(|s| loadfn(s));
    TexCoord1dv::load_with(|s| loadfn(s));
    TexCoord1f::load_with(|s| loadfn(s));
    TexCoord1fv::load_with(|s| loadfn(s));
    TexCoord1i::load_with(|s| loadfn(s));
    TexCoord1iv::load_with(|s| loadfn(s));
    TexCoord1s::load_with(|s| loadfn(s));
    TexCoord1sv::load_with(|s| loadfn(s));
    TexCoord2d::load_with(|s| loadfn(s));
    TexCoord2dv::load_with(|s| loadfn(s));
    TexCoord2f::load_with(|s| loadfn(s));
    TexCoord2fv::load_with(|s| loadfn(s));
    TexCoord2i::load_with(|s| loadfn(s));
    TexCoord2iv::load_with(|s| loadfn(s));
    TexCoord2s::load_with(|s| loadfn(s));
    TexCoord2sv::load_with(|s| loadfn(s));
    TexCoord3d::load_with(|s| loadfn(s));
    TexCoord3dv::load_with(|s| loadfn(s));
    TexCoord3f::load_with(|s| loadfn(s));
    TexCoord3fv::load_with(|s| loadfn(s));
    TexCoord3i::load_with(|s| loadfn(s));
    TexCoord3iv::load_with(|s| loadfn(s));
    TexCoord3s::load_with(|s| loadfn(s));
    TexCoord3sv::load_with(|s| loadfn(s));
    TexCoord4d::load_with(|s| loadfn(s));
    TexCoord4dv::load_with(|s| loadfn(s));
    TexCoord4f::load_with(|s| loadfn(s));
    TexCoord4fv::load_with(|s| loadfn(s));
    TexCoord4i::load_with(|s| loadfn(s));
    TexCoord4iv::load_with(|s| loadfn(s));
    TexCoord4s::load_with(|s| loadfn(s));
    TexCoord4sv::load_with(|s| loadfn(s));
    TexCoordPointer::load_with(|s| loadfn(s));
    TexEnvf::load_with(|s| loadfn(s));
    TexEnvfv::load_with(|s| loadfn(s));
    TexEnvi::load_with(|s| loadfn(s));
    TexEnviv::load_with(|s| loadfn(s));
    TexGend::load_with(|s| loadfn(s));
    TexGendv::load_with(|s| loadfn(s));
    TexGenf::load_with(|s| loadfn(s));
    TexGenfv::load_with(|s| loadfn(s));
    TexGeni::load_with(|s| loadfn(s));
    TexGeniv::load_with(|s| loadfn(s));
    TexImage1D::load_with(|s| loadfn(s));
    TexImage2D::load_with(|s| loadfn(s));
    TexImage3D::load_with(|s| loadfn(s));
    TexParameterf::load_with(|s| loadfn(s));
    TexParameterfv::load_with(|s| loadfn(s));
    TexParameteri::load_with(|s| loadfn(s));
    TexParameteriv::load_with(|s| loadfn(s));
    TexSubImage1D::load_with(|s| loadfn(s));
    TexSubImage2D::load_with(|s| loadfn(s));
    TexSubImage3D::load_with(|s| loadfn(s));
    Translated::load_with(|s| loadfn(s));
    Translatef::load_with(|s| loadfn(s));
    Uniform1f::load_with(|s| loadfn(s));
    Uniform1fv::load_with(|s| loadfn(s));
    Uniform1i::load_with(|s| loadfn(s));
    Uniform1iv::load_with(|s| loadfn(s));
    Uniform2f::load_with(|s| loadfn(s));
    Uniform2fv::load_with(|s| loadfn(s));
    Uniform2i::load_with(|s| loadfn(s));
    Uniform2iv::load_with(|s| loadfn(s));
    Uniform3f::load_with(|s| loadfn(s));
    Uniform3fv::load_with(|s| loadfn(s));
    Uniform3i::load_with(|s| loadfn(s));
    Uniform3iv::load_with(|s| loadfn(s));
    Uniform4f::load_with(|s| loadfn(s));
    Uniform4fv::load_with(|s| loadfn(s));
    Uniform4i::load_with(|s| loadfn(s));
    Uniform4iv::load_with(|s| loadfn(s));
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
    Vertex2d::load_with(|s| loadfn(s));
    Vertex2dv::load_with(|s| loadfn(s));
    Vertex2f::load_with(|s| loadfn(s));
    Vertex2fv::load_with(|s| loadfn(s));
    Vertex2i::load_with(|s| loadfn(s));
    Vertex2iv::load_with(|s| loadfn(s));
    Vertex2s::load_with(|s| loadfn(s));
    Vertex2sv::load_with(|s| loadfn(s));
    Vertex3d::load_with(|s| loadfn(s));
    Vertex3dv::load_with(|s| loadfn(s));
    Vertex3f::load_with(|s| loadfn(s));
    Vertex3fv::load_with(|s| loadfn(s));
    Vertex3i::load_with(|s| loadfn(s));
    Vertex3iv::load_with(|s| loadfn(s));
    Vertex3s::load_with(|s| loadfn(s));
    Vertex3sv::load_with(|s| loadfn(s));
    Vertex4d::load_with(|s| loadfn(s));
    Vertex4dv::load_with(|s| loadfn(s));
    Vertex4f::load_with(|s| loadfn(s));
    Vertex4fv::load_with(|s| loadfn(s));
    Vertex4i::load_with(|s| loadfn(s));
    Vertex4iv::load_with(|s| loadfn(s));
    Vertex4s::load_with(|s| loadfn(s));
    Vertex4sv::load_with(|s| loadfn(s));
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
    VertexAttribPointer::load_with(|s| loadfn(s));
    VertexPointer::load_with(|s| loadfn(s));
    Viewport::load_with(|s| loadfn(s));
    WindowPos2d::load_with(|s| loadfn(s));
    WindowPos2dv::load_with(|s| loadfn(s));
    WindowPos2f::load_with(|s| loadfn(s));
    WindowPos2fv::load_with(|s| loadfn(s));
    WindowPos2i::load_with(|s| loadfn(s));
    WindowPos2iv::load_with(|s| loadfn(s));
    WindowPos2s::load_with(|s| loadfn(s));
    WindowPos2sv::load_with(|s| loadfn(s));
    WindowPos3d::load_with(|s| loadfn(s));
    WindowPos3dv::load_with(|s| loadfn(s));
    WindowPos3f::load_with(|s| loadfn(s));
    WindowPos3fv::load_with(|s| loadfn(s));
    WindowPos3i::load_with(|s| loadfn(s));
    WindowPos3iv::load_with(|s| loadfn(s));
    WindowPos3s::load_with(|s| loadfn(s));
    WindowPos3sv::load_with(|s| loadfn(s));
}

