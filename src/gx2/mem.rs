use crate::gx2::surface::ResourceFlags;
use crate::{UnsafeInit, ffi::*};
use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Invalidate : u32 {
        const Cpu = 1 << 6;
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
    _internal: [u32; 1],
}

// impl Drop for Buffer {
//     fn drop(&mut self) {
//         unsafe {
//             destroy_buffer(self, ResourceFlags::empty());
//         }
//     }
// }

// #[bon]
// impl Buffer {
//     #[builder]
//     pub fn new(flags: ResourceFlags, element_size: usize, element_count: usize) -> Self {
//         let mut s = Self {
//             flags,
//             element_size: element_size as u32,
//             element_count: element_count as u32,
//             _internal: [0; 1],
//         };

//         let success = unsafe { create_buffer(&mut s) } != 0;
//         if !success {
//             panic!("OOM");
//         }

//         s
//     }
// }

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
}
