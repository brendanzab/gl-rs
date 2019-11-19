// platform-specific aliases are unknown
// IMPORTANT: these are aliases to the same level of the bindings
// the values must be defined by the user
pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;
pub type khronos_uint64_t = super::khronos_uint64_t;
pub type khronos_ssize_t = super::khronos_ssize_t;
pub type EGLNativeDisplayType = super::EGLNativeDisplayType;
pub type EGLNativePixmapType = super::EGLNativePixmapType;
pub type EGLNativeWindowType = super::EGLNativeWindowType;
pub type EGLint = super::EGLint;
pub type NativeDisplayType = super::NativeDisplayType;
pub type NativePixmapType = super::NativePixmapType;
pub type NativeWindowType = super::NativeWindowType;

// EGL alises
pub type Bool = EGLBoolean; // TODO: not sure
pub type EGLBoolean = c_uint;
pub type EGLenum = c_uint;
pub type EGLAttribKHR = isize;
pub type EGLAttrib = isize;
pub type EGLConfig = *const c_void;
pub type EGLContext = *const c_void;
pub type EGLDeviceEXT = *const c_void;
pub type EGLDisplay = *const c_void;
pub type EGLSurface = *const c_void;
pub type EGLClientBuffer = *const c_void;
pub enum __eglMustCastToProperFunctionPointerType_fn {}
pub type __eglMustCastToProperFunctionPointerType =
    *mut __eglMustCastToProperFunctionPointerType_fn;
pub type EGLImageKHR = *const c_void;
pub type EGLImage = *const c_void;
pub type EGLOutputLayerEXT = *const c_void;
pub type EGLOutputPortEXT = *const c_void;
pub type EGLSyncKHR = *const c_void;
pub type EGLSync = *const c_void;
pub type EGLTimeKHR = khronos_utime_nanoseconds_t;
pub type EGLTime = khronos_utime_nanoseconds_t;
pub type EGLSyncNV = *const c_void;
pub type EGLTimeNV = khronos_utime_nanoseconds_t;
pub type EGLuint64NV = khronos_utime_nanoseconds_t;
pub type EGLStreamKHR = *const c_void;
pub type EGLuint64KHR = khronos_uint64_t;
pub type EGLNativeFileDescriptorKHR = c_int;
pub type EGLsizeiANDROID = khronos_ssize_t;
pub type EGLSetBlobFuncANDROID = extern "system" fn(*const c_void,
                                                    EGLsizeiANDROID,
                                                    *const c_void,
                                                    EGLsizeiANDROID)
                                                    -> ();
pub type EGLGetBlobFuncANDROID = extern "system" fn(*const c_void,
                                                    EGLsizeiANDROID,
                                                    *mut c_void,
                                                    EGLsizeiANDROID)
                                                    -> EGLsizeiANDROID;

#[repr(C)]
pub struct EGLClientPixmapHI {
    pData: *const c_void,
    iWidth: EGLint,
    iHeight: EGLint,
    iStride: EGLint,
}
