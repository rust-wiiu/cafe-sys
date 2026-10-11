//! GameCube Controller Adapter (HPAD)

use crate::ffi;

/// HPAD error code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// HPADChan
///
/// Controller port, 4 per adapter.
#[doc(alias = "HPADChan")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    #[default]
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
}

/// HPADGGGGChan
#[doc(alias = "HPADGGGGChan")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdapterChannel {
    #[default]
    A0 = 0,
    A1 = 1,
}

/// HPADButtons
#[doc(alias = "HPADButtons")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Buttons(pub u32);

impl Buttons {
    pub const A: Self = Self(0x0001);
    pub const B: Self = Self(0x0002);
    pub const X: Self = Self(0x0004);
    pub const Y: Self = Self(0x0008);
    pub const LEFT: Self = Self(0x0010);
    pub const RIGHT: Self = Self(0x0020);
    pub const DOWN: Self = Self(0x0040);
    pub const UP: Self = Self(0x0080);
    pub const START: Self = Self(0x0100);
    pub const Z: Self = Self(0x0200);
    pub const R: Self = Self(0x0400);
    pub const L: Self = Self(0x0800);
    pub const STICK_EMULATION_LEFT: Self = Self(0x1000);
    pub const STICK_EMULATION_RIGHT: Self = Self(0x2000);
    pub const STICK_EMULATION_DOWN: Self = Self(0x4000);
    pub const STICK_EMULATION_UP: Self = Self(0x8000);
    pub const SUBSTICK_EMULATION_LEFT: Self = Self(0x10000);
    pub const SUBSTICK_EMULATION_RIGHT: Self = Self(0x20000);
    pub const SUBSTICK_EMULATION_DOWN: Self = Self(0x40000);
    pub const SUBSTICK_EMULATION_UP: Self = Self(0x80000);
}

/// HPADMotorCommand
#[doc(alias = "HPADMotorCommand")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotorCommand {
    Stop = 0,
    Rumble = 1,
    StopHard = 2,
}

/// HPADStatus
#[doc(alias = "HPADStatus")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status {
    pub hold: Buttons,
    /// Pressed since the last sample.
    pub trigger: Buttons,
    /// Released since the last sample.
    pub release: Buttons,
    pub stick_x: i8,
    pub stick_y: i8,
    /// C-Stick.
    pub substick_x: i8,
    pub substick_y: i8,
    pub trigger_l: u8,
    pub trigger_r: u8,
    pub status: u8,
    _pad: ffi::unknown<1>,
    pub error: Error,
}

/// HPADGGGGStatus
#[doc(alias = "HPADGGGGStatus")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdapterStatus {
    pub connected: ffi::c_bool,
    pub power_supply_connected: ffi::c_bool,
    /// The adapter is sending samples.
    pub active: ffi::c_bool,
}

/// HPADConnectCallback
#[doc(alias = "HPADConnectCallback")]
pub type ConnectCallback = Option<unsafe extern "C" fn(chan: Channel, status: i32)>;

/// HPADGGGGConnectCallback
#[doc(alias = "HPADGGGGConnectCallback")]
pub type AdapterConnectCallback =
    Option<unsafe extern "C" fn(chan: AdapterChannel, connected: ffi::c_bool)>;

/// HPADPowerSupplyCallback
#[doc(alias = "HPADPowerSupplyCallback")]
pub type PowerSupplyCallback =
    Option<unsafe extern "C" fn(chan: AdapterChannel, connected: ffi::c_bool)>;

/// HPADSamplingCallback
#[doc(alias = "HPADSamplingCallback")]
pub type SamplingCallback = Option<unsafe extern "C" fn(chan: AdapterChannel)>;

/// HPADResetCallback
#[doc(alias = "HPADResetCallback")]
pub type ResetCallback = Option<unsafe extern "C" fn(chan: AdapterChannel, status: i32)>;

unsafe extern "C" {
    /// HPADInit
    #[doc(alias = "HPADInit")]
    #[link_name = "HPADInit"]
    pub unsafe fn init() -> Error;

    /// HPADShutdown
    #[doc(alias = "HPADShutdown")]
    #[link_name = "HPADShutdown"]
    pub unsafe fn deinit() -> Error;

    /// HPADRead
    ///
    /// Always fills 16 buffers, regardless of `count`. Returns the number of buffers read, or a negative error.
    #[doc(alias = "HPADRead")]
    #[link_name = "__rplwrap_HPADRead"]
    pub unsafe fn read(chan: Channel, buffers: *mut [Status; 16], count: i32) -> i32;

    /// HPADControlMotor
    #[doc(alias = "HPADControlMotor")]
    #[link_name = "HPADControlMotor"]
    pub unsafe fn control_motor(chan: Channel, command: MotorCommand) -> Error;

    /// HPADRecalibrate
    ///
    /// Invalidates the calibration data of the channel.
    #[doc(alias = "HPADRecalibrate")]
    #[link_name = "HPADRecalibrate"]
    pub unsafe fn recalibrate(chan: Channel) -> Error;

    /// HPADSetConnectCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "HPADSetConnectCallback")]
    #[link_name = "HPADSetConnectCallback"]
    pub unsafe fn set_connect_callback(chan: Channel, callback: ConnectCallback)
    -> ConnectCallback;

    /// HPADGetGGGGStatus
    #[doc(alias = "HPADGetGGGGStatus")]
    #[link_name = "HPADGetGGGGStatus"]
    pub unsafe fn get_adapter_status(chan: AdapterChannel, status: *mut AdapterStatus) -> Error;

    /// HPADSetGgggConnectCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "HPADSetGgggConnectCallback")]
    #[link_name = "HPADSetGgggConnectCallback"]
    pub unsafe fn set_adapter_connect_callback(
        chan: AdapterChannel,
        callback: AdapterConnectCallback,
    ) -> AdapterConnectCallback;

    /// HPADSetPowerSupplyCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "HPADSetPowerSupplyCallback")]
    #[link_name = "HPADSetPowerSupplyCallback"]
    pub unsafe fn set_power_supply_callback(
        chan: AdapterChannel,
        callback: PowerSupplyCallback,
    ) -> PowerSupplyCallback;

    /// HPADSetSamplingCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "HPADSetSamplingCallback")]
    #[link_name = "HPADSetSamplingCallback"]
    pub unsafe fn set_sampling_callback(
        chan: AdapterChannel,
        callback: SamplingCallback,
    ) -> SamplingCallback;

    /// HPADResetDevice
    #[doc(alias = "HPADResetDevice")]
    #[link_name = "HPADResetDevice"]
    pub unsafe fn reset_device(chan: AdapterChannel, callback: ResetCallback) -> Error;

    /// BETA_DEBUG_DUMP
    #[doc(alias = "BETA_DEBUG_DUMP")]
    #[link_name = "BETA_DEBUG_DUMP"]
    pub unsafe fn beta_debug_dump();

    /// BETA_DEBUG_GET_QUEUE_SIZE
    ///
    /// Size of the command queue.
    #[doc(alias = "BETA_DEBUG_GET_QUEUE_SIZE")]
    #[link_name = "BETA_DEBUG_GET_QUEUE_SIZE"]
    pub unsafe fn beta_debug_get_queue_size(chan: AdapterChannel) -> i32;

    /// BETA_DEBUG_SEND_REPT_ID
    #[doc(alias = "BETA_DEBUG_SEND_REPT_ID")]
    #[link_name = "BETA_DEBUG_SEND_REPT_ID"]
    pub unsafe fn beta_debug_send_rept_id(chan: AdapterChannel, rept_id: u32);

    /// BETA_DEBUG_GET_RAW_DATA
    #[doc(alias = "BETA_DEBUG_GET_RAW_DATA")]
    #[link_name = "BETA_DEBUG_GET_RAW_DATA"]
    pub unsafe fn beta_debug_get_raw_data();

    /// BETA_DEBUG_SET_DUMP_MODE
    #[doc(alias = "BETA_DEBUG_SET_DUMP_MODE")]
    #[link_name = "BETA_DEBUG_SET_DUMP_MODE"]
    pub unsafe fn beta_debug_set_dump_mode();
}

imports_section!(
    "nn_hpad",
    [
        "BETA_DEBUG_DUMP",
        "BETA_DEBUG_GET_QUEUE_SIZE",
        "BETA_DEBUG_GET_RAW_DATA",
        "BETA_DEBUG_SEND_REPT_ID",
        "BETA_DEBUG_SET_DUMP_MODE",
        "HPADControlMotor",
        "HPADGetGGGGStatus",
        "HPADInit",
        "HPADRecalibrate",
        "HPADResetDevice",
        "HPADSetConnectCallback",
        "HPADSetGgggConnectCallback",
        "HPADSetPowerSupplyCallback",
        "HPADSetSamplingCallback",
        "HPADShutdown",
        "__rplwrap_HPADRead"
    ],
    []
);
