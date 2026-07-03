use crate::ffi::*;

unsafe extern "C" {
    #[doc(alias = "DCFlushRange")]
    #[link_name = "DCFlushRange"]
    pub unsafe fn flush_range(addr: *mut c_void, size: u32);

    #[doc(alias = "ICInvalidateRange")]
    #[link_name = "ICInvalidateRange"]
    pub unsafe fn invalidate_range(addr: *mut c_void, size: u32);
}
