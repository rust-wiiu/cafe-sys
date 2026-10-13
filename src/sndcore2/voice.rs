//! AX voices

use super::device::DeviceType;
use super::{Error, SyncError};
use crate::ffi;

/// Voice priority.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Priority(pub u32);

impl Priority {
    pub const FREE: Self = Self(0);
    pub const LOWEST: Self = Self(1);
    /// Voices with this priority are never dropped.
    pub const NODROP: Self = Self(31);
}

/// AXVoiceFormat
#[doc(alias = "AXVoiceFormat")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Format(pub u16);

impl Format {
    pub const ADPCM: Self = Self(0x00);
    /// Signed 16-bit mono.
    pub const PCM16: Self = Self(0x0A);
    /// Signed 8-bit mono.
    pub const PCM8: Self = Self(0x19);
}

/// AXVoiceLoop
#[doc(alias = "AXVoiceLoop")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loop(pub u16);

impl Loop {
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
}

/// AXVoiceState
#[doc(alias = "AXVoiceState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(pub u32);

impl State {
    pub const STOPPED: Self = Self(0);
    pub const PLAYING: Self = Self(1);
}

/// AXVoiceType
#[doc(alias = "AXVoiceType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    #[default]
    Normal = 0,
    Stream = 1,
}

/// AXVoiceSrcType
#[doc(alias = "AXVoiceSrcType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SrcType {
    /// No sample rate conversion.
    #[default]
    None = 0,
    Linear = 1,
    /// 4-tap filter.
    FourTap8Khz = 2,
    /// 4-tap filter.
    FourTap12Khz = 3,
    /// 4-tap filter.
    FourTap16Khz = 4,
}

/// AXVoiceRenderer
#[doc(alias = "AXVoiceRenderer")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Renderer(pub u32);

impl Renderer {
    pub const DSP: Self = Self(0);
    pub const PPC: Self = Self(1);
    pub const DSP_OR_PPC: Self = Self(2);
}

/// AXVoiceSrcRatioResult
#[doc(alias = "AXVoiceSrcRatioResult")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SrcRatioResult(pub i32);

impl SrcRatioResult {
    pub const SUCCESS: Self = Self(0);
    pub const TOO_SMALL: Self = Self(-1);
    pub const TOO_LARGE: Self = Self(-2);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallbackReason(pub u32);

impl CallbackReason {
    /// Dropped because the DSP or PPC load limit was exceeded.
    pub const NO_RESOURCES: Self = Self(1);
    /// The sample buffer crosses a 512 MiB boundary, see [check_offsets].
    pub const SEG_VIOLATION: Self = Self(2);
}

/// AXVoiceCallbackFn
///
/// Called when the voice is dropped.
#[doc(alias = "AXVoiceCallbackFn")]
pub type Callback = Option<unsafe extern "C" fn(voice: *mut Voice)>;

/// AXVoiceCallbackExFn
///
/// Called when the voice is dropped.
#[doc(alias = "AXVoiceCallbackExFn")]
pub type CallbackEx = Option<
    unsafe extern "C" fn(voice: *mut Voice, user_context: *mut ffi::c_void, reason: CallbackReason),
>;

/// AXVoiceOffsets
///
/// Offsets are in samples (nibbles for ADPCM) from `data`.
#[doc(alias = "AXVoiceOffsets", alias = "AXPBOFFSET")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offsets {
    pub format: Format,
    pub loop_flag: Loop,
    pub loop_offset: u32,
    pub end_offset: u32,
    pub current_offset: u32,
    pub data: *const ffi::c_void,
}

/// AXVoice
#[doc(alias = "AXVoice", alias = "AXVPB")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Voice {
    pub index: u32,
    pub state: State,
    pub volume: u32,
    pub renderer: Renderer,
    pub next: *mut Voice,
    pub prev: *mut Voice,
    pub callback_next: *mut Voice,
    pub priority: Priority,
    pub callback: Callback,
    pub user_context: *mut ffi::c_void,
    pub sync: u32,
    pub depop: u32,
    pub itd_buffer: *mut ffi::c_void,
    pub offsets: Offsets,
    pub callback_ex: CallbackEx,
    pub callback_reason: CallbackReason,
    pub dsp_load_estimate: f32,
    pub ppc_load_estimate: f32,
}

/// AXVoiceVeData
///
/// Volume envelope.
#[doc(alias = "AXVoiceVeData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VeData {
    pub volume: u16,
    pub delta: i16,
}

/// AXVoiceDeviceBusMixData
#[doc(alias = "AXVoiceDeviceBusMixData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceBusMixData {
    pub volume: u16,
    pub delta: i16,
}

/// AXVoiceDeviceMixData
///
/// Mix of one channel, indexed by [Bus].
#[doc(alias = "AXVoiceDeviceMixData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceMixData {
    pub bus: [DeviceBusMixData; 4],
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bus {
    #[default]
    Main = 0,
    AuxA = 1,
    AuxB = 2,
    AuxC = 3,
}

/// AXVoiceAdpcm
#[doc(alias = "AXVoiceAdpcm")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Adpcm {
    pub coefficients: [i16; 16],
    pub gain: u16,
    pub pred_scale: u16,
    pub prev_sample: [i16; 2],
}

/// AXVoiceAdpcmLoopData
#[doc(alias = "AXVoiceAdpcmLoopData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdpcmLoopData {
    pub pred_scale: u16,
    pub prev_sample: [i16; 2],
}

/// AXVoiceSrc
#[doc(alias = "AXVoiceSrc")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Src {
    /// Integer part of the playback ratio.
    pub ratio_hi: u16,
    /// Fractional part of the playback ratio.
    pub ratio_lo: u16,
    pub current_offset_frac: u16,
    pub last_sample: [i16; 4],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtSrc {
    pub current_offset_frac: u16,
    pub last_sample: [i16; 4],
}

/// Low-pass filter.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lpf {
    /// 1 to enable.
    pub on: u16,
    pub yn1: i16,
    pub a0: u16,
    pub b0: u16,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biquad {
    /// 2 to enable.
    pub on: u16,
    pub xn1: i16,
    pub xn2: i16,
    pub yn1: i16,
    pub yn2: i16,
    pub b0: i16,
    pub b1: i16,
    pub b2: i16,
    pub a1: i16,
    pub a2: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union RmtIir {
    pub lpf: Lpf,
    pub biquad: Biquad,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mix {
    pub left: VeData,
    pub right: VeData,
    pub aux_a_left: VeData,
    pub aux_a_right: VeData,
    pub aux_b_left: VeData,
    pub aux_b_right: VeData,
    pub aux_c_left: VeData,
    pub aux_c_right: VeData,
    pub surround: VeData,
    pub aux_a_surround: VeData,
    pub aux_b_surround: VeData,
    pub aux_c_surround: VeData,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcMix {
    pub left: VeData,
    pub right: VeData,
    pub aux_left: VeData,
    pub aux_right: VeData,
    pub surround: VeData,
    pub aux_surround: VeData,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtMix {
    pub main0: VeData,
    pub aux0: VeData,
    pub main1: VeData,
    pub aux1: VeData,
    pub main2: VeData,
    pub aux2: VeData,
    pub main3: VeData,
    pub aux3: VeData,
}

const _: () = {
    assert!(size_of::<VeData>() == 0x4);
    assert!(size_of::<DeviceMixData>() == 0x10);
    assert!(size_of::<Adpcm>() == 0x28);
    assert!(size_of::<AdpcmLoopData>() == 0x6);
    assert!(size_of::<Src>() == 0xE);
    assert!(size_of::<RmtSrc>() == 0xA);
    assert!(size_of::<Lpf>() == 0x8);
    assert!(size_of::<Biquad>() == 0x14);
    assert!(size_of::<RmtIir>() == 0x14);
    assert!(size_of::<Mix>() == 0x30);
    assert!(size_of::<DrcMix>() == 0x18);
    assert!(size_of::<RmtMix>() == 0x20);
};

unsafe extern "C" {
    /// AXAcquireVoice
    ///
    /// Returns null if no voice is free and none has a lower priority than `priority`.
    #[doc(alias = "AXAcquireVoice")]
    #[link_name = "AXAcquireVoice"]
    pub unsafe fn acquire(
        priority: Priority,
        callback: Callback,
        user_context: *mut ffi::c_void,
    ) -> *mut Voice;

    /// AXAcquireVoiceEx
    #[doc(alias = "AXAcquireVoiceEx")]
    #[link_name = "AXAcquireVoiceEx"]
    pub unsafe fn acquire_ex(
        priority: Priority,
        callback: CallbackEx,
        user_context: *mut ffi::c_void,
    ) -> *mut Voice;

    /// AXFreeVoice
    #[doc(alias = "AXFreeVoice")]
    #[link_name = "AXFreeVoice"]
    pub unsafe fn free(voice: *mut Voice);

    /// AXSetVoicePriority
    #[doc(alias = "AXSetVoicePriority")]
    #[link_name = "AXSetVoicePriority"]
    pub unsafe fn set_priority(voice: *mut Voice, priority: Priority);

    /// AXVoiceBegin
    ///
    /// Starts a protection clause for `voice`, ended by [end].
    #[doc(alias = "AXVoiceBegin")]
    #[link_name = "AXVoiceBegin"]
    pub unsafe fn begin(voice: *mut Voice) -> SyncError;

    /// AXVoiceEnd
    #[doc(alias = "AXVoiceEnd")]
    #[link_name = "AXVoiceEnd"]
    pub unsafe fn end(voice: *mut Voice) -> SyncError;

    /// AXVoiceIsProtected
    #[doc(alias = "AXVoiceIsProtected")]
    #[link_name = "AXVoiceIsProtected"]
    pub unsafe fn is_protected(voice: *mut Voice) -> ffi::c_bool;

    /// AXIsVoiceRunning
    #[doc(alias = "AXIsVoiceRunning")]
    #[link_name = "AXIsVoiceRunning"]
    pub unsafe fn is_running(voice: *mut Voice) -> ffi::c_bool;

    /// AXSetVoiceState
    #[doc(alias = "AXSetVoiceState")]
    #[link_name = "AXSetVoiceState"]
    pub unsafe fn set_state(voice: *mut Voice, state: State);

    /// AXSetVoiceType
    #[doc(alias = "AXSetVoiceType")]
    #[link_name = "AXSetVoiceType"]
    pub unsafe fn set_type(voice: *mut Voice, kind: Type);

    /// AXSetVoiceMixerSelect
    ///
    /// Returns the previous renderer.
    #[doc(alias = "AXSetVoiceMixerSelect")]
    #[link_name = "AXSetVoiceMixerSelect"]
    pub unsafe fn set_renderer(voice: *mut Voice, renderer: Renderer) -> Renderer;

    /// AXGetVoiceMixerSelect
    #[doc(alias = "AXGetVoiceMixerSelect")]
    #[link_name = "AXGetVoiceMixerSelect"]
    pub unsafe fn get_renderer(voice: *mut Voice) -> Renderer;

    /// AXSetVoiceVe
    #[doc(alias = "AXSetVoiceVe")]
    #[link_name = "AXSetVoiceVe"]
    pub unsafe fn set_ve(voice: *mut Voice, ve: *const VeData);

    /// AXSetVoiceVeDelta
    #[doc(alias = "AXSetVoiceVeDelta")]
    #[link_name = "AXSetVoiceVeDelta"]
    pub unsafe fn set_ve_delta(voice: *mut Voice, delta: i16);

    /// AXSetVoiceDeviceMix
    ///
    /// `mix` points to one [DeviceMixData] per channel. `id` is 0 for the TV, 0-1 for DRCs and 0-3 for controllers.
    #[doc(alias = "AXSetVoiceDeviceMix")]
    #[link_name = "AXSetVoiceDeviceMix"]
    pub unsafe fn set_device_mix(
        voice: *mut Voice,
        device: DeviceType,
        id: u32,
        mix: *const DeviceMixData,
    ) -> Error;

    /// AXSetVoiceInitialTimeDelay
    #[doc(alias = "AXSetVoiceInitialTimeDelay")]
    #[link_name = "AXSetVoiceInitialTimeDelay"]
    pub unsafe fn set_initial_time_delay(voice: *mut Voice, delay: u16) -> Error;

    /// AXCheckVoiceOffsets
    ///
    /// Returns false if a sample buffer crosses a 512 MiB boundary. Such voices are dropped.
    #[doc(alias = "AXCheckVoiceOffsets")]
    #[link_name = "AXCheckVoiceOffsets"]
    pub unsafe fn check_offsets(offsets: *const Offsets) -> ffi::c_bool;

    /// AXSetVoiceOffsets
    #[doc(alias = "AXSetVoiceOffsets")]
    #[link_name = "AXSetVoiceOffsets"]
    pub unsafe fn set_offsets(voice: *mut Voice, offsets: *const Offsets);

    /// AXGetVoiceOffsets
    #[doc(alias = "AXGetVoiceOffsets")]
    #[link_name = "AXGetVoiceOffsets"]
    pub unsafe fn get_offsets(voice: *mut Voice, offsets: *mut Offsets);

    /// AXSetVoiceSamplesAddr
    #[doc(alias = "AXSetVoiceSamplesAddr")]
    #[link_name = "AXSetVoiceSamplesAddr"]
    pub unsafe fn set_samples_addr(voice: *mut Voice, samples: *const ffi::c_void);

    /// AXSetVoiceLoop
    #[doc(alias = "AXSetVoiceLoop")]
    #[link_name = "AXSetVoiceLoop"]
    pub unsafe fn set_loop(voice: *mut Voice, flag: Loop);

    /// AXSetVoiceLoopOffset
    #[doc(alias = "AXSetVoiceLoopOffset")]
    #[link_name = "AXSetVoiceLoopOffset"]
    pub unsafe fn set_loop_offset(voice: *mut Voice, offset: u32);

    /// AXSetVoiceEndOffset
    #[doc(alias = "AXSetVoiceEndOffset")]
    #[link_name = "AXSetVoiceEndOffset"]
    pub unsafe fn set_end_offset(voice: *mut Voice, offset: u32);

    /// AXSetVoiceCurrentOffset
    #[doc(alias = "AXSetVoiceCurrentOffset")]
    #[link_name = "AXSetVoiceCurrentOffset"]
    pub unsafe fn set_current_offset(voice: *mut Voice, offset: u32);

    /// AXGetVoiceLoopCount
    ///
    /// Number of loops since the voice was acquired.
    #[doc(alias = "AXGetVoiceLoopCount")]
    #[link_name = "AXGetVoiceLoopCount"]
    pub unsafe fn get_loop_count(voice: *mut Voice) -> u32;

    /// AXGetVoiceOffsetsEx
    #[doc(alias = "AXGetVoiceOffsetsEx")]
    #[link_name = "AXGetVoiceOffsetsEx"]
    #[deprecated(note = "Use get_offsets instead")]
    pub unsafe fn get_offsets_ex(
        voice: *mut Voice,
        offsets: *mut Offsets,
        samples: *const ffi::c_void,
    );

    /// AXSetVoiceOffsetsEx
    #[doc(alias = "AXSetVoiceOffsetsEx")]
    #[link_name = "AXSetVoiceOffsetsEx"]
    #[deprecated(note = "Use set_offsets instead")]
    pub unsafe fn set_offsets_ex(
        voice: *mut Voice,
        offsets: *const Offsets,
        samples: *const ffi::c_void,
    );

    /// AXSetVoiceLoopOffsetEx
    #[doc(alias = "AXSetVoiceLoopOffsetEx")]
    #[link_name = "AXSetVoiceLoopOffsetEx"]
    #[deprecated(note = "Use set_loop_offset instead")]
    pub unsafe fn set_loop_offset_ex(voice: *mut Voice, offset: u32, samples: *const ffi::c_void);

    /// AXSetVoiceEndOffsetEx
    #[doc(alias = "AXSetVoiceEndOffsetEx")]
    #[link_name = "AXSetVoiceEndOffsetEx"]
    #[deprecated(note = "Use set_end_offset instead")]
    pub unsafe fn set_end_offset_ex(voice: *mut Voice, offset: u32, samples: *const ffi::c_void);

    /// AXSetVoiceCurrentOffsetEx
    #[doc(alias = "AXSetVoiceCurrentOffsetEx")]
    #[link_name = "AXSetVoiceCurrentOffsetEx"]
    #[deprecated(note = "Use set_current_offset instead")]
    pub unsafe fn set_current_offset_ex(
        voice: *mut Voice,
        offset: u32,
        samples: *const ffi::c_void,
    );

    /// AXGetVoiceCurrentOffsetEx
    #[doc(alias = "AXGetVoiceCurrentOffsetEx")]
    #[link_name = "AXGetVoiceCurrentOffsetEx"]
    #[deprecated(note = "Use get_offsets instead")]
    pub unsafe fn get_current_offset_ex(voice: *mut Voice, samples: *const ffi::c_void) -> u32;

    /// AXSetVoiceAdpcm
    #[doc(alias = "AXSetVoiceAdpcm")]
    #[link_name = "AXSetVoiceAdpcm"]
    pub unsafe fn set_adpcm(voice: *mut Voice, adpcm: *const Adpcm);

    /// AXSetVoiceAdpcmLoop
    #[doc(alias = "AXSetVoiceAdpcmLoop")]
    #[link_name = "AXSetVoiceAdpcmLoop"]
    pub unsafe fn set_adpcm_loop(voice: *mut Voice, loop_data: *const AdpcmLoopData);

    /// AXSetVoiceSrc
    #[doc(alias = "AXSetVoiceSrc")]
    #[link_name = "AXSetVoiceSrc"]
    pub unsafe fn set_src(voice: *mut Voice, src: *const Src);

    /// AXSetVoiceSrcType
    #[doc(alias = "AXSetVoiceSrcType")]
    #[link_name = "AXSetVoiceSrcType"]
    pub unsafe fn set_src_type(voice: *mut Voice, kind: SrcType);

    /// AXSetVoiceSrcRatio
    #[doc(alias = "AXSetVoiceSrcRatio")]
    #[link_name = "AXSetVoiceSrcRatio"]
    pub unsafe fn set_src_ratio(voice: *mut Voice, ratio: f32) -> SrcRatioResult;

    /// AXSetVoiceLpf
    #[doc(alias = "AXSetVoiceLpf")]
    #[link_name = "AXSetVoiceLpf"]
    pub unsafe fn set_lpf(voice: *mut Voice, lpf: *const Lpf);

    /// AXSetVoiceLpfCoefs
    ///
    /// See [compute_lpf_coefs][super::compute_lpf_coefs].
    #[doc(alias = "AXSetVoiceLpfCoefs")]
    #[link_name = "AXSetVoiceLpfCoefs"]
    pub unsafe fn set_lpf_coefs(voice: *mut Voice, a0: u16, b0: u16);

    /// AXSetVoiceBiquad
    #[doc(alias = "AXSetVoiceBiquad")]
    #[link_name = "AXSetVoiceBiquad"]
    pub unsafe fn set_biquad(voice: *mut Voice, biquad: *const Biquad);

    /// AXSetVoiceBiquadCoefs
    #[doc(alias = "AXSetVoiceBiquadCoefs")]
    #[link_name = "AXSetVoiceBiquadCoefs"]
    pub unsafe fn set_biquad_coefs(voice: *mut Voice, b0: u16, b1: u16, b2: u16, a1: u16, a2: u16);

    /// AXSetVoiceRmtOn
    #[doc(alias = "AXSetVoiceRmtOn")]
    #[link_name = "AXSetVoiceRmtOn"]
    pub unsafe fn set_rmt_on(voice: *mut Voice, on: u16);

    /// AXSetVoiceRmtSrc
    #[doc(alias = "AXSetVoiceRmtSrc")]
    #[link_name = "AXSetVoiceRmtSrc"]
    pub unsafe fn set_rmt_src(voice: *mut Voice, src: *const RmtSrc);

    /// AXSetVoiceRmtIIR
    #[doc(alias = "AXSetVoiceRmtIIR")]
    #[link_name = "AXSetVoiceRmtIIR"]
    pub unsafe fn set_rmt_iir(voice: *mut Voice, iir: *const RmtIir);

    /// AXSetVoiceRmtIIRCoefs
    ///
    /// `kind` must match the filter set with [set_rmt_iir]. For the low-pass filter (1) pass `a0, b0`, for the biquad filter (2) pass `b0, b1, b2, a1, a2`, each as [ffi::c_uint].
    #[doc(alias = "AXSetVoiceRmtIIRCoefs")]
    #[link_name = "AXSetVoiceRmtIIRCoefs"]
    pub unsafe fn set_rmt_iir_coefs(voice: *mut Voice, kind: u16, ...);

    /// AXSetVoiceMix
    #[doc(alias = "AXSetVoiceMix")]
    #[link_name = "AXSetVoiceMix"]
    #[deprecated(note = "Use set_device_mix instead")]
    pub unsafe fn set_mix(voice: *mut Voice, mix: *const Mix);

    /// AXSetVoiceDRCMix
    #[doc(alias = "AXSetVoiceDRCMix")]
    #[link_name = "AXSetVoiceDRCMix"]
    #[deprecated(note = "Use set_device_mix instead")]
    pub unsafe fn set_drc_mix(voice: *mut Voice, mix: *const DrcMix);

    /// AXSetVoiceRmtMix
    #[doc(alias = "AXSetVoiceRmtMix")]
    #[link_name = "AXSetVoiceRmtMix"]
    #[deprecated(note = "Use set_device_mix instead")]
    pub unsafe fn set_rmt_mix(voice: *mut Voice, mix: *const RmtMix);

    /// AXSetVoiceItdOn
    #[doc(alias = "AXSetVoiceItdOn")]
    #[link_name = "AXSetVoiceItdOn"]
    #[deprecated]
    pub unsafe fn set_itd_on(voice: *mut Voice);

    /// AXSetVoiceItdTarget
    #[doc(alias = "AXSetVoiceItdTarget")]
    #[link_name = "AXSetVoiceItdTarget"]
    #[deprecated]
    pub unsafe fn set_itd_target(voice: *mut Voice, left_shift: u16, right_shift: u16);
}
