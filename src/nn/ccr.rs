//! GamePad (DRC) System Control (CCR)

use crate::{coreinit::time::Time, ffi};

/// CCR error code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// CCRSysPairingState
#[doc(alias = "CCRSysPairingState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairingState(pub i32);

impl PairingState {
    /// Pairing is complete or hasn't started yet.
    pub const FINISHED: Self = Self(0);
    pub const IN_PROGRESS: Self = Self(1);
    pub const TIMED_OUT: Self = Self(2);
}

/// CCRSysInitBootFlag
#[doc(alias = "CCRSysInitBootFlag")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InitBootFlag {
    #[default]
    None = 0,
    /// Shows the first boot text on the next GamePad boot.
    FirstBoot = 1,
}

/// CCRSysLCDMode
#[doc(alias = "CCRSysLCDMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LcdMode(pub i32);

impl LcdMode {
    pub const BRIGHTNESS_1: Self = Self(1);
    pub const BRIGHTNESS_2: Self = Self(2);
    pub const BRIGHTNESS_3: Self = Self(3);
    pub const BRIGHTNESS_4: Self = Self(4);
    pub const BRIGHTNESS_5: Self = Self(5);
}

/// CCRSysUpdateState
#[doc(alias = "CCRSysUpdateState")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateState {
    pub state: u32,
    /// 0 to 100.
    pub progress: u32,
}

/// CCRAppLaunchParam
#[doc(alias = "CCRAppLaunchParam")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppLaunchParam {
    /// Entry id in the `nn::sl` launch info database.
    pub launch_info_database_entry_id: u64,
    /// Account UUID.
    pub uuid: [u8; 16],
    _unk: ffi::unknown<0xE7>,
}

unsafe extern "C" {
    /// CCRSysInit
    #[doc(alias = "CCRSysInit")]
    #[link_name = "CCRSysInit"]
    pub unsafe fn init();

    /// CCRSysExit
    #[doc(alias = "CCRSysExit")]
    #[link_name = "CCRSysExit"]
    pub unsafe fn exit();

    /// CCRSysDRCShutdown
    #[doc(alias = "CCRSysDRCShutdown")]
    #[link_name = "CCRSysDRCShutdown"]
    pub unsafe fn drc_shutdown() -> Error;

    /// CCRSysInvalidatePairing
    #[doc(alias = "CCRSysInvalidatePairing")]
    #[link_name = "CCRSysInvalidatePairing"]
    pub unsafe fn invalidate_pairing() -> Error;

    /// __CCRSysDRCIsAttached
    #[doc(alias = "__CCRSysDRCIsAttached")]
    #[link_name = "__CCRSysDRCIsAttached"]
    pub unsafe fn drc_is_attached(drc_slot: u32) -> ffi::c_bool;

    /// CCRSysStartPairing
    ///
    /// `timeout` is in seconds.
    #[doc(alias = "CCRSysStartPairing")]
    #[link_name = "CCRSysStartPairing"]
    pub unsafe fn start_pairing(drc_slot: u32, timeout: u32) -> Error;

    /// CCRSysStopPairing
    #[doc(alias = "CCRSysStopPairing")]
    #[link_name = "CCRSysStopPairing"]
    pub unsafe fn stop_pairing() -> Error;

    /// CCRSysGetPairingState
    #[doc(alias = "CCRSysGetPairingState")]
    #[link_name = "CCRSysGetPairingState"]
    pub unsafe fn get_pairing_state() -> PairingState;

    /// CCRSysGetPincode
    ///
    /// PIN shown during pairing.
    #[doc(alias = "CCRSysGetPincode")]
    #[link_name = "CCRSysGetPincode"]
    pub unsafe fn get_pincode(pin: *mut u32) -> Error;

    /// CCRSysSetSystemTime
    #[doc(alias = "CCRSysSetSystemTime")]
    #[link_name = "CCRSysSetSystemTime"]
    pub unsafe fn set_system_time(time: Time) -> Error;

    /// CCRSysNeedsDRCFWUpdate
    #[doc(alias = "CCRSysNeedsDRCFWUpdate")]
    #[link_name = "CCRSysNeedsDRCFWUpdate"]
    pub unsafe fn needs_drc_fw_update(drc_slot: u32, needs_update: *mut ffi::c_bool) -> Error;

    /// __CCRSysNeedsDRCFWUpdate
    #[doc(alias = "__CCRSysNeedsDRCFWUpdate")]
    #[link_name = "__CCRSysNeedsDRCFWUpdate"]
    pub unsafe fn needs_drc_fw_update_ex(
        drc_slot: u32,
        needs_update: *mut ffi::c_bool,
        allow_downgrade: ffi::c_bool,
    ) -> Error;

    /// CCRSysDRCFWUpdate
    #[doc(alias = "CCRSysDRCFWUpdate")]
    #[link_name = "CCRSysDRCFWUpdate"]
    pub unsafe fn drc_fw_update(drc_slot: u32) -> Error;

    /// __CCRSysDRCFWUpdate
    #[doc(alias = "__CCRSysDRCFWUpdate")]
    #[link_name = "__CCRSysDRCFWUpdate"]
    pub unsafe fn drc_fw_update_ex(drc_slot: u32, allow_downgrade: ffi::c_bool) -> Error;

    /// CCRSysDRCFWUpdateForward
    #[doc(alias = "CCRSysDRCFWUpdateForward")]
    #[link_name = "CCRSysDRCFWUpdateForward"]
    pub unsafe fn drc_fw_update_forward();

    /// CCRSysGetUpdateState
    #[doc(alias = "CCRSysGetUpdateState")]
    #[link_name = "CCRSysGetUpdateState"]
    pub unsafe fn get_update_state(state: *mut UpdateState);

    /// __CCRSysInitReattach
    #[doc(alias = "__CCRSysInitReattach")]
    #[link_name = "__CCRSysInitReattach"]
    pub unsafe fn init_reattach(drc_slot: u32);

    /// __CCRSysWaitReattach
    #[doc(alias = "__CCRSysWaitReattach")]
    #[link_name = "__CCRSysWaitReattach"]
    pub unsafe fn wait_reattach(drc_slot: u32, unk: ffi::c_bool) -> Error;

    /// CCRSysGetVersionCheckFlag
    #[doc(alias = "CCRSysGetVersionCheckFlag")]
    #[link_name = "CCRSysGetVersionCheckFlag"]
    pub unsafe fn get_version_check_flag(flag: *mut u32) -> Error;

    /// CCRSysSetVersionCheckFlag
    #[doc(alias = "CCRSysSetVersionCheckFlag")]
    #[link_name = "CCRSysSetVersionCheckFlag"]
    pub unsafe fn set_version_check_flag(flag: u32) -> Error;

    /// CCRSysCaffeineSetCaffeineSlot
    #[doc(alias = "CCRSysCaffeineSetCaffeineSlot")]
    #[link_name = "CCRSysCaffeineSetCaffeineSlot"]
    pub unsafe fn caffeine_set_caffeine_slot(slot: u32) -> Error;

    /// CCRSysSetInitBootFlag
    #[doc(alias = "CCRSysSetInitBootFlag")]
    #[link_name = "CCRSysSetInitBootFlag"]
    pub unsafe fn set_init_boot_flag(flag: InitBootFlag) -> Error;

    /// CCRSysInitializeSettings
    #[doc(alias = "CCRSysInitializeSettings")]
    #[link_name = "CCRSysInitializeSettings"]
    pub unsafe fn initialize_settings() -> Error;

    /// CCRSysSetCurrentLCDMode
    #[doc(alias = "CCRSysSetCurrentLCDMode")]
    #[link_name = "CCRSysSetCurrentLCDMode"]
    pub unsafe fn set_current_lcd_mode(mode: LcdMode) -> Error;

    /// CCRSysGetCurrentLCDMode
    #[doc(alias = "CCRSysGetCurrentLCDMode")]
    #[link_name = "CCRSysGetCurrentLCDMode"]
    pub unsafe fn get_current_lcd_mode(mode: *mut LcdMode) -> Error;

    /// CCRSysCaffeineGetAppLaunchParam
    #[doc(alias = "CCRSysCaffeineGetAppLaunchParam")]
    #[link_name = "CCRSysCaffeineGetAppLaunchParam"]
    pub unsafe fn caffeine_get_app_launch_param(param: *mut AppLaunchParam);

    /// CCRSysCaffeineBootCheck
    #[doc(alias = "CCRSysCaffeineBootCheck")]
    #[link_name = "CCRSysCaffeineBootCheck"]
    pub unsafe fn caffeine_boot_check() -> u32;

    /// CCRSysCaffeineBootCheckAbort
    #[doc(alias = "CCRSysCaffeineBootCheckAbort")]
    #[link_name = "CCRSysCaffeineBootCheckAbort"]
    pub unsafe fn caffeine_boot_check_abort();

    /// CCRSysCaffeineSetDRCEnableFlag
    #[doc(alias = "CCRSysCaffeineSetDRCEnableFlag")]
    #[link_name = "CCRSysCaffeineSetDRCEnableFlag"]
    pub unsafe fn caffeine_set_drc_enable_flag(enabled: i32) -> Error;

    /// CCRSysCaffeineSetEnableFlag
    #[doc(alias = "CCRSysCaffeineSetEnableFlag")]
    #[link_name = "CCRSysCaffeineSetEnableFlag"]
    pub unsafe fn caffeine_set_enable_flag(enabled: i32) -> Error;
}

imports_section!(
    "nn_ccr",
    [
        "CCRSysCaffeineBootCheck",
        "CCRSysCaffeineBootCheckAbort",
        "CCRSysCaffeineDisplayNotification",
        "CCRSysCaffeineDisplayNotificationAbort",
        "CCRSysCaffeineGetAppLaunchParam",
        "CCRSysCaffeineGetEnableFlag",
        "CCRSysCaffeineGetNotificationReadCount",
        "CCRSysCaffeineGetNotificationSoundMode",
        "CCRSysCaffeineInitializeSettings",
        "CCRSysCaffeineSetCaffeineSlot",
        "CCRSysCaffeineSetDRCEnableFlag",
        "CCRSysCaffeineSetEnableFlag",
        "CCRSysCaffeineSetInitialBootFlag",
        "CCRSysCaffeineSetNotificationInfo",
        "CCRSysCaffeineSetNotificationReadCount",
        "CCRSysCaffeineSetNotificationSoundMode",
        "CCRSysCaffeineSyncEnableFlag",
        "CCRSysCaffeineUpdate",
        "CCRSysCaffeineUpdateAbort",
        "CCRSysCaffeineUpdateEx",
        "CCRSysChangeDRCStateForBGUP",
        "CCRSysDRCFWUpdate",
        "CCRSysDRCFWUpdateForward",
        "CCRSysDRCShutdown",
        "CCRSysDRHReset",
        "CCRSysDbgGetDRHStateChangeFlag",
        "CCRSysDbgSetDRHStateChangeFlag",
        "CCRSysExit",
        "CCRSysGetAudioMode",
        "CCRSysGetCurrentLCDMode",
        "CCRSysGetInitBootFlag",
        "CCRSysGetLCDCabcMode",
        "CCRSysGetLCDMode",
        "CCRSysGetLanguage",
        "CCRSysGetMessageLEDMode",
        "CCRSysGetPairingState",
        "CCRSysGetPincode",
        "CCRSysGetRemoteInfo",
        "CCRSysGetRumbleMode",
        "CCRSysGetTVControlId",
        "CCRSysGetUpdateState",
        "CCRSysGetVersionCheckFlag",
        "CCRSysInit",
        "CCRSysInitializeSettings",
        "CCRSysInvalidatePairing",
        "CCRSysIsRemoteValid",
        "CCRSysNeedsDRCFWUpdate",
        "CCRSysSetAudioMode",
        "CCRSysSetCurrentLCDMode",
        "CCRSysSetInitBootFlag",
        "CCRSysSetLCDCabcMode",
        "CCRSysSetLCDMode",
        "CCRSysSetLCDMute",
        "CCRSysSetLanguage",
        "CCRSysSetMessageLEDMode",
        "CCRSysSetRemoteInfo",
        "CCRSysSetRumbleMode",
        "CCRSysSetSystemTime",
        "CCRSysSetVersionCheckFlag",
        "CCRSysSetVideoQuality",
        "CCRSysStartPairing",
        "CCRSysStopPairing",
        "CCRSysTestTVControl",
        "CCRSysWlChannelScan",
        "CCRSysWlGetChannelStats",
        "CCRSysWlGetWorkSize",
        "CCRSysWlSetWorkarea",
        "CCRSysWlUpdateCondition",
        "__CCRSysCaffeineSetEnableFlag",
        "__CCRSysDRCFWUpdate",
        "__CCRSysDRCIsAttached",
        "__CCRSysInitReattach",
        "__CCRSysLanguageUpdate",
        "__CCRSysNeedsDRCFWUpdate",
        "__CCRSysWaitReattach"
    ],
    []
);
