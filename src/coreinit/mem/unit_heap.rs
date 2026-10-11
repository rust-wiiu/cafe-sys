//! Unit heaps
//!
//! Heaps of equally sized blocks.

use super::{HeapFlags, HeapHandle};
use crate::ffi;

unsafe extern "C" {
    /// MEMCreateUnitHeapEx
    #[doc(alias = "MEMCreateUnitHeapEx")]
    #[link_name = "MEMCreateUnitHeapEx"]
    pub unsafe fn create_ex(
        start: *mut ffi::c_void,
        size: u32,
        block_size: u32,
        alignment: i32,
        flags: HeapFlags,
    ) -> HeapHandle;

    /// MEMDestroyUnitHeap
    ///
    /// Returns the start of the heap memory.
    #[doc(alias = "MEMDestroyUnitHeap")]
    #[link_name = "MEMDestroyUnitHeap"]
    pub unsafe fn destroy(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMAllocFromUnitHeap
    #[doc(alias = "MEMAllocFromUnitHeap")]
    #[link_name = "MEMAllocFromUnitHeap"]
    pub unsafe fn alloc(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMFreeToUnitHeap
    #[doc(alias = "MEMFreeToUnitHeap")]
    #[link_name = "MEMFreeToUnitHeap"]
    pub unsafe fn free(heap: HeapHandle, block: *mut ffi::c_void);

    /// MEMCountFreeBlockForUnitHeap
    #[doc(alias = "MEMCountFreeBlockForUnitHeap")]
    #[link_name = "MEMCountFreeBlockForUnitHeap"]
    pub unsafe fn count_free_blocks(heap: HeapHandle) -> u32;

    /// MEMCalcHeapSizeForUnitHeap
    ///
    /// Heap size needed for `count` blocks of `block_size` bytes.
    #[doc(alias = "MEMCalcHeapSizeForUnitHeap")]
    #[link_name = "MEMCalcHeapSizeForUnitHeap"]
    pub unsafe fn calc_heap_size(block_size: u32, count: u32, alignment: i32) -> u32;
}
