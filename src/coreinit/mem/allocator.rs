//! Allocators (MEMAllocator)
//!
//! A uniform alloc/free interface on top of any heap kind.

use super::HeapHandle;
use crate::ffi;

/// MEMAllocatorAllocFn
#[doc(alias = "MEMAllocatorAllocFn", alias = "MEMFuncAllocatorAlloc")]
pub type AllocFn =
    Option<unsafe extern "C" fn(allocator: *mut Allocator, size: u32) -> *mut ffi::c_void>;

/// MEMAllocatorFreeFn
#[doc(alias = "MEMAllocatorFreeFn", alias = "MEMFuncAllocatorFree")]
pub type FreeFn = Option<unsafe extern "C" fn(allocator: *mut Allocator, ptr: *mut ffi::c_void)>;

/// MEMAllocatorFunctions
#[doc(alias = "MEMAllocatorFunctions", alias = "MEMAllocatorFunc")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AllocatorFunctions {
    pub alloc: AllocFn,
    pub free: FreeFn,
}

/// MEMAllocator
#[doc(alias = "MEMAllocator")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Allocator {
    pub funcs: *const AllocatorFunctions,
    pub heap: HeapHandle,
    pub align: u32,
    _unk: ffi::unknown<4>,
}

impl Default for Allocator {
    fn default() -> Self {
        Self {
            funcs: core::ptr::null(),
            heap: HeapHandle::INVALID,
            align: 0,
            _unk: ffi::unknown::zero(),
        }
    }
}

unsafe extern "C" {
    /// MEMAllocFromAllocator
    #[doc(alias = "MEMAllocFromAllocator")]
    #[link_name = "MEMAllocFromAllocator"]
    pub unsafe fn alloc(allocator: *mut Allocator, size: u32) -> *mut ffi::c_void;

    /// MEMFreeToAllocator
    #[doc(alias = "MEMFreeToAllocator")]
    #[link_name = "MEMFreeToAllocator"]
    pub unsafe fn free(allocator: *mut Allocator, ptr: *mut ffi::c_void);

    /// MEMInitAllocatorForExpHeap
    #[doc(alias = "MEMInitAllocatorForExpHeap")]
    #[link_name = "MEMInitAllocatorForExpHeap"]
    pub unsafe fn init_for_exp_heap(allocator: *mut Allocator, heap: HeapHandle, alignment: i32);

    /// MEMInitAllocatorForFrmHeap
    #[doc(alias = "MEMInitAllocatorForFrmHeap")]
    #[link_name = "MEMInitAllocatorForFrmHeap"]
    pub unsafe fn init_for_frm_heap(allocator: *mut Allocator, heap: HeapHandle, alignment: i32);

    /// MEMInitAllocatorForUnitHeap
    #[doc(alias = "MEMInitAllocatorForUnitHeap")]
    #[link_name = "MEMInitAllocatorForUnitHeap"]
    pub unsafe fn init_for_unit_heap(allocator: *mut Allocator, heap: HeapHandle);

    /// MEMInitAllocatorForBlockHeap
    #[doc(alias = "MEMInitAllocatorForBlockHeap")]
    #[link_name = "MEMInitAllocatorForBlockHeap"]
    pub unsafe fn init_for_block_heap(allocator: *mut Allocator, heap: HeapHandle, alignment: i32);

    /// MEMInitAllocatorForDefaultHeap
    #[doc(alias = "MEMInitAllocatorForDefaultHeap")]
    #[link_name = "MEMInitAllocatorForDefaultHeap"]
    pub unsafe fn init_for_default_heap(allocator: *mut Allocator);
}
