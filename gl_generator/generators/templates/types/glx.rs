pub type XID = c_ulong;
pub type Bool = c_int; // Not sure if this is correct...
pub enum Display {}

pub type Font = XID;
pub type Pixmap = XID;
pub enum Visual {} // TODO: not sure
pub type VisualID = c_ulong; // TODO: not sure
pub type Window = XID;
pub type GLXFBConfigID = XID;
pub type GLXFBConfig = *const c_void;
pub type GLXContextID = XID;
pub type GLXContext = *const c_void;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXWindow = XID;
pub type GLXPbuffer = XID;
pub enum __GLXextFuncPtr_fn {}
pub type __GLXextFuncPtr = *mut __GLXextFuncPtr_fn;
pub type GLXVideoCaptureDeviceNV = XID;
pub type GLXVideoDeviceNV = c_int;
pub type GLXVideoSourceSGIX = XID;
pub type GLXFBConfigIDSGIX = XID;
pub type GLXFBConfigSGIX = *const c_void;
pub type GLXPbufferSGIX = XID;

#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}

#[repr(C)]
pub struct GLXPbufferClobberEvent {
    pub event_type: c_int, // GLX_DAMAGED or GLX_SAVED
    pub draw_type: c_int, // GLX_WINDOW or GLX_PBUFFER
    pub serial: c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came for SendEvent request
    pub display: *const Display, // display the event was read from
    pub drawable: GLXDrawable, // XID of Drawable
    pub buffer_mask: c_uint, // mask indicating which buffers are affected
    pub aux_buffer: c_uint, // which aux buffer was affected
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int, // if nonzero, at least this many more
}

#[repr(C)]
pub struct GLXBufferSwapComplete {
    pub type_: c_int,
    pub serial: c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came from a SendEvent request
    pub display: *const Display, // Display the event was read from
    pub drawable: GLXDrawable, // drawable on which event was requested in event mask
    pub event_type: c_int,
    pub ust: i64,
    pub msc: i64,
    pub sbc: i64,
}

// typedef union __GLXEvent {
//     GLXPbufferClobberEvent glxpbufferclobber;
//     GLXBufferSwapComplete glxbufferswapcomplete;
//     long pad[24];
// }

#[repr(C)]
pub struct GLXBufferClobberEventSGIX {
    pub type_: c_int,
    pub serial: c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came for SendEvent request
    pub display: *const Display, // display the event was read from
    pub drawable: GLXDrawable, // i.d. of Drawable
    pub event_type: c_int, // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX
    pub draw_type: c_int, // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX
    pub mask: c_uint, // mask indicating which buffers are affected
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int, // if nonzero, at least this many more
}

#[repr(C)]
pub struct GLXHyperpipeNetworkSGIX {
    pub pipeName: [c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub networkId: c_int,
}

#[repr(C)]
pub struct GLXHyperpipeConfigSGIX {
    pub pipeName: [c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub channel: c_int,
    pub participationType: c_uint,
    pub timeSlice: c_int,
}

#[repr(C)]
pub struct GLXPipeRect {
    pub pipeName: [c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub srcXOrigin: c_int,
    pub srcYOrigin: c_int,
    pub srcWidth: c_int,
    pub srcHeight: c_int,
    pub destXOrigin: c_int,
    pub destYOrigin: c_int,
    pub destWidth: c_int,
    pub destHeight: c_int,
}

#[repr(C)]
pub struct GLXPipeRectLimits {
    pub pipeName: [c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub XOrigin: c_int,
    pub YOrigin: c_int,
    pub maxHeight: c_int,
    pub maxWidth: c_int,
}
