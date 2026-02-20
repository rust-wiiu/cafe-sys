/// OSTime
pub type Time = u64;

/// OSTicks
pub type Ticks = u32;

/// OSCalendarTime
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DateTime {
    /// Seconds after the minute [0, 59].
    pub sec: i32,
    /// Minutes after the hour [0, 59].
    pub min: i32,
    /// Hours since midnight [0, 23].
    pub hour: i32,
    /// Day of the month [1, 31].
    pub mday: i32,
    /// Month since January [0, 11].
    pub mon: i32,
    /// Years in AD [1, ...].
    pub year: i32,
    /// Days since Sunday [0, 6].
    pub wday: i32,
    /// Days since January 1 [0, 365].
    pub yday: i32,
    /// Milliseconds after the second [0, 999].
    pub msec: i32,
    /// Microseconds after the millisecond [0, 999].
    pub usec: i32,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// Converts time from ticks to datetime.
    ///
    /// # Symbol
    ///
    /// OSCalendarTimeToTicks
    #[link_name = "OSCalendarTimeToTicks"]
    pub unsafe fn datetime_to_time(calender: *const DateTime) -> Time;

    /// Converts time from callender time to ticks (Hz)
    ///
    /// # Symbol
    ///
    /// OSTicksToCalendarTime
    #[link_name = "OSTicksToCalendarTime"]
    pub unsafe fn time_to_datetime(time: Time, calender: *mut DateTime) -> Ticks;

    /// The current system uptime.
    ///
    /// The OS sets this valus to the `00:00 AM January 1, 2000` on boot. Application changes might cause reboots.
    ///
    /// # Symbol
    ///
    /// OSGetSystemTime
    #[link_name = "OSGetSystemTime"]
    pub unsafe fn system_uptime() -> Time;

    /// The current local time.
    ///
    /// Local time is the time configured in the system settings.
    ///
    /// # Symbol
    ///
    /// OSGetTime
    #[link_name = "OSGetTime"]
    pub unsafe fn local_time() -> Time;
}
