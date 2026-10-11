//! Fast condition variable
//!
//! Used together with a [FastMutex].

use super::fast_mutex::FastMutex;
use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_FAST_CONDITION_TAG
///
/// Value of [FastCondition::tag].
pub const FAST_CONDITION_TAG: u32 = 0x664E6456;

/// OSFastCondition
#[doc(alias = "OSFastCondition")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastCondition {
    /// [FAST_CONDITION_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    pub queue: ThreadQueue,
}

unsafe extern "C" {
    /// OSFastCond_Init
    #[doc(alias = "OSFastCond_Init")]
    #[link_name = "OSFastCond_Init"]
    pub unsafe fn init(condition: *mut FastCondition, name: *const ffi::c_char);

    /// OSFastCond_Wait
    ///
    /// Unlocks `mutex`, waits for a signal and locks `mutex` again.
    #[doc(alias = "OSFastCond_Wait")]
    #[link_name = "OSFastCond_Wait"]
    pub unsafe fn wait(condition: *mut FastCondition, mutex: *mut FastMutex);

    /// OSFastCond_Signal
    ///
    /// Wakes all waiting threads.
    #[doc(alias = "OSFastCond_Signal")]
    #[link_name = "OSFastCond_Signal"]
    pub unsafe fn signal(condition: *mut FastCondition);
}
