//! dynload

use crate::ffi::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// OSDynLoad_Error
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Status {
    Ok = 0,
    OutOfMemory = 0xBAD10002,
    InvalidNotifyPointer = 0xBAD1000E,
    InvalidModuleNamePointer = 0xBAD1000F,
    InvalidModuleName = 0xBAD10010,
    InvalidAcquirePointer = 0xBAD10011,
    EmptyModuleName = 0xBAD10012,
    InvalidAllocatorPointer = 0xBAD10017,
    OutOfSystemMemory = 0xBAD1002F,
    TlsAllocatorLocked = 0xBAD10031,
    ModuleNotFound = 0xFFFFFFFA,
}

pub type AllocFn =
    unsafe extern "C" fn(size: i32, align: i32, out_addr: *mut *mut c_void) -> Status;

pub type FreeFn = unsafe extern "C" fn(addr: *mut c_void);

unsafe extern "C" {
    /// Get the allocator functions used for dynamic loading.
    ///
    /// OSDynLoad_GetAllocator
    #[link_name = "OSDynLoad_GetAllocator"]
    pub unsafe fn get_allocator(alloc: *mut AllocFn, free: *mut FreeFn) -> Status;

    /// Set the allocator functions to use for dynamic loading.
    ///
    /// OSDynLoad_SetAllocator
    #[link_name = "OSDynLoad_SetAllocator"]
    pub unsafe fn set_allocator(alloc: AllocFn, free: FreeFn) -> Status;

    /// Get the allocator functions used for thread local storage.
    ///
    /// OSDynLoad_GetTLSAllocator
    #[link_name = "OSDynLoad_GetTLSAllocator"]
    pub unsafe fn get_tls_allocator(alloc: *mut AllocFn, free: *mut FreeFn) -> Status;

    /// Set the allocator functions to use for thread local storage.
    ///
    /// OSDynLoad_SetTLSAllocator
    #[link_name = "OSDynLoad_SetTLSAllocator"]
    pub unsafe fn set_tls_allocator(alloc: AllocFn, free: FreeFn) -> Status;
}
