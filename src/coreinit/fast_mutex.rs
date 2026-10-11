//! Fast mutex
//!
//! Recursive mutex without priority inheritance, cheaper when uncontended.

use super::thread_queue::ThreadSimpleQueue;
use crate::ffi;

/// OS_FAST_MUTEX_TAG
///
/// Value of [FastMutex::tag].
pub const FAST_MUTEX_TAG: u32 = 0x664D7458;

/// OSFastMutexLink
#[doc(alias = "OSFastMutexLink")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastMutexLink {
    pub next: *mut FastMutex,
    pub prev: *mut FastMutex,
}

/// OSFastMutexQueue
#[doc(alias = "OSFastMutexQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastMutexQueue {
    pub head: *mut FastMutex,
    pub tail: *mut FastMutex,
}

/// OSFastMutex
#[doc(alias = "OSFastMutex")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastMutex {
    /// [FAST_MUTEX_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    pub in_contended_queue: ffi::c_bool,
    pub contended_queue: ThreadSimpleQueue,
    pub owned_link: FastMutexLink,
    /// Owning thread, with bit 0 set while contended.
    pub lock: u32,
    /// Recursion count.
    pub lock_count: i32,
    pub contended_link: FastMutexLink,
}

unsafe extern "C" {
    /// OSFastMutex_Init
    #[doc(alias = "OSFastMutex_Init")]
    #[link_name = "OSFastMutex_Init"]
    pub unsafe fn init(mutex: *mut FastMutex, name: *const ffi::c_char);

    /// OSFastMutex_Lock
    #[doc(alias = "OSFastMutex_Lock")]
    #[link_name = "OSFastMutex_Lock"]
    pub unsafe fn lock(mutex: *mut FastMutex);

    /// OSFastMutex_TryLock
    #[doc(alias = "OSFastMutex_TryLock")]
    #[link_name = "OSFastMutex_TryLock"]
    pub unsafe fn try_lock(mutex: *mut FastMutex) -> ffi::c_bool;

    /// OSFastMutex_Unlock
    #[doc(alias = "OSFastMutex_Unlock")]
    #[link_name = "OSFastMutex_Unlock"]
    pub unsafe fn unlock(mutex: *mut FastMutex);
}
