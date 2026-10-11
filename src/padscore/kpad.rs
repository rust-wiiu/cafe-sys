//! KPAD
//!
//! High level Wii Remote, Nunchuk, Classic Controller, Wii U Pro Controller and Balance Board input.

use super::wpad;
use crate::ffi;

/// KPAD_RING_BUFS
#[doc(alias("KPAD_RING_BUFS", "KPAD_MAX_READ_BUFS"))]
pub const RING_BUFS: usize = 16;

pub const BUTTON_MASK: u32 = 0x00ff_ffff;
pub const URCC_BUTTON_MASK: u32 = 0x03ff_ffff;

/// KPAD_BUTTON
///
/// Wii Remote buttons in [Status::hold]. `Z` and `C` are on the Nunchuk.
#[doc(alias = "KPAD_BUTTON")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Button(pub u32);

impl Button {
    pub const LEFT: Self = Self(0x0001);
    pub const RIGHT: Self = Self(0x0002);
    pub const DOWN: Self = Self(0x0004);
    pub const UP: Self = Self(0x0008);
    pub const PLUS: Self = Self(0x0010);
    pub const TWO: Self = Self(0x0100);
    pub const ONE: Self = Self(0x0200);
    pub const B: Self = Self(0x0400);
    pub const A: Self = Self(0x0800);
    pub const MINUS: Self = Self(0x1000);
    pub const Z: Self = Self(0x2000);
    pub const C: Self = Self(0x4000);
    pub const HOME: Self = Self(0x8000);
    #[doc = "Set while a button repeats, see [set_btn_repeat]."]
    pub const REPEAT: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for Button {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// KPAD_FS_BUTTON
///
/// Nunchuk stick emulation in [FsStatus::hold].
#[doc(alias = "KPAD_FS_BUTTON")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FsButton(pub u32);

impl FsButton {
    pub const STICK_EMULATION_LEFT: Self = Self(0x0001);
    pub const STICK_EMULATION_RIGHT: Self = Self(0x0002);
    pub const STICK_EMULATION_DOWN: Self = Self(0x0004);
    pub const STICK_EMULATION_UP: Self = Self(0x0008);
    pub const REPEAT: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for FsButton {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// KPAD_CL_BUTTON
///
/// Classic Controller buttons.
#[doc(alias = "KPAD_CL_BUTTON")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClButton(pub u32);

impl ClButton {
    pub const UP: Self = Self(0x0001);
    pub const LEFT: Self = Self(0x0002);
    pub const ZR: Self = Self(0x0004);
    pub const X: Self = Self(0x0008);
    pub const A: Self = Self(0x0010);
    pub const Y: Self = Self(0x0020);
    pub const B: Self = Self(0x0040);
    pub const ZL: Self = Self(0x0080);
    pub const R: Self = Self(0x0200);
    pub const PLUS: Self = Self(0x0400);
    pub const HOME: Self = Self(0x0800);
    pub const MINUS: Self = Self(0x1000);
    pub const L: Self = Self(0x2000);
    pub const DOWN: Self = Self(0x4000);
    pub const RIGHT: Self = Self(0x8000);
    pub const STICK_L_EMULATION_LEFT: Self = Self(0x0001_0000);
    pub const STICK_L_EMULATION_RIGHT: Self = Self(0x0002_0000);
    pub const STICK_L_EMULATION_DOWN: Self = Self(0x0004_0000);
    pub const STICK_L_EMULATION_UP: Self = Self(0x0008_0000);
    pub const STICK_R_EMULATION_LEFT: Self = Self(0x0010_0000);
    pub const STICK_R_EMULATION_RIGHT: Self = Self(0x0020_0000);
    pub const STICK_R_EMULATION_DOWN: Self = Self(0x0040_0000);
    pub const STICK_R_EMULATION_UP: Self = Self(0x0080_0000);
    pub const REPEAT: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for ClButton {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// KPAD_UC_BUTTON
///
/// Wii U Pro Controller buttons.
#[doc(alias = "KPAD_UC_BUTTON")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UcButton(pub u32);

impl UcButton {
    pub const UP: Self = Self(0x0000_0001);
    pub const LEFT: Self = Self(0x0000_0002);
    pub const ZR: Self = Self(0x0000_0004);
    pub const X: Self = Self(0x0000_0008);
    pub const A: Self = Self(0x0000_0010);
    pub const Y: Self = Self(0x0000_0020);
    pub const B: Self = Self(0x0000_0040);
    pub const ZL: Self = Self(0x0000_0080);
    pub const R: Self = Self(0x0000_0200);
    pub const PLUS: Self = Self(0x0000_0400);
    pub const HOME: Self = Self(0x0000_0800);
    pub const MINUS: Self = Self(0x0000_1000);
    pub const L: Self = Self(0x0000_2000);
    pub const DOWN: Self = Self(0x0000_4000);
    pub const RIGHT: Self = Self(0x0000_8000);
    pub const STICK_R: Self = Self(0x0001_0000);
    pub const STICK_L: Self = Self(0x0002_0000);
    pub const STICK_L_EMULATION_LEFT: Self = Self(0x0004_0000);
    pub const STICK_L_EMULATION_RIGHT: Self = Self(0x0008_0000);
    pub const STICK_L_EMULATION_DOWN: Self = Self(0x0010_0000);
    pub const STICK_L_EMULATION_UP: Self = Self(0x0020_0000);
    pub const STICK_R_EMULATION_LEFT: Self = Self(0x0040_0000);
    pub const STICK_R_EMULATION_RIGHT: Self = Self(0x0080_0000);
    pub const STICK_R_EMULATION_DOWN: Self = Self(0x0100_0000);
    pub const STICK_R_EMULATION_UP: Self = Self(0x0200_0000);
    pub const REPEAT: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for UcButton {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// KPADError
#[doc(alias = "KPADError")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadError(pub i32);

impl ReadError {
    pub const NONE: Self = Self(0);
    pub const NO_DATA: Self = Self(-1);
    pub const NO_CONTROLLER: Self = Self(-2);
    pub const SETUP: Self = Self(-3);
    pub const LOCKED: Self = Self(-4);
    pub const INIT: Self = Self(-5);
}

/// KPAD_WBC_ERR
///
/// Balance Board weight error.
#[doc(alias = "KPAD_WBC_ERR")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WbcError(pub i32);

impl WbcError {
    pub const EXIST: Self = Self(1);
    pub const NONE: Self = Self(0);
    pub const NO_BATTERY: Self = Self(-1);
    pub const SETUP: Self = Self(-2);
    pub const WRONG_TEMP: Self = Self(-3);
    pub const WRONG_ZERO: Self = Self(-4);
    pub const WEIGHT_OVER: Self = Self(-5);
    pub const CALIBRATION: Self = Self(-6);
    pub const NO_ZEROPOINT: Self = Self(-7);
    pub const ZEROPOINT: Self = Self(-8);
}

/// KPAD_WBC_ERR_TGC
///
/// Balance Board temperature/gravity correction error.
#[doc(alias = "KPAD_WBC_ERR_TGC")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WbcTgcError(pub i32);

impl WbcTgcError {
    pub const READY: Self = Self(1);
    pub const NONE: Self = Self(0);
    pub const UNSTABLE: Self = Self(-1);
    pub const UNKNOWN: Self = Self(-2);
    pub const TIMEOUT: Self = Self(-3);
    pub const BUSY: Self = Self(-4);
}

/// KPADPlayMode
#[doc(alias = "KPADPlayMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayMode(pub u32);

impl PlayMode {
    pub const LOOSE: Self = Self(0);
    pub const TIGHT: Self = Self(1);
}

/// KPADButtonProcMode
#[doc(alias = "KPADButtonProcMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonProcMode(pub u8);

impl ButtonProcMode {
    pub const LOOSE: Self = Self(0);
    pub const TIGHT: Self = Self(1);
}

/// KPADMplsZeroDriftMode
#[doc(alias = "KPADMplsZeroDriftMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MplsZeroDriftMode(pub u32);

impl MplsZeroDriftMode {
    pub const LOOSE: Self = Self(0);
    pub const STANDARD: Self = Self(1);
    pub const TIGHT: Self = Self(2);
}

/// KPAD_DPD
#[doc(alias = "KPAD_DPD")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpdDetection(pub u8);

impl DpdDetection {
    pub const DEFAULT: Self = Self(0);
    pub const DOWNWARD: Self = Self(1);
}

/// KPAD_STATE_CTRL_DPD
#[doc(alias = "KPAD_STATE_CTRL_DPD")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpdControlState(pub i32);

impl DpdControlState {
    pub const START: Self = Self(0);
    pub const FINISHED: Self = Self(1);
}

/// KPAD_STATE_CTRL_MPLS
#[doc(alias = "KPAD_STATE_CTRL_MPLS")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MplsControlState(pub i32);

impl MplsControlState {
    pub const FAILED_CL: Self = Self(-3);
    pub const FAILED_FS: Self = Self(-2);
    pub const FAILED_STD: Self = Self(-1);
    pub const START: Self = Self(0);
    pub const FINISHED: Self = Self(1);
}

/// Vec2
#[doc(alias = "KPADVec2D")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Vec
#[doc(alias("Vec", "KPADVec3D"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Rect
#[doc(alias = "KPADRect")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// KPADMPDir
#[doc(alias("KPADMPDir", "KPADBase3D"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MpDir {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

/// KPADMPStatus
///
/// MotionPlus.
#[doc(alias = "KPADMPStatus")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MpStatus {
    pub mpls: Vec3,
    pub angle: Vec3,
    pub dir: MpDir,
}

/// Nunchuk data of [ExStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FsStatus {
    pub stick: Vec2,
    pub acc: Vec3,
    pub acc_value: f32,
    pub acc_speed: f32,
    pub hold: FsButton,
    pub trig: FsButton,
    pub release: FsButton,
}

/// Classic Controller data of [ExStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ClStatus {
    pub hold: ClButton,
    pub trig: ClButton,
    pub release: ClButton,
    pub l_stick: Vec2,
    pub r_stick: Vec2,
    pub l_trigger: f32,
    pub r_trigger: f32,
}

/// Wii U Pro Controller data of [ExStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UcStatus {
    pub hold: UcButton,
    pub trig: UcButton,
    pub release: UcButton,
    pub l_stick: Vec2,
    pub r_stick: Vec2,
    pub charge: ffi::c_bool,
    pub cable: ffi::c_bool,
}

/// Balance Board data of [ExStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BlStatus {
    pub tgc_weight: f64,
    pub weight: [f64; wpad::PRESS_UNITS],
    pub weight_ave: [f64; wpad::PRESS_UNITS],
    pub weight_err: WbcError,
    pub tgc_weight_err: WbcTgcError,
}

/// Custom device data of [ExStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CmStatus {
    pub hold: u32,
    pub trig: u32,
    pub release: u32,
}

/// KPADEXStatus
///
/// Extension data, depending on [Status::dev_type].
#[doc(alias = "KPADEXStatus")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ExStatus {
    pub fs: FsStatus,
    pub cl: ClStatus,
    pub uc: UcStatus,
    pub bl: BlStatus,
    pub cm: CmStatus,
}

/// KPADStatus
#[doc(alias = "KPADStatus")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Status {
    pub hold: Button,
    /// Pressed since the last sample.
    pub trig: Button,
    /// Released since the last sample.
    pub release: Button,
    pub acc: Vec3,
    pub acc_value: f32,
    pub acc_speed: f32,
    /// Pointer position, positive is right and down.
    pub pos: Vec2,
    pub vec: Vec2,
    pub speed: f32,
    pub horizon: Vec2,
    pub hori_vec: Vec2,
    pub hori_speed: f32,
    pub dist: f32,
    pub dist_vec: f32,
    pub dist_speed: f32,
    pub acc_vertical: Vec2,
    /// [wpad::DeviceType].
    pub dev_type: u8,
    /// [wpad::Error].
    pub wpad_err: i8,
    pub dpd_valid_fg: i8,
    /// [wpad::DataFormat].
    pub data_format: u8,
    pub ex_status: ExStatus,
    pub mpls: MpStatus,
    _pad: ffi::unknown<4>,
}

/// Raw WPAD status, depending on [UnifiedWpadStatus::fmt].
#[repr(C)]
#[derive(Clone, Copy)]
pub union WpadStatus {
    pub core: wpad::Status,
    pub fs: wpad::FsStatus,
    pub cl: wpad::ClStatus,
    pub tr: wpad::TrStatus,
    pub bl: wpad::BlStatus,
    pub mp: wpad::MpStatus,
    pub bk: wpad::BkStatus,
    pub uc: wpad::UcStatus,
    pub cm: wpad::CmStatus,
}

/// KPADUnifiedWpadStatus
#[doc(alias = "KPADUnifiedWpadStatus")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnifiedWpadStatus {
    pub u: WpadStatus,
    /// [wpad::DataFormat].
    pub fmt: u8,
    _pad: u8,
}

const _: () = {
    assert!(size_of::<Status>() == 0xF0);
    assert!(size_of::<UnifiedWpadStatus>() == 0x44);
    assert!(size_of::<MpDir>() == 0x24);
};

/// KPADControlDpdCallback
#[doc(alias = "KPADControlDpdCallback")]
pub type ControlDpdCallback =
    Option<unsafe extern "C" fn(chan: wpad::Channel, state: DpdControlState)>;

/// KPADControlMplsCallback
#[doc(alias = "KPADControlMplsCallback")]
pub type ControlMplsCallback =
    Option<unsafe extern "C" fn(chan: wpad::Channel, state: MplsControlState)>;

unsafe extern "C" {
    /// KPADCalibrateDPD
    #[doc(alias = "KPADCalibrateDPD")]
    #[link_name = "KPADCalibrateDPD"]
    pub unsafe fn calibrate_d_p_d(chan: wpad::Channel) -> i32;

    /// KPADDisableAimingMode
    #[doc(alias = "KPADDisableAimingMode")]
    #[link_name = "KPADDisableAimingMode"]
    pub unsafe fn disable_aiming_mode(chan: wpad::Channel);

    /// KPADDisableDPD
    #[doc(alias = "KPADDisableDPD")]
    #[link_name = "KPADDisableDPD"]
    pub unsafe fn disable_d_p_d(chan: wpad::Channel);

    /// KPADDisableMpls
    #[doc(alias = "KPADDisableMpls")]
    #[link_name = "KPADDisableMpls"]
    pub unsafe fn disable_mpls(chan: wpad::Channel);

    /// KPADDisableMplsAccRevise
    #[doc(alias = "KPADDisableMplsAccRevise")]
    #[link_name = "KPADDisableMplsAccRevise"]
    pub unsafe fn disable_mpls_acc_revise(chan: wpad::Channel);

    /// KPADDisableMplsDirRevise
    #[doc(alias = "KPADDisableMplsDirRevise")]
    #[link_name = "KPADDisableMplsDirRevise"]
    pub unsafe fn disable_mpls_dir_revise(chan: wpad::Channel);

    /// KPADDisableMplsDpdRevise
    #[doc(alias = "KPADDisableMplsDpdRevise")]
    #[link_name = "KPADDisableMplsDpdRevise"]
    pub unsafe fn disable_mpls_dpd_revise(chan: wpad::Channel);

    /// KPADDisableMplsZeroPlay
    #[doc(alias = "KPADDisableMplsZeroPlay")]
    #[link_name = "KPADDisableMplsZeroPlay"]
    pub unsafe fn disable_mpls_zero_play(chan: wpad::Channel);

    /// KPADDisableStickCrossClamp
    #[doc(alias = "KPADDisableStickCrossClamp")]
    #[link_name = "KPADDisableStickCrossClamp"]
    pub unsafe fn disable_stick_cross_clamp();

    /// KPADEnableAimingMode
    #[doc(alias = "KPADEnableAimingMode")]
    #[link_name = "KPADEnableAimingMode"]
    pub unsafe fn enable_aiming_mode(chan: wpad::Channel);

    /// KPADEnableDPD
    #[doc(alias = "KPADEnableDPD")]
    #[link_name = "KPADEnableDPD"]
    pub unsafe fn enable_d_p_d(chan: wpad::Channel);

    /// KPADEnableMpls
    #[doc(alias = "KPADEnableMpls")]
    #[link_name = "KPADEnableMpls"]
    pub unsafe fn enable_mpls(chan: wpad::Channel, mode: wpad::MplsMode);

    /// KPADEnableMplsAccRevise
    #[doc(alias = "KPADEnableMplsAccRevise")]
    #[link_name = "KPADEnableMplsAccRevise"]
    pub unsafe fn enable_mpls_acc_revise(chan: wpad::Channel);

    /// KPADEnableMplsDirRevise
    #[doc(alias = "KPADEnableMplsDirRevise")]
    #[link_name = "KPADEnableMplsDirRevise"]
    pub unsafe fn enable_mpls_dir_revise(chan: wpad::Channel);

    /// KPADEnableMplsDpdRevise
    #[doc(alias = "KPADEnableMplsDpdRevise")]
    #[link_name = "KPADEnableMplsDpdRevise"]
    pub unsafe fn enable_mpls_dpd_revise(chan: wpad::Channel);

    /// KPADEnableMplsZeroPlay
    #[doc(alias = "KPADEnableMplsZeroPlay")]
    #[link_name = "KPADEnableMplsZeroPlay"]
    pub unsafe fn enable_mpls_zero_play(chan: wpad::Channel);

    /// KPADEnableStickCrossClamp
    #[doc(alias = "KPADEnableStickCrossClamp")]
    #[link_name = "KPADEnableStickCrossClamp"]
    pub unsafe fn enable_stick_cross_clamp();

    /// KPADGetAccParam
    #[doc(alias = "KPADGetAccParam")]
    #[link_name = "KPADGetAccParam"]
    pub unsafe fn get_acc_param(chan: wpad::Channel, play_radius: *mut f32, sensitivity: *mut f32);

    /// KPADGetAccPlayMode
    #[doc(alias = "KPADGetAccPlayMode")]
    #[link_name = "KPADGetAccPlayMode"]
    pub unsafe fn get_acc_play_mode(chan: wpad::Channel) -> PlayMode;

    /// KPADGetButtonProcMode
    #[doc(alias = "KPADGetButtonProcMode")]
    #[link_name = "KPADGetButtonProcMode"]
    pub unsafe fn get_button_proc_mode(chan: wpad::Channel) -> ButtonProcMode;

    /// KPADGetCrossStickEmulationParamsL
    #[doc(alias = "KPADGetCrossStickEmulationParamsL")]
    #[link_name = "KPADGetCrossStickEmulationParamsL"]
    pub unsafe fn get_cross_stick_emulation_params_l(
        chan: wpad::Channel,
        rot_deg: *mut f32,
        xy_deg: *mut f32,
        radius: *mut f32,
    );

    /// KPADGetCrossStickEmulationParamsR
    #[doc(alias = "KPADGetCrossStickEmulationParamsR")]
    #[link_name = "KPADGetCrossStickEmulationParamsR"]
    pub unsafe fn get_cross_stick_emulation_params_r(
        chan: wpad::Channel,
        rot_deg: *mut f32,
        xy_deg: *mut f32,
        radius: *mut f32,
    );

    /// KPADGetDistParam
    #[doc(alias = "KPADGetDistParam")]
    #[link_name = "KPADGetDistParam"]
    pub unsafe fn get_dist_param(chan: wpad::Channel, play_radius: *mut f32, sensitivity: *mut f32);

    /// KPADGetDistPlayMode
    #[doc(alias = "KPADGetDistPlayMode")]
    #[link_name = "KPADGetDistPlayMode"]
    pub unsafe fn get_dist_play_mode(chan: wpad::Channel) -> PlayMode;

    /// KPADGetDpdDetection
    #[doc(alias = "KPADGetDpdDetection")]
    #[link_name = "KPADGetDpdDetection"]
    pub unsafe fn get_dpd_detection() -> DpdDetection;

    /// KPADGetGameMaxControllers
    #[doc(alias = "KPADGetGameMaxControllers")]
    #[link_name = "KPADGetGameMaxControllers"]
    pub unsafe fn get_game_max_controllers() -> u32;

    /// KPADGetHoriParam
    #[doc(alias = "KPADGetHoriParam")]
    #[link_name = "KPADGetHoriParam"]
    pub unsafe fn get_hori_param(chan: wpad::Channel, play_radius: *mut f32, sensitivity: *mut f32);

    /// KPADGetHoriPlayMode
    #[doc(alias = "KPADGetHoriPlayMode")]
    #[link_name = "KPADGetHoriPlayMode"]
    pub unsafe fn get_hori_play_mode(chan: wpad::Channel) -> PlayMode;

    /// KPADGetMaxControllers
    #[doc(alias = "KPADGetMaxControllers")]
    #[link_name = "KPADGetMaxControllers"]
    pub unsafe fn get_max_controllers() -> u32;

    /// KPADGetMplsAccReviseParam
    #[doc(alias = "KPADGetMplsAccReviseParam")]
    #[link_name = "KPADGetMplsAccReviseParam"]
    pub unsafe fn get_mpls_acc_revise_param(
        chan: wpad::Channel,
        revise_power: *mut f32,
        revise_range: *mut f32,
    );

    /// KPADGetMplsDirReviseParam
    #[doc(alias = "KPADGetMplsDirReviseParam")]
    #[link_name = "KPADGetMplsDirReviseParam"]
    pub unsafe fn get_mpls_dir_revise_param(chan: wpad::Channel, revise_power: *mut f32);

    /// KPADGetMplsDpdReviseParam
    #[doc(alias = "KPADGetMplsDpdReviseParam")]
    #[link_name = "KPADGetMplsDpdReviseParam"]
    pub unsafe fn get_mpls_dpd_revise_param(chan: wpad::Channel, revise_power: *mut f32);

    /// KPADGetMplsStatus
    #[doc(alias = "KPADGetMplsStatus")]
    #[link_name = "KPADGetMplsStatus"]
    pub unsafe fn get_mpls_status(chan: wpad::Channel) -> wpad::MplsMode;

    /// KPADGetMplsWorkSize
    #[doc(alias = "KPADGetMplsWorkSize")]
    #[link_name = "KPADGetMplsWorkSize"]
    pub unsafe fn get_mpls_work_size() -> u32;

    /// KPADGetMplsZeroDriftMode
    #[doc(alias = "KPADGetMplsZeroDriftMode")]
    #[link_name = "KPADGetMplsZeroDriftMode"]
    pub unsafe fn get_mpls_zero_drift_mode(chan: wpad::Channel, mode: *mut MplsZeroDriftMode);

    /// KPADGetMplsZeroPlayParam
    #[doc(alias = "KPADGetMplsZeroPlayParam")]
    #[link_name = "KPADGetMplsZeroPlayParam"]
    pub unsafe fn get_mpls_zero_play_param(chan: wpad::Channel, radius: *mut f32);

    /// KPADGetPosParam
    #[doc(alias = "KPADGetPosParam")]
    #[link_name = "KPADGetPosParam"]
    pub unsafe fn get_pos_param(chan: wpad::Channel, play_radius: *mut f32, sensitivity: *mut f32);

    /// KPADGetPosPlayMode
    #[doc(alias = "KPADGetPosPlayMode")]
    #[link_name = "KPADGetPosPlayMode"]
    pub unsafe fn get_pos_play_mode(chan: wpad::Channel) -> PlayMode;

    /// KPADGetProjectionPos
    #[doc(alias = "KPADGetProjectionPos")]
    #[link_name = "KPADGetProjectionPos"]
    pub unsafe fn get_projection_pos(
        dst: *mut Vec2,
        src: *const Vec2,
        proj_rect: *const Rect,
        vi_ratio: f32,
    );

    /// KPADGetReviseAngle
    #[doc(alias = "KPADGetReviseAngle")]
    #[link_name = "KPADGetReviseAngle"]
    pub unsafe fn get_revise_angle() -> f32;

    /// KPADGetSensorHeight
    #[doc(alias = "KPADGetSensorHeight")]
    #[link_name = "KPADGetSensorHeight"]
    pub unsafe fn get_sensor_height(chan: wpad::Channel) -> f32;

    /// KPADGetUnifiedWpadStatus
    #[doc(alias = "KPADGetUnifiedWpadStatus")]
    #[link_name = "KPADGetUnifiedWpadStatus"]
    pub unsafe fn get_unified_wpad_status(
        chan: wpad::Channel,
        dst: *mut UnifiedWpadStatus,
        count: u32,
    );

    /// KPADInit
    #[doc(alias = "KPADInit")]
    #[link_name = "KPADInit"]
    pub unsafe fn init();

    /// KPADInitEx
    ///
    /// Like [init], with ring buffers for [get_unified_wpad_status].
    #[doc(alias = "KPADInitEx")]
    #[link_name = "KPADInitEx"]
    pub unsafe fn init_ex(bufs: *mut UnifiedWpadStatus, length: u32);

    /// KPADInitMplsAccReviseParam
    #[doc(alias = "KPADInitMplsAccReviseParam")]
    #[link_name = "KPADInitMplsAccReviseParam"]
    pub unsafe fn init_mpls_acc_revise_param(chan: wpad::Channel);

    /// KPADInitMplsDirReviseParam
    #[doc(alias = "KPADInitMplsDirReviseParam")]
    #[link_name = "KPADInitMplsDirReviseParam"]
    pub unsafe fn init_mpls_dir_revise_param(chan: wpad::Channel);

    /// KPADInitMplsDpdReviseParam
    #[doc(alias = "KPADInitMplsDpdReviseParam")]
    #[link_name = "KPADInitMplsDpdReviseParam"]
    pub unsafe fn init_mpls_dpd_revise_param(chan: wpad::Channel);

    /// KPADInitMplsZeroDriftMode
    #[doc(alias = "KPADInitMplsZeroDriftMode")]
    #[link_name = "KPADInitMplsZeroDriftMode"]
    pub unsafe fn init_mpls_zero_drift_mode(chan: wpad::Channel);

    /// KPADInitMplsZeroPlayParam
    #[doc(alias = "KPADInitMplsZeroPlayParam")]
    #[link_name = "KPADInitMplsZeroPlayParam"]
    pub unsafe fn init_mpls_zero_play_param(chan: wpad::Channel);

    /// KPADIsEnableAimingMode
    #[doc(alias = "KPADIsEnableAimingMode")]
    #[link_name = "KPADIsEnableAimingMode"]
    pub unsafe fn is_enable_aiming_mode(chan: wpad::Channel) -> u8;

    /// KPADIsEnableMplsAccRevise
    #[doc(alias = "KPADIsEnableMplsAccRevise")]
    #[link_name = "KPADIsEnableMplsAccRevise"]
    pub unsafe fn is_enable_mpls_acc_revise(chan: wpad::Channel) -> f32;

    /// KPADIsEnableMplsDirRevise
    #[doc(alias = "KPADIsEnableMplsDirRevise")]
    #[link_name = "KPADIsEnableMplsDirRevise"]
    pub unsafe fn is_enable_mpls_dir_revise(chan: wpad::Channel) -> f32;

    /// KPADIsEnableMplsDpdRevise
    #[doc(alias = "KPADIsEnableMplsDpdRevise")]
    #[link_name = "KPADIsEnableMplsDpdRevise"]
    pub unsafe fn is_enable_mpls_dpd_revise(chan: wpad::Channel) -> f32;

    /// KPADIsEnableMplsZeroDrift
    #[doc(alias = "KPADIsEnableMplsZeroDrift")]
    #[link_name = "KPADIsEnableMplsZeroDrift"]
    pub unsafe fn is_enable_mpls_zero_drift(chan: wpad::Channel) -> f32;

    /// KPADIsEnableMplsZeroPlay
    #[doc(alias = "KPADIsEnableMplsZeroPlay")]
    #[link_name = "KPADIsEnableMplsZeroPlay"]
    pub unsafe fn is_enable_mpls_zero_play(chan: wpad::Channel) -> f32;

    /// KPADRead
    ///
    /// Reads up to `length` samples. Returns the number of samples read.
    #[doc(alias = "KPADRead")]
    #[link_name = "KPADRead"]
    pub unsafe fn read(chan: wpad::Channel, bufs: *mut Status, length: u32) -> i32;

    /// KPADReadEx
    ///
    /// Like [read], `err` receives the [ReadError].
    #[doc(alias = "KPADReadEx")]
    #[link_name = "KPADReadEx"]
    pub unsafe fn read_ex(
        chan: wpad::Channel,
        bufs: *mut Status,
        length: u32,
        err: *mut ReadError,
    ) -> i32;

    /// KPADReset
    #[doc(alias = "KPADReset")]
    #[link_name = "KPADReset"]
    pub unsafe fn reset();

    /// KPADResetMpls
    #[doc(alias = "KPADResetMpls")]
    #[link_name = "KPADResetMpls"]
    pub unsafe fn reset_mpls(chan: wpad::Channel);

    /// KPADResetWbcTgcWeight
    #[doc(alias = "KPADResetWbcTgcWeight")]
    #[link_name = "KPADResetWbcTgcWeight"]
    pub unsafe fn reset_wbc_tgc_weight();

    /// KPADResetWbcZeroPoint
    #[doc(alias = "KPADResetWbcZeroPoint")]
    #[link_name = "KPADResetWbcZeroPoint"]
    pub unsafe fn reset_wbc_zero_point();

    /// KPADReviseAcc
    #[doc(alias = "KPADReviseAcc")]
    #[link_name = "KPADReviseAcc"]
    pub unsafe fn revise_acc(acc: *mut Vec3) -> f32;

    /// KPADSetAccParam
    #[doc(alias = "KPADSetAccParam")]
    #[link_name = "KPADSetAccParam"]
    pub unsafe fn set_acc_param(chan: wpad::Channel, play_radius: f32, sensitivity: f32);

    /// KPADSetAccPlayMode
    #[doc(alias = "KPADSetAccPlayMode")]
    #[link_name = "KPADSetAccPlayMode"]
    pub unsafe fn set_acc_play_mode(chan: wpad::Channel, mode: PlayMode);

    /// KPADSetBtnRepeat
    #[doc(alias = "KPADSetBtnRepeat")]
    #[link_name = "KPADSetBtnRepeat"]
    pub unsafe fn set_btn_repeat(chan: wpad::Channel, delay_sec: f32, pulse_sec: f32);

    /// KPADSetButtonProcMode
    #[doc(alias = "KPADSetButtonProcMode")]
    #[link_name = "KPADSetButtonProcMode"]
    pub unsafe fn set_button_proc_mode(chan: wpad::Channel, mode: ButtonProcMode);

    /// KPADSetConnectCallback
    #[doc(alias = "KPADSetConnectCallback")]
    #[link_name = "KPADSetConnectCallback"]
    pub unsafe fn set_connect_callback(
        chan: wpad::Channel,
        callback: wpad::ConnectCallback,
    ) -> wpad::ConnectCallback;

    /// KPADSetControlDpdCallback
    #[doc(alias = "KPADSetControlDpdCallback")]
    #[link_name = "KPADSetControlDpdCallback"]
    pub unsafe fn set_control_dpd_callback(chan: wpad::Channel, callback: ControlDpdCallback);

    /// KPADSetControlMplsCallback
    #[doc(alias = "KPADSetControlMplsCallback")]
    #[link_name = "KPADSetControlMplsCallback"]
    pub unsafe fn set_control_mpls_callback(chan: wpad::Channel, callback: ControlMplsCallback);

    /// KPADSetCrossStickEmulationParamsL
    #[doc(alias = "KPADSetCrossStickEmulationParamsL")]
    #[link_name = "KPADSetCrossStickEmulationParamsL"]
    pub unsafe fn set_cross_stick_emulation_params_l(
        chan: wpad::Channel,
        rot_deg: f32,
        xy_deg: f32,
        radius: f32,
    );

    /// KPADSetCrossStickEmulationParamsR
    #[doc(alias = "KPADSetCrossStickEmulationParamsR")]
    #[link_name = "KPADSetCrossStickEmulationParamsR"]
    pub unsafe fn set_cross_stick_emulation_params_r(
        chan: wpad::Channel,
        rot_deg: f32,
        xy_deg: f32,
        radius: f32,
    );

    /// KPADSetDistParam
    #[doc(alias = "KPADSetDistParam")]
    #[link_name = "KPADSetDistParam"]
    pub unsafe fn set_dist_param(chan: wpad::Channel, play_radius: f32, sensitivity: f32);

    /// KPADSetDistPlayMode
    #[doc(alias = "KPADSetDistPlayMode")]
    #[link_name = "KPADSetDistPlayMode"]
    pub unsafe fn set_dist_play_mode(chan: wpad::Channel, mode: PlayMode);

    /// KPADSetDpdDetection
    #[doc(alias = "KPADSetDpdDetection")]
    #[link_name = "KPADSetDpdDetection"]
    pub unsafe fn set_dpd_detection(mode: DpdDetection);

    /// KPADSetFSStickClamp
    #[doc(alias = "KPADSetFSStickClamp")]
    #[link_name = "KPADSetFSStickClamp"]
    pub unsafe fn set_fs_stick_clamp(min: i8, max: i8);

    /// KPADSetHoriParam
    #[doc(alias = "KPADSetHoriParam")]
    #[link_name = "KPADSetHoriParam"]
    pub unsafe fn set_hori_param(chan: wpad::Channel, play_radius: f32, sensitivity: f32);

    /// KPADSetHoriPlayMode
    #[doc(alias = "KPADSetHoriPlayMode")]
    #[link_name = "KPADSetHoriPlayMode"]
    pub unsafe fn set_hori_play_mode(chan: wpad::Channel, mode: PlayMode);

    /// KPADSetMaxControllers
    ///
    /// Sets the number of supported controllers (4 or 7).
    #[doc(alias = "KPADSetMaxControllers")]
    #[link_name = "KPADSetMaxControllers"]
    pub unsafe fn set_max_controllers(max_controllers: u32) -> i32;

    /// KPADSetMplsAccReviseParam
    #[doc(alias = "KPADSetMplsAccReviseParam")]
    #[link_name = "KPADSetMplsAccReviseParam"]
    pub unsafe fn set_mpls_acc_revise_param(
        chan: wpad::Channel,
        revise_power: f32,
        revise_range: f32,
    );

    /// KPADSetMplsAngle
    #[doc(alias = "KPADSetMplsAngle")]
    #[link_name = "KPADSetMplsAngle"]
    pub unsafe fn set_mpls_angle(chan: wpad::Channel, ax: f32, ay: f32, az: f32);

    /// KPADSetMplsDirReviseBase
    #[doc(alias = "KPADSetMplsDirReviseBase")]
    #[link_name = "KPADSetMplsDirReviseBase"]
    pub unsafe fn set_mpls_dir_revise_base(chan: wpad::Channel, base: *mut MpDir);

    /// KPADSetMplsDirReviseParam
    #[doc(alias = "KPADSetMplsDirReviseParam")]
    #[link_name = "KPADSetMplsDirReviseParam"]
    pub unsafe fn set_mpls_dir_revise_param(chan: wpad::Channel, revise_power: f32);

    /// KPADSetMplsDirection
    #[doc(alias = "KPADSetMplsDirection")]
    #[link_name = "KPADSetMplsDirection"]
    pub unsafe fn set_mpls_direction(chan: wpad::Channel, dir: *mut MpDir);

    /// KPADSetMplsDirectionMag
    #[doc(alias = "KPADSetMplsDirectionMag")]
    #[link_name = "KPADSetMplsDirectionMag"]
    pub unsafe fn set_mpls_direction_mag(chan: wpad::Channel, mag: f32);

    /// KPADSetMplsDpdReviseParam
    #[doc(alias = "KPADSetMplsDpdReviseParam")]
    #[link_name = "KPADSetMplsDpdReviseParam"]
    pub unsafe fn set_mpls_dpd_revise_param(chan: wpad::Channel, revise_power: f32);

    /// KPADSetMplsMagnification
    #[doc(alias = "KPADSetMplsMagnification")]
    #[link_name = "KPADSetMplsMagnification"]
    pub unsafe fn set_mpls_magnification(chan: wpad::Channel, pitch: f32, yaw: f32, roll: f32);

    /// KPADSetMplsWorkarea
    #[doc(alias = "KPADSetMplsWorkarea")]
    #[link_name = "KPADSetMplsWorkarea"]
    pub unsafe fn set_mpls_workarea(ptr: *mut ffi::c_void);

    /// KPADSetMplsZeroDriftMode
    #[doc(alias = "KPADSetMplsZeroDriftMode")]
    #[link_name = "KPADSetMplsZeroDriftMode"]
    pub unsafe fn set_mpls_zero_drift_mode(chan: wpad::Channel, mode: MplsZeroDriftMode);

    /// KPADSetMplsZeroPlayParam
    #[doc(alias = "KPADSetMplsZeroPlayParam")]
    #[link_name = "KPADSetMplsZeroPlayParam"]
    pub unsafe fn set_mpls_zero_play_param(chan: wpad::Channel, radius: f32);

    /// KPADSetObjInterval
    #[doc(alias = "KPADSetObjInterval")]
    #[link_name = "KPADSetObjInterval"]
    pub unsafe fn set_obj_interval(interval: f32);

    /// KPADSetPosParam
    #[doc(alias = "KPADSetPosParam")]
    #[link_name = "KPADSetPosParam"]
    pub unsafe fn set_pos_param(chan: wpad::Channel, play_radius: f32, sensitivity: f32);

    /// KPADSetPosPlayMode
    #[doc(alias = "KPADSetPosPlayMode")]
    #[link_name = "KPADSetPosPlayMode"]
    pub unsafe fn set_pos_play_mode(chan: wpad::Channel, mode: PlayMode);

    /// KPADSetReviseMode
    #[doc(alias = "KPADSetReviseMode")]
    #[link_name = "KPADSetReviseMode"]
    pub unsafe fn set_revise_mode(chan: wpad::Channel, sw: ffi::c_bool);

    /// KPADSetSamplingCallback
    #[doc(alias = "KPADSetSamplingCallback")]
    #[link_name = "KPADSetSamplingCallback"]
    pub unsafe fn set_sampling_callback(
        chan: wpad::Channel,
        callback: wpad::SamplingCallback,
    ) -> wpad::SamplingCallback;

    /// KPADSetSensorHeight
    #[doc(alias = "KPADSetSensorHeight")]
    #[link_name = "KPADSetSensorHeight"]
    pub unsafe fn set_sensor_height(chan: wpad::Channel, level: f32);

    /// KPADShutdown
    #[doc(alias = "KPADShutdown")]
    #[link_name = "KPADShutdown"]
    pub unsafe fn deinit();

    /// KPADStartMplsCalibration
    #[doc(alias = "KPADStartMplsCalibration")]
    #[link_name = "KPADStartMplsCalibration"]
    pub unsafe fn start_mpls_calibration(chan: wpad::Channel);

    /// KPADStopMplsCalibration
    #[doc(alias = "KPADStopMplsCalibration")]
    #[link_name = "KPADStopMplsCalibration"]
    pub unsafe fn stop_mpls_calibration(chan: wpad::Channel);

    /// KPADWorkMplsCalibration
    #[doc(alias = "KPADWorkMplsCalibration")]
    #[link_name = "KPADWorkMplsCalibration"]
    pub unsafe fn work_mpls_calibration(chan: wpad::Channel) -> f32;
}
