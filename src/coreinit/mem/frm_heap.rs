//! Frame heaps
//!
//! Stack-like heaps allocating from both ends, freed all at once or back to a recorded state.

use super::{HeapFlags, HeapHandle};
use crate::ffi;

/// MEMFrmHeapFreeMode
#[doc(alias = "MEMFrmHeapFreeMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreeMode(pub i32);

impl FreeMode {
    /// Frees the blocks allocated from the head.
    pub const HEAD: Self = Self(1 << 0);
    /// Frees the blocks allocated from the tail.
    pub const TAIL: Self = Self(1 << 1);
    pub const ALL: Self = Self(0b11);
}

impl core::ops::BitOr for FreeMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MEMFrmHeapState
#[doc(alias = "MEMFrmHeapState", alias = "MEMiFrmHeapState")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrmHeapState {
    pub tag: u32,
    pub head: *mut ffi::c_void,
    pub tail: *mut ffi::c_void,
    pub previous: *mut FrmHeapState,
}

unsafe extern "C" {
    /// MEMCreateFrmHeapEx
    #[doc(alias = "MEMCreateFrmHeapEx")]
    #[link_name = "MEMCreateFrmHeapEx"]
    pub unsafe fn create_ex(start: *mut ffi::c_void, size: u32, flags: HeapFlags) -> HeapHandle;

    /// MEMDestroyFrmHeap
    ///
    /// Returns the start of the heap memory.
    #[doc(alias = "MEMDestroyFrmHeap")]
    #[link_name = "MEMDestroyFrmHeap"]
    pub unsafe fn destroy(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMAllocFromFrmHeapEx
    ///
    /// A negative `alignment` allocates from the tail.
    #[doc(alias = "MEMAllocFromFrmHeapEx")]
    #[link_name = "MEMAllocFromFrmHeapEx"]
    pub unsafe fn alloc_ex(heap: HeapHandle, size: u32, alignment: i32) -> *mut ffi::c_void;

    /// MEMFreeToFrmHeap
    #[doc(alias = "MEMFreeToFrmHeap")]
    #[link_name = "MEMFreeToFrmHeap"]
    pub unsafe fn free(heap: HeapHandle, mode: FreeMode);

    /// MEMRecordStateForFrmHeap
    ///
    /// Records the current allocation state under `tag`, see [free_by_state].
    #[doc(alias = "MEMRecordStateForFrmHeap")]
    #[link_name = "MEMRecordStateForFrmHeap"]
    pub unsafe fn record_state(heap: HeapHandle, tag: u32) -> ffi::c_bool;

    /// MEMFreeByStateToFrmHeap
    ///
    /// Frees back to the state recorded under `tag`, or the last state if `tag` is 0.
    #[doc(alias = "MEMFreeByStateToFrmHeap")]
    #[link_name = "MEMFreeByStateToFrmHeap"]
    pub unsafe fn free_by_state(heap: HeapHandle, tag: u32) -> ffi::c_bool;

    /// MEMAdjustFrmHeap
    ///
    /// Shrinks the heap to its used part. Returns the new size.
    #[doc(alias = "MEMAdjustFrmHeap")]
    #[link_name = "MEMAdjustFrmHeap"]
    pub unsafe fn adjust(heap: HeapHandle) -> u32;

    /// MEMResizeForMBlockFrmHeap
    ///
    /// Only the last block allocated from the head can be resized. Returns the new size, or 0 on failure.
    #[doc(alias = "MEMResizeForMBlockFrmHeap")]
    #[link_name = "MEMResizeForMBlockFrmHeap"]
    pub unsafe fn resize_for_mblock(heap: HeapHandle, block: *mut ffi::c_void, size: u32) -> u32;

    /// MEMGetAllocatableSizeForFrmHeapEx
    #[doc(alias = "MEMGetAllocatableSizeForFrmHeapEx")]
    #[link_name = "MEMGetAllocatableSizeForFrmHeapEx"]
    pub unsafe fn get_allocatable_size_ex(heap: HeapHandle, alignment: i32) -> u32;

    /// MEMiGetFreeStartForFrmHeap
    #[doc(alias = "MEMiGetFreeStartForFrmHeap")]
    #[link_name = "MEMiGetFreeStartForFrmHeap"]
    pub unsafe fn get_free_start(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMiGetFreeEndForFrmHeap
    #[doc(alias = "MEMiGetFreeEndForFrmHeap")]
    #[link_name = "MEMiGetFreeEndForFrmHeap"]
    pub unsafe fn get_free_end(heap: HeapHandle) -> *mut ffi::c_void;
}
