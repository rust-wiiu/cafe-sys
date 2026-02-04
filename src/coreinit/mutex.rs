use super::{
    Link, Queue,
    thread::{Thread, ThreadQueue},
};
use crate::ffi;
use core::ptr;

/// OSMutex
#[repr(C)]
#[derive(Clone)]
pub struct Mutex {
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk: ffi::unknown<0x4>,
    pub queue: ThreadQueue,
    pub owner: *const Thread,
    pub count: i32,
    pub link: MutexLink,
}

impl Mutex {
    pub const TAG: u32 = 0x6D557458;

    pub const fn new() -> Self {
        Self {
            tag: 0,
            name: ptr::null(),
            _unk: ffi::unknown::zero(),
            queue: ThreadQueue::new(),
            owner: ptr::null_mut(),
            count: 0,
            link: MutexLink::new(),
        }
    }
}

/// OSMutexQueue
pub type MutexQueue = Queue<Mutex>;

/// OSMutexLink
pub type MutexLink = Link<Mutex>;

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSInitMutex
    #[link_name = "OSInitMutex"]
    pub unsafe fn init(mutex: *mut Mutex);

    /// OSInitMutexEx
    #[link_name = "OSInitMutexEx"]
    pub unsafe fn init_ex(mutex: *mut Mutex, name: *const ffi::c_char);

    /// OSLockMutex
    #[link_name = "OSLockMutex"]
    pub unsafe fn lock(mutex: *mut Mutex);

    /// OSTryLockMutex
    #[link_name = "OSTryLockMutex"]
    pub unsafe fn try_lock(mutex: *mut Mutex) -> ffi::c_bool;

    /// OSUnlockMutex
    #[link_name = "OSUnlockMutex"]
    pub unsafe fn unlock(mutex: *mut Mutex);
}
