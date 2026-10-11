//! Screen
//!
//! Simple framebuffer output for text and pixels, and screen capture permission.

use crate::ffi;

/// OSScreenID
#[doc(alias = "OSScreenID")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    #[default]
    Tv = 0,
    Drc = 1,
}

unsafe extern "C" {
    /// OSScreenInit
    #[doc(alias = "OSScreenInit")]
    #[link_name = "OSScreenInit"]
    pub unsafe fn init();

    /// OSScreenShutdown
    #[doc(alias = "OSScreenShutdown")]
    #[link_name = "OSScreenShutdown"]
    pub unsafe fn shutdown();

    /// OSScreenGetBufferSizeEx
    #[doc(alias = "OSScreenGetBufferSizeEx")]
    #[link_name = "OSScreenGetBufferSizeEx"]
    pub unsafe fn get_buffer_size(screen: Screen) -> u32;

    /// OSScreenSetBufferEx
    ///
    /// `buffer` must be 0x100-byte aligned and [get_buffer_size] bytes.
    #[doc(alias = "OSScreenSetBufferEx")]
    #[link_name = "OSScreenSetBufferEx"]
    pub unsafe fn set_buffer(screen: Screen, buffer: *mut ffi::c_void);

    /// OSScreenClearBufferEx
    ///
    /// `color` is RGBA.
    #[doc(alias = "OSScreenClearBufferEx")]
    #[link_name = "OSScreenClearBufferEx"]
    pub unsafe fn clear_buffer(screen: Screen, color: u32);

    /// OSScreenFlipBuffersEx
    #[doc(alias = "OSScreenFlipBuffersEx")]
    #[link_name = "OSScreenFlipBuffersEx"]
    pub unsafe fn flip_buffers(screen: Screen);

    /// OSScreenPutFontEx
    #[doc(alias = "OSScreenPutFontEx")]
    #[link_name = "OSScreenPutFontEx"]
    pub unsafe fn put_font(screen: Screen, column: u32, row: u32, text: *const ffi::c_char);

    /// OSScreenPutPixelEx
    #[doc(alias = "OSScreenPutPixelEx")]
    #[link_name = "OSScreenPutPixelEx"]
    pub unsafe fn put_pixel(screen: Screen, x: u32, y: u32, color: u32);

    /// OSScreenEnableEx
    #[doc(alias = "OSScreenEnableEx")]
    #[link_name = "OSScreenEnableEx"]
    pub unsafe fn enable(screen: Screen, enable: ffi::c_bool);

    /// OSGetScreenCapturePermission
    #[doc(alias = "OSGetScreenCapturePermission")]
    #[link_name = "OSGetScreenCapturePermission"]
    pub unsafe fn get_capture_permission() -> ffi::c_bool;

    /// OSSetScreenCapturePermission
    ///
    /// Returns the previous permission.
    #[doc(alias = "OSSetScreenCapturePermission")]
    #[link_name = "OSSetScreenCapturePermission"]
    pub unsafe fn set_capture_permission(enabled: ffi::c_bool) -> ffi::c_bool;

    /// OSGetScreenCapturePermissionEx
    #[doc(alias = "OSGetScreenCapturePermissionEx")]
    #[link_name = "OSGetScreenCapturePermissionEx"]
    pub unsafe fn get_capture_permission_ex(
        tv: *mut ffi::c_bool,
        drc: *mut ffi::c_bool,
    ) -> ffi::c_bool;

    /// OSSetScreenCapturePermissionEx
    #[doc(alias = "OSSetScreenCapturePermissionEx")]
    #[link_name = "OSSetScreenCapturePermissionEx"]
    pub unsafe fn set_capture_permission_ex(tv: ffi::c_bool, drc: ffi::c_bool) -> ffi::c_bool;
}
