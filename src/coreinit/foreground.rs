//! Foreground
//!
//! Releasing the foreground when switching to another process.

use crate::ffi;

unsafe extern "C" {
    /// OSEnableForegroundExit
    #[doc(alias = "OSEnableForegroundExit")]
    #[link_name = "OSEnableForegroundExit"]
    pub unsafe fn enable_exit();

    /// OSReleaseForeground
    #[doc(alias = "OSReleaseForeground")]
    #[link_name = "OSReleaseForeground"]
    pub unsafe fn release();

    /// OSSavesDone_ReadyToRelease
    ///
    /// Signals that saving is done and the foreground can be released.
    #[doc(alias = "OSSavesDone_ReadyToRelease")]
    #[link_name = "OSSavesDone_ReadyToRelease"]
    pub unsafe fn ready_to_release();

    /// OSGetForegroundBucketFreeArea
    ///
    /// Free part of the foreground bucket the application may use.
    #[doc(alias = "OSGetForegroundBucketFreeArea")]
    #[link_name = "OSGetForegroundBucketFreeArea"]
    pub unsafe fn get_bucket_free_area(area: *mut *mut ffi::c_void, size: *mut u32) -> ffi::c_bool;
}
