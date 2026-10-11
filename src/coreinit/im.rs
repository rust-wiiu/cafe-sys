//! IM
//!
//! Input manager: inactivity timers, dimming and auto power-down parameters.

use super::ios::{self, IoctlVec};
use crate::ffi;

/// IMError
#[doc(alias = "IMError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// IMPadType
#[doc(alias = "IMPadType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PadType(pub u32);

impl PadType {
    pub const NONE: Self = Self(0);
    pub const WII_REMOTE: Self = Self(1);
    pub const WIIU_PRO_CONTROLLER: Self = Self(2);
    pub const WII_REMOTE_EXTENSION: Self = Self(3);
    pub const WIIU_GAMEPAD: Self = Self(4);
}

/// IMDeviceState
#[doc(alias = "IMDeviceState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceState(pub u32);

impl DeviceState {
    pub const CLEAR: Self = Self(0);
    pub const INACTIVE: Self = Self(1);
    pub const ACTIVE: Self = Self(2);
    pub const HOME: Self = Self(3);
    pub const POWER: Self = Self(4);
    pub const SYNC: Self = Self(5);
}

/// IMParameter
#[doc(alias = "IMParameter")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parameter(pub u32);

impl Parameter {
    pub const INACTIVE_SECONDS: Self = Self(0);
    pub const DIM_ENABLED: Self = Self(1);
    pub const DIM_PERIOD: Self = Self(2);
    pub const APD_ENABLED: Self = Self(3);
    pub const APD_PERIOD: Self = Self(4);
    pub const RESET_ENABLE: Self = Self(5);
    pub const RESET_SECONDS: Self = Self(6);
    pub const POWER_OFF_ENABLE: Self = Self(7);
    pub const APD_OCCURRED: Self = Self(8);
    pub const DIM_ENABLE_TV: Self = Self(9);
    pub const DIM_ENABLE_DRC: Self = Self(10);
}

/// IMTimer
#[doc(alias = "IMTimer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Timer(pub u32);

impl Timer {
    pub const DIM: Self = Self(0);
    pub const APD: Self = Self(1);
}

/// IMEventMask
#[doc(alias = "IMEventMask", alias = "IMEvent")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventMask(pub u32);

impl EventMask {
    pub const ACTIVE: Self = Self(1 << 0);
    pub const INACTIVE: Self = Self(1 << 1);
    pub const DIM: Self = Self(1 << 2);
    pub const UNDIM: Self = Self(1 << 3);
    pub const APD: Self = Self(1 << 4);
    pub const POWER: Self = Self(1 << 5);
    pub const HOME: Self = Self(1 << 6);
    pub const SYNC: Self = Self(1 << 7);
    pub const RESET: Self = Self(1 << 8);
    pub const CANCELLED: Self = Self(1 << 31);
}

impl core::ops::BitOr for EventMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// IMRequest
#[doc(alias = "IMRequest")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Request {
    pub args: [u8; 0x80],
    pub ioctl_vecs: [IoctlVec; 2],
    pub handle: ios::Handle,
    pub request: i32,
    pub async_callback: ios::AsyncCallback,
    pub async_callback_context: *mut ffi::c_void,
    pub copy_src: *mut ffi::c_void,
    pub copy_dst: *mut ffi::c_void,
    pub copy_size: u32,
}

/// IMHomeButtonParams
#[doc(alias = "IMHomeButtonParams")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HomeButtonParams {
    pub kind: PadType,
    pub index: i32,
}

/// IMParameters
#[doc(alias = "IMParameters")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parameters {
    pub inactive_seconds: u32,
    pub dim_enabled: u32,
    pub dim_seconds: u32,
    pub apd_enabled: u32,
    pub apd_seconds: u32,
}

/// IMDeviceStateEx
#[doc(alias = "IMDeviceStateEx")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceStateEx {
    pub state: DeviceState,
    pub params: HomeButtonParams,
}

const _: () = {
    assert!(size_of::<HomeButtonParams>() == 0x8);
    assert!(size_of::<Parameters>() == 0x14);
    assert!(size_of::<DeviceStateEx>() == 0xC);
};

unsafe extern "C" {
    /// IM_Open
    #[doc(alias = "IM_Open")]
    #[link_name = "IM_Open"]
    pub unsafe fn open() -> ios::Handle;

    /// IM_Close
    #[doc(alias = "IM_Close")]
    #[link_name = "IM_Close"]
    pub unsafe fn close(handle: ios::Handle) -> ios::Error;

    /// IM_GetParameters
    #[doc(alias = "IM_GetParameters")]
    #[link_name = "IM_GetParameters"]
    pub unsafe fn get_parameters(parameters: *mut Parameters) -> ios::Error;

    /// IM_GetParameter
    #[doc(alias = "IM_GetParameter")]
    #[link_name = "IM_GetParameter"]
    pub unsafe fn get_parameter(
        handle: ios::Handle,
        request: *mut Request,
        parameter: Parameter,
        value: *mut u32,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_SetParameter
    #[doc(alias = "IM_SetParameter")]
    #[link_name = "IM_SetParameter"]
    pub unsafe fn set_parameter(
        handle: ios::Handle,
        request: *mut Request,
        parameter: Parameter,
        value: u32,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_GetNvParameter
    #[doc(alias = "IM_GetNvParameter")]
    #[link_name = "IM_GetNvParameter"]
    pub unsafe fn get_nv_parameter(
        handle: ios::Handle,
        request: *mut Request,
        parameter: Parameter,
        value: *mut u32,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_GetNvParameterWithoutHandleAndItb
    #[doc(alias = "IM_GetNvParameterWithoutHandleAndItb")]
    #[link_name = "IM_GetNvParameterWithoutHandleAndItb"]
    pub unsafe fn get_nv_parameter_without_handle(
        parameter: Parameter,
        value: *mut u32,
    ) -> ios::Error;

    /// IM_GetRuntimeParameter
    #[doc(alias = "IM_GetRuntimeParameter")]
    #[link_name = "IM_GetRuntimeParameter"]
    pub unsafe fn get_runtime_parameter(parameter: Parameter, value: *mut u32) -> ios::Error;

    /// IM_SetRuntimeParameter
    #[doc(alias = "IM_SetRuntimeParameter")]
    #[link_name = "IM_SetRuntimeParameter"]
    pub unsafe fn set_runtime_parameter(parameter: Parameter, value: u32) -> ios::Error;

    /// IM_GetHomeButtonParams
    #[doc(alias = "IM_GetHomeButtonParams")]
    #[link_name = "IM_GetHomeButtonParams"]
    pub unsafe fn get_home_button_params(
        handle: ios::Handle,
        request: *mut Request,
        params: *mut HomeButtonParams,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_GetTimerRemaining
    #[doc(alias = "IM_GetTimerRemaining")]
    #[link_name = "IM_GetTimerRemaining"]
    pub unsafe fn get_timer_remaining(
        handle: ios::Handle,
        request: *mut Request,
        timer: Timer,
        seconds: *mut u32,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_GetTimerRemainingSeconds
    #[doc(alias = "IM_GetTimerRemainingSeconds")]
    #[link_name = "IM_GetTimerRemainingSeconds"]
    pub unsafe fn get_timer_remaining_seconds(timer: Timer, seconds: *mut u32) -> ios::Error;

    /// IM_GetEventNotify
    ///
    /// Waits for one of the events in `events`, which receives the events that occurred.
    #[doc(alias = "IM_GetEventNotify")]
    #[link_name = "IM_GetEventNotify"]
    pub unsafe fn get_event_notify(
        handle: ios::Handle,
        request: *mut Request,
        events: *mut EventMask,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_CancelGetEventNotify
    #[doc(alias = "IM_CancelGetEventNotify")]
    #[link_name = "IM_CancelGetEventNotify"]
    pub unsafe fn cancel_get_event_notify(
        handle: ios::Handle,
        request: *mut Request,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_SetDeviceState
    #[doc(alias = "IM_SetDeviceState")]
    #[link_name = "IM_SetDeviceState"]
    pub unsafe fn set_device_state(
        handle: ios::Handle,
        request: *mut Request,
        state: DeviceState,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IM_SetDeviceStateEx
    #[doc(alias = "IM_SetDeviceStateEx")]
    #[link_name = "IM_SetDeviceStateEx"]
    pub unsafe fn set_device_state_ex(
        handle: ios::Handle,
        request: *mut Request,
        state: *mut DeviceStateEx,
        callback: ios::AsyncCallback,
        context: *mut ffi::c_void,
    ) -> ios::Error;

    /// IMGetAPDPeriod
    ///
    /// Seconds of inactivity before auto power-down.
    #[doc(alias = "IMGetAPDPeriod")]
    #[link_name = "IMGetAPDPeriod"]
    pub unsafe fn get_apd_period(seconds: *mut u32) -> Error;
}
