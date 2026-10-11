//! Alarm
//!
//! One-shot and periodic timers. Alarms are queued on the core they are set on.

use super::context::Context;
use super::thread_queue::ThreadQueue;
use super::time::Time;
use crate::ffi;

/// OS_ALARM_TAG
#[doc(alias = "OS_ALARM_TAG")]
pub const ALARM_TAG: u32 = 0x614C724D;

/// OS_ALARM_QUEUE_TAG
#[doc(alias = "OS_ALARM_QUEUE_TAG")]
pub const ALARM_QUEUE_TAG: u32 = 0x614C6D51;

/// OSAlarmCallback
#[doc(alias = "OSAlarmCallback")]
pub type AlarmCallback = Option<unsafe extern "C" fn(alarm: *mut Alarm, context: *mut Context)>;

/// OSAlarmQueue
#[doc(alias = "OSAlarmQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AlarmQueue {
    /// [ALARM_QUEUE_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk: ffi::unknown<4>,
    pub thread_queue: ThreadQueue,
    pub head: *mut Alarm,
    pub tail: *mut Alarm,
}

/// OSAlarmLink
#[doc(alias = "OSAlarmLink")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlarmLink {
    pub prev: *mut Alarm,
    pub next: *mut Alarm,
}

/// OSAlarm
#[doc(alias = "OSAlarm")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Alarm {
    /// [ALARM_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _unk0: ffi::unknown<4>,
    pub callback: AlarmCallback,
    /// Group for [cancel_group], see [set_tag].
    pub group: u32,
    _unk1: ffi::unknown<4>,
    pub next_fire: Time,
    pub link: AlarmLink,
    /// Only set for periodic alarms.
    pub period: Time,
    pub start: Time,
    pub user_data: *mut ffi::c_void,
    pub state: u32,
    /// Threads waiting in [wait].
    pub thread_queue: ThreadQueue,
    pub alarm_queue: *mut AlarmQueue,
    pub context: *mut Context,
}

unsafe extern "C" {
    /// OSCreateAlarm
    #[doc(alias = "OSCreateAlarm")]
    #[link_name = "OSCreateAlarm"]
    pub unsafe fn create(alarm: *mut Alarm);

    /// OSCreateAlarmEx
    #[doc(alias = "OSCreateAlarmEx")]
    #[link_name = "OSCreateAlarmEx"]
    pub unsafe fn create_ex(alarm: *mut Alarm, name: *const ffi::c_char);

    /// OSSetAlarm
    ///
    /// Fires once after `time` ticks.
    #[doc(alias = "OSSetAlarm")]
    #[link_name = "OSSetAlarm"]
    pub unsafe fn set(alarm: *mut Alarm, time: Time, callback: AlarmCallback) -> ffi::c_bool;

    /// OSSetPeriodicAlarm
    ///
    /// Fires at `start` and then every `period` ticks.
    #[doc(alias = "OSSetPeriodicAlarm")]
    #[link_name = "OSSetPeriodicAlarm"]
    pub unsafe fn set_periodic(
        alarm: *mut Alarm,
        start: Time,
        period: Time,
        callback: AlarmCallback,
    ) -> ffi::c_bool;

    /// OSSetAlarmTag
    #[doc(alias = "OSSetAlarmTag")]
    #[link_name = "OSSetAlarmTag"]
    pub unsafe fn set_tag(alarm: *mut Alarm, group: u32);

    /// OSSetAlarmUserData
    #[doc(alias = "OSSetAlarmUserData")]
    #[link_name = "OSSetAlarmUserData"]
    pub unsafe fn set_user_data(alarm: *mut Alarm, data: *mut ffi::c_void);

    /// OSGetAlarmUserData
    #[doc(alias = "OSGetAlarmUserData")]
    #[link_name = "OSGetAlarmUserData"]
    pub unsafe fn get_user_data(alarm: *const Alarm) -> *mut ffi::c_void;

    /// OSCancelAlarm
    #[doc(alias = "OSCancelAlarm")]
    #[link_name = "OSCancelAlarm"]
    pub unsafe fn cancel(alarm: *mut Alarm) -> ffi::c_bool;

    /// OSCancelAlarms
    ///
    /// Cancels all alarms with this group, see [set_tag].
    #[doc(alias = "OSCancelAlarms")]
    #[link_name = "OSCancelAlarms"]
    pub unsafe fn cancel_group(group: u32);

    /// OSWaitAlarm
    ///
    /// Blocks until the alarm fired or was cancelled. Returns false if cancelled.
    #[doc(alias = "OSWaitAlarm")]
    #[link_name = "OSWaitAlarm"]
    pub unsafe fn wait(alarm: *mut Alarm) -> ffi::c_bool;

    /// OSInitAlarmQueue
    #[doc(alias = "OSInitAlarmQueue")]
    #[link_name = "OSInitAlarmQueue"]
    pub unsafe fn init_queue(queue: *mut AlarmQueue);

    /// OSInitAlarmQueueEx
    #[doc(alias = "OSInitAlarmQueueEx")]
    #[link_name = "OSInitAlarmQueueEx"]
    pub unsafe fn init_queue_ex(queue: *mut AlarmQueue, name: *const ffi::c_char);
}
