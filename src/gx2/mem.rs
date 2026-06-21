use crate::gx2::surface::ResourceFlags;
use crate::{UnsafeInit, ffi::*};
use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Invalidate : u32 {
        const AttributeBuffer = 1 << 0;
        const Texture = 1 << 1;
        const UniformBlock = 1 << 2;
        const ConstantBuffer = 1 << 2;
        const Shader = 1 << 3;
        const ColorBuffer = 1 << 4;
        const DepthBuffer = 1 << 5;
        const Cpu = 1 << 6;
        const StreamOutBuffer = 1 << 7;
        const ExportBuffer = 1 << 8;
    }
}

pub type AllocFn =
    Option<unsafe extern "C" fn(flags: ResourceFlags, size: u32, align: u32) -> *mut c_void>;

pub type FreeFn = Option<unsafe extern "C" fn(flags: ResourceFlags, ptr: *mut c_void)>;

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2Invalidate
    #[link_name = "GX2Invalidate"]
    pub unsafe fn invalidate(mode: Invalidate, buffer: *mut c_void, size: u32);

    /// GX2RInvalidateBuffer
    #[link_name = "GX2RInvalidateBuffer"]
    pub unsafe fn invalidate_ex(buffer: *const Buffer, options: ResourceFlags);

    /// GX2RSetAllocator
    #[link_name = "GX2RSetAllocator"]
    pub unsafe fn set_allocator(alloc: AllocFn, free: FreeFn);
}

/// GX2RBuffer
#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    pub flags: ResourceFlags,
    pub element_size: u32,
    pub element_count: u32,
    pub ptr: *mut c_void,
}

impl UnsafeInit for Buffer {}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2RCreateBuffer
    #[link_name = "GX2RCreateBuffer"]
    pub unsafe fn create_buffer(buf: *mut Buffer) -> c_bool;

    /// GX2RDestroyBufferEx
    #[link_name = "GX2RDestroyBufferEx"]
    pub unsafe fn destroy_buffer(buf: *mut Buffer, flags: ResourceFlags);

    /// GX2RLockBufferEx
    #[link_name = "GX2RLockBufferEx"]
    pub unsafe fn lock_buffer_ex(buf: *const Buffer, flags: ResourceFlags) -> *mut c_void;

    /// GX2RUnlockBufferEx
    #[link_name = "GX2RUnlockBufferEx"]
    pub unsafe fn unlock_buffer_ex(buf: *const Buffer, flags: ResourceFlags);

    #[doc(alias = "GX2RInvalidateBuffer")]
    #[link_name = "GX2RInvalidateBuffer"]
    pub unsafe fn invalidate_buffer(buf: *const Buffer, flags: ResourceFlags);
}
