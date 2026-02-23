use crate::ffi::{self};
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// MEMAllocFromDefaultHeapFn
pub type AllocFromDefaultHeapFn = Option<unsafe extern "C" fn(size: u32) -> *mut ffi::c_void>;

/// MEMAllocFromDefaultHeapExFn
pub type AllocFromDefaultHeapExFn =
    Option<unsafe extern "C" fn(size: u32, alignment: i32) -> *mut ffi::c_void>;

/// MEMFreeToDefaultHeapFn
pub type FreeToDefaultHeapFn = Option<unsafe extern "C" fn(ptr: *mut ffi::c_void)>;

/// MEMHeapTag
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum HeapTag {
    BlockHeap = 0x424C4B48,
    ExpandedHeap = 0x45585048,
    FrameHeap = 0x46524D48,
    UnitHeap = 0x554E5448,
    UserHeap = 0x55535248,
}

/// MEMMemoryLink
#[repr(C)]
#[derive(Clone)]
pub struct MemoryLink {
    pub prev: *mut ffi::c_void,
    pub next: *mut ffi::c_void,
}

/// MEMMemoryList
#[repr(C)]
#[derive(Clone)]
pub struct MemoryList {
    head: *mut ffi::c_void,
    tail: *mut ffi::c_void,
    count: u16,
    offset_to_memory_link: u16,
}

/// MEMHeapHeader
#[repr(C)]
#[derive(Clone)]
pub struct HeapHeader {
    pub tag: HeapTag,
    pub link: MemoryLink,
    pub list: MemoryList,
    pub data_start: *mut ffi::c_void,
    pub data_end: *mut ffi::c_void,
    pub lock: super::spinlock::SpinLock,
    pub flags: u32,
    _unk: ffi::unknown<0xC>,
}

/// OSMemoryType
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum MemoryType {
    Mem1 = 1,
    Mem2 = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Arena {
    Mem1 = 0,
    Mem2 = 1,
    Foreground = 8,
}

// /// MEMHeapFlags
// #[repr(u32)]
// #[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
// pub enum HeapFlags {
//     ZeroAllocated = 1 << 0,
//     Mem2 = 2,
// }

/// MEMHeapHandle
pub type HeapHandle = *mut HeapHeader;

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// MEMAllocFromDefaultHeap
    #[link_name = "MEMAllocFromDefaultHeap"]
    pub unsafe static mut AllocFromDefaultHeap: AllocFromDefaultHeapFn;

    /// MEMAllocFromDefaultHeapEx
    #[link_name = "MEMAllocFromDefaultHeapEx"]
    pub unsafe static mut AllocFromDefaultHeapEx: AllocFromDefaultHeapExFn;

    /// MEMFreeToDefaultHeap
    #[link_name = "MEMFreeToDefaultHeap"]
    pub unsafe static mut FreeToDefaultHeap: FreeToDefaultHeapFn;
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// CoreInitDefaultHeap
    #[link_name = "CoreInitDefaultHeap"]
    pub unsafe fn InitDefaultHeap(
        mem1: *mut HeapHandle,
        fg: *mut HeapHandle,
        mem2: *mut HeapHandle,
    );

    /// OSGetForegroundBucket
    #[link_name = "OSGetForegroundBucket"]
    pub unsafe fn foreground_bucket(out_addr: *mut u32, out_size: *mut u32) -> bool;

    /// OSGetForegroundBucketFreeArea
    #[link_name = "OSGetForegroundBucketFreeArea"]
    pub unsafe fn foreground_bucket_free_area(out_addr: *mut u32, out_size: *mut u32) -> bool;

    /// OSGetMemBound
    #[link_name = "OSGetMemBound"]
    pub unsafe fn bounds(r#type: MemoryType, out_addr: *mut u32, out_size: *mut u32) -> i32;

    /// MEMGetBaseHeapHandle
    #[link_name = "MEMGetBaseHeapHandle"]
    pub unsafe fn get_base_handle(arena: Arena) -> HeapHandle;
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum FrameHeapFree {
    Head = 1,
    Tail = 2,
    #[default]
    All = 4,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct HeapFlags : u32 {
        const ClearOnAllocation = 1 << 0;
        const Debug = 1 << 1;
        const ThreadSafe = 1 << 2;
    }
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// MEMCreateFrmHeapEx
    #[link_name = "MEMCreateFrmHeapEx"]
    pub unsafe fn create_frm_heap(
        heap: *mut ffi::c_void,
        size: u32,
        flags: HeapFlags,
    ) -> HeapHandle;

    /// MEMDestroyFrmHeap
    #[link_name = "MEMDestroyFrmHeap"]
    pub unsafe fn destroy_frm_heap(heap: HeapHandle);

    /// MEMRecordStateForFrmHeap
    #[link_name = "MEMRecordStateForFrmHeap"]
    pub unsafe fn record_state_frm_heap(heap: HeapHandle, tag: u32) -> ffi::c_bool;

    /// MEMFreeByStateToFrmHeap
    #[link_name = "MEMFreeByStateToFrmHeap"]
    pub unsafe fn free_state_frm_heap(heap: HeapHandle, tag: u32) -> ffi::c_bool;

    /// MEMGetAllocatableSizeForFrmHeapEx
    #[link_name = "MEMGetAllocatableSizeForFrmHeapEx"]
    pub unsafe fn allocatable_frm_heap(heap: HeapHandle, align: u32) -> u32;

    /// MEMAllocFromFrmHeapEx
    #[link_name = "MEMAllocFromFrmHeapEx"]
    pub unsafe fn alloc_frm_heap(heap: HeapHandle, size: u32, align: i32) -> *mut ffi::c_void;

    /// MEMFreeToFrmHeap
    #[link_name = "MEMFreeToFrmHeap"]
    pub unsafe fn free_frm_heap(heap: HeapHandle, mode: FrameHeapFree);
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// MEMCreateExpHeapEx
    #[link_name = "MEMCreateExpHeapEx"]
    pub unsafe fn create_exp_heap(
        heap: *mut ffi::c_void,
        size: u32,
        flags: HeapFlags,
    ) -> HeapHandle;

    /// MEMDestroyExpHeap
    #[link_name = "MEMDestroyExpHeap"]
    pub unsafe fn destroy_exp_heap(heap: HeapHandle);

    /// MEMAllocFromExpHeapEx
    #[link_name = "MEMAllocFromExpHeapEx"]
    pub unsafe fn alloc_exp_heap(heap: HeapHandle, size: u32, align: i32) -> *mut ffi::c_void;

    #[doc(alias = "MEMGetTotalFreeSizeForExpHeap")]
    #[link_name = "MEMGetTotalFreeSizeForExpHeap"]
    pub unsafe fn available_exp_heap(heap: HeapHandle) -> u32;

    #[doc(alias = "MEMGetAllocatableSizeForExpHeapEx")]
    #[link_name = "MEMGetAllocatableSizeForExpHeapEx"]
    pub unsafe fn allocatable_exp_heap(heap: HeapHandle, align: i32) -> u32;

    /// MEMFreeToExpHeap
    #[link_name = "MEMFreeToExpHeap"]
    pub unsafe fn free_exp_heap(heap: HeapHandle, ptr: *const ffi::c_void);
}
