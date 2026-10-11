//! Internal

use crate::ffi;

unsafe extern "C" {
    /// __os_snprintf
    #[doc(alias = "__os_snprintf")]
    #[link_name = "__os_snprintf"]
    pub unsafe fn snprintf(
        buffer: *mut ffi::c_char,
        size: usize,
        format: *const ffi::c_char,
        ...
    ) -> ffi::c_int;
}
