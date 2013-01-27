use libc::*;

/* Base GL types */
pub type GLenum         = c_uint;
pub type GLboolean      = c_uchar;
pub type GLbitfield     = c_uint;
pub type GLbyte         = c_schar;
pub type GLshort        = c_short;
pub type GLint          = c_int;
pub type GLsizei        = c_int;
pub type GLubyte        = c_uchar;
pub type GLushort       = c_ushort;
pub type GLuint         = c_uint;
pub type GLhalf         = c_ushort;
pub type GLfloat        = c_float;
pub type GLclampf       = c_float;
pub type GLdouble       = c_double;
pub type GLclampd       = c_double;
pub type GLvoid         = c_void;

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

pub struct __GLsync {}
pub type GLsync = *__GLsync;

/* These incomplete types let us declare types compatible with OpenCL's cl_context and cl_event */
pub struct _cl_context {}
pub struct _cl_event {}

pub type GLDEBUGPROCARB = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);
pub type GLDEBUGPROCAMD = extern "C" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);
pub type GLDEBUGPROC = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *GLvoid);

pub type GLvdpauSurfaceNV = GLintptr;