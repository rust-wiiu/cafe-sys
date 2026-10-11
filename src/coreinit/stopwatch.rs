//! Stopwatch
//!
//! Accumulating timers, with a spin lock or atomic.

use super::spinlock::SpinLock;
use super::time::Time;
use crate::ffi;

/// OSStopwatch
#[doc(alias = "OSStopwatch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Stopwatch {
    pub lock: SpinLock,
    pub name: *const ffi::c_char,
    /// Number of stops.
    pub hit_count: u32,
    /// Time from the first start to the last stop.
    pub total_time: Time,
    /// Shortest time between stops.
    pub min_time: Time,
    /// Longest time between stops.
    pub max_time: Time,
    pub start_time: Time,
    pub running: ffi::c_bool,
    _pad: ffi::unknown<4>,
}

/// OSStopWatchAtomic
#[doc(alias = "OSStopWatchAtomic")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StopWatchAtomic {
    pub start_time: Time,
    pub total_time: Time,
}

const _: () = {
    assert!(size_of::<StopWatchAtomic>() == 0x10);
};

unsafe extern "C" {
    /// OSInitStopwatch
    #[doc(alias = "OSInitStopwatch")]
    #[link_name = "OSInitStopwatch"]
    pub unsafe fn init(stopwatch: *mut Stopwatch, name: *const ffi::c_char);

    /// OSResetStopwatch
    #[doc(alias = "OSResetStopwatch")]
    #[link_name = "OSResetStopwatch"]
    pub unsafe fn reset(stopwatch: *mut Stopwatch);

    /// OSStartStopwatch
    #[doc(alias = "OSStartStopwatch")]
    #[link_name = "OSStartStopwatch"]
    pub unsafe fn start(stopwatch: *mut Stopwatch);

    /// OSStopStopwatch
    #[doc(alias = "OSStopStopwatch")]
    #[link_name = "OSStopStopwatch"]
    pub unsafe fn stop(stopwatch: *mut Stopwatch);

    /// OSCheckStopwatch
    ///
    /// Returns the total time, including the running interval.
    #[doc(alias = "OSCheckStopwatch")]
    #[link_name = "OSCheckStopwatch"]
    pub unsafe fn check(stopwatch: *mut Stopwatch) -> Time;

    /// OSDumpStopwatch
    ///
    /// Prints the statistics.
    #[doc(alias = "OSDumpStopwatch")]
    #[link_name = "OSDumpStopwatch"]
    pub unsafe fn dump(stopwatch: *mut Stopwatch);

    /// OSStopWatchStart
    #[doc(alias = "OSStopWatchStart")]
    #[link_name = "OSStopWatchStart"]
    pub unsafe fn atomic_start(stopwatch: *mut StopWatchAtomic) -> Time;

    /// OSStopWatchStop
    #[doc(alias = "OSStopWatchStop")]
    #[link_name = "OSStopWatchStop"]
    pub unsafe fn atomic_stop(stopwatch: *mut StopWatchAtomic) -> Time;

    /// OSStopWatchLap
    #[doc(alias = "OSStopWatchLap")]
    #[link_name = "OSStopWatchLap"]
    pub unsafe fn atomic_lap(stopwatch: *mut StopWatchAtomic) -> Time;

    /// OSStopWatchReset
    #[doc(alias = "OSStopWatchReset")]
    #[link_name = "OSStopWatchReset"]
    pub unsafe fn atomic_reset(stopwatch: *mut StopWatchAtomic) -> Time;
}
