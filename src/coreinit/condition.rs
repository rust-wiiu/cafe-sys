//! Condition variable
//!
//! Used together with a [Mutex].

use super::mutex::Mutex;
use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_CONDITION_TAG
///
/// Value of [Condition::tag].
pub const CONDITION_TAG: u32 = 0x634E6456;

/// OSCondition
#[doc(alias = "OSCondition", alias = "OSCond")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Condition {
    /// [CONDITION_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    pub queue: ThreadQueue,
}

unsafe extern "C" {
    /// OSInitCond
    #[doc(alias = "OSInitCond")]
    #[link_name = "OSInitCond"]
    pub unsafe fn init(condition: *mut Condition);

    /// OSInitCondEx
    #[doc(alias = "OSInitCondEx")]
    #[link_name = "OSInitCondEx"]
    pub unsafe fn init_ex(condition: *mut Condition, name: *const ffi::c_char);

    /// OSWaitCond
    ///
    /// Unlocks `mutex`, waits for a signal and locks `mutex` again.
    #[doc(alias = "OSWaitCond")]
    #[link_name = "OSWaitCond"]
    pub unsafe fn wait(condition: *mut Condition, mutex: *mut Mutex);

    /// OSSignalCond
    ///
    /// Wakes all waiting threads.
    #[doc(alias = "OSSignalCond")]
    #[link_name = "OSSignalCond"]
    pub unsafe fn signal(condition: *mut Condition);
}
