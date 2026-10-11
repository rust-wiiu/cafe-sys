//! IPC buffer pools
//!
//! Message buffer pools for IPC.

use crate::coreinit::ios;
use crate::coreinit::mutex::Mutex;
use crate::ffi;

/// IPC_BUF_POOL_MAGIC
#[doc(alias = "IPC_BUF_POOL_MAGIC")]
pub const MAGIC: u32 = 0x0BADF00D;

/// IPCBufPoolFIFO
#[doc(alias = "IPCBufPoolFIFO")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fifo {
    pub push_index: i32,
    pub pop_index: i32,
    pub count: i32,
    pub max_count: i32,
    pub messages: *mut *mut ffi::c_void,
}

/// IPCBufPoolAttributes
#[doc(alias = "IPCBufPoolAttributes")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes {
    pub message_size: u32,
    pub pool_size: u32,
    pub num_messages: u32,
}

/// IPCBufPool
#[doc(alias = "IPCBufPool")]
#[repr(C)]
#[derive(Debug)]
pub struct Pool {
    pub magic: u32,
    pub buffer: *mut ffi::c_void,
    pub size: u32,
    _unk0x0c: ffi::unknown<4>,
    _unk0x10: ffi::unknown<4>,
    pub message_size_0x14: u32,
    pub message_size_0x18: u32,
    pub message_count: u32,
    pub messages: *mut ffi::c_void,
    pub message_index_size: *mut u32,
    pub fifo: Fifo,
    pub mutex: Mutex,
    _unk0x68: ffi::unknown<4>,
}

const _: () = {
    assert!(size_of::<Attributes>() == 0xC);
};

unsafe extern "C" {
    /// IPCBufPoolCreate
    ///
    /// Creates a pool of `message_size` messages in `buffer`, the pool header included.
    /// `message_size` is rounded up to a multiple of 64. Returns null if `buffer` is too small.
    #[doc(alias = "IPCBufPoolCreate")]
    #[link_name = "IPCBufPoolCreate"]
    pub unsafe fn create(
        buffer: *mut ffi::c_void,
        size: u32,
        message_size: u32,
        num_messages: *mut u32,
        unknown: u32,
    ) -> *mut Pool;

    /// IPCBufPoolAllocate
    #[doc(alias = "IPCBufPoolAllocate")]
    #[link_name = "IPCBufPoolAllocate"]
    pub unsafe fn allocate(pool: *mut Pool, size: u32) -> *mut ffi::c_void;

    /// IPCBufPoolFree
    #[doc(alias = "IPCBufPoolFree")]
    #[link_name = "IPCBufPoolFree"]
    pub unsafe fn free(pool: *mut Pool, message: *mut ffi::c_void) -> ios::Error;

    /// IPCBufPoolGetAttributes
    #[doc(alias = "IPCBufPoolGetAttributes")]
    #[link_name = "IPCBufPoolGetAttributes"]
    pub unsafe fn get_attributes(pool: *mut Pool, attributes: *mut Attributes) -> ios::Error;
}
