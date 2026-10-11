//! GX2R display lists

use super::{Buffer, ResourceFlags};
use crate::ffi;

unsafe extern "C" {
    /// GX2RBeginDisplayListEx
    ///
    /// Starts recording into `display_list`. Invalidates the buffer unless disabled in `flags`.
    #[doc(alias = "GX2RBeginDisplayListEx")]
    #[link_name = "GX2RBeginDisplayListEx"]
    pub unsafe fn begin_ex(
        display_list: *const Buffer,
        enable_profiling: ffi::c_bool,
        flags: ResourceFlags,
    );

    /// GX2REndDisplayList
    ///
    /// Returns the size of the recorded display list in bytes.
    #[doc(alias = "GX2REndDisplayList")]
    #[link_name = "GX2REndDisplayList"]
    pub unsafe fn end(display_list: *const Buffer) -> u32;

    /// GX2RCallDisplayList
    #[doc(alias = "GX2RCallDisplayList")]
    #[link_name = "GX2RCallDisplayList"]
    pub unsafe fn call(display_list: *const Buffer, size: u32);

    /// GX2RDirectCallDisplayList
    ///
    /// Same as [call], but adds it directly to the graphics ring buffer.
    #[doc(alias = "GX2RDirectCallDisplayList")]
    #[link_name = "GX2RDirectCallDisplayList"]
    pub unsafe fn direct_call(display_list: *const Buffer, size: u32);
}
