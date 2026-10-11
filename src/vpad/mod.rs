//! VPAD
//!
//! Wii U GamePad input.

use crate::ffi;

pub use crate::vpadbase::{BatteryLevel, Channel, MAX_CONTROLLERS};

/// Number of samples buffered per channel.
pub const MAX_READ_BUFS: usize = 16;

/// Maximum rumble pattern length in bits, one second at 120 Hz.
pub const MOTOR_PATTERN_LENGTH_MAX: u8 = 120;

/// VPADButtons
#[doc(alias = "VPADButtons")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Button(pub u32);

impl Button {
    pub const NONE: Self = Self(0);
    pub const A: Self = Self(0x8000);
    pub const B: Self = Self(0x4000);
    pub const X: Self = Self(0x2000);
    pub const Y: Self = Self(0x1000);
    pub const LEFT: Self = Self(0x0800);
    pub const RIGHT: Self = Self(0x0400);
    pub const UP: Self = Self(0x0200);
    pub const DOWN: Self = Self(0x0100);
    pub const ZL: Self = Self(0x0080);
    pub const ZR: Self = Self(0x0040);
    pub const L: Self = Self(0x0020);
    pub const R: Self = Self(0x0010);
    pub const PLUS: Self = Self(0x0008);
    pub const MINUS: Self = Self(0x0004);
    pub const HOME: Self = Self(0x0002);
    pub const SYNC: Self = Self(0x0001);
    pub const STICK_R: Self = Self(0x0002_0000);
    pub const STICK_L: Self = Self(0x0004_0000);
    pub const TV: Self = Self(0x0001_0000);
    pub const STICK_R_EMULATION_LEFT: Self = Self(0x0400_0000);
    pub const STICK_R_EMULATION_RIGHT: Self = Self(0x0200_0000);
    pub const STICK_R_EMULATION_UP: Self = Self(0x0100_0000);
    pub const STICK_R_EMULATION_DOWN: Self = Self(0x0080_0000);
    pub const STICK_L_EMULATION_LEFT: Self = Self(0x4000_0000);
    pub const STICK_L_EMULATION_RIGHT: Self = Self(0x2000_0000);
    pub const STICK_L_EMULATION_UP: Self = Self(0x1000_0000);
    pub const STICK_L_EMULATION_DOWN: Self = Self(0x0800_0000);
    /// Set on repeat pulses, see [set_btn_repeat].
    pub const REPEAT: Self = Self(0x8000_0000);

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl core::ops::BitOr for Button {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// VPADReadError
#[doc(alias = "VPADReadError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const NO_SAMPLES: Self = Self(-1);
    pub const INVALID_CONTROLLER: Self = Self(-2);
    pub const SETUP: Self = Self(-3);
    /// Accessed by another thread.
    pub const BUSY: Self = Self(-4);
    pub const UNINITIALIZED: Self = Self(-5);
}

/// VPADButtonProcMode
#[doc(alias = "VPADButtonProcMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonProcMode(pub u8);

impl ButtonProcMode {
    /// Only the most recent button state is copied into all samples. The default.
    pub const LOOSE: Self = Self(0);
    /// Every button change is tracked.
    pub const TIGHT: Self = Self(1);
}

/// VPADTouchPadValidity
#[doc(alias = "VPADTouchPadValidity")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchValidity(pub u16);

impl TouchValidity {
    pub const VALID: Self = Self(0);
    pub const INVALID_X: Self = Self(1 << 0);
    pub const INVALID_Y: Self = Self(1 << 1);
    pub const INVALID_XY: Self = Self(0b11);
}

/// VPADTouchPadResolution
#[doc(alias = "VPADTouchPadResolution")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TouchPadResolution {
    #[default]
    R1920x1080 = 0,
    R1280x720 = 1,
    R854x480 = 2,
}

/// VPADLcdMode
#[doc(alias = "VPADLcdMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LcdMode(pub u32);

impl LcdMode {
    /// Standby. Turns back on when a button is pressed.
    pub const STANDBY: Self = Self(0);
    /// The screen is off and the GamePad only works as a controller.
    pub const GAME_CONTROLLER: Self = Self(1);
    pub const ON: Self = Self(0xFF);
}

/// VPADGyroZeroDriftMode
#[doc(alias = "VPADGyroZeroDriftMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GyroZeroDriftMode(pub u32);

impl GyroZeroDriftMode {
    pub const LOOSE: Self = Self(0);
    pub const STANDARD: Self = Self(1);
    pub const TIGHT: Self = Self(2);
    pub const NONE: Self = Self(3);
}

/// VPADPlayMode
#[doc(alias = "VPADPlayMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayMode(pub u32);

impl PlayMode {
    pub const LOOSE: Self = Self(0);
    pub const TIGHT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MicStatus(pub u8);

impl MicStatus {
    pub const EXTERNAL_MIC_ON: Self = Self(0x08);
}

/// VPADVec2D
#[doc(alias = "VPADVec2D")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// VPADVec3D
#[doc(alias = "VPADVec3D")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// VPADDirection
///
/// Orientation of the GamePad as three axis vectors.
#[doc(alias = "VPADDirection")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Direction {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

/// VPADAccStatus
#[doc(alias = "VPADAccStatus")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AccStatus {
    /// 1.0 is 1 g.
    pub acc: Vec3,
    /// Length of `acc`.
    pub magnitude: f32,
    /// Change of `acc` since the previous sample.
    pub variation: f32,
    /// `x` is 0 when held vertically and 1 when lying flat, `y` is 1 when upright and -1 upside down.
    pub vertical: Vec2,
}

/// VPADTouchData
#[doc(alias = "VPADTouchData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchData {
    pub x: u16,
    pub y: u16,
    /// 1 while touched.
    pub touched: u16,
    pub validity: TouchValidity,
}

/// VPADTouchCalibrationParam
#[doc(alias = "VPADTouchCalibrationParam")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TouchCalibrationParam {
    pub offset_x: i16,
    pub offset_y: i16,
    pub scale_x: f32,
    pub scale_y: f32,
}

/// VPADStatus
#[doc(alias = "VPADStatus")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Status {
    pub hold: Button,
    pub trigger: Button,
    pub release: Button,
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub accelerometer: AccStatus,
    /// 1.0 is 360 degrees per second.
    pub gyro: Vec3,
    /// 1.0 is 360 degrees.
    pub angle: Vec3,
    pub error: i8,
    /// Raw touch position, see [get_tp_calibrated_point].
    pub tp_normal: TouchData,
    pub tp_filtered1: TouchData,
    pub tp_filtered2: TouchData,
    pub direction: Direction,
    pub using_headphones: ffi::c_bool,
    pub mag: Vec3,
    /// 0-255.
    pub slide_volume: u8,
    pub battery: BatteryLevel,
    pub mic_status: MicStatus,
    pub slide_volume_ex: u8,
    _pad: ffi::unknown<8>,
}

/// VPADSamplingCallback
#[doc(alias = "VPADSamplingCallback")]
pub type SamplingCallback = Option<unsafe extern "C" fn(channel: Channel)>;

const _: () = {
    assert!(size_of::<Direction>() == 0x24);
    assert!(size_of::<AccStatus>() == 0x1C);
    assert!(size_of::<TouchData>() == 0x8);
    assert!(size_of::<TouchCalibrationParam>() == 0xC);
    assert!(size_of::<Status>() == 0xAC);
    assert!(core::mem::offset_of!(Status, tp_normal) == 0x52);
    assert!(core::mem::offset_of!(Status, direction) == 0x6C);
    assert!(core::mem::offset_of!(Status, slide_volume) == 0xA0);
};

unsafe extern "C" {
    /// VPADInit
    #[doc(alias = "VPADInit")]
    #[link_name = "VPADInit"]
    #[deprecated(note = "Not needed anymore")]
    pub unsafe fn init();

    /// VPADShutdown
    #[doc(alias = "VPADShutdown")]
    #[link_name = "VPADShutdown"]
    #[deprecated(note = "Not needed anymore")]
    pub unsafe fn shutdown();

    /// VPADRead
    ///
    /// Reads up to `count` samples into `buffers`, newest first. Returns the number of samples read.
    #[doc(alias = "VPADRead")]
    #[link_name = "VPADRead"]
    pub unsafe fn read(
        channel: Channel,
        buffers: *mut Status,
        count: u32,
        error: *mut Error,
    ) -> i32;

    /// VPADSetSamplingCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "VPADSetSamplingCallback")]
    #[link_name = "VPADSetSamplingCallback"]
    pub unsafe fn set_sampling_callback(
        channel: Channel,
        callback: SamplingCallback,
    ) -> SamplingCallback;

    /// VPADSetButtonProcMode
    #[doc(alias = "VPADSetButtonProcMode")]
    #[link_name = "VPADSetButtonProcMode"]
    pub unsafe fn set_button_proc_mode(channel: Channel, mode: ButtonProcMode);

    /// VPADGetButtonProcMode
    #[doc(alias = "VPADGetButtonProcMode")]
    #[link_name = "VPADGetButtonProcMode"]
    pub unsafe fn get_button_proc_mode(channel: Channel) -> ButtonProcMode;

    /// VPADSetBtnRepeat
    ///
    /// Sets [Button::REPEAT] every `pulse_sec` after a button is held for `delay_sec`. 0 disables it.
    #[doc(alias = "VPADSetBtnRepeat")]
    #[link_name = "VPADSetBtnRepeat"]
    pub unsafe fn set_btn_repeat(channel: Channel, delay_sec: f32, pulse_sec: f32);

    /// VPADEnableStickCrossClamp
    #[doc(alias = "VPADEnableStickCrossClamp")]
    #[link_name = "VPADEnableStickCrossClamp"]
    pub unsafe fn enable_stick_cross_clamp(channel: Channel);

    /// VPADDisableStickCrossClamp
    #[doc(alias = "VPADDisableStickCrossClamp")]
    #[link_name = "VPADDisableStickCrossClamp"]
    pub unsafe fn disable_stick_cross_clamp(channel: Channel);

    /// VPADSetLStickClampThreshold
    #[doc(alias = "VPADSetLStickClampThreshold")]
    #[link_name = "VPADSetLStickClampThreshold"]
    pub unsafe fn set_lstick_clamp_threshold(channel: Channel, max: i32, min: i32);

    /// VPADSetRStickClampThreshold
    #[doc(alias = "VPADSetRStickClampThreshold")]
    #[link_name = "VPADSetRStickClampThreshold"]
    pub unsafe fn set_rstick_clamp_threshold(channel: Channel, max: i32, min: i32);

    /// VPADGetLStickClampThreshold
    #[doc(alias = "VPADGetLStickClampThreshold")]
    #[link_name = "VPADGetLStickClampThreshold"]
    pub unsafe fn get_lstick_clamp_threshold(channel: Channel, max: *mut i32, min: *mut i32);

    /// VPADGetRStickClampThreshold
    #[doc(alias = "VPADGetRStickClampThreshold")]
    #[link_name = "VPADGetRStickClampThreshold"]
    pub unsafe fn get_rstick_clamp_threshold(channel: Channel, max: *mut i32, min: *mut i32);

    /// VPADSetStickOrigin
    #[doc(alias = "VPADSetStickOrigin")]
    #[link_name = "VPADSetStickOrigin"]
    pub unsafe fn set_stick_origin(channel: Channel);

    /// VPADEnableLStickZeroClamp
    #[doc(alias = "VPADEnableLStickZeroClamp")]
    #[link_name = "VPADEnableLStickZeroClamp"]
    pub unsafe fn enable_lstick_zero_clamp(channel: Channel);

    /// VPADEnableRStickZeroClamp
    #[doc(alias = "VPADEnableRStickZeroClamp")]
    #[link_name = "VPADEnableRStickZeroClamp"]
    pub unsafe fn enable_rstick_zero_clamp(channel: Channel);

    /// VPADDisableLStickZeroClamp
    #[doc(alias = "VPADDisableLStickZeroClamp")]
    #[link_name = "VPADDisableLStickZeroClamp"]
    pub unsafe fn disable_lstick_zero_clamp(channel: Channel);

    /// VPADDisableRStickZeroClamp
    #[doc(alias = "VPADDisableRStickZeroClamp")]
    #[link_name = "VPADDisableRStickZeroClamp"]
    pub unsafe fn disable_rstick_zero_clamp(channel: Channel);

    /// VPADSetCrossStickEmulationParamsL
    ///
    /// Configures the emulated D-pad buttons of the left stick.
    #[doc(alias = "VPADSetCrossStickEmulationParamsL")]
    #[link_name = "VPADSetCrossStickEmulationParamsL"]
    pub unsafe fn set_cross_stick_emulation_params_l(
        channel: Channel,
        rotation_deg: f32,
        range_deg: f32,
        radius: f32,
    );

    /// VPADSetCrossStickEmulationParamsR
    ///
    /// Configures the emulated D-pad buttons of the right stick.
    #[doc(alias = "VPADSetCrossStickEmulationParamsR")]
    #[link_name = "VPADSetCrossStickEmulationParamsR"]
    pub unsafe fn set_cross_stick_emulation_params_r(
        channel: Channel,
        rotation_deg: f32,
        range_deg: f32,
        radius: f32,
    );

    /// VPADGetCrossStickEmulationParamsL
    #[doc(alias = "VPADGetCrossStickEmulationParamsL")]
    #[link_name = "VPADGetCrossStickEmulationParamsL"]
    pub unsafe fn get_cross_stick_emulation_params_l(
        channel: Channel,
        rotation_deg: *mut f32,
        range_deg: *mut f32,
        radius: *mut f32,
    );

    /// VPADGetCrossStickEmulationParamsR
    #[doc(alias = "VPADGetCrossStickEmulationParamsR")]
    #[link_name = "VPADGetCrossStickEmulationParamsR"]
    pub unsafe fn get_cross_stick_emulation_params_r(
        channel: Channel,
        rotation_deg: *mut f32,
        range_deg: *mut f32,
        radius: *mut f32,
    );

    /// VPADSetAccParam
    #[doc(alias = "VPADSetAccParam")]
    #[link_name = "VPADSetAccParam"]
    pub unsafe fn set_acc_param(channel: Channel, play_radius: f32, sensitivity: f32);

    /// VPADGetAccParam
    #[doc(alias = "VPADGetAccParam")]
    #[link_name = "VPADGetAccParam"]
    pub unsafe fn get_acc_param(channel: Channel, play_radius: *mut f32, sensitivity: *mut f32);

    /// VPADSetAccPlayMode
    #[doc(alias = "VPADSetAccPlayMode")]
    #[link_name = "VPADSetAccPlayMode"]
    pub unsafe fn set_acc_play_mode(channel: Channel, mode: PlayMode);

    /// VPADGetAccPlayMode
    #[doc(alias = "VPADGetAccPlayMode")]
    #[link_name = "VPADGetAccPlayMode"]
    pub unsafe fn get_acc_play_mode(channel: Channel) -> PlayMode;

    /// VPADResetAccToDefault
    #[doc(alias = "VPADResetAccToDefault")]
    #[link_name = "VPADResetAccToDefault"]
    pub unsafe fn reset_acc_to_default(channel: Channel) -> i32;

    /// VPADStartAccCalibration
    #[doc(alias = "VPADStartAccCalibration")]
    #[link_name = "VPADStartAccCalibration"]
    pub unsafe fn start_acc_calibration(channel: Channel, unknown: u32);

    /// VPADSetGyroAngle
    #[doc(alias = "VPADSetGyroAngle")]
    #[link_name = "VPADSetGyroAngle"]
    pub unsafe fn set_gyro_angle(channel: Channel, ax: f32, ay: f32, az: f32);

    /// VPADSetGyroDirection
    #[doc(alias = "VPADSetGyroDirection")]
    #[link_name = "VPADSetGyroDirection"]
    pub unsafe fn set_gyro_direction(channel: Channel, direction: *const Direction);

    /// VPADSetGyroDirectionMag
    #[doc(alias = "VPADSetGyroDirectionMag")]
    #[link_name = "VPADSetGyroDirectionMag"]
    pub unsafe fn set_gyro_direction_mag(channel: Channel, mag: f32);

    /// VPADSetGyroMagnification
    #[doc(alias = "VPADSetGyroMagnification")]
    #[link_name = "VPADSetGyroMagnification"]
    pub unsafe fn set_gyro_magnification(channel: Channel, pitch: f32, yaw: f32, roll: f32);

    /// VPADEnableGyroZeroPlay
    #[doc(alias = "VPADEnableGyroZeroPlay")]
    #[link_name = "VPADEnableGyroZeroPlay"]
    pub unsafe fn enable_gyro_zero_play(channel: Channel);

    /// VPADEnableGyroDirRevise
    #[doc(alias = "VPADEnableGyroDirRevise")]
    #[link_name = "VPADEnableGyroDirRevise"]
    pub unsafe fn enable_gyro_dir_revise(channel: Channel);

    /// VPADEnableGyroAccRevise
    #[doc(alias = "VPADEnableGyroAccRevise")]
    #[link_name = "VPADEnableGyroAccRevise"]
    pub unsafe fn enable_gyro_acc_revise(channel: Channel);

    /// VPADDisableGyroZeroPlay
    #[doc(alias = "VPADDisableGyroZeroPlay")]
    #[link_name = "VPADDisableGyroZeroPlay"]
    pub unsafe fn disable_gyro_zero_play(channel: Channel);

    /// VPADDisableGyroDirRevise
    #[doc(alias = "VPADDisableGyroDirRevise")]
    #[link_name = "VPADDisableGyroDirRevise"]
    pub unsafe fn disable_gyro_dir_revise(channel: Channel);

    /// VPADDisableGyroAccRevise
    #[doc(alias = "VPADDisableGyroAccRevise")]
    #[link_name = "VPADDisableGyroAccRevise"]
    pub unsafe fn disable_gyro_acc_revise(channel: Channel);

    /// VPADIsEnableGyroZeroPlay
    ///
    /// Negative if disabled, otherwise the correction level.
    #[doc(alias = "VPADIsEnableGyroZeroPlay")]
    #[link_name = "VPADIsEnableGyroZeroPlay"]
    pub unsafe fn is_enable_gyro_zero_play(channel: Channel) -> f32;

    /// VPADIsEnableGyroZeroDrift
    ///
    /// Negative if disabled, otherwise the correction level.
    #[doc(alias = "VPADIsEnableGyroZeroDrift")]
    #[link_name = "VPADIsEnableGyroZeroDrift"]
    pub unsafe fn is_enable_gyro_zero_drift(channel: Channel) -> f32;

    /// VPADIsEnableGyroDirRevise
    ///
    /// Negative if disabled, otherwise the correction level.
    #[doc(alias = "VPADIsEnableGyroDirRevise")]
    #[link_name = "VPADIsEnableGyroDirRevise"]
    pub unsafe fn is_enable_gyro_dir_revise(channel: Channel) -> f32;

    /// VPADIsEnableGyroAccRevise
    ///
    /// Negative if disabled, otherwise the correction level.
    #[doc(alias = "VPADIsEnableGyroAccRevise")]
    #[link_name = "VPADIsEnableGyroAccRevise"]
    pub unsafe fn is_enable_gyro_acc_revise(channel: Channel) -> f32;

    /// VPADSetGyroZeroPlayParam
    #[doc(alias = "VPADSetGyroZeroPlayParam")]
    #[link_name = "VPADSetGyroZeroPlayParam"]
    pub unsafe fn set_gyro_zero_play_param(channel: Channel, radius: f32);

    /// VPADGetGyroZeroPlayParam
    #[doc(alias = "VPADGetGyroZeroPlayParam")]
    #[link_name = "VPADGetGyroZeroPlayParam"]
    pub unsafe fn get_gyro_zero_play_param(channel: Channel, radius: *mut f32);

    /// VPADInitGyroZeroPlayParam
    #[doc(alias = "VPADInitGyroZeroPlayParam")]
    #[link_name = "VPADInitGyroZeroPlayParam"]
    pub unsafe fn init_gyro_zero_play_param(channel: Channel);

    /// VPADSetGyroDirReviseParam
    #[doc(alias = "VPADSetGyroDirReviseParam")]
    #[link_name = "VPADSetGyroDirReviseParam"]
    pub unsafe fn set_gyro_dir_revise_param(channel: Channel, revise_pw: f32);

    /// VPADGetGyroDirReviseParam
    #[doc(alias = "VPADGetGyroDirReviseParam")]
    #[link_name = "VPADGetGyroDirReviseParam"]
    pub unsafe fn get_gyro_dir_revise_param(channel: Channel, revise_pw: *mut f32);

    /// VPADInitGyroDirReviseParam
    #[doc(alias = "VPADInitGyroDirReviseParam")]
    #[link_name = "VPADInitGyroDirReviseParam"]
    pub unsafe fn init_gyro_dir_revise_param(channel: Channel);

    /// VPADSetGyroDirReviseBase
    #[doc(alias = "VPADSetGyroDirReviseBase")]
    #[link_name = "VPADSetGyroDirReviseBase"]
    pub unsafe fn set_gyro_dir_revise_base(channel: Channel, base: *const Direction);

    /// VPADSetGyroAccReviseParam
    #[doc(alias = "VPADSetGyroAccReviseParam")]
    #[link_name = "VPADSetGyroAccReviseParam"]
    pub unsafe fn set_gyro_acc_revise_param(channel: Channel, revise_pw: f32, revise_range: f32);

    /// VPADGetGyroAccReviseParam
    #[doc(alias = "VPADGetGyroAccReviseParam")]
    #[link_name = "VPADGetGyroAccReviseParam"]
    pub unsafe fn get_gyro_acc_revise_param(
        channel: Channel,
        revise_pw: *mut f32,
        revise_range: *mut f32,
    );

    /// VPADInitGyroAccReviseParam
    #[doc(alias = "VPADInitGyroAccReviseParam")]
    #[link_name = "VPADInitGyroAccReviseParam"]
    pub unsafe fn init_gyro_acc_revise_param(channel: Channel);

    /// VPADSetGyroZeroDriftMode
    #[doc(alias = "VPADSetGyroZeroDriftMode")]
    #[link_name = "VPADSetGyroZeroDriftMode"]
    pub unsafe fn set_gyro_zero_drift_mode(channel: Channel, mode: GyroZeroDriftMode);

    /// VPADGetGyroZeroDriftMode
    #[doc(alias = "VPADGetGyroZeroDriftMode")]
    #[link_name = "VPADGetGyroZeroDriftMode"]
    pub unsafe fn get_gyro_zero_drift_mode(channel: Channel, mode: *mut GyroZeroDriftMode);

    /// VPADInitGyroZeroDriftMode
    #[doc(alias = "VPADInitGyroZeroDriftMode")]
    #[link_name = "VPADInitGyroZeroDriftMode"]
    pub unsafe fn init_gyro_zero_drift_mode(channel: Channel);

    /// VPADIsStartedGyroMagRevise
    ///
    /// Negative if not started, otherwise the correction level.
    #[doc(alias = "VPADIsStartedGyroMagRevise")]
    #[link_name = "VPADIsStartedGyroMagRevise"]
    pub unsafe fn is_started_gyro_mag_revise(channel: Channel) -> f32;

    /// VPADStartGyroMagRevise
    #[doc(alias = "VPADStartGyroMagRevise")]
    #[link_name = "VPADStartGyroMagRevise"]
    pub unsafe fn start_gyro_mag_revise(channel: Channel);

    /// VPADStopGyroMagRevise
    #[doc(alias = "VPADStopGyroMagRevise")]
    #[link_name = "VPADStopGyroMagRevise"]
    pub unsafe fn stop_gyro_mag_revise(channel: Channel);

    /// VPADSetGyroMagReviseParam
    #[doc(alias = "VPADSetGyroMagReviseParam")]
    #[link_name = "VPADSetGyroMagReviseParam"]
    pub unsafe fn set_gyro_mag_revise_param(channel: Channel, revise_pw: f32, play_angle: f32);

    /// VPADGetGyroMagReviseParam
    #[doc(alias = "VPADGetGyroMagReviseParam")]
    #[link_name = "VPADGetGyroMagReviseParam"]
    pub unsafe fn get_gyro_mag_revise_param(
        channel: Channel,
        revise_pw: *mut f32,
        play_angle: *mut f32,
    );

    /// VPADCalcTPCalibrationParam
    ///
    /// Computes the calibration from two raw touch points and their screen positions.
    #[doc(alias = "VPADCalcTPCalibrationParam")]
    #[link_name = "VPADCalcTPCalibrationParam"]
    pub unsafe fn calc_tp_calibration_param(
        param: *mut TouchCalibrationParam,
        raw_x1: u16,
        raw_y1: u16,
        x1: u16,
        y1: u16,
        raw_x2: u16,
        raw_y2: u16,
        x2: u16,
        y2: u16,
    ) -> i16;

    /// VPADSetTPCalibrationParam
    #[doc(alias = "VPADSetTPCalibrationParam")]
    #[link_name = "VPADSetTPCalibrationParam"]
    pub unsafe fn set_tp_calibration_param(channel: Channel, param: *const TouchCalibrationParam);

    /// VPADGetTPCalibrationParam
    #[doc(alias = "VPADGetTPCalibrationParam")]
    #[link_name = "VPADGetTPCalibrationParam"]
    pub unsafe fn get_tp_calibration_param(channel: Channel, param: *mut TouchCalibrationParam);

    /// VPADGetTPCalibratedPoint
    ///
    /// Converts a raw touch point to 1280x720 screen coordinates.
    #[doc(alias = "VPADGetTPCalibratedPoint")]
    #[link_name = "VPADGetTPCalibratedPoint"]
    pub unsafe fn get_tp_calibrated_point(
        channel: Channel,
        calibrated: *mut TouchData,
        raw: *const TouchData,
    );

    /// VPADGetTPCalibratedPointEx
    ///
    /// Same as [get_tp_calibrated_point] with a chosen resolution.
    #[doc(alias = "VPADGetTPCalibratedPointEx")]
    #[link_name = "VPADGetTPCalibratedPointEx"]
    pub unsafe fn get_tp_calibrated_point_ex(
        channel: Channel,
        resolution: TouchPadResolution,
        calibrated: *mut TouchData,
        raw: *const TouchData,
    );

    /// VPADResetTPToDefault
    #[doc(alias = "VPADResetTPToDefault")]
    #[link_name = "VPADResetTPToDefault"]
    pub unsafe fn reset_tp_to_default(channel: Channel) -> i32;

    /// VPADWriteTPCalibrationValueToEEPROM
    #[doc(alias = "VPADWriteTPCalibrationValueToEEPROM")]
    #[link_name = "VPADWriteTPCalibrationValueToEEPROM"]
    pub unsafe fn write_tp_calibration_value_to_eeprom(
        channel: Channel,
        old_min_x: u16,
        old_min_y: u16,
        new_min_x: u16,
        new_min_y: u16,
        old_max_x: u16,
        old_max_y: u16,
        new_max_x: u16,
        new_max_y: u16,
    ) -> i32;

    /// VPADControlMotor
    ///
    /// Plays a rumble pattern of `length` bits (up to [MOTOR_PATTERN_LENGTH_MAX]), one bit per 1/120 s. A length of 0 stops the rumble. Does nothing while the TV Menu is open. Negative on failure.
    #[doc(alias = "VPADControlMotor")]
    #[link_name = "VPADControlMotor"]
    pub unsafe fn control_motor(channel: Channel, pattern: *const u8, length: u8) -> i32;

    /// VPADStopMotor
    #[doc(alias = "VPADStopMotor")]
    #[link_name = "VPADStopMotor"]
    pub unsafe fn stop_motor(channel: Channel);

    /// VPADSetSensorBar
    ///
    /// Negative on failure.
    #[doc(alias = "VPADSetSensorBar")]
    #[link_name = "VPADSetSensorBar"]
    pub unsafe fn set_sensor_bar(channel: Channel, on: ffi::c_bool) -> i32;

    /// VPADSetLcdMode
    ///
    /// Negative on failure.
    #[doc(alias = "VPADSetLcdMode")]
    #[link_name = "VPADSetLcdMode"]
    pub unsafe fn set_lcd_mode(channel: Channel, mode: LcdMode) -> i32;

    /// VPADGetLcdMode
    ///
    /// Negative on failure.
    #[doc(alias = "VPADGetLcdMode")]
    #[link_name = "VPADGetLcdMode"]
    pub unsafe fn get_lcd_mode(channel: Channel, mode: *mut LcdMode) -> i32;

    /// VPADGetTVMenuStatus
    ///
    /// Whether the TV Menu is open.
    #[doc(alias = "VPADGetTVMenuStatus")]
    #[link_name = "VPADGetTVMenuStatus"]
    pub unsafe fn get_tv_menu_status(channel: Channel) -> ffi::c_bool;

    /// VPADSetTVMenuInvalid
    #[doc(alias = "VPADSetTVMenuInvalid")]
    #[link_name = "VPADSetTVMenuInvalid"]
    pub unsafe fn set_tv_menu_invalid(channel: Channel, invalid: ffi::c_bool);

    /// VPADDisablePowerButton
    #[doc(alias = "VPADDisablePowerButton")]
    #[link_name = "VPADDisablePowerButton"]
    pub unsafe fn disable_power_button(channel: Channel);

    /// VPADEnablePowerButton
    #[doc(alias = "VPADEnablePowerButton")]
    #[link_name = "VPADEnablePowerButton"]
    pub unsafe fn enable_power_button(channel: Channel);

    /// VPADSetAudioVolumeOverride
    #[doc(alias = "VPADSetAudioVolumeOverride")]
    #[link_name = "VPADSetAudioVolumeOverride"]
    pub unsafe fn set_audio_volume_override(
        channel: Channel,
        enable: ffi::c_bool,
        volume: u8,
    ) -> i32;
}

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
