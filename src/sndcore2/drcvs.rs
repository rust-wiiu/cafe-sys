//! AX DRC virtual surround

use super::Error;
use crate::ffi;

/// AXDRCVSMode
#[doc(alias = "AXDRCVSMode", alias = "AXDRCVSModeType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mode(pub u32);

impl Mode {
    pub const OFF: Self = Self(0);
    pub const ON_NORMAL: Self = Self(1);
    pub const ON_FRONT_BYPASS: Self = Self(2);
}

/// AXDRCVSOutput
#[doc(alias = "AXDRCVSOutput", alias = "AXDRCVSOutputMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Output {
    #[default]
    Speaker = 0,
    Headphone = 1,
}

/// AXDRCVSLC
///
/// Whether to use locked caches.
#[doc(alias = "AXDRCVSLC", alias = "AXDRCVSLCMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockedCache {
    #[default]
    NotUsed = 0,
    Used = 1,
    UsedMax = 2,
}

/// AXDRCVSSpeakerPosition
#[doc(alias = "AXDRCVSSpeakerPosition")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeakerPosition {
    #[default]
    Wide = 0,
    Square = 1,
}

/// AXDRCVSSurroundLevelGain
///
/// Surround channel level when downmixing.
#[doc(alias = "AXDRCVSSurroundLevelGain")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurroundLevelGain {
    #[default]
    Gain1_0 = 0,
    Gain0_707 = 1,
    Gain0_5 = 2,
}

unsafe extern "C" {
    /// AXGetDRCVSMode
    #[doc(alias = "AXGetDRCVSMode")]
    #[link_name = "AXGetDRCVSMode"]
    pub unsafe fn get_mode(mode: *mut Mode) -> Error;

    /// AXSetDRCVSMode
    ///
    /// Only active if surround is selected for the GamePad in the system settings.
    #[doc(alias = "AXSetDRCVSMode")]
    #[link_name = "AXSetDRCVSMode"]
    pub unsafe fn set_mode(mode: Mode) -> Error;

    /// AXSetDRCVSLC
    #[doc(alias = "AXSetDRCVSLC")]
    #[link_name = "AXSetDRCVSLC"]
    pub unsafe fn set_lc(mode: LockedCache) -> Error;

    /// AXSetDRCVSSpeakerPosition
    #[doc(alias = "AXSetDRCVSSpeakerPosition")]
    #[link_name = "AXSetDRCVSSpeakerPosition"]
    pub unsafe fn set_speaker_position(output: Output, position: SpeakerPosition) -> Error;

    /// AXSetDRCVSSurroundDepth
    #[doc(alias = "AXSetDRCVSSurroundDepth")]
    #[link_name = "AXSetDRCVSSurroundDepth"]
    pub unsafe fn set_surround_depth(output: Output, depth: f32) -> Error;

    /// AXSetDRCVSDownmixBalance
    ///
    /// Volume ratio of the downmix to the virtual surround component.
    #[doc(alias = "AXSetDRCVSDownmixBalance")]
    #[link_name = "AXSetDRCVSDownmixBalance"]
    pub unsafe fn set_downmix_balance(output: Output, balance: f32) -> Error;

    /// AXSetDRCVSSurroundLevelGain
    #[doc(alias = "AXSetDRCVSSurroundLevelGain")]
    #[link_name = "AXSetDRCVSSurroundLevelGain"]
    pub unsafe fn set_surround_level_gain(gain: SurroundLevelGain) -> Error;

    /// AXSetDRCVSOutputGain
    #[doc(alias = "AXSetDRCVSOutputGain")]
    #[link_name = "AXSetDRCVSOutputGain"]
    pub unsafe fn set_output_gain(output: Output, gain: f32) -> Error;

    /// AXSetDRCVSLimiter
    #[doc(alias = "AXSetDRCVSLimiter")]
    #[link_name = "AXSetDRCVSLimiter"]
    pub unsafe fn set_limiter(enable: ffi::c_bool) -> Error;

    /// AXSetDRCVSLimiterThreshold
    #[doc(alias = "AXSetDRCVSLimiterThreshold")]
    #[link_name = "AXSetDRCVSLimiterThreshold"]
    pub unsafe fn set_limiter_threshold(level: f32) -> Error;
}
