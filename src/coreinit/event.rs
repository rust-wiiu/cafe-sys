//! Event

use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_EVENT_TAG
///
/// Value of [Event::tag].
pub const EVENT_TAG: u32 = 0x65566E54;

/// OSEventMode
#[doc(alias = "OSEventMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventMode(pub i32);

impl EventMode {
    /// Stays signaled until [reset].
    pub const MANUAL: Self = Self(0);
    /// Resets after waking a single thread.
    pub const AUTO: Self = Self(1);
}

/// OSEvent
#[doc(alias = "OSEvent")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Event {
    /// [EVENT_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    /// Signaled state.
    pub value: ffi::c_bool,
    pub queue: ThreadQueue,
    pub mode: EventMode,
}

unsafe extern "C" {
    /// OSInitEvent
    #[doc(alias = "OSInitEvent")]
    #[link_name = "OSInitEvent"]
    pub unsafe fn init(event: *mut Event, value: ffi::c_bool, mode: EventMode);

    /// OSInitEventEx
    #[doc(alias = "OSInitEventEx")]
    #[link_name = "OSInitEventEx"]
    pub unsafe fn init_ex(
        event: *mut Event,
        value: ffi::c_bool,
        mode: EventMode,
        name: *const ffi::c_char,
    );

    /// OSSignalEvent
    ///
    /// Wakes one thread for [EventMode::AUTO], all threads for [EventMode::MANUAL].
    #[doc(alias = "OSSignalEvent")]
    #[link_name = "OSSignalEvent"]
    pub unsafe fn signal(event: *mut Event);

    /// OSSignalEventAll
    ///
    /// Wakes all waiting threads.
    #[doc(alias = "OSSignalEventAll")]
    #[link_name = "OSSignalEventAll"]
    pub unsafe fn signal_all(event: *mut Event);

    /// OSWaitEvent
    #[doc(alias = "OSWaitEvent")]
    #[link_name = "OSWaitEvent"]
    pub unsafe fn wait(event: *mut Event);

    /// OSWaitEventWithTimeout
    ///
    /// Returns false on timeout.
    #[doc(alias = "OSWaitEventWithTimeout")]
    #[link_name = "OSWaitEventWithTimeout"]
    pub unsafe fn wait_with_timeout(event: *mut Event, timeout_ns: i64) -> ffi::c_bool;

    /// OSResetEvent
    #[doc(alias = "OSResetEvent")]
    #[link_name = "OSResetEvent"]
    pub unsafe fn reset(event: *mut Event);
}
