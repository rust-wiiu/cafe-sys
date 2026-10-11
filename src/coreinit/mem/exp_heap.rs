//! Expanded heaps
//!
//! General-purpose heaps with arbitrary block sizes, allocation from either end and group IDs.

use super::{CheckFlags, HeapFlags, HeapHandle};
use crate::ffi;

/// MEMExpHeapMode
#[doc(alias = "MEMExpHeapMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpHeapMode(pub u16);

impl ExpHeapMode {
    /// Uses the first free region that is large enough.
    pub const FIRST_FREE: Self = Self(0);
    /// Uses the free region closest in size.
    pub const NEAREST_SIZE: Self = Self(1);
}

/// MEMExpHeapDirection
#[doc(alias = "MEMExpHeapDirection")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpHeapDirection(pub u16);

impl ExpHeapDirection {
    pub const FROM_TOP: Self = Self(0);
    pub const FROM_BOTTOM: Self = Self(1);
}

/// MEMExpHeapBlockVisitor
#[doc(alias = "MEMExpHeapBlockVisitor")]
pub type BlockVisitor = Option<
    unsafe extern "C" fn(block: *mut ffi::c_void, heap: HeapHandle, context: *mut ffi::c_void),
>;

unsafe extern "C" {
    /// MEMCreateExpHeapEx
    #[doc(alias = "MEMCreateExpHeapEx")]
    #[link_name = "MEMCreateExpHeapEx"]
    pub unsafe fn create_ex(start: *mut ffi::c_void, size: u32, flags: HeapFlags) -> HeapHandle;

    /// MEMDestroyExpHeap
    ///
    /// Returns `heap`, which is the start of the heap memory rounded up to 4 bytes.
    #[doc(alias = "MEMDestroyExpHeap")]
    #[link_name = "MEMDestroyExpHeap"]
    pub unsafe fn destroy(heap: HeapHandle) -> *mut ffi::c_void;

    /// MEMAllocFromExpHeapEx
    ///
    /// A negative `alignment` allocates from the end of the heap.
    #[doc(alias = "MEMAllocFromExpHeapEx")]
    #[link_name = "MEMAllocFromExpHeapEx"]
    pub unsafe fn alloc_ex(heap: HeapHandle, size: u32, alignment: i32) -> *mut ffi::c_void;

    /// MEMFreeToExpHeap
    #[doc(alias = "MEMFreeToExpHeap")]
    #[link_name = "MEMFreeToExpHeap"]
    pub unsafe fn free(heap: HeapHandle, block: *mut ffi::c_void);

    /// MEMResizeForMBlockExpHeap
    ///
    /// Returns the new size, or 0 on failure.
    #[doc(alias = "MEMResizeForMBlockExpHeap")]
    #[link_name = "MEMResizeForMBlockExpHeap"]
    pub unsafe fn resize_for_mblock(heap: HeapHandle, block: *mut ffi::c_void, size: u32) -> u32;

    /// MEMAdjustExpHeap
    ///
    /// Removes the free space at the end of the heap. Returns the new size, or 0 if the heap does not end with free space.
    #[doc(alias = "MEMAdjustExpHeap")]
    #[link_name = "MEMAdjustExpHeap"]
    pub unsafe fn adjust(heap: HeapHandle) -> u32;

    /// MEMGetTotalFreeSizeForExpHeap
    #[doc(alias = "MEMGetTotalFreeSizeForExpHeap")]
    #[link_name = "MEMGetTotalFreeSizeForExpHeap"]
    pub unsafe fn get_total_free_size(heap: HeapHandle) -> u32;

    /// MEMGetAllocatableSizeForExpHeapEx
    #[doc(alias = "MEMGetAllocatableSizeForExpHeapEx")]
    #[link_name = "MEMGetAllocatableSizeForExpHeapEx"]
    pub unsafe fn get_allocatable_size_ex(heap: HeapHandle, alignment: i32) -> u32;

    /// MEMiIsEmptyExpHeap
    #[doc(alias = "MEMiIsEmptyExpHeap")]
    #[link_name = "MEMiIsEmptyExpHeap"]
    pub unsafe fn is_empty(heap: HeapHandle) -> ffi::c_bool;

    /// MEMSetAllocModeForExpHeap
    ///
    /// Returns the previous mode.
    #[doc(alias = "MEMSetAllocModeForExpHeap")]
    #[link_name = "MEMSetAllocModeForExpHeap"]
    pub unsafe fn set_alloc_mode(heap: HeapHandle, mode: ExpHeapMode) -> ExpHeapMode;

    /// MEMGetAllocModeForExpHeap
    #[doc(alias = "MEMGetAllocModeForExpHeap")]
    #[link_name = "MEMGetAllocModeForExpHeap"]
    pub unsafe fn get_alloc_mode(heap: HeapHandle) -> ExpHeapMode;

    /// MEMUseMarginOfAlignmentForExpHeap
    ///
    /// Whether the space lost to alignment is reused. Returns the previous setting.
    #[doc(alias = "MEMUseMarginOfAlignmentForExpHeap")]
    #[link_name = "MEMUseMarginOfAlignmentForExpHeap"]
    pub unsafe fn use_margin_of_alignment(heap: HeapHandle, reuse: ffi::c_bool) -> ffi::c_bool;

    /// MEMSetGroupIDForExpHeap
    ///
    /// Group ID (0-255) of subsequently allocated blocks. Returns the previous ID.
    #[doc(alias = "MEMSetGroupIDForExpHeap")]
    #[link_name = "MEMSetGroupIDForExpHeap"]
    pub unsafe fn set_group_id(heap: HeapHandle, id: u16) -> u16;

    /// MEMGetGroupIDForExpHeap
    #[doc(alias = "MEMGetGroupIDForExpHeap")]
    #[link_name = "MEMGetGroupIDForExpHeap"]
    pub unsafe fn get_group_id(heap: HeapHandle) -> u16;

    /// MEMGetSizeForMBlockExpHeap
    #[doc(alias = "MEMGetSizeForMBlockExpHeap")]
    #[link_name = "MEMGetSizeForMBlockExpHeap"]
    pub unsafe fn get_size_for_mblock(block: *const ffi::c_void) -> u32;

    /// MEMGetGroupIDForMBlockExpHeap
    #[doc(alias = "MEMGetGroupIDForMBlockExpHeap")]
    #[link_name = "MEMGetGroupIDForMBlockExpHeap"]
    pub unsafe fn get_group_id_for_mblock(block: *const ffi::c_void) -> u16;

    /// MEMGetAllocDirForMBlockExpHeap
    #[doc(alias = "MEMGetAllocDirForMBlockExpHeap")]
    #[link_name = "MEMGetAllocDirForMBlockExpHeap"]
    pub unsafe fn get_alloc_dir_for_mblock(block: *const ffi::c_void) -> ExpHeapDirection;

    /// MEMVisitAllocatedForExpHeap
    ///
    /// Calls `visitor` for every allocated block.
    #[doc(alias = "MEMVisitAllocatedForExpHeap")]
    #[link_name = "MEMVisitAllocatedForExpHeap"]
    pub unsafe fn visit_allocated(
        heap: HeapHandle,
        visitor: BlockVisitor,
        context: *mut ffi::c_void,
    );

    /// MEMCheckExpHeap
    #[doc(alias = "MEMCheckExpHeap")]
    #[link_name = "MEMCheckExpHeap"]
    pub unsafe fn check(heap: HeapHandle, flags: CheckFlags) -> ffi::c_bool;

    /// MEMCheckForMBlockExpHeap
    #[doc(alias = "MEMCheckForMBlockExpHeap")]
    #[link_name = "MEMCheckForMBlockExpHeap"]
    pub unsafe fn check_for_mblock(
        block: *const ffi::c_void,
        heap: HeapHandle,
        flags: CheckFlags,
    ) -> ffi::c_bool;
}
