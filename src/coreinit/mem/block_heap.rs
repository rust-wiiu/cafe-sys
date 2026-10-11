//! Block heaps
//!
//! Heaps over an address range whose blocks are tracked in separately provided tracking memory, allowing allocation at fixed addresses.

use super::{HeapFlags, HeapHandle, HeapHeader};
use crate::ffi;

/// MEM_BLKHEAP_BYTES_PER_BLOCK_TRACK
#[doc(alias = "MEM_BLKHEAP_BYTES_PER_BLOCK_TRACK")]
pub const BYTES_PER_BLOCK_TRACK: u32 = 20;
/// MEM_BLKHEAP_TRACK_GROUP_OVERHEAD
#[doc(alias = "MEM_BLKHEAP_TRACK_GROUP_OVERHEAD")]
pub const TRACK_GROUP_OVERHEAD: u32 = 16;

/// MEMBlockHeapTracking
#[doc(alias = "MEMBlockHeapTracking")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockHeapTracking {
    _unk: ffi::unknown<8>,
    pub blocks: *mut BlockHeapBlock,
    pub block_count: u32,
}

/// MEMBlockHeapBlock
#[doc(alias = "MEMBlockHeapBlock")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockHeapBlock {
    pub start: *mut ffi::c_void,
    pub end: *mut ffi::c_void,
    pub is_free: ffi::c_bool,
    pub prev: *mut BlockHeapBlock,
    pub next: *mut BlockHeapBlock,
}

/// MEMBlockHeap
#[doc(alias = "MEMBlockHeap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockHeap {
    pub header: HeapHeader,
    pub default_track: BlockHeapTracking,
    pub default_block: BlockHeapBlock,
    pub first_block: *mut BlockHeapBlock,
    pub last_block: *mut BlockHeapBlock,
    pub first_free_block: *mut BlockHeapBlock,
    pub num_free_blocks: u32,
}

unsafe extern "C" {
    /// MEMInitBlockHeap
    ///
    /// Manages `start..end`. `tracking` provides `tracking_size` bytes of tracking memory, [BYTES_PER_BLOCK_TRACK] per block plus [TRACK_GROUP_OVERHEAD].
    #[doc(alias = "MEMInitBlockHeap")]
    #[link_name = "MEMInitBlockHeap"]
    pub unsafe fn init(
        heap: *mut BlockHeap,
        start: *mut ffi::c_void,
        end: *mut ffi::c_void,
        tracking: *mut ffi::c_void,
        tracking_size: u32,
        flags: HeapFlags,
    ) -> HeapHandle;

    /// MEMDestroyBlockHeap
    #[doc(alias = "MEMDestroyBlockHeap")]
    #[link_name = "MEMDestroyBlockHeap"]
    pub unsafe fn destroy(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMAddBlockHeapTracking
    #[doc(alias = "MEMAddBlockHeapTracking")]
    #[link_name = "MEMAddBlockHeapTracking"]
    pub unsafe fn add_tracking(heap: HeapHandle, tracking: *mut ffi::c_void, size: u32) -> i32;

    /// MEMAllocFromBlockHeapAt
    #[doc(alias = "MEMAllocFromBlockHeapAt")]
    #[link_name = "MEMAllocFromBlockHeapAt"]
    pub unsafe fn alloc_at(heap: HeapHandle, addr: *mut ffi::c_void, size: u32)
    -> *mut ffi::c_void;

    /// MEMAllocFromBlockHeapEx
    ///
    /// A negative `alignment` allocates from the end of the heap.
    #[doc(alias = "MEMAllocFromBlockHeapEx")]
    #[link_name = "MEMAllocFromBlockHeapEx"]
    pub unsafe fn alloc_ex(heap: HeapHandle, size: u32, alignment: i32) -> *mut ffi::c_void;

    /// MEMFreeToBlockHeap
    #[doc(alias = "MEMFreeToBlockHeap")]
    #[link_name = "MEMFreeToBlockHeap"]
    pub unsafe fn free(heap: HeapHandle, block: *mut ffi::c_void);

    /// MEMGetAllocatableSizeForBlockHeapEx
    #[doc(alias = "MEMGetAllocatableSizeForBlockHeapEx")]
    #[link_name = "MEMGetAllocatableSizeForBlockHeapEx"]
    pub unsafe fn get_allocatable_size_ex(heap: HeapHandle, alignment: i32) -> u32;

    /// MEMGetTrackingLeftInBlockHeap
    #[doc(alias = "MEMGetTrackingLeftInBlockHeap")]
    #[link_name = "MEMGetTrackingLeftInBlockHeap"]
    pub unsafe fn get_tracking_left(heap: HeapHandle) -> u32;

    /// MEMGetTotalFreeSizeForBlockHeap
    #[doc(alias = "MEMGetTotalFreeSizeForBlockHeap")]
    #[link_name = "MEMGetTotalFreeSizeForBlockHeap"]
    pub unsafe fn get_total_free_size(heap: HeapHandle) -> u32;
}
