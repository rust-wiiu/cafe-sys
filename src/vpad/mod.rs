//! vpad

use crate::ffi::*;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[doc(alias = "VPADChan")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Channel {
    #[default]
    C0,
    C1,
}

#[doc(alias = "VPADVec2D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[doc(alias = "VPADVec3D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[doc(alias = "VPADAccStatus")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Accelerometer {
    pub acceleration: Vec3,
    pub magnitude: f32,
    pub variation: f32,
    pub vertical: Vec2,
}

#[doc(alias = "VPADDirection")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Validity: u16 {
        const Valid = 0;
        const InvalidX = 1;
        const InvalidY = 2;
        const InvalidXY = Self::InvalidX.bits() | Self::InvalidY.bits();
    }
}

#[doc(alias = "VPADTouchData")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct TouchPanel {
    pub x: u16,
    pub y: u16,
    pub touched: bool,
    pub validity: Validity,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Button : u32 {
        const A = 0x8000;
        const B = 0x4000;
        const X = 0x2000;
        const Y = 0x1000;
        const Left = 0x0800;
        const Right = 0x0400;
        const Up = 0x0200;
        const Down = 0x0100;
        const ZL = 0x0080;
        const ZR = 0x0040;
        const L = 0x0020;
        const R = 0x0010;
        const Plus = 0x0008;
        const Minus = 0x0004;
        const Home = 0x0002;
        const Sync = 0x0001;
        const R3 = 0x00020000;
        const L3 = 0x00040000;
        const TV = 0x00010000;
        const RStickLeft = 0x04000000;
        const RStickRight = 0x02000000;
        const RStickUp = 0x01000000;
        const RStickDown = 0x00800000;
        const LStickLeft = 0x40000000;
        const LStickRight = 0x20000000;
        const LStickUp = 0x10000000;
        const LStickDown = 0x08000000;
    }
}

#[doc(alias = "VPADStatus")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    pub hold: Button,
    pub trigger: Button,
    pub release: Button,
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub accelerometer: Accelerometer,
    pub gyro: Vec3,
    pub angle: Vec3,
    pub _error: u8,
    pub _unk0: unknown<0x1>, // ?
    pub tp_normal: TouchPanel,
    pub tp_filtered1: TouchPanel,
    pub tp_filtered2: TouchPanel,
    pub _unk1: unknown<0x2>, // ?
    pub direction: Direction,
    pub headphones: bool,
    pub magnetometer: Vec3,
    pub volume_slider: u8,
    pub battery: u8,
    pub microphone_status: u8,
    pub volume_slider_ex: u8,
    pub _pad: unknown<0x8>,
}

#[doc(alias = "VPADReadError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Error {
    Success = 0,
    NoSamples = -1,
    InvalidController = -2,
    Busy = -4,
    Uninitialized = -5,
}

unsafe extern "C" {
    #[deprecated(note = "Deprecated since Cafe OS 5.5.x")]
    #[doc(alias = "VPADInit")]
    #[link_name = "VPADInit"]
    pub unsafe fn init();

    #[deprecated(note = "Deprecated since Cafe OS 5.5.x")]
    #[doc(alias = "VPADShutdown")]
    #[link_name = "VPADShutdown"]
    pub unsafe fn deinit();

    #[doc(alias = "VPADRead")]
    #[link_name = "VPADRead"]
    pub unsafe fn poll(
        channel: Channel,
        buffers: *mut Status,
        samples: u32,
        error: *mut Error,
    ) -> i32;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "vpad",
    [
        "VPADCalcTPCalibrationParam",
        "VPADControlMotor",
        "VPADDisableGyroAccRevise",
        "VPADDisableGyroDirRevise",
        "VPADDisableGyroZeroPlay",
        "VPADDisableLStickZeroClamp",
        "VPADDisablePowerButton",
        "VPADDisableRStickZeroClamp",
        "VPADDisableStickCrossClamp",
        "VPADEnableGyroAccRevise",
        "VPADEnableGyroDirRevise",
        "VPADEnableGyroZeroPlay",
        "VPADEnableLStickZeroClamp",
        "VPADEnablePowerButton",
        "VPADEnableRStickZeroClamp",
        "VPADEnableStickCrossClamp",
        "VPADGetAccParam",
        "VPADGetAccPlayMode",
        "VPADGetButtonProcMode",
        "VPADGetCrossStickEmulationParamsL",
        "VPADGetCrossStickEmulationParamsR",
        "VPADGetGyroAccReviseParam",
        "VPADGetGyroDirReviseParam",
        "VPADGetGyroMagReviseParam",
        "VPADGetGyroZeroDriftMode",
        "VPADGetGyroZeroPlayParam",
        "VPADGetLStickClampThreshold",
        "VPADGetLcdMode",
        "VPADGetRStickClampThreshold",
        "VPADGetTPCalibratedPoint",
        "VPADGetTPCalibratedPointEx",
        "VPADGetTPCalibrationParam",
        "VPADGetTVMenuStatus",
        "VPADInit",
        "VPADInitGyroAccReviseParam",
        "VPADInitGyroDirReviseParam",
        "VPADInitGyroZeroDriftMode",
        "VPADInitGyroZeroPlayParam",
        "VPADIsEnableGyroAccRevise",
        "VPADIsEnableGyroDirRevise",
        "VPADIsEnableGyroZeroDrift",
        "VPADIsEnableGyroZeroPlay",
        "VPADIsStartedGyroMagRevise",
        "VPADRead",
        "VPADResetAccToDefault",
        "VPADResetTPToDefault",
        "VPADSetAccParam",
        "VPADSetAccPlayMode",
        "VPADSetAudioVolumeOverride",
        "VPADSetBtnRepeat",
        "VPADSetButtonProcMode",
        "VPADSetCrossStickEmulationParamsL",
        "VPADSetCrossStickEmulationParamsR",
        "VPADSetGyroAccReviseParam",
        "VPADSetGyroAngle",
        "VPADSetGyroDirReviseBase",
        "VPADSetGyroDirReviseParam",
        "VPADSetGyroDirection",
        "VPADSetGyroDirectionMag",
        "VPADSetGyroMagReviseParam",
        "VPADSetGyroMagnification",
        "VPADSetGyroZeroDriftMode",
        "VPADSetGyroZeroPlayParam",
        "VPADSetLStickClampThreshold",
        "VPADSetLcdMode",
        "VPADSetRStickClampThreshold",
        "VPADSetSamplingCallback",
        "VPADSetSensorBar",
        "VPADSetStickOrigin",
        "VPADSetTPCalibrationParam",
        "VPADSetTVMenuInvalid",
        "VPADShutdown",
        "VPADStartAccCalibration",
        "VPADStartGyroMagRevise",
        "VPADStopGyroMagRevise",
        "VPADStopMotor",
        "VPADWriteTPCalibrationValueToEEPROM"
    ],
    []
);
