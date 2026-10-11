//! GX2 events and timestamps

use super::enums::{CompareFunction, EventType};
use crate::ffi;

/// GX2PipeEvent
#[doc(alias = "GX2PipeEvent")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipeEvent(pub u32);

impl PipeEvent {
    pub const TOP: Self = Self(0);
    pub const BOTTOM: Self = Self(1);
    pub const BOTTOM_AFTER_FLUSH: Self = Self(2);
}

/// GX2DisplayListOverrunData
///
/// Passed for [EventType::DISPLAY_LIST_OVERRUN]. Set `new_list` and `new_size` to continue in a new list.
#[doc(alias = "GX2DisplayListOverrunData")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayListOverrunData {
    pub old_list: *mut ffi::c_void,
    pub old_size: u32,
    pub new_list: *mut ffi::c_void,
    pub new_size: u32,
    _unk: ffi::unknown<8>,
}

/// GX2EventCallbackFunction
#[doc(alias = "GX2EventCallbackFunction", alias = "GX2CallbackFunction")]
pub type EventCallback =
    Option<unsafe extern "C" fn(event: EventType, user_data: *mut ffi::c_void)>;

unsafe extern "C" {
    /// GX2DrawDone
    ///
    /// Flushes and waits until the GPU is idle. Returns false on timeout.
    #[doc(alias = "GX2DrawDone")]
    #[link_name = "GX2DrawDone"]
    pub unsafe fn draw_done() -> ffi::c_bool;

    /// GX2WaitForVsync
    #[doc(alias = "GX2WaitForVsync")]
    #[link_name = "GX2WaitForVsync"]
    pub unsafe fn wait_for_vsync();

    /// GX2WaitForFlip
    #[doc(alias = "GX2WaitForFlip")]
    #[link_name = "GX2WaitForFlip"]
    pub unsafe fn wait_for_flip();

    /// GX2SetEventCallback
    #[doc(alias = "GX2SetEventCallback")]
    #[link_name = "GX2SetEventCallback"]
    pub unsafe fn set_event_callback(
        event: EventType,
        callback: EventCallback,
        user_data: *mut ffi::c_void,
    );

    /// GX2GetEventCallback
    #[doc(alias = "GX2GetEventCallback")]
    #[link_name = "GX2GetEventCallback"]
    pub unsafe fn get_event_callback(
        event: EventType,
        callback: *mut EventCallback,
        user_data: *mut *mut ffi::c_void,
    );

    /// GX2GetRetiredTimeStamp
    ///
    /// Timestamp of the last command buffer the GPU finished.
    #[doc(alias = "GX2GetRetiredTimeStamp")]
    #[link_name = "GX2GetRetiredTimeStamp"]
    pub unsafe fn get_retired_time_stamp() -> u64;

    /// GX2GetLastSubmittedTimeStamp
    #[doc(alias = "GX2GetLastSubmittedTimeStamp")]
    #[link_name = "GX2GetLastSubmittedTimeStamp"]
    pub unsafe fn get_last_submitted_time_stamp() -> u64;

    /// GX2WaitTimeStamp
    ///
    /// Waits until the GPU retired `time_stamp`. Returns false on timeout.
    #[doc(alias = "GX2WaitTimeStamp")]
    #[link_name = "GX2WaitTimeStamp"]
    pub unsafe fn wait_time_stamp(time_stamp: u64) -> ffi::c_bool;

    /// GX2SubmitUserTimeStamp
    ///
    /// Has the GPU write `value` to `buffer` at the `when` stage, optionally triggering the user timestamp event.
    #[doc(alias = "GX2SubmitUserTimeStamp")]
    #[link_name = "GX2SubmitUserTimeStamp"]
    pub unsafe fn submit_user_time_stamp(
        buffer: *mut u64,
        value: u64,
        when: PipeEvent,
        trigger_interrupt: ffi::c_bool,
    );

    /// GX2SetGPUFence
    ///
    /// Makes the GPU wait until `*fence & mask` compares true against `value`.
    #[doc(alias = "GX2SetGPUFence")]
    #[link_name = "GX2SetGPUFence"]
    pub unsafe fn set_gpu_fence(fence: *mut u32, mask: u32, op: CompareFunction, value: u32);
}
