//! Memory
//!
//! System memory allocation, block copy/fill, memory bounds, shared data and the overlay arena.

use crate::ffi;

/// OSMemoryType
#[doc(alias = "OSMemoryType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryType(pub u32);

impl MemoryType {
    pub const MEM1: Self = Self(1);
    pub const MEM2: Self = Self(2);
}

/// OSSharedDataType
#[doc(alias = "OSSharedDataType", alias = "OSShared")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SharedDataType(pub u32);

impl SharedDataType {
    pub const FONT_CHINESE: Self = Self(0);
    pub const FONT_KOREAN: Self = Self(1);
    pub const FONT_STANDARD: Self = Self(2);
    pub const FONT_TAIWANESE: Self = Self(3);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverlayArenaError(pub u32);

impl OverlayArenaError {
    pub const INVALID: Self = Self(0xFFFFFFFF);
    pub const IN_SHUTDOWN: Self = Self(0xFFFFFFFD);
    pub const TIMEOUT: Self = Self(0xFFFFFFFC);
    pub const ALREADY_ENABLED: Self = Self(0xFFFFFFFB);
    pub const NOT_ENABLED: Self = Self(0xFFFFFFFA);
    pub const UNKNOWN: Self = Self(0xFFFFFFF9);
    pub const NOT_SET_IN_PLAT_INFO: Self = Self(0xFFFFFFF8);
}

unsafe extern "C" {
    /// OSAllocFromSystem
    #[doc(alias = "OSAllocFromSystem")]
    #[link_name = "OSAllocFromSystem"]
    pub unsafe fn alloc_from_system(size: u32, align: i32) -> *mut ffi::c_void;

    /// OSFreeToSystem
    #[doc(alias = "OSFreeToSystem")]
    #[link_name = "OSFreeToSystem"]
    pub unsafe fn free_to_system(ptr: *mut ffi::c_void);

    /// OSBlockMove
    ///
    /// Copies `size` bytes, also between overlapping areas. Flushes the data cache of `dst` if `flush` is true. Returns `dst`.
    #[doc(alias = "OSBlockMove")]
    #[link_name = "OSBlockMove"]
    pub unsafe fn block_move(
        dst: *mut ffi::c_void,
        src: *const ffi::c_void,
        size: u32,
        flush: ffi::c_bool,
    ) -> *mut ffi::c_void;

    /// OSBlockSet
    ///
    /// Returns `dst`.
    #[doc(alias = "OSBlockSet")]
    #[link_name = "OSBlockSet"]
    pub unsafe fn block_set(dst: *mut ffi::c_void, value: u8, size: u32) -> *mut ffi::c_void;

    /// OSSetMemBound
    #[doc(alias = "OSSetMemBound")]
    #[link_name = "OSSetMemBound"]
    pub unsafe fn set_mem_bound(kind: MemoryType, start: u32, size: u32) -> i32;

    /// OSGetForegroundBucket
    #[doc(alias = "OSGetForegroundBucket")]
    #[link_name = "OSGetForegroundBucket"]
    pub unsafe fn get_foreground_bucket(area: *mut *mut ffi::c_void, size: *mut u32)
    -> ffi::c_bool;

    /// OSGetSharedData
    ///
    /// Gets the address and size of a system font. Returns false on failure.
    #[doc(alias = "OSGetSharedData")]
    #[link_name = "OSGetSharedData"]
    pub unsafe fn get_shared_data(
        kind: SharedDataType,
        flags: u32,
        area: *mut *mut ffi::c_void,
        size: *mut u32,
    ) -> ffi::c_bool;

    /// __OSRootLoadShared
    #[doc(alias = "__OSRootLoadShared")]
    #[link_name = "__OSRootLoadShared"]
    pub unsafe fn root_load_shared();

    /// __OSZeroProcessMemory
    #[doc(alias = "__OSZeroProcessMemory")]
    #[link_name = "__OSZeroProcessMemory"]
    pub unsafe fn zero_process_memory(process_id: u32);

    /// OSIsEnabledOverlayArenaDev
    #[doc(alias = "OSIsEnabledOverlayArenaDev")]
    #[link_name = "OSIsEnabledOverlayArenaDev"]
    #[deprecated = "Only for development / devkits."]
    pub unsafe fn is_enabled_overlay_arena_dev() -> ffi::c_bool;

    /// OSEnableOverlayArenaDev
    #[doc(alias = "OSEnableOverlayArenaDev")]
    #[link_name = "OSEnableOverlayArenaDev"]
    #[deprecated = "Only for development / devkits."]
    pub unsafe fn enable_overlay_arena_dev(allow_av: ffi::c_bool, addr: *mut u32, size: *mut u32);

    /// OSEnableOverlayArenaWithTimeoutDev
    #[doc(alias = "OSEnableOverlayArenaWithTimeoutDev")]
    #[link_name = "OSEnableOverlayArenaWithTimeoutDev"]
    #[deprecated = "Only for development / devkits."]
    pub unsafe fn enable_overlay_arena_with_timeout_dev(
        allow_av: ffi::c_bool,
        timeout: u32,
        error: *mut OverlayArenaError,
        addr: *mut u32,
        size: *mut u32,
    ) -> ffi::c_bool;

    /// OSDisableOverlayArenaDev
    #[doc(alias = "OSDisableOverlayArenaDev")]
    #[link_name = "OSDisableOverlayArenaDev"]
    #[deprecated = "Only for development / devkits."]
    pub unsafe fn disable_overlay_arena_dev();

    /// OSDisableOverlayArenaWithTimeoutDev
    #[doc(alias = "OSDisableOverlayArenaWithTimeoutDev")]
    #[link_name = "OSDisableOverlayArenaWithTimeoutDev"]
    #[deprecated = "Only for development / devkits."]
    pub unsafe fn disable_overlay_arena_with_timeout_dev(
        timeout: u32,
        error: *mut OverlayArenaError,
    ) -> ffi::c_bool;
}
