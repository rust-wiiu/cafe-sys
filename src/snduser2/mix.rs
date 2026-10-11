//! MIX
//!
//! Voice mixer on top of AX. Levels are in 0.1 dB.

use crate::ffi;
use crate::sndcore2::device::{AuxId, DeviceType};
use crate::sndcore2::voice::Voice;

/// MIX_SOUND_MODE
#[doc(alias = "MIX_SOUND_MODE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundMode(pub u32);

impl SoundMode {
    /// Ignores left and right pan.
    pub const MONO: Self = Self(0);
    pub const STEREO: Self = Self(1);
    pub const SURROUND: Self = Self(2);
    pub const DPL2: Self = Self(3);
    pub const SIX_CHANNEL: Self = Self(4);
}

/// MIX_MODE
///
/// Channel mixing mode flags.
#[doc(alias = "MIX_MODE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mode(pub u32);

impl Mode {
    pub const AUX_A_PREFADER: Self = Self(0x0000_0001);
    pub const AUX_B_PREFADER: Self = Self(0x0000_0002);
    pub const AUX_C_PREFADER: Self = Self(0x0000_0004);
    pub const MUTE: Self = Self(0x0000_0008);
    pub const DRC_PREFADER: Self = Self(0x0000_0010);
    pub const UPDATE_INPUT: Self = Self(0x1000_0000);
    pub const UPDATE_TARGET_INPUT: Self = Self(0x2000_0000);
    pub const UPDATE_MIX: Self = Self(0x4000_0000);
    pub const UPDATE_TARGET_MIX: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for Mode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MIX_MODE_AUX*_PREFADER
///
/// Controller speaker mixing mode flags.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtMode(pub u32);

impl RmtMode {
    pub const AUX0_PREFADER: Self = Self(0x1);
    pub const AUX1_PREFADER: Self = Self(0x2);
    pub const AUX2_PREFADER: Self = Self(0x4);
    pub const AUX3_PREFADER: Self = Self(0x8);
}

impl core::ops::BitOr for RmtMode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MIXCntrlStruct
#[doc(alias = "MIXCntrlStruct")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Control {
    /// Aux sends, indexed by [AuxId].
    pub aux: [i16; 3],
    pub pan: i16,
    pub span: i16,
    pub fader: i16,
    pub lfe: i16,
}

/// MIXCHVOLSTRUCT
#[doc(alias = "MIXCHVOLSTRUCT")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelVolume {
    pub current: u16,
    pub target: u16,
}

/// MIXChannel
///
/// Internal mixer state of a voice.
#[doc(alias = "MIXChannel")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel {
    pub voice: *mut Voice,
    pub mix_update: u32,
    pub input_level: i16,
    pub input_volume: ChannelVolume,
    pub tv_control: [Control; 1],
    pub tv_channels: [[i16; 6]; 1],
    pub tv_channel_volume: [[[ChannelVolume; 6]; 4]; 1],
    pub tv_mode: [u32; 1],
    pub drc_channel_volume: [[[ChannelVolume; 4]; 4]; 2],
    pub drc_control: [Control; 2],
    pub drc_channels: [[i16; 4]; 2],
    pub drc_mode: [u32; 2],
    pub rmt_channel_volume: [[[ChannelVolume; 1]; 4]; 4],
    pub rmt_control: [Control; 4],
    pub rmt_channels: [[i16; 1]; 4],
    pub rmt_mode: [u32; 4],
}

/// MIXRmtChannel
#[doc(alias = "MIXRmtChannel")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtChannel {
    pub mode: RmtMode,
    pub fader: [ffi::c_int; 4],
    pub aux: [ffi::c_int; 4],
    /// Two values each for main 0, aux 0, main 1, aux 1, ... main 3, aux 3.
    pub volumes: [u16; 16],
}

const _: () = {
    assert!(size_of::<Control>() == 0xE);
    assert!(size_of::<ChannelVolume>() == 0x4);
    assert!(size_of::<RmtChannel>() == 0x44);
};

unsafe extern "C" {
    /// MIXInit
    ///
    /// Must be called before any other mixer function.
    #[doc(alias = "MIXInit")]
    #[link_name = "MIXInit"]
    pub unsafe fn init();

    /// MIXInitSpecifyMem
    ///
    /// Same as [init], but uses `mem` instead of allocating. It needs `size_of::<Channel>() + size_of::<RmtChannel>()` bytes per voice.
    #[doc(alias = "MIXInitSpecifyMem")]
    #[link_name = "MIXInitSpecifyMem"]
    pub unsafe fn init_specify_mem(mem: *mut ffi::c_void);

    /// MIXQuit
    #[doc(alias = "MIXQuit")]
    #[link_name = "MIXQuit"]
    pub unsafe fn quit();

    /// MIXUpdateSettings
    ///
    /// Applies the mixer settings to the voices. Call once from the AX frame callback.
    #[doc(alias = "MIXUpdateSettings")]
    #[link_name = "MIXUpdateSettings"]
    pub unsafe fn update_settings();

    /// MIXSetSoundMode
    #[doc(alias = "MIXSetSoundMode")]
    #[link_name = "MIXSetSoundMode"]
    pub unsafe fn set_sound_mode(mode: SoundMode);

    /// MIXGetSoundMode
    #[doc(alias = "MIXGetSoundMode")]
    #[link_name = "MIXGetSoundMode"]
    pub unsafe fn get_sound_mode() -> SoundMode;

    /// MIXAssignChannel
    ///
    /// Assigns a mixer channel to an acquired voice and resets its controls. Must be called before any other per-voice function.
    #[doc(alias = "MIXAssignChannel")]
    #[link_name = "MIXAssignChannel"]
    pub unsafe fn assign_channel(voice: *mut Voice);

    /// MIXReleaseChannel
    #[doc(alias = "MIXReleaseChannel")]
    #[link_name = "MIXReleaseChannel"]
    pub unsafe fn release_channel(voice: *mut Voice);

    /// MIXInitInputControl
    ///
    /// `input` is in 0.1 dB.
    #[doc(alias = "MIXInitInputControl")]
    #[link_name = "MIXInitInputControl"]
    pub unsafe fn init_input_control(voice: *mut Voice, input: i16, mode: Mode);

    /// MIXInitDeviceControl
    ///
    /// Call after [assign_channel].
    #[doc(alias = "MIXInitDeviceControl")]
    #[link_name = "MIXInitDeviceControl"]
    pub unsafe fn init_device_control(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        control: *mut Control,
        mode: Mode,
    );

    /// MIXResetDeviceControl
    #[doc(alias = "MIXResetDeviceControl")]
    #[link_name = "MIXResetDeviceControl"]
    pub unsafe fn reset_device_control(voice: *mut Voice, device: DeviceType, id: u32);

    /// MIXResetAllDeviceControls
    #[doc(alias = "MIXResetAllDeviceControls")]
    #[link_name = "MIXResetAllDeviceControls"]
    pub unsafe fn reset_all_device_controls(voice: *mut Voice);

    /// MIXDeviceAuxPostFader
    #[doc(alias = "MIXDeviceAuxPostFader")]
    #[link_name = "MIXDeviceAuxPostFader"]
    pub unsafe fn device_aux_post_fader(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
    );

    /// MIXDeviceAuxPreFader
    #[doc(alias = "MIXDeviceAuxPreFader")]
    #[link_name = "MIXDeviceAuxPreFader"]
    pub unsafe fn device_aux_pre_fader(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
    );

    /// MIXDeviceAuxIsPostFader
    #[doc(alias = "MIXDeviceAuxIsPostFader")]
    #[link_name = "MIXDeviceAuxIsPostFader"]
    pub unsafe fn device_aux_is_post_fader(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
    ) -> ffi::c_bool;

    /// MIXSetDeviceAux
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "MIXSetDeviceAux")]
    #[link_name = "MIXSetDeviceAux"]
    pub unsafe fn set_device_aux(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        db: i16,
    );

    /// MIXAdjustDeviceAux
    #[doc(alias = "MIXAdjustDeviceAux")]
    #[link_name = "MIXAdjustDeviceAux"]
    pub unsafe fn adjust_device_aux(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        db: i16,
    );

    /// MIXGetDeviceAux
    #[doc(alias = "MIXGetDeviceAux")]
    #[link_name = "MIXGetDeviceAux"]
    pub unsafe fn get_device_aux(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
    ) -> i16;

    /// MIXSetDevicePan
    #[doc(alias = "MIXSetDevicePan")]
    #[link_name = "MIXSetDevicePan"]
    pub unsafe fn set_device_pan(voice: *mut Voice, device: DeviceType, id: u32, pan: i16);

    /// MIXAdjustDevicePan
    #[doc(alias = "MIXAdjustDevicePan")]
    #[link_name = "MIXAdjustDevicePan"]
    pub unsafe fn adjust_device_pan(voice: *mut Voice, device: DeviceType, id: u32, pan: i16);

    /// MIXGetDevicePan
    #[doc(alias = "MIXGetDevicePan")]
    #[link_name = "MIXGetDevicePan"]
    pub unsafe fn get_device_pan(voice: *mut Voice, device: DeviceType, id: u32) -> i16;

    /// MIXSetDeviceSPan
    #[doc(alias = "MIXSetDeviceSPan")]
    #[link_name = "MIXSetDeviceSPan"]
    pub unsafe fn set_device_span(voice: *mut Voice, device: DeviceType, id: u32, span: i16);

    /// MIXAdjustDeviceSPan
    #[doc(alias = "MIXAdjustDeviceSPan")]
    #[link_name = "MIXAdjustDeviceSPan"]
    pub unsafe fn adjust_device_span(voice: *mut Voice, device: DeviceType, id: u32, span: i16);

    /// MIXGetDeviceSPan
    #[doc(alias = "MIXGetDeviceSPan")]
    #[link_name = "MIXGetDeviceSPan"]
    pub unsafe fn get_device_span(voice: *mut Voice, device: DeviceType, id: u32) -> i16;

    /// MIXSetDeviceFader
    ///
    /// `fader` is in 0.1 dB.
    #[doc(alias = "MIXSetDeviceFader")]
    #[link_name = "MIXSetDeviceFader"]
    pub unsafe fn set_device_fader(voice: *mut Voice, device: DeviceType, id: u32, fader: i16);

    /// MIXAdjustDeviceFader
    #[doc(alias = "MIXAdjustDeviceFader")]
    #[link_name = "MIXAdjustDeviceFader"]
    pub unsafe fn adjust_device_fader(voice: *mut Voice, device: DeviceType, id: u32, fader: i16);

    /// MIXGetDeviceFader
    #[doc(alias = "MIXGetDeviceFader")]
    #[link_name = "MIXGetDeviceFader"]
    pub unsafe fn get_device_fader(voice: *mut Voice, device: DeviceType, id: u32) -> i16;

    /// MIXSetDeviceLFE
    #[doc(alias = "MIXSetDeviceLFE")]
    #[link_name = "MIXSetDeviceLFE"]
    pub unsafe fn set_device_lfe(voice: *mut Voice, device: DeviceType, id: u32, lfe: i16);

    /// MIXAdjustDeviceLFE
    #[doc(alias = "MIXAdjustDeviceLFE")]
    #[link_name = "MIXAdjustDeviceLFE"]
    pub unsafe fn adjust_device_lfe(voice: *mut Voice, device: DeviceType, id: u32, lfe: i16);

    /// MIXGetDeviceLFE
    #[doc(alias = "MIXGetDeviceLFE")]
    #[link_name = "MIXGetDeviceLFE"]
    pub unsafe fn get_device_lfe(voice: *mut Voice, device: DeviceType, id: u32) -> i16;

    /// MIXSetDeviceSoundMode
    ///
    /// Applies to all instances of `device`. Must match the AX device mode.
    #[doc(alias = "MIXSetDeviceSoundMode")]
    #[link_name = "MIXSetDeviceSoundMode"]
    pub unsafe fn set_device_sound_mode(device: DeviceType, mode: SoundMode);

    /// MIXGetDeviceSoundMode
    #[doc(alias = "MIXGetDeviceSoundMode")]
    #[link_name = "MIXGetDeviceSoundMode"]
    pub unsafe fn get_device_sound_mode(device: DeviceType) -> SoundMode;

    /// MIXInitChannel
    ///
    /// Levels are in 0.1 dB.
    #[doc(alias = "MIXInitChannel")]
    #[link_name = "MIXInitChannel"]
    pub unsafe fn init_channel(
        voice: *mut Voice,
        mode: Mode,
        input: ffi::c_int,
        aux_a: ffi::c_int,
        aux_b: ffi::c_int,
        aux_c: ffi::c_int,
        pan: ffi::c_int,
        span: ffi::c_int,
        fader: ffi::c_int,
    );

    /// MIXResetControls
    #[doc(alias = "MIXResetControls")]
    #[link_name = "MIXResetControls"]
    pub unsafe fn reset_controls(voice: *mut Voice);

    /// MIXSetInput
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "MIXSetInput")]
    #[link_name = "MIXSetInput"]
    pub unsafe fn set_input(voice: *mut Voice, db: ffi::c_int);

    /// MIXAdjustInput
    #[doc(alias = "MIXAdjustInput")]
    #[link_name = "MIXAdjustInput"]
    pub unsafe fn adjust_input(voice: *mut Voice, db: ffi::c_int);

    /// MIXGetInput
    #[doc(alias = "MIXGetInput")]
    #[link_name = "MIXGetInput"]
    pub unsafe fn get_input(voice: *mut Voice) -> ffi::c_int;

    /// MIXAuxAPostFader
    #[doc(alias = "MIXAuxAPostFader")]
    #[link_name = "MIXAuxAPostFader"]
    pub unsafe fn aux_a_post_fader(voice: *mut Voice);

    /// MIXAuxAPreFader
    #[doc(alias = "MIXAuxAPreFader")]
    #[link_name = "MIXAuxAPreFader"]
    pub unsafe fn aux_a_pre_fader(voice: *mut Voice);

    /// MIXAuxAIsPostFader
    #[doc(alias = "MIXAuxAIsPostFader")]
    #[link_name = "MIXAuxAIsPostFader"]
    pub unsafe fn aux_a_is_post_fader(voice: *mut Voice) -> ffi::c_bool;

    /// MIXSetAuxA
    #[doc(alias = "MIXSetAuxA")]
    #[link_name = "MIXSetAuxA"]
    pub unsafe fn set_aux_a(voice: *mut Voice, db: ffi::c_int);

    /// MIXAdjustAuxA
    #[doc(alias = "MIXAdjustAuxA")]
    #[link_name = "MIXAdjustAuxA"]
    pub unsafe fn adjust_aux_a(voice: *mut Voice, db: ffi::c_int);

    /// MIXGetAuxA
    #[doc(alias = "MIXGetAuxA")]
    #[link_name = "MIXGetAuxA"]
    pub unsafe fn get_aux_a(voice: *mut Voice) -> ffi::c_int;

    /// MIXAuxBPostFader
    #[doc(alias = "MIXAuxBPostFader")]
    #[link_name = "MIXAuxBPostFader"]
    pub unsafe fn aux_b_post_fader(voice: *mut Voice);

    /// MIXAuxBPreFader
    #[doc(alias = "MIXAuxBPreFader")]
    #[link_name = "MIXAuxBPreFader"]
    pub unsafe fn aux_b_pre_fader(voice: *mut Voice);

    /// MIXAuxBIsPostFader
    #[doc(alias = "MIXAuxBIsPostFader")]
    #[link_name = "MIXAuxBIsPostFader"]
    pub unsafe fn aux_b_is_post_fader(voice: *mut Voice) -> ffi::c_bool;

    /// MIXSetAuxB
    #[doc(alias = "MIXSetAuxB")]
    #[link_name = "MIXSetAuxB"]
    pub unsafe fn set_aux_b(voice: *mut Voice, db: ffi::c_int);

    /// MIXAdjustAuxB
    #[doc(alias = "MIXAdjustAuxB")]
    #[link_name = "MIXAdjustAuxB"]
    pub unsafe fn adjust_aux_b(voice: *mut Voice, db: ffi::c_int);

    /// MIXGetAuxB
    #[doc(alias = "MIXGetAuxB")]
    #[link_name = "MIXGetAuxB"]
    pub unsafe fn get_aux_b(voice: *mut Voice) -> ffi::c_int;

    /// MIXAuxCPostFader
    #[doc(alias = "MIXAuxCPostFader")]
    #[link_name = "MIXAuxCPostFader"]
    pub unsafe fn aux_c_post_fader(voice: *mut Voice);

    /// MIXAuxCPreFader
    #[doc(alias = "MIXAuxCPreFader")]
    #[link_name = "MIXAuxCPreFader"]
    pub unsafe fn aux_c_pre_fader(voice: *mut Voice);

    /// MIXAuxCIsPostFader
    #[doc(alias = "MIXAuxCIsPostFader")]
    #[link_name = "MIXAuxCIsPostFader"]
    pub unsafe fn aux_c_is_post_fader(voice: *mut Voice) -> ffi::c_bool;

    /// MIXSetAuxC
    #[doc(alias = "MIXSetAuxC")]
    #[link_name = "MIXSetAuxC"]
    pub unsafe fn set_aux_c(voice: *mut Voice, db: ffi::c_int);

    /// MIXAdjustAuxC
    #[doc(alias = "MIXAdjustAuxC")]
    #[link_name = "MIXAdjustAuxC"]
    pub unsafe fn adjust_aux_c(voice: *mut Voice, db: ffi::c_int);

    /// MIXGetAuxC
    #[doc(alias = "MIXGetAuxC")]
    #[link_name = "MIXGetAuxC"]
    pub unsafe fn get_aux_c(voice: *mut Voice) -> ffi::c_int;

    /// MIXSetPan
    #[doc(alias = "MIXSetPan")]
    #[link_name = "MIXSetPan"]
    pub unsafe fn set_pan(voice: *mut Voice, pan: ffi::c_int);

    /// MIXAdjustPan
    #[doc(alias = "MIXAdjustPan")]
    #[link_name = "MIXAdjustPan"]
    pub unsafe fn adjust_pan(voice: *mut Voice, pan: ffi::c_int);

    /// MIXGetPan
    #[doc(alias = "MIXGetPan")]
    #[link_name = "MIXGetPan"]
    pub unsafe fn get_pan(voice: *mut Voice) -> ffi::c_int;

    /// MIXSetSPan
    #[doc(alias = "MIXSetSPan")]
    #[link_name = "MIXSetSPan"]
    pub unsafe fn set_span(voice: *mut Voice, span: ffi::c_int);

    /// MIXAdjustSPan
    #[doc(alias = "MIXAdjustSPan")]
    #[link_name = "MIXAdjustSPan"]
    pub unsafe fn adjust_span(voice: *mut Voice, span: ffi::c_int);

    /// MIXGetSPan
    #[doc(alias = "MIXGetSPan")]
    #[link_name = "MIXGetSPan"]
    pub unsafe fn get_span(voice: *mut Voice) -> ffi::c_int;

    /// MIXSetFader
    #[doc(alias = "MIXSetFader")]
    #[link_name = "MIXSetFader"]
    pub unsafe fn set_fader(voice: *mut Voice, db: ffi::c_int);

    /// MIXAdjustFader
    #[doc(alias = "MIXAdjustFader")]
    #[link_name = "MIXAdjustFader"]
    pub unsafe fn adjust_fader(voice: *mut Voice, db: ffi::c_int);

    /// MIXGetFader
    #[doc(alias = "MIXGetFader")]
    #[link_name = "MIXGetFader"]
    pub unsafe fn get_fader(voice: *mut Voice) -> ffi::c_int;

    /// MIXMute
    #[doc(alias = "MIXMute")]
    #[link_name = "MIXMute"]
    pub unsafe fn mute(voice: *mut Voice);

    /// MIXUnMute
    #[doc(alias = "MIXUnMute")]
    #[link_name = "MIXUnMute"]
    pub unsafe fn unmute(voice: *mut Voice);

    /// MIXIsMute
    #[doc(alias = "MIXIsMute")]
    #[link_name = "MIXIsMute"]
    pub unsafe fn is_mute(voice: *mut Voice) -> ffi::c_bool;

    /// MIXDRCInitVoice
    ///
    /// Levels are in 0.1 dB.
    #[doc(alias = "MIXDRCInitVoice")]
    #[link_name = "MIXDRCInitVoice"]
    pub unsafe fn drc_init_voice(
        voice: *mut Voice,
        id: u32,
        mode: Mode,
        aux: ffi::c_int,
        pan: ffi::c_int,
        span: ffi::c_int,
        fader: ffi::c_int,
    );

    /// MIXDRCInitChannel
    #[doc(alias = "MIXDRCInitChannel")]
    #[link_name = "MIXDRCInitChannel"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_init_channel(
        voice: *mut Voice,
        mode: Mode,
        aux: ffi::c_int,
        pan: ffi::c_int,
        fader: ffi::c_int,
    );

    /// MIXDRCAuxPostFader
    #[doc(alias = "MIXDRCAuxPostFader")]
    #[link_name = "MIXDRCAuxPostFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_aux_post_fader(voice: *mut Voice);

    /// MIXDRCAuxPreFader
    #[doc(alias = "MIXDRCAuxPreFader")]
    #[link_name = "MIXDRCAuxPreFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_aux_pre_fader(voice: *mut Voice);

    /// MIXDRCAuxIsPostFader
    #[doc(alias = "MIXDRCAuxIsPostFader")]
    #[link_name = "MIXDRCAuxIsPostFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_aux_is_post_fader(voice: *mut Voice) -> ffi::c_bool;

    /// MIXDRCSetAux
    #[doc(alias = "MIXDRCSetAux")]
    #[link_name = "MIXDRCSetAux"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_set_aux(voice: *mut Voice, db: ffi::c_int);

    /// MIXDRCAdjustAux
    #[doc(alias = "MIXDRCAdjustAux")]
    #[link_name = "MIXDRCAdjustAux"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_adjust_aux(voice: *mut Voice, db: ffi::c_int);

    /// MIXDRCGetAux
    #[doc(alias = "MIXDRCGetAux")]
    #[link_name = "MIXDRCGetAux"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_get_aux(voice: *mut Voice) -> ffi::c_int;

    /// MIXDRCSetPan
    #[doc(alias = "MIXDRCSetPan")]
    #[link_name = "MIXDRCSetPan"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_set_pan(voice: *mut Voice, pan: ffi::c_int);

    /// MIXDRCAdjustPan
    #[doc(alias = "MIXDRCAdjustPan")]
    #[link_name = "MIXDRCAdjustPan"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_adjust_pan(voice: *mut Voice, pan: ffi::c_int);

    /// MIXDRCGetPan
    #[doc(alias = "MIXDRCGetPan")]
    #[link_name = "MIXDRCGetPan"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_get_pan(voice: *mut Voice) -> ffi::c_int;

    /// MIXDRCSetFader
    #[doc(alias = "MIXDRCSetFader")]
    #[link_name = "MIXDRCSetFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_set_fader(voice: *mut Voice, db: ffi::c_int);

    /// MIXDRCAdjustFader
    #[doc(alias = "MIXDRCAdjustFader")]
    #[link_name = "MIXDRCAdjustFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_adjust_fader(voice: *mut Voice, db: ffi::c_int);

    /// MIXDRCGetFader
    #[doc(alias = "MIXDRCGetFader")]
    #[link_name = "MIXDRCGetFader"]
    #[deprecated(note = "Use the device functions instead")]
    pub unsafe fn drc_get_fader(voice: *mut Voice) -> ffi::c_int;

    /// MIXDRCSetSPan
    #[doc(alias = "MIXDRCSetSPan")]
    #[link_name = "MIXDRCSetSPan"]
    pub unsafe fn drc_set_span(voice: *mut Voice, span: ffi::c_int);

    /// MIXDRCAdjustSPan
    #[doc(alias = "MIXDRCAdjustSPan")]
    #[link_name = "MIXDRCAdjustSPan"]
    pub unsafe fn drc_adjust_span(voice: *mut Voice, span: ffi::c_int);

    /// MIXDRCGetSPan
    #[doc(alias = "MIXDRCGetSPan")]
    #[link_name = "MIXDRCGetSPan"]
    pub unsafe fn drc_get_span(voice: *mut Voice) -> ffi::c_int;

    /// MIXRmtSetVolumes
    ///
    /// Levels are in 0.1 dB, one per controller.
    #[doc(alias = "MIXRmtSetVolumes")]
    #[link_name = "MIXRmtSetVolumes"]
    pub unsafe fn rmt_set_volumes(
        voice: *mut Voice,
        mode: RmtMode,
        fader0: ffi::c_int,
        fader1: ffi::c_int,
        fader2: ffi::c_int,
        fader3: ffi::c_int,
        aux0: ffi::c_int,
        aux1: ffi::c_int,
        aux2: ffi::c_int,
        aux3: ffi::c_int,
    );

    /// MIXRmtAuxPostFader
    #[doc(alias = "MIXRmtAuxPostFader")]
    #[link_name = "MIXRmtAuxPostFader"]
    pub unsafe fn rmt_aux_post_fader(voice: *mut Voice, num: ffi::c_int);

    /// MIXRmtAuxPreFader
    #[doc(alias = "MIXRmtAuxPreFader")]
    #[link_name = "MIXRmtAuxPreFader"]
    pub unsafe fn rmt_aux_pre_fader(voice: *mut Voice, num: ffi::c_int);

    /// MIXRmtAuxIsPostFader
    #[doc(alias = "MIXRmtAuxIsPostFader")]
    #[link_name = "MIXRmtAuxIsPostFader"]
    pub unsafe fn rmt_aux_is_post_fader(voice: *mut Voice, num: ffi::c_int) -> ffi::c_bool;

    /// MIXRmtSetAux
    #[doc(alias = "MIXRmtSetAux")]
    #[link_name = "MIXRmtSetAux"]
    pub unsafe fn rmt_set_aux(voice: *mut Voice, num: ffi::c_int, db: ffi::c_int);

    /// MIXRmtAdjustAux
    #[doc(alias = "MIXRmtAdjustAux")]
    #[link_name = "MIXRmtAdjustAux"]
    pub unsafe fn rmt_adjust_aux(voice: *mut Voice, num: ffi::c_int, db: ffi::c_int);

    /// MIXRmtGetAux
    #[doc(alias = "MIXRmtGetAux")]
    #[link_name = "MIXRmtGetAux"]
    pub unsafe fn rmt_get_aux(voice: *mut Voice, num: ffi::c_int) -> ffi::c_int;

    /// MIXRmtSetFader
    #[doc(alias = "MIXRmtSetFader")]
    #[link_name = "MIXRmtSetFader"]
    pub unsafe fn rmt_set_fader(voice: *mut Voice, num: ffi::c_int, db: ffi::c_int);

    /// MIXRmtAdjustFader
    #[doc(alias = "MIXRmtAdjustFader")]
    #[link_name = "MIXRmtAdjustFader"]
    pub unsafe fn rmt_adjust_fader(voice: *mut Voice, num: ffi::c_int, db: ffi::c_int);

    /// MIXRmtGetFader
    #[doc(alias = "MIXRmtGetFader")]
    #[link_name = "MIXRmtGetFader"]
    pub unsafe fn rmt_get_fader(voice: *mut Voice, num: ffi::c_int) -> ffi::c_int;
}
