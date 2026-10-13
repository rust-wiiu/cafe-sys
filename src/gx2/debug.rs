//! GX2 debugging

use super::surface::Surface;
use crate::ffi;

/// GX2DebugMode
#[doc(alias = "GX2DebugMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugMode(pub u32);

impl DebugMode {
    pub const NONE: Self = Self(0);
    pub const FLUSH_PER_DRAW: Self = Self(1);
    pub const DONE_PER_FLUSH: Self = Self(2);
}

impl core::ops::BitOr for DebugMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[doc(alias = "GX2DebugCaptureStartFlags", alias = "GX2DebugCaptureEndFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugCaptureOptions(pub u32);

impl DebugCaptureOptions {
    pub const DEFAULT: Self = Self(0);
    /// Skips the flush or draw done when starting or ending the capture.
    pub const NO_FLUSH: Self = Self(1);
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugTagUserStringType {
    #[default]
    Indent = 0,
    Undent = 1,
    Comment = 2,
    Bookmark = 3,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogAttrib(pub u32);

impl LogAttrib {
    pub const ENABLE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerifyLevel(pub u32);

impl VerifyLevel {
    pub const NONE: Self = Self(0);
    pub const SEVERE: Self = Self(1);
    pub const MEDIUM: Self = Self(2);
    pub const ALL: Self = Self(3);
}

/// `format` is a printf format string for the arguments that follow.
pub type VerifyCallback = Option<
    unsafe extern "C" fn(
        file: *const ffi::c_char,
        level: VerifyLevel,
        line: u32,
        format: *const ffi::c_char,
        ...
    ),
>;

/// GX2DebugCaptureInterfaceVersion
#[doc(alias = "GX2_DEBUG_CAPTURE_INTERFACE_VERSION")]
pub const DEBUG_CAPTURE_INTERFACE_VERSION: u32 = 1;

/// GX2DebugCaptureInterface
#[doc(alias = "GX2DebugCaptureInterface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugCaptureInterface {
    /// [DEBUG_CAPTURE_INTERFACE_VERSION].
    pub version: u32,
    pub on_shutdown: Option<unsafe extern "C" fn()>,
    pub set_allocator: Option<unsafe extern "C" fn()>,
    pub on_capture_start: Option<unsafe extern "C" fn(filename: *const ffi::c_char)>,
    pub on_capture_end: Option<unsafe extern "C" fn()>,
    pub is_capture_enabled: Option<unsafe extern "C" fn() -> ffi::c_bool>,
    pub on_alloc: Option<unsafe extern "C" fn(ptr: *mut ffi::c_void, size: u32, align: u32)>,
    pub on_free: Option<unsafe extern "C" fn(ptr: *mut ffi::c_void)>,
    pub on_unknown_memory: Option<unsafe extern "C" fn(ptr: *mut ffi::c_void, size: u32)>,
    pub set_output_surface: Option<unsafe extern "C" fn(surface: *mut Surface)>,
    pub on_swap_scan_buffers: Option<unsafe extern "C" fn(surface: *mut Surface)>,
    pub submit_to_ring: Option<
        unsafe extern "C" fn(
            unk1: *mut ffi::c_void,
            unk2: u32,
            unk3: *mut u32,
            submit_timestamp: *mut u64,
        ),
    >,
}

unsafe extern "C" {
    /// _GX2DebugSetCaptureInterface
    #[doc(alias = "_GX2DebugSetCaptureInterface")]
    #[link_name = "_GX2DebugSetCaptureInterface"]
    pub unsafe fn set_capture_interface(interface: *const DebugCaptureInterface) -> ffi::c_bool;

    /// GX2DebugCaptureStart
    ///
    /// Starts a debug capture. `filename` is passed to the capture interface.
    #[doc(alias = "GX2DebugCaptureStart")]
    #[link_name = "GX2DebugCaptureStart"]
    pub unsafe fn capture_start(filename: *const ffi::c_char, options: DebugCaptureOptions);

    /// GX2DebugCaptureEnd
    #[doc(alias = "GX2DebugCaptureEnd")]
    #[link_name = "GX2DebugCaptureEnd"]
    pub unsafe fn capture_end(options: DebugCaptureOptions);

    /// GX2DebugCaptureFrame
    ///
    /// Captures the next frame, from one swap to the next.
    #[doc(alias = "GX2DebugCaptureFrame")]
    #[link_name = "GX2DebugCaptureFrame"]
    pub unsafe fn capture_frame(filename: *const ffi::c_char);

    /// GX2DebugCaptureFrames
    #[doc(alias = "GX2DebugCaptureFrames")]
    #[link_name = "GX2DebugCaptureFrames"]
    #[deprecated(note = "Use capture_frame instead")]
    pub unsafe fn capture_frames(filename: *const ffi::c_char, num_frames: u32);

    /// GX2DebugTagUserString
    ///
    /// Inserts a printf formatted tag into the command stream for debugging tools. Only while a debug capture is active.
    #[doc(alias = "GX2DebugTagUserString")]
    #[link_name = "GX2DebugTagUserString"]
    pub unsafe fn tag_user_string(kind: DebugTagUserStringType, format: *const ffi::c_char, ...);

    /// GX2DebugTagUserStringVA
    ///
    /// Same as [tag_user_string], `args` points to a `va_list`.
    #[doc(alias = "GX2DebugTagUserStringVA")]
    #[link_name = "GX2DebugTagUserStringVA"]
    pub unsafe fn tag_user_string_va(
        kind: DebugTagUserStringType,
        format: *const ffi::c_char,
        args: *mut ffi::c_void,
    );

    /// GX2SetDebugMode
    #[doc(alias = "GX2SetDebugMode")]
    #[link_name = "GX2SetDebugMode"]
    pub unsafe fn set_debug_mode(mode: DebugMode);

    /// GX2SetVerifyLevel
    #[doc(alias = "GX2SetVerifyLevel")]
    #[link_name = "GX2SetVerifyLevel"]
    pub unsafe fn set_verify_level(level: VerifyLevel);

    /// GX2SetVerifyCallback
    ///
    /// Returns the previous callback. Has no effect and returns null in the release gx2.rpl.
    #[doc(alias = "GX2SetVerifyCallback")]
    #[link_name = "GX2SetVerifyCallback"]
    pub unsafe fn set_verify_callback(callback: VerifyCallback) -> VerifyCallback;

    /// GX2PrintGPUStatus
    #[doc(alias = "GX2PrintGPUStatus")]
    #[link_name = "GX2PrintGPUStatus"]
    pub unsafe fn print_gpu_status();

    /// GX2LogSetMisc
    #[doc(alias = "GX2LogSetMisc")]
    #[link_name = "GX2LogSetMisc"]
    pub unsafe fn log_set_misc(attrib: LogAttrib, value: u32);
}
