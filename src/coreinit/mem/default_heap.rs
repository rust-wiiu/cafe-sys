//! Default heap

use super::HeapHandle;
use crate::ffi;

/// MEMAllocFromDefaultHeapFn
#[doc(alias = "MEMAllocFromDefaultHeapFn")]
pub type AllocFromDefaultHeapFn = Option<unsafe extern "C" fn(size: u32) -> *mut ffi::c_void>;

/// MEMAllocFromDefaultHeapExFn
#[doc(alias = "MEMAllocFromDefaultHeapExFn")]
pub type AllocFromDefaultHeapExFn =
    Option<unsafe extern "C" fn(size: u32, alignment: i32) -> *mut ffi::c_void>;

/// MEMFreeToDefaultHeapFn
#[doc(alias = "MEMFreeToDefaultHeapFn")]
pub type FreeToDefaultHeapFn = Option<unsafe extern "C" fn(ptr: *mut ffi::c_void)>;

unsafe extern "C" {
    /// CoreInitDefaultHeap
    ///
    /// Creates the default heaps and returns the MEM1, foreground and MEM2 heaps.
    #[doc(alias = "CoreInitDefaultHeap")]
    #[link_name = "CoreInitDefaultHeap"]
    pub unsafe fn init(mem1: *mut HeapHandle, fg: *mut HeapHandle, mem2: *mut HeapHandle);
}

unsafe extern "C" {
    /// MEMAllocFromDefaultHeap
    #[doc(alias = "MEMAllocFromDefaultHeap")]
    #[link_name = "MEMAllocFromDefaultHeap"]
    pub static mut ALLOC_FROM_DEFAULT_HEAP: AllocFromDefaultHeapFn;

    /// MEMAllocFromDefaultHeapEx
    #[doc(alias = "MEMAllocFromDefaultHeapEx")]
    #[link_name = "MEMAllocFromDefaultHeapEx"]
    pub static mut ALLOC_FROM_DEFAULT_HEAP_EX: AllocFromDefaultHeapExFn;

    /// MEMFreeToDefaultHeap
    #[doc(alias = "MEMFreeToDefaultHeap")]
    #[link_name = "MEMFreeToDefaultHeap"]
    pub static mut FREE_TO_DEFAULT_HEAP: FreeToDefaultHeapFn;
}
