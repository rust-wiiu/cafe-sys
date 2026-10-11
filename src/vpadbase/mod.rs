//! VPAD Base (vpadbase)
//!
//! Low-level GamePad driver used by `vpad`.

use crate::ffi;

/// VPADBASE_MAX_CONTROLLERS
#[doc(alias = "VPADBASE_MAX_CONTROLLERS")]
pub const MAX_CONTROLLERS: usize = 2;

/// VPADChan
#[doc(alias = "VPADChan")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    #[default]
    C0 = 0,
    C1 = 1,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryLevel(pub u8);

impl BatteryLevel {
    pub const CHARGE: Self = Self(0);
    pub const SHUTDOWN: Self = Self(1);
    pub const CRITICAL: Self = Self(2);
    pub const LOW: Self = Self(3);
    pub const MEDIUM: Self = Self(4);
    pub const HIGH: Self = Self(5);
    pub const MAX: Self = Self(6);
}

/// VPADIRCStatusFlags
#[doc(alias = "VPADIRCStatusFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrcStatusFlags(pub u32);

impl IrcStatusFlags {
    pub const HAS_DATA: Self = Self(1 << 0);
    pub const CONNECTED: Self = Self(1 << 1);
}

unsafe extern "C" {
    /// VPADBASEInit
    #[doc(alias = "VPADBASEInit")]
    #[link_name = "VPADBASEInit"]
    pub unsafe fn init();

    /// VPADBASEShutdown
    #[doc(alias = "VPADBASEShutdown")]
    #[link_name = "VPADBASEShutdown"]
    pub unsafe fn shutdown();

    /// VPADBASEIsInit
    #[doc(alias = "VPADBASEIsInit")]
    #[link_name = "VPADBASEIsInit"]
    pub unsafe fn is_init() -> ffi::c_bool;

    /// VPADBASEGetMotorOnRemainingCount
    #[doc(alias = "VPADBASEGetMotorOnRemainingCount")]
    #[link_name = "VPADBASEGetMotorOnRemainingCount"]
    pub unsafe fn get_motor_on_remaining_count(channel: Channel) -> i32;

    /// VPADBASESetMotorOnRemainingCount
    #[doc(alias = "VPADBASESetMotorOnRemainingCount")]
    #[link_name = "VPADBASESetMotorOnRemainingCount"]
    pub unsafe fn set_motor_on_remaining_count(channel: Channel, count: i32) -> i32;

    /// VPADBASESetSensorBarSetting
    #[doc(alias = "VPADBASESetSensorBarSetting")]
    #[link_name = "VPADBASESetSensorBarSetting"]
    pub unsafe fn set_sensor_bar_setting(channel: Channel, setting: i8);

    /// VPADBASEGetSensorBarSetting
    #[doc(alias = "VPADBASEGetSensorBarSetting")]
    #[link_name = "VPADBASEGetSensorBarSetting"]
    pub unsafe fn get_sensor_bar_setting(channel: Channel, setting: *mut i8);

    /// VPADBASEGetHeadphoneStatus
    #[doc(alias = "VPADBASEGetHeadphoneStatus")]
    #[link_name = "VPADBASEGetHeadphoneStatus"]
    pub unsafe fn get_headphone_status(channel: Channel) -> i32;

    /// VPADBASEGetGameControllerMode
    #[doc(alias = "VPADBASEGetGameControllerMode")]
    #[link_name = "VPADBASEGetGameControllerMode"]
    pub unsafe fn get_game_controller_mode(channel: Channel, mode: *mut i32);

    /// VPADBASESetGameControllerMode
    #[doc(alias = "VPADBASESetGameControllerMode")]
    #[link_name = "VPADBASESetGameControllerMode"]
    pub unsafe fn set_game_controller_mode(channel: Channel, mode: i32);

    /// VPADBASEGetPowerButtonPressStatus
    #[doc(alias = "VPADBASEGetPowerButtonPressStatus")]
    #[link_name = "VPADBASEGetPowerButtonPressStatus"]
    pub unsafe fn get_power_button_press_status(channel: Channel, tick: *mut u32, status: *mut u32);

    /// VPADBASESetPowerButtonPressStatus
    #[doc(alias = "VPADBASESetPowerButtonPressStatus")]
    #[link_name = "VPADBASESetPowerButtonPressStatus"]
    pub unsafe fn set_power_button_press_status(channel: Channel, tick: u32, status: u32);

    /// VPADBASESetPowerButtonDisableMode
    #[doc(alias = "VPADBASESetPowerButtonDisableMode")]
    #[link_name = "VPADBASESetPowerButtonDisableMode"]
    pub unsafe fn set_power_button_disable_mode(channel: Channel, mode: u32);

    /// VPADBASEGetTVMenuStatus
    ///
    /// Whether the TV Menu is open.
    #[doc(alias = "VPADBASEGetTVMenuStatus")]
    #[link_name = "VPADBASEGetTVMenuStatus"]
    pub unsafe fn get_tv_menu_status(channel: Channel) -> ffi::c_bool;

    /// VPADBASEClearIRCEvent
    #[doc(alias = "VPADBASEClearIRCEvent")]
    #[link_name = "VPADBASEClearIRCEvent"]
    pub unsafe fn clear_irc_event(channel: Channel);

    /// VPADBASEGetIRCStatus
    #[doc(alias = "VPADBASEGetIRCStatus")]
    #[link_name = "VPADBASEGetIRCStatus"]
    pub unsafe fn get_irc_status(channel: Channel) -> IrcStatusFlags;
}

imports_section!(
    "vpadbase",
    [
        "VPADBASEClearIRCEvent",
        "VPADBASEClearNFCEvent",
        "VPADBASEClearNFCMode",
        "VPADBASEClearSyncWaitCount",
        "VPADBASEGetCalibrationData",
        "VPADBASEGetEEPROMData",
        "VPADBASEGetFactorySetting",
        "VPADBASEGetGameControllerMode",
        "VPADBASEGetHeadphoneStatus",
        "VPADBASEGetIRCStatus",
        "VPADBASEGetLCDStatus",
        "VPADBASEGetMessageLEDStatus",
        "VPADBASEGetMicStatus",
        "VPADBASEGetMotorOnRemainingCount",
        "VPADBASEGetNFCStatus",
        "VPADBASEGetPowerButtonPressStatus",
        "VPADBASEGetSensorBarSetting",
        "VPADBASEGetSensorBarStatus",
        "VPADBASEGetState",
        "VPADBASEGetStickOffset",
        "VPADBASEGetTVMenuStatus",
        "VPADBASEGetTrollInvalidSetting",
        "VPADBASEGetTrollInvalidStatus",
        "VPADBASEGetVideoStreamingStatus",
        "VPADBASEGetVolumeOverrideSetting",
        "VPADBASEGetVolumeOverrideStatus",
        "VPADBASEInit",
        "VPADBASEInitVolumeOverrideSettingSyncTime",
        "VPADBASEIsInit",
        "VPADBASERead",
        "VPADBASESetAutoSamplingBuf",
        "VPADBASESetCalibrationData",
        "VPADBASESetGameControllerMode",
        "VPADBASESetMotorOnRemainingCount",
        "VPADBASESetPowerButtonDisableMode",
        "VPADBASESetPowerButtonPressStatus",
        "VPADBASESetSamplingCallback",
        "VPADBASESetSensorBarSetting",
        "VPADBASESetStickOffset",
        "VPADBASESetStickOrigin",
        "VPADBASESetTrollInvalidSetting",
        "VPADBASESetVolumeOverrideSetting",
        "VPADBASEShutdown"
    ],
    []
);
