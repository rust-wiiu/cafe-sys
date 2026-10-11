//! Thread queue

use super::thread::Thread;
use crate::ffi;

/// OSThreadLink
#[doc(alias = "OSThreadLink")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadLink {
    pub next: *mut Thread,
    pub prev: *mut Thread,
}

/// OSThreadQueue
#[doc(alias = "OSThreadQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadQueue {
    pub head: *mut Thread,
    pub tail: *mut Thread,
    /// Object containing this queue.
    pub parent: *mut ffi::c_void,
    _reserved: ffi::unknown<4>,
}

/// OSThreadSimpleQueue
#[doc(alias = "OSThreadSimpleQueue", alias = "OSThreadSmallQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadSimpleQueue {
    pub head: *mut Thread,
    pub tail: *mut Thread,
}

impl Default for ThreadLink {
    fn default() -> Self {
        Self {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
        }
    }
}

impl Default for ThreadQueue {
    fn default() -> Self {
        Self {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            parent: core::ptr::null_mut(),
            _reserved: ffi::unknown::zero(),
        }
    }
}

impl Default for ThreadSimpleQueue {
    fn default() -> Self {
        Self {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
        }
    }
}

unsafe extern "C" {
    /// OSInitThreadQueue
    #[doc(alias = "OSInitThreadQueue")]
    #[link_name = "OSInitThreadQueue"]
    pub unsafe fn init(queue: *mut ThreadQueue);

    /// OSInitThreadQueueEx
    #[doc(alias = "OSInitThreadQueueEx")]
    #[link_name = "OSInitThreadQueueEx"]
    pub unsafe fn init_ex(queue: *mut ThreadQueue, parent: *mut ffi::c_void);
}
