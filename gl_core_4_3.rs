#[link(name = "gl",
       vers = "0.1")];
#[comment = "OpenGL bindings for the Rust programming language."];
#[crate_type = "lib"];

use core::libc::*;
use types::*;


//////////////////////////////////////////////////////////////////////////////
//
// Typedefs
//
//////////////////////////////////////////////////////////////////////////////

pub mod types {
	// Common types from OpenGL 1.1
	use core::libc::*;
	
	pub type GLenum = c_uint;
	pub type GLboolean = c_uchar;
	pub type GLbitfield = c_uint;
	pub type GLbyte = c_schar;
	pub type GLshort = c_short;
	pub type GLint = c_int;
	pub type GLsizei = c_int;
	pub type GLubyte = c_uchar;
	pub type GLushort = c_ushort;
	pub type GLuint = c_uint;
	pub type GLhalf = c_ushort;
	pub type GLfloat = c_float;
	pub type GLclampf = c_float;
	pub type GLdouble = c_double;
	pub type GLclampd = c_double;
	pub type GLvoid = c_void;
	
	/* GL type for program/shader text */
	pub type GLchar = c_char;
	
	/* GL types for handling large vertex buffer objects */
	pub type GLintptr = ptrdiff_t;
	pub type GLsizeiptr = ptrdiff_t;
	
	/* GL types for handling large vertex buffer objects */
	pub type GLintptrARB = ptrdiff_t;
	pub type GLsizeiptrARB = ptrdiff_t;
	
	/* GL types for program/shader text and shader object handles */
	pub type GLcharARB = c_char;
	pub type GLhandleARB = c_uint;
	
	/* GL type for "half" precision (s10e5) float data in host memory */
	pub type GLhalfARB = c_ushort;
	pub type GLhalfNV = c_ushort;
	
	pub type GLint64EXT = i64;
	pub type GLuint64EXT = u64;
	pub type GLint64 = i64;
	pub type GLuint64 = u64;
	
	pub struct __GLsync;
	pub type GLsync = *__GLsync;
	
	/* These incomplete types let us declare types compatible with OpenCL's cl_context and cl_event */
	pub struct _cl_context;
	pub struct _cl_event;
	
	pub type GLDEBUGPROCARB = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);
	pub type GLDEBUGPROCAMD = extern "C" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);
	pub type GLDEBUGPROC = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);
	
	pub type GLvdpauSurfaceNV = GLintptr;
}

//////////////////////////////////////////////////////////////////////////////
//
// Enums
//
//////////////////////////////////////////////////////////////////////////////

// Version: 1.1
pub static DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub static STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub static COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub static FALSE: GLenum = 0;
pub static TRUE: GLenum = 1;
pub static POINTS: GLenum = 0x0000;
pub static LINES: GLenum = 0x0001;
pub static LINE_LOOP: GLenum = 0x0002;
pub static LINE_STRIP: GLenum = 0x0003;
pub static TRIANGLE_STRIP: GLenum = 0x0005;
pub static TRIANGLE_FAN: GLenum = 0x0006;
pub static NEVER: GLenum = 0x0200;
pub static LESS: GLenum = 0x0201;
pub static LEQUAL: GLenum = 0x0203;
pub static GREATER: GLenum = 0x0204;
pub static NOTEQUAL: GLenum = 0x0205;
pub static GEQUAL: GLenum = 0x0206;
pub static ALWAYS: GLenum = 0x0207;
pub static ZERO: GLenum = 0;
pub static ONE: GLenum = 1;
pub static SRC_COLOR: GLenum = 0x0300;
pub static ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub static SRC_ALPHA: GLenum = 0x0302;
pub static ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub static DST_ALPHA: GLenum = 0x0304;
pub static ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub static DST_COLOR: GLenum = 0x0306;
pub static ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub static SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub static NONE: GLenum = 0;
pub static FRONT_LEFT: GLenum = 0x0400;
pub static FRONT_RIGHT: GLenum = 0x0401;
pub static BACK_LEFT: GLenum = 0x0402;
pub static BACK_RIGHT: GLenum = 0x0403;
pub static FRONT: GLenum = 0x0404;
pub static BACK: GLenum = 0x0405;
pub static LEFT: GLenum = 0x0406;
pub static RIGHT: GLenum = 0x0407;
pub static FRONT_AND_BACK: GLenum = 0x0408;
pub static NO_ERROR: GLenum = 0;
pub static INVALID_ENUM: GLenum = 0x0500;
pub static INVALID_VALUE: GLenum = 0x0501;
pub static INVALID_OPERATION: GLenum = 0x0502;
pub static OUT_OF_MEMORY: GLenum = 0x0505;
pub static POINT_SIZE: GLenum = 0x0B11;
pub static POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static LINE_SMOOTH: GLenum = 0x0B20;
pub static LINE_WIDTH: GLenum = 0x0B21;
pub static LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static POLYGON_MODE: GLenum = 0x0B40;
pub static POLYGON_SMOOTH: GLenum = 0x0B41;
pub static CULL_FACE: GLenum = 0x0B44;
pub static CULL_FACE_MODE: GLenum = 0x0B45;
pub static FRONT_FACE: GLenum = 0x0B46;
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
pub static DITHER: GLenum = 0x0BD0;
pub static BLEND_DST: GLenum = 0x0BE0;
pub static BLEND_SRC: GLenum = 0x0BE1;
pub static BLEND: GLenum = 0x0BE2;
pub static LOGIC_OP_MODE: GLenum = 0x0BF0;
pub static COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub static DRAW_BUFFER: GLenum = 0x0C01;
pub static READ_BUFFER: GLenum = 0x0C02;
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
pub static MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub static MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub static SUBPIXEL_BITS: GLenum = 0x0D50;
pub static POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub static POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub static POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub static POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub static POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub static TEXTURE_BINDING_1D: GLenum = 0x8068;
pub static TEXTURE_BINDING_2D: GLenum = 0x8069;
pub static TEXTURE_WIDTH: GLenum = 0x1000;
pub static TEXTURE_HEIGHT: GLenum = 0x1001;
pub static TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub static TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub static TEXTURE_RED_SIZE: GLenum = 0x805C;
pub static TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub static TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub static TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
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
pub static PROXY_TEXTURE_1D: GLenum = 0x8063;
pub static PROXY_TEXTURE_2D: GLenum = 0x8064;
pub static REPEAT: GLenum = 0x2901;
pub static R3_G3_B2: GLenum = 0x2A10;
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

// Version: 1.2
pub static UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub static UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub static UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub static UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub static UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub static TEXTURE_BINDING_3D: GLenum = 0x806A;
pub static PACK_SKIP_IMAGES: GLenum = 0x806B;
pub static PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub static UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub static UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub static PROXY_TEXTURE_3D: GLenum = 0x8070;
pub static TEXTURE_DEPTH: GLenum = 0x8071;
pub static TEXTURE_WRAP_R: GLenum = 0x8072;
pub static MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub static UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub static UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub static UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub static UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub static UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub static UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub static BGR: GLenum = 0x80E0;
pub static MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub static MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub static CLAMP_TO_EDGE: GLenum = 0x812F;
pub static TEXTURE_MIN_LOD: GLenum = 0x813A;
pub static TEXTURE_MAX_LOD: GLenum = 0x813B;
pub static TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub static TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub static SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;

// Core Extension: ARB_imaging
pub static CONSTANT_COLOR: GLenum = 0x8001;
pub static ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub static CONSTANT_ALPHA: GLenum = 0x8003;
pub static ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub static BLEND_COLOR: GLenum = 0x8005;
pub static FUNC_ADD: GLenum = 0x8006;
pub static MIN: GLenum = 0x8007;
pub static MAX: GLenum = 0x8008;
pub static BLEND_EQUATION: GLenum = 0x8009;
pub static FUNC_SUBTRACT: GLenum = 0x800A;
pub static FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub static CONVOLUTION_1D: GLenum = 0x8010;
pub static CONVOLUTION_2D: GLenum = 0x8011;
pub static SEPARABLE_2D: GLenum = 0x8012;
pub static CONVOLUTION_BORDER_MODE: GLenum = 0x8013;
pub static CONVOLUTION_FILTER_SCALE: GLenum = 0x8014;
pub static CONVOLUTION_FILTER_BIAS: GLenum = 0x8015;
pub static REDUCE: GLenum = 0x8016;
pub static CONVOLUTION_FORMAT: GLenum = 0x8017;
pub static CONVOLUTION_WIDTH: GLenum = 0x8018;
pub static CONVOLUTION_HEIGHT: GLenum = 0x8019;
pub static MAX_CONVOLUTION_WIDTH: GLenum = 0x801A;
pub static MAX_CONVOLUTION_HEIGHT: GLenum = 0x801B;
pub static POST_CONVOLUTION_RED_SCALE: GLenum = 0x801C;
pub static POST_CONVOLUTION_GREEN_SCALE: GLenum = 0x801D;
pub static POST_CONVOLUTION_BLUE_SCALE: GLenum = 0x801E;
pub static POST_CONVOLUTION_ALPHA_SCALE: GLenum = 0x801F;
pub static POST_CONVOLUTION_RED_BIAS: GLenum = 0x8020;
pub static POST_CONVOLUTION_GREEN_BIAS: GLenum = 0x8021;
pub static POST_CONVOLUTION_BLUE_BIAS: GLenum = 0x8022;
pub static POST_CONVOLUTION_ALPHA_BIAS: GLenum = 0x8023;
pub static HISTOGRAM: GLenum = 0x8024;
pub static PROXY_HISTOGRAM: GLenum = 0x8025;
pub static HISTOGRAM_WIDTH: GLenum = 0x8026;
pub static HISTOGRAM_FORMAT: GLenum = 0x8027;
pub static HISTOGRAM_RED_SIZE: GLenum = 0x8028;
pub static HISTOGRAM_GREEN_SIZE: GLenum = 0x8029;
pub static HISTOGRAM_BLUE_SIZE: GLenum = 0x802A;
pub static HISTOGRAM_ALPHA_SIZE: GLenum = 0x802B;
pub static HISTOGRAM_LUMINANCE_SIZE: GLenum = 0x802C;
pub static HISTOGRAM_SINK: GLenum = 0x802D;
pub static MINMAX: GLenum = 0x802E;
pub static MINMAX_FORMAT: GLenum = 0x802F;
pub static MINMAX_SINK: GLenum = 0x8030;
pub static TABLE_TOO_LARGE: GLenum = 0x8031;
pub static COLOR_MATRIX: GLenum = 0x80B1;
pub static COLOR_MATRIX_STACK_DEPTH: GLenum = 0x80B2;
pub static MAX_COLOR_MATRIX_STACK_DEPTH: GLenum = 0x80B3;
pub static POST_COLOR_MATRIX_RED_SCALE: GLenum = 0x80B4;
pub static POST_COLOR_MATRIX_GREEN_SCALE: GLenum = 0x80B5;
pub static POST_COLOR_MATRIX_BLUE_SCALE: GLenum = 0x80B6;
pub static POST_COLOR_MATRIX_ALPHA_SCALE: GLenum = 0x80B7;
pub static POST_COLOR_MATRIX_RED_BIAS: GLenum = 0x80B8;
pub static POST_COLOR_MATRIX_GREEN_BIAS: GLenum = 0x80B9;
pub static POST_COLOR_MATRIX_BLUE_BIAS: GLenum = 0x80BA;
pub static POST_COLOR_MATRIX_ALPHA_BIAS: GLenum = 0x80BB;
pub static COLOR_TABLE: GLenum = 0x80D0;
pub static POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D1;
pub static POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D2;
pub static PROXY_COLOR_TABLE: GLenum = 0x80D3;
pub static PROXY_POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D4;
pub static PROXY_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D5;
pub static COLOR_TABLE_SCALE: GLenum = 0x80D6;
pub static COLOR_TABLE_BIAS: GLenum = 0x80D7;
pub static COLOR_TABLE_FORMAT: GLenum = 0x80D8;
pub static COLOR_TABLE_WIDTH: GLenum = 0x80D9;
pub static COLOR_TABLE_RED_SIZE: GLenum = 0x80DA;
pub static COLOR_TABLE_GREEN_SIZE: GLenum = 0x80DB;
pub static COLOR_TABLE_BLUE_SIZE: GLenum = 0x80DC;
pub static COLOR_TABLE_ALPHA_SIZE: GLenum = 0x80DD;
pub static COLOR_TABLE_LUMINANCE_SIZE: GLenum = 0x80DE;
pub static COLOR_TABLE_INTENSITY_SIZE: GLenum = 0x80DF;
pub static CONSTANT_BORDER: GLenum = 0x8151;
pub static REPLICATE_BORDER: GLenum = 0x8153;
pub static CONVOLUTION_BORDER_COLOR: GLenum = 0x8154;

// Version: 1.3
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
pub static MULTISAMPLE: GLenum = 0x809D;
pub static SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub static SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub static SAMPLE_COVERAGE: GLenum = 0x80A0;
pub static SAMPLE_BUFFERS: GLenum = 0x80A8;
pub static SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub static SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub static TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub static TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub static TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub static TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub static TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub static PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub static MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub static COMPRESSED_RGB: GLenum = 0x84ED;
pub static COMPRESSED_RGBA: GLenum = 0x84EE;
pub static TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub static TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub static NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub static COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub static CLAMP_TO_BORDER: GLenum = 0x812D;

// Version: 1.4
pub static BLEND_DST_RGB: GLenum = 0x80C8;
pub static BLEND_SRC_RGB: GLenum = 0x80C9;
pub static BLEND_DST_ALPHA: GLenum = 0x80CA;
pub static BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub static POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub static DEPTH_COMPONENT16: GLenum = 0x81A5;
pub static DEPTH_COMPONENT24: GLenum = 0x81A6;
pub static DEPTH_COMPONENT32: GLenum = 0x81A7;
pub static MIRRORED_REPEAT: GLenum = 0x8370;
pub static MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub static TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub static INCR_WRAP: GLenum = 0x8507;
pub static DECR_WRAP: GLenum = 0x8508;
pub static TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub static TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub static TEXTURE_COMPARE_FUNC: GLenum = 0x884D;

// Version: 1.5
pub static BUFFER_SIZE: GLenum = 0x8764;
pub static BUFFER_USAGE: GLenum = 0x8765;
pub static QUERY_COUNTER_BITS: GLenum = 0x8864;
pub static CURRENT_QUERY: GLenum = 0x8865;
pub static QUERY_RESULT: GLenum = 0x8866;
pub static QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
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
pub static SAMPLES_PASSED: GLenum = 0x8914;

// Version: 2.0
pub static BLEND_EQUATION_RGB: GLenum = 0x8009;
pub static VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub static VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub static VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub static VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub static CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub static VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
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
pub static MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub static VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub static MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
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
pub static POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub static LOWER_LEFT: GLenum = 0x8CA1;
pub static UPPER_LEFT: GLenum = 0x8CA2;
pub static STENCIL_BACK_REF: GLenum = 0x8CA3;
pub static STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub static STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;

// Version: 2.1
pub static PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub static PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub static PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub static PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub static FLOAT_MAT2x3: GLenum = 0x8B65;
pub static FLOAT_MAT2x4: GLenum = 0x8B66;
pub static FLOAT_MAT3x2: GLenum = 0x8B67;
pub static FLOAT_MAT3x4: GLenum = 0x8B68;
pub static FLOAT_MAT4x2: GLenum = 0x8B69;
pub static FLOAT_MAT4x3: GLenum = 0x8B6A;
pub static SRGB: GLenum = 0x8C40;
pub static SRGB8: GLenum = 0x8C41;
pub static SRGB_ALPHA: GLenum = 0x8C42;
pub static SRGB8_ALPHA8: GLenum = 0x8C43;
pub static COMPRESSED_SRGB: GLenum = 0x8C48;
pub static COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;

// Version: 3.0
pub static COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub static CLIP_DISTANCE0: GLenum = 0x3000;
pub static CLIP_DISTANCE1: GLenum = 0x3001;
pub static CLIP_DISTANCE2: GLenum = 0x3002;
pub static CLIP_DISTANCE3: GLenum = 0x3003;
pub static CLIP_DISTANCE4: GLenum = 0x3004;
pub static CLIP_DISTANCE5: GLenum = 0x3005;
pub static CLIP_DISTANCE6: GLenum = 0x3006;
pub static CLIP_DISTANCE7: GLenum = 0x3007;
pub static MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub static MAJOR_VERSION: GLenum = 0x821B;
pub static MINOR_VERSION: GLenum = 0x821C;
pub static NUM_EXTENSIONS: GLenum = 0x821D;
pub static CONTEXT_FLAGS: GLenum = 0x821E;
pub static COMPRESSED_RED: GLenum = 0x8225;
pub static COMPRESSED_RG: GLenum = 0x8226;
pub static CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x0001;
pub static RGBA32F: GLenum = 0x8814;
pub static RGB32F: GLenum = 0x8815;
pub static RGBA16F: GLenum = 0x881A;
pub static RGB16F: GLenum = 0x881B;
pub static VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub static MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub static MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub static MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub static CLAMP_READ_COLOR: GLenum = 0x891C;
pub static FIXED_ONLY: GLenum = 0x891D;
pub static MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub static PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub static PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub static TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub static TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub static R11F_G11F_B10F: GLenum = 0x8C3A;
pub static UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub static RGB9_E5: GLenum = 0x8C3D;
pub static UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub static TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
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
pub static RGBA32UI: GLenum = 0x8D70;
pub static RGB32UI: GLenum = 0x8D71;
pub static RGBA16UI: GLenum = 0x8D76;
pub static RGB16UI: GLenum = 0x8D77;
pub static RGBA8UI: GLenum = 0x8D7C;
pub static RGB8UI: GLenum = 0x8D7D;
pub static RGBA32I: GLenum = 0x8D82;
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
pub static SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub static SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
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
pub static INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub static INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub static UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub static UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub static UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub static UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub static UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub static UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub static QUERY_WAIT: GLenum = 0x8E13;
pub static QUERY_NO_WAIT: GLenum = 0x8E14;
pub static QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub static QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub static BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub static BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub static BUFFER_MAP_OFFSET: GLenum = 0x9121;

// Core Extension: ARB_vertex_array_object
pub static VERTEX_ARRAY_BINDING: GLenum = 0x85B5;

// Core Extension: ARB_texture_rg
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

// Core Extension: ARB_texture_compression_rgtc
pub static COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub static COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub static COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub static COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;

// Core Extension: ARB_map_buffer_range
pub static MAP_READ_BIT: GLenum = 0x0001;
pub static MAP_WRITE_BIT: GLenum = 0x0002;
pub static MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub static MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub static MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub static MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;

// Core Extension: ARB_half_float_vertex
pub static HALF_FLOAT: GLenum = 0x140B;

// Core Extension: ARB_framebuffer_sRGB
pub static FRAMEBUFFER_SRGB: GLenum = 0x8DB9;

// Core Extension: ARB_framebuffer_object
pub static INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
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
pub static INDEX: GLenum = 0x8222;
pub static MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub static DEPTH_STENCIL: GLenum = 0x84F9;
pub static UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub static DEPTH24_STENCIL8: GLenum = 0x88F0;
pub static TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub static TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub static TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub static TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub static TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub static TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub static UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub static FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub static READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub static DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub static READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub static RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
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
pub static TEXTURE_LUMINANCE_TYPE: GLenum = 0x8C14;
pub static TEXTURE_INTENSITY_TYPE: GLenum = 0x8C15;

// Core Extension: ARB_depth_buffer_float
pub static DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub static DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub static FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;

// Version: 3.1
pub static SAMPLER_2D_RECT: GLenum = 0x8B63;
pub static SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub static SAMPLER_BUFFER: GLenum = 0x8DC2;
pub static INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub static INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub static UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub static UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub static MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub static TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub static TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub static TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub static PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub static MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub static RED_SNORM: GLenum = 0x8F90;
pub static RG_SNORM: GLenum = 0x8F91;
pub static RGB_SNORM: GLenum = 0x8F92;
pub static RGBA_SNORM: GLenum = 0x8F93;
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

// Core Extension: ARB_uniform_buffer_object
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
pub static INVALID_INDEX: GLenum = 0xFFFFFFFF;
pub static MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub static MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub static UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;

// Core Extension: ARB_copy_buffer
pub static COPY_READ_BUFFER: GLenum = 0x8F36;
pub static COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub static COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
pub static COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;

// Version: 3.2
pub static CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub static CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub static LINES_ADJACENCY: GLenum = 0x000A;
pub static LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub static TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub static TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub static PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub static FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub static FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub static GEOMETRY_SHADER: GLenum = 0x8DD9;
pub static GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub static GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub static GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub static MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub static MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub static MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub static MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub static MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub static MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub static MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub static CONTEXT_PROFILE_MASK: GLenum = 0x9126;

// Core Extension: ARB_depth_clamp
pub static DEPTH_CLAMP: GLenum = 0x864F;

// Core Extension: ARB_provoking_vertex
pub static QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub static FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub static LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub static PROVOKING_VERTEX: GLenum = 0x8E4F;

// Core Extension: ARB_seamless_cube_map
pub static TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;

// Core Extension: ARB_sync
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
pub static TIMEOUT_IGNORED: GLenum = 0xFFFFFFFFFFFFFFFF;
pub static SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;

// Core Extension: ARB_texture_multisample
pub static SAMPLE_POSITION: GLenum = 0x8E50;
pub static SAMPLE_MASK: GLenum = 0x8E51;
pub static SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub static MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
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

// Core Extension: ARB_vertex_array_bgra
pub static BGRA: GLenum = 0x80E1;

// Version: 3.3
pub static VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;

// Core Extension: ARB_texture_rgb10_a2ui
pub static RGB10_A2UI: GLenum = 0x906F;

// Core Extension: ARB_texture_swizzle
pub static TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub static TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub static TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub static TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub static TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;

// Core Extension: ARB_timer_query
pub static TIME_ELAPSED: GLenum = 0x88BF;
pub static TIMESTAMP: GLenum = 0x8E28;

// Core Extension: ARB_vertex_type_2_10_10_10_rev
pub static UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub static INT_2_10_10_10_REV: GLenum = 0x8D9F;

// Core Extension: ARB_blend_func_extended
pub static SRC1_ALPHA: GLenum = 0x8589;
pub static SRC1_COLOR: GLenum = 0x88F9;
pub static ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub static ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub static MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;

// Core Extension: ARB_occlusion_query2
pub static ANY_SAMPLES_PASSED: GLenum = 0x8C2F;

// Core Extension: ARB_sampler_objects
pub static SAMPLER_BINDING: GLenum = 0x8919;

// Version: 4.0
pub static SAMPLE_SHADING: GLenum = 0x8C36;
pub static MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
pub static MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub static MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
pub static MAX_PROGRAM_TEXTURE_GATHER_COMPONENTS: GLenum = 0x8F9F;
pub static TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub static PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub static SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub static SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub static INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub static UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;

// Core Extension: ARB_draw_indirect
pub static DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub static DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;

// Core Extension: ARB_gpu_shader5
pub static GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub static MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
pub static MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
pub static MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
pub static FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub static MAX_VERTEX_STREAMS: GLenum = 0x8E71;

// Core Extension: ARB_gpu_shader_fp64
pub static DOUBLE: GLenum = 0x140A;
pub static DOUBLE_VEC2: GLenum = 0x8FFC;
pub static DOUBLE_VEC3: GLenum = 0x8FFD;
pub static DOUBLE_VEC4: GLenum = 0x8FFE;
pub static DOUBLE_MAT2: GLenum = 0x8F46;
pub static DOUBLE_MAT3: GLenum = 0x8F47;
pub static DOUBLE_MAT4: GLenum = 0x8F48;
pub static DOUBLE_MAT2x3: GLenum = 0x8F49;
pub static DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub static DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub static DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub static DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub static DOUBLE_MAT4x3: GLenum = 0x8F4E;

// Core Extension: ARB_shader_subroutine
/* reuse GL_UNIFORM_SIZE from ARB_uniform_buffer_object */
/* reuse GL_UNIFORM_NAME_LENGTH from ARB_uniform_buffer_object */
pub static ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub static ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub static ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub static ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub static ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub static MAX_SUBROUTINES: GLenum = 0x8DE7;
pub static MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub static NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub static COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;

// Core Extension: ARB_tessellation_shader
pub static TRIANGLES: GLenum = 0x0004;
pub static QUADS: GLenum = 0x0007;
pub static EQUAL: GLenum = 0x0202;
pub static CW: GLenum = 0x0900;
pub static CCW: GLenum = 0x0901;
pub static PATCHES: GLenum = 0x000E;
pub static PATCH_VERTICES: GLenum = 0x8E72;
pub static PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub static PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub static TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub static TESS_GEN_MODE: GLenum = 0x8E76;
pub static TESS_GEN_SPACING: GLenum = 0x8E77;
pub static TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub static TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub static ISOLINES: GLenum = 0x8E7A;
pub static FRACTIONAL_ODD: GLenum = 0x8E7B;
pub static FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub static MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub static MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub static MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub static MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub static MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub static MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub static MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub static MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub static MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub static MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub static MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub static MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub static MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub static MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub static MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub static MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub static UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub static UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
pub static TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub static TESS_CONTROL_SHADER: GLenum = 0x8E88;

// Core Extension: ARB_transform_feedback2
pub static TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub static TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub static TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub static TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
pub static TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
pub static TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;

// Core Extension: ARB_transform_feedback3
/* reuse GL_MAX_VERTEX_STREAMS from ARB_gpu_shader5 */
pub static MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;

// Core Extension: ARB_ES2_compatibility
pub static FIXED: GLenum = 0x140C;
pub static IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub static IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub static LOW_FLOAT: GLenum = 0x8DF0;
pub static MEDIUM_FLOAT: GLenum = 0x8DF1;
pub static HIGH_FLOAT: GLenum = 0x8DF2;
pub static LOW_INT: GLenum = 0x8DF3;
pub static MEDIUM_INT: GLenum = 0x8DF4;
pub static HIGH_INT: GLenum = 0x8DF5;
pub static SHADER_COMPILER: GLenum = 0x8DFA;
pub static SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub static NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub static MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub static MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub static MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub static RGB565: GLenum = 0x8D62;

// Core Extension: ARB_get_program_binary
pub static PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub static PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub static NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub static PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;

// Core Extension: ARB_separate_shader_objects
pub static VERTEX_SHADER_BIT: GLenum = 0x00000001;
pub static FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
pub static GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
pub static TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
pub static TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
pub static ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub static PROGRAM_SEPARABLE: GLenum = 0x8258;
pub static ACTIVE_PROGRAM: GLenum = 0x8259;
pub static PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;

// Core Extension: ARB_vertex_attrib_64bit
pub static RGB32I: GLenum = 0x8D83;
/* reuse GL_DOUBLE_VEC2 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_VEC3 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_VEC4 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT2 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT3 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT4 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT2x3 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT2x4 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT3x2 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT3x4 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT4x2 from ARB_gpu_shader_fp64 */
/* reuse GL_DOUBLE_MAT4x3 from ARB_gpu_shader_fp64 */

// Core Extension: ARB_viewport_array
pub static DEPTH_RANGE: GLenum = 0x0B70;
pub static VIEWPORT: GLenum = 0x0BA2;
pub static SCISSOR_BOX: GLenum = 0x0C10;
pub static SCISSOR_TEST: GLenum = 0x0C11;
/* reuse GL_FIRST_VERTEX_CONVENTION from ARB_provoking_vertex */
/* reuse GL_LAST_VERTEX_CONVENTION from ARB_provoking_vertex */
/* reuse GL_PROVOKING_VERTEX from ARB_provoking_vertex */
pub static MAX_VIEWPORTS: GLenum = 0x825B;
pub static VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub static VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub static LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub static VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub static UNDEFINED_VERTEX: GLenum = 0x8260;

// Core Extension: ARB_compressed_texture_pixel_storage
pub static UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub static UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub static UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub static UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub static PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub static PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub static PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub static PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;

// Core Extension: ARB_internalformat_query
pub static NUM_SAMPLE_COUNTS: GLenum = 0x9380;

// Core Extension: ARB_map_buffer_alignment
pub static MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;

// Core Extension: ARB_shader_atomic_counters
pub static ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub static ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub static ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub static ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub static ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub static ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub static ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub static MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub static MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub static MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub static MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub static MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub static MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub static MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub static MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub static MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub static MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub static MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub static MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub static MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub static MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub static ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub static UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub static UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;

// Core Extension: ARB_shader_image_load_store
pub static VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
pub static ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
pub static UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
pub static TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
pub static SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
pub static COMMAND_BARRIER_BIT: GLenum = 0x00000040;
pub static PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
pub static TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
pub static BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
pub static FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
pub static TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
pub static ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
pub static ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
pub static MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub static MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub static IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub static IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub static IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub static IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub static IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub static IMAGE_1D: GLenum = 0x904C;
pub static IMAGE_2D: GLenum = 0x904D;
pub static IMAGE_3D: GLenum = 0x904E;
pub static IMAGE_2D_RECT: GLenum = 0x904F;
pub static IMAGE_CUBE: GLenum = 0x9050;
pub static IMAGE_BUFFER: GLenum = 0x9051;
pub static IMAGE_1D_ARRAY: GLenum = 0x9052;
pub static IMAGE_2D_ARRAY: GLenum = 0x9053;
pub static IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub static IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub static IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub static INT_IMAGE_1D: GLenum = 0x9057;
pub static INT_IMAGE_2D: GLenum = 0x9058;
pub static INT_IMAGE_3D: GLenum = 0x9059;
pub static INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub static INT_IMAGE_CUBE: GLenum = 0x905B;
pub static INT_IMAGE_BUFFER: GLenum = 0x905C;
pub static INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub static INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub static INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub static INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub static INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub static UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub static UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub static UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub static UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub static UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub static UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub static UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub static UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub static UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub static UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub static UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub static MAX_IMAGE_SAMPLES: GLenum = 0x906D;
pub static IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub static IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub static IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub static IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub static MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub static MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub static MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub static MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub static MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub static MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;

// Core Extension: ARB_texture_storage
pub static TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;

// Version: 4.3
pub static NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub static VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;

// Core Extension: KHR_debug
pub static STACK_OVERFLOW: GLenum = 0x0503;
pub static STACK_UNDERFLOW: GLenum = 0x0504;
pub static DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub static DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub static DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub static DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub static DEBUG_SOURCE_API: GLenum = 0x8246;
pub static DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub static DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub static DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub static DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub static DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub static DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub static DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub static DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub static DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub static DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub static DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub static MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub static MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub static DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub static DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub static DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub static DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub static DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub static DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub static DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub static DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub static MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub static DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub static BUFFER: GLenum = 0x82E0;
pub static SHADER: GLenum = 0x82E1;
pub static PROGRAM: GLenum = 0x82E2;
pub static QUERY: GLenum = 0x82E3;
pub static PROGRAM_PIPELINE: GLenum = 0x82E4;
pub static SAMPLER: GLenum = 0x82E6;
pub static DISPLAY_LIST: GLenum = 0x82E7;
pub static MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub static DEBUG_OUTPUT: GLenum = 0x92E0;
pub static CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;

// Core Extension: ARB_compute_shader
pub static COMPUTE_SHADER: GLenum = 0x91B9;
pub static MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub static MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub static MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub static MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub static MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub static MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub static MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub static MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub static MAX_COMPUTE_LOCAL_INVOCATIONS: GLenum = 0x90EB;
pub static MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub static MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub static COMPUTE_LOCAL_WORK_SIZE: GLenum = 0x8267;
pub static UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub static DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub static DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub static COMPUTE_SHADER_BIT: GLenum = 0x00000020;

// Core Extension: ARB_ES3_compatibility
pub static COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub static COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub static COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub static COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub static COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub static COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub static COMPRESSED_R11_EAC: GLenum = 0x9270;
pub static COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub static COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub static COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub static PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub static ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub static MAX_ELEMENT_INDEX: GLenum = 0x8D6B;

// Core Extension: ARB_explicit_uniform_location
pub static MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;

// Core Extension: ARB_framebuffer_no_attachments
pub static FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub static FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub static FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub static FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub static FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub static MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub static MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub static MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub static MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;

// Core Extension: ARB_internalformat_query2
pub static TEXTURE_1D: GLenum = 0x0DE0;
pub static TEXTURE_2D: GLenum = 0x0DE1;
pub static TEXTURE_3D: GLenum = 0x806F;
pub static SAMPLES: GLenum = 0x80A9;
pub static TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub static TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub static TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub static TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
/* reuse GL_RENDERBUFFER from ARB_framebuffer_object */
pub static TEXTURE_BUFFER: GLenum = 0x8C2A;
pub static TEXTURE_RECTANGLE: GLenum = 0x84F5;
/* reuse GL_TEXTURE_2D_MULTISAMPLE from ARB_texture_multisample */
/* reuse GL_TEXTURE_2D_MULTISAMPLE_ARRAY from ARB_texture_multisample */
pub static TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
/* reuse GL_NUM_SAMPLE_COUNTS from ARB_internalformat_query */
/* reuse GL_IMAGE_FORMAT_COMPATIBILITY_TYPE from ARB_shader_image_load_store */
pub static INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub static INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub static INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub static INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub static INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub static INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub static INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub static INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub static INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub static INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub static INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub static INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub static INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub static INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub static INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub static MAX_WIDTH: GLenum = 0x827E;
pub static MAX_HEIGHT: GLenum = 0x827F;
pub static MAX_DEPTH: GLenum = 0x8280;
pub static MAX_LAYERS: GLenum = 0x8281;
pub static MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub static COLOR_COMPONENTS: GLenum = 0x8283;
pub static DEPTH_COMPONENTS: GLenum = 0x8284;
pub static STENCIL_COMPONENTS: GLenum = 0x8285;
pub static COLOR_RENDERABLE: GLenum = 0x8286;
pub static DEPTH_RENDERABLE: GLenum = 0x8287;
pub static STENCIL_RENDERABLE: GLenum = 0x8288;
pub static FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub static FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub static FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub static READ_PIXELS: GLenum = 0x828C;
pub static READ_PIXELS_FORMAT: GLenum = 0x828D;
pub static READ_PIXELS_TYPE: GLenum = 0x828E;
pub static TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub static TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub static GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub static GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub static MIPMAP: GLenum = 0x8293;
pub static MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub static AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub static COLOR_ENCODING: GLenum = 0x8296;
pub static SRGB_READ: GLenum = 0x8297;
pub static SRGB_WRITE: GLenum = 0x8298;
pub static FILTER: GLenum = 0x829A;
pub static VERTEX_TEXTURE: GLenum = 0x829B;
pub static TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub static TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub static GEOMETRY_TEXTURE: GLenum = 0x829E;
pub static FRAGMENT_TEXTURE: GLenum = 0x829F;
pub static COMPUTE_TEXTURE: GLenum = 0x82A0;
pub static TEXTURE_SHADOW: GLenum = 0x82A1;
pub static TEXTURE_GATHER: GLenum = 0x82A2;
pub static TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub static SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub static SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub static SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub static IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub static IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub static IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub static IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub static SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub static SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub static SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub static SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub static TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub static TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub static TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub static CLEAR_BUFFER: GLenum = 0x82B4;
pub static TEXTURE_VIEW: GLenum = 0x82B5;
pub static VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub static FULL_SUPPORT: GLenum = 0x82B7;
pub static CAVEAT_SUPPORT: GLenum = 0x82B8;
pub static IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub static IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub static IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub static IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub static IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub static IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub static IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub static IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub static IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub static IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub static IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub static VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub static VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub static VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub static VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub static VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub static VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub static VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub static VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub static VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub static VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub static VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub static VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub static VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub static VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub static VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub static VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub static SRGB_DECODE_ARB: GLenum = 0x8299;

// Core Extension: ARB_program_interface_query
/* reuse GL_NUM_COMPATIBLE_SUBROUTINES from ARB_shader_subroutine */
/* reuse GL_COMPATIBLE_SUBROUTINES from ARB_shader_subroutine */
/* reuse GL_ATOMIC_COUNTER_BUFFER from ARB_shader_atomic_counters */
pub static UNIFORM: GLenum = 0x92E1;
pub static UNIFORM_BLOCK: GLenum = 0x92E2;
pub static PROGRAM_INPUT: GLenum = 0x92E3;
pub static PROGRAM_OUTPUT: GLenum = 0x92E4;
pub static BUFFER_VARIABLE: GLenum = 0x92E5;
pub static SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub static VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub static TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub static TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub static GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub static FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub static COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub static VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub static TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub static TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub static GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub static FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub static COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub static TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub static ACTIVE_RESOURCES: GLenum = 0x92F5;
pub static MAX_NAME_LENGTH: GLenum = 0x92F6;
pub static MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub static MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub static NAME_LENGTH: GLenum = 0x92F9;
pub static TYPE: GLenum = 0x92FA;
pub static ARRAY_SIZE: GLenum = 0x92FB;
pub static OFFSET: GLenum = 0x92FC;
pub static BLOCK_INDEX: GLenum = 0x92FD;
pub static ARRAY_STRIDE: GLenum = 0x92FE;
pub static MATRIX_STRIDE: GLenum = 0x92FF;
pub static IS_ROW_MAJOR: GLenum = 0x9300;
pub static ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub static BUFFER_BINDING: GLenum = 0x9302;
pub static BUFFER_DATA_SIZE: GLenum = 0x9303;
pub static NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub static ACTIVE_VARIABLES: GLenum = 0x9305;
pub static REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub static REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub static REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub static REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub static REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub static REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub static TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub static TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub static LOCATION: GLenum = 0x930E;
pub static LOCATION_INDEX: GLenum = 0x930F;
pub static IS_PER_PATCH: GLenum = 0x92E7;

// Core Extension: ARB_shader_storage_buffer_object
/* reuse GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS from ARB_shader_image_load_store */
pub static SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub static SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub static SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub static SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub static MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub static MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub static MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub static MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub static MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub static MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub static MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub static MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub static MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub static SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub static SHADER_STORAGE_BARRIER_BIT: GLenum = 0x2000;
pub static MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;

// Core Extension: ARB_stencil_texturing
pub static DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;

// Core Extension: ARB_texture_buffer_range
pub static TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub static TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub static TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;

// Core Extension: ARB_texture_view
pub static TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub static TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub static TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub static TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub static TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;

// Core Extension: ARB_vertex_attrib_binding
pub static VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub static VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub static VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub static VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub static VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub static MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub static MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;


pub struct FPointer<F> { f: F, is_loaded: bool }

priv impl<F> FPointer<F> {
    fn load(fn_ptr: *c_void, def: F) -> FPointer<F> {
        do fn_ptr.to_option().map_default(
            FPointer { f: def, is_loaded: false }
        ) |_| {
            FPointer { f: cast::transmute(fn_ptr), is_loaded: true }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//
// Function pointer storage struct
//
//////////////////////////////////////////////////////////////////////////////

pub struct GL {
	// Version: 1.1
	CullFace: FPointer<extern "C" fn(mode: GLenum) -> void>,
	FrontFace: FPointer<extern "C" fn(mode: GLenum) -> void>,
	Hint: FPointer<extern "C" fn(target: GLenum, mode: GLenum) -> void>,
	LineWidth: FPointer<extern "C" fn(width: GLfloat) -> void>,
	PointSize: FPointer<extern "C" fn(size: GLfloat) -> void>,
	PolygonMode: FPointer<extern "C" fn(face: GLenum, mode: GLenum) -> void>,
	Scissor: FPointer<extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> void>,
	TexParameterf: FPointer<extern "C" fn(target: GLenum, pname: GLenum, param: GLfloat) -> void>,
	TexParameterfv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLfloat) -> void>,
	TexParameteri: FPointer<extern "C" fn(target: GLenum, pname: GLenum, param: GLint) -> void>,
	TexParameteriv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	TexImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	TexImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	DrawBuffer: FPointer<extern "C" fn(mode: GLenum) -> void>,
	Clear: FPointer<extern "C" fn(mask: GLbitfield) -> void>,
	ClearColor: FPointer<extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> void>,
	ClearStencil: FPointer<extern "C" fn(s: GLint) -> void>,
	ClearDepth: FPointer<extern "C" fn(depth: GLdouble) -> void>,
	StencilMask: FPointer<extern "C" fn(mask: GLuint) -> void>,
	ColorMask: FPointer<extern "C" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> void>,
	DepthMask: FPointer<extern "C" fn(flag: GLboolean) -> void>,
	Disable: FPointer<extern "C" fn(cap: GLenum) -> void>,
	Enable: FPointer<extern "C" fn(cap: GLenum) -> void>,
	Finish: FPointer<extern "C" fn() -> void>,
	Flush: FPointer<extern "C" fn() -> void>,
	BlendFunc: FPointer<extern "C" fn(sfactor: GLenum, dfactor: GLenum) -> void>,
	LogicOp: FPointer<extern "C" fn(opcode: GLenum) -> void>,
	StencilFunc: FPointer<extern "C" fn(func: GLenum, ref_: GLint, mask: GLuint) -> void>,
	StencilOp: FPointer<extern "C" fn(fail: GLenum, zfail: GLenum, zpass: GLenum) -> void>,
	DepthFunc: FPointer<extern "C" fn(func: GLenum) -> void>,
	PixelStoref: FPointer<extern "C" fn(pname: GLenum, param: GLfloat) -> void>,
	PixelStorei: FPointer<extern "C" fn(pname: GLenum, param: GLint) -> void>,
	ReadBuffer: FPointer<extern "C" fn(mode: GLenum) -> void>,
	ReadPixels: FPointer<extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	GetBooleanv: FPointer<extern "C" fn(pname: GLenum, params: *GLboolean) -> void>,
	GetDoublev: FPointer<extern "C" fn(pname: GLenum, params: *GLdouble) -> void>,
	GetError: FPointer<extern "C" fn() -> GLenum>,
	GetFloatv: FPointer<extern "C" fn(pname: GLenum, params: *GLfloat) -> void>,
	GetIntegerv: FPointer<extern "C" fn(pname: GLenum, params: *GLint) -> void>,
	GetString: FPointer<extern "C" fn(name: GLenum) -> const GLubyte *>,
	GetTexImage: FPointer<extern "C" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	GetTexParameterfv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLfloat) -> void>,
	GetTexParameteriv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	GetTexLevelParameterfv: FPointer<extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *GLfloat) -> void>,
	GetTexLevelParameteriv: FPointer<extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *GLint) -> void>,
	IsEnabled: FPointer<extern "C" fn(cap: GLenum) -> GLboolean>,
	DepthRange: FPointer<extern "C" fn(near: GLdouble, far: GLdouble) -> void>,
	Viewport: FPointer<extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> void>,
	DrawArrays: FPointer<extern "C" fn(mode: GLenum, first: GLint, count: GLsizei) -> void>,
	DrawElements: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid) -> void>,
	GetPointerv: FPointer<extern "C" fn(pname: GLenum, params: *GLvoid*) -> void>,
	PolygonOffset: FPointer<extern "C" fn(factor: GLfloat, units: GLfloat) -> void>,
	CopyTexImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> void>,
	CopyTexImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> void>,
	CopyTexSubImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> void>,
	CopyTexSubImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> void>,
	TexSubImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	TexSubImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	BindTexture: FPointer<extern "C" fn(target: GLenum, texture: GLuint) -> void>,
	DeleteTextures: FPointer<extern "C" fn(n: GLsizei, textures: *GLuint) -> void>,
	GenTextures: FPointer<extern "C" fn(n: GLsizei, textures: *GLuint) -> void>,
	IsTexture: FPointer<extern "C" fn(texture: GLuint) -> GLboolean>,
	Indexub: FPointer<extern "C" fn(c: GLubyte) -> void>,
	Indexubv: FPointer<extern "C" fn(c: *GLubyte) -> void>,
	
	// Version: 1.2
	BlendColor: FPointer<extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> void>,
	BlendEquation: FPointer<extern "C" fn(mode: GLenum) -> void>,
	DrawRangeElements: FPointer<extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid) -> void>,
	TexImage3D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	TexSubImage3D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) -> void>,
	CopyTexSubImage3D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> void>,
	
	// Version: 1.3
	ActiveTexture: FPointer<extern "C" fn(texture: GLenum) -> void>,
	SampleCoverage: FPointer<extern "C" fn(value: GLfloat, invert: GLboolean) -> void>,
	CompressedTexImage3D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) -> void>,
	CompressedTexImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) -> void>,
	CompressedTexImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) -> void>,
	CompressedTexSubImage3D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) -> void>,
	CompressedTexSubImage2D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) -> void>,
	CompressedTexSubImage1D: FPointer<extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) -> void>,
	GetCompressedTexImage: FPointer<extern "C" fn(target: GLenum, level: GLint, img: *GLvoid) -> void>,
	
	// Version: 1.4
	BlendFuncSeparate: FPointer<extern "C" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> void>,
	MultiDrawArrays: FPointer<extern "C" fn(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei) -> void>,
	MultiDrawElements: FPointer<extern "C" fn(mode: GLenum, count: *GLsizei, type_: GLenum, indices: *GLvoid* const, drawcount: GLsizei) -> void>,
	PointParameterf: FPointer<extern "C" fn(pname: GLenum, param: GLfloat) -> void>,
	PointParameterfv: FPointer<extern "C" fn(pname: GLenum, params: *GLfloat) -> void>,
	PointParameteri: FPointer<extern "C" fn(pname: GLenum, param: GLint) -> void>,
	PointParameteriv: FPointer<extern "C" fn(pname: GLenum, params: *GLint) -> void>,
	
	// Version: 1.5
	GenQueries: FPointer<extern "C" fn(n: GLsizei, ids: *GLuint) -> void>,
	DeleteQueries: FPointer<extern "C" fn(n: GLsizei, ids: *GLuint) -> void>,
	IsQuery: FPointer<extern "C" fn(id: GLuint) -> GLboolean>,
	BeginQuery: FPointer<extern "C" fn(target: GLenum, id: GLuint) -> void>,
	EndQuery: FPointer<extern "C" fn(target: GLenum) -> void>,
	GetQueryiv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	GetQueryObjectiv: FPointer<extern "C" fn(id: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetQueryObjectuiv: FPointer<extern "C" fn(id: GLuint, pname: GLenum, params: *GLuint) -> void>,
	BindBuffer: FPointer<extern "C" fn(target: GLenum, buffer: GLuint) -> void>,
	DeleteBuffers: FPointer<extern "C" fn(n: GLsizei, buffers: *GLuint) -> void>,
	GenBuffers: FPointer<extern "C" fn(n: GLsizei, buffers: *GLuint) -> void>,
	IsBuffer: FPointer<extern "C" fn(buffer: GLuint) -> GLboolean>,
	BufferData: FPointer<extern "C" fn(target: GLenum, size: GLsizeiptr, data: *GLvoid, usage: GLenum) -> void>,
	BufferSubData: FPointer<extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid) -> void>,
	GetBufferSubData: FPointer<extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid) -> void>,
	MapBuffer: FPointer<extern "C" fn(target: GLenum, access: GLenum) -> GLvoid*>,
	UnmapBuffer: FPointer<extern "C" fn(target: GLenum) -> GLboolean>,
	GetBufferParameteriv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	GetBufferPointerv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLvoid*) -> void>,
	
	// Version: 2.0
	BlendEquationSeparate: FPointer<extern "C" fn(modeRGB: GLenum, modeAlpha: GLenum) -> void>,
	DrawBuffers: FPointer<extern "C" fn(n: GLsizei, bufs: *GLenum) -> void>,
	StencilOpSeparate: FPointer<extern "C" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> void>,
	StencilFuncSeparate: FPointer<extern "C" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> void>,
	StencilMaskSeparate: FPointer<extern "C" fn(face: GLenum, mask: GLuint) -> void>,
	AttachShader: FPointer<extern "C" fn(program: GLuint, shader: GLuint) -> void>,
	BindAttribLocation: FPointer<extern "C" fn(program: GLuint, index: GLuint, name: *GLchar) -> void>,
	CompileShader: FPointer<extern "C" fn(shader: GLuint) -> void>,
	CreateProgram: FPointer<extern "C" fn() -> GLuint>,
	CreateShader: FPointer<extern "C" fn(type_: GLenum) -> GLuint>,
	DeleteProgram: FPointer<extern "C" fn(program: GLuint) -> void>,
	DeleteShader: FPointer<extern "C" fn(shader: GLuint) -> void>,
	DetachShader: FPointer<extern "C" fn(program: GLuint, shader: GLuint) -> void>,
	DisableVertexAttribArray: FPointer<extern "C" fn(index: GLuint) -> void>,
	EnableVertexAttribArray: FPointer<extern "C" fn(index: GLuint) -> void>,
	GetActiveAttrib: FPointer<extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, type_: *GLenum, name: *GLchar) -> void>,
	GetActiveUniform: FPointer<extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, type_: *GLenum, name: *GLchar) -> void>,
	GetAttachedShaders: FPointer<extern "C" fn(program: GLuint, maxCount: GLsizei, count: *GLsizei, obj: *GLuint) -> void>,
	GetAttribLocation: FPointer<extern "C" fn(program: GLuint, name: *GLchar) -> GLint>,
	GetProgramiv: FPointer<extern "C" fn(program: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetProgramInfoLog: FPointer<extern "C" fn(program: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) -> void>,
	GetShaderiv: FPointer<extern "C" fn(shader: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetShaderInfoLog: FPointer<extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) -> void>,
	GetShaderSource: FPointer<extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *GLsizei, source: *GLchar) -> void>,
	GetUniformLocation: FPointer<extern "C" fn(program: GLuint, name: *GLchar) -> GLint>,
	GetUniformfv: FPointer<extern "C" fn(program: GLuint, location: GLint, params: *GLfloat) -> void>,
	GetUniformiv: FPointer<extern "C" fn(program: GLuint, location: GLint, params: *GLint) -> void>,
	GetVertexAttribdv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLdouble) -> void>,
	GetVertexAttribfv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLfloat) -> void>,
	GetVertexAttribiv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetVertexAttribPointerv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, pointer: *GLvoid*) -> void>,
	IsProgram: FPointer<extern "C" fn(program: GLuint) -> GLboolean>,
	IsShader: FPointer<extern "C" fn(shader: GLuint) -> GLboolean>,
	LinkProgram: FPointer<extern "C" fn(program: GLuint) -> void>,
	ShaderSource: FPointer<extern "C" fn(shader: GLuint, count: GLsizei, string: *GLchar* const, length: *GLint) -> void>,
	UseProgram: FPointer<extern "C" fn(program: GLuint) -> void>,
	Uniform1f: FPointer<extern "C" fn(location: GLint, v0: GLfloat) -> void>,
	Uniform2f: FPointer<extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat) -> void>,
	Uniform3f: FPointer<extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> void>,
	Uniform4f: FPointer<extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> void>,
	Uniform1i: FPointer<extern "C" fn(location: GLint, v0: GLint) -> void>,
	Uniform2i: FPointer<extern "C" fn(location: GLint, v0: GLint, v1: GLint) -> void>,
	Uniform3i: FPointer<extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> void>,
	Uniform4i: FPointer<extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> void>,
	Uniform1fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	Uniform2fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	Uniform3fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	Uniform4fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	Uniform1iv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLint) -> void>,
	Uniform2iv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLint) -> void>,
	Uniform3iv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLint) -> void>,
	Uniform4iv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLint) -> void>,
	UniformMatrix2fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix3fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix4fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ValidateProgram: FPointer<extern "C" fn(program: GLuint) -> void>,
	VertexAttribPointer: FPointer<extern "C" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *GLvoid) -> void>,
	
	// Version: 2.1
	UniformMatrix2x3fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix3x2fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix2x4fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix4x2fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix3x4fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	UniformMatrix4x3fv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	
	// Version: 3.0
	ColorMaski: FPointer<extern "C" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> void>,
	GetBooleani_v: FPointer<extern "C" fn(target: GLenum, index: GLuint, data: *GLboolean) -> void>,
	GetIntegeri_v: FPointer<extern "C" fn(target: GLenum, index: GLuint, data: *GLint) -> void>,
	Enablei: FPointer<extern "C" fn(target: GLenum, index: GLuint) -> void>,
	Disablei: FPointer<extern "C" fn(target: GLenum, index: GLuint) -> void>,
	IsEnabledi: FPointer<extern "C" fn(target: GLenum, index: GLuint) -> GLboolean>,
	BeginTransformFeedback: FPointer<extern "C" fn(primitiveMode: GLenum) -> void>,
	EndTransformFeedback: FPointer<extern "C" fn() -> void>,
	BindBufferRange: FPointer<extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> void>,
	BindBufferBase: FPointer<extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint) -> void>,
	TransformFeedbackVaryings: FPointer<extern "C" fn(program: GLuint, count: GLsizei, varyings: *GLchar* const, bufferMode: GLenum) -> void>,
	GetTransformFeedbackVarying: FPointer<extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLsizei, type_: *GLenum, name: *GLchar) -> void>,
	ClampColor: FPointer<extern "C" fn(target: GLenum, clamp: GLenum) -> void>,
	BeginConditionalRender: FPointer<extern "C" fn(id: GLuint, mode: GLenum) -> void>,
	EndConditionalRender: FPointer<extern "C" fn() -> void>,
	VertexAttribIPointer: FPointer<extern "C" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) -> void>,
	GetVertexAttribIiv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetVertexAttribIuiv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLuint) -> void>,
	VertexAttribI1i: FPointer<extern "C" fn(index: GLuint, x: GLint) -> void>,
	VertexAttribI2i: FPointer<extern "C" fn(index: GLuint, x: GLint, y: GLint) -> void>,
	VertexAttribI3i: FPointer<extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint) -> void>,
	VertexAttribI4i: FPointer<extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> void>,
	VertexAttribI1ui: FPointer<extern "C" fn(index: GLuint, x: GLuint) -> void>,
	VertexAttribI2ui: FPointer<extern "C" fn(index: GLuint, x: GLuint, y: GLuint) -> void>,
	VertexAttribI3ui: FPointer<extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> void>,
	VertexAttribI4ui: FPointer<extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> void>,
	VertexAttribI1iv: FPointer<extern "C" fn(index: GLuint, v: *GLint) -> void>,
	VertexAttribI2iv: FPointer<extern "C" fn(index: GLuint, v: *GLint) -> void>,
	VertexAttribI3iv: FPointer<extern "C" fn(index: GLuint, v: *GLint) -> void>,
	VertexAttribI4iv: FPointer<extern "C" fn(index: GLuint, v: *GLint) -> void>,
	VertexAttribI1uiv: FPointer<extern "C" fn(index: GLuint, v: *GLuint) -> void>,
	VertexAttribI2uiv: FPointer<extern "C" fn(index: GLuint, v: *GLuint) -> void>,
	VertexAttribI3uiv: FPointer<extern "C" fn(index: GLuint, v: *GLuint) -> void>,
	VertexAttribI4uiv: FPointer<extern "C" fn(index: GLuint, v: *GLuint) -> void>,
	VertexAttribI4bv: FPointer<extern "C" fn(index: GLuint, v: *GLbyte) -> void>,
	VertexAttribI4sv: FPointer<extern "C" fn(index: GLuint, v: *GLshort) -> void>,
	VertexAttribI4ubv: FPointer<extern "C" fn(index: GLuint, v: *GLubyte) -> void>,
	VertexAttribI4usv: FPointer<extern "C" fn(index: GLuint, v: *GLushort) -> void>,
	GetUniformuiv: FPointer<extern "C" fn(program: GLuint, location: GLint, params: *GLuint) -> void>,
	BindFragDataLocation: FPointer<extern "C" fn(program: GLuint, color: GLuint, name: *GLchar) -> void>,
	GetFragDataLocation: FPointer<extern "C" fn(program: GLuint, name: *GLchar) -> GLint>,
	Uniform1ui: FPointer<extern "C" fn(location: GLint, v0: GLuint) -> void>,
	Uniform2ui: FPointer<extern "C" fn(location: GLint, v0: GLuint, v1: GLuint) -> void>,
	Uniform3ui: FPointer<extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> void>,
	Uniform4ui: FPointer<extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> void>,
	Uniform1uiv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLuint) -> void>,
	Uniform2uiv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLuint) -> void>,
	Uniform3uiv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLuint) -> void>,
	Uniform4uiv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLuint) -> void>,
	TexParameterIiv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	TexParameterIuiv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLuint) -> void>,
	GetTexParameterIiv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	GetTexParameterIuiv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLuint) -> void>,
	ClearBufferiv: FPointer<extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *GLint) -> void>,
	ClearBufferuiv: FPointer<extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *GLuint) -> void>,
	ClearBufferfv: FPointer<extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *GLfloat) -> void>,
	ClearBufferfi: FPointer<extern "C" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> void>,
	GetStringi: FPointer<extern "C" fn(name: GLenum, index: GLuint) -> const GLubyte *>,
	
	// Core Extension: ARB_vertex_array_object
	BindVertexArray: FPointer<extern "C" fn(array: GLuint) -> void>,
	DeleteVertexArrays: FPointer<extern "C" fn(n: GLsizei, arrays: *GLuint) -> void>,
	GenVertexArrays: FPointer<extern "C" fn(n: GLsizei, arrays: *GLuint) -> void>,
	IsVertexArray: FPointer<extern "C" fn(array: GLuint) -> GLboolean>,
	
	// Core Extension: ARB_map_buffer_range
	MapBufferRange: FPointer<extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> GLvoid*>,
	FlushMappedBufferRange: FPointer<extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> void>,
	
	// Core Extension: ARB_framebuffer_object
	IsRenderbuffer: FPointer<extern "C" fn(renderbuffer: GLuint) -> GLboolean>,
	BindRenderbuffer: FPointer<extern "C" fn(target: GLenum, renderbuffer: GLuint) -> void>,
	DeleteRenderbuffers: FPointer<extern "C" fn(n: GLsizei, renderbuffers: *GLuint) -> void>,
	GenRenderbuffers: FPointer<extern "C" fn(n: GLsizei, renderbuffers: *GLuint) -> void>,
	RenderbufferStorage: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> void>,
	GetRenderbufferParameteriv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	IsFramebuffer: FPointer<extern "C" fn(framebuffer: GLuint) -> GLboolean>,
	BindFramebuffer: FPointer<extern "C" fn(target: GLenum, framebuffer: GLuint) -> void>,
	DeleteFramebuffers: FPointer<extern "C" fn(n: GLsizei, framebuffers: *GLuint) -> void>,
	GenFramebuffers: FPointer<extern "C" fn(n: GLsizei, framebuffers: *GLuint) -> void>,
	CheckFramebufferStatus: FPointer<extern "C" fn(target: GLenum) -> GLenum>,
	FramebufferTexture1D: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> void>,
	FramebufferTexture2D: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> void>,
	FramebufferTexture3D: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> void>,
	FramebufferRenderbuffer: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> void>,
	GetFramebufferAttachmentParameteriv: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *GLint) -> void>,
	GenerateMipmap: FPointer<extern "C" fn(target: GLenum) -> void>,
	BlitFramebuffer: FPointer<extern "C" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> void>,
	RenderbufferStorageMultisample: FPointer<extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> void>,
	FramebufferTextureLayer: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> void>,
	
	// Version: 3.1
	DrawArraysInstanced: FPointer<extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> void>,
	DrawElementsInstanced: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei) -> void>,
	TexBuffer: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, buffer: GLuint) -> void>,
	PrimitiveRestartIndex: FPointer<extern "C" fn(index: GLuint) -> void>,
	
	// Core Extension: ARB_uniform_buffer_object
	GetUniformIndices: FPointer<extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *GLchar* const, uniformIndices: *GLuint) -> void>,
	GetActiveUniformsiv: FPointer<extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *GLint) -> void>,
	GetActiveUniformName: FPointer<extern "C" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *GLsizei, uniformName: *GLchar) -> void>,
	GetUniformBlockIndex: FPointer<extern "C" fn(program: GLuint, uniformBlockName: *GLchar) -> GLuint>,
	GetActiveUniformBlockiv: FPointer<extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetActiveUniformBlockName: FPointer<extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *GLsizei, uniformBlockName: *GLchar) -> void>,
	UniformBlockBinding: FPointer<extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> void>,
	
	// Core Extension: ARB_copy_buffer
	CopyBufferSubData: FPointer<extern "C" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> void>,
	
	// Version: 3.2
	GetInteger64i_v: FPointer<extern "C" fn(target: GLenum, index: GLuint, data: *GLint64) -> void>,
	GetBufferParameteri64v: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint64) -> void>,
	FramebufferTexture: FPointer<extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> void>,
	
	// Core Extension: ARB_draw_elements_base_vertex
	DrawElementsBaseVertex: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) -> void>,
	DrawRangeElementsBaseVertex: FPointer<extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) -> void>,
	DrawElementsInstancedBaseVertex: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, basevertex: GLint) -> void>,
	MultiDrawElementsBaseVertex: FPointer<extern "C" fn(mode: GLenum, count: *GLsizei, type_: GLenum, indices: *GLvoid* const, drawcount: GLsizei, basevertex: *GLint) -> void>,
	
	// Core Extension: ARB_provoking_vertex
	ProvokingVertex: FPointer<extern "C" fn(mode: GLenum) -> void>,
	
	// Core Extension: ARB_sync
	FenceSync: FPointer<extern "C" fn(condition: GLenum, flags: GLbitfield) -> GLsync>,
	IsSync: FPointer<extern "C" fn(sync: GLsync) -> GLboolean>,
	DeleteSync: FPointer<extern "C" fn(sync: GLsync) -> void>,
	ClientWaitSync: FPointer<extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum>,
	WaitSync: FPointer<extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> void>,
	GetInteger64v: FPointer<extern "C" fn(pname: GLenum, params: *GLint64) -> void>,
	GetSynciv: FPointer<extern "C" fn(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *GLsizei, values: *GLint) -> void>,
	
	// Core Extension: ARB_texture_multisample
	TexImage2DMultisample: FPointer<extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLint, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	TexImage3DMultisample: FPointer<extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	GetMultisamplefv: FPointer<extern "C" fn(pname: GLenum, index: GLuint, val: *GLfloat) -> void>,
	SampleMaski: FPointer<extern "C" fn(index: GLuint, mask: GLbitfield) -> void>,
	
	// Version: 3.3
	VertexAttribDivisor: FPointer<extern "C" fn(index: GLuint, divisor: GLuint) -> void>,
	
	// Core Extension: ARB_timer_query
	QueryCounter: FPointer<extern "C" fn(id: GLuint, target: GLenum) -> void>,
	GetQueryObjecti64v: FPointer<extern "C" fn(id: GLuint, pname: GLenum, params: *GLint64) -> void>,
	GetQueryObjectui64v: FPointer<extern "C" fn(id: GLuint, pname: GLenum, params: *GLuint64) -> void>,
	
	// Core Extension: ARB_vertex_type_2_10_10_10_rev
	VertexP2ui: FPointer<extern "C" fn(type_: GLenum, value: GLuint) -> void>,
	VertexP2uiv: FPointer<extern "C" fn(type_: GLenum, value: *GLuint) -> void>,
	VertexP3ui: FPointer<extern "C" fn(type_: GLenum, value: GLuint) -> void>,
	VertexP3uiv: FPointer<extern "C" fn(type_: GLenum, value: *GLuint) -> void>,
	VertexP4ui: FPointer<extern "C" fn(type_: GLenum, value: GLuint) -> void>,
	VertexP4uiv: FPointer<extern "C" fn(type_: GLenum, value: *GLuint) -> void>,
	TexCoordP1ui: FPointer<extern "C" fn(type_: GLenum, coords: GLuint) -> void>,
	TexCoordP1uiv: FPointer<extern "C" fn(type_: GLenum, coords: *GLuint) -> void>,
	TexCoordP2ui: FPointer<extern "C" fn(type_: GLenum, coords: GLuint) -> void>,
	TexCoordP2uiv: FPointer<extern "C" fn(type_: GLenum, coords: *GLuint) -> void>,
	TexCoordP3ui: FPointer<extern "C" fn(type_: GLenum, coords: GLuint) -> void>,
	TexCoordP3uiv: FPointer<extern "C" fn(type_: GLenum, coords: *GLuint) -> void>,
	TexCoordP4ui: FPointer<extern "C" fn(type_: GLenum, coords: GLuint) -> void>,
	TexCoordP4uiv: FPointer<extern "C" fn(type_: GLenum, coords: *GLuint) -> void>,
	MultiTexCoordP1ui: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: GLuint) -> void>,
	MultiTexCoordP1uiv: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: *GLuint) -> void>,
	MultiTexCoordP2ui: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: GLuint) -> void>,
	MultiTexCoordP2uiv: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: *GLuint) -> void>,
	MultiTexCoordP3ui: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: GLuint) -> void>,
	MultiTexCoordP3uiv: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: *GLuint) -> void>,
	MultiTexCoordP4ui: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: GLuint) -> void>,
	MultiTexCoordP4uiv: FPointer<extern "C" fn(texture: GLenum, type_: GLenum, coords: *GLuint) -> void>,
	NormalP3ui: FPointer<extern "C" fn(type_: GLenum, coords: GLuint) -> void>,
	NormalP3uiv: FPointer<extern "C" fn(type_: GLenum, coords: *GLuint) -> void>,
	ColorP3ui: FPointer<extern "C" fn(type_: GLenum, color: GLuint) -> void>,
	ColorP3uiv: FPointer<extern "C" fn(type_: GLenum, color: *GLuint) -> void>,
	ColorP4ui: FPointer<extern "C" fn(type_: GLenum, color: GLuint) -> void>,
	ColorP4uiv: FPointer<extern "C" fn(type_: GLenum, color: *GLuint) -> void>,
	SecondaryColorP3ui: FPointer<extern "C" fn(type_: GLenum, color: GLuint) -> void>,
	SecondaryColorP3uiv: FPointer<extern "C" fn(type_: GLenum, color: *GLuint) -> void>,
	VertexAttribP1ui: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> void>,
	VertexAttribP1uiv: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) -> void>,
	VertexAttribP2ui: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> void>,
	VertexAttribP2uiv: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) -> void>,
	VertexAttribP3ui: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> void>,
	VertexAttribP3uiv: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) -> void>,
	VertexAttribP4ui: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> void>,
	VertexAttribP4uiv: FPointer<extern "C" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) -> void>,
	
	// Core Extension: ARB_blend_func_extended
	BindFragDataLocationIndexed: FPointer<extern "C" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *GLchar) -> void>,
	GetFragDataIndex: FPointer<extern "C" fn(program: GLuint, name: *GLchar) -> GLint>,
	
	// Core Extension: ARB_sampler_objects
	GenSamplers: FPointer<extern "C" fn(count: GLsizei, samplers: *GLuint) -> void>,
	DeleteSamplers: FPointer<extern "C" fn(count: GLsizei, samplers: *GLuint) -> void>,
	IsSampler: FPointer<extern "C" fn(sampler: GLuint) -> GLboolean>,
	BindSampler: FPointer<extern "C" fn(unit: GLuint, sampler: GLuint) -> void>,
	SamplerParameteri: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: GLint) -> void>,
	SamplerParameteriv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: *GLint) -> void>,
	SamplerParameterf: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: GLfloat) -> void>,
	SamplerParameterfv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: *GLfloat) -> void>,
	SamplerParameterIiv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: *GLint) -> void>,
	SamplerParameterIuiv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, param: *GLuint) -> void>,
	GetSamplerParameteriv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetSamplerParameterIiv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, params: *GLint) -> void>,
	GetSamplerParameterfv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, params: *GLfloat) -> void>,
	GetSamplerParameterIuiv: FPointer<extern "C" fn(sampler: GLuint, pname: GLenum, params: *GLuint) -> void>,
	
	// Version: 4.0
	MinSampleShading: FPointer<extern "C" fn(value: GLfloat) -> void>,
	BlendEquationi: FPointer<extern "C" fn(buf: GLuint, mode: GLenum) -> void>,
	BlendEquationSeparatei: FPointer<extern "C" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) -> void>,
	BlendFunci: FPointer<extern "C" fn(buf: GLuint, src: GLenum, dst: GLenum) -> void>,
	BlendFuncSeparatei: FPointer<extern "C" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) -> void>,
	
	// Core Extension: ARB_draw_indirect
	DrawArraysIndirect: FPointer<extern "C" fn(mode: GLenum, indirect: *GLvoid) -> void>,
	DrawElementsIndirect: FPointer<extern "C" fn(mode: GLenum, type_: GLenum, indirect: *GLvoid) -> void>,
	
	// Core Extension: ARB_gpu_shader_fp64
	Uniform1d: FPointer<extern "C" fn(location: GLint, x: GLdouble) -> void>,
	Uniform2d: FPointer<extern "C" fn(location: GLint, x: GLdouble, y: GLdouble) -> void>,
	Uniform3d: FPointer<extern "C" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) -> void>,
	Uniform4d: FPointer<extern "C" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> void>,
	Uniform1dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	Uniform2dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	Uniform3dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	Uniform4dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	UniformMatrix2dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix3dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix4dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix2x3dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix2x4dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix3x2dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix3x4dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix4x2dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	UniformMatrix4x3dv: FPointer<extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	GetUniformdv: FPointer<extern "C" fn(program: GLuint, location: GLint, params: *GLdouble) -> void>,
	
	// Core Extension: ARB_shader_subroutine
	GetSubroutineUniformLocation: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLint>,
	GetSubroutineIndex: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLuint>,
	GetActiveSubroutineUniformiv: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *GLint) -> void>,
	GetActiveSubroutineUniformName: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *GLsizei, name: *GLchar) -> void>,
	GetActiveSubroutineName: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *GLsizei, name: *GLchar) -> void>,
	UniformSubroutinesuiv: FPointer<extern "C" fn(shadertype: GLenum, count: GLsizei, indices: *GLuint) -> void>,
	GetUniformSubroutineuiv: FPointer<extern "C" fn(shadertype: GLenum, location: GLint, params: *GLuint) -> void>,
	GetProgramStageiv: FPointer<extern "C" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *GLint) -> void>,
	
	// Core Extension: ARB_tessellation_shader
	PatchParameteri: FPointer<extern "C" fn(pname: GLenum, value: GLint) -> void>,
	PatchParameterfv: FPointer<extern "C" fn(pname: GLenum, values: *GLfloat) -> void>,
	
	// Core Extension: ARB_transform_feedback2
	BindTransformFeedback: FPointer<extern "C" fn(target: GLenum, id: GLuint) -> void>,
	DeleteTransformFeedbacks: FPointer<extern "C" fn(n: GLsizei, ids: *GLuint) -> void>,
	GenTransformFeedbacks: FPointer<extern "C" fn(n: GLsizei, ids: *GLuint) -> void>,
	IsTransformFeedback: FPointer<extern "C" fn(id: GLuint) -> GLboolean>,
	PauseTransformFeedback: FPointer<extern "C" fn() -> void>,
	ResumeTransformFeedback: FPointer<extern "C" fn() -> void>,
	DrawTransformFeedback: FPointer<extern "C" fn(mode: GLenum, id: GLuint) -> void>,
	
	// Core Extension: ARB_transform_feedback3
	DrawTransformFeedbackStream: FPointer<extern "C" fn(mode: GLenum, id: GLuint, stream: GLuint) -> void>,
	BeginQueryIndexed: FPointer<extern "C" fn(target: GLenum, index: GLuint, id: GLuint) -> void>,
	EndQueryIndexed: FPointer<extern "C" fn(target: GLenum, index: GLuint) -> void>,
	GetQueryIndexediv: FPointer<extern "C" fn(target: GLenum, index: GLuint, pname: GLenum, params: *GLint) -> void>,
	
	// Core Extension: ARB_ES2_compatibility
	ReleaseShaderCompiler: FPointer<extern "C" fn() -> void>,
	ShaderBinary: FPointer<extern "C" fn(count: GLsizei, shaders: *GLuint, binaryformat: GLenum, binary: *GLvoid, length: GLsizei) -> void>,
	GetShaderPrecisionFormat: FPointer<extern "C" fn(shadertype: GLenum, precisiontype: GLenum, range: *GLint, precision: *GLint) -> void>,
	DepthRangef: FPointer<extern "C" fn(n: GLfloat, f: GLfloat) -> void>,
	ClearDepthf: FPointer<extern "C" fn(d: GLfloat) -> void>,
	
	// Core Extension: ARB_get_program_binary
	GetProgramBinary: FPointer<extern "C" fn(program: GLuint, bufSize: GLsizei, length: *GLsizei, binaryFormat: *GLenum, binary: *GLvoid) -> void>,
	ProgramBinary: FPointer<extern "C" fn(program: GLuint, binaryFormat: GLenum, binary: *GLvoid, length: GLsizei) -> void>,
	ProgramParameteri: FPointer<extern "C" fn(program: GLuint, pname: GLenum, value: GLint) -> void>,
	
	// Core Extension: ARB_separate_shader_objects
	UseProgramStages: FPointer<extern "C" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint) -> void>,
	ActiveShaderProgram: FPointer<extern "C" fn(pipeline: GLuint, program: GLuint) -> void>,
	CreateShaderProgramv: FPointer<extern "C" fn(type_: GLenum, count: GLsizei, strings: *GLchar* const) -> GLuint>,
	BindProgramPipeline: FPointer<extern "C" fn(pipeline: GLuint) -> void>,
	DeleteProgramPipelines: FPointer<extern "C" fn(n: GLsizei, pipelines: *GLuint) -> void>,
	GenProgramPipelines: FPointer<extern "C" fn(n: GLsizei, pipelines: *GLuint) -> void>,
	IsProgramPipeline: FPointer<extern "C" fn(pipeline: GLuint) -> GLboolean>,
	GetProgramPipelineiv: FPointer<extern "C" fn(pipeline: GLuint, pname: GLenum, params: *GLint) -> void>,
	ProgramUniform1i: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLint) -> void>,
	ProgramUniform1iv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLint) -> void>,
	ProgramUniform1f: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLfloat) -> void>,
	ProgramUniform1fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	ProgramUniform1d: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLdouble) -> void>,
	ProgramUniform1dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	ProgramUniform1ui: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLuint) -> void>,
	ProgramUniform1uiv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) -> void>,
	ProgramUniform2i: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint) -> void>,
	ProgramUniform2iv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLint) -> void>,
	ProgramUniform2f: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) -> void>,
	ProgramUniform2fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	ProgramUniform2d: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) -> void>,
	ProgramUniform2dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	ProgramUniform2ui: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) -> void>,
	ProgramUniform2uiv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) -> void>,
	ProgramUniform3i: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) -> void>,
	ProgramUniform3iv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLint) -> void>,
	ProgramUniform3f: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> void>,
	ProgramUniform3fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	ProgramUniform3d: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) -> void>,
	ProgramUniform3dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	ProgramUniform3ui: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> void>,
	ProgramUniform3uiv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) -> void>,
	ProgramUniform4i: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> void>,
	ProgramUniform4iv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLint) -> void>,
	ProgramUniform4f: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> void>,
	ProgramUniform4fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) -> void>,
	ProgramUniform4d: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) -> void>,
	ProgramUniform4dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) -> void>,
	ProgramUniform4ui: FPointer<extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> void>,
	ProgramUniform4uiv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) -> void>,
	ProgramUniformMatrix2fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix3fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix4fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix2dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix3dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix4dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix2x3fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix3x2fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix2x4fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix4x2fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix3x4fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix4x3fv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) -> void>,
	ProgramUniformMatrix2x3dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix3x2dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix2x4dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix4x2dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix3x4dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ProgramUniformMatrix4x3dv: FPointer<extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) -> void>,
	ValidateProgramPipeline: FPointer<extern "C" fn(pipeline: GLuint) -> void>,
	GetProgramPipelineInfoLog: FPointer<extern "C" fn(pipeline: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) -> void>,
	
	// Core Extension: ARB_vertex_attrib_64bit
	VertexAttribL1d: FPointer<extern "C" fn(index: GLuint, x: GLdouble) -> void>,
	VertexAttribL2d: FPointer<extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble) -> void>,
	VertexAttribL3d: FPointer<extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> void>,
	VertexAttribL4d: FPointer<extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> void>,
	VertexAttribL1dv: FPointer<extern "C" fn(index: GLuint, v: *GLdouble) -> void>,
	VertexAttribL2dv: FPointer<extern "C" fn(index: GLuint, v: *GLdouble) -> void>,
	VertexAttribL3dv: FPointer<extern "C" fn(index: GLuint, v: *GLdouble) -> void>,
	VertexAttribL4dv: FPointer<extern "C" fn(index: GLuint, v: *GLdouble) -> void>,
	VertexAttribLPointer: FPointer<extern "C" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) -> void>,
	GetVertexAttribLdv: FPointer<extern "C" fn(index: GLuint, pname: GLenum, params: *GLdouble) -> void>,
	
	// Core Extension: ARB_viewport_array
	ViewportArrayv: FPointer<extern "C" fn(first: GLuint, count: GLsizei, v: *GLfloat) -> void>,
	ViewportIndexedf: FPointer<extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) -> void>,
	ViewportIndexedfv: FPointer<extern "C" fn(index: GLuint, v: *GLfloat) -> void>,
	ScissorArrayv: FPointer<extern "C" fn(first: GLuint, count: GLsizei, v: *GLint) -> void>,
	ScissorIndexed: FPointer<extern "C" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) -> void>,
	ScissorIndexedv: FPointer<extern "C" fn(index: GLuint, v: *GLint) -> void>,
	DepthRangeArrayv: FPointer<extern "C" fn(first: GLuint, count: GLsizei, v: *GLdouble) -> void>,
	DepthRangeIndexed: FPointer<extern "C" fn(index: GLuint, n: GLdouble, f: GLdouble) -> void>,
	GetFloati_v: FPointer<extern "C" fn(target: GLenum, index: GLuint, data: *GLfloat) -> void>,
	GetDoublei_v: FPointer<extern "C" fn(target: GLenum, index: GLuint, data: *GLdouble) -> void>,
	
	// Core Extension: ARB_base_instance
	DrawArraysInstancedBaseInstance: FPointer<extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) -> void>,
	DrawElementsInstancedBaseInstance: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *void, instancecount: GLsizei, baseinstance: GLuint) -> void>,
	DrawElementsInstancedBaseVertexBaseInstance: FPointer<extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) -> void>,
	
	// Core Extension: ARB_transform_feedback_instanced
	DrawTransformFeedbackInstanced: FPointer<extern "C" fn(mode: GLenum, id: GLuint, instancecount: GLsizei) -> void>,
	DrawTransformFeedbackStreamInstanced: FPointer<extern "C" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) -> void>,
	
	// Core Extension: ARB_internalformat_query
	GetInternalformativ: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *GLint) -> void>,
	
	// Core Extension: ARB_shader_atomic_counters
	GetActiveAtomicCounterBufferiv: FPointer<extern "C" fn(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *GLint) -> void>,
	
	// Core Extension: ARB_shader_image_load_store
	BindImageTexture: FPointer<extern "C" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) -> void>,
	MemoryBarrier: FPointer<extern "C" fn(barriers: GLbitfield) -> void>,
	
	// Core Extension: ARB_texture_storage
	TexStorage1D: FPointer<extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> void>,
	TexStorage2D: FPointer<extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> void>,
	TexStorage3D: FPointer<extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> void>,
	TextureStorage1DEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> void>,
	TextureStorage2DEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> void>,
	TextureStorage3DEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> void>,
	
	// Core Extension: KHR_debug
	DebugMessageControl: FPointer<extern "C" fn(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *GLuint, enabled: GLboolean) -> void>,
	DebugMessageInsert: FPointer<extern "C" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *GLchar) -> void>,
	DebugMessageCallback: FPointer<extern "C" fn(callback: GLDEBUGPROC, userParam: *void) -> void>,
	GetDebugMessageLog: FPointer<extern "C" fn(count: GLuint, bufsize: GLsizei, sources: *GLenum, types: *GLenum, ids: *GLuint, severities: *GLenum, lengths: *GLsizei, messageLog: *GLchar) -> GLuint>,
	PushDebugGroup: FPointer<extern "C" fn(source: GLenum, id: GLuint, length: GLsizei, message: *GLchar) -> void>,
	PopDebugGroup: FPointer<extern "C" fn() -> void>,
	ObjectLabel: FPointer<extern "C" fn(identifier: GLenum, name: GLuint, length: GLsizei, label: *GLchar) -> void>,
	GetObjectLabel: FPointer<extern "C" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *GLsizei, label: *GLchar) -> void>,
	ObjectPtrLabel: FPointer<extern "C" fn(ptr: *void, length: GLsizei, label: *GLchar) -> void>,
	GetObjectPtrLabel: FPointer<extern "C" fn(ptr: *void, bufSize: GLsizei, length: *GLsizei, label: *GLchar) -> void>,
	
	// Core Extension: ARB_clear_buffer_object
	ClearBufferData: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *void) -> void>,
	ClearBufferSubData: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *void) -> void>,
	ClearNamedBufferDataEXT: FPointer<extern "C" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *void) -> void>,
	ClearNamedBufferSubDataEXT: FPointer<extern "C" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, offset: GLsizeiptr, size: GLsizeiptr, data: *void) -> void>,
	
	// Core Extension: ARB_compute_shader
	DispatchCompute: FPointer<extern "C" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> void>,
	DispatchComputeIndirect: FPointer<extern "C" fn(indirect: GLintptr) -> void>,
	
	// Core Extension: ARB_copy_image
	CopyImageSubData: FPointer<extern "C" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) -> void>,
	
	// Core Extension: ARB_framebuffer_no_attachments
	FramebufferParameteri: FPointer<extern "C" fn(target: GLenum, pname: GLenum, param: GLint) -> void>,
	GetFramebufferParameteriv: FPointer<extern "C" fn(target: GLenum, pname: GLenum, params: *GLint) -> void>,
	NamedFramebufferParameteriEXT: FPointer<extern "C" fn(framebuffer: GLuint, pname: GLenum, param: GLint) -> void>,
	GetNamedFramebufferParameterivEXT: FPointer<extern "C" fn(framebuffer: GLuint, pname: GLenum, params: *GLint) -> void>,
	
	// Core Extension: ARB_internalformat_query2
	GetInternalformati64v: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *GLint64) -> void>,
	
	// Core Extension: ARB_invalidate_subdata
	InvalidateTexSubImage: FPointer<extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> void>,
	InvalidateTexImage: FPointer<extern "C" fn(texture: GLuint, level: GLint) -> void>,
	InvalidateBufferSubData: FPointer<extern "C" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> void>,
	InvalidateBufferData: FPointer<extern "C" fn(buffer: GLuint) -> void>,
	InvalidateFramebuffer: FPointer<extern "C" fn(target: GLenum, numAttachments: GLsizei, attachments: *GLenum) -> void>,
	InvalidateSubFramebuffer: FPointer<extern "C" fn(target: GLenum, numAttachments: GLsizei, attachments: *GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> void>,
	
	// Core Extension: ARB_multi_draw_indirect
	MultiDrawArraysIndirect: FPointer<extern "C" fn(mode: GLenum, indirect: *void, drawcount: GLsizei, stride: GLsizei) -> void>,
	MultiDrawElementsIndirect: FPointer<extern "C" fn(mode: GLenum, type_: GLenum, indirect: *void, drawcount: GLsizei, stride: GLsizei) -> void>,
	
	// Core Extension: ARB_program_interface_query
	GetProgramInterfaceiv: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, pname: GLenum, params: *GLint) -> void>,
	GetProgramResourceIndex: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLuint>,
	GetProgramResourceName: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *GLsizei, name: *GLchar) -> void>,
	GetProgramResourceiv: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *GLenum, bufSize: GLsizei, length: *GLsizei, params: *GLint) -> void>,
	GetProgramResourceLocation: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint>,
	GetProgramResourceLocationIndex: FPointer<extern "C" fn(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint>,
	
	// Core Extension: ARB_shader_storage_buffer_object
	ShaderStorageBlockBinding: FPointer<extern "C" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) -> void>,
	
	// Core Extension: ARB_texture_buffer_range
	TexBufferRange: FPointer<extern "C" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> void>,
	TextureBufferRangeEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> void>,
	
	// Core Extension: ARB_texture_storage_multisample
	TexStorage2DMultisample: FPointer<extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	TexStorage3DMultisample: FPointer<extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	TextureStorage2DMultisampleEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	TextureStorage3DMultisampleEXT: FPointer<extern "C" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> void>,
	
	// Core Extension: ARB_texture_view
	TextureView: FPointer<extern "C" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) -> void>,
	
	// Core Extension: ARB_vertex_attrib_binding
	BindVertexBuffer: FPointer<extern "C" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> void>,
	VertexAttribFormat: FPointer<extern "C" fn(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> void>,
	VertexAttribIFormat: FPointer<extern "C" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> void>,
	VertexAttribLFormat: FPointer<extern "C" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> void>,
	VertexAttribBinding: FPointer<extern "C" fn(attribindex: GLuint, bindingindex: GLuint) -> void>,
	VertexBindingDivisor: FPointer<extern "C" fn(bindingindex: GLuint, divisor: GLuint) -> void>,
	VertexArrayBindVertexBufferEXT: FPointer<extern "C" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> void>,
	VertexArrayVertexAttribFormatEXT: FPointer<extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> void>,
	VertexArrayVertexAttribIFormatEXT: FPointer<extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> void>,
	VertexArrayVertexAttribLFormatEXT: FPointer<extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> void>,
	VertexArrayVertexAttribBindingEXT: FPointer<extern "C" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) -> void>,
	VertexArrayVertexBindingDivisorEXT: FPointer<extern "C" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) -> void>,
	
}

priv mod failing {
	use core::libc::*;
	use types::*;
	
	// Version: 1.1
	pub extern "C" fn glCullFace(_: GLenum) { fail!("Function glCullFace not initialised") }
	pub extern "C" fn glFrontFace(_: GLenum) { fail!("Function glFrontFace not initialised") }
	pub extern "C" fn glHint(_: GLenum, _: GLenum) { fail!("Function glHint not initialised") }
	pub extern "C" fn glLineWidth(_: GLfloat) { fail!("Function glLineWidth not initialised") }
	pub extern "C" fn glPointSize(_: GLfloat) { fail!("Function glPointSize not initialised") }
	pub extern "C" fn glPolygonMode(_: GLenum, _: GLenum) { fail!("Function glPolygonMode not initialised") }
	pub extern "C" fn glScissor(_: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glScissor not initialised") }
	pub extern "C" fn glTexParameterf(_: GLenum, _: GLenum, _: GLfloat) { fail!("Function glTexParameterf not initialised") }
	pub extern "C" fn glTexParameterfv(_: GLenum, _: GLenum, _: *GLfloat) { fail!("Function glTexParameterfv not initialised") }
	pub extern "C" fn glTexParameteri(_: GLenum, _: GLenum, _: GLint) { fail!("Function glTexParameteri not initialised") }
	pub extern "C" fn glTexParameteriv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glTexParameteriv not initialised") }
	pub extern "C" fn glTexImage1D(_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexImage1D not initialised") }
	pub extern "C" fn glTexImage2D(_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexImage2D not initialised") }
	pub extern "C" fn glDrawBuffer(_: GLenum) { fail!("Function glDrawBuffer not initialised") }
	pub extern "C" fn glClear(_: GLbitfield) { fail!("Function glClear not initialised") }
	pub extern "C" fn glClearColor(_: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glClearColor not initialised") }
	pub extern "C" fn glClearStencil(_: GLint) { fail!("Function glClearStencil not initialised") }
	pub extern "C" fn glClearDepth(_: GLdouble) { fail!("Function glClearDepth not initialised") }
	pub extern "C" fn glStencilMask(_: GLuint) { fail!("Function glStencilMask not initialised") }
	pub extern "C" fn glColorMask(_: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) { fail!("Function glColorMask not initialised") }
	pub extern "C" fn glDepthMask(_: GLboolean) { fail!("Function glDepthMask not initialised") }
	pub extern "C" fn glDisable(_: GLenum) { fail!("Function glDisable not initialised") }
	pub extern "C" fn glEnable(_: GLenum) { fail!("Function glEnable not initialised") }
	pub extern "C" fn glFinish() { fail!("Function glFinish not initialised") }
	pub extern "C" fn glFlush() { fail!("Function glFlush not initialised") }
	pub extern "C" fn glBlendFunc(_: GLenum, _: GLenum) { fail!("Function glBlendFunc not initialised") }
	pub extern "C" fn glLogicOp(_: GLenum) { fail!("Function glLogicOp not initialised") }
	pub extern "C" fn glStencilFunc(_: GLenum, _: GLint, _: GLuint) { fail!("Function glStencilFunc not initialised") }
	pub extern "C" fn glStencilOp(_: GLenum, _: GLenum, _: GLenum) { fail!("Function glStencilOp not initialised") }
	pub extern "C" fn glDepthFunc(_: GLenum) { fail!("Function glDepthFunc not initialised") }
	pub extern "C" fn glPixelStoref(_: GLenum, _: GLfloat) { fail!("Function glPixelStoref not initialised") }
	pub extern "C" fn glPixelStorei(_: GLenum, _: GLint) { fail!("Function glPixelStorei not initialised") }
	pub extern "C" fn glReadBuffer(_: GLenum) { fail!("Function glReadBuffer not initialised") }
	pub extern "C" fn glReadPixels(_: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glReadPixels not initialised") }
	pub extern "C" fn glGetBooleanv(_: GLenum, _: *GLboolean) { fail!("Function glGetBooleanv not initialised") }
	pub extern "C" fn glGetDoublev(_: GLenum, _: *GLdouble) { fail!("Function glGetDoublev not initialised") }
	pub extern "C" fn glGetError() -> GLenum { fail!("Function glGetError not initialised") }
	pub extern "C" fn glGetFloatv(_: GLenum, _: *GLfloat) { fail!("Function glGetFloatv not initialised") }
	pub extern "C" fn glGetIntegerv(_: GLenum, _: *GLint) { fail!("Function glGetIntegerv not initialised") }
	pub extern "C" fn glGetString(_: GLenum) -> *GLubyte { fail!("Function glGetString not initialised") }
	pub extern "C" fn glGetTexImage(_: GLenum, _: GLint, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glGetTexImage not initialised") }
	pub extern "C" fn glGetTexParameterfv(_: GLenum, _: GLenum, _: *GLfloat) { fail!("Function glGetTexParameterfv not initialised") }
	pub extern "C" fn glGetTexParameteriv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetTexParameteriv not initialised") }
	pub extern "C" fn glGetTexLevelParameterfv(_: GLenum, _: GLint, _: GLenum, _: *GLfloat) { fail!("Function glGetTexLevelParameterfv not initialised") }
	pub extern "C" fn glGetTexLevelParameteriv(_: GLenum, _: GLint, _: GLenum, _: *GLint) { fail!("Function glGetTexLevelParameteriv not initialised") }
	pub extern "C" fn glIsEnabled(_: GLenum) -> GLboolean { fail!("Function glIsEnabled not initialised") }
	pub extern "C" fn glDepthRange(_: GLdouble, _: GLdouble) { fail!("Function glDepthRange not initialised") }
	pub extern "C" fn glViewport(_: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glViewport not initialised") }
	pub extern "C" fn glDrawArrays(_: GLenum, _: GLint, _: GLsizei) { fail!("Function glDrawArrays not initialised") }
	pub extern "C" fn glDrawElements(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid) { fail!("Function glDrawElements not initialised") }
	pub extern "C" fn glGetPointerv(_: GLenum, _: **GLvoid) { fail!("Function glGetPointerv not initialised") }
	pub extern "C" fn glPolygonOffset(_: GLfloat, _: GLfloat) { fail!("Function glPolygonOffset not initialised") }
	pub extern "C" fn glCopyTexImage1D(_: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint) { fail!("Function glCopyTexImage1D not initialised") }
	pub extern "C" fn glCopyTexImage2D(_: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint) { fail!("Function glCopyTexImage2D not initialised") }
	pub extern "C" fn glCopyTexSubImage1D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei) { fail!("Function glCopyTexSubImage1D not initialised") }
	pub extern "C" fn glCopyTexSubImage2D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glCopyTexSubImage2D not initialised") }
	pub extern "C" fn glTexSubImage1D(_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexSubImage1D not initialised") }
	pub extern "C" fn glTexSubImage2D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexSubImage2D not initialised") }
	pub extern "C" fn glBindTexture(_: GLenum, _: GLuint) { fail!("Function glBindTexture not initialised") }
	pub extern "C" fn glDeleteTextures(_: GLsizei, _: *GLuint) { fail!("Function glDeleteTextures not initialised") }
	pub extern "C" fn glGenTextures(_: GLsizei, _: *GLuint) { fail!("Function glGenTextures not initialised") }
	pub extern "C" fn glIsTexture(_: GLuint) -> GLboolean { fail!("Function glIsTexture not initialised") }
	pub extern "C" fn glIndexub(_: GLubyte) { fail!("Function glIndexub not initialised") }
	pub extern "C" fn glIndexubv(_: *GLubyte) { fail!("Function glIndexubv not initialised") }
	
	// Version: 1.2
	pub extern "C" fn glBlendColor(_: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glBlendColor not initialised") }
	pub extern "C" fn glBlendEquation(_: GLenum) { fail!("Function glBlendEquation not initialised") }
	pub extern "C" fn glDrawRangeElements(_: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *GLvoid) { fail!("Function glDrawRangeElements not initialised") }
	pub extern "C" fn glTexImage3D(_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexImage3D not initialised") }
	pub extern "C" fn glTexSubImage3D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glTexSubImage3D not initialised") }
	pub extern "C" fn glCopyTexSubImage3D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glCopyTexSubImage3D not initialised") }
	
	// Version: 1.3
	pub extern "C" fn glActiveTexture(_: GLenum) { fail!("Function glActiveTexture not initialised") }
	pub extern "C" fn glSampleCoverage(_: GLfloat, _: GLboolean) { fail!("Function glSampleCoverage not initialised") }
	pub extern "C" fn glCompressedTexImage3D(_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexImage3D not initialised") }
	pub extern "C" fn glCompressedTexImage2D(_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexImage2D not initialised") }
	pub extern "C" fn glCompressedTexImage1D(_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLint, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexImage1D not initialised") }
	pub extern "C" fn glCompressedTexSubImage3D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexSubImage3D not initialised") }
	pub extern "C" fn glCompressedTexSubImage2D(_: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexSubImage2D not initialised") }
	pub extern "C" fn glCompressedTexSubImage1D(_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLsizei, _: *GLvoid) { fail!("Function glCompressedTexSubImage1D not initialised") }
	pub extern "C" fn glGetCompressedTexImage(_: GLenum, _: GLint, _: *GLvoid) { fail!("Function glGetCompressedTexImage not initialised") }
	
	// Version: 1.4
	pub extern "C" fn glBlendFuncSeparate(_: GLenum, _: GLenum, _: GLenum, _: GLenum) { fail!("Function glBlendFuncSeparate not initialised") }
	pub extern "C" fn glMultiDrawArrays(_: GLenum, _: *GLint, _: *GLsizei, _: GLsizei) { fail!("Function glMultiDrawArrays not initialised") }
	pub extern "C" fn glMultiDrawElements(_: GLenum, _: *GLsizei, _: GLenum, _: **GLvoid, _: GLsizei) { fail!("Function glMultiDrawElements not initialised") }
	pub extern "C" fn glPointParameterf(_: GLenum, _: GLfloat) { fail!("Function glPointParameterf not initialised") }
	pub extern "C" fn glPointParameterfv(_: GLenum, _: *GLfloat) { fail!("Function glPointParameterfv not initialised") }
	pub extern "C" fn glPointParameteri(_: GLenum, _: GLint) { fail!("Function glPointParameteri not initialised") }
	pub extern "C" fn glPointParameteriv(_: GLenum, _: *GLint) { fail!("Function glPointParameteriv not initialised") }
	
	// Version: 1.5
	pub extern "C" fn glGenQueries(_: GLsizei, _: *GLuint) { fail!("Function glGenQueries not initialised") }
	pub extern "C" fn glDeleteQueries(_: GLsizei, _: *GLuint) { fail!("Function glDeleteQueries not initialised") }
	pub extern "C" fn glIsQuery(_: GLuint) -> GLboolean { fail!("Function glIsQuery not initialised") }
	pub extern "C" fn glBeginQuery(_: GLenum, _: GLuint) { fail!("Function glBeginQuery not initialised") }
	pub extern "C" fn glEndQuery(_: GLenum) { fail!("Function glEndQuery not initialised") }
	pub extern "C" fn glGetQueryiv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetQueryiv not initialised") }
	pub extern "C" fn glGetQueryObjectiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetQueryObjectiv not initialised") }
	pub extern "C" fn glGetQueryObjectuiv(_: GLuint, _: GLenum, _: *GLuint) { fail!("Function glGetQueryObjectuiv not initialised") }
	pub extern "C" fn glBindBuffer(_: GLenum, _: GLuint) { fail!("Function glBindBuffer not initialised") }
	pub extern "C" fn glDeleteBuffers(_: GLsizei, _: *GLuint) { fail!("Function glDeleteBuffers not initialised") }
	pub extern "C" fn glGenBuffers(_: GLsizei, _: *GLuint) { fail!("Function glGenBuffers not initialised") }
	pub extern "C" fn glIsBuffer(_: GLuint) -> GLboolean { fail!("Function glIsBuffer not initialised") }
	pub extern "C" fn glBufferData(_: GLenum, _: GLsizeiptr, _: *GLvoid, _: GLenum) { fail!("Function glBufferData not initialised") }
	pub extern "C" fn glBufferSubData(_: GLenum, _: GLintptr, _: GLsizeiptr, _: *GLvoid) { fail!("Function glBufferSubData not initialised") }
	pub extern "C" fn glGetBufferSubData(_: GLenum, _: GLintptr, _: GLsizeiptr, _: *GLvoid) { fail!("Function glGetBufferSubData not initialised") }
	pub extern "C" fn glMapBuffer(_: GLenum, _: GLenum) -> *GLvoid { fail!("Function glMapBuffer not initialised") }
	pub extern "C" fn glUnmapBuffer(_: GLenum) -> GLboolean { fail!("Function glUnmapBuffer not initialised") }
	pub extern "C" fn glGetBufferParameteriv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetBufferParameteriv not initialised") }
	pub extern "C" fn glGetBufferPointerv(_: GLenum, _: GLenum, _: **GLvoid) { fail!("Function glGetBufferPointerv not initialised") }
	
	// Version: 2.0
	pub extern "C" fn glBlendEquationSeparate(_: GLenum, _: GLenum) { fail!("Function glBlendEquationSeparate not initialised") }
	pub extern "C" fn glDrawBuffers(_: GLsizei, _: *GLenum) { fail!("Function glDrawBuffers not initialised") }
	pub extern "C" fn glStencilOpSeparate(_: GLenum, _: GLenum, _: GLenum, _: GLenum) { fail!("Function glStencilOpSeparate not initialised") }
	pub extern "C" fn glStencilFuncSeparate(_: GLenum, _: GLenum, _: GLint, _: GLuint) { fail!("Function glStencilFuncSeparate not initialised") }
	pub extern "C" fn glStencilMaskSeparate(_: GLenum, _: GLuint) { fail!("Function glStencilMaskSeparate not initialised") }
	pub extern "C" fn glAttachShader(_: GLuint, _: GLuint) { fail!("Function glAttachShader not initialised") }
	pub extern "C" fn glBindAttribLocation(_: GLuint, _: GLuint, _: *GLchar) { fail!("Function glBindAttribLocation not initialised") }
	pub extern "C" fn glCompileShader(_: GLuint) { fail!("Function glCompileShader not initialised") }
	pub extern "C" fn glCreateProgram() -> GLuint { fail!("Function glCreateProgram not initialised") }
	pub extern "C" fn glCreateShader(_: GLenum) -> GLuint { fail!("Function glCreateShader not initialised") }
	pub extern "C" fn glDeleteProgram(_: GLuint) { fail!("Function glDeleteProgram not initialised") }
	pub extern "C" fn glDeleteShader(_: GLuint) { fail!("Function glDeleteShader not initialised") }
	pub extern "C" fn glDetachShader(_: GLuint, _: GLuint) { fail!("Function glDetachShader not initialised") }
	pub extern "C" fn glDisableVertexAttribArray(_: GLuint) { fail!("Function glDisableVertexAttribArray not initialised") }
	pub extern "C" fn glEnableVertexAttribArray(_: GLuint) { fail!("Function glEnableVertexAttribArray not initialised") }
	pub extern "C" fn glGetActiveAttrib(_: GLuint, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLint, _: *GLenum, _: *GLchar) { fail!("Function glGetActiveAttrib not initialised") }
	pub extern "C" fn glGetActiveUniform(_: GLuint, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLint, _: *GLenum, _: *GLchar) { fail!("Function glGetActiveUniform not initialised") }
	pub extern "C" fn glGetAttachedShaders(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLuint) { fail!("Function glGetAttachedShaders not initialised") }
	pub extern "C" fn glGetAttribLocation(_: GLuint, _: *GLchar) -> GLint { fail!("Function glGetAttribLocation not initialised") }
	pub extern "C" fn glGetProgramiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetProgramiv not initialised") }
	pub extern "C" fn glGetProgramInfoLog(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetProgramInfoLog not initialised") }
	pub extern "C" fn glGetShaderiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetShaderiv not initialised") }
	pub extern "C" fn glGetShaderInfoLog(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetShaderInfoLog not initialised") }
	pub extern "C" fn glGetShaderSource(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetShaderSource not initialised") }
	pub extern "C" fn glGetUniformLocation(_: GLuint, _: *GLchar) -> GLint { fail!("Function glGetUniformLocation not initialised") }
	pub extern "C" fn glGetUniformfv(_: GLuint, _: GLint, _: *GLfloat) { fail!("Function glGetUniformfv not initialised") }
	pub extern "C" fn glGetUniformiv(_: GLuint, _: GLint, _: *GLint) { fail!("Function glGetUniformiv not initialised") }
	pub extern "C" fn glGetVertexAttribdv(_: GLuint, _: GLenum, _: *GLdouble) { fail!("Function glGetVertexAttribdv not initialised") }
	pub extern "C" fn glGetVertexAttribfv(_: GLuint, _: GLenum, _: *GLfloat) { fail!("Function glGetVertexAttribfv not initialised") }
	pub extern "C" fn glGetVertexAttribiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetVertexAttribiv not initialised") }
	pub extern "C" fn glGetVertexAttribPointerv(_: GLuint, _: GLenum, _: **GLvoid) { fail!("Function glGetVertexAttribPointerv not initialised") }
	pub extern "C" fn glIsProgram(_: GLuint) -> GLboolean { fail!("Function glIsProgram not initialised") }
	pub extern "C" fn glIsShader(_: GLuint) -> GLboolean { fail!("Function glIsShader not initialised") }
	pub extern "C" fn glLinkProgram(_: GLuint) { fail!("Function glLinkProgram not initialised") }
	pub extern "C" fn glShaderSource(_: GLuint, _: GLsizei, _: **GLchar, _: *GLint) { fail!("Function glShaderSource not initialised") }
	pub extern "C" fn glUseProgram(_: GLuint) { fail!("Function glUseProgram not initialised") }
	pub extern "C" fn glUniform1f(_: GLint, _: GLfloat) { fail!("Function glUniform1f not initialised") }
	pub extern "C" fn glUniform2f(_: GLint, _: GLfloat, _: GLfloat) { fail!("Function glUniform2f not initialised") }
	pub extern "C" fn glUniform3f(_: GLint, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glUniform3f not initialised") }
	pub extern "C" fn glUniform4f(_: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glUniform4f not initialised") }
	pub extern "C" fn glUniform1i(_: GLint, _: GLint) { fail!("Function glUniform1i not initialised") }
	pub extern "C" fn glUniform2i(_: GLint, _: GLint, _: GLint) { fail!("Function glUniform2i not initialised") }
	pub extern "C" fn glUniform3i(_: GLint, _: GLint, _: GLint, _: GLint) { fail!("Function glUniform3i not initialised") }
	pub extern "C" fn glUniform4i(_: GLint, _: GLint, _: GLint, _: GLint, _: GLint) { fail!("Function glUniform4i not initialised") }
	pub extern "C" fn glUniform1fv(_: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glUniform1fv not initialised") }
	pub extern "C" fn glUniform2fv(_: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glUniform2fv not initialised") }
	pub extern "C" fn glUniform3fv(_: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glUniform3fv not initialised") }
	pub extern "C" fn glUniform4fv(_: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glUniform4fv not initialised") }
	pub extern "C" fn glUniform1iv(_: GLint, _: GLsizei, _: *GLint) { fail!("Function glUniform1iv not initialised") }
	pub extern "C" fn glUniform2iv(_: GLint, _: GLsizei, _: *GLint) { fail!("Function glUniform2iv not initialised") }
	pub extern "C" fn glUniform3iv(_: GLint, _: GLsizei, _: *GLint) { fail!("Function glUniform3iv not initialised") }
	pub extern "C" fn glUniform4iv(_: GLint, _: GLsizei, _: *GLint) { fail!("Function glUniform4iv not initialised") }
	pub extern "C" fn glUniformMatrix2fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix2fv not initialised") }
	pub extern "C" fn glUniformMatrix3fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix3fv not initialised") }
	pub extern "C" fn glUniformMatrix4fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix4fv not initialised") }
	pub extern "C" fn glValidateProgram(_: GLuint) { fail!("Function glValidateProgram not initialised") }
	pub extern "C" fn glVertexAttribPointer(_: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLsizei, _: *GLvoid) { fail!("Function glVertexAttribPointer not initialised") }
	
	// Version: 2.1
	pub extern "C" fn glUniformMatrix2x3fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix2x3fv not initialised") }
	pub extern "C" fn glUniformMatrix3x2fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix3x2fv not initialised") }
	pub extern "C" fn glUniformMatrix2x4fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix2x4fv not initialised") }
	pub extern "C" fn glUniformMatrix4x2fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix4x2fv not initialised") }
	pub extern "C" fn glUniformMatrix3x4fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix3x4fv not initialised") }
	pub extern "C" fn glUniformMatrix4x3fv(_: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glUniformMatrix4x3fv not initialised") }
	
	// Version: 3.0
	pub extern "C" fn glColorMaski(_: GLuint, _: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) { fail!("Function glColorMaski not initialised") }
	pub extern "C" fn glGetBooleani_v(_: GLenum, _: GLuint, _: *GLboolean) { fail!("Function glGetBooleani_v not initialised") }
	pub extern "C" fn glGetIntegeri_v(_: GLenum, _: GLuint, _: *GLint) { fail!("Function glGetIntegeri_v not initialised") }
	pub extern "C" fn glEnablei(_: GLenum, _: GLuint) { fail!("Function glEnablei not initialised") }
	pub extern "C" fn glDisablei(_: GLenum, _: GLuint) { fail!("Function glDisablei not initialised") }
	pub extern "C" fn glIsEnabledi(_: GLenum, _: GLuint) -> GLboolean { fail!("Function glIsEnabledi not initialised") }
	pub extern "C" fn glBeginTransformFeedback(_: GLenum) { fail!("Function glBeginTransformFeedback not initialised") }
	pub extern "C" fn glEndTransformFeedback() { fail!("Function glEndTransformFeedback not initialised") }
	pub extern "C" fn glBindBufferRange(_: GLenum, _: GLuint, _: GLuint, _: GLintptr, _: GLsizeiptr) { fail!("Function glBindBufferRange not initialised") }
	pub extern "C" fn glBindBufferBase(_: GLenum, _: GLuint, _: GLuint) { fail!("Function glBindBufferBase not initialised") }
	pub extern "C" fn glTransformFeedbackVaryings(_: GLuint, _: GLsizei, _: **GLchar, _: GLenum) { fail!("Function glTransformFeedbackVaryings not initialised") }
	pub extern "C" fn glGetTransformFeedbackVarying(_: GLuint, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLsizei, _: *GLenum, _: *GLchar) { fail!("Function glGetTransformFeedbackVarying not initialised") }
	pub extern "C" fn glClampColor(_: GLenum, _: GLenum) { fail!("Function glClampColor not initialised") }
	pub extern "C" fn glBeginConditionalRender(_: GLuint, _: GLenum) { fail!("Function glBeginConditionalRender not initialised") }
	pub extern "C" fn glEndConditionalRender() { fail!("Function glEndConditionalRender not initialised") }
	pub extern "C" fn glVertexAttribIPointer(_: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *GLvoid) { fail!("Function glVertexAttribIPointer not initialised") }
	pub extern "C" fn glGetVertexAttribIiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetVertexAttribIiv not initialised") }
	pub extern "C" fn glGetVertexAttribIuiv(_: GLuint, _: GLenum, _: *GLuint) { fail!("Function glGetVertexAttribIuiv not initialised") }
	pub extern "C" fn glVertexAttribI1i(_: GLuint, _: GLint) { fail!("Function glVertexAttribI1i not initialised") }
	pub extern "C" fn glVertexAttribI2i(_: GLuint, _: GLint, _: GLint) { fail!("Function glVertexAttribI2i not initialised") }
	pub extern "C" fn glVertexAttribI3i(_: GLuint, _: GLint, _: GLint, _: GLint) { fail!("Function glVertexAttribI3i not initialised") }
	pub extern "C" fn glVertexAttribI4i(_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) { fail!("Function glVertexAttribI4i not initialised") }
	pub extern "C" fn glVertexAttribI1ui(_: GLuint, _: GLuint) { fail!("Function glVertexAttribI1ui not initialised") }
	pub extern "C" fn glVertexAttribI2ui(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glVertexAttribI2ui not initialised") }
	pub extern "C" fn glVertexAttribI3ui(_: GLuint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glVertexAttribI3ui not initialised") }
	pub extern "C" fn glVertexAttribI4ui(_: GLuint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glVertexAttribI4ui not initialised") }
	pub extern "C" fn glVertexAttribI1iv(_: GLuint, _: *GLint) { fail!("Function glVertexAttribI1iv not initialised") }
	pub extern "C" fn glVertexAttribI2iv(_: GLuint, _: *GLint) { fail!("Function glVertexAttribI2iv not initialised") }
	pub extern "C" fn glVertexAttribI3iv(_: GLuint, _: *GLint) { fail!("Function glVertexAttribI3iv not initialised") }
	pub extern "C" fn glVertexAttribI4iv(_: GLuint, _: *GLint) { fail!("Function glVertexAttribI4iv not initialised") }
	pub extern "C" fn glVertexAttribI1uiv(_: GLuint, _: *GLuint) { fail!("Function glVertexAttribI1uiv not initialised") }
	pub extern "C" fn glVertexAttribI2uiv(_: GLuint, _: *GLuint) { fail!("Function glVertexAttribI2uiv not initialised") }
	pub extern "C" fn glVertexAttribI3uiv(_: GLuint, _: *GLuint) { fail!("Function glVertexAttribI3uiv not initialised") }
	pub extern "C" fn glVertexAttribI4uiv(_: GLuint, _: *GLuint) { fail!("Function glVertexAttribI4uiv not initialised") }
	pub extern "C" fn glVertexAttribI4bv(_: GLuint, _: *GLbyte) { fail!("Function glVertexAttribI4bv not initialised") }
	pub extern "C" fn glVertexAttribI4sv(_: GLuint, _: *GLshort) { fail!("Function glVertexAttribI4sv not initialised") }
	pub extern "C" fn glVertexAttribI4ubv(_: GLuint, _: *GLubyte) { fail!("Function glVertexAttribI4ubv not initialised") }
	pub extern "C" fn glVertexAttribI4usv(_: GLuint, _: *GLushort) { fail!("Function glVertexAttribI4usv not initialised") }
	pub extern "C" fn glGetUniformuiv(_: GLuint, _: GLint, _: *GLuint) { fail!("Function glGetUniformuiv not initialised") }
	pub extern "C" fn glBindFragDataLocation(_: GLuint, _: GLuint, _: *GLchar) { fail!("Function glBindFragDataLocation not initialised") }
	pub extern "C" fn glGetFragDataLocation(_: GLuint, _: *GLchar) -> GLint { fail!("Function glGetFragDataLocation not initialised") }
	pub extern "C" fn glUniform1ui(_: GLint, _: GLuint) { fail!("Function glUniform1ui not initialised") }
	pub extern "C" fn glUniform2ui(_: GLint, _: GLuint, _: GLuint) { fail!("Function glUniform2ui not initialised") }
	pub extern "C" fn glUniform3ui(_: GLint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glUniform3ui not initialised") }
	pub extern "C" fn glUniform4ui(_: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glUniform4ui not initialised") }
	pub extern "C" fn glUniform1uiv(_: GLint, _: GLsizei, _: *GLuint) { fail!("Function glUniform1uiv not initialised") }
	pub extern "C" fn glUniform2uiv(_: GLint, _: GLsizei, _: *GLuint) { fail!("Function glUniform2uiv not initialised") }
	pub extern "C" fn glUniform3uiv(_: GLint, _: GLsizei, _: *GLuint) { fail!("Function glUniform3uiv not initialised") }
	pub extern "C" fn glUniform4uiv(_: GLint, _: GLsizei, _: *GLuint) { fail!("Function glUniform4uiv not initialised") }
	pub extern "C" fn glTexParameterIiv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glTexParameterIiv not initialised") }
	pub extern "C" fn glTexParameterIuiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glTexParameterIuiv not initialised") }
	pub extern "C" fn glGetTexParameterIiv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetTexParameterIiv not initialised") }
	pub extern "C" fn glGetTexParameterIuiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glGetTexParameterIuiv not initialised") }
	pub extern "C" fn glClearBufferiv(_: GLenum, _: GLint, _: *GLint) { fail!("Function glClearBufferiv not initialised") }
	pub extern "C" fn glClearBufferuiv(_: GLenum, _: GLint, _: *GLuint) { fail!("Function glClearBufferuiv not initialised") }
	pub extern "C" fn glClearBufferfv(_: GLenum, _: GLint, _: *GLfloat) { fail!("Function glClearBufferfv not initialised") }
	pub extern "C" fn glClearBufferfi(_: GLenum, _: GLint, _: GLfloat, _: GLint) { fail!("Function glClearBufferfi not initialised") }
	pub extern "C" fn glGetStringi(_: GLenum, _: GLuint) -> *GLubyte { fail!("Function glGetStringi not initialised") }
	
	// Core Extension: ARB_vertex_array_object
	pub extern "C" fn glBindVertexArray(_: GLuint) { fail!("Function glBindVertexArray not initialised") }
	pub extern "C" fn glDeleteVertexArrays(_: GLsizei, _: *GLuint) { fail!("Function glDeleteVertexArrays not initialised") }
	pub extern "C" fn glGenVertexArrays(_: GLsizei, _: *GLuint) { fail!("Function glGenVertexArrays not initialised") }
	pub extern "C" fn glIsVertexArray(_: GLuint) -> GLboolean { fail!("Function glIsVertexArray not initialised") }
	
	// Core Extension: ARB_map_buffer_range
	pub extern "C" fn glMapBufferRange(_: GLenum, _: GLintptr, _: GLsizeiptr, _: GLbitfield) -> *GLvoid { fail!("Function glMapBufferRange not initialised") }
	pub extern "C" fn glFlushMappedBufferRange(_: GLenum, _: GLintptr, _: GLsizeiptr) { fail!("Function glFlushMappedBufferRange not initialised") }
	
	// Core Extension: ARB_framebuffer_object
	pub extern "C" fn glIsRenderbuffer(_: GLuint) -> GLboolean { fail!("Function glIsRenderbuffer not initialised") }
	pub extern "C" fn glBindRenderbuffer(_: GLenum, _: GLuint) { fail!("Function glBindRenderbuffer not initialised") }
	pub extern "C" fn glDeleteRenderbuffers(_: GLsizei, _: *GLuint) { fail!("Function glDeleteRenderbuffers not initialised") }
	pub extern "C" fn glGenRenderbuffers(_: GLsizei, _: *GLuint) { fail!("Function glGenRenderbuffers not initialised") }
	pub extern "C" fn glRenderbufferStorage(_: GLenum, _: GLenum, _: GLsizei, _: GLsizei) { fail!("Function glRenderbufferStorage not initialised") }
	pub extern "C" fn glGetRenderbufferParameteriv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetRenderbufferParameteriv not initialised") }
	pub extern "C" fn glIsFramebuffer(_: GLuint) -> GLboolean { fail!("Function glIsFramebuffer not initialised") }
	pub extern "C" fn glBindFramebuffer(_: GLenum, _: GLuint) { fail!("Function glBindFramebuffer not initialised") }
	pub extern "C" fn glDeleteFramebuffers(_: GLsizei, _: *GLuint) { fail!("Function glDeleteFramebuffers not initialised") }
	pub extern "C" fn glGenFramebuffers(_: GLsizei, _: *GLuint) { fail!("Function glGenFramebuffers not initialised") }
	pub extern "C" fn glCheckFramebufferStatus(_: GLenum) -> GLenum { fail!("Function glCheckFramebufferStatus not initialised") }
	pub extern "C" fn glFramebufferTexture1D(_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) { fail!("Function glFramebufferTexture1D not initialised") }
	pub extern "C" fn glFramebufferTexture2D(_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) { fail!("Function glFramebufferTexture2D not initialised") }
	pub extern "C" fn glFramebufferTexture3D(_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) { fail!("Function glFramebufferTexture3D not initialised") }
	pub extern "C" fn glFramebufferRenderbuffer(_: GLenum, _: GLenum, _: GLenum, _: GLuint) { fail!("Function glFramebufferRenderbuffer not initialised") }
	pub extern "C" fn glGetFramebufferAttachmentParameteriv(_: GLenum, _: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetFramebufferAttachmentParameteriv not initialised") }
	pub extern "C" fn glGenerateMipmap(_: GLenum) { fail!("Function glGenerateMipmap not initialised") }
	pub extern "C" fn glBlitFramebuffer(_: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLbitfield, _: GLenum) { fail!("Function glBlitFramebuffer not initialised") }
	pub extern "C" fn glRenderbufferStorageMultisample(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) { fail!("Function glRenderbufferStorageMultisample not initialised") }
	pub extern "C" fn glFramebufferTextureLayer(_: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) { fail!("Function glFramebufferTextureLayer not initialised") }
	
	// Version: 3.1
	pub extern "C" fn glDrawArraysInstanced(_: GLenum, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glDrawArraysInstanced not initialised") }
	pub extern "C" fn glDrawElementsInstanced(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid, _: GLsizei) { fail!("Function glDrawElementsInstanced not initialised") }
	pub extern "C" fn glTexBuffer(_: GLenum, _: GLenum, _: GLuint) { fail!("Function glTexBuffer not initialised") }
	pub extern "C" fn glPrimitiveRestartIndex(_: GLuint) { fail!("Function glPrimitiveRestartIndex not initialised") }
	
	// Core Extension: ARB_uniform_buffer_object
	pub extern "C" fn glGetUniformIndices(_: GLuint, _: GLsizei, _: **GLchar, _: *GLuint) { fail!("Function glGetUniformIndices not initialised") }
	pub extern "C" fn glGetActiveUniformsiv(_: GLuint, _: GLsizei, _: *GLuint, _: GLenum, _: *GLint) { fail!("Function glGetActiveUniformsiv not initialised") }
	pub extern "C" fn glGetActiveUniformName(_: GLuint, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetActiveUniformName not initialised") }
	pub extern "C" fn glGetUniformBlockIndex(_: GLuint, _: *GLchar) -> GLuint { fail!("Function glGetUniformBlockIndex not initialised") }
	pub extern "C" fn glGetActiveUniformBlockiv(_: GLuint, _: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetActiveUniformBlockiv not initialised") }
	pub extern "C" fn glGetActiveUniformBlockName(_: GLuint, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetActiveUniformBlockName not initialised") }
	pub extern "C" fn glUniformBlockBinding(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glUniformBlockBinding not initialised") }
	
	// Core Extension: ARB_copy_buffer
	pub extern "C" fn glCopyBufferSubData(_: GLenum, _: GLenum, _: GLintptr, _: GLintptr, _: GLsizeiptr) { fail!("Function glCopyBufferSubData not initialised") }
	
	// Version: 3.2
	pub extern "C" fn glGetInteger64i_v(_: GLenum, _: GLuint, _: *GLint64) { fail!("Function glGetInteger64i_v not initialised") }
	pub extern "C" fn glGetBufferParameteri64v(_: GLenum, _: GLenum, _: *GLint64) { fail!("Function glGetBufferParameteri64v not initialised") }
	pub extern "C" fn glFramebufferTexture(_: GLenum, _: GLenum, _: GLuint, _: GLint) { fail!("Function glFramebufferTexture not initialised") }
	
	// Core Extension: ARB_draw_elements_base_vertex
	pub extern "C" fn glDrawElementsBaseVertex(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid, _: GLint) { fail!("Function glDrawElementsBaseVertex not initialised") }
	pub extern "C" fn glDrawRangeElementsBaseVertex(_: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *GLvoid, _: GLint) { fail!("Function glDrawRangeElementsBaseVertex not initialised") }
	pub extern "C" fn glDrawElementsInstancedBaseVertex(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid, _: GLsizei, _: GLint) { fail!("Function glDrawElementsInstancedBaseVertex not initialised") }
	pub extern "C" fn glMultiDrawElementsBaseVertex(_: GLenum, _: *GLsizei, _: GLenum, _: **GLvoid, _: GLsizei, _: *GLint) { fail!("Function glMultiDrawElementsBaseVertex not initialised") }
	
	// Core Extension: ARB_provoking_vertex
	pub extern "C" fn glProvokingVertex(_: GLenum) { fail!("Function glProvokingVertex not initialised") }
	
	// Core Extension: ARB_sync
	pub extern "C" fn glFenceSync(_: GLenum, _: GLbitfield) -> GLsync { fail!("Function glFenceSync not initialised") }
	pub extern "C" fn glIsSync(_: GLsync) -> GLboolean { fail!("Function glIsSync not initialised") }
	pub extern "C" fn glDeleteSync(_: GLsync) { fail!("Function glDeleteSync not initialised") }
	pub extern "C" fn glClientWaitSync(_: GLsync, _: GLbitfield, _: GLuint64) -> GLenum { fail!("Function glClientWaitSync not initialised") }
	pub extern "C" fn glWaitSync(_: GLsync, _: GLbitfield, _: GLuint64) { fail!("Function glWaitSync not initialised") }
	pub extern "C" fn glGetInteger64v(_: GLenum, _: *GLint64) { fail!("Function glGetInteger64v not initialised") }
	pub extern "C" fn glGetSynciv(_: GLsync, _: GLenum, _: GLsizei, _: *GLsizei, _: *GLint) { fail!("Function glGetSynciv not initialised") }
	
	// Core Extension: ARB_texture_multisample
	pub extern "C" fn glTexImage2DMultisample(_: GLenum, _: GLsizei, _: GLint, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTexImage2DMultisample not initialised") }
	pub extern "C" fn glTexImage3DMultisample(_: GLenum, _: GLsizei, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTexImage3DMultisample not initialised") }
	pub extern "C" fn glGetMultisamplefv(_: GLenum, _: GLuint, _: *GLfloat) { fail!("Function glGetMultisamplefv not initialised") }
	pub extern "C" fn glSampleMaski(_: GLuint, _: GLbitfield) { fail!("Function glSampleMaski not initialised") }
	
	// Version: 3.3
	pub extern "C" fn glVertexAttribDivisor(_: GLuint, _: GLuint) { fail!("Function glVertexAttribDivisor not initialised") }
	
	// Core Extension: ARB_timer_query
	pub extern "C" fn glQueryCounter(_: GLuint, _: GLenum) { fail!("Function glQueryCounter not initialised") }
	pub extern "C" fn glGetQueryObjecti64v(_: GLuint, _: GLenum, _: *GLint64) { fail!("Function glGetQueryObjecti64v not initialised") }
	pub extern "C" fn glGetQueryObjectui64v(_: GLuint, _: GLenum, _: *GLuint64) { fail!("Function glGetQueryObjectui64v not initialised") }
	
	// Core Extension: ARB_vertex_type_2_10_10_10_rev
	pub extern "C" fn glVertexP2ui(_: GLenum, _: GLuint) { fail!("Function glVertexP2ui not initialised") }
	pub extern "C" fn glVertexP2uiv(_: GLenum, _: *GLuint) { fail!("Function glVertexP2uiv not initialised") }
	pub extern "C" fn glVertexP3ui(_: GLenum, _: GLuint) { fail!("Function glVertexP3ui not initialised") }
	pub extern "C" fn glVertexP3uiv(_: GLenum, _: *GLuint) { fail!("Function glVertexP3uiv not initialised") }
	pub extern "C" fn glVertexP4ui(_: GLenum, _: GLuint) { fail!("Function glVertexP4ui not initialised") }
	pub extern "C" fn glVertexP4uiv(_: GLenum, _: *GLuint) { fail!("Function glVertexP4uiv not initialised") }
	pub extern "C" fn glTexCoordP1ui(_: GLenum, _: GLuint) { fail!("Function glTexCoordP1ui not initialised") }
	pub extern "C" fn glTexCoordP1uiv(_: GLenum, _: *GLuint) { fail!("Function glTexCoordP1uiv not initialised") }
	pub extern "C" fn glTexCoordP2ui(_: GLenum, _: GLuint) { fail!("Function glTexCoordP2ui not initialised") }
	pub extern "C" fn glTexCoordP2uiv(_: GLenum, _: *GLuint) { fail!("Function glTexCoordP2uiv not initialised") }
	pub extern "C" fn glTexCoordP3ui(_: GLenum, _: GLuint) { fail!("Function glTexCoordP3ui not initialised") }
	pub extern "C" fn glTexCoordP3uiv(_: GLenum, _: *GLuint) { fail!("Function glTexCoordP3uiv not initialised") }
	pub extern "C" fn glTexCoordP4ui(_: GLenum, _: GLuint) { fail!("Function glTexCoordP4ui not initialised") }
	pub extern "C" fn glTexCoordP4uiv(_: GLenum, _: *GLuint) { fail!("Function glTexCoordP4uiv not initialised") }
	pub extern "C" fn glMultiTexCoordP1ui(_: GLenum, _: GLenum, _: GLuint) { fail!("Function glMultiTexCoordP1ui not initialised") }
	pub extern "C" fn glMultiTexCoordP1uiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glMultiTexCoordP1uiv not initialised") }
	pub extern "C" fn glMultiTexCoordP2ui(_: GLenum, _: GLenum, _: GLuint) { fail!("Function glMultiTexCoordP2ui not initialised") }
	pub extern "C" fn glMultiTexCoordP2uiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glMultiTexCoordP2uiv not initialised") }
	pub extern "C" fn glMultiTexCoordP3ui(_: GLenum, _: GLenum, _: GLuint) { fail!("Function glMultiTexCoordP3ui not initialised") }
	pub extern "C" fn glMultiTexCoordP3uiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glMultiTexCoordP3uiv not initialised") }
	pub extern "C" fn glMultiTexCoordP4ui(_: GLenum, _: GLenum, _: GLuint) { fail!("Function glMultiTexCoordP4ui not initialised") }
	pub extern "C" fn glMultiTexCoordP4uiv(_: GLenum, _: GLenum, _: *GLuint) { fail!("Function glMultiTexCoordP4uiv not initialised") }
	pub extern "C" fn glNormalP3ui(_: GLenum, _: GLuint) { fail!("Function glNormalP3ui not initialised") }
	pub extern "C" fn glNormalP3uiv(_: GLenum, _: *GLuint) { fail!("Function glNormalP3uiv not initialised") }
	pub extern "C" fn glColorP3ui(_: GLenum, _: GLuint) { fail!("Function glColorP3ui not initialised") }
	pub extern "C" fn glColorP3uiv(_: GLenum, _: *GLuint) { fail!("Function glColorP3uiv not initialised") }
	pub extern "C" fn glColorP4ui(_: GLenum, _: GLuint) { fail!("Function glColorP4ui not initialised") }
	pub extern "C" fn glColorP4uiv(_: GLenum, _: *GLuint) { fail!("Function glColorP4uiv not initialised") }
	pub extern "C" fn glSecondaryColorP3ui(_: GLenum, _: GLuint) { fail!("Function glSecondaryColorP3ui not initialised") }
	pub extern "C" fn glSecondaryColorP3uiv(_: GLenum, _: *GLuint) { fail!("Function glSecondaryColorP3uiv not initialised") }
	pub extern "C" fn glVertexAttribP1ui(_: GLuint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexAttribP1ui not initialised") }
	pub extern "C" fn glVertexAttribP1uiv(_: GLuint, _: GLenum, _: GLboolean, _: *GLuint) { fail!("Function glVertexAttribP1uiv not initialised") }
	pub extern "C" fn glVertexAttribP2ui(_: GLuint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexAttribP2ui not initialised") }
	pub extern "C" fn glVertexAttribP2uiv(_: GLuint, _: GLenum, _: GLboolean, _: *GLuint) { fail!("Function glVertexAttribP2uiv not initialised") }
	pub extern "C" fn glVertexAttribP3ui(_: GLuint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexAttribP3ui not initialised") }
	pub extern "C" fn glVertexAttribP3uiv(_: GLuint, _: GLenum, _: GLboolean, _: *GLuint) { fail!("Function glVertexAttribP3uiv not initialised") }
	pub extern "C" fn glVertexAttribP4ui(_: GLuint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexAttribP4ui not initialised") }
	pub extern "C" fn glVertexAttribP4uiv(_: GLuint, _: GLenum, _: GLboolean, _: *GLuint) { fail!("Function glVertexAttribP4uiv not initialised") }
	
	// Core Extension: ARB_blend_func_extended
	pub extern "C" fn glBindFragDataLocationIndexed(_: GLuint, _: GLuint, _: GLuint, _: *GLchar) { fail!("Function glBindFragDataLocationIndexed not initialised") }
	pub extern "C" fn glGetFragDataIndex(_: GLuint, _: *GLchar) -> GLint { fail!("Function glGetFragDataIndex not initialised") }
	
	// Core Extension: ARB_sampler_objects
	pub extern "C" fn glGenSamplers(_: GLsizei, _: *GLuint) { fail!("Function glGenSamplers not initialised") }
	pub extern "C" fn glDeleteSamplers(_: GLsizei, _: *GLuint) { fail!("Function glDeleteSamplers not initialised") }
	pub extern "C" fn glIsSampler(_: GLuint) -> GLboolean { fail!("Function glIsSampler not initialised") }
	pub extern "C" fn glBindSampler(_: GLuint, _: GLuint) { fail!("Function glBindSampler not initialised") }
	pub extern "C" fn glSamplerParameteri(_: GLuint, _: GLenum, _: GLint) { fail!("Function glSamplerParameteri not initialised") }
	pub extern "C" fn glSamplerParameteriv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glSamplerParameteriv not initialised") }
	pub extern "C" fn glSamplerParameterf(_: GLuint, _: GLenum, _: GLfloat) { fail!("Function glSamplerParameterf not initialised") }
	pub extern "C" fn glSamplerParameterfv(_: GLuint, _: GLenum, _: *GLfloat) { fail!("Function glSamplerParameterfv not initialised") }
	pub extern "C" fn glSamplerParameterIiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glSamplerParameterIiv not initialised") }
	pub extern "C" fn glSamplerParameterIuiv(_: GLuint, _: GLenum, _: *GLuint) { fail!("Function glSamplerParameterIuiv not initialised") }
	pub extern "C" fn glGetSamplerParameteriv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetSamplerParameteriv not initialised") }
	pub extern "C" fn glGetSamplerParameterIiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetSamplerParameterIiv not initialised") }
	pub extern "C" fn glGetSamplerParameterfv(_: GLuint, _: GLenum, _: *GLfloat) { fail!("Function glGetSamplerParameterfv not initialised") }
	pub extern "C" fn glGetSamplerParameterIuiv(_: GLuint, _: GLenum, _: *GLuint) { fail!("Function glGetSamplerParameterIuiv not initialised") }
	
	// Version: 4.0
	pub extern "C" fn glMinSampleShading(_: GLfloat) { fail!("Function glMinSampleShading not initialised") }
	pub extern "C" fn glBlendEquationi(_: GLuint, _: GLenum) { fail!("Function glBlendEquationi not initialised") }
	pub extern "C" fn glBlendEquationSeparatei(_: GLuint, _: GLenum, _: GLenum) { fail!("Function glBlendEquationSeparatei not initialised") }
	pub extern "C" fn glBlendFunci(_: GLuint, _: GLenum, _: GLenum) { fail!("Function glBlendFunci not initialised") }
	pub extern "C" fn glBlendFuncSeparatei(_: GLuint, _: GLenum, _: GLenum, _: GLenum, _: GLenum) { fail!("Function glBlendFuncSeparatei not initialised") }
	
	// Core Extension: ARB_draw_indirect
	pub extern "C" fn glDrawArraysIndirect(_: GLenum, _: *GLvoid) { fail!("Function glDrawArraysIndirect not initialised") }
	pub extern "C" fn glDrawElementsIndirect(_: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glDrawElementsIndirect not initialised") }
	
	// Core Extension: ARB_gpu_shader_fp64
	pub extern "C" fn glUniform1d(_: GLint, _: GLdouble) { fail!("Function glUniform1d not initialised") }
	pub extern "C" fn glUniform2d(_: GLint, _: GLdouble, _: GLdouble) { fail!("Function glUniform2d not initialised") }
	pub extern "C" fn glUniform3d(_: GLint, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glUniform3d not initialised") }
	pub extern "C" fn glUniform4d(_: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glUniform4d not initialised") }
	pub extern "C" fn glUniform1dv(_: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glUniform1dv not initialised") }
	pub extern "C" fn glUniform2dv(_: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glUniform2dv not initialised") }
	pub extern "C" fn glUniform3dv(_: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glUniform3dv not initialised") }
	pub extern "C" fn glUniform4dv(_: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glUniform4dv not initialised") }
	pub extern "C" fn glUniformMatrix2dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix2dv not initialised") }
	pub extern "C" fn glUniformMatrix3dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix3dv not initialised") }
	pub extern "C" fn glUniformMatrix4dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix4dv not initialised") }
	pub extern "C" fn glUniformMatrix2x3dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix2x3dv not initialised") }
	pub extern "C" fn glUniformMatrix2x4dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix2x4dv not initialised") }
	pub extern "C" fn glUniformMatrix3x2dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix3x2dv not initialised") }
	pub extern "C" fn glUniformMatrix3x4dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix3x4dv not initialised") }
	pub extern "C" fn glUniformMatrix4x2dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix4x2dv not initialised") }
	pub extern "C" fn glUniformMatrix4x3dv(_: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glUniformMatrix4x3dv not initialised") }
	pub extern "C" fn glGetUniformdv(_: GLuint, _: GLint, _: *GLdouble) { fail!("Function glGetUniformdv not initialised") }
	
	// Core Extension: ARB_shader_subroutine
	pub extern "C" fn glGetSubroutineUniformLocation(_: GLuint, _: GLenum, _: *GLchar) -> GLint { fail!("Function glGetSubroutineUniformLocation not initialised") }
	pub extern "C" fn glGetSubroutineIndex(_: GLuint, _: GLenum, _: *GLchar) -> GLuint { fail!("Function glGetSubroutineIndex not initialised") }
	pub extern "C" fn glGetActiveSubroutineUniformiv(_: GLuint, _: GLenum, _: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetActiveSubroutineUniformiv not initialised") }
	pub extern "C" fn glGetActiveSubroutineUniformName(_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetActiveSubroutineUniformName not initialised") }
	pub extern "C" fn glGetActiveSubroutineName(_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetActiveSubroutineName not initialised") }
	pub extern "C" fn glUniformSubroutinesuiv(_: GLenum, _: GLsizei, _: *GLuint) { fail!("Function glUniformSubroutinesuiv not initialised") }
	pub extern "C" fn glGetUniformSubroutineuiv(_: GLenum, _: GLint, _: *GLuint) { fail!("Function glGetUniformSubroutineuiv not initialised") }
	pub extern "C" fn glGetProgramStageiv(_: GLuint, _: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetProgramStageiv not initialised") }
	
	// Core Extension: ARB_tessellation_shader
	pub extern "C" fn glPatchParameteri(_: GLenum, _: GLint) { fail!("Function glPatchParameteri not initialised") }
	pub extern "C" fn glPatchParameterfv(_: GLenum, _: *GLfloat) { fail!("Function glPatchParameterfv not initialised") }
	
	// Core Extension: ARB_transform_feedback2
	pub extern "C" fn glBindTransformFeedback(_: GLenum, _: GLuint) { fail!("Function glBindTransformFeedback not initialised") }
	pub extern "C" fn glDeleteTransformFeedbacks(_: GLsizei, _: *GLuint) { fail!("Function glDeleteTransformFeedbacks not initialised") }
	pub extern "C" fn glGenTransformFeedbacks(_: GLsizei, _: *GLuint) { fail!("Function glGenTransformFeedbacks not initialised") }
	pub extern "C" fn glIsTransformFeedback(_: GLuint) -> GLboolean { fail!("Function glIsTransformFeedback not initialised") }
	pub extern "C" fn glPauseTransformFeedback() { fail!("Function glPauseTransformFeedback not initialised") }
	pub extern "C" fn glResumeTransformFeedback() { fail!("Function glResumeTransformFeedback not initialised") }
	pub extern "C" fn glDrawTransformFeedback(_: GLenum, _: GLuint) { fail!("Function glDrawTransformFeedback not initialised") }
	
	// Core Extension: ARB_transform_feedback3
	pub extern "C" fn glDrawTransformFeedbackStream(_: GLenum, _: GLuint, _: GLuint) { fail!("Function glDrawTransformFeedbackStream not initialised") }
	pub extern "C" fn glBeginQueryIndexed(_: GLenum, _: GLuint, _: GLuint) { fail!("Function glBeginQueryIndexed not initialised") }
	pub extern "C" fn glEndQueryIndexed(_: GLenum, _: GLuint) { fail!("Function glEndQueryIndexed not initialised") }
	pub extern "C" fn glGetQueryIndexediv(_: GLenum, _: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetQueryIndexediv not initialised") }
	
	// Core Extension: ARB_ES2_compatibility
	pub extern "C" fn glReleaseShaderCompiler() { fail!("Function glReleaseShaderCompiler not initialised") }
	pub extern "C" fn glShaderBinary(_: GLsizei, _: *GLuint, _: GLenum, _: *GLvoid, _: GLsizei) { fail!("Function glShaderBinary not initialised") }
	pub extern "C" fn glGetShaderPrecisionFormat(_: GLenum, _: GLenum, _: *GLint, _: *GLint) { fail!("Function glGetShaderPrecisionFormat not initialised") }
	pub extern "C" fn glDepthRangef(_: GLfloat, _: GLfloat) { fail!("Function glDepthRangef not initialised") }
	pub extern "C" fn glClearDepthf(_: GLfloat) { fail!("Function glClearDepthf not initialised") }
	
	// Core Extension: ARB_get_program_binary
	pub extern "C" fn glGetProgramBinary(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLenum, _: *GLvoid) { fail!("Function glGetProgramBinary not initialised") }
	pub extern "C" fn glProgramBinary(_: GLuint, _: GLenum, _: *GLvoid, _: GLsizei) { fail!("Function glProgramBinary not initialised") }
	pub extern "C" fn glProgramParameteri(_: GLuint, _: GLenum, _: GLint) { fail!("Function glProgramParameteri not initialised") }
	
	// Core Extension: ARB_separate_shader_objects
	pub extern "C" fn glUseProgramStages(_: GLuint, _: GLbitfield, _: GLuint) { fail!("Function glUseProgramStages not initialised") }
	pub extern "C" fn glActiveShaderProgram(_: GLuint, _: GLuint) { fail!("Function glActiveShaderProgram not initialised") }
	pub extern "C" fn glCreateShaderProgramv(_: GLenum, _: GLsizei, _: **GLchar) -> GLuint { fail!("Function glCreateShaderProgramv not initialised") }
	pub extern "C" fn glBindProgramPipeline(_: GLuint) { fail!("Function glBindProgramPipeline not initialised") }
	pub extern "C" fn glDeleteProgramPipelines(_: GLsizei, _: *GLuint) { fail!("Function glDeleteProgramPipelines not initialised") }
	pub extern "C" fn glGenProgramPipelines(_: GLsizei, _: *GLuint) { fail!("Function glGenProgramPipelines not initialised") }
	pub extern "C" fn glIsProgramPipeline(_: GLuint) -> GLboolean { fail!("Function glIsProgramPipeline not initialised") }
	pub extern "C" fn glGetProgramPipelineiv(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetProgramPipelineiv not initialised") }
	pub extern "C" fn glProgramUniform1i(_: GLuint, _: GLint, _: GLint) { fail!("Function glProgramUniform1i not initialised") }
	pub extern "C" fn glProgramUniform1iv(_: GLuint, _: GLint, _: GLsizei, _: *GLint) { fail!("Function glProgramUniform1iv not initialised") }
	pub extern "C" fn glProgramUniform1f(_: GLuint, _: GLint, _: GLfloat) { fail!("Function glProgramUniform1f not initialised") }
	pub extern "C" fn glProgramUniform1fv(_: GLuint, _: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glProgramUniform1fv not initialised") }
	pub extern "C" fn glProgramUniform1d(_: GLuint, _: GLint, _: GLdouble) { fail!("Function glProgramUniform1d not initialised") }
	pub extern "C" fn glProgramUniform1dv(_: GLuint, _: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glProgramUniform1dv not initialised") }
	pub extern "C" fn glProgramUniform1ui(_: GLuint, _: GLint, _: GLuint) { fail!("Function glProgramUniform1ui not initialised") }
	pub extern "C" fn glProgramUniform1uiv(_: GLuint, _: GLint, _: GLsizei, _: *GLuint) { fail!("Function glProgramUniform1uiv not initialised") }
	pub extern "C" fn glProgramUniform2i(_: GLuint, _: GLint, _: GLint, _: GLint) { fail!("Function glProgramUniform2i not initialised") }
	pub extern "C" fn glProgramUniform2iv(_: GLuint, _: GLint, _: GLsizei, _: *GLint) { fail!("Function glProgramUniform2iv not initialised") }
	pub extern "C" fn glProgramUniform2f(_: GLuint, _: GLint, _: GLfloat, _: GLfloat) { fail!("Function glProgramUniform2f not initialised") }
	pub extern "C" fn glProgramUniform2fv(_: GLuint, _: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glProgramUniform2fv not initialised") }
	pub extern "C" fn glProgramUniform2d(_: GLuint, _: GLint, _: GLdouble, _: GLdouble) { fail!("Function glProgramUniform2d not initialised") }
	pub extern "C" fn glProgramUniform2dv(_: GLuint, _: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glProgramUniform2dv not initialised") }
	pub extern "C" fn glProgramUniform2ui(_: GLuint, _: GLint, _: GLuint, _: GLuint) { fail!("Function glProgramUniform2ui not initialised") }
	pub extern "C" fn glProgramUniform2uiv(_: GLuint, _: GLint, _: GLsizei, _: *GLuint) { fail!("Function glProgramUniform2uiv not initialised") }
	pub extern "C" fn glProgramUniform3i(_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) { fail!("Function glProgramUniform3i not initialised") }
	pub extern "C" fn glProgramUniform3iv(_: GLuint, _: GLint, _: GLsizei, _: *GLint) { fail!("Function glProgramUniform3iv not initialised") }
	pub extern "C" fn glProgramUniform3f(_: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glProgramUniform3f not initialised") }
	pub extern "C" fn glProgramUniform3fv(_: GLuint, _: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glProgramUniform3fv not initialised") }
	pub extern "C" fn glProgramUniform3d(_: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glProgramUniform3d not initialised") }
	pub extern "C" fn glProgramUniform3dv(_: GLuint, _: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glProgramUniform3dv not initialised") }
	pub extern "C" fn glProgramUniform3ui(_: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glProgramUniform3ui not initialised") }
	pub extern "C" fn glProgramUniform3uiv(_: GLuint, _: GLint, _: GLsizei, _: *GLuint) { fail!("Function glProgramUniform3uiv not initialised") }
	pub extern "C" fn glProgramUniform4i(_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint) { fail!("Function glProgramUniform4i not initialised") }
	pub extern "C" fn glProgramUniform4iv(_: GLuint, _: GLint, _: GLsizei, _: *GLint) { fail!("Function glProgramUniform4iv not initialised") }
	pub extern "C" fn glProgramUniform4f(_: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glProgramUniform4f not initialised") }
	pub extern "C" fn glProgramUniform4fv(_: GLuint, _: GLint, _: GLsizei, _: *GLfloat) { fail!("Function glProgramUniform4fv not initialised") }
	pub extern "C" fn glProgramUniform4d(_: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glProgramUniform4d not initialised") }
	pub extern "C" fn glProgramUniform4dv(_: GLuint, _: GLint, _: GLsizei, _: *GLdouble) { fail!("Function glProgramUniform4dv not initialised") }
	pub extern "C" fn glProgramUniform4ui(_: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glProgramUniform4ui not initialised") }
	pub extern "C" fn glProgramUniform4uiv(_: GLuint, _: GLint, _: GLsizei, _: *GLuint) { fail!("Function glProgramUniform4uiv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix2fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix3fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix4fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix2dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix3dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix4dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2x3fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix2x3fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3x2fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix3x2fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2x4fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix2x4fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4x2fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix4x2fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3x4fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix3x4fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4x3fv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLfloat) { fail!("Function glProgramUniformMatrix4x3fv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2x3dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix2x3dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3x2dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix3x2dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix2x4dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix2x4dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4x2dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix4x2dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix3x4dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix3x4dv not initialised") }
	pub extern "C" fn glProgramUniformMatrix4x3dv(_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *GLdouble) { fail!("Function glProgramUniformMatrix4x3dv not initialised") }
	pub extern "C" fn glValidateProgramPipeline(_: GLuint) { fail!("Function glValidateProgramPipeline not initialised") }
	pub extern "C" fn glGetProgramPipelineInfoLog(_: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetProgramPipelineInfoLog not initialised") }
	
	// Core Extension: ARB_vertex_attrib_64bit
	pub extern "C" fn glVertexAttribL1d(_: GLuint, _: GLdouble) { fail!("Function glVertexAttribL1d not initialised") }
	pub extern "C" fn glVertexAttribL2d(_: GLuint, _: GLdouble, _: GLdouble) { fail!("Function glVertexAttribL2d not initialised") }
	pub extern "C" fn glVertexAttribL3d(_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glVertexAttribL3d not initialised") }
	pub extern "C" fn glVertexAttribL4d(_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) { fail!("Function glVertexAttribL4d not initialised") }
	pub extern "C" fn glVertexAttribL1dv(_: GLuint, _: *GLdouble) { fail!("Function glVertexAttribL1dv not initialised") }
	pub extern "C" fn glVertexAttribL2dv(_: GLuint, _: *GLdouble) { fail!("Function glVertexAttribL2dv not initialised") }
	pub extern "C" fn glVertexAttribL3dv(_: GLuint, _: *GLdouble) { fail!("Function glVertexAttribL3dv not initialised") }
	pub extern "C" fn glVertexAttribL4dv(_: GLuint, _: *GLdouble) { fail!("Function glVertexAttribL4dv not initialised") }
	pub extern "C" fn glVertexAttribLPointer(_: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *GLvoid) { fail!("Function glVertexAttribLPointer not initialised") }
	pub extern "C" fn glGetVertexAttribLdv(_: GLuint, _: GLenum, _: *GLdouble) { fail!("Function glGetVertexAttribLdv not initialised") }
	
	// Core Extension: ARB_viewport_array
	pub extern "C" fn glViewportArrayv(_: GLuint, _: GLsizei, _: *GLfloat) { fail!("Function glViewportArrayv not initialised") }
	pub extern "C" fn glViewportIndexedf(_: GLuint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) { fail!("Function glViewportIndexedf not initialised") }
	pub extern "C" fn glViewportIndexedfv(_: GLuint, _: *GLfloat) { fail!("Function glViewportIndexedfv not initialised") }
	pub extern "C" fn glScissorArrayv(_: GLuint, _: GLsizei, _: *GLint) { fail!("Function glScissorArrayv not initialised") }
	pub extern "C" fn glScissorIndexed(_: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glScissorIndexed not initialised") }
	pub extern "C" fn glScissorIndexedv(_: GLuint, _: *GLint) { fail!("Function glScissorIndexedv not initialised") }
	pub extern "C" fn glDepthRangeArrayv(_: GLuint, _: GLsizei, _: *GLdouble) { fail!("Function glDepthRangeArrayv not initialised") }
	pub extern "C" fn glDepthRangeIndexed(_: GLuint, _: GLdouble, _: GLdouble) { fail!("Function glDepthRangeIndexed not initialised") }
	pub extern "C" fn glGetFloati_v(_: GLenum, _: GLuint, _: *GLfloat) { fail!("Function glGetFloati_v not initialised") }
	pub extern "C" fn glGetDoublei_v(_: GLenum, _: GLuint, _: *GLdouble) { fail!("Function glGetDoublei_v not initialised") }
	
	// Core Extension: ARB_base_instance
	pub extern "C" fn glDrawArraysInstancedBaseInstance(_: GLenum, _: GLint, _: GLsizei, _: GLsizei, _: GLuint) { fail!("Function glDrawArraysInstancedBaseInstance not initialised") }
	pub extern "C" fn glDrawElementsInstancedBaseInstance(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid, _: GLsizei, _: GLuint) { fail!("Function glDrawElementsInstancedBaseInstance not initialised") }
	pub extern "C" fn glDrawElementsInstancedBaseVertexBaseInstance(_: GLenum, _: GLsizei, _: GLenum, _: *GLvoid, _: GLsizei, _: GLint, _: GLuint) { fail!("Function glDrawElementsInstancedBaseVertexBaseInstance not initialised") }
	
	// Core Extension: ARB_transform_feedback_instanced
	pub extern "C" fn glDrawTransformFeedbackInstanced(_: GLenum, _: GLuint, _: GLsizei) { fail!("Function glDrawTransformFeedbackInstanced not initialised") }
	pub extern "C" fn glDrawTransformFeedbackStreamInstanced(_: GLenum, _: GLuint, _: GLuint, _: GLsizei) { fail!("Function glDrawTransformFeedbackStreamInstanced not initialised") }
	
	// Core Extension: ARB_internalformat_query
	pub extern "C" fn glGetInternalformativ(_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *GLint) { fail!("Function glGetInternalformativ not initialised") }
	
	// Core Extension: ARB_shader_atomic_counters
	pub extern "C" fn glGetActiveAtomicCounterBufferiv(_: GLuint, _: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetActiveAtomicCounterBufferiv not initialised") }
	
	// Core Extension: ARB_shader_image_load_store
	pub extern "C" fn glBindImageTexture(_: GLuint, _: GLuint, _: GLint, _: GLboolean, _: GLint, _: GLenum, _: GLenum) { fail!("Function glBindImageTexture not initialised") }
	pub extern "C" fn glMemoryBarrier(_: GLbitfield) { fail!("Function glMemoryBarrier not initialised") }
	
	// Core Extension: ARB_texture_storage
	pub extern "C" fn glTexStorage1D(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei) { fail!("Function glTexStorage1D not initialised") }
	pub extern "C" fn glTexStorage2D(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) { fail!("Function glTexStorage2D not initialised") }
	pub extern "C" fn glTexStorage3D(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) { fail!("Function glTexStorage3D not initialised") }
	pub extern "C" fn glTextureStorage1DEXT(_: GLuint, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei) { fail!("Function glTextureStorage1DEXT not initialised") }
	pub extern "C" fn glTextureStorage2DEXT(_: GLuint, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) { fail!("Function glTextureStorage2DEXT not initialised") }
	pub extern "C" fn glTextureStorage3DEXT(_: GLuint, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) { fail!("Function glTextureStorage3DEXT not initialised") }
	
	// Core Extension: KHR_debug
	pub extern "C" fn glDebugMessageControl(_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *GLuint, _: GLboolean) { fail!("Function glDebugMessageControl not initialised") }
	pub extern "C" fn glDebugMessageInsert(_: GLenum, _: GLenum, _: GLuint, _: GLenum, _: GLsizei, _: *GLchar) { fail!("Function glDebugMessageInsert not initialised") }
	pub extern "C" fn glDebugMessageCallback(_: GLDEBUGPROC, _: *GLvoid) { fail!("Function glDebugMessageCallback not initialised") }
	pub extern "C" fn glGetDebugMessageLog(_: GLuint, _: GLsizei, _: *GLenum, _: *GLenum, _: *GLuint, _: *GLenum, _: *GLsizei, _: *GLchar) -> GLuint { fail!("Function glGetDebugMessageLog not initialised") }
	pub extern "C" fn glPushDebugGroup(_: GLenum, _: GLuint, _: GLsizei, _: *GLchar) { fail!("Function glPushDebugGroup not initialised") }
	pub extern "C" fn glPopDebugGroup() { fail!("Function glPopDebugGroup not initialised") }
	pub extern "C" fn glObjectLabel(_: GLenum, _: GLuint, _: GLsizei, _: *GLchar) { fail!("Function glObjectLabel not initialised") }
	pub extern "C" fn glGetObjectLabel(_: GLenum, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetObjectLabel not initialised") }
	pub extern "C" fn glObjectPtrLabel(_: *GLvoid, _: GLsizei, _: *GLchar) { fail!("Function glObjectPtrLabel not initialised") }
	pub extern "C" fn glGetObjectPtrLabel(_: *GLvoid, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetObjectPtrLabel not initialised") }
	
	// Core Extension: ARB_clear_buffer_object
	pub extern "C" fn glClearBufferData(_: GLenum, _: GLenum, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glClearBufferData not initialised") }
	pub extern "C" fn glClearBufferSubData(_: GLenum, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glClearBufferSubData not initialised") }
	pub extern "C" fn glClearNamedBufferDataEXT(_: GLuint, _: GLenum, _: GLenum, _: GLenum, _: *GLvoid) { fail!("Function glClearNamedBufferDataEXT not initialised") }
	pub extern "C" fn glClearNamedBufferSubDataEXT(_: GLuint, _: GLenum, _: GLenum, _: GLenum, _: GLsizeiptr, _: GLsizeiptr, _: *GLvoid) { fail!("Function glClearNamedBufferSubDataEXT not initialised") }
	
	// Core Extension: ARB_compute_shader
	pub extern "C" fn glDispatchCompute(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glDispatchCompute not initialised") }
	pub extern "C" fn glDispatchComputeIndirect(_: GLintptr) { fail!("Function glDispatchComputeIndirect not initialised") }
	
	// Core Extension: ARB_copy_image
	pub extern "C" fn glCopyImageSubData(_: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) { fail!("Function glCopyImageSubData not initialised") }
	
	// Core Extension: ARB_framebuffer_no_attachments
	pub extern "C" fn glFramebufferParameteri(_: GLenum, _: GLenum, _: GLint) { fail!("Function glFramebufferParameteri not initialised") }
	pub extern "C" fn glGetFramebufferParameteriv(_: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetFramebufferParameteriv not initialised") }
	pub extern "C" fn glNamedFramebufferParameteriEXT(_: GLuint, _: GLenum, _: GLint) { fail!("Function glNamedFramebufferParameteriEXT not initialised") }
	pub extern "C" fn glGetNamedFramebufferParameterivEXT(_: GLuint, _: GLenum, _: *GLint) { fail!("Function glGetNamedFramebufferParameterivEXT not initialised") }
	
	// Core Extension: ARB_internalformat_query2
	pub extern "C" fn glGetInternalformati64v(_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *GLint64) { fail!("Function glGetInternalformati64v not initialised") }
	
	// Core Extension: ARB_invalidate_subdata
	pub extern "C" fn glInvalidateTexSubImage(_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) { fail!("Function glInvalidateTexSubImage not initialised") }
	pub extern "C" fn glInvalidateTexImage(_: GLuint, _: GLint) { fail!("Function glInvalidateTexImage not initialised") }
	pub extern "C" fn glInvalidateBufferSubData(_: GLuint, _: GLintptr, _: GLsizeiptr) { fail!("Function glInvalidateBufferSubData not initialised") }
	pub extern "C" fn glInvalidateBufferData(_: GLuint) { fail!("Function glInvalidateBufferData not initialised") }
	pub extern "C" fn glInvalidateFramebuffer(_: GLenum, _: GLsizei, _: *GLenum) { fail!("Function glInvalidateFramebuffer not initialised") }
	pub extern "C" fn glInvalidateSubFramebuffer(_: GLenum, _: GLsizei, _: *GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei) { fail!("Function glInvalidateSubFramebuffer not initialised") }
	
	// Core Extension: ARB_multi_draw_indirect
	pub extern "C" fn glMultiDrawArraysIndirect(_: GLenum, _: *GLvoid, _: GLsizei, _: GLsizei) { fail!("Function glMultiDrawArraysIndirect not initialised") }
	pub extern "C" fn glMultiDrawElementsIndirect(_: GLenum, _: GLenum, _: *GLvoid, _: GLsizei, _: GLsizei) { fail!("Function glMultiDrawElementsIndirect not initialised") }
	
	// Core Extension: ARB_program_interface_query
	pub extern "C" fn glGetProgramInterfaceiv(_: GLuint, _: GLenum, _: GLenum, _: *GLint) { fail!("Function glGetProgramInterfaceiv not initialised") }
	pub extern "C" fn glGetProgramResourceIndex(_: GLuint, _: GLenum, _: *GLchar) -> GLuint { fail!("Function glGetProgramResourceIndex not initialised") }
	pub extern "C" fn glGetProgramResourceName(_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *GLsizei, _: *GLchar) { fail!("Function glGetProgramResourceName not initialised") }
	pub extern "C" fn glGetProgramResourceiv(_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *GLenum, _: GLsizei, _: *GLsizei, _: *GLint) { fail!("Function glGetProgramResourceiv not initialised") }
	pub extern "C" fn glGetProgramResourceLocation(_: GLuint, _: GLenum, _: *GLchar) -> GLint { fail!("Function glGetProgramResourceLocation not initialised") }
	pub extern "C" fn glGetProgramResourceLocationIndex(_: GLuint, _: GLenum, _: *GLchar) -> GLint { fail!("Function glGetProgramResourceLocationIndex not initialised") }
	
	// Core Extension: ARB_shader_storage_buffer_object
	pub extern "C" fn glShaderStorageBlockBinding(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glShaderStorageBlockBinding not initialised") }
	
	// Core Extension: ARB_texture_buffer_range
	pub extern "C" fn glTexBufferRange(_: GLenum, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) { fail!("Function glTexBufferRange not initialised") }
	pub extern "C" fn glTextureBufferRangeEXT(_: GLuint, _: GLenum, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) { fail!("Function glTextureBufferRangeEXT not initialised") }
	
	// Core Extension: ARB_texture_storage_multisample
	pub extern "C" fn glTexStorage2DMultisample(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTexStorage2DMultisample not initialised") }
	pub extern "C" fn glTexStorage3DMultisample(_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTexStorage3DMultisample not initialised") }
	pub extern "C" fn glTextureStorage2DMultisampleEXT(_: GLuint, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTextureStorage2DMultisampleEXT not initialised") }
	pub extern "C" fn glTextureStorage3DMultisampleEXT(_: GLuint, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) { fail!("Function glTextureStorage3DMultisampleEXT not initialised") }
	
	// Core Extension: ARB_texture_view
	pub extern "C" fn glTextureView(_: GLuint, _: GLenum, _: GLuint, _: GLenum, _: GLuint, _: GLuint, _: GLuint, _: GLuint) { fail!("Function glTextureView not initialised") }
	
	// Core Extension: ARB_vertex_attrib_binding
	pub extern "C" fn glBindVertexBuffer(_: GLuint, _: GLuint, _: GLintptr, _: GLsizei) { fail!("Function glBindVertexBuffer not initialised") }
	pub extern "C" fn glVertexAttribFormat(_: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexAttribFormat not initialised") }
	pub extern "C" fn glVertexAttribIFormat(_: GLuint, _: GLint, _: GLenum, _: GLuint) { fail!("Function glVertexAttribIFormat not initialised") }
	pub extern "C" fn glVertexAttribLFormat(_: GLuint, _: GLint, _: GLenum, _: GLuint) { fail!("Function glVertexAttribLFormat not initialised") }
	pub extern "C" fn glVertexAttribBinding(_: GLuint, _: GLuint) { fail!("Function glVertexAttribBinding not initialised") }
	pub extern "C" fn glVertexBindingDivisor(_: GLuint, _: GLuint) { fail!("Function glVertexBindingDivisor not initialised") }
	pub extern "C" fn glVertexArrayBindVertexBufferEXT(_: GLuint, _: GLuint, _: GLuint, _: GLintptr, _: GLsizei) { fail!("Function glVertexArrayBindVertexBufferEXT not initialised") }
	pub extern "C" fn glVertexArrayVertexAttribFormatEXT(_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) { fail!("Function glVertexArrayVertexAttribFormatEXT not initialised") }
	pub extern "C" fn glVertexArrayVertexAttribIFormatEXT(_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) { fail!("Function glVertexArrayVertexAttribIFormatEXT not initialised") }
	pub extern "C" fn glVertexArrayVertexAttribLFormatEXT(_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) { fail!("Function glVertexArrayVertexAttribLFormatEXT not initialised") }
	pub extern "C" fn glVertexArrayVertexAttribBindingEXT(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glVertexArrayVertexAttribBindingEXT not initialised") }
	pub extern "C" fn glVertexArrayVertexBindingDivisorEXT(_: GLuint, _: GLuint, _: GLuint) { fail!("Function glVertexArrayVertexBindingDivisorEXT not initialised") }
	
}

///
/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
///
/// ~~~
/// let gl = gl::load_with(glfw::get_proc_address);
/// ~~~
///
pub fn load_with(loadfn: &fn(symbol: &str) -> *c_void) -> ~GL {
	~GL {
		// Version: 1.1
		CullFace: FPointer::load(loadfn("glCullFace"), failing::glCullFace),
		FrontFace: FPointer::load(loadfn("glFrontFace"), failing::glFrontFace),
		Hint: FPointer::load(loadfn("glHint"), failing::glHint),
		LineWidth: FPointer::load(loadfn("glLineWidth"), failing::glLineWidth),
		PointSize: FPointer::load(loadfn("glPointSize"), failing::glPointSize),
		PolygonMode: FPointer::load(loadfn("glPolygonMode"), failing::glPolygonMode),
		Scissor: FPointer::load(loadfn("glScissor"), failing::glScissor),
		TexParameterf: FPointer::load(loadfn("glTexParameterf"), failing::glTexParameterf),
		TexParameterfv: FPointer::load(loadfn("glTexParameterfv"), failing::glTexParameterfv),
		TexParameteri: FPointer::load(loadfn("glTexParameteri"), failing::glTexParameteri),
		TexParameteriv: FPointer::load(loadfn("glTexParameteriv"), failing::glTexParameteriv),
		TexImage1D: FPointer::load(loadfn("glTexImage1D"), failing::glTexImage1D),
		TexImage2D: FPointer::load(loadfn("glTexImage2D"), failing::glTexImage2D),
		DrawBuffer: FPointer::load(loadfn("glDrawBuffer"), failing::glDrawBuffer),
		Clear: FPointer::load(loadfn("glClear"), failing::glClear),
		ClearColor: FPointer::load(loadfn("glClearColor"), failing::glClearColor),
		ClearStencil: FPointer::load(loadfn("glClearStencil"), failing::glClearStencil),
		ClearDepth: FPointer::load(loadfn("glClearDepth"), failing::glClearDepth),
		StencilMask: FPointer::load(loadfn("glStencilMask"), failing::glStencilMask),
		ColorMask: FPointer::load(loadfn("glColorMask"), failing::glColorMask),
		DepthMask: FPointer::load(loadfn("glDepthMask"), failing::glDepthMask),
		Disable: FPointer::load(loadfn("glDisable"), failing::glDisable),
		Enable: FPointer::load(loadfn("glEnable"), failing::glEnable),
		Finish: FPointer::load(loadfn("glFinish"), failing::glFinish),
		Flush: FPointer::load(loadfn("glFlush"), failing::glFlush),
		BlendFunc: FPointer::load(loadfn("glBlendFunc"), failing::glBlendFunc),
		LogicOp: FPointer::load(loadfn("glLogicOp"), failing::glLogicOp),
		StencilFunc: FPointer::load(loadfn("glStencilFunc"), failing::glStencilFunc),
		StencilOp: FPointer::load(loadfn("glStencilOp"), failing::glStencilOp),
		DepthFunc: FPointer::load(loadfn("glDepthFunc"), failing::glDepthFunc),
		PixelStoref: FPointer::load(loadfn("glPixelStoref"), failing::glPixelStoref),
		PixelStorei: FPointer::load(loadfn("glPixelStorei"), failing::glPixelStorei),
		ReadBuffer: FPointer::load(loadfn("glReadBuffer"), failing::glReadBuffer),
		ReadPixels: FPointer::load(loadfn("glReadPixels"), failing::glReadPixels),
		GetBooleanv: FPointer::load(loadfn("glGetBooleanv"), failing::glGetBooleanv),
		GetDoublev: FPointer::load(loadfn("glGetDoublev"), failing::glGetDoublev),
		GetError: FPointer::load(loadfn("glGetError"), failing::glGetError),
		GetFloatv: FPointer::load(loadfn("glGetFloatv"), failing::glGetFloatv),
		GetIntegerv: FPointer::load(loadfn("glGetIntegerv"), failing::glGetIntegerv),
		GetString: FPointer::load(loadfn("glGetString"), failing::glGetString),
		GetTexImage: FPointer::load(loadfn("glGetTexImage"), failing::glGetTexImage),
		GetTexParameterfv: FPointer::load(loadfn("glGetTexParameterfv"), failing::glGetTexParameterfv),
		GetTexParameteriv: FPointer::load(loadfn("glGetTexParameteriv"), failing::glGetTexParameteriv),
		GetTexLevelParameterfv: FPointer::load(loadfn("glGetTexLevelParameterfv"), failing::glGetTexLevelParameterfv),
		GetTexLevelParameteriv: FPointer::load(loadfn("glGetTexLevelParameteriv"), failing::glGetTexLevelParameteriv),
		IsEnabled: FPointer::load(loadfn("glIsEnabled"), failing::glIsEnabled),
		DepthRange: FPointer::load(loadfn("glDepthRange"), failing::glDepthRange),
		Viewport: FPointer::load(loadfn("glViewport"), failing::glViewport),
		DrawArrays: FPointer::load(loadfn("glDrawArrays"), failing::glDrawArrays),
		DrawElements: FPointer::load(loadfn("glDrawElements"), failing::glDrawElements),
		GetPointerv: FPointer::load(loadfn("glGetPointerv"), failing::glGetPointerv),
		PolygonOffset: FPointer::load(loadfn("glPolygonOffset"), failing::glPolygonOffset),
		CopyTexImage1D: FPointer::load(loadfn("glCopyTexImage1D"), failing::glCopyTexImage1D),
		CopyTexImage2D: FPointer::load(loadfn("glCopyTexImage2D"), failing::glCopyTexImage2D),
		CopyTexSubImage1D: FPointer::load(loadfn("glCopyTexSubImage1D"), failing::glCopyTexSubImage1D),
		CopyTexSubImage2D: FPointer::load(loadfn("glCopyTexSubImage2D"), failing::glCopyTexSubImage2D),
		TexSubImage1D: FPointer::load(loadfn("glTexSubImage1D"), failing::glTexSubImage1D),
		TexSubImage2D: FPointer::load(loadfn("glTexSubImage2D"), failing::glTexSubImage2D),
		BindTexture: FPointer::load(loadfn("glBindTexture"), failing::glBindTexture),
		DeleteTextures: FPointer::load(loadfn("glDeleteTextures"), failing::glDeleteTextures),
		GenTextures: FPointer::load(loadfn("glGenTextures"), failing::glGenTextures),
		IsTexture: FPointer::load(loadfn("glIsTexture"), failing::glIsTexture),
		Indexub: FPointer::load(loadfn("glIndexub"), failing::glIndexub),
		Indexubv: FPointer::load(loadfn("glIndexubv"), failing::glIndexubv),
		
		// Version: 1.2
		BlendColor: FPointer::load(loadfn("glBlendColor"), failing::glBlendColor),
		BlendEquation: FPointer::load(loadfn("glBlendEquation"), failing::glBlendEquation),
		DrawRangeElements: FPointer::load(loadfn("glDrawRangeElements"), failing::glDrawRangeElements),
		TexImage3D: FPointer::load(loadfn("glTexImage3D"), failing::glTexImage3D),
		TexSubImage3D: FPointer::load(loadfn("glTexSubImage3D"), failing::glTexSubImage3D),
		CopyTexSubImage3D: FPointer::load(loadfn("glCopyTexSubImage3D"), failing::glCopyTexSubImage3D),
		
		// Version: 1.3
		ActiveTexture: FPointer::load(loadfn("glActiveTexture"), failing::glActiveTexture),
		SampleCoverage: FPointer::load(loadfn("glSampleCoverage"), failing::glSampleCoverage),
		CompressedTexImage3D: FPointer::load(loadfn("glCompressedTexImage3D"), failing::glCompressedTexImage3D),
		CompressedTexImage2D: FPointer::load(loadfn("glCompressedTexImage2D"), failing::glCompressedTexImage2D),
		CompressedTexImage1D: FPointer::load(loadfn("glCompressedTexImage1D"), failing::glCompressedTexImage1D),
		CompressedTexSubImage3D: FPointer::load(loadfn("glCompressedTexSubImage3D"), failing::glCompressedTexSubImage3D),
		CompressedTexSubImage2D: FPointer::load(loadfn("glCompressedTexSubImage2D"), failing::glCompressedTexSubImage2D),
		CompressedTexSubImage1D: FPointer::load(loadfn("glCompressedTexSubImage1D"), failing::glCompressedTexSubImage1D),
		GetCompressedTexImage: FPointer::load(loadfn("glGetCompressedTexImage"), failing::glGetCompressedTexImage),
		
		// Version: 1.4
		BlendFuncSeparate: FPointer::load(loadfn("glBlendFuncSeparate"), failing::glBlendFuncSeparate),
		MultiDrawArrays: FPointer::load(loadfn("glMultiDrawArrays"), failing::glMultiDrawArrays),
		MultiDrawElements: FPointer::load(loadfn("glMultiDrawElements"), failing::glMultiDrawElements),
		PointParameterf: FPointer::load(loadfn("glPointParameterf"), failing::glPointParameterf),
		PointParameterfv: FPointer::load(loadfn("glPointParameterfv"), failing::glPointParameterfv),
		PointParameteri: FPointer::load(loadfn("glPointParameteri"), failing::glPointParameteri),
		PointParameteriv: FPointer::load(loadfn("glPointParameteriv"), failing::glPointParameteriv),
		
		// Version: 1.5
		GenQueries: FPointer::load(loadfn("glGenQueries"), failing::glGenQueries),
		DeleteQueries: FPointer::load(loadfn("glDeleteQueries"), failing::glDeleteQueries),
		IsQuery: FPointer::load(loadfn("glIsQuery"), failing::glIsQuery),
		BeginQuery: FPointer::load(loadfn("glBeginQuery"), failing::glBeginQuery),
		EndQuery: FPointer::load(loadfn("glEndQuery"), failing::glEndQuery),
		GetQueryiv: FPointer::load(loadfn("glGetQueryiv"), failing::glGetQueryiv),
		GetQueryObjectiv: FPointer::load(loadfn("glGetQueryObjectiv"), failing::glGetQueryObjectiv),
		GetQueryObjectuiv: FPointer::load(loadfn("glGetQueryObjectuiv"), failing::glGetQueryObjectuiv),
		BindBuffer: FPointer::load(loadfn("glBindBuffer"), failing::glBindBuffer),
		DeleteBuffers: FPointer::load(loadfn("glDeleteBuffers"), failing::glDeleteBuffers),
		GenBuffers: FPointer::load(loadfn("glGenBuffers"), failing::glGenBuffers),
		IsBuffer: FPointer::load(loadfn("glIsBuffer"), failing::glIsBuffer),
		BufferData: FPointer::load(loadfn("glBufferData"), failing::glBufferData),
		BufferSubData: FPointer::load(loadfn("glBufferSubData"), failing::glBufferSubData),
		GetBufferSubData: FPointer::load(loadfn("glGetBufferSubData"), failing::glGetBufferSubData),
		MapBuffer: FPointer::load(loadfn("glMapBuffer"), failing::glMapBuffer),
		UnmapBuffer: FPointer::load(loadfn("glUnmapBuffer"), failing::glUnmapBuffer),
		GetBufferParameteriv: FPointer::load(loadfn("glGetBufferParameteriv"), failing::glGetBufferParameteriv),
		GetBufferPointerv: FPointer::load(loadfn("glGetBufferPointerv"), failing::glGetBufferPointerv),
		
		// Version: 2.0
		BlendEquationSeparate: FPointer::load(loadfn("glBlendEquationSeparate"), failing::glBlendEquationSeparate),
		DrawBuffers: FPointer::load(loadfn("glDrawBuffers"), failing::glDrawBuffers),
		StencilOpSeparate: FPointer::load(loadfn("glStencilOpSeparate"), failing::glStencilOpSeparate),
		StencilFuncSeparate: FPointer::load(loadfn("glStencilFuncSeparate"), failing::glStencilFuncSeparate),
		StencilMaskSeparate: FPointer::load(loadfn("glStencilMaskSeparate"), failing::glStencilMaskSeparate),
		AttachShader: FPointer::load(loadfn("glAttachShader"), failing::glAttachShader),
		BindAttribLocation: FPointer::load(loadfn("glBindAttribLocation"), failing::glBindAttribLocation),
		CompileShader: FPointer::load(loadfn("glCompileShader"), failing::glCompileShader),
		CreateProgram: FPointer::load(loadfn("glCreateProgram"), failing::glCreateProgram),
		CreateShader: FPointer::load(loadfn("glCreateShader"), failing::glCreateShader),
		DeleteProgram: FPointer::load(loadfn("glDeleteProgram"), failing::glDeleteProgram),
		DeleteShader: FPointer::load(loadfn("glDeleteShader"), failing::glDeleteShader),
		DetachShader: FPointer::load(loadfn("glDetachShader"), failing::glDetachShader),
		DisableVertexAttribArray: FPointer::load(loadfn("glDisableVertexAttribArray"), failing::glDisableVertexAttribArray),
		EnableVertexAttribArray: FPointer::load(loadfn("glEnableVertexAttribArray"), failing::glEnableVertexAttribArray),
		GetActiveAttrib: FPointer::load(loadfn("glGetActiveAttrib"), failing::glGetActiveAttrib),
		GetActiveUniform: FPointer::load(loadfn("glGetActiveUniform"), failing::glGetActiveUniform),
		GetAttachedShaders: FPointer::load(loadfn("glGetAttachedShaders"), failing::glGetAttachedShaders),
		GetAttribLocation: FPointer::load(loadfn("glGetAttribLocation"), failing::glGetAttribLocation),
		GetProgramiv: FPointer::load(loadfn("glGetProgramiv"), failing::glGetProgramiv),
		GetProgramInfoLog: FPointer::load(loadfn("glGetProgramInfoLog"), failing::glGetProgramInfoLog),
		GetShaderiv: FPointer::load(loadfn("glGetShaderiv"), failing::glGetShaderiv),
		GetShaderInfoLog: FPointer::load(loadfn("glGetShaderInfoLog"), failing::glGetShaderInfoLog),
		GetShaderSource: FPointer::load(loadfn("glGetShaderSource"), failing::glGetShaderSource),
		GetUniformLocation: FPointer::load(loadfn("glGetUniformLocation"), failing::glGetUniformLocation),
		GetUniformfv: FPointer::load(loadfn("glGetUniformfv"), failing::glGetUniformfv),
		GetUniformiv: FPointer::load(loadfn("glGetUniformiv"), failing::glGetUniformiv),
		GetVertexAttribdv: FPointer::load(loadfn("glGetVertexAttribdv"), failing::glGetVertexAttribdv),
		GetVertexAttribfv: FPointer::load(loadfn("glGetVertexAttribfv"), failing::glGetVertexAttribfv),
		GetVertexAttribiv: FPointer::load(loadfn("glGetVertexAttribiv"), failing::glGetVertexAttribiv),
		GetVertexAttribPointerv: FPointer::load(loadfn("glGetVertexAttribPointerv"), failing::glGetVertexAttribPointerv),
		IsProgram: FPointer::load(loadfn("glIsProgram"), failing::glIsProgram),
		IsShader: FPointer::load(loadfn("glIsShader"), failing::glIsShader),
		LinkProgram: FPointer::load(loadfn("glLinkProgram"), failing::glLinkProgram),
		ShaderSource: FPointer::load(loadfn("glShaderSource"), failing::glShaderSource),
		UseProgram: FPointer::load(loadfn("glUseProgram"), failing::glUseProgram),
		Uniform1f: FPointer::load(loadfn("glUniform1f"), failing::glUniform1f),
		Uniform2f: FPointer::load(loadfn("glUniform2f"), failing::glUniform2f),
		Uniform3f: FPointer::load(loadfn("glUniform3f"), failing::glUniform3f),
		Uniform4f: FPointer::load(loadfn("glUniform4f"), failing::glUniform4f),
		Uniform1i: FPointer::load(loadfn("glUniform1i"), failing::glUniform1i),
		Uniform2i: FPointer::load(loadfn("glUniform2i"), failing::glUniform2i),
		Uniform3i: FPointer::load(loadfn("glUniform3i"), failing::glUniform3i),
		Uniform4i: FPointer::load(loadfn("glUniform4i"), failing::glUniform4i),
		Uniform1fv: FPointer::load(loadfn("glUniform1fv"), failing::glUniform1fv),
		Uniform2fv: FPointer::load(loadfn("glUniform2fv"), failing::glUniform2fv),
		Uniform3fv: FPointer::load(loadfn("glUniform3fv"), failing::glUniform3fv),
		Uniform4fv: FPointer::load(loadfn("glUniform4fv"), failing::glUniform4fv),
		Uniform1iv: FPointer::load(loadfn("glUniform1iv"), failing::glUniform1iv),
		Uniform2iv: FPointer::load(loadfn("glUniform2iv"), failing::glUniform2iv),
		Uniform3iv: FPointer::load(loadfn("glUniform3iv"), failing::glUniform3iv),
		Uniform4iv: FPointer::load(loadfn("glUniform4iv"), failing::glUniform4iv),
		UniformMatrix2fv: FPointer::load(loadfn("glUniformMatrix2fv"), failing::glUniformMatrix2fv),
		UniformMatrix3fv: FPointer::load(loadfn("glUniformMatrix3fv"), failing::glUniformMatrix3fv),
		UniformMatrix4fv: FPointer::load(loadfn("glUniformMatrix4fv"), failing::glUniformMatrix4fv),
		ValidateProgram: FPointer::load(loadfn("glValidateProgram"), failing::glValidateProgram),
		VertexAttribPointer: FPointer::load(loadfn("glVertexAttribPointer"), failing::glVertexAttribPointer),
		
		// Version: 2.1
		UniformMatrix2x3fv: FPointer::load(loadfn("glUniformMatrix2x3fv"), failing::glUniformMatrix2x3fv),
		UniformMatrix3x2fv: FPointer::load(loadfn("glUniformMatrix3x2fv"), failing::glUniformMatrix3x2fv),
		UniformMatrix2x4fv: FPointer::load(loadfn("glUniformMatrix2x4fv"), failing::glUniformMatrix2x4fv),
		UniformMatrix4x2fv: FPointer::load(loadfn("glUniformMatrix4x2fv"), failing::glUniformMatrix4x2fv),
		UniformMatrix3x4fv: FPointer::load(loadfn("glUniformMatrix3x4fv"), failing::glUniformMatrix3x4fv),
		UniformMatrix4x3fv: FPointer::load(loadfn("glUniformMatrix4x3fv"), failing::glUniformMatrix4x3fv),
		
		// Version: 3.0
		ColorMaski: FPointer::load(loadfn("glColorMaski"), failing::glColorMaski),
		GetBooleani_v: FPointer::load(loadfn("glGetBooleani_v"), failing::glGetBooleani_v),
		GetIntegeri_v: FPointer::load(loadfn("glGetIntegeri_v"), failing::glGetIntegeri_v),
		Enablei: FPointer::load(loadfn("glEnablei"), failing::glEnablei),
		Disablei: FPointer::load(loadfn("glDisablei"), failing::glDisablei),
		IsEnabledi: FPointer::load(loadfn("glIsEnabledi"), failing::glIsEnabledi),
		BeginTransformFeedback: FPointer::load(loadfn("glBeginTransformFeedback"), failing::glBeginTransformFeedback),
		EndTransformFeedback: FPointer::load(loadfn("glEndTransformFeedback"), failing::glEndTransformFeedback),
		BindBufferRange: FPointer::load(loadfn("glBindBufferRange"), failing::glBindBufferRange),
		BindBufferBase: FPointer::load(loadfn("glBindBufferBase"), failing::glBindBufferBase),
		TransformFeedbackVaryings: FPointer::load(loadfn("glTransformFeedbackVaryings"), failing::glTransformFeedbackVaryings),
		GetTransformFeedbackVarying: FPointer::load(loadfn("glGetTransformFeedbackVarying"), failing::glGetTransformFeedbackVarying),
		ClampColor: FPointer::load(loadfn("glClampColor"), failing::glClampColor),
		BeginConditionalRender: FPointer::load(loadfn("glBeginConditionalRender"), failing::glBeginConditionalRender),
		EndConditionalRender: FPointer::load(loadfn("glEndConditionalRender"), failing::glEndConditionalRender),
		VertexAttribIPointer: FPointer::load(loadfn("glVertexAttribIPointer"), failing::glVertexAttribIPointer),
		GetVertexAttribIiv: FPointer::load(loadfn("glGetVertexAttribIiv"), failing::glGetVertexAttribIiv),
		GetVertexAttribIuiv: FPointer::load(loadfn("glGetVertexAttribIuiv"), failing::glGetVertexAttribIuiv),
		VertexAttribI1i: FPointer::load(loadfn("glVertexAttribI1i"), failing::glVertexAttribI1i),
		VertexAttribI2i: FPointer::load(loadfn("glVertexAttribI2i"), failing::glVertexAttribI2i),
		VertexAttribI3i: FPointer::load(loadfn("glVertexAttribI3i"), failing::glVertexAttribI3i),
		VertexAttribI4i: FPointer::load(loadfn("glVertexAttribI4i"), failing::glVertexAttribI4i),
		VertexAttribI1ui: FPointer::load(loadfn("glVertexAttribI1ui"), failing::glVertexAttribI1ui),
		VertexAttribI2ui: FPointer::load(loadfn("glVertexAttribI2ui"), failing::glVertexAttribI2ui),
		VertexAttribI3ui: FPointer::load(loadfn("glVertexAttribI3ui"), failing::glVertexAttribI3ui),
		VertexAttribI4ui: FPointer::load(loadfn("glVertexAttribI4ui"), failing::glVertexAttribI4ui),
		VertexAttribI1iv: FPointer::load(loadfn("glVertexAttribI1iv"), failing::glVertexAttribI1iv),
		VertexAttribI2iv: FPointer::load(loadfn("glVertexAttribI2iv"), failing::glVertexAttribI2iv),
		VertexAttribI3iv: FPointer::load(loadfn("glVertexAttribI3iv"), failing::glVertexAttribI3iv),
		VertexAttribI4iv: FPointer::load(loadfn("glVertexAttribI4iv"), failing::glVertexAttribI4iv),
		VertexAttribI1uiv: FPointer::load(loadfn("glVertexAttribI1uiv"), failing::glVertexAttribI1uiv),
		VertexAttribI2uiv: FPointer::load(loadfn("glVertexAttribI2uiv"), failing::glVertexAttribI2uiv),
		VertexAttribI3uiv: FPointer::load(loadfn("glVertexAttribI3uiv"), failing::glVertexAttribI3uiv),
		VertexAttribI4uiv: FPointer::load(loadfn("glVertexAttribI4uiv"), failing::glVertexAttribI4uiv),
		VertexAttribI4bv: FPointer::load(loadfn("glVertexAttribI4bv"), failing::glVertexAttribI4bv),
		VertexAttribI4sv: FPointer::load(loadfn("glVertexAttribI4sv"), failing::glVertexAttribI4sv),
		VertexAttribI4ubv: FPointer::load(loadfn("glVertexAttribI4ubv"), failing::glVertexAttribI4ubv),
		VertexAttribI4usv: FPointer::load(loadfn("glVertexAttribI4usv"), failing::glVertexAttribI4usv),
		GetUniformuiv: FPointer::load(loadfn("glGetUniformuiv"), failing::glGetUniformuiv),
		BindFragDataLocation: FPointer::load(loadfn("glBindFragDataLocation"), failing::glBindFragDataLocation),
		GetFragDataLocation: FPointer::load(loadfn("glGetFragDataLocation"), failing::glGetFragDataLocation),
		Uniform1ui: FPointer::load(loadfn("glUniform1ui"), failing::glUniform1ui),
		Uniform2ui: FPointer::load(loadfn("glUniform2ui"), failing::glUniform2ui),
		Uniform3ui: FPointer::load(loadfn("glUniform3ui"), failing::glUniform3ui),
		Uniform4ui: FPointer::load(loadfn("glUniform4ui"), failing::glUniform4ui),
		Uniform1uiv: FPointer::load(loadfn("glUniform1uiv"), failing::glUniform1uiv),
		Uniform2uiv: FPointer::load(loadfn("glUniform2uiv"), failing::glUniform2uiv),
		Uniform3uiv: FPointer::load(loadfn("glUniform3uiv"), failing::glUniform3uiv),
		Uniform4uiv: FPointer::load(loadfn("glUniform4uiv"), failing::glUniform4uiv),
		TexParameterIiv: FPointer::load(loadfn("glTexParameterIiv"), failing::glTexParameterIiv),
		TexParameterIuiv: FPointer::load(loadfn("glTexParameterIuiv"), failing::glTexParameterIuiv),
		GetTexParameterIiv: FPointer::load(loadfn("glGetTexParameterIiv"), failing::glGetTexParameterIiv),
		GetTexParameterIuiv: FPointer::load(loadfn("glGetTexParameterIuiv"), failing::glGetTexParameterIuiv),
		ClearBufferiv: FPointer::load(loadfn("glClearBufferiv"), failing::glClearBufferiv),
		ClearBufferuiv: FPointer::load(loadfn("glClearBufferuiv"), failing::glClearBufferuiv),
		ClearBufferfv: FPointer::load(loadfn("glClearBufferfv"), failing::glClearBufferfv),
		ClearBufferfi: FPointer::load(loadfn("glClearBufferfi"), failing::glClearBufferfi),
		GetStringi: FPointer::load(loadfn("glGetStringi"), failing::glGetStringi),
		
		// Core Extension: ARB_vertex_array_object
		BindVertexArray: FPointer::load(loadfn("glBindVertexArray"), failing::glBindVertexArray),
		DeleteVertexArrays: FPointer::load(loadfn("glDeleteVertexArrays"), failing::glDeleteVertexArrays),
		GenVertexArrays: FPointer::load(loadfn("glGenVertexArrays"), failing::glGenVertexArrays),
		IsVertexArray: FPointer::load(loadfn("glIsVertexArray"), failing::glIsVertexArray),
		
		// Core Extension: ARB_map_buffer_range
		MapBufferRange: FPointer::load(loadfn("glMapBufferRange"), failing::glMapBufferRange),
		FlushMappedBufferRange: FPointer::load(loadfn("glFlushMappedBufferRange"), failing::glFlushMappedBufferRange),
		
		// Core Extension: ARB_framebuffer_object
		IsRenderbuffer: FPointer::load(loadfn("glIsRenderbuffer"), failing::glIsRenderbuffer),
		BindRenderbuffer: FPointer::load(loadfn("glBindRenderbuffer"), failing::glBindRenderbuffer),
		DeleteRenderbuffers: FPointer::load(loadfn("glDeleteRenderbuffers"), failing::glDeleteRenderbuffers),
		GenRenderbuffers: FPointer::load(loadfn("glGenRenderbuffers"), failing::glGenRenderbuffers),
		RenderbufferStorage: FPointer::load(loadfn("glRenderbufferStorage"), failing::glRenderbufferStorage),
		GetRenderbufferParameteriv: FPointer::load(loadfn("glGetRenderbufferParameteriv"), failing::glGetRenderbufferParameteriv),
		IsFramebuffer: FPointer::load(loadfn("glIsFramebuffer"), failing::glIsFramebuffer),
		BindFramebuffer: FPointer::load(loadfn("glBindFramebuffer"), failing::glBindFramebuffer),
		DeleteFramebuffers: FPointer::load(loadfn("glDeleteFramebuffers"), failing::glDeleteFramebuffers),
		GenFramebuffers: FPointer::load(loadfn("glGenFramebuffers"), failing::glGenFramebuffers),
		CheckFramebufferStatus: FPointer::load(loadfn("glCheckFramebufferStatus"), failing::glCheckFramebufferStatus),
		FramebufferTexture1D: FPointer::load(loadfn("glFramebufferTexture1D"), failing::glFramebufferTexture1D),
		FramebufferTexture2D: FPointer::load(loadfn("glFramebufferTexture2D"), failing::glFramebufferTexture2D),
		FramebufferTexture3D: FPointer::load(loadfn("glFramebufferTexture3D"), failing::glFramebufferTexture3D),
		FramebufferRenderbuffer: FPointer::load(loadfn("glFramebufferRenderbuffer"), failing::glFramebufferRenderbuffer),
		GetFramebufferAttachmentParameteriv: FPointer::load(loadfn("glGetFramebufferAttachmentParameteriv"), failing::glGetFramebufferAttachmentParameteriv),
		GenerateMipmap: FPointer::load(loadfn("glGenerateMipmap"), failing::glGenerateMipmap),
		BlitFramebuffer: FPointer::load(loadfn("glBlitFramebuffer"), failing::glBlitFramebuffer),
		RenderbufferStorageMultisample: FPointer::load(loadfn("glRenderbufferStorageMultisample"), failing::glRenderbufferStorageMultisample),
		FramebufferTextureLayer: FPointer::load(loadfn("glFramebufferTextureLayer"), failing::glFramebufferTextureLayer),
		
		// Version: 3.1
		DrawArraysInstanced: FPointer::load(loadfn("glDrawArraysInstanced"), failing::glDrawArraysInstanced),
		DrawElementsInstanced: FPointer::load(loadfn("glDrawElementsInstanced"), failing::glDrawElementsInstanced),
		TexBuffer: FPointer::load(loadfn("glTexBuffer"), failing::glTexBuffer),
		PrimitiveRestartIndex: FPointer::load(loadfn("glPrimitiveRestartIndex"), failing::glPrimitiveRestartIndex),
		
		// Core Extension: ARB_uniform_buffer_object
		GetUniformIndices: FPointer::load(loadfn("glGetUniformIndices"), failing::glGetUniformIndices),
		GetActiveUniformsiv: FPointer::load(loadfn("glGetActiveUniformsiv"), failing::glGetActiveUniformsiv),
		GetActiveUniformName: FPointer::load(loadfn("glGetActiveUniformName"), failing::glGetActiveUniformName),
		GetUniformBlockIndex: FPointer::load(loadfn("glGetUniformBlockIndex"), failing::glGetUniformBlockIndex),
		GetActiveUniformBlockiv: FPointer::load(loadfn("glGetActiveUniformBlockiv"), failing::glGetActiveUniformBlockiv),
		GetActiveUniformBlockName: FPointer::load(loadfn("glGetActiveUniformBlockName"), failing::glGetActiveUniformBlockName),
		UniformBlockBinding: FPointer::load(loadfn("glUniformBlockBinding"), failing::glUniformBlockBinding),
		
		// Core Extension: ARB_copy_buffer
		CopyBufferSubData: FPointer::load(loadfn("glCopyBufferSubData"), failing::glCopyBufferSubData),
		
		// Version: 3.2
		GetInteger64i_v: FPointer::load(loadfn("glGetInteger64i_v"), failing::glGetInteger64i_v),
		GetBufferParameteri64v: FPointer::load(loadfn("glGetBufferParameteri64v"), failing::glGetBufferParameteri64v),
		FramebufferTexture: FPointer::load(loadfn("glFramebufferTexture"), failing::glFramebufferTexture),
		
		// Core Extension: ARB_draw_elements_base_vertex
		DrawElementsBaseVertex: FPointer::load(loadfn("glDrawElementsBaseVertex"), failing::glDrawElementsBaseVertex),
		DrawRangeElementsBaseVertex: FPointer::load(loadfn("glDrawRangeElementsBaseVertex"), failing::glDrawRangeElementsBaseVertex),
		DrawElementsInstancedBaseVertex: FPointer::load(loadfn("glDrawElementsInstancedBaseVertex"), failing::glDrawElementsInstancedBaseVertex),
		MultiDrawElementsBaseVertex: FPointer::load(loadfn("glMultiDrawElementsBaseVertex"), failing::glMultiDrawElementsBaseVertex),
		
		// Core Extension: ARB_provoking_vertex
		ProvokingVertex: FPointer::load(loadfn("glProvokingVertex"), failing::glProvokingVertex),
		
		// Core Extension: ARB_sync
		FenceSync: FPointer::load(loadfn("glFenceSync"), failing::glFenceSync),
		IsSync: FPointer::load(loadfn("glIsSync"), failing::glIsSync),
		DeleteSync: FPointer::load(loadfn("glDeleteSync"), failing::glDeleteSync),
		ClientWaitSync: FPointer::load(loadfn("glClientWaitSync"), failing::glClientWaitSync),
		WaitSync: FPointer::load(loadfn("glWaitSync"), failing::glWaitSync),
		GetInteger64v: FPointer::load(loadfn("glGetInteger64v"), failing::glGetInteger64v),
		GetSynciv: FPointer::load(loadfn("glGetSynciv"), failing::glGetSynciv),
		
		// Core Extension: ARB_texture_multisample
		TexImage2DMultisample: FPointer::load(loadfn("glTexImage2DMultisample"), failing::glTexImage2DMultisample),
		TexImage3DMultisample: FPointer::load(loadfn("glTexImage3DMultisample"), failing::glTexImage3DMultisample),
		GetMultisamplefv: FPointer::load(loadfn("glGetMultisamplefv"), failing::glGetMultisamplefv),
		SampleMaski: FPointer::load(loadfn("glSampleMaski"), failing::glSampleMaski),
		
		// Version: 3.3
		VertexAttribDivisor: FPointer::load(loadfn("glVertexAttribDivisor"), failing::glVertexAttribDivisor),
		
		// Core Extension: ARB_timer_query
		QueryCounter: FPointer::load(loadfn("glQueryCounter"), failing::glQueryCounter),
		GetQueryObjecti64v: FPointer::load(loadfn("glGetQueryObjecti64v"), failing::glGetQueryObjecti64v),
		GetQueryObjectui64v: FPointer::load(loadfn("glGetQueryObjectui64v"), failing::glGetQueryObjectui64v),
		
		// Core Extension: ARB_vertex_type_2_10_10_10_rev
		VertexP2ui: FPointer::load(loadfn("glVertexP2ui"), failing::glVertexP2ui),
		VertexP2uiv: FPointer::load(loadfn("glVertexP2uiv"), failing::glVertexP2uiv),
		VertexP3ui: FPointer::load(loadfn("glVertexP3ui"), failing::glVertexP3ui),
		VertexP3uiv: FPointer::load(loadfn("glVertexP3uiv"), failing::glVertexP3uiv),
		VertexP4ui: FPointer::load(loadfn("glVertexP4ui"), failing::glVertexP4ui),
		VertexP4uiv: FPointer::load(loadfn("glVertexP4uiv"), failing::glVertexP4uiv),
		TexCoordP1ui: FPointer::load(loadfn("glTexCoordP1ui"), failing::glTexCoordP1ui),
		TexCoordP1uiv: FPointer::load(loadfn("glTexCoordP1uiv"), failing::glTexCoordP1uiv),
		TexCoordP2ui: FPointer::load(loadfn("glTexCoordP2ui"), failing::glTexCoordP2ui),
		TexCoordP2uiv: FPointer::load(loadfn("glTexCoordP2uiv"), failing::glTexCoordP2uiv),
		TexCoordP3ui: FPointer::load(loadfn("glTexCoordP3ui"), failing::glTexCoordP3ui),
		TexCoordP3uiv: FPointer::load(loadfn("glTexCoordP3uiv"), failing::glTexCoordP3uiv),
		TexCoordP4ui: FPointer::load(loadfn("glTexCoordP4ui"), failing::glTexCoordP4ui),
		TexCoordP4uiv: FPointer::load(loadfn("glTexCoordP4uiv"), failing::glTexCoordP4uiv),
		MultiTexCoordP1ui: FPointer::load(loadfn("glMultiTexCoordP1ui"), failing::glMultiTexCoordP1ui),
		MultiTexCoordP1uiv: FPointer::load(loadfn("glMultiTexCoordP1uiv"), failing::glMultiTexCoordP1uiv),
		MultiTexCoordP2ui: FPointer::load(loadfn("glMultiTexCoordP2ui"), failing::glMultiTexCoordP2ui),
		MultiTexCoordP2uiv: FPointer::load(loadfn("glMultiTexCoordP2uiv"), failing::glMultiTexCoordP2uiv),
		MultiTexCoordP3ui: FPointer::load(loadfn("glMultiTexCoordP3ui"), failing::glMultiTexCoordP3ui),
		MultiTexCoordP3uiv: FPointer::load(loadfn("glMultiTexCoordP3uiv"), failing::glMultiTexCoordP3uiv),
		MultiTexCoordP4ui: FPointer::load(loadfn("glMultiTexCoordP4ui"), failing::glMultiTexCoordP4ui),
		MultiTexCoordP4uiv: FPointer::load(loadfn("glMultiTexCoordP4uiv"), failing::glMultiTexCoordP4uiv),
		NormalP3ui: FPointer::load(loadfn("glNormalP3ui"), failing::glNormalP3ui),
		NormalP3uiv: FPointer::load(loadfn("glNormalP3uiv"), failing::glNormalP3uiv),
		ColorP3ui: FPointer::load(loadfn("glColorP3ui"), failing::glColorP3ui),
		ColorP3uiv: FPointer::load(loadfn("glColorP3uiv"), failing::glColorP3uiv),
		ColorP4ui: FPointer::load(loadfn("glColorP4ui"), failing::glColorP4ui),
		ColorP4uiv: FPointer::load(loadfn("glColorP4uiv"), failing::glColorP4uiv),
		SecondaryColorP3ui: FPointer::load(loadfn("glSecondaryColorP3ui"), failing::glSecondaryColorP3ui),
		SecondaryColorP3uiv: FPointer::load(loadfn("glSecondaryColorP3uiv"), failing::glSecondaryColorP3uiv),
		VertexAttribP1ui: FPointer::load(loadfn("glVertexAttribP1ui"), failing::glVertexAttribP1ui),
		VertexAttribP1uiv: FPointer::load(loadfn("glVertexAttribP1uiv"), failing::glVertexAttribP1uiv),
		VertexAttribP2ui: FPointer::load(loadfn("glVertexAttribP2ui"), failing::glVertexAttribP2ui),
		VertexAttribP2uiv: FPointer::load(loadfn("glVertexAttribP2uiv"), failing::glVertexAttribP2uiv),
		VertexAttribP3ui: FPointer::load(loadfn("glVertexAttribP3ui"), failing::glVertexAttribP3ui),
		VertexAttribP3uiv: FPointer::load(loadfn("glVertexAttribP3uiv"), failing::glVertexAttribP3uiv),
		VertexAttribP4ui: FPointer::load(loadfn("glVertexAttribP4ui"), failing::glVertexAttribP4ui),
		VertexAttribP4uiv: FPointer::load(loadfn("glVertexAttribP4uiv"), failing::glVertexAttribP4uiv),
		
		// Core Extension: ARB_blend_func_extended
		BindFragDataLocationIndexed: FPointer::load(loadfn("glBindFragDataLocationIndexed"), failing::glBindFragDataLocationIndexed),
		GetFragDataIndex: FPointer::load(loadfn("glGetFragDataIndex"), failing::glGetFragDataIndex),
		
		// Core Extension: ARB_sampler_objects
		GenSamplers: FPointer::load(loadfn("glGenSamplers"), failing::glGenSamplers),
		DeleteSamplers: FPointer::load(loadfn("glDeleteSamplers"), failing::glDeleteSamplers),
		IsSampler: FPointer::load(loadfn("glIsSampler"), failing::glIsSampler),
		BindSampler: FPointer::load(loadfn("glBindSampler"), failing::glBindSampler),
		SamplerParameteri: FPointer::load(loadfn("glSamplerParameteri"), failing::glSamplerParameteri),
		SamplerParameteriv: FPointer::load(loadfn("glSamplerParameteriv"), failing::glSamplerParameteriv),
		SamplerParameterf: FPointer::load(loadfn("glSamplerParameterf"), failing::glSamplerParameterf),
		SamplerParameterfv: FPointer::load(loadfn("glSamplerParameterfv"), failing::glSamplerParameterfv),
		SamplerParameterIiv: FPointer::load(loadfn("glSamplerParameterIiv"), failing::glSamplerParameterIiv),
		SamplerParameterIuiv: FPointer::load(loadfn("glSamplerParameterIuiv"), failing::glSamplerParameterIuiv),
		GetSamplerParameteriv: FPointer::load(loadfn("glGetSamplerParameteriv"), failing::glGetSamplerParameteriv),
		GetSamplerParameterIiv: FPointer::load(loadfn("glGetSamplerParameterIiv"), failing::glGetSamplerParameterIiv),
		GetSamplerParameterfv: FPointer::load(loadfn("glGetSamplerParameterfv"), failing::glGetSamplerParameterfv),
		GetSamplerParameterIuiv: FPointer::load(loadfn("glGetSamplerParameterIuiv"), failing::glGetSamplerParameterIuiv),
		
		// Version: 4.0
		MinSampleShading: FPointer::load(loadfn("glMinSampleShading"), failing::glMinSampleShading),
		BlendEquationi: FPointer::load(loadfn("glBlendEquationi"), failing::glBlendEquationi),
		BlendEquationSeparatei: FPointer::load(loadfn("glBlendEquationSeparatei"), failing::glBlendEquationSeparatei),
		BlendFunci: FPointer::load(loadfn("glBlendFunci"), failing::glBlendFunci),
		BlendFuncSeparatei: FPointer::load(loadfn("glBlendFuncSeparatei"), failing::glBlendFuncSeparatei),
		
		// Core Extension: ARB_draw_indirect
		DrawArraysIndirect: FPointer::load(loadfn("glDrawArraysIndirect"), failing::glDrawArraysIndirect),
		DrawElementsIndirect: FPointer::load(loadfn("glDrawElementsIndirect"), failing::glDrawElementsIndirect),
		
		// Core Extension: ARB_gpu_shader_fp64
		Uniform1d: FPointer::load(loadfn("glUniform1d"), failing::glUniform1d),
		Uniform2d: FPointer::load(loadfn("glUniform2d"), failing::glUniform2d),
		Uniform3d: FPointer::load(loadfn("glUniform3d"), failing::glUniform3d),
		Uniform4d: FPointer::load(loadfn("glUniform4d"), failing::glUniform4d),
		Uniform1dv: FPointer::load(loadfn("glUniform1dv"), failing::glUniform1dv),
		Uniform2dv: FPointer::load(loadfn("glUniform2dv"), failing::glUniform2dv),
		Uniform3dv: FPointer::load(loadfn("glUniform3dv"), failing::glUniform3dv),
		Uniform4dv: FPointer::load(loadfn("glUniform4dv"), failing::glUniform4dv),
		UniformMatrix2dv: FPointer::load(loadfn("glUniformMatrix2dv"), failing::glUniformMatrix2dv),
		UniformMatrix3dv: FPointer::load(loadfn("glUniformMatrix3dv"), failing::glUniformMatrix3dv),
		UniformMatrix4dv: FPointer::load(loadfn("glUniformMatrix4dv"), failing::glUniformMatrix4dv),
		UniformMatrix2x3dv: FPointer::load(loadfn("glUniformMatrix2x3dv"), failing::glUniformMatrix2x3dv),
		UniformMatrix2x4dv: FPointer::load(loadfn("glUniformMatrix2x4dv"), failing::glUniformMatrix2x4dv),
		UniformMatrix3x2dv: FPointer::load(loadfn("glUniformMatrix3x2dv"), failing::glUniformMatrix3x2dv),
		UniformMatrix3x4dv: FPointer::load(loadfn("glUniformMatrix3x4dv"), failing::glUniformMatrix3x4dv),
		UniformMatrix4x2dv: FPointer::load(loadfn("glUniformMatrix4x2dv"), failing::glUniformMatrix4x2dv),
		UniformMatrix4x3dv: FPointer::load(loadfn("glUniformMatrix4x3dv"), failing::glUniformMatrix4x3dv),
		GetUniformdv: FPointer::load(loadfn("glGetUniformdv"), failing::glGetUniformdv),
		
		// Core Extension: ARB_shader_subroutine
		GetSubroutineUniformLocation: FPointer::load(loadfn("glGetSubroutineUniformLocation"), failing::glGetSubroutineUniformLocation),
		GetSubroutineIndex: FPointer::load(loadfn("glGetSubroutineIndex"), failing::glGetSubroutineIndex),
		GetActiveSubroutineUniformiv: FPointer::load(loadfn("glGetActiveSubroutineUniformiv"), failing::glGetActiveSubroutineUniformiv),
		GetActiveSubroutineUniformName: FPointer::load(loadfn("glGetActiveSubroutineUniformName"), failing::glGetActiveSubroutineUniformName),
		GetActiveSubroutineName: FPointer::load(loadfn("glGetActiveSubroutineName"), failing::glGetActiveSubroutineName),
		UniformSubroutinesuiv: FPointer::load(loadfn("glUniformSubroutinesuiv"), failing::glUniformSubroutinesuiv),
		GetUniformSubroutineuiv: FPointer::load(loadfn("glGetUniformSubroutineuiv"), failing::glGetUniformSubroutineuiv),
		GetProgramStageiv: FPointer::load(loadfn("glGetProgramStageiv"), failing::glGetProgramStageiv),
		
		// Core Extension: ARB_tessellation_shader
		PatchParameteri: FPointer::load(loadfn("glPatchParameteri"), failing::glPatchParameteri),
		PatchParameterfv: FPointer::load(loadfn("glPatchParameterfv"), failing::glPatchParameterfv),
		
		// Core Extension: ARB_transform_feedback2
		BindTransformFeedback: FPointer::load(loadfn("glBindTransformFeedback"), failing::glBindTransformFeedback),
		DeleteTransformFeedbacks: FPointer::load(loadfn("glDeleteTransformFeedbacks"), failing::glDeleteTransformFeedbacks),
		GenTransformFeedbacks: FPointer::load(loadfn("glGenTransformFeedbacks"), failing::glGenTransformFeedbacks),
		IsTransformFeedback: FPointer::load(loadfn("glIsTransformFeedback"), failing::glIsTransformFeedback),
		PauseTransformFeedback: FPointer::load(loadfn("glPauseTransformFeedback"), failing::glPauseTransformFeedback),
		ResumeTransformFeedback: FPointer::load(loadfn("glResumeTransformFeedback"), failing::glResumeTransformFeedback),
		DrawTransformFeedback: FPointer::load(loadfn("glDrawTransformFeedback"), failing::glDrawTransformFeedback),
		
		// Core Extension: ARB_transform_feedback3
		DrawTransformFeedbackStream: FPointer::load(loadfn("glDrawTransformFeedbackStream"), failing::glDrawTransformFeedbackStream),
		BeginQueryIndexed: FPointer::load(loadfn("glBeginQueryIndexed"), failing::glBeginQueryIndexed),
		EndQueryIndexed: FPointer::load(loadfn("glEndQueryIndexed"), failing::glEndQueryIndexed),
		GetQueryIndexediv: FPointer::load(loadfn("glGetQueryIndexediv"), failing::glGetQueryIndexediv),
		
		// Core Extension: ARB_ES2_compatibility
		ReleaseShaderCompiler: FPointer::load(loadfn("glReleaseShaderCompiler"), failing::glReleaseShaderCompiler),
		ShaderBinary: FPointer::load(loadfn("glShaderBinary"), failing::glShaderBinary),
		GetShaderPrecisionFormat: FPointer::load(loadfn("glGetShaderPrecisionFormat"), failing::glGetShaderPrecisionFormat),
		DepthRangef: FPointer::load(loadfn("glDepthRangef"), failing::glDepthRangef),
		ClearDepthf: FPointer::load(loadfn("glClearDepthf"), failing::glClearDepthf),
		
		// Core Extension: ARB_get_program_binary
		GetProgramBinary: FPointer::load(loadfn("glGetProgramBinary"), failing::glGetProgramBinary),
		ProgramBinary: FPointer::load(loadfn("glProgramBinary"), failing::glProgramBinary),
		ProgramParameteri: FPointer::load(loadfn("glProgramParameteri"), failing::glProgramParameteri),
		
		// Core Extension: ARB_separate_shader_objects
		UseProgramStages: FPointer::load(loadfn("glUseProgramStages"), failing::glUseProgramStages),
		ActiveShaderProgram: FPointer::load(loadfn("glActiveShaderProgram"), failing::glActiveShaderProgram),
		CreateShaderProgramv: FPointer::load(loadfn("glCreateShaderProgramv"), failing::glCreateShaderProgramv),
		BindProgramPipeline: FPointer::load(loadfn("glBindProgramPipeline"), failing::glBindProgramPipeline),
		DeleteProgramPipelines: FPointer::load(loadfn("glDeleteProgramPipelines"), failing::glDeleteProgramPipelines),
		GenProgramPipelines: FPointer::load(loadfn("glGenProgramPipelines"), failing::glGenProgramPipelines),
		IsProgramPipeline: FPointer::load(loadfn("glIsProgramPipeline"), failing::glIsProgramPipeline),
		GetProgramPipelineiv: FPointer::load(loadfn("glGetProgramPipelineiv"), failing::glGetProgramPipelineiv),
		ProgramUniform1i: FPointer::load(loadfn("glProgramUniform1i"), failing::glProgramUniform1i),
		ProgramUniform1iv: FPointer::load(loadfn("glProgramUniform1iv"), failing::glProgramUniform1iv),
		ProgramUniform1f: FPointer::load(loadfn("glProgramUniform1f"), failing::glProgramUniform1f),
		ProgramUniform1fv: FPointer::load(loadfn("glProgramUniform1fv"), failing::glProgramUniform1fv),
		ProgramUniform1d: FPointer::load(loadfn("glProgramUniform1d"), failing::glProgramUniform1d),
		ProgramUniform1dv: FPointer::load(loadfn("glProgramUniform1dv"), failing::glProgramUniform1dv),
		ProgramUniform1ui: FPointer::load(loadfn("glProgramUniform1ui"), failing::glProgramUniform1ui),
		ProgramUniform1uiv: FPointer::load(loadfn("glProgramUniform1uiv"), failing::glProgramUniform1uiv),
		ProgramUniform2i: FPointer::load(loadfn("glProgramUniform2i"), failing::glProgramUniform2i),
		ProgramUniform2iv: FPointer::load(loadfn("glProgramUniform2iv"), failing::glProgramUniform2iv),
		ProgramUniform2f: FPointer::load(loadfn("glProgramUniform2f"), failing::glProgramUniform2f),
		ProgramUniform2fv: FPointer::load(loadfn("glProgramUniform2fv"), failing::glProgramUniform2fv),
		ProgramUniform2d: FPointer::load(loadfn("glProgramUniform2d"), failing::glProgramUniform2d),
		ProgramUniform2dv: FPointer::load(loadfn("glProgramUniform2dv"), failing::glProgramUniform2dv),
		ProgramUniform2ui: FPointer::load(loadfn("glProgramUniform2ui"), failing::glProgramUniform2ui),
		ProgramUniform2uiv: FPointer::load(loadfn("glProgramUniform2uiv"), failing::glProgramUniform2uiv),
		ProgramUniform3i: FPointer::load(loadfn("glProgramUniform3i"), failing::glProgramUniform3i),
		ProgramUniform3iv: FPointer::load(loadfn("glProgramUniform3iv"), failing::glProgramUniform3iv),
		ProgramUniform3f: FPointer::load(loadfn("glProgramUniform3f"), failing::glProgramUniform3f),
		ProgramUniform3fv: FPointer::load(loadfn("glProgramUniform3fv"), failing::glProgramUniform3fv),
		ProgramUniform3d: FPointer::load(loadfn("glProgramUniform3d"), failing::glProgramUniform3d),
		ProgramUniform3dv: FPointer::load(loadfn("glProgramUniform3dv"), failing::glProgramUniform3dv),
		ProgramUniform3ui: FPointer::load(loadfn("glProgramUniform3ui"), failing::glProgramUniform3ui),
		ProgramUniform3uiv: FPointer::load(loadfn("glProgramUniform3uiv"), failing::glProgramUniform3uiv),
		ProgramUniform4i: FPointer::load(loadfn("glProgramUniform4i"), failing::glProgramUniform4i),
		ProgramUniform4iv: FPointer::load(loadfn("glProgramUniform4iv"), failing::glProgramUniform4iv),
		ProgramUniform4f: FPointer::load(loadfn("glProgramUniform4f"), failing::glProgramUniform4f),
		ProgramUniform4fv: FPointer::load(loadfn("glProgramUniform4fv"), failing::glProgramUniform4fv),
		ProgramUniform4d: FPointer::load(loadfn("glProgramUniform4d"), failing::glProgramUniform4d),
		ProgramUniform4dv: FPointer::load(loadfn("glProgramUniform4dv"), failing::glProgramUniform4dv),
		ProgramUniform4ui: FPointer::load(loadfn("glProgramUniform4ui"), failing::glProgramUniform4ui),
		ProgramUniform4uiv: FPointer::load(loadfn("glProgramUniform4uiv"), failing::glProgramUniform4uiv),
		ProgramUniformMatrix2fv: FPointer::load(loadfn("glProgramUniformMatrix2fv"), failing::glProgramUniformMatrix2fv),
		ProgramUniformMatrix3fv: FPointer::load(loadfn("glProgramUniformMatrix3fv"), failing::glProgramUniformMatrix3fv),
		ProgramUniformMatrix4fv: FPointer::load(loadfn("glProgramUniformMatrix4fv"), failing::glProgramUniformMatrix4fv),
		ProgramUniformMatrix2dv: FPointer::load(loadfn("glProgramUniformMatrix2dv"), failing::glProgramUniformMatrix2dv),
		ProgramUniformMatrix3dv: FPointer::load(loadfn("glProgramUniformMatrix3dv"), failing::glProgramUniformMatrix3dv),
		ProgramUniformMatrix4dv: FPointer::load(loadfn("glProgramUniformMatrix4dv"), failing::glProgramUniformMatrix4dv),
		ProgramUniformMatrix2x3fv: FPointer::load(loadfn("glProgramUniformMatrix2x3fv"), failing::glProgramUniformMatrix2x3fv),
		ProgramUniformMatrix3x2fv: FPointer::load(loadfn("glProgramUniformMatrix3x2fv"), failing::glProgramUniformMatrix3x2fv),
		ProgramUniformMatrix2x4fv: FPointer::load(loadfn("glProgramUniformMatrix2x4fv"), failing::glProgramUniformMatrix2x4fv),
		ProgramUniformMatrix4x2fv: FPointer::load(loadfn("glProgramUniformMatrix4x2fv"), failing::glProgramUniformMatrix4x2fv),
		ProgramUniformMatrix3x4fv: FPointer::load(loadfn("glProgramUniformMatrix3x4fv"), failing::glProgramUniformMatrix3x4fv),
		ProgramUniformMatrix4x3fv: FPointer::load(loadfn("glProgramUniformMatrix4x3fv"), failing::glProgramUniformMatrix4x3fv),
		ProgramUniformMatrix2x3dv: FPointer::load(loadfn("glProgramUniformMatrix2x3dv"), failing::glProgramUniformMatrix2x3dv),
		ProgramUniformMatrix3x2dv: FPointer::load(loadfn("glProgramUniformMatrix3x2dv"), failing::glProgramUniformMatrix3x2dv),
		ProgramUniformMatrix2x4dv: FPointer::load(loadfn("glProgramUniformMatrix2x4dv"), failing::glProgramUniformMatrix2x4dv),
		ProgramUniformMatrix4x2dv: FPointer::load(loadfn("glProgramUniformMatrix4x2dv"), failing::glProgramUniformMatrix4x2dv),
		ProgramUniformMatrix3x4dv: FPointer::load(loadfn("glProgramUniformMatrix3x4dv"), failing::glProgramUniformMatrix3x4dv),
		ProgramUniformMatrix4x3dv: FPointer::load(loadfn("glProgramUniformMatrix4x3dv"), failing::glProgramUniformMatrix4x3dv),
		ValidateProgramPipeline: FPointer::load(loadfn("glValidateProgramPipeline"), failing::glValidateProgramPipeline),
		GetProgramPipelineInfoLog: FPointer::load(loadfn("glGetProgramPipelineInfoLog"), failing::glGetProgramPipelineInfoLog),
		
		// Core Extension: ARB_vertex_attrib_64bit
		VertexAttribL1d: FPointer::load(loadfn("glVertexAttribL1d"), failing::glVertexAttribL1d),
		VertexAttribL2d: FPointer::load(loadfn("glVertexAttribL2d"), failing::glVertexAttribL2d),
		VertexAttribL3d: FPointer::load(loadfn("glVertexAttribL3d"), failing::glVertexAttribL3d),
		VertexAttribL4d: FPointer::load(loadfn("glVertexAttribL4d"), failing::glVertexAttribL4d),
		VertexAttribL1dv: FPointer::load(loadfn("glVertexAttribL1dv"), failing::glVertexAttribL1dv),
		VertexAttribL2dv: FPointer::load(loadfn("glVertexAttribL2dv"), failing::glVertexAttribL2dv),
		VertexAttribL3dv: FPointer::load(loadfn("glVertexAttribL3dv"), failing::glVertexAttribL3dv),
		VertexAttribL4dv: FPointer::load(loadfn("glVertexAttribL4dv"), failing::glVertexAttribL4dv),
		VertexAttribLPointer: FPointer::load(loadfn("glVertexAttribLPointer"), failing::glVertexAttribLPointer),
		GetVertexAttribLdv: FPointer::load(loadfn("glGetVertexAttribLdv"), failing::glGetVertexAttribLdv),
		
		// Core Extension: ARB_viewport_array
		ViewportArrayv: FPointer::load(loadfn("glViewportArrayv"), failing::glViewportArrayv),
		ViewportIndexedf: FPointer::load(loadfn("glViewportIndexedf"), failing::glViewportIndexedf),
		ViewportIndexedfv: FPointer::load(loadfn("glViewportIndexedfv"), failing::glViewportIndexedfv),
		ScissorArrayv: FPointer::load(loadfn("glScissorArrayv"), failing::glScissorArrayv),
		ScissorIndexed: FPointer::load(loadfn("glScissorIndexed"), failing::glScissorIndexed),
		ScissorIndexedv: FPointer::load(loadfn("glScissorIndexedv"), failing::glScissorIndexedv),
		DepthRangeArrayv: FPointer::load(loadfn("glDepthRangeArrayv"), failing::glDepthRangeArrayv),
		DepthRangeIndexed: FPointer::load(loadfn("glDepthRangeIndexed"), failing::glDepthRangeIndexed),
		GetFloati_v: FPointer::load(loadfn("glGetFloati_v"), failing::glGetFloati_v),
		GetDoublei_v: FPointer::load(loadfn("glGetDoublei_v"), failing::glGetDoublei_v),
		
		// Core Extension: ARB_base_instance
		DrawArraysInstancedBaseInstance: FPointer::load(loadfn("glDrawArraysInstancedBaseInstance"), failing::glDrawArraysInstancedBaseInstance),
		DrawElementsInstancedBaseInstance: FPointer::load(loadfn("glDrawElementsInstancedBaseInstance"), failing::glDrawElementsInstancedBaseInstance),
		DrawElementsInstancedBaseVertexBaseInstance: FPointer::load(loadfn("glDrawElementsInstancedBaseVertexBaseInstance"), failing::glDrawElementsInstancedBaseVertexBaseInstance),
		
		// Core Extension: ARB_transform_feedback_instanced
		DrawTransformFeedbackInstanced: FPointer::load(loadfn("glDrawTransformFeedbackInstanced"), failing::glDrawTransformFeedbackInstanced),
		DrawTransformFeedbackStreamInstanced: FPointer::load(loadfn("glDrawTransformFeedbackStreamInstanced"), failing::glDrawTransformFeedbackStreamInstanced),
		
		// Core Extension: ARB_internalformat_query
		GetInternalformativ: FPointer::load(loadfn("glGetInternalformativ"), failing::glGetInternalformativ),
		
		// Core Extension: ARB_shader_atomic_counters
		GetActiveAtomicCounterBufferiv: FPointer::load(loadfn("glGetActiveAtomicCounterBufferiv"), failing::glGetActiveAtomicCounterBufferiv),
		
		// Core Extension: ARB_shader_image_load_store
		BindImageTexture: FPointer::load(loadfn("glBindImageTexture"), failing::glBindImageTexture),
		MemoryBarrier: FPointer::load(loadfn("glMemoryBarrier"), failing::glMemoryBarrier),
		
		// Core Extension: ARB_texture_storage
		TexStorage1D: FPointer::load(loadfn("glTexStorage1D"), failing::glTexStorage1D),
		TexStorage2D: FPointer::load(loadfn("glTexStorage2D"), failing::glTexStorage2D),
		TexStorage3D: FPointer::load(loadfn("glTexStorage3D"), failing::glTexStorage3D),
		TextureStorage1DEXT: FPointer::load(loadfn("glTextureStorage1DEXT"), failing::glTextureStorage1DEXT),
		TextureStorage2DEXT: FPointer::load(loadfn("glTextureStorage2DEXT"), failing::glTextureStorage2DEXT),
		TextureStorage3DEXT: FPointer::load(loadfn("glTextureStorage3DEXT"), failing::glTextureStorage3DEXT),
		
		// Core Extension: KHR_debug
		DebugMessageControl: FPointer::load(loadfn("glDebugMessageControl"), failing::glDebugMessageControl),
		DebugMessageInsert: FPointer::load(loadfn("glDebugMessageInsert"), failing::glDebugMessageInsert),
		DebugMessageCallback: FPointer::load(loadfn("glDebugMessageCallback"), failing::glDebugMessageCallback),
		GetDebugMessageLog: FPointer::load(loadfn("glGetDebugMessageLog"), failing::glGetDebugMessageLog),
		PushDebugGroup: FPointer::load(loadfn("glPushDebugGroup"), failing::glPushDebugGroup),
		PopDebugGroup: FPointer::load(loadfn("glPopDebugGroup"), failing::glPopDebugGroup),
		ObjectLabel: FPointer::load(loadfn("glObjectLabel"), failing::glObjectLabel),
		GetObjectLabel: FPointer::load(loadfn("glGetObjectLabel"), failing::glGetObjectLabel),
		ObjectPtrLabel: FPointer::load(loadfn("glObjectPtrLabel"), failing::glObjectPtrLabel),
		GetObjectPtrLabel: FPointer::load(loadfn("glGetObjectPtrLabel"), failing::glGetObjectPtrLabel),
		
		// Core Extension: ARB_clear_buffer_object
		ClearBufferData: FPointer::load(loadfn("glClearBufferData"), failing::glClearBufferData),
		ClearBufferSubData: FPointer::load(loadfn("glClearBufferSubData"), failing::glClearBufferSubData),
		ClearNamedBufferDataEXT: FPointer::load(loadfn("glClearNamedBufferDataEXT"), failing::glClearNamedBufferDataEXT),
		ClearNamedBufferSubDataEXT: FPointer::load(loadfn("glClearNamedBufferSubDataEXT"), failing::glClearNamedBufferSubDataEXT),
		
		// Core Extension: ARB_compute_shader
		DispatchCompute: FPointer::load(loadfn("glDispatchCompute"), failing::glDispatchCompute),
		DispatchComputeIndirect: FPointer::load(loadfn("glDispatchComputeIndirect"), failing::glDispatchComputeIndirect),
		
		// Core Extension: ARB_copy_image
		CopyImageSubData: FPointer::load(loadfn("glCopyImageSubData"), failing::glCopyImageSubData),
		
		// Core Extension: ARB_framebuffer_no_attachments
		FramebufferParameteri: FPointer::load(loadfn("glFramebufferParameteri"), failing::glFramebufferParameteri),
		GetFramebufferParameteriv: FPointer::load(loadfn("glGetFramebufferParameteriv"), failing::glGetFramebufferParameteriv),
		NamedFramebufferParameteriEXT: FPointer::load(loadfn("glNamedFramebufferParameteriEXT"), failing::glNamedFramebufferParameteriEXT),
		GetNamedFramebufferParameterivEXT: FPointer::load(loadfn("glGetNamedFramebufferParameterivEXT"), failing::glGetNamedFramebufferParameterivEXT),
		
		// Core Extension: ARB_internalformat_query2
		GetInternalformati64v: FPointer::load(loadfn("glGetInternalformati64v"), failing::glGetInternalformati64v),
		
		// Core Extension: ARB_invalidate_subdata
		InvalidateTexSubImage: FPointer::load(loadfn("glInvalidateTexSubImage"), failing::glInvalidateTexSubImage),
		InvalidateTexImage: FPointer::load(loadfn("glInvalidateTexImage"), failing::glInvalidateTexImage),
		InvalidateBufferSubData: FPointer::load(loadfn("glInvalidateBufferSubData"), failing::glInvalidateBufferSubData),
		InvalidateBufferData: FPointer::load(loadfn("glInvalidateBufferData"), failing::glInvalidateBufferData),
		InvalidateFramebuffer: FPointer::load(loadfn("glInvalidateFramebuffer"), failing::glInvalidateFramebuffer),
		InvalidateSubFramebuffer: FPointer::load(loadfn("glInvalidateSubFramebuffer"), failing::glInvalidateSubFramebuffer),
		
		// Core Extension: ARB_multi_draw_indirect
		MultiDrawArraysIndirect: FPointer::load(loadfn("glMultiDrawArraysIndirect"), failing::glMultiDrawArraysIndirect),
		MultiDrawElementsIndirect: FPointer::load(loadfn("glMultiDrawElementsIndirect"), failing::glMultiDrawElementsIndirect),
		
		// Core Extension: ARB_program_interface_query
		GetProgramInterfaceiv: FPointer::load(loadfn("glGetProgramInterfaceiv"), failing::glGetProgramInterfaceiv),
		GetProgramResourceIndex: FPointer::load(loadfn("glGetProgramResourceIndex"), failing::glGetProgramResourceIndex),
		GetProgramResourceName: FPointer::load(loadfn("glGetProgramResourceName"), failing::glGetProgramResourceName),
		GetProgramResourceiv: FPointer::load(loadfn("glGetProgramResourceiv"), failing::glGetProgramResourceiv),
		GetProgramResourceLocation: FPointer::load(loadfn("glGetProgramResourceLocation"), failing::glGetProgramResourceLocation),
		GetProgramResourceLocationIndex: FPointer::load(loadfn("glGetProgramResourceLocationIndex"), failing::glGetProgramResourceLocationIndex),
		
		// Core Extension: ARB_shader_storage_buffer_object
		ShaderStorageBlockBinding: FPointer::load(loadfn("glShaderStorageBlockBinding"), failing::glShaderStorageBlockBinding),
		
		// Core Extension: ARB_texture_buffer_range
		TexBufferRange: FPointer::load(loadfn("glTexBufferRange"), failing::glTexBufferRange),
		TextureBufferRangeEXT: FPointer::load(loadfn("glTextureBufferRangeEXT"), failing::glTextureBufferRangeEXT),
		
		// Core Extension: ARB_texture_storage_multisample
		TexStorage2DMultisample: FPointer::load(loadfn("glTexStorage2DMultisample"), failing::glTexStorage2DMultisample),
		TexStorage3DMultisample: FPointer::load(loadfn("glTexStorage3DMultisample"), failing::glTexStorage3DMultisample),
		TextureStorage2DMultisampleEXT: FPointer::load(loadfn("glTextureStorage2DMultisampleEXT"), failing::glTextureStorage2DMultisampleEXT),
		TextureStorage3DMultisampleEXT: FPointer::load(loadfn("glTextureStorage3DMultisampleEXT"), failing::glTextureStorage3DMultisampleEXT),
		
		// Core Extension: ARB_texture_view
		TextureView: FPointer::load(loadfn("glTextureView"), failing::glTextureView),
		
		// Core Extension: ARB_vertex_attrib_binding
		BindVertexBuffer: FPointer::load(loadfn("glBindVertexBuffer"), failing::glBindVertexBuffer),
		VertexAttribFormat: FPointer::load(loadfn("glVertexAttribFormat"), failing::glVertexAttribFormat),
		VertexAttribIFormat: FPointer::load(loadfn("glVertexAttribIFormat"), failing::glVertexAttribIFormat),
		VertexAttribLFormat: FPointer::load(loadfn("glVertexAttribLFormat"), failing::glVertexAttribLFormat),
		VertexAttribBinding: FPointer::load(loadfn("glVertexAttribBinding"), failing::glVertexAttribBinding),
		VertexBindingDivisor: FPointer::load(loadfn("glVertexBindingDivisor"), failing::glVertexBindingDivisor),
		VertexArrayBindVertexBufferEXT: FPointer::load(loadfn("glVertexArrayBindVertexBufferEXT"), failing::glVertexArrayBindVertexBufferEXT),
		VertexArrayVertexAttribFormatEXT: FPointer::load(loadfn("glVertexArrayVertexAttribFormatEXT"), failing::glVertexArrayVertexAttribFormatEXT),
		VertexArrayVertexAttribIFormatEXT: FPointer::load(loadfn("glVertexArrayVertexAttribIFormatEXT"), failing::glVertexArrayVertexAttribIFormatEXT),
		VertexArrayVertexAttribLFormatEXT: FPointer::load(loadfn("glVertexArrayVertexAttribLFormatEXT"), failing::glVertexArrayVertexAttribLFormatEXT),
		VertexArrayVertexAttribBindingEXT: FPointer::load(loadfn("glVertexArrayVertexAttribBindingEXT"), failing::glVertexArrayVertexAttribBindingEXT),
		VertexArrayVertexBindingDivisorEXT: FPointer::load(loadfn("glVertexArrayVertexBindingDivisorEXT"), failing::glVertexArrayVertexBindingDivisorEXT),
		
	}
}

//////////////////////////////////////////////////////////////////////////////
//
// Function Wrapper Methods
//
//////////////////////////////////////////////////////////////////////////////

pub impl GL {
	// Version: 1.1
	#[inline(always)] unsafe fn CullFace(&self, mode: GLenum) { (self.CullFace.f)(mode) }
	#[inline(always)] unsafe fn FrontFace(&self, mode: GLenum) { (self.FrontFace.f)(mode) }
	#[inline(always)] unsafe fn Hint(&self, target: GLenum, mode: GLenum) { (self.Hint.f)(target, mode) }
	#[inline(always)] unsafe fn LineWidth(&self, width: GLfloat) { (self.LineWidth.f)(width) }
	#[inline(always)] unsafe fn PointSize(&self, size: GLfloat) { (self.PointSize.f)(size) }
	#[inline(always)] unsafe fn PolygonMode(&self, face: GLenum, mode: GLenum) { (self.PolygonMode.f)(face, mode) }
	#[inline(always)] unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (self.Scissor.f)(x, y, width, height) }
	#[inline(always)] unsafe fn TexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) { (self.TexParameterf.f)(target, pname, param) }
	#[inline(always)] unsafe fn TexParameterfv(&self, target: GLenum, pname: GLenum, params: *GLfloat) { (self.TexParameterfv.f)(target, pname, params) }
	#[inline(always)] unsafe fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) { (self.TexParameteri.f)(target, pname, param) }
	#[inline(always)] unsafe fn TexParameteriv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.TexParameteriv.f)(target, pname, params) }
	#[inline(always)] unsafe fn TexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
	#[inline(always)] unsafe fn TexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
	#[inline(always)] unsafe fn DrawBuffer(&self, mode: GLenum) { (self.DrawBuffer.f)(mode) }
	#[inline(always)] unsafe fn Clear(&self, mask: GLbitfield) { (self.Clear.f)(mask) }
	#[inline(always)] unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (self.ClearColor.f)(red, green, blue, alpha) }
	#[inline(always)] unsafe fn ClearStencil(&self, s: GLint) { (self.ClearStencil.f)(s) }
	#[inline(always)] unsafe fn ClearDepth(&self, depth: GLdouble) { (self.ClearDepth.f)(depth) }
	#[inline(always)] unsafe fn StencilMask(&self, mask: GLuint) { (self.StencilMask.f)(mask) }
	#[inline(always)] unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { (self.ColorMask.f)(red, green, blue, alpha) }
	#[inline(always)] unsafe fn DepthMask(&self, flag: GLboolean) { (self.DepthMask.f)(flag) }
	#[inline(always)] unsafe fn Disable(&self, cap: GLenum) { (self.Disable.f)(cap) }
	#[inline(always)] unsafe fn Enable(&self, cap: GLenum) { (self.Enable.f)(cap) }
	#[inline(always)] unsafe fn Finish(&self) { (self.Finish.f)() }
	#[inline(always)] unsafe fn Flush(&self) { (self.Flush.f)() }
	#[inline(always)] unsafe fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) { (self.BlendFunc.f)(sfactor, dfactor) }
	#[inline(always)] unsafe fn LogicOp(&self, opcode: GLenum) { (self.LogicOp.f)(opcode) }
	#[inline(always)] unsafe fn StencilFunc(&self, func: GLenum, ref_: GLint, mask: GLuint) { (self.StencilFunc.f)(func, ref_, mask) }
	#[inline(always)] unsafe fn StencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) { (self.StencilOp.f)(fail, zfail, zpass) }
	#[inline(always)] unsafe fn DepthFunc(&self, func: GLenum) { (self.DepthFunc.f)(func) }
	#[inline(always)] unsafe fn PixelStoref(&self, pname: GLenum, param: GLfloat) { (self.PixelStoref.f)(pname, param) }
	#[inline(always)] unsafe fn PixelStorei(&self, pname: GLenum, param: GLint) { (self.PixelStorei.f)(pname, param) }
	#[inline(always)] unsafe fn ReadBuffer(&self, mode: GLenum) { (self.ReadBuffer.f)(mode) }
	#[inline(always)] unsafe fn ReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
	#[inline(always)] unsafe fn GetBooleanv(&self, pname: GLenum, params: *GLboolean) { (self.GetBooleanv.f)(pname, params) }
	#[inline(always)] unsafe fn GetDoublev(&self, pname: GLenum, params: *GLdouble) { (self.GetDoublev.f)(pname, params) }
	#[inline(always)] unsafe fn GetError(&self) -> GLenum { (self.GetError.f)() }
	#[inline(always)] unsafe fn GetFloatv(&self, pname: GLenum, params: *GLfloat) { (self.GetFloatv.f)(pname, params) }
	#[inline(always)] unsafe fn GetIntegerv(&self, pname: GLenum, params: *GLint) { (self.GetIntegerv.f)(pname, params) }
	#[inline(always)] unsafe fn GetString(&self, name: GLenum) -> *GLubyte { (self.GetString.f)(name) }
	#[inline(always)] unsafe fn GetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.GetTexImage.f)(target, level, format, type_, pixels) }
	#[inline(always)] unsafe fn GetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *GLfloat) { (self.GetTexParameterfv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetTexParameteriv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *GLfloat) { (self.GetTexLevelParameterfv.f)(target, level, pname, params) }
	#[inline(always)] unsafe fn GetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *GLint) { (self.GetTexLevelParameteriv.f)(target, level, pname, params) }
	#[inline(always)] unsafe fn IsEnabled(&self, cap: GLenum) -> GLboolean { (self.IsEnabled.f)(cap) }
	#[inline(always)] unsafe fn DepthRange(&self, near: GLdouble, far: GLdouble) { (self.DepthRange.f)(near, far) }
	#[inline(always)] unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (self.Viewport.f)(x, y, width, height) }
	#[inline(always)] unsafe fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) { (self.DrawArrays.f)(mode, first, count) }
	#[inline(always)] unsafe fn DrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid) { (self.DrawElements.f)(mode, count, type_, indices) }
	#[inline(always)] unsafe fn GetPointerv(&self, pname: GLenum, params: **GLvoid) { (self.GetPointerv.f)(pname, params) }
	#[inline(always)] unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) { (self.PolygonOffset.f)(factor, units) }
	#[inline(always)] unsafe fn CopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { (self.CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
	#[inline(always)] unsafe fn CopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { (self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
	#[inline(always)] unsafe fn CopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { (self.CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
	#[inline(always)] unsafe fn CopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
	#[inline(always)] unsafe fn TexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
	#[inline(always)] unsafe fn TexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
	#[inline(always)] unsafe fn BindTexture(&self, target: GLenum, texture: GLuint) { (self.BindTexture.f)(target, texture) }
	#[inline(always)] unsafe fn DeleteTextures(&self, n: GLsizei, textures: *GLuint) { (self.DeleteTextures.f)(n, textures) }
	#[inline(always)] unsafe fn GenTextures(&self, n: GLsizei, textures: *GLuint) { (self.GenTextures.f)(n, textures) }
	#[inline(always)] unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean { (self.IsTexture.f)(texture) }
	#[inline(always)] unsafe fn Indexub(&self, c: GLubyte) { (self.Indexub.f)(c) }
	#[inline(always)] unsafe fn Indexubv(&self, c: *GLubyte) { (self.Indexubv.f)(c) }
	
	// Version: 1.2
	#[inline(always)] unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (self.BlendColor.f)(red, green, blue, alpha) }
	#[inline(always)] unsafe fn BlendEquation(&self, mode: GLenum) { (self.BlendEquation.f)(mode) }
	#[inline(always)] unsafe fn DrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid) { (self.DrawRangeElements.f)(mode, start, end, count, type_, indices) }
	#[inline(always)] unsafe fn TexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
	#[inline(always)] unsafe fn TexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (self.TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
	#[inline(always)] unsafe fn CopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (self.CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
	
	// Version: 1.3
	#[inline(always)] unsafe fn ActiveTexture(&self, texture: GLenum) { (self.ActiveTexture.f)(texture) }
	#[inline(always)] unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) { (self.SampleCoverage.f)(value, invert) }
	#[inline(always)] unsafe fn CompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
	#[inline(always)] unsafe fn CompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
	#[inline(always)] unsafe fn CompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
	#[inline(always)] unsafe fn CompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
	#[inline(always)] unsafe fn CompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
	#[inline(always)] unsafe fn CompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (self.CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
	#[inline(always)] unsafe fn GetCompressedTexImage(&self, target: GLenum, level: GLint, img: *GLvoid) { (self.GetCompressedTexImage.f)(target, level, img) }
	
	// Version: 1.4
	#[inline(always)] unsafe fn BlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { (self.BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
	#[inline(always)] unsafe fn MultiDrawArrays(&self, mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei) { (self.MultiDrawArrays.f)(mode, first, count, drawcount) }
	#[inline(always)] unsafe fn MultiDrawElements(&self, mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei) { (self.MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
	#[inline(always)] unsafe fn PointParameterf(&self, pname: GLenum, param: GLfloat) { (self.PointParameterf.f)(pname, param) }
	#[inline(always)] unsafe fn PointParameterfv(&self, pname: GLenum, params: *GLfloat) { (self.PointParameterfv.f)(pname, params) }
	#[inline(always)] unsafe fn PointParameteri(&self, pname: GLenum, param: GLint) { (self.PointParameteri.f)(pname, param) }
	#[inline(always)] unsafe fn PointParameteriv(&self, pname: GLenum, params: *GLint) { (self.PointParameteriv.f)(pname, params) }
	
	// Version: 1.5
	#[inline(always)] unsafe fn GenQueries(&self, n: GLsizei, ids: *GLuint) { (self.GenQueries.f)(n, ids) }
	#[inline(always)] unsafe fn DeleteQueries(&self, n: GLsizei, ids: *GLuint) { (self.DeleteQueries.f)(n, ids) }
	#[inline(always)] unsafe fn IsQuery(&self, id: GLuint) -> GLboolean { (self.IsQuery.f)(id) }
	#[inline(always)] unsafe fn BeginQuery(&self, target: GLenum, id: GLuint) { (self.BeginQuery.f)(target, id) }
	#[inline(always)] unsafe fn EndQuery(&self, target: GLenum) { (self.EndQuery.f)(target) }
	#[inline(always)] unsafe fn GetQueryiv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetQueryiv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *GLint) { (self.GetQueryObjectiv.f)(id, pname, params) }
	#[inline(always)] unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *GLuint) { (self.GetQueryObjectuiv.f)(id, pname, params) }
	#[inline(always)] unsafe fn BindBuffer(&self, target: GLenum, buffer: GLuint) { (self.BindBuffer.f)(target, buffer) }
	#[inline(always)] unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *GLuint) { (self.DeleteBuffers.f)(n, buffers) }
	#[inline(always)] unsafe fn GenBuffers(&self, n: GLsizei, buffers: *GLuint) { (self.GenBuffers.f)(n, buffers) }
	#[inline(always)] unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean { (self.IsBuffer.f)(buffer) }
	#[inline(always)] unsafe fn BufferData(&self, target: GLenum, size: GLsizeiptr, data: *GLvoid, usage: GLenum) { (self.BufferData.f)(target, size, data, usage) }
	#[inline(always)] unsafe fn BufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid) { (self.BufferSubData.f)(target, offset, size, data) }
	#[inline(always)] unsafe fn GetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid) { (self.GetBufferSubData.f)(target, offset, size, data) }
	#[inline(always)] unsafe fn MapBuffer(&self, target: GLenum, access: GLenum) -> *GLvoid { (self.MapBuffer.f)(target, access) }
	#[inline(always)] unsafe fn UnmapBuffer(&self, target: GLenum) -> GLboolean { (self.UnmapBuffer.f)(target) }
	#[inline(always)] unsafe fn GetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetBufferParameteriv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetBufferPointerv(&self, target: GLenum, pname: GLenum, params: **GLvoid) { (self.GetBufferPointerv.f)(target, pname, params) }
	
	// Version: 2.0
	#[inline(always)] unsafe fn BlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) { (self.BlendEquationSeparate.f)(modeRGB, modeAlpha) }
	#[inline(always)] unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *GLenum) { (self.DrawBuffers.f)(n, bufs) }
	#[inline(always)] unsafe fn StencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { (self.StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
	#[inline(always)] unsafe fn StencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) { (self.StencilFuncSeparate.f)(face, func, ref_, mask) }
	#[inline(always)] unsafe fn StencilMaskSeparate(&self, face: GLenum, mask: GLuint) { (self.StencilMaskSeparate.f)(face, mask) }
	#[inline(always)] unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) { (self.AttachShader.f)(program, shader) }
	#[inline(always)] unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *GLchar) { (self.BindAttribLocation.f)(program, index, name) }
	#[inline(always)] unsafe fn CompileShader(&self, shader: GLuint) { (self.CompileShader.f)(shader) }
	#[inline(always)] unsafe fn CreateProgram(&self) -> GLuint { (self.CreateProgram.f)() }
	#[inline(always)] unsafe fn CreateShader(&self, type_: GLenum) -> GLuint { (self.CreateShader.f)(type_) }
	#[inline(always)] unsafe fn DeleteProgram(&self, program: GLuint) { (self.DeleteProgram.f)(program) }
	#[inline(always)] unsafe fn DeleteShader(&self, shader: GLuint) { (self.DeleteShader.f)(shader) }
	#[inline(always)] unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) { (self.DetachShader.f)(program, shader) }
	#[inline(always)] unsafe fn DisableVertexAttribArray(&self, index: GLuint) { (self.DisableVertexAttribArray.f)(index) }
	#[inline(always)] unsafe fn EnableVertexAttribArray(&self, index: GLuint) { (self.EnableVertexAttribArray.f)(index) }
	#[inline(always)] unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, type_: *GLenum, name: *GLchar) { (self.GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
	#[inline(always)] unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, type_: *GLenum, name: *GLchar) { (self.GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
	#[inline(always)] unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *GLsizei, obj: *GLuint) { (self.GetAttachedShaders.f)(program, maxCount, count, obj) }
	#[inline(always)] unsafe fn GetAttribLocation(&self, program: GLuint, name: *GLchar) -> GLint { (self.GetAttribLocation.f)(program, name) }
	#[inline(always)] unsafe fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: *GLint) { (self.GetProgramiv.f)(program, pname, params) }
	#[inline(always)] unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) { (self.GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
	#[inline(always)] unsafe fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: *GLint) { (self.GetShaderiv.f)(shader, pname, params) }
	#[inline(always)] unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) { (self.GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
	#[inline(always)] unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *GLsizei, source: *GLchar) { (self.GetShaderSource.f)(shader, bufSize, length, source) }
	#[inline(always)] unsafe fn GetUniformLocation(&self, program: GLuint, name: *GLchar) -> GLint { (self.GetUniformLocation.f)(program, name) }
	#[inline(always)] unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *GLfloat) { (self.GetUniformfv.f)(program, location, params) }
	#[inline(always)] unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *GLint) { (self.GetUniformiv.f)(program, location, params) }
	#[inline(always)] unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *GLdouble) { (self.GetVertexAttribdv.f)(index, pname, params) }
	#[inline(always)] unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *GLfloat) { (self.GetVertexAttribfv.f)(index, pname, params) }
	#[inline(always)] unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *GLint) { (self.GetVertexAttribiv.f)(index, pname, params) }
	#[inline(always)] unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: **GLvoid) { (self.GetVertexAttribPointerv.f)(index, pname, pointer) }
	#[inline(always)] unsafe fn IsProgram(&self, program: GLuint) -> GLboolean { (self.IsProgram.f)(program) }
	#[inline(always)] unsafe fn IsShader(&self, shader: GLuint) -> GLboolean { (self.IsShader.f)(shader) }
	#[inline(always)] unsafe fn LinkProgram(&self, program: GLuint) { (self.LinkProgram.f)(program) }
	#[inline(always)] unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint) { (self.ShaderSource.f)(shader, count, string, length) }
	#[inline(always)] unsafe fn UseProgram(&self, program: GLuint) { (self.UseProgram.f)(program) }
	#[inline(always)] unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) { (self.Uniform1f.f)(location, v0) }
	#[inline(always)] unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) { (self.Uniform2f.f)(location, v0, v1) }
	#[inline(always)] unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { (self.Uniform3f.f)(location, v0, v1, v2) }
	#[inline(always)] unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { (self.Uniform4f.f)(location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn Uniform1i(&self, location: GLint, v0: GLint) { (self.Uniform1i.f)(location, v0) }
	#[inline(always)] unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) { (self.Uniform2i.f)(location, v0, v1) }
	#[inline(always)] unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) { (self.Uniform3i.f)(location, v0, v1, v2) }
	#[inline(always)] unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { (self.Uniform4i.f)(location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *GLfloat) { (self.Uniform1fv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *GLfloat) { (self.Uniform2fv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *GLfloat) { (self.Uniform3fv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *GLfloat) { (self.Uniform4fv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *GLint) { (self.Uniform1iv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *GLint) { (self.Uniform2iv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *GLint) { (self.Uniform3iv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *GLint) { (self.Uniform4iv.f)(location, count, value) }
	#[inline(always)] unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix2fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix3fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix4fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn ValidateProgram(&self, program: GLuint) { (self.ValidateProgram.f)(program) }
	#[inline(always)] unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *GLvoid) { (self.VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
	
	// Version: 2.1
	#[inline(always)] unsafe fn UniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix2x3fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix3x2fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix2x4fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix4x2fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix3x4fv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.UniformMatrix4x3fv.f)(location, count, transpose, value) }
	
	// Version: 3.0
	#[inline(always)] unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) { (self.ColorMaski.f)(index, r, g, b, a) }
	#[inline(always)] unsafe fn GetBooleani_v(&self, target: GLenum, index: GLuint, data: *GLboolean) { (self.GetBooleani_v.f)(target, index, data) }
	#[inline(always)] unsafe fn GetIntegeri_v(&self, target: GLenum, index: GLuint, data: *GLint) { (self.GetIntegeri_v.f)(target, index, data) }
	#[inline(always)] unsafe fn Enablei(&self, target: GLenum, index: GLuint) { (self.Enablei.f)(target, index) }
	#[inline(always)] unsafe fn Disablei(&self, target: GLenum, index: GLuint) { (self.Disablei.f)(target, index) }
	#[inline(always)] unsafe fn IsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean { (self.IsEnabledi.f)(target, index) }
	#[inline(always)] unsafe fn BeginTransformFeedback(&self, primitiveMode: GLenum) { (self.BeginTransformFeedback.f)(primitiveMode) }
	#[inline(always)] unsafe fn EndTransformFeedback(&self) { (self.EndTransformFeedback.f)() }
	#[inline(always)] unsafe fn BindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { (self.BindBufferRange.f)(target, index, buffer, offset, size) }
	#[inline(always)] unsafe fn BindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) { (self.BindBufferBase.f)(target, index, buffer) }
	#[inline(always)] unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: **GLchar, bufferMode: GLenum) { (self.TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
	#[inline(always)] unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLsizei, type_: *GLenum, name: *GLchar) { (self.GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
	#[inline(always)] unsafe fn ClampColor(&self, target: GLenum, clamp: GLenum) { (self.ClampColor.f)(target, clamp) }
	#[inline(always)] unsafe fn BeginConditionalRender(&self, id: GLuint, mode: GLenum) { (self.BeginConditionalRender.f)(id, mode) }
	#[inline(always)] unsafe fn EndConditionalRender(&self) { (self.EndConditionalRender.f)() }
	#[inline(always)] unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) { (self.VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
	#[inline(always)] unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *GLint) { (self.GetVertexAttribIiv.f)(index, pname, params) }
	#[inline(always)] unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *GLuint) { (self.GetVertexAttribIuiv.f)(index, pname, params) }
	#[inline(always)] unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) { (self.VertexAttribI1i.f)(index, x) }
	#[inline(always)] unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) { (self.VertexAttribI2i.f)(index, x, y) }
	#[inline(always)] unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) { (self.VertexAttribI3i.f)(index, x, y, z) }
	#[inline(always)] unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) { (self.VertexAttribI4i.f)(index, x, y, z, w) }
	#[inline(always)] unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) { (self.VertexAttribI1ui.f)(index, x) }
	#[inline(always)] unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) { (self.VertexAttribI2ui.f)(index, x, y) }
	#[inline(always)] unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) { (self.VertexAttribI3ui.f)(index, x, y, z) }
	#[inline(always)] unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) { (self.VertexAttribI4ui.f)(index, x, y, z, w) }
	#[inline(always)] unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *GLint) { (self.VertexAttribI1iv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *GLint) { (self.VertexAttribI2iv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *GLint) { (self.VertexAttribI3iv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *GLint) { (self.VertexAttribI4iv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *GLuint) { (self.VertexAttribI1uiv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *GLuint) { (self.VertexAttribI2uiv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *GLuint) { (self.VertexAttribI3uiv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *GLuint) { (self.VertexAttribI4uiv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *GLbyte) { (self.VertexAttribI4bv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *GLshort) { (self.VertexAttribI4sv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *GLubyte) { (self.VertexAttribI4ubv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *GLushort) { (self.VertexAttribI4usv.f)(index, v) }
	#[inline(always)] unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *GLuint) { (self.GetUniformuiv.f)(program, location, params) }
	#[inline(always)] unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *GLchar) { (self.BindFragDataLocation.f)(program, color, name) }
	#[inline(always)] unsafe fn GetFragDataLocation(&self, program: GLuint, name: *GLchar) -> GLint { (self.GetFragDataLocation.f)(program, name) }
	#[inline(always)] unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) { (self.Uniform1ui.f)(location, v0) }
	#[inline(always)] unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) { (self.Uniform2ui.f)(location, v0, v1) }
	#[inline(always)] unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { (self.Uniform3ui.f)(location, v0, v1, v2) }
	#[inline(always)] unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { (self.Uniform4ui.f)(location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *GLuint) { (self.Uniform1uiv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *GLuint) { (self.Uniform2uiv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *GLuint) { (self.Uniform3uiv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *GLuint) { (self.Uniform4uiv.f)(location, count, value) }
	#[inline(always)] unsafe fn TexParameterIiv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.TexParameterIiv.f)(target, pname, params) }
	#[inline(always)] unsafe fn TexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *GLuint) { (self.TexParameterIuiv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetTexParameterIiv.f)(target, pname, params) }
	#[inline(always)] unsafe fn GetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *GLuint) { (self.GetTexParameterIuiv.f)(target, pname, params) }
	#[inline(always)] unsafe fn ClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *GLint) { (self.ClearBufferiv.f)(buffer, drawbuffer, value) }
	#[inline(always)] unsafe fn ClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *GLuint) { (self.ClearBufferuiv.f)(buffer, drawbuffer, value) }
	#[inline(always)] unsafe fn ClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *GLfloat) { (self.ClearBufferfv.f)(buffer, drawbuffer, value) }
	#[inline(always)] unsafe fn ClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { (self.ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) }
	#[inline(always)] unsafe fn GetStringi(&self, name: GLenum, index: GLuint) -> *GLubyte { (self.GetStringi.f)(name, index) }
	
	// Core Extension: ARB_vertex_array_object
	#[inline(always)] unsafe fn BindVertexArray(&self, array: GLuint) { (self.BindVertexArray.f)(array) }
	#[inline(always)] unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *GLuint) { (self.DeleteVertexArrays.f)(n, arrays) }
	#[inline(always)] unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *GLuint) { (self.GenVertexArrays.f)(n, arrays) }
	#[inline(always)] unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean { (self.IsVertexArray.f)(array) }
	
	// Core Extension: ARB_map_buffer_range
	#[inline(always)] unsafe fn MapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *GLvoid { (self.MapBufferRange.f)(target, offset, length, access) }
	#[inline(always)] unsafe fn FlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) { (self.FlushMappedBufferRange.f)(target, offset, length) }
	
	// Core Extension: ARB_framebuffer_object
	#[inline(always)] unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean { (self.IsRenderbuffer.f)(renderbuffer) }
	#[inline(always)] unsafe fn BindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) { (self.BindRenderbuffer.f)(target, renderbuffer) }
	#[inline(always)] unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *GLuint) { (self.DeleteRenderbuffers.f)(n, renderbuffers) }
	#[inline(always)] unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *GLuint) { (self.GenRenderbuffers.f)(n, renderbuffers) }
	#[inline(always)] unsafe fn RenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) { (self.RenderbufferStorage.f)(target, internalformat, width, height) }
	#[inline(always)] unsafe fn GetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetRenderbufferParameteriv.f)(target, pname, params) }
	#[inline(always)] unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean { (self.IsFramebuffer.f)(framebuffer) }
	#[inline(always)] unsafe fn BindFramebuffer(&self, target: GLenum, framebuffer: GLuint) { (self.BindFramebuffer.f)(target, framebuffer) }
	#[inline(always)] unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *GLuint) { (self.DeleteFramebuffers.f)(n, framebuffers) }
	#[inline(always)] unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *GLuint) { (self.GenFramebuffers.f)(n, framebuffers) }
	#[inline(always)] unsafe fn CheckFramebufferStatus(&self, target: GLenum) -> GLenum { (self.CheckFramebufferStatus.f)(target) }
	#[inline(always)] unsafe fn FramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { (self.FramebufferTexture1D.f)(target, attachment, textarget, texture, level) }
	#[inline(always)] unsafe fn FramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { (self.FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
	#[inline(always)] unsafe fn FramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) { (self.FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) }
	#[inline(always)] unsafe fn FramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { (self.FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
	#[inline(always)] unsafe fn GetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *GLint) { (self.GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
	#[inline(always)] unsafe fn GenerateMipmap(&self, target: GLenum) { (self.GenerateMipmap.f)(target) }
	#[inline(always)] unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { (self.BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
	#[inline(always)] unsafe fn RenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { (self.RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) }
	#[inline(always)] unsafe fn FramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { (self.FramebufferTextureLayer.f)(target, attachment, texture, level, layer) }
	
	// Version: 3.1
	#[inline(always)] unsafe fn DrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) { (self.DrawArraysInstanced.f)(mode, first, count, instancecount) }
	#[inline(always)] unsafe fn DrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei) { (self.DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
	#[inline(always)] unsafe fn TexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) { (self.TexBuffer.f)(target, internalformat, buffer) }
	#[inline(always)] unsafe fn PrimitiveRestartIndex(&self, index: GLuint) { (self.PrimitiveRestartIndex.f)(index) }
	
	// Core Extension: ARB_uniform_buffer_object
	#[inline(always)] unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: **GLchar, uniformIndices: *GLuint) { (self.GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
	#[inline(always)] unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *GLint) { (self.GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
	#[inline(always)] unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *GLsizei, uniformName: *GLchar) { (self.GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
	#[inline(always)] unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *GLchar) -> GLuint { (self.GetUniformBlockIndex.f)(program, uniformBlockName) }
	#[inline(always)] unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *GLint) { (self.GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
	#[inline(always)] unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *GLsizei, uniformBlockName: *GLchar) { (self.GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
	#[inline(always)] unsafe fn UniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) { (self.UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) }
	
	// Core Extension: ARB_copy_buffer
	#[inline(always)] unsafe fn CopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { (self.CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) }
	
	// Version: 3.2
	#[inline(always)] unsafe fn GetInteger64i_v(&self, target: GLenum, index: GLuint, data: *GLint64) { (self.GetInteger64i_v.f)(target, index, data) }
	#[inline(always)] unsafe fn GetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *GLint64) { (self.GetBufferParameteri64v.f)(target, pname, params) }
	#[inline(always)] unsafe fn FramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) { (self.FramebufferTexture.f)(target, attachment, texture, level) }
	
	// Core Extension: ARB_draw_elements_base_vertex
	#[inline(always)] unsafe fn DrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) { (self.DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
	#[inline(always)] unsafe fn DrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) { (self.DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
	#[inline(always)] unsafe fn DrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, basevertex: GLint) { (self.DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
	#[inline(always)] unsafe fn MultiDrawElementsBaseVertex(&self, mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei, basevertex: *GLint) { (self.MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
	
	// Core Extension: ARB_provoking_vertex
	#[inline(always)] unsafe fn ProvokingVertex(&self, mode: GLenum) { (self.ProvokingVertex.f)(mode) }
	
	// Core Extension: ARB_sync
	#[inline(always)] unsafe fn FenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync { (self.FenceSync.f)(condition, flags) }
	#[inline(always)] unsafe fn IsSync(&self, sync: GLsync) -> GLboolean { (self.IsSync.f)(sync) }
	#[inline(always)] unsafe fn DeleteSync(&self, sync: GLsync) { (self.DeleteSync.f)(sync) }
	#[inline(always)] unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { (self.ClientWaitSync.f)(sync, flags, timeout) }
	#[inline(always)] unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) { (self.WaitSync.f)(sync, flags, timeout) }
	#[inline(always)] unsafe fn GetInteger64v(&self, pname: GLenum, params: *GLint64) { (self.GetInteger64v.f)(pname, params) }
	#[inline(always)] unsafe fn GetSynciv(&self, sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *GLsizei, values: *GLint) { (self.GetSynciv.f)(sync, pname, bufSize, length, values) }
	
	// Core Extension: ARB_texture_multisample
	#[inline(always)] unsafe fn TexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLint, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { (self.TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
	#[inline(always)] unsafe fn TexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { (self.TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
	#[inline(always)] unsafe fn GetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *GLfloat) { (self.GetMultisamplefv.f)(pname, index, val) }
	#[inline(always)] unsafe fn SampleMaski(&self, index: GLuint, mask: GLbitfield) { (self.SampleMaski.f)(index, mask) }
	
	// Version: 3.3
	#[inline(always)] unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) { (self.VertexAttribDivisor.f)(index, divisor) }
	
	// Core Extension: ARB_timer_query
	#[inline(always)] unsafe fn QueryCounter(&self, id: GLuint, target: GLenum) { (self.QueryCounter.f)(id, target) }
	#[inline(always)] unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *GLint64) { (self.GetQueryObjecti64v.f)(id, pname, params) }
	#[inline(always)] unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *GLuint64) { (self.GetQueryObjectui64v.f)(id, pname, params) }
	
	// Core Extension: ARB_vertex_type_2_10_10_10_rev
	#[inline(always)] unsafe fn VertexP2ui(&self, type_: GLenum, value: GLuint) { (self.VertexP2ui.f)(type_, value) }
	#[inline(always)] unsafe fn VertexP2uiv(&self, type_: GLenum, value: *GLuint) { (self.VertexP2uiv.f)(type_, value) }
	#[inline(always)] unsafe fn VertexP3ui(&self, type_: GLenum, value: GLuint) { (self.VertexP3ui.f)(type_, value) }
	#[inline(always)] unsafe fn VertexP3uiv(&self, type_: GLenum, value: *GLuint) { (self.VertexP3uiv.f)(type_, value) }
	#[inline(always)] unsafe fn VertexP4ui(&self, type_: GLenum, value: GLuint) { (self.VertexP4ui.f)(type_, value) }
	#[inline(always)] unsafe fn VertexP4uiv(&self, type_: GLenum, value: *GLuint) { (self.VertexP4uiv.f)(type_, value) }
	#[inline(always)] unsafe fn TexCoordP1ui(&self, type_: GLenum, coords: GLuint) { (self.TexCoordP1ui.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP1uiv(&self, type_: GLenum, coords: *GLuint) { (self.TexCoordP1uiv.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP2ui(&self, type_: GLenum, coords: GLuint) { (self.TexCoordP2ui.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP2uiv(&self, type_: GLenum, coords: *GLuint) { (self.TexCoordP2uiv.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP3ui(&self, type_: GLenum, coords: GLuint) { (self.TexCoordP3ui.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP3uiv(&self, type_: GLenum, coords: *GLuint) { (self.TexCoordP3uiv.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP4ui(&self, type_: GLenum, coords: GLuint) { (self.TexCoordP4ui.f)(type_, coords) }
	#[inline(always)] unsafe fn TexCoordP4uiv(&self, type_: GLenum, coords: *GLuint) { (self.TexCoordP4uiv.f)(type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP1ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) { (self.MultiTexCoordP1ui.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP1uiv(&self, texture: GLenum, type_: GLenum, coords: *GLuint) { (self.MultiTexCoordP1uiv.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP2ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) { (self.MultiTexCoordP2ui.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP2uiv(&self, texture: GLenum, type_: GLenum, coords: *GLuint) { (self.MultiTexCoordP2uiv.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP3ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) { (self.MultiTexCoordP3ui.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP3uiv(&self, texture: GLenum, type_: GLenum, coords: *GLuint) { (self.MultiTexCoordP3uiv.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP4ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) { (self.MultiTexCoordP4ui.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn MultiTexCoordP4uiv(&self, texture: GLenum, type_: GLenum, coords: *GLuint) { (self.MultiTexCoordP4uiv.f)(texture, type_, coords) }
	#[inline(always)] unsafe fn NormalP3ui(&self, type_: GLenum, coords: GLuint) { (self.NormalP3ui.f)(type_, coords) }
	#[inline(always)] unsafe fn NormalP3uiv(&self, type_: GLenum, coords: *GLuint) { (self.NormalP3uiv.f)(type_, coords) }
	#[inline(always)] unsafe fn ColorP3ui(&self, type_: GLenum, color: GLuint) { (self.ColorP3ui.f)(type_, color) }
	#[inline(always)] unsafe fn ColorP3uiv(&self, type_: GLenum, color: *GLuint) { (self.ColorP3uiv.f)(type_, color) }
	#[inline(always)] unsafe fn ColorP4ui(&self, type_: GLenum, color: GLuint) { (self.ColorP4ui.f)(type_, color) }
	#[inline(always)] unsafe fn ColorP4uiv(&self, type_: GLenum, color: *GLuint) { (self.ColorP4uiv.f)(type_, color) }
	#[inline(always)] unsafe fn SecondaryColorP3ui(&self, type_: GLenum, color: GLuint) { (self.SecondaryColorP3ui.f)(type_, color) }
	#[inline(always)] unsafe fn SecondaryColorP3uiv(&self, type_: GLenum, color: *GLuint) { (self.SecondaryColorP3uiv.f)(type_, color) }
	#[inline(always)] unsafe fn VertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { (self.VertexAttribP1ui.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (self.VertexAttribP1uiv.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { (self.VertexAttribP2ui.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (self.VertexAttribP2uiv.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { (self.VertexAttribP3ui.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (self.VertexAttribP3uiv.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { (self.VertexAttribP4ui.f)(index, type_, normalized, value) }
	#[inline(always)] unsafe fn VertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (self.VertexAttribP4uiv.f)(index, type_, normalized, value) }
	
	// Core Extension: ARB_blend_func_extended
	#[inline(always)] unsafe fn BindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *GLchar) { (self.BindFragDataLocationIndexed.f)(program, colorNumber, index, name) }
	#[inline(always)] unsafe fn GetFragDataIndex(&self, program: GLuint, name: *GLchar) -> GLint { (self.GetFragDataIndex.f)(program, name) }
	
	// Core Extension: ARB_sampler_objects
	#[inline(always)] unsafe fn GenSamplers(&self, count: GLsizei, samplers: *GLuint) { (self.GenSamplers.f)(count, samplers) }
	#[inline(always)] unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *GLuint) { (self.DeleteSamplers.f)(count, samplers) }
	#[inline(always)] unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean { (self.IsSampler.f)(sampler) }
	#[inline(always)] unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) { (self.BindSampler.f)(unit, sampler) }
	#[inline(always)] unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) { (self.SamplerParameteri.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *GLint) { (self.SamplerParameteriv.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) { (self.SamplerParameterf.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *GLfloat) { (self.SamplerParameterfv.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *GLint) { (self.SamplerParameterIiv.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn SamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *GLuint) { (self.SamplerParameterIuiv.f)(sampler, pname, param) }
	#[inline(always)] unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *GLint) { (self.GetSamplerParameteriv.f)(sampler, pname, params) }
	#[inline(always)] unsafe fn GetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *GLint) { (self.GetSamplerParameterIiv.f)(sampler, pname, params) }
	#[inline(always)] unsafe fn GetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *GLfloat) { (self.GetSamplerParameterfv.f)(sampler, pname, params) }
	#[inline(always)] unsafe fn GetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *GLuint) { (self.GetSamplerParameterIuiv.f)(sampler, pname, params) }
	
	// Version: 4.0
	#[inline(always)] unsafe fn MinSampleShading(&self, value: GLfloat) { (self.MinSampleShading.f)(value) }
	#[inline(always)] unsafe fn BlendEquationi(&self, buf: GLuint, mode: GLenum) { (self.BlendEquationi.f)(buf, mode) }
	#[inline(always)] unsafe fn BlendEquationSeparatei(&self, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) { (self.BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) }
	#[inline(always)] unsafe fn BlendFunci(&self, buf: GLuint, src: GLenum, dst: GLenum) { (self.BlendFunci.f)(buf, src, dst) }
	#[inline(always)] unsafe fn BlendFuncSeparatei(&self, buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) { (self.BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
	
	// Core Extension: ARB_draw_indirect
	#[inline(always)] unsafe fn DrawArraysIndirect(&self, mode: GLenum, indirect: *GLvoid) { (self.DrawArraysIndirect.f)(mode, indirect) }
	#[inline(always)] unsafe fn DrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *GLvoid) { (self.DrawElementsIndirect.f)(mode, type_, indirect) }
	
	// Core Extension: ARB_gpu_shader_fp64
	#[inline(always)] unsafe fn Uniform1d(&self, location: GLint, x: GLdouble) { (self.Uniform1d.f)(location, x) }
	#[inline(always)] unsafe fn Uniform2d(&self, location: GLint, x: GLdouble, y: GLdouble) { (self.Uniform2d.f)(location, x, y) }
	#[inline(always)] unsafe fn Uniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) { (self.Uniform3d.f)(location, x, y, z) }
	#[inline(always)] unsafe fn Uniform4d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { (self.Uniform4d.f)(location, x, y, z, w) }
	#[inline(always)] unsafe fn Uniform1dv(&self, location: GLint, count: GLsizei, value: *GLdouble) { (self.Uniform1dv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform2dv(&self, location: GLint, count: GLsizei, value: *GLdouble) { (self.Uniform2dv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform3dv(&self, location: GLint, count: GLsizei, value: *GLdouble) { (self.Uniform3dv.f)(location, count, value) }
	#[inline(always)] unsafe fn Uniform4dv(&self, location: GLint, count: GLsizei, value: *GLdouble) { (self.Uniform4dv.f)(location, count, value) }
	#[inline(always)] unsafe fn UniformMatrix2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix2dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix3dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix4dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix2x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix2x3dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix2x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix2x4dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix3x2dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix3x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix3x4dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix4x2dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn UniformMatrix4x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.UniformMatrix4x3dv.f)(location, count, transpose, value) }
	#[inline(always)] unsafe fn GetUniformdv(&self, program: GLuint, location: GLint, params: *GLdouble) { (self.GetUniformdv.f)(program, location, params) }
	
	// Core Extension: ARB_shader_subroutine
	#[inline(always)] unsafe fn GetSubroutineUniformLocation(&self, program: GLuint, shadertype: GLenum, name: *GLchar) -> GLint { (self.GetSubroutineUniformLocation.f)(program, shadertype, name) }
	#[inline(always)] unsafe fn GetSubroutineIndex(&self, program: GLuint, shadertype: GLenum, name: *GLchar) -> GLuint { (self.GetSubroutineIndex.f)(program, shadertype, name) }
	#[inline(always)] unsafe fn GetActiveSubroutineUniformiv(&self, program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *GLint) { (self.GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) }
	#[inline(always)] unsafe fn GetActiveSubroutineUniformName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *GLsizei, name: *GLchar) { (self.GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) }
	#[inline(always)] unsafe fn GetActiveSubroutineName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *GLsizei, name: *GLchar) { (self.GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) }
	#[inline(always)] unsafe fn UniformSubroutinesuiv(&self, shadertype: GLenum, count: GLsizei, indices: *GLuint) { (self.UniformSubroutinesuiv.f)(shadertype, count, indices) }
	#[inline(always)] unsafe fn GetUniformSubroutineuiv(&self, shadertype: GLenum, location: GLint, params: *GLuint) { (self.GetUniformSubroutineuiv.f)(shadertype, location, params) }
	#[inline(always)] unsafe fn GetProgramStageiv(&self, program: GLuint, shadertype: GLenum, pname: GLenum, values: *GLint) { (self.GetProgramStageiv.f)(program, shadertype, pname, values) }
	
	// Core Extension: ARB_tessellation_shader
	#[inline(always)] unsafe fn PatchParameteri(&self, pname: GLenum, value: GLint) { (self.PatchParameteri.f)(pname, value) }
	#[inline(always)] unsafe fn PatchParameterfv(&self, pname: GLenum, values: *GLfloat) { (self.PatchParameterfv.f)(pname, values) }
	
	// Core Extension: ARB_transform_feedback2
	#[inline(always)] unsafe fn BindTransformFeedback(&self, target: GLenum, id: GLuint) { (self.BindTransformFeedback.f)(target, id) }
	#[inline(always)] unsafe fn DeleteTransformFeedbacks(&self, n: GLsizei, ids: *GLuint) { (self.DeleteTransformFeedbacks.f)(n, ids) }
	#[inline(always)] unsafe fn GenTransformFeedbacks(&self, n: GLsizei, ids: *GLuint) { (self.GenTransformFeedbacks.f)(n, ids) }
	#[inline(always)] unsafe fn IsTransformFeedback(&self, id: GLuint) -> GLboolean { (self.IsTransformFeedback.f)(id) }
	#[inline(always)] unsafe fn PauseTransformFeedback(&self) { (self.PauseTransformFeedback.f)() }
	#[inline(always)] unsafe fn ResumeTransformFeedback(&self) { (self.ResumeTransformFeedback.f)() }
	#[inline(always)] unsafe fn DrawTransformFeedback(&self, mode: GLenum, id: GLuint) { (self.DrawTransformFeedback.f)(mode, id) }
	
	// Core Extension: ARB_transform_feedback3
	#[inline(always)] unsafe fn DrawTransformFeedbackStream(&self, mode: GLenum, id: GLuint, stream: GLuint) { (self.DrawTransformFeedbackStream.f)(mode, id, stream) }
	#[inline(always)] unsafe fn BeginQueryIndexed(&self, target: GLenum, index: GLuint, id: GLuint) { (self.BeginQueryIndexed.f)(target, index, id) }
	#[inline(always)] unsafe fn EndQueryIndexed(&self, target: GLenum, index: GLuint) { (self.EndQueryIndexed.f)(target, index) }
	#[inline(always)] unsafe fn GetQueryIndexediv(&self, target: GLenum, index: GLuint, pname: GLenum, params: *GLint) { (self.GetQueryIndexediv.f)(target, index, pname, params) }
	
	// Core Extension: ARB_ES2_compatibility
	#[inline(always)] unsafe fn ReleaseShaderCompiler(&self) { (self.ReleaseShaderCompiler.f)() }
	#[inline(always)] unsafe fn ShaderBinary(&self, count: GLsizei, shaders: *GLuint, binaryformat: GLenum, binary: *GLvoid, length: GLsizei) { (self.ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
	#[inline(always)] unsafe fn GetShaderPrecisionFormat(&self, shadertype: GLenum, precisiontype: GLenum, range: *GLint, precision: *GLint) { (self.GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
	#[inline(always)] unsafe fn DepthRangef(&self, n: GLfloat, f: GLfloat) { (self.DepthRangef.f)(n, f) }
	#[inline(always)] unsafe fn ClearDepthf(&self, d: GLfloat) { (self.ClearDepthf.f)(d) }
	
	// Core Extension: ARB_get_program_binary
	#[inline(always)] unsafe fn GetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *GLsizei, binaryFormat: *GLenum, binary: *GLvoid) { (self.GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
	#[inline(always)] unsafe fn ProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *GLvoid, length: GLsizei) { (self.ProgramBinary.f)(program, binaryFormat, binary, length) }
	#[inline(always)] unsafe fn ProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint) { (self.ProgramParameteri.f)(program, pname, value) }
	
	// Core Extension: ARB_separate_shader_objects
	#[inline(always)] unsafe fn UseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) { (self.UseProgramStages.f)(pipeline, stages, program) }
	#[inline(always)] unsafe fn ActiveShaderProgram(&self, pipeline: GLuint, program: GLuint) { (self.ActiveShaderProgram.f)(pipeline, program) }
	#[inline(always)] unsafe fn CreateShaderProgramv(&self, type_: GLenum, count: GLsizei, strings: **GLchar) -> GLuint { (self.CreateShaderProgramv.f)(type_, count, strings) }
	#[inline(always)] unsafe fn BindProgramPipeline(&self, pipeline: GLuint) { (self.BindProgramPipeline.f)(pipeline) }
	#[inline(always)] unsafe fn DeleteProgramPipelines(&self, n: GLsizei, pipelines: *GLuint) { (self.DeleteProgramPipelines.f)(n, pipelines) }
	#[inline(always)] unsafe fn GenProgramPipelines(&self, n: GLsizei, pipelines: *GLuint) { (self.GenProgramPipelines.f)(n, pipelines) }
	#[inline(always)] unsafe fn IsProgramPipeline(&self, pipeline: GLuint) -> GLboolean { (self.IsProgramPipeline.f)(pipeline) }
	#[inline(always)] unsafe fn GetProgramPipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *GLint) { (self.GetProgramPipelineiv.f)(pipeline, pname, params) }
	#[inline(always)] unsafe fn ProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint) { (self.ProgramUniform1i.f)(program, location, v0) }
	#[inline(always)] unsafe fn ProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (self.ProgramUniform1iv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) { (self.ProgramUniform1f.f)(program, location, v0) }
	#[inline(always)] unsafe fn ProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (self.ProgramUniform1fv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble) { (self.ProgramUniform1d.f)(program, location, v0) }
	#[inline(always)] unsafe fn ProgramUniform1dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (self.ProgramUniform1dv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) { (self.ProgramUniform1ui.f)(program, location, v0) }
	#[inline(always)] unsafe fn ProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (self.ProgramUniform1uiv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) { (self.ProgramUniform2i.f)(program, location, v0, v1) }
	#[inline(always)] unsafe fn ProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (self.ProgramUniform2iv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) { (self.ProgramUniform2f.f)(program, location, v0, v1) }
	#[inline(always)] unsafe fn ProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (self.ProgramUniform2fv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform2d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) { (self.ProgramUniform2d.f)(program, location, v0, v1) }
	#[inline(always)] unsafe fn ProgramUniform2dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (self.ProgramUniform2dv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint) { (self.ProgramUniform2ui.f)(program, location, v0, v1) }
	#[inline(always)] unsafe fn ProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (self.ProgramUniform2uiv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform3i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) { (self.ProgramUniform3i.f)(program, location, v0, v1, v2) }
	#[inline(always)] unsafe fn ProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (self.ProgramUniform3iv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform3f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { (self.ProgramUniform3f.f)(program, location, v0, v1, v2) }
	#[inline(always)] unsafe fn ProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (self.ProgramUniform3fv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform3d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) { (self.ProgramUniform3d.f)(program, location, v0, v1, v2) }
	#[inline(always)] unsafe fn ProgramUniform3dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (self.ProgramUniform3dv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform3ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { (self.ProgramUniform3ui.f)(program, location, v0, v1, v2) }
	#[inline(always)] unsafe fn ProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (self.ProgramUniform3uiv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform4i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { (self.ProgramUniform4i.f)(program, location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn ProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (self.ProgramUniform4iv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform4f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { (self.ProgramUniform4f.f)(program, location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn ProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (self.ProgramUniform4fv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform4d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) { (self.ProgramUniform4d.f)(program, location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn ProgramUniform4dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (self.ProgramUniform4dv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniform4ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { (self.ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) }
	#[inline(always)] unsafe fn ProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (self.ProgramUniform4uiv.f)(program, location, count, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (self.ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix2x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix3x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ProgramUniformMatrix4x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (self.ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) }
	#[inline(always)] unsafe fn ValidateProgramPipeline(&self, pipeline: GLuint) { (self.ValidateProgramPipeline.f)(pipeline) }
	#[inline(always)] unsafe fn GetProgramPipelineInfoLog(&self, pipeline: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar) { (self.GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
	
	// Core Extension: ARB_vertex_attrib_64bit
	#[inline(always)] unsafe fn VertexAttribL1d(&self, index: GLuint, x: GLdouble) { (self.VertexAttribL1d.f)(index, x) }
	#[inline(always)] unsafe fn VertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble) { (self.VertexAttribL2d.f)(index, x, y) }
	#[inline(always)] unsafe fn VertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { (self.VertexAttribL3d.f)(index, x, y, z) }
	#[inline(always)] unsafe fn VertexAttribL4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { (self.VertexAttribL4d.f)(index, x, y, z, w) }
	#[inline(always)] unsafe fn VertexAttribL1dv(&self, index: GLuint, v: *GLdouble) { (self.VertexAttribL1dv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribL2dv(&self, index: GLuint, v: *GLdouble) { (self.VertexAttribL2dv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribL3dv(&self, index: GLuint, v: *GLdouble) { (self.VertexAttribL3dv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribL4dv(&self, index: GLuint, v: *GLdouble) { (self.VertexAttribL4dv.f)(index, v) }
	#[inline(always)] unsafe fn VertexAttribLPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) { (self.VertexAttribLPointer.f)(index, size, type_, stride, pointer) }
	#[inline(always)] unsafe fn GetVertexAttribLdv(&self, index: GLuint, pname: GLenum, params: *GLdouble) { (self.GetVertexAttribLdv.f)(index, pname, params) }
	
	// Core Extension: ARB_viewport_array
	#[inline(always)] unsafe fn ViewportArrayv(&self, first: GLuint, count: GLsizei, v: *GLfloat) { (self.ViewportArrayv.f)(first, count, v) }
	#[inline(always)] unsafe fn ViewportIndexedf(&self, index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) { (self.ViewportIndexedf.f)(index, x, y, w, h) }
	#[inline(always)] unsafe fn ViewportIndexedfv(&self, index: GLuint, v: *GLfloat) { (self.ViewportIndexedfv.f)(index, v) }
	#[inline(always)] unsafe fn ScissorArrayv(&self, first: GLuint, count: GLsizei, v: *GLint) { (self.ScissorArrayv.f)(first, count, v) }
	#[inline(always)] unsafe fn ScissorIndexed(&self, index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) { (self.ScissorIndexed.f)(index, left, bottom, width, height) }
	#[inline(always)] unsafe fn ScissorIndexedv(&self, index: GLuint, v: *GLint) { (self.ScissorIndexedv.f)(index, v) }
	#[inline(always)] unsafe fn DepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *GLdouble) { (self.DepthRangeArrayv.f)(first, count, v) }
	#[inline(always)] unsafe fn DepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble) { (self.DepthRangeIndexed.f)(index, n, f) }
	#[inline(always)] unsafe fn GetFloati_v(&self, target: GLenum, index: GLuint, data: *GLfloat) { (self.GetFloati_v.f)(target, index, data) }
	#[inline(always)] unsafe fn GetDoublei_v(&self, target: GLenum, index: GLuint, data: *GLdouble) { (self.GetDoublei_v.f)(target, index, data) }
	
	// Core Extension: ARB_base_instance
	#[inline(always)] unsafe fn DrawArraysInstancedBaseInstance(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) { (self.DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) }
	#[inline(always)] unsafe fn DrawElementsInstancedBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, baseinstance: GLuint) { (self.DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) }
	#[inline(always)] unsafe fn DrawElementsInstancedBaseVertexBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) { (self.DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
	
	// Core Extension: ARB_transform_feedback_instanced
	#[inline(always)] unsafe fn DrawTransformFeedbackInstanced(&self, mode: GLenum, id: GLuint, instancecount: GLsizei) { (self.DrawTransformFeedbackInstanced.f)(mode, id, instancecount) }
	#[inline(always)] unsafe fn DrawTransformFeedbackStreamInstanced(&self, mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) { (self.DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) }
	
	// Core Extension: ARB_internalformat_query
	#[inline(always)] unsafe fn GetInternalformativ(&self, target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *GLint) { (self.GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
	
	// Core Extension: ARB_shader_atomic_counters
	#[inline(always)] unsafe fn GetActiveAtomicCounterBufferiv(&self, program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *GLint) { (self.GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) }
	
	// Core Extension: ARB_shader_image_load_store
	#[inline(always)] unsafe fn BindImageTexture(&self, unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) { (self.BindImageTexture.f)(unit, texture, level, layered, layer, access, format) }
	#[inline(always)] unsafe fn MemoryBarrier(&self, barriers: GLbitfield) { (self.MemoryBarrier.f)(barriers) }
	
	// Core Extension: ARB_texture_storage
	#[inline(always)] unsafe fn TexStorage1D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) { (self.TexStorage1D.f)(target, levels, internalformat, width) }
	#[inline(always)] unsafe fn TexStorage2D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { (self.TexStorage2D.f)(target, levels, internalformat, width, height) }
	#[inline(always)] unsafe fn TexStorage3D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) { (self.TexStorage3D.f)(target, levels, internalformat, width, height, depth) }
	#[inline(always)] unsafe fn TextureStorage1DEXT(&self, texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) { (self.TextureStorage1DEXT.f)(texture, target, levels, internalformat, width) }
	#[inline(always)] unsafe fn TextureStorage2DEXT(&self, texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { (self.TextureStorage2DEXT.f)(texture, target, levels, internalformat, width, height) }
	#[inline(always)] unsafe fn TextureStorage3DEXT(&self, texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) { (self.TextureStorage3DEXT.f)(texture, target, levels, internalformat, width, height, depth) }
	
	// Core Extension: KHR_debug
	#[inline(always)] unsafe fn DebugMessageControl(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *GLuint, enabled: GLboolean) { (self.DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
	#[inline(always)] unsafe fn DebugMessageInsert(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *GLchar) { (self.DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
	#[inline(always)] unsafe fn DebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *GLvoid) { (self.DebugMessageCallback.f)(callback, userParam) }
	#[inline(always)] unsafe fn GetDebugMessageLog(&self, count: GLuint, bufsize: GLsizei, sources: *GLenum, types: *GLenum, ids: *GLuint, severities: *GLenum, lengths: *GLsizei, messageLog: *GLchar) -> GLuint { (self.GetDebugMessageLog.f)(count, bufsize, sources, types, ids, severities, lengths, messageLog) }
	#[inline(always)] unsafe fn PushDebugGroup(&self, source: GLenum, id: GLuint, length: GLsizei, message: *GLchar) { (self.PushDebugGroup.f)(source, id, length, message) }
	#[inline(always)] unsafe fn PopDebugGroup(&self) { (self.PopDebugGroup.f)() }
	#[inline(always)] unsafe fn ObjectLabel(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *GLchar) { (self.ObjectLabel.f)(identifier, name, length, label) }
	#[inline(always)] unsafe fn GetObjectLabel(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *GLsizei, label: *GLchar) { (self.GetObjectLabel.f)(identifier, name, bufSize, length, label) }
	#[inline(always)] unsafe fn ObjectPtrLabel(&self, ptr: *GLvoid, length: GLsizei, label: *GLchar) { (self.ObjectPtrLabel.f)(ptr, length, label) }
	#[inline(always)] unsafe fn GetObjectPtrLabel(&self, ptr: *GLvoid, bufSize: GLsizei, length: *GLsizei, label: *GLchar) { (self.GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
	
	// Core Extension: ARB_clear_buffer_object
	#[inline(always)] unsafe fn ClearBufferData(&self, target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *GLvoid) { (self.ClearBufferData.f)(target, internalformat, format, type_, data) }
	#[inline(always)] unsafe fn ClearBufferSubData(&self, target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *GLvoid) { (self.ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) }
	#[inline(always)] unsafe fn ClearNamedBufferDataEXT(&self, buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *GLvoid) { (self.ClearNamedBufferDataEXT.f)(buffer, internalformat, format, type_, data) }
	#[inline(always)] unsafe fn ClearNamedBufferSubDataEXT(&self, buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, offset: GLsizeiptr, size: GLsizeiptr, data: *GLvoid) { (self.ClearNamedBufferSubDataEXT.f)(buffer, internalformat, format, type_, offset, size, data) }
	
	// Core Extension: ARB_compute_shader
	#[inline(always)] unsafe fn DispatchCompute(&self, num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) { (self.DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) }
	#[inline(always)] unsafe fn DispatchComputeIndirect(&self, indirect: GLintptr) { (self.DispatchComputeIndirect.f)(indirect) }
	
	// Core Extension: ARB_copy_image
	#[inline(always)] unsafe fn CopyImageSubData(&self, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) { (self.CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
	
	// Core Extension: ARB_framebuffer_no_attachments
	#[inline(always)] unsafe fn FramebufferParameteri(&self, target: GLenum, pname: GLenum, param: GLint) { (self.FramebufferParameteri.f)(target, pname, param) }
	#[inline(always)] unsafe fn GetFramebufferParameteriv(&self, target: GLenum, pname: GLenum, params: *GLint) { (self.GetFramebufferParameteriv.f)(target, pname, params) }
	#[inline(always)] unsafe fn NamedFramebufferParameteriEXT(&self, framebuffer: GLuint, pname: GLenum, param: GLint) { (self.NamedFramebufferParameteriEXT.f)(framebuffer, pname, param) }
	#[inline(always)] unsafe fn GetNamedFramebufferParameterivEXT(&self, framebuffer: GLuint, pname: GLenum, params: *GLint) { (self.GetNamedFramebufferParameterivEXT.f)(framebuffer, pname, params) }
	
	// Core Extension: ARB_internalformat_query2
	#[inline(always)] unsafe fn GetInternalformati64v(&self, target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *GLint64) { (self.GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) }
	
	// Core Extension: ARB_invalidate_subdata
	#[inline(always)] unsafe fn InvalidateTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) { (self.InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
	#[inline(always)] unsafe fn InvalidateTexImage(&self, texture: GLuint, level: GLint) { (self.InvalidateTexImage.f)(texture, level) }
	#[inline(always)] unsafe fn InvalidateBufferSubData(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) { (self.InvalidateBufferSubData.f)(buffer, offset, length) }
	#[inline(always)] unsafe fn InvalidateBufferData(&self, buffer: GLuint) { (self.InvalidateBufferData.f)(buffer) }
	#[inline(always)] unsafe fn InvalidateFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *GLenum) { (self.InvalidateFramebuffer.f)(target, numAttachments, attachments) }
	#[inline(always)] unsafe fn InvalidateSubFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (self.InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
	
	// Core Extension: ARB_multi_draw_indirect
	#[inline(always)] unsafe fn MultiDrawArraysIndirect(&self, mode: GLenum, indirect: *GLvoid, drawcount: GLsizei, stride: GLsizei) { (self.MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) }
	#[inline(always)] unsafe fn MultiDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *GLvoid, drawcount: GLsizei, stride: GLsizei) { (self.MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) }
	
	// Core Extension: ARB_program_interface_query
	#[inline(always)] unsafe fn GetProgramInterfaceiv(&self, program: GLuint, programInterface: GLenum, pname: GLenum, params: *GLint) { (self.GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
	#[inline(always)] unsafe fn GetProgramResourceIndex(&self, program: GLuint, programInterface: GLenum, name: *GLchar) -> GLuint { (self.GetProgramResourceIndex.f)(program, programInterface, name) }
	#[inline(always)] unsafe fn GetProgramResourceName(&self, program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *GLsizei, name: *GLchar) { (self.GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
	#[inline(always)] unsafe fn GetProgramResourceiv(&self, program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *GLenum, bufSize: GLsizei, length: *GLsizei, params: *GLint) { (self.GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
	#[inline(always)] unsafe fn GetProgramResourceLocation(&self, program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint { (self.GetProgramResourceLocation.f)(program, programInterface, name) }
	#[inline(always)] unsafe fn GetProgramResourceLocationIndex(&self, program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint { (self.GetProgramResourceLocationIndex.f)(program, programInterface, name) }
	
	// Core Extension: ARB_shader_storage_buffer_object
	#[inline(always)] unsafe fn ShaderStorageBlockBinding(&self, program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) { (self.ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) }
	
	// Core Extension: ARB_texture_buffer_range
	#[inline(always)] unsafe fn TexBufferRange(&self, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { (self.TexBufferRange.f)(target, internalformat, buffer, offset, size) }
	#[inline(always)] unsafe fn TextureBufferRangeEXT(&self, texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { (self.TextureBufferRangeEXT.f)(texture, target, internalformat, buffer, offset, size) }
	
	// Core Extension: ARB_texture_storage_multisample
	#[inline(always)] unsafe fn TexStorage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { (self.TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
	#[inline(always)] unsafe fn TexStorage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { (self.TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
	#[inline(always)] unsafe fn TextureStorage2DMultisampleEXT(&self, texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { (self.TextureStorage2DMultisampleEXT.f)(texture, target, samples, internalformat, width, height, fixedsamplelocations) }
	#[inline(always)] unsafe fn TextureStorage3DMultisampleEXT(&self, texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { (self.TextureStorage3DMultisampleEXT.f)(texture, target, samples, internalformat, width, height, depth, fixedsamplelocations) }
	
	// Core Extension: ARB_texture_view
	#[inline(always)] unsafe fn TextureView(&self, texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) { (self.TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
	
	// Core Extension: ARB_vertex_attrib_binding
	#[inline(always)] unsafe fn BindVertexBuffer(&self, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) { (self.BindVertexBuffer.f)(bindingindex, buffer, offset, stride) }
	#[inline(always)] unsafe fn VertexAttribFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) { (self.VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) }
	#[inline(always)] unsafe fn VertexAttribIFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { (self.VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) }
	#[inline(always)] unsafe fn VertexAttribLFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { (self.VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) }
	#[inline(always)] unsafe fn VertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint) { (self.VertexAttribBinding.f)(attribindex, bindingindex) }
	#[inline(always)] unsafe fn VertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint) { (self.VertexBindingDivisor.f)(bindingindex, divisor) }
	#[inline(always)] unsafe fn VertexArrayBindVertexBufferEXT(&self, vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) { (self.VertexArrayBindVertexBufferEXT.f)(vaobj, bindingindex, buffer, offset, stride) }
	#[inline(always)] unsafe fn VertexArrayVertexAttribFormatEXT(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) { (self.VertexArrayVertexAttribFormatEXT.f)(vaobj, attribindex, size, type_, normalized, relativeoffset) }
	#[inline(always)] unsafe fn VertexArrayVertexAttribIFormatEXT(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { (self.VertexArrayVertexAttribIFormatEXT.f)(vaobj, attribindex, size, type_, relativeoffset) }
	#[inline(always)] unsafe fn VertexArrayVertexAttribLFormatEXT(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { (self.VertexArrayVertexAttribLFormatEXT.f)(vaobj, attribindex, size, type_, relativeoffset) }
	#[inline(always)] unsafe fn VertexArrayVertexAttribBindingEXT(&self, vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) { (self.VertexArrayVertexAttribBindingEXT.f)(vaobj, attribindex, bindingindex) }
	#[inline(always)] unsafe fn VertexArrayVertexBindingDivisorEXT(&self, vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) { (self.VertexArrayVertexBindingDivisorEXT.f)(vaobj, bindingindex, divisor) }
	
}
