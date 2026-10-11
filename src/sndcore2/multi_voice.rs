//! AX multi voices
//!
//! Controls several voices at once.

use super::device::DeviceType;
use super::voice::{
    AdpcmLoopData, Biquad, Bus, CallbackEx, Lpf, Offsets, Renderer, Src, SrcRatioResult, SrcType,
    State, Type, VeData, Voice,
};
use super::{DspAdpcm, Error};
use crate::ffi;

pub const MAX_VOICES: usize = 6;

/// AXMVPB
#[doc(alias = "AXMVPB")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiVoice {
    pub active: ffi::c_bool,
    /// 1-6.
    pub count: u32,
    pub voices: [*mut Voice; MAX_VOICES],
}

unsafe extern "C" {
    /// AXAcquireMultiVoice
    ///
    /// Acquires one voice per channel described by `adpcm`.
    #[doc(alias = "AXAcquireMultiVoice")]
    #[link_name = "AXAcquireMultiVoice"]
    pub unsafe fn acquire(
        priority: u32,
        callback: CallbackEx,
        user_context: *mut ffi::c_void,
        adpcm: *mut DspAdpcm,
        multi_voice: *mut *mut MultiVoice,
    ) -> Error;

    /// AXFreeMultiVoice
    #[doc(alias = "AXFreeMultiVoice")]
    #[link_name = "AXFreeMultiVoice"]
    pub unsafe fn free(multi_voice: *mut MultiVoice);

    /// AXIsMultiVoiceRunning
    #[doc(alias = "AXIsMultiVoiceRunning")]
    #[link_name = "AXIsMultiVoiceRunning"]
    pub unsafe fn is_running(multi_voice: *mut MultiVoice) -> ffi::c_bool;

    /// AXGetMultiVoiceReformatBufferSize
    #[doc(alias = "AXGetMultiVoiceReformatBufferSize")]
    #[link_name = "AXGetMultiVoiceReformatBufferSize"]
    pub unsafe fn get_reformat_buffer_size(
        format: u32,
        num_channels: u32,
        num_bytes: u32,
        size: *mut u32,
    ) -> Error;

    /// AXSetMultiVoicePriority
    #[doc(alias = "AXSetMultiVoicePriority")]
    #[link_name = "AXSetMultiVoicePriority"]
    pub unsafe fn set_priority(multi_voice: *mut MultiVoice, priority: u32);

    /// AXSetMultiVoiceState
    #[doc(alias = "AXSetMultiVoiceState")]
    #[link_name = "AXSetMultiVoiceState"]
    pub unsafe fn set_state(multi_voice: *mut MultiVoice, state: State);

    /// AXSetMultiVoiceType
    #[doc(alias = "AXSetMultiVoiceType")]
    #[link_name = "AXSetMultiVoiceType"]
    pub unsafe fn set_type(multi_voice: *mut MultiVoice, kind: Type);

    /// AXSetMultiVoiceRenderer
    ///
    /// Returns the previous renderer.
    #[doc(alias = "AXSetMultiVoiceRenderer")]
    #[link_name = "AXSetMultiVoiceRenderer"]
    pub unsafe fn set_renderer(multi_voice: *mut MultiVoice, renderer: Renderer) -> Renderer;

    /// AXGetMultiVoiceRenderer
    #[doc(alias = "AXGetMultiVoiceRenderer")]
    #[link_name = "AXGetMultiVoiceRenderer"]
    pub unsafe fn get_renderer(multi_voice: *mut MultiVoice) -> Renderer;

    /// AXSetMultiVoiceVe
    #[doc(alias = "AXSetMultiVoiceVe")]
    #[link_name = "AXSetMultiVoiceVe"]
    pub unsafe fn set_ve(multi_voice: *mut MultiVoice, ve: *const VeData);

    /// AXSetMultiVoiceVeDelta
    #[doc(alias = "AXSetMultiVoiceVeDelta")]
    #[link_name = "AXSetMultiVoiceVeDelta"]
    pub unsafe fn set_ve_delta(multi_voice: *mut MultiVoice, delta: i16);

    /// AXSetMultiVoiceDeviceMix
    #[doc(alias = "AXSetMultiVoiceDeviceMix")]
    #[link_name = "AXSetMultiVoiceDeviceMix"]
    pub unsafe fn set_device_mix(
        multi_voice: *mut MultiVoice,
        device: DeviceType,
        id: u32,
        bus: Bus,
        volume: u16,
        delta: i16,
    ) -> Error;

    /// AXSetMultiVoiceOffsets
    #[doc(alias = "AXSetMultiVoiceOffsets")]
    #[link_name = "AXSetMultiVoiceOffsets"]
    pub unsafe fn set_offsets(multi_voice: *mut MultiVoice, offsets: *mut Offsets);

    /// AXGetMultiVoiceOffsets
    #[doc(alias = "AXGetMultiVoiceOffsets")]
    #[link_name = "AXGetMultiVoiceOffsets"]
    pub unsafe fn get_offsets(multi_voice: *mut MultiVoice, offsets: *mut Offsets);

    /// AXSetMultiVoiceLoop
    #[doc(alias = "AXSetMultiVoiceLoop")]
    #[link_name = "AXSetMultiVoiceLoop"]
    pub unsafe fn set_loop(multi_voice: *mut MultiVoice, flag: super::voice::Loop);

    /// AXSetMultiVoiceLoopOffset
    #[doc(alias = "AXSetMultiVoiceLoopOffset")]
    #[link_name = "AXSetMultiVoiceLoopOffset"]
    pub unsafe fn set_loop_offset(multi_voice: *mut MultiVoice, offset: u32);

    /// AXSetMultiVoiceEndOffset
    #[doc(alias = "AXSetMultiVoiceEndOffset")]
    #[link_name = "AXSetMultiVoiceEndOffset"]
    pub unsafe fn set_end_offset(multi_voice: *mut MultiVoice, offset: u32);

    /// AXSetMultiVoiceCurrentOffset
    #[doc(alias = "AXSetMultiVoiceCurrentOffset")]
    #[link_name = "AXSetMultiVoiceCurrentOffset"]
    pub unsafe fn set_current_offset(multi_voice: *mut MultiVoice, offset: u32);

    /// AXSetMultiVoiceAdpcm
    #[doc(alias = "AXSetMultiVoiceAdpcm")]
    #[link_name = "AXSetMultiVoiceAdpcm"]
    pub unsafe fn set_adpcm(multi_voice: *mut MultiVoice, adpcm: *mut DspAdpcm);

    /// AXSetMultiVoiceAdpcmLoop
    #[doc(alias = "AXSetMultiVoiceAdpcmLoop")]
    #[link_name = "AXSetMultiVoiceAdpcmLoop"]
    pub unsafe fn set_adpcm_loop(multi_voice: *mut MultiVoice, loop_data: *mut AdpcmLoopData);

    /// AXSetMultiVoiceSrc
    #[doc(alias = "AXSetMultiVoiceSrc")]
    #[link_name = "AXSetMultiVoiceSrc"]
    pub unsafe fn set_src(multi_voice: *mut MultiVoice, src: *const Src);

    /// AXSetMultiVoiceSrcType
    #[doc(alias = "AXSetMultiVoiceSrcType")]
    #[link_name = "AXSetMultiVoiceSrcType"]
    pub unsafe fn set_src_type(multi_voice: *mut MultiVoice, kind: SrcType);

    /// AXSetMultiVoiceSrcRatio
    #[doc(alias = "AXSetMultiVoiceSrcRatio")]
    #[link_name = "AXSetMultiVoiceSrcRatio"]
    pub unsafe fn set_src_ratio(multi_voice: *mut MultiVoice, ratio: f32) -> SrcRatioResult;

    /// AXSetMultiVoiceLpf
    #[doc(alias = "AXSetMultiVoiceLpf")]
    #[link_name = "AXSetMultiVoiceLpf"]
    pub unsafe fn set_lpf(multi_voice: *mut MultiVoice, lpf: *const Lpf);

    /// AXSetMultiVoiceLpfCoefs
    #[doc(alias = "AXSetMultiVoiceLpfCoefs")]
    #[link_name = "AXSetMultiVoiceLpfCoefs"]
    pub unsafe fn set_lpf_coefs(multi_voice: *mut MultiVoice, a0: u16, b0: u16);

    /// AXSetMultiVoiceBiquad
    #[doc(alias = "AXSetMultiVoiceBiquad")]
    #[link_name = "AXSetMultiVoiceBiquad"]
    pub unsafe fn set_biquad(multi_voice: *mut MultiVoice, biquad: *const Biquad);

    /// AXSetMultiVoiceBiquadCoefs
    #[doc(alias = "AXSetMultiVoiceBiquadCoefs")]
    #[link_name = "AXSetMultiVoiceBiquadCoefs"]
    pub unsafe fn set_biquad_coefs(
        multi_voice: *mut MultiVoice,
        b0: u16,
        b1: u16,
        b2: u16,
        a1: u16,
        a2: u16,
    );
}
