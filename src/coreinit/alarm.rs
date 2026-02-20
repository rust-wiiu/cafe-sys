use super::{Link, context::Context, thread::ThreadQueue, time::Time};
use crate::ffi;

/// OSAlarmCallback
pub type AlarmCallback = Option<unsafe extern "C" fn(alarm: *mut Alarm, context: *mut Context)>;

/// OSAlarmLink
pub type AlarmLink = Link<Alarm>;

/// OSAlarmQueue
#[repr(C)]
#[derive(Clone)]
pub struct AlarmQueue {
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk: ffi::unknown<0x4>,
    pub thread_queue: ThreadQueue,
    pub head: *mut Alarm,
    pub tail: *mut Alarm,
}

impl AlarmQueue {
    pub const TAG: u32 = 0x614C6D51;
}

/// OSAlarm
#[repr(C)]
#[derive(Clone)]
pub struct Alarm {
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk0: ffi::unknown<0x4>,
    pub callback: AlarmCallback,
    pub group: u32,
    _unk1: ffi::unknown<0x4>,
    pub next_fire: Time,
    pub link: AlarmLink,
    pub period: Time,
    pub start: Time,
    pub user_data: *mut ffi::c_void,
    pub state: u32,
    pub thread_queue: ThreadQueue,
    pub alarm_queue: *mut AlarmQueue,
    pub context: *mut Context,
}

impl Alarm {
    pub const TAG: u32 = 0x614C724D;
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSCancelAlarm
    #[link_name = "OSCancelAlarm"]
    pub unsafe fn cancel(alarm: *mut Alarm) -> ffi::c_bool;

    /// OSCancelAlarms
    #[link_name = "OSCancelAlarms"]
    pub unsafe fn cancel_group(group: u32);

    /// OSCreateAlarm
    #[link_name = "OSCreateAlarm"]
    pub unsafe fn create(alarm: *mut Alarm);

    /// OSCreateAlarmEx
    #[link_name = "OSCreateAlarmEx"]
    pub unsafe fn create_ex(alarm: *mut Alarm, name: *const ffi::c_char);

    /// OSGetAlarmUserData
    #[link_name = "OSGetAlarmUserData"]
    pub unsafe fn get_user_data(alarm: *mut Alarm) -> *mut ffi::c_void;

    /// OSInitAlarmQueue
    #[link_name = "OSInitAlarmQueue"]
    pub unsafe fn init_queue(queue: *mut AlarmQueue);

    /// OSInitAlarmQueueEx
    #[link_name = "OSInitAlarmQueueEx"]
    pub unsafe fn init_queue_ex(queue: *mut AlarmQueue, name: *const ffi::c_char);

    /// OSSetAlarm
    #[link_name = "OSSetAlarm"]
    pub unsafe fn set(alarm: *mut Alarm, time: Time, callback: AlarmCallback) -> ffi::c_bool;

    /// OSSetPeriodicAlarm
    #[link_name = "OSSetPeriodicAlarm"]
    pub unsafe fn set_periodic(
        alarm: *mut Alarm,
        start: Time,
        interval: Time,
        callback: AlarmCallback,
    ) -> ffi::c_bool;

    /// OSSetAlarmTag
    #[link_name = "OSSetAlarmTag"]
    pub unsafe fn set_tag(alarm: *mut Alarm, group: u32);

    /// OSSetAlarmUserData
    #[link_name = "OSSetAlarmUserData"]
    pub unsafe fn set_user_data(alarm: *mut Alarm, data: *mut ffi::c_void);

    /// OSWaitAlarm
    #[link_name = "OSWaitAlarm"]
    pub unsafe fn wait(alarm: *mut Alarm) -> ffi::c_bool;
}
