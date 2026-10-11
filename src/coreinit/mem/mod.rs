//! Memory heaps (MEM)
//!
//! Heap management shared by all heap kinds: base heaps per arena, heap lookup, debug fill values.

use crate::coreinit::spinlock::SpinLock;
use crate::ffi;

pub mod allocator;
pub mod block_heap;
pub mod default_heap;
pub mod exp_heap;
pub mod frm_heap;
pub mod list;
pub mod unit_heap;

/// MEM_HEAP_DEFAULT_ALIGNMENT
#[doc(alias = "MEM_HEAP_DEFAULT_ALIGNMENT")]
pub const DEFAULT_ALIGNMENT: i32 = 4;

/// MEMHeapTag
///
/// Signature at the start of every heap header.
#[doc(alias = "MEMHeapTag")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapTag(pub u32);

impl HeapTag {
    pub const BLOCK: Self = Self(0x424C4B48);
    pub const EXPANDED: Self = Self(0x45585048);
    pub const FRAME: Self = Self(0x46524D48);
    pub const UNIT: Self = Self(0x554E5448);
    pub const USER: Self = Self(0x55535248);
}

/// MEMHeapFlags
#[doc(alias = "MEMHeapFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapFlags(pub u16);

impl HeapFlags {
    pub const NONE: Self = Self(0);
    /// Zero-fills allocated blocks.
    pub const ZERO_ALLOCATED: Self = Self(1 << 0);
    /// Fills blocks with the debug fill values, see [set_fill_val].
    pub const DEBUG_MODE: Self = Self(1 << 1);
    /// Makes the heap thread-safe.
    pub const USE_LOCK: Self = Self(1 << 2);
}

impl core::ops::BitOr for HeapFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MEMBaseHeapType
#[doc(alias = "MEMBaseHeapType", alias = "MEMArena")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaseHeapType(pub u32);

impl BaseHeapType {
    pub const MEM1: Self = Self(0);
    pub const MEM2: Self = Self(1);
    pub const ARENA_3: Self = Self(2);
    pub const ARENA_4: Self = Self(3);
    pub const ARENA_5: Self = Self(4);
    pub const ARENA_6: Self = Self(5);
    pub const ARENA_7: Self = Self(6);
    pub const ARENA_8: Self = Self(7);
    /// Foreground bucket.
    pub const FG: Self = Self(8);
    pub const INVALID: Self = Self(10);
}

/// MEMHeapFillType
#[doc(alias = "MEMHeapFillType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapFillType(pub u32);

impl HeapFillType {
    pub const UNUSED: Self = Self(0);
    pub const ALLOCATED: Self = Self(1);
    pub const FREED: Self = Self(2);
}

/// MEMHeapCheckFlags
#[doc(alias = "MEMExpHeapCheckFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckFlags(pub u32);

impl CheckFlags {
    pub const NONE: Self = Self(0);
    /// Prints the errors found.
    pub const LOG_ERRORS: Self = Self(1 << 0);
}

/// MEMHeapHeader
#[doc(alias = "MEMHeapHeader", alias = "MEMiHeapHead")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HeapHeader {
    pub tag: HeapTag,
    pub link: list::Link,
    pub list: list::List,
    pub data_start: *mut ffi::c_void,
    pub data_end: *mut ffi::c_void,
    pub lock: SpinLock,
    /// [HeapFlags] in the low 8 bits.
    pub flags: u32,
}

/// MEMHeapHandle
#[doc(alias = "MEMHeapHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapHandle(pub *mut HeapHeader);

impl HeapHandle {
    pub const INVALID: Self = Self(core::ptr::null_mut());

    #[inline]
    pub fn is_invalid(self) -> bool {
        self.0.is_null()
    }
}

impl Default for HeapHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

unsafe extern "C" {
    /// MEMGetBaseHeapHandle
    #[doc(alias = "MEMGetBaseHeapHandle")]
    #[link_name = "MEMGetBaseHeapHandle"]
    pub unsafe fn get_base_heap_handle(kind: BaseHeapType) -> HeapHandle;

    /// MEMSetBaseHeapHandle
    ///
    /// Returns the previous handle.
    #[doc(alias = "MEMSetBaseHeapHandle")]
    #[link_name = "MEMSetBaseHeapHandle"]
    pub unsafe fn set_base_heap_handle(kind: BaseHeapType, heap: HeapHandle) -> HeapHandle;

    /// MEMGetArena
    #[doc(alias = "MEMGetArena")]
    #[link_name = "MEMGetArena"]
    pub unsafe fn get_arena(heap: HeapHandle) -> BaseHeapType;

    /// MEMCreateUserHeapHandle
    ///
    /// Creates a handle for a user-managed heap occupying `size` bytes at `heap`.
    #[doc(alias = "MEMCreateUserHeapHandle")]
    #[link_name = "MEMCreateUserHeapHandle"]
    pub unsafe fn create_user_heap_handle(heap: *mut ffi::c_void, size: u32) -> HeapHandle;

    /// MEMFindContainHeap
    ///
    /// Returns the heap containing `block`.
    #[doc(alias = "MEMFindContainHeap")]
    #[link_name = "MEMFindContainHeap"]
    pub unsafe fn find_contain_heap(block: *const ffi::c_void) -> HeapHandle;

    /// MEMFindParentHeap
    #[doc(alias = "MEMFindParentHeap")]
    #[link_name = "MEMFindParentHeap"]
    pub unsafe fn find_parent_heap(heap: HeapHandle) -> HeapHandle;

    /// MEMDumpHeap
    #[doc(alias = "MEMDumpHeap")]
    #[link_name = "MEMDumpHeap"]
    pub unsafe fn dump(heap: HeapHandle);

    /// MEMCheckHeap
    #[doc(alias = "MEMCheckHeap")]
    #[link_name = "MEMCheckHeap"]
    pub unsafe fn check(heap: HeapHandle) -> ffi::c_bool;

    /// MEMGetFillValForHeap
    #[doc(alias = "MEMGetFillValForHeap")]
    #[link_name = "MEMGetFillValForHeap"]
    pub unsafe fn get_fill_val(kind: HeapFillType) -> u32;

    /// MEMSetFillValForHeap
    ///
    /// Value written by heaps created with [HeapFlags::DEBUG_MODE]. Returns the previous value.
    #[doc(alias = "MEMSetFillValForHeap")]
    #[link_name = "MEMSetFillValForHeap"]
    pub unsafe fn set_fill_val(kind: HeapFillType, value: u32) -> u32;
}
