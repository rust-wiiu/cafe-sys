//! Clipboard

use crate::ffi;

/// OS_MAX_CLIPBOARD_SIZE
#[doc(alias = "OS_MAX_CLIPBOARD_SIZE")]
pub const MAX_SIZE: u32 = 2048;

unsafe extern "C" {
    /// OSCopyToClipboard
    #[doc(alias = "OSCopyToClipboard")]
    #[link_name = "OSCopyToClipboard"]
    pub unsafe fn copy_to(data: *const ffi::c_void, size: u32) -> ffi::c_bool;

    /// OSCopyFromClipboard
    ///
    /// `size` is the buffer size on input and the copied size on output.
    #[doc(alias = "OSCopyFromClipboard")]
    #[link_name = "OSCopyFromClipboard"]
    pub unsafe fn copy_from(data: *mut ffi::c_void, size: *mut u32) -> ffi::c_bool;
}
