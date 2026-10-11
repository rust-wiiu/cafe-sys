//! Copy data

use crate::ffi;

unsafe extern "C" {
    /// __OSClearCopyData
    #[doc(alias = "__OSClearCopyData")]
    #[link_name = "__OSClearCopyData"]
    pub unsafe fn clear();

    /// __OSAppendCopyData
    #[doc(alias = "__OSAppendCopyData")]
    #[link_name = "__OSAppendCopyData"]
    pub unsafe fn append(data: *const ffi::c_void, size: u32) -> ffi::c_bool;

    /// __OSGetCopyDataPtr
    #[doc(alias = "__OSGetCopyDataPtr")]
    #[link_name = "__OSGetCopyDataPtr"]
    pub unsafe fn get_ptr() -> *mut ffi::c_void;

    /// __OSGetCopyDataSize
    #[doc(alias = "__OSGetCopyDataSize")]
    #[link_name = "__OSGetCopyDataSize"]
    pub unsafe fn get_size() -> u32;

    /// __OSResizeCopyData
    #[doc(alias = "__OSResizeCopyData")]
    #[link_name = "__OSResizeCopyData"]
    pub unsafe fn resize(size: u32) -> ffi::c_bool;
}
