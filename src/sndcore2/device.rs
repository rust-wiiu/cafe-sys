//! AX output devices

use super::Error;
use crate::ffi;

/// AXDeviceType
#[doc(alias = "AXDeviceType", alias = "AXPBDeviceType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    #[default]
    Tv = 0,
    Drc = 1,
    /// Wii Remote speaker.
    Controller = 2,
}

/// AXDeviceMode
#[doc(alias = "AXDeviceMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceMode(pub u32);

impl DeviceMode {
    pub const STEREO: Self = Self(0);
    pub const SURROUND: Self = Self(1);
    pub const DPL2: Self = Self(2);
    pub const SIX_CHANNEL: Self = Self(3);
    pub const MONO: Self = Self(5);
}

/// AXPB_AUX_ID
#[doc(alias = "AXPB_AUX_ID")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxId {
    #[default]
    A = 0,
    B = 1,
    C = 2,
}

/// AXAUXCBSTRUCT
#[doc(alias = "AXAUXCBSTRUCT")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxCallbackInfo {
    pub num_channels: u32,
    /// Samples per channel.
    pub num_samples: u32,
}

/// AXAuxCallback
///
/// `data` points to one sample buffer per channel.
#[doc(alias = "AXAuxCallback", alias = "AXUserAuxCallback")]
pub type AuxCallback = Option<
    unsafe extern "C" fn(
        data: *mut *mut i32,
        context: *mut ffi::c_void,
        info: *mut AuxCallbackInfo,
    ),
>;

/// AX_FINAL_MIX_CB_STRUCT
#[doc(alias = "AX_FINAL_MIX_CB_STRUCT")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FinalMixCallbackInfo {
    /// `num_channels_in * num_devices` sample buffers, grouped by channel.
    pub data: *mut *mut i32,
    pub num_channels_in: u16,
    pub num_samples: u16,
    pub num_devices: u16,
    /// Set to the number of channels to output, initially `num_channels_in`.
    pub num_channels_out: u16,
}

/// AXDeviceFinalMixCallback
#[doc(alias = "AXDeviceFinalMixCallback", alias = "AXUserFinalMixCallback")]
pub type FinalMixCallback = Option<unsafe extern "C" fn(info: *mut FinalMixCallbackInfo)>;

/// AXDEVICEFINALOUTPUTSTRUCT
#[doc(alias = "AXDEVICEFINALOUTPUTSTRUCT")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FinalOutputInfo {
    pub num_channels: u32,
    pub sample_freq: u32,
    pub num_devices: u32,
    pub format: u32,
    pub length: u32,
}

const _: () = {
    assert!(size_of::<AuxCallbackInfo>() == 0x8);
    assert!(size_of::<FinalOutputInfo>() == 0x14);
};

unsafe extern "C" {
    /// AXGetDeviceMode
    #[doc(alias = "AXGetDeviceMode")]
    #[link_name = "AXGetDeviceMode"]
    pub unsafe fn get_mode(device: DeviceType, mode: *mut DeviceMode) -> Error;

    /// AXSetDeviceMode
    ///
    /// Always fails, the mode is set in the system settings.
    #[doc(alias = "AXSetDeviceMode")]
    #[link_name = "AXSetDeviceMode"]
    #[deprecated]
    pub unsafe fn set_mode(device: DeviceType, mode: DeviceMode) -> Error;

    /// AXUpdateDeviceModes
    ///
    /// Updates the TV and DRC modes.
    #[doc(alias = "AXUpdateDeviceModes")]
    #[link_name = "AXUpdateDeviceModes"]
    pub unsafe fn update_modes();

    /// AXGetDeviceChannelCount
    ///
    /// `id` is 0 for the TV, 0-1 for DRCs and 0-3 for controllers.
    #[doc(alias = "AXGetDeviceChannelCount")]
    #[link_name = "AXGetDeviceChannelCount"]
    pub unsafe fn get_channel_count(device: DeviceType, id: u32, count: *mut u32) -> Error;

    /// AXGetDeviceVolume
    #[doc(alias = "AXGetDeviceVolume")]
    #[link_name = "AXGetDeviceVolume"]
    pub unsafe fn get_volume(device: DeviceType, id: u32, volume: *mut u16) -> Error;

    /// AXSetDeviceVolume
    #[doc(alias = "AXSetDeviceVolume")]
    #[link_name = "AXSetDeviceVolume"]
    pub unsafe fn set_volume(device: DeviceType, id: u32, volume: u16) -> Error;

    /// AXRegisterAuxCallback
    ///
    /// Called once each audio frame.
    #[doc(alias = "AXRegisterAuxCallback")]
    #[link_name = "AXRegisterAuxCallback"]
    pub unsafe fn register_aux_callback(
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        callback: AuxCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// AXGetAuxCallback
    #[doc(alias = "AXGetAuxCallback")]
    #[link_name = "AXGetAuxCallback"]
    pub unsafe fn get_aux_callback(
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        callback: *mut AuxCallback,
        context: *mut *mut ffi::c_void,
    ) -> Error;

    /// AXGetAuxReturnVolume
    #[doc(alias = "AXGetAuxReturnVolume")]
    #[link_name = "AXGetAuxReturnVolume"]
    pub unsafe fn get_aux_return_volume(
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        volume: *mut u16,
    ) -> Error;

    /// AXSetAuxReturnVolume
    #[doc(alias = "AXSetAuxReturnVolume")]
    #[link_name = "AXSetAuxReturnVolume"]
    pub unsafe fn set_aux_return_volume(
        device: DeviceType,
        id: u32,
        aux_id: AuxId,
        volume: u16,
    ) -> Error;

    /// AXRegisterDeviceFinalMixCallback
    ///
    /// Called each audio frame after the main and aux buses are mixed.
    #[doc(alias = "AXRegisterDeviceFinalMixCallback")]
    #[link_name = "AXRegisterDeviceFinalMixCallback"]
    pub unsafe fn register_final_mix_callback(
        device: DeviceType,
        callback: FinalMixCallback,
    ) -> Error;

    /// AXGetDeviceFinalMixCallback
    #[doc(alias = "AXGetDeviceFinalMixCallback")]
    #[link_name = "AXGetDeviceFinalMixCallback"]
    pub unsafe fn get_final_mix_callback(
        device: DeviceType,
        callback: *mut FinalMixCallback,
    ) -> Error;

    /// AXGetDeviceFinalOutput
    ///
    /// Copies the final output samples of the TV or DRC into `data`, interleaved. `size` is in bytes and must be at least 1728 for the TV and 576 for the DRC.
    #[doc(alias = "AXGetDeviceFinalOutput")]
    #[link_name = "AXGetDeviceFinalOutput"]
    pub unsafe fn get_final_output(
        device: DeviceType,
        data: *mut i16,
        size: u32,
        info: *mut FinalOutputInfo,
    ) -> Error;

    /// AXSetDeviceCompressor
    #[doc(alias = "AXSetDeviceCompressor")]
    #[link_name = "AXSetDeviceCompressor"]
    pub unsafe fn set_compressor(device: DeviceType, on: u16) -> Error;

    /// AXSetDeviceCompressorTable
    #[doc(alias = "AXSetDeviceCompressorTable")]
    #[link_name = "AXSetDeviceCompressorTable"]
    pub unsafe fn set_compressor_table(device: DeviceType, table: *mut u16, frames: u16) -> Error;

    /// AXMakeCompressorTable
    ///
    /// Creates a table for [set_compressor_table].
    #[doc(alias = "AXMakeCompressorTable")]
    #[link_name = "AXMakeCompressorTable"]
    pub unsafe fn make_compressor_table(gain: f32, frames: u16, table: *mut u16);

    /// AXSetDeviceRemixMatrix
    ///
    /// Up- or downmixes the final output with a `num_in_channels` by `num_out_channels` matrix.
    #[doc(alias = "AXSetDeviceRemixMatrix")]
    #[link_name = "AXSetDeviceRemixMatrix"]
    pub unsafe fn set_remix_matrix(
        device: DeviceType,
        num_in_channels: u32,
        num_out_channels: u32,
        matrix: *const f32,
    ) -> Error;

    /// AXGetDeviceRemixMatrix
    #[doc(alias = "AXGetDeviceRemixMatrix")]
    #[link_name = "AXGetDeviceRemixMatrix"]
    pub unsafe fn get_remix_matrix(
        device: DeviceType,
        num_in_channels: u32,
        num_out_channels: u32,
        matrix: *mut *const f32,
    ) -> Error;

    /// AXGetDeviceUpsampleStage
    #[doc(alias = "AXGetDeviceUpsampleStage")]
    #[link_name = "AXGetDeviceUpsampleStage"]
    pub unsafe fn get_upsample_stage(device: DeviceType, post: *mut ffi::c_bool) -> Error;

    /// AXSetDeviceUpsampleStage
    ///
    /// The final mix callback receives 32 kHz samples if `post` is true, otherwise 48 kHz.
    #[doc(alias = "AXSetDeviceUpsampleStage")]
    #[link_name = "AXSetDeviceUpsampleStage"]
    pub unsafe fn set_upsample_stage(device: DeviceType, post: ffi::c_bool) -> Error;

    /// AXSetDeviceLinearUpsampler
    ///
    /// Uses linear interpolation instead of a polyphase filter for upsampling.
    #[doc(alias = "AXSetDeviceLinearUpsampler")]
    #[link_name = "AXSetDeviceLinearUpsampler"]
    pub unsafe fn set_linear_upsampler(device: DeviceType, id: u32, linear: ffi::c_bool) -> Error;
}
