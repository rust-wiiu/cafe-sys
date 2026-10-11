//! GX2 memory

use super::enums::InvalidateMode;
use crate::ffi;

/// GX2DefaultAllocateFunc
///
/// `user_arg` is the resource flags of the allocation.
#[doc(alias = "GX2DefaultAllocateFunc")]
pub type DefaultAllocateFn =
    Option<unsafe extern "C" fn(user_arg: u32, size: u32, alignment: u32) -> *mut ffi::c_void>;

/// GX2DefaultFreeFunc
#[doc(alias = "GX2DefaultFreeFunc")]
pub type DefaultFreeFn = Option<unsafe extern "C" fn(user_arg: u32, ptr: *mut ffi::c_void)>;

unsafe extern "C" {
    /// GX2Invalidate
    ///
    /// Invalidates CPU and GPU caches of `buffer` for the given uses.
    #[doc(alias = "GX2Invalidate")]
    #[link_name = "GX2Invalidate"]
    pub unsafe fn invalidate(mode: InvalidateMode, buffer: *mut ffi::c_void, size: u32);

    /// GX2GetDefaultAllocator
    #[doc(alias = "GX2GetDefaultAllocator")]
    #[link_name = "GX2GetDefaultAllocator"]
    pub unsafe fn get_default_allocator(alloc: *mut DefaultAllocateFn, free: *mut DefaultFreeFn);

    /// GX2SetDefaultAllocator
    ///
    /// Allocator used for internal buffers such as the geometry shader ring buffers.
    #[doc(alias = "GX2SetDefaultAllocator")]
    #[link_name = "GX2SetDefaultAllocator"]
    pub unsafe fn set_default_allocator(alloc: DefaultAllocateFn, free: DefaultFreeFn);

    /// GX2NotifyMemAlloc
    ///
    /// Tells debugging tools about memory used by the GPU.
    #[doc(alias = "GX2NotifyMemAlloc")]
    #[link_name = "GX2NotifyMemAlloc"]
    pub unsafe fn notify_mem_alloc(ptr: *mut ffi::c_void, size: u32, alignment: u32);

    /// GX2NotifyMemFree
    #[doc(alias = "GX2NotifyMemFree")]
    #[link_name = "GX2NotifyMemFree"]
    pub unsafe fn notify_mem_free(ptr: *mut ffi::c_void);
}
