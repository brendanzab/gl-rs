// From WinNT.h

pub type CHAR = c_char;
pub type HANDLE = PVOID;
pub type LONG = c_long;
pub type LPCSTR = *const c_char;
pub type VOID = ();
// #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name
pub type HPBUFFERARB = *const c_void;
pub type HPBUFFEREXT = *const c_void;
pub type HVIDEOOUTPUTDEVICENV = *const c_void;
pub type HPVIDEODEV = *const c_void;
pub type HPGPUNV = *const c_void;
pub type HGPUNV = *const c_void;
pub type HVIDEOINPUTDEVICENV = *const c_void;

// From Windef.h

pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type COLORREF = DWORD;
pub type FLOAT = c_float;
pub type HDC = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HGLRC = *const c_void;
pub type INT = c_int;
pub type PVOID = *const c_void;
pub type LPVOID = *const c_void;
pub enum __PROC_fn {}
pub type PROC = *mut __PROC_fn;

#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

pub type UINT = c_uint;
pub type USHORT = c_ushort;
pub type WORD = c_ushort;

// From BaseTsd.h

pub type INT32 = i32;
pub type INT64 = i64;

// From IntSafe.h

pub type DWORD = c_ulong;

// From Wingdi.h

#[repr(C)]
pub struct POINTFLOAT {
    pub x: FLOAT,
    pub y: FLOAT,
}

#[repr(C)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: FLOAT,
    pub gmfBlackBoxY: FLOAT,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: FLOAT,
    pub gmfCellIncY: FLOAT,
}
pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;

#[repr(C)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub crTransparent: COLORREF,
}

#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}

#[repr(C)]
pub struct _GPU_DEVICE {
    cb: DWORD,
    DeviceName: [CHAR; 32],
    DeviceString: [CHAR; 128],
    Flags: DWORD,
    rcVirtualScreen: RECT,
}

pub struct GPU_DEVICE(_GPU_DEVICE);
pub struct PGPU_DEVICE(*const _GPU_DEVICE);
