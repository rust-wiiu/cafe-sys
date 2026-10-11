//! Semaphore

use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_SEMAPHORE_TAG
///
/// Value of [Semaphore::tag].
pub const SEMAPHORE_TAG: u32 = 0x73506852;

/// OSSemaphore
#[doc(alias = "OSSemaphore")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Semaphore {
    /// [SEMAPHORE_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    pub count: i32,
    pub queue: ThreadQueue,
}

unsafe extern "C" {
    /// OSInitSemaphore
    #[doc(alias = "OSInitSemaphore")]
    #[link_name = "OSInitSemaphore"]
    pub unsafe fn init(semaphore: *mut Semaphore, count: i32);

    /// OSInitSemaphoreEx
    #[doc(alias = "OSInitSemaphoreEx")]
    #[link_name = "OSInitSemaphoreEx"]
    pub unsafe fn init_ex(semaphore: *mut Semaphore, count: i32, name: *const ffi::c_char);

    /// OSWaitSemaphore
    ///
    /// Waits until the count is positive, then decrements it. Returns the previous count.
    #[doc(alias = "OSWaitSemaphore")]
    #[link_name = "OSWaitSemaphore"]
    pub unsafe fn wait(semaphore: *mut Semaphore) -> i32;

    /// OSTryWaitSemaphore
    ///
    /// Same as [wait] without blocking. Returns the previous count.
    #[doc(alias = "OSTryWaitSemaphore")]
    #[link_name = "OSTryWaitSemaphore"]
    pub unsafe fn try_wait(semaphore: *mut Semaphore) -> i32;

    /// OSSignalSemaphore
    ///
    /// Increments the count. Returns the previous count.
    #[doc(alias = "OSSignalSemaphore")]
    #[link_name = "OSSignalSemaphore"]
    pub unsafe fn signal(semaphore: *mut Semaphore) -> i32;

    /// OSGetSemaphoreCount
    #[doc(alias = "OSGetSemaphoreCount")]
    #[link_name = "OSGetSemaphoreCount"]
    pub unsafe fn get_count(semaphore: *mut Semaphore) -> i32;
}
