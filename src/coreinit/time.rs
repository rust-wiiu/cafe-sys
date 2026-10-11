//! Time
//!
//! System time and ticks. One tick is a quarter of the bus clock, see [SystemInfo][super::system_info::SystemInfo].

use crate::ffi;

/// OSTime
///
/// Ticks. Absolute times count from 2000-01-01.
#[doc(alias = "OSTime")]
pub type Time = i64;

/// OSTick
///
/// Lower 32 bits of [Time].
#[doc(alias = "OSTick")]
pub type Tick = i32;

/// OSCalendarTime
#[doc(alias = "OSCalendarTime")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    /// Seconds after the minute, 0-59.
    pub sec: i32,
    /// Minutes after the hour, 0-59.
    pub min: i32,
    /// Hours since midnight, 0-23.
    pub hour: i32,
    /// Day of the month, 1-31.
    pub mday: i32,
    /// Months since January, 0-11.
    pub mon: i32,
    /// Years AD.
    pub year: i32,
    /// Days since Sunday, 0-6.
    pub wday: i32,
    /// Days since January 1, 0-365.
    pub yday: i32,
    /// Milliseconds after the second, 0-999.
    pub msec: i32,
    /// Microseconds after the millisecond, 0-999.
    pub usec: i32,
}

const _: () = {
    assert!(size_of::<DateTime>() == 0x28);
};

unsafe extern "C" {
    /// OSGetTime
    ///
    /// Current date and time.
    #[doc(alias = "OSGetTime")]
    #[link_name = "OSGetTime"]
    pub unsafe fn get_time() -> Time;

    /// OSGetSystemTime
    ///
    /// Time since the system started.
    #[doc(alias = "OSGetSystemTime")]
    #[link_name = "OSGetSystemTime"]
    pub unsafe fn get_system_time() -> Time;

    /// OSGetTick
    #[doc(alias = "OSGetTick")]
    #[link_name = "OSGetTick"]
    pub unsafe fn get_tick() -> Tick;

    /// OSGetSystemTick
    #[doc(alias = "OSGetSystemTick")]
    #[link_name = "OSGetSystemTick"]
    pub unsafe fn get_system_tick() -> Tick;

    /// OSCalendarTimeToTicks
    #[doc(alias = "OSCalendarTimeToTicks")]
    #[link_name = "OSCalendarTimeToTicks"]
    pub unsafe fn calendar_time_to_ticks(date_time: *const DateTime) -> Time;

    /// OSTicksToCalendarTime
    #[doc(alias = "OSTicksToCalendarTime")]
    #[link_name = "OSTicksToCalendarTime"]
    pub unsafe fn ticks_to_calendar_time(time: Time, date_time: *mut DateTime);

    /// OSWaitMicroseconds
    ///
    /// Busy-waits.
    #[doc(alias = "OSWaitMicroseconds")]
    #[link_name = "OSWaitMicroseconds"]
    pub unsafe fn wait_microseconds(microseconds: i64);

    /// __OSSetAbsoluteSystemTime
    #[doc(alias = "__OSSetAbsoluteSystemTime")]
    #[link_name = "__OSSetAbsoluteSystemTime"]
    pub unsafe fn set_absolute_system_time(time: Time) -> ffi::c_bool;
}
