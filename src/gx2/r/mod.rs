//! GX2R
//!
//! Resource management on top of GX2: buffers and surfaces with allocation, locking and cache invalidation.

use crate::ffi;

pub mod buffer;
pub mod display_list;
pub mod draw;
pub mod surface;

/// GX2RResourceFlags
#[doc(alias = "GX2RResourceFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceFlags(pub u32);

impl ResourceFlags {
    pub const BIND_NONE: Self = Self(0);
    pub const BIND_TEXTURE: Self = Self(1 << 0);
    pub const BIND_COLOR_BUFFER: Self = Self(1 << 1);
    pub const BIND_DEPTH_BUFFER: Self = Self(1 << 2);
    pub const BIND_SCAN_BUFFER: Self = Self(1 << 3);
    pub const BIND_VERTEX_BUFFER: Self = Self(1 << 4);
    pub const BIND_INDEX_BUFFER: Self = Self(1 << 5);
    pub const BIND_UNIFORM_BLOCK: Self = Self(1 << 6);
    pub const BIND_SHADER_PROGRAM: Self = Self(1 << 7);
    pub const BIND_STREAM_OUTPUT: Self = Self(1 << 8);
    pub const BIND_DISPLAY_LIST: Self = Self(1 << 9);
    pub const BIND_GS_RING_BUFFER: Self = Self(1 << 10);
    pub const USAGE_CPU_READ: Self = Self(1 << 11);
    pub const USAGE_CPU_WRITE: Self = Self(1 << 12);
    pub const USAGE_GPU_READ: Self = Self(1 << 13);
    pub const USAGE_GPU_WRITE: Self = Self(1 << 14);
    pub const USAGE_DMA_READ: Self = Self(1 << 15);
    pub const USAGE_DMA_WRITE: Self = Self(1 << 16);
    pub const USAGE_FORCE_MEM1: Self = Self(1 << 17);
    pub const USAGE_FORCE_MEM2: Self = Self(1 << 18);
    /// Ignores that the resource is still in use by the GPU.
    pub const IGNORE_IN_USE: Self = Self(1 << 19);
    pub const DISABLE_CPU_INVALIDATE: Self = Self(1 << 20);
    pub const DISABLE_GPU_INVALIDATE: Self = Self(1 << 21);
    pub const LOCKED_READ_ONLY: Self = Self(1 << 22);
    /// Does not touch the memory on destruction.
    pub const NO_TOUCH_DESTROY: Self = Self(1 << 23);
    pub const GX2R_ALLOCATED: Self = Self(1 << 29);
    pub const LOCKED: Self = Self(1 << 30);
}

impl core::ops::BitOr for ResourceFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2RBuffer
#[doc(alias = "GX2RBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Buffer {
    pub flags: ResourceFlags,
    pub elem_size: u32,
    pub elem_count: u32,
    pub buffer: *mut ffi::c_void,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            flags: ResourceFlags::BIND_NONE,
            elem_size: 0,
            elem_count: 0,
            buffer: core::ptr::null_mut(),
        }
    }
}

/// GX2RDebugOptions
#[doc(alias = "GX2RDebugOptions")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugOptions(pub u32);

impl DebugOptions {
    pub const NONE: Self = Self(0);
    pub const GUARD_BANDS_ENABLED: Self = Self(1 << 0);
    pub const GUARD_BANDS_CHECK_ALWAYS: Self = Self(1 << 1);
    pub const LEAK_CHECK: Self = Self(1 << 2);
    pub const CHECK_GPU_CONTENTION: Self = Self(1 << 3);
}

impl core::ops::BitOr for DebugOptions {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2RAllocFunction
#[doc(alias = "GX2RAllocFunction", alias = "GX2RAllocateFunc")]
pub type AllocFunction = Option<
    unsafe extern "C" fn(flags: ResourceFlags, size: u32, alignment: u32) -> *mut ffi::c_void,
>;

/// GX2RFreeFunction
#[doc(alias = "GX2RFreeFunction", alias = "GX2RFreeFunc")]
pub type FreeFunction =
    Option<unsafe extern "C" fn(flags: ResourceFlags, memory: *mut ffi::c_void)>;

unsafe extern "C" {
    /// GX2RSetAllocator
    #[doc(alias = "GX2RSetAllocator")]
    #[link_name = "GX2RSetAllocator"]
    pub unsafe fn set_allocator(alloc: AllocFunction, free: FreeFunction);

    /// GX2RSetDebugOptions
    #[doc(alias = "GX2RSetDebugOptions")]
    #[link_name = "GX2RSetDebugOptions"]
    pub unsafe fn set_debug_options(options: DebugOptions);

    /// GX2RGetDebugOptions
    #[doc(alias = "GX2RGetDebugOptions")]
    #[link_name = "GX2RGetDebugOptions"]
    pub unsafe fn get_debug_options() -> DebugOptions;

    /// GX2RIsUserMemory
    #[doc(alias = "GX2RIsUserMemory")]
    #[link_name = "GX2RIsUserMemory"]
    pub unsafe fn is_user_memory(flags: ResourceFlags) -> ffi::c_bool;

    /// GX2RInvalidateMemory
    ///
    /// Invalidates the caches for `memory` according to the bind and usage `flags`.
    #[doc(alias = "GX2RInvalidateMemory")]
    #[link_name = "GX2RInvalidateMemory"]
    pub unsafe fn invalidate_memory(flags: ResourceFlags, memory: *mut ffi::c_void, size: u32);
}
