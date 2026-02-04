use super::{Link, thread::ThreadSimpleQueue};
use crate::ffi;

/// OSFastMutex
#[repr(C)]
#[derive(Clone)]
pub struct FastMutex {
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk0: ffi::unknown<0x4>,
    pub queue: ThreadSimpleQueue,
    pub link: FastMutexLink,
    _unk1: ffi::unknown<0x10>,
}

impl FastMutex {
    pub const TAG: u32 = 0x664D7458;
}

/// OSFastMutexLink
pub type FastMutexLink = Link<FastMutex>;

/// OSFastMutexQueue
#[repr(C)]
#[derive(Clone)]
pub struct FastMutexQueue {
    pub head: *mut FastMutex,
    pub tail: *mut FastMutex,
}

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSFastMutex_Init
    #[link_name = "OSFastMutex_Init"]
    pub unsafe fn init(mutex: *mut FastMutex, name: *const ffi::c_char);

    /// OSFastMutex_Lock
    #[link_name = "OSFastMutex_Lock"]
    pub unsafe fn lock(mutex: *mut FastMutex);

    /// OSFastMutex_Unlock
    #[link_name = "OSFastMutex_Unlock"]
    pub unsafe fn unlock(mutex: *mut FastMutex);

    /// OSFastMutex_TryLock
    #[link_name = "OSFastMutex_TryLock"]
    pub unsafe fn try_lock(mutex: *mut FastMutex) -> ffi::c_bool;
}
