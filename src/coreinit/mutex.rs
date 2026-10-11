//! Mutex
//!
//! Recursive mutex with priority inheritance.

use super::thread::Thread;
use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_MUTEX_TAG
///
/// Value of [Mutex::tag].
pub const MUTEX_TAG: u32 = 0x6D557458;

/// OSMutexLink
#[doc(alias = "OSMutexLink")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutexLink {
    pub next: *mut Mutex,
    pub prev: *mut Mutex,
}

/// OSMutexQueue
#[doc(alias = "OSMutexQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutexQueue {
    pub head: *mut Mutex,
    pub tail: *mut Mutex,
    pub parent: *mut ffi::c_void,
    _reserved: ffi::unknown<4>,
}

/// OSMutex
#[doc(alias = "OSMutex")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mutex {
    /// [MUTEX_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    pub queue: ThreadQueue,
    pub owner: *mut Thread,
    /// Recursion count.
    pub count: i32,
    pub link: MutexLink,
}

unsafe extern "C" {
    /// OSInitMutex
    #[doc(alias = "OSInitMutex")]
    #[link_name = "OSInitMutex"]
    pub unsafe fn init(mutex: *mut Mutex);

    /// OSInitMutexEx
    ///
    /// `name` is not copied.
    #[doc(alias = "OSInitMutexEx")]
    #[link_name = "OSInitMutexEx"]
    pub unsafe fn init_ex(mutex: *mut Mutex, name: *const ffi::c_char);

    /// OSLockMutex
    #[doc(alias = "OSLockMutex")]
    #[link_name = "OSLockMutex"]
    pub unsafe fn lock(mutex: *mut Mutex);

    /// OSTryLockMutex
    #[doc(alias = "OSTryLockMutex")]
    #[link_name = "OSTryLockMutex"]
    pub unsafe fn try_lock(mutex: *mut Mutex) -> ffi::c_bool;

    /// OSUnlockMutex
    #[doc(alias = "OSUnlockMutex")]
    #[link_name = "OSUnlockMutex"]
    pub unsafe fn unlock(mutex: *mut Mutex);
}
