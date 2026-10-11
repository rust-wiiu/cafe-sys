//! sndcore2

use crate::coreinit::time::Time;
use crate::ffi;

pub mod ai;
pub mod device;
pub mod drcvs;
pub mod multi_voice;
pub mod rmt;
pub mod voice;

/// AX_MAX_VOICES
#[doc(alias = "AX_MAX_VOICES")]
pub const MAX_VOICES: u32 = 96;
/// AX_MAX_APP_FRAME_CBS
#[doc(alias = "AX_MAX_APP_FRAME_CBS")]
pub const MAX_APP_FRAME_CALLBACKS: u32 = 64;
/// AX_MS_PER_FRAME
#[doc(alias = "AX_MS_PER_FRAME")]
pub const MS_PER_FRAME: u32 = 3;

/// AXResult
#[doc(alias = "AXResult", alias = "AXPB_ERROR_CODE")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_DEVICE_TYPE: Self = Self(-1);
    pub const INVALID_DEVICE_ID: Self = Self(-2);
    pub const NULL_MIX_PTR: Self = Self(-3);
    pub const NULL_VOICE_PTR: Self = Self(-4);
    pub const INVALID_AUX_ID: Self = Self(-5);
    pub const ILLEGAL_COMPRESSOR_SETTINGS: Self = Self(-6);
    pub const REMIX_ILLEGAL_IN_CHANNELS: Self = Self(-7);
    pub const REMIX_ILLEGAL_OUT_CHANNELS: Self = Self(-8);
    pub const REMIX_EXCEED_MAX_ALLOWED: Self = Self(-9);
    pub const REMIX_MATRIX_UNINITIALIZED: Self = Self(-10);
    pub const FINAL_OUTPUT_SIZE: Self = Self(-11);
    pub const DRC_VS_CONFIG: Self = Self(-12);
    pub const INVALID_DRC_VS_MODE: Self = Self(-13);
    pub const FRAME_CALLBACK_TRY_AGAIN: Self = Self(-14);
    pub const FRAME_CALLBACK_MAX_DEPTH_REACHED: Self = Self(-15);
    pub const FRAME_CALLBACK_NOT_FOUND: Self = Self(-16);
    pub const FRAME_CALLBACK_NULL_FUNC: Self = Self(-17);
    pub const VOICE_IS_RUNNING: Self = Self(-18);
    pub const DELAY_TOO_BIG: Self = Self(-19);
    pub const MULTI_VOICE_COUNT_EXCEEDED: Self = Self(-20);
    pub const MULTI_VOICE_INVALID_CHANNEL_COUNT: Self = Self(-21);
    pub const MULTI_VOICE_UNABLE_TO_ALLOCATE_ALL_VOICES: Self = Self(-22);
    pub const MULTI_VOICE_INVALID_FORMAT: Self = Self(-23);
}

/// AXINIT_ERROR_CODE
#[doc(alias = "AXINIT_ERROR_CODE")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitError(pub i32);

impl InitError {
    pub const SUCCESS: Self = Self(0);
    pub const ALREADY_INITIALIZED: Self = Self(-1);
    pub const INVALID_RENDERER: Self = Self(-2);
    pub const INVALID_FRAME_SIZE: Self = Self(-3);
    pub const INVALID_PIPELINE_STAGE: Self = Self(-4);
}

/// AXSYNC_ERROR
///
/// Non-negative values are success.
#[doc(alias = "AXSYNC_ERROR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyncError(pub i32);

impl SyncError {
    pub const SUCCESS: Self = Self(0);
    pub const IN_USE: Self = Self(-1);
    /// Called from an AX callback, or from another core.
    pub const IN_CALLBACK: Self = Self(-2);
    pub const NOT_PROTECTED: Self = Self(-3);
    pub const TOO_MANY_PROTECTS: Self = Self(-4);
}

/// AXInitRenderer
#[doc(alias = "AXInitRenderer", alias = "AX_RENDERER_FREQ_ENUM")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitRenderer(pub u32);

impl InitRenderer {
    pub const KHZ_32: Self = Self(0);
    pub const KHZ_48: Self = Self(1);
}

/// AXInitPipeline
#[doc(alias = "AXInitPipeline", alias = "AX_PIPELINE_STAGES_ENUM")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitPipeline(pub u32);

impl InitPipeline {
    pub const SINGLE: Self = Self(0);
    pub const FOUR_STAGE: Self = Self(1);
}

/// AXInitParams
#[doc(alias = "AXInitParams", alias = "AXINITPARAMS")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitParams {
    pub renderer: InitRenderer,
    /// Must be 0 (3 ms).
    pub frame_size: u32,
    pub pipeline: InitPipeline,
}

/// AXOutputBufferMode
#[doc(alias = "AX_OUTPUT_BUFFER")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputBufferMode {
    #[default]
    Double = 0,
    Triple = 1,
}

/// AXProfile
#[doc(alias = "AXProfile", alias = "AXPROFILE")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Profile {
    pub ax_frame_start: Time,
    pub ax_frame_end: Time,
    pub user_aux_proc_start: Time,
    pub user_aux_proc_end: Time,
    pub user_callback_start: Time,
    pub user_callback_end: Time,
    pub user_final_mix_start: Time,
    pub user_final_mix_end: Time,
    pub ax_num_voices: u32,
    pub ax_num_dsp_voices: u32,
    pub ax_dsp_start: Time,
    pub ax_dsp_end: Time,
    pub ax_ppc_start: Time,
    pub ax_ppc_end: Time,
    pub ax_ppc_post_start: Time,
    pub ax_ppc_post_end: Time,
    pub ist_latency: Time,
    pub ist_post_latency: Time,
}

/// DSPADPCM
///
/// Header of DSP-ADPCM sample data.
#[doc(alias = "DSPADPCM")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DspAdpcm {
    pub num_samples: u32,
    /// Including frame headers.
    pub num_adpcm_nibbles: u32,
    pub sample_rate: u32,
    pub loop_flag: voice::Loop,
    pub format: voice::Format,
    pub loop_start: u32,
    pub loop_end: u32,
    pub current_address: u32,
    pub coef: [i16; 16],
    pub gain: u16,
    pub pred_scale: u16,
    pub yn1: i16,
    pub yn2: i16,
    pub loop_pred_scale: u16,
    pub loop_yn1: i16,
    pub loop_yn2: i16,
    /// Number of interleaved channels (1-6), or 0 for a single channel.
    pub multi_ch_count: u16,
    /// ADPCM frames (8 bytes each) per interleaved channel block.
    pub block_frame_count: u16,
    _pad: ffi::unknown<18>,
}

/// AXFrameCallback
#[doc(alias = "AXFrameCallback", alias = "AXUserCallback")]
pub type FrameCallback = Option<unsafe extern "C" fn()>;

/// AXExceedCallback
///
/// Called when the DSP did not finish within the audio frame.
#[doc(alias = "AXExceedCallback")]
pub type ExceedCallback = Option<unsafe extern "C" fn(cycles: u32)>;

/// AXAuxCallback
///
/// Callback of the deprecated per-bus aux functions.
pub type LegacyAuxCallback =
    Option<unsafe extern "C" fn(data: *mut ffi::c_void, context: *mut ffi::c_void)>;

/// AXTransitionAudioBufferDevice
#[doc(alias = "AXTransitionAudioBufferDevice")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransitionAudioBufferDevice {
    pub mode: device::DeviceMode,
    _unk1: ffi::unknown<4>,
    _unk2: ffi::unknown<4>,
    pub enabled: ffi::c_bool,
}

/// AXTransitionAudioBuffer
#[doc(alias = "AXTransitionAudioBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransitionAudioBuffer {
    _unk1: ffi::unknown<4>,
    _unk2: ffi::unknown<4>,
    pub length: u32,
    pub loop_point: u32,
    pub audio_buffer: *mut ffi::c_void,
    pub audio_buffer_len: u32,
    pub tv: TransitionAudioBufferDevice,
    pub drc: TransitionAudioBufferDevice,
}

const _: () = {
    assert!(size_of::<InitParams>() == 0xC);
    assert!(size_of::<Profile>() == 0x88);
    assert!(size_of::<DspAdpcm>() == 0x60);
    assert!(size_of::<TransitionAudioBufferDevice>() == 0x10);
};

unsafe extern "C" {
    /// AXInit
    ///
    /// Same as [init_with_params] with a 32 kHz renderer and a single pipeline stage.
    #[doc(alias = "AXInit")]
    #[link_name = "AXInit"]
    #[deprecated(note = "Use init_with_params instead")]
    pub unsafe fn init();

    /// AXInitEx
    #[doc(alias = "AXInitEx")]
    #[link_name = "AXInitEx"]
    #[deprecated(note = "Use init_with_params instead")]
    pub unsafe fn init_ex(output_buffer_mode: OutputBufferMode);

    /// AXInitWithParams
    #[doc(alias = "AXInitWithParams")]
    #[link_name = "AXInitWithParams"]
    pub unsafe fn init_with_params(params: *const InitParams) -> InitError;

    /// AXIsInit
    #[doc(alias = "AXIsInit")]
    #[link_name = "AXIsInit"]
    pub unsafe fn is_init() -> ffi::c_bool;

    /// AXQuit
    #[doc(alias = "AXQuit")]
    #[link_name = "AXQuit"]
    pub unsafe fn quit();

    /// AXGetCurrentParams
    #[doc(alias = "AXGetCurrentParams")]
    #[link_name = "AXGetCurrentParams"]
    pub unsafe fn get_current_params(params: *mut InitParams);

    /// AXGetRendererFreq
    #[doc(alias = "AXGetRendererFreq")]
    #[link_name = "AXGetRendererFreq"]
    pub unsafe fn get_renderer_freq() -> InitRenderer;

    /// AXGetInputSamplesPerFrame
    #[doc(alias = "AXGetInputSamplesPerFrame")]
    #[link_name = "AXGetInputSamplesPerFrame"]
    pub unsafe fn get_input_samples_per_frame() -> u32;

    /// AXGetInputSamplesPerSec
    #[doc(alias = "AXGetInputSamplesPerSec")]
    #[link_name = "AXGetInputSamplesPerSec"]
    pub unsafe fn get_input_samples_per_sec() -> u32;

    /// AXGetLibraryVersion
    #[doc(alias = "AXGetLibraryVersion")]
    #[link_name = "AXGetLibraryVersion"]
    pub unsafe fn get_library_version() -> u32;

    /// AXIsAudioOutReady
    ///
    /// Whether the system audio output setting is ready.
    #[doc(alias = "AXIsAudioOutReady")]
    #[link_name = "AXIsAudioOutReady"]
    pub unsafe fn is_audio_out_ready() -> ffi::c_bool;

    /// AXSetUpTransitionAudio
    #[doc(alias = "AXSetUpTransitionAudio")]
    #[link_name = "AXSetUpTransitionAudio"]
    pub unsafe fn set_up_transition_audio(buffer: *mut TransitionAudioBuffer);

    /// AXStartTransitionAudio
    #[doc(alias = "AXStartTransitionAudio")]
    #[link_name = "AXStartTransitionAudio"]
    pub unsafe fn start_transition_audio();

    /// AXInitProfile
    ///
    /// Profiles of each audio frame are recorded into `profiles` until [get_swap_profile] is called. Double buffer the array.
    #[doc(alias = "AXInitProfile")]
    #[link_name = "AXInitProfile"]
    pub unsafe fn init_profile(profiles: *mut Profile, max_profiles: u32);

    /// AXGetSwapProfile
    ///
    /// Same as [init_profile], but returns the number of valid profiles recorded since the last call.
    #[doc(alias = "AXGetSwapProfile")]
    #[link_name = "AXGetSwapProfile"]
    pub unsafe fn get_swap_profile(profiles: *mut Profile, max_profiles: u32) -> u32;

    /// AXGetProfile
    #[doc(alias = "AXGetProfile")]
    #[link_name = "AXGetProfile"]
    #[deprecated]
    pub unsafe fn get_profile() -> u32;

    /// AXRegisterFrameCallback
    ///
    /// Called once each audio frame. Returns the previous callback.
    #[doc(alias = "AXRegisterFrameCallback")]
    #[link_name = "AXRegisterFrameCallback"]
    pub unsafe fn register_frame_callback(callback: FrameCallback) -> FrameCallback;

    /// AXRegisterAppFrameCallback
    ///
    /// Up to [MAX_APP_FRAME_CALLBACKS] callbacks, called each audio frame.
    #[doc(alias = "AXRegisterAppFrameCallback")]
    #[link_name = "AXRegisterAppFrameCallback"]
    pub unsafe fn register_app_frame_callback(callback: FrameCallback) -> Error;

    /// AXDeregisterAppFrameCallback
    #[doc(alias = "AXDeregisterAppFrameCallback")]
    #[link_name = "AXDeregisterAppFrameCallback"]
    pub unsafe fn deregister_app_frame_callback(callback: FrameCallback) -> Error;

    /// AXRegisterExceedCallback
    #[doc(alias = "AXRegisterExceedCallback")]
    #[link_name = "AXRegisterExceedCallback"]
    pub unsafe fn register_exceed_callback(callback: ExceedCallback);

    /// AXRegisterCallback
    #[doc(alias = "AXRegisterCallback")]
    #[link_name = "AXRegisterCallback"]
    #[deprecated(note = "Use register_frame_callback instead")]
    pub unsafe fn register_callback(callback: FrameCallback) -> FrameCallback;

    /// AXRegisterDRCCallback
    #[doc(alias = "AXRegisterDRCCallback")]
    #[link_name = "AXRegisterDRCCallback"]
    #[deprecated]
    pub unsafe fn register_drc_callback(callback: FrameCallback) -> FrameCallback;

    /// AXGetMaxVoices
    #[doc(alias = "AXGetMaxVoices")]
    #[link_name = "AXGetMaxVoices"]
    pub unsafe fn get_max_voices() -> u32;

    /// AXSetMaxVoices
    ///
    /// Only limits the usable voices, does not change memory allocation.
    #[doc(alias = "AXSetMaxVoices")]
    #[link_name = "AXSetMaxVoices"]
    pub unsafe fn set_max_voices(num: u32);

    /// AXGetMaxDspVoices
    #[doc(alias = "AXGetMaxDspVoices")]
    #[link_name = "AXGetMaxDspVoices"]
    pub unsafe fn get_max_dsp_voices() -> u32;

    /// AXSetMaxDspVoices
    ///
    /// Non-zero overrides the load calculation as the reason to move voices to the PPC.
    #[doc(alias = "AXSetMaxDspVoices")]
    #[link_name = "AXSetMaxDspVoices"]
    pub unsafe fn set_max_dsp_voices(max_voices: u32);

    /// AXGetDspLoadLimit
    #[doc(alias = "AXGetDspLoadLimit")]
    #[link_name = "AXGetDspLoadLimit"]
    pub unsafe fn get_dsp_load_limit() -> f32;

    /// AXSetDspLoadLimit
    ///
    /// Above this load (0.0-100.0) voices move to the PPC, or are dropped if pinned to the DSP.
    #[doc(alias = "AXSetDspLoadLimit")]
    #[link_name = "AXSetDspLoadLimit"]
    pub unsafe fn set_dsp_load_limit(limit: f32);

    /// AXGetPpcLoadLimit
    #[doc(alias = "AXGetPpcLoadLimit")]
    #[link_name = "AXGetPpcLoadLimit"]
    pub unsafe fn get_ppc_load_limit() -> f32;

    /// AXSetPpcLoadLimit
    ///
    /// Above this load (0.0-100.0) voices are dropped.
    #[doc(alias = "AXSetPpcLoadLimit")]
    #[link_name = "AXSetPpcLoadLimit"]
    pub unsafe fn set_ppc_load_limit(limit: f32);

    /// AXGetDspLoad
    ///
    /// Only call from the frame callback.
    #[doc(alias = "AXGetDspLoad")]
    #[link_name = "AXGetDspLoad"]
    pub unsafe fn get_dsp_load() -> f32;

    /// AXGetPpcLoad
    ///
    /// Only call from the frame callback.
    #[doc(alias = "AXGetPpcLoad")]
    #[link_name = "AXGetPpcLoad"]
    pub unsafe fn get_ppc_load() -> f32;

    /// AXGetNumVoices
    #[doc(alias = "AXGetNumVoices")]
    #[link_name = "AXGetNumVoices"]
    pub unsafe fn get_num_voices() -> u32;

    /// AXGetNumDspVoices
    #[doc(alias = "AXGetNumDspVoices")]
    #[link_name = "AXGetNumDspVoices"]
    pub unsafe fn get_num_dsp_voices() -> u32;

    /// AXGetDroppedVoiceCount
    #[doc(alias = "AXGetDroppedVoiceCount")]
    #[link_name = "AXGetDroppedVoiceCount"]
    pub unsafe fn get_dropped_voice_count() -> u32;

    /// AXGetMaxDspCycles
    #[doc(alias = "AXGetMaxDspCycles")]
    #[link_name = "AXGetMaxDspCycles"]
    #[deprecated]
    pub unsafe fn get_max_dsp_cycles() -> u32;

    /// AXSetMaxDspCycles
    #[doc(alias = "AXSetMaxDspCycles")]
    #[link_name = "AXSetMaxDspCycles"]
    #[deprecated]
    pub unsafe fn set_max_dsp_cycles(cycles: u32);

    /// AXGetDspCycles
    #[doc(alias = "AXGetDspCycles")]
    #[link_name = "AXGetDspCycles"]
    #[deprecated]
    pub unsafe fn get_dsp_cycles() -> u32;

    /// AXSetDefaultMixerSelect
    ///
    /// Renderer of newly acquired voices. Returns the previous one.
    #[doc(alias = "AXSetDefaultMixerSelect", alias = "AXSetDefaultRenderer")]
    #[link_name = "AXSetDefaultMixerSelect"]
    pub unsafe fn set_default_renderer(renderer: voice::Renderer) -> voice::Renderer;

    /// AXGetDefaultMixerSelect
    #[doc(alias = "AXGetDefaultMixerSelect", alias = "AXGetDefaultRenderer")]
    #[link_name = "AXGetDefaultMixerSelect"]
    pub unsafe fn get_default_renderer() -> voice::Renderer;

    /// AXGetMasterVolume
    #[doc(alias = "AXGetMasterVolume")]
    #[link_name = "AXGetMasterVolume"]
    pub unsafe fn get_master_volume() -> u16;

    /// AXSetMasterVolume
    #[doc(alias = "AXSetMasterVolume")]
    #[link_name = "AXSetMasterVolume"]
    pub unsafe fn set_master_volume(volume: u16);

    /// AXGetMasterDRCVolume
    #[doc(alias = "AXGetMasterDRCVolume")]
    #[link_name = "AXGetMasterDRCVolume"]
    #[deprecated(note = "Use device::get_volume instead")]
    pub unsafe fn get_master_drc_volume() -> u16;

    /// AXSetMasterDRCVolume
    #[doc(alias = "AXSetMasterDRCVolume")]
    #[link_name = "AXSetMasterDRCVolume"]
    #[deprecated(note = "Use device::set_volume instead")]
    pub unsafe fn set_master_drc_volume(volume: u16);

    /// AXGetMode
    #[doc(alias = "AXGetMode")]
    #[link_name = "AXGetMode"]
    #[deprecated(note = "Use device::get_mode instead")]
    pub unsafe fn get_mode() -> device::DeviceMode;

    /// AXSetMode
    #[doc(alias = "AXSetMode")]
    #[link_name = "AXSetMode"]
    #[deprecated]
    pub unsafe fn set_mode(mode: device::DeviceMode);

    /// AXSetStepMode
    #[doc(alias = "AXSetStepMode")]
    #[link_name = "AXSetStepMode"]
    pub unsafe fn set_step_mode(mode: u32);

    /// AXUserBegin
    ///
    /// Starts a user protection clause, ended by [user_end]. Protects data shared between threads and AX callbacks. Voice parameters changed inside are protected too. Can be nested up to [MAX_VOICES] times.
    #[doc(alias = "AXUserBegin")]
    #[link_name = "AXUserBegin"]
    pub unsafe fn user_begin() -> SyncError;

    /// AXUserEnd
    #[doc(alias = "AXUserEnd")]
    #[link_name = "AXUserEnd"]
    pub unsafe fn user_end() -> SyncError;

    /// AXUserIsProtected
    #[doc(alias = "AXUserIsProtected")]
    #[link_name = "AXUserIsProtected"]
    pub unsafe fn user_is_protected() -> ffi::c_bool;

    /// AXComputeLpfCoefs
    ///
    /// Computes the low-pass filter coefficients for a cutoff frequency `freq` in Hz.
    #[doc(alias = "AXComputeLpfCoefs")]
    #[link_name = "AXComputeLpfCoefs"]
    pub unsafe fn compute_lpf_coefs(freq: u16, a0: *mut u16, b0: *mut u16);

    /// AXGetAdpcmOutputSize
    #[doc(alias = "AXGetAdpcmOutputSize")]
    #[link_name = "AXGetAdpcmOutputSize"]
    pub unsafe fn get_adpcm_output_size(samples: i32) -> i32;

    /// AXGetAdpcmWorkSize
    #[doc(alias = "AXGetAdpcmWorkSize")]
    #[link_name = "AXGetAdpcmWorkSize"]
    pub unsafe fn get_adpcm_work_size(samples: i32) -> i32;

    /// AXGetAdpcmData
    ///
    /// Encodes mono 16-bit big-endian PCM into DSP-ADPCM. Sizes come from [get_adpcm_output_size] and [get_adpcm_work_size].
    #[doc(alias = "AXGetAdpcmData")]
    #[link_name = "AXGetAdpcmData"]
    pub unsafe fn get_adpcm_data(
        input: *const i16,
        samples: i32,
        rate: i32,
        loop_start: i32,
        loop_end: i32,
        output: *mut u8,
        info: *mut DspAdpcm,
        work: *mut ffi::c_void,
    ) -> i32;

    /// AXDecodeAdpcmData
    ///
    /// Decodes DSP-ADPCM into mono 16-bit big-endian PCM.
    #[doc(alias = "AXDecodeAdpcmData")]
    #[link_name = "AXDecodeAdpcmData"]
    pub unsafe fn decode_adpcm_data(
        input: *const u8,
        info: *mut DspAdpcm,
        samples: i32,
        output: *mut i16,
    ) -> i32;

    /// AXPrepareEfxData
    #[doc(alias = "AXPrepareEfxData")]
    #[link_name = "AXPrepareEfxData"]
    #[deprecated]
    pub unsafe fn prepare_efx_data(samples: *const ffi::c_void, size: u32);

    /// AXRegisterAuxACallback
    #[doc(alias = "AXRegisterAuxACallback")]
    #[link_name = "AXRegisterAuxACallback"]
    #[deprecated(note = "Use device::register_aux_callback instead")]
    pub unsafe fn register_aux_a_callback(callback: LegacyAuxCallback, context: *mut ffi::c_void);

    /// AXRegisterAuxBCallback
    #[doc(alias = "AXRegisterAuxBCallback")]
    #[link_name = "AXRegisterAuxBCallback"]
    #[deprecated(note = "Use device::register_aux_callback instead")]
    pub unsafe fn register_aux_b_callback(callback: LegacyAuxCallback, context: *mut ffi::c_void);

    /// AXRegisterAuxCCallback
    #[doc(alias = "AXRegisterAuxCCallback")]
    #[link_name = "AXRegisterAuxCCallback"]
    #[deprecated(note = "Use device::register_aux_callback instead")]
    pub unsafe fn register_aux_c_callback(callback: LegacyAuxCallback, context: *mut ffi::c_void);

    /// AXRegisterAuxDRCCallback
    #[doc(alias = "AXRegisterAuxDRCCallback")]
    #[link_name = "AXRegisterAuxDRCCallback"]
    #[deprecated(note = "Use device::register_aux_callback instead")]
    pub unsafe fn register_aux_drc_callback(callback: LegacyAuxCallback, context: *mut ffi::c_void);

    /// AXGetAuxACallback
    #[doc(alias = "AXGetAuxACallback")]
    #[link_name = "AXGetAuxACallback"]
    #[deprecated(note = "Use device::get_aux_callback instead")]
    pub unsafe fn get_aux_a_callback(
        callback: *mut LegacyAuxCallback,
        context: *mut *mut ffi::c_void,
    );

    /// AXGetAuxBCallback
    #[doc(alias = "AXGetAuxBCallback")]
    #[link_name = "AXGetAuxBCallback"]
    #[deprecated(note = "Use device::get_aux_callback instead")]
    pub unsafe fn get_aux_b_callback(
        callback: *mut LegacyAuxCallback,
        context: *mut *mut ffi::c_void,
    );

    /// AXGetAuxCCallback
    #[doc(alias = "AXGetAuxCCallback")]
    #[link_name = "AXGetAuxCCallback"]
    #[deprecated(note = "Use device::get_aux_callback instead")]
    pub unsafe fn get_aux_c_callback(
        callback: *mut LegacyAuxCallback,
        context: *mut *mut ffi::c_void,
    );

    /// AXGetAuxDRCCallback
    #[doc(alias = "AXGetAuxDRCCallback")]
    #[link_name = "AXGetAuxDRCCallback"]
    #[deprecated(note = "Use device::get_aux_callback instead")]
    pub unsafe fn get_aux_drc_callback(
        callback: *mut LegacyAuxCallback,
        context: *mut *mut ffi::c_void,
    );

    /// AXGetAuxAReturnVolume
    #[doc(alias = "AXGetAuxAReturnVolume")]
    #[link_name = "AXGetAuxAReturnVolume"]
    #[deprecated(note = "Use device::get_aux_return_volume instead")]
    pub unsafe fn get_aux_a_return_volume() -> u16;

    /// AXGetAuxBReturnVolume
    #[doc(alias = "AXGetAuxBReturnVolume")]
    #[link_name = "AXGetAuxBReturnVolume"]
    #[deprecated(note = "Use device::get_aux_return_volume instead")]
    pub unsafe fn get_aux_b_return_volume() -> u16;

    /// AXGetAuxCReturnVolume
    #[doc(alias = "AXGetAuxCReturnVolume")]
    #[link_name = "AXGetAuxCReturnVolume"]
    #[deprecated(note = "Use device::get_aux_return_volume instead")]
    pub unsafe fn get_aux_c_return_volume() -> u16;

    /// AXGetDRCAuxReturnVolume
    #[doc(alias = "AXGetDRCAuxReturnVolume")]
    #[link_name = "AXGetDRCAuxReturnVolume"]
    #[deprecated(note = "Use device::get_aux_return_volume instead")]
    pub unsafe fn get_drc_aux_return_volume() -> u16;

    /// AXSetAuxAReturnVolume
    #[doc(alias = "AXSetAuxAReturnVolume")]
    #[link_name = "AXSetAuxAReturnVolume"]
    #[deprecated(note = "Use device::set_aux_return_volume instead")]
    pub unsafe fn set_aux_a_return_volume(volume: u16);

    /// AXSetAuxBReturnVolume
    #[doc(alias = "AXSetAuxBReturnVolume")]
    #[link_name = "AXSetAuxBReturnVolume"]
    #[deprecated(note = "Use device::set_aux_return_volume instead")]
    pub unsafe fn set_aux_b_return_volume(volume: u16);

    /// AXSetAuxCReturnVolume
    #[doc(alias = "AXSetAuxCReturnVolume")]
    #[link_name = "AXSetAuxCReturnVolume"]
    #[deprecated(note = "Use device::set_aux_return_volume instead")]
    pub unsafe fn set_aux_c_return_volume(volume: u16);

    /// AXSetDRCAuxReturnVolume
    #[doc(alias = "AXSetDRCAuxReturnVolume")]
    #[link_name = "AXSetDRCAuxReturnVolume"]
    #[deprecated(note = "Use device::set_aux_return_volume instead")]
    pub unsafe fn set_drc_aux_return_volume(volume: u16);
}

imports_section!(
    "sndcore2",
    [
        "AI2CheckInit",
        "AI2GetDMABytesLeft",
        "AI2GetDMAEnableFlag",
        "AI2GetDMALength",
        "AI2GetDMAStartAddr",
        "AI2GetDSPSampleRate",
        "AI2Init",
        "AI2InitDMA",
        "AI2RegisterDMACallback",
        "AI2Reset",
        "AI2SetDSPSampleRate",
        "AI2StartDMA",
        "AI2StopDMA",
        "AICheckInit",
        "AIDRCGetDMALength",
        "AIDRCGetDMAStartAddr",
        "AIDRCRegisterDMACallback",
        "AIGetAudioFrameCount",
        "AIGetDMABytesLeft",
        "AIGetDMAEnableFlag",
        "AIGetDMALength",
        "AIGetDMARange",
        "AIGetDMAStartAddr",
        "AIGetDSPSampleRate",
        "AII2S3Init",
        "AII2S5Init",
        "AIInit",
        "AIInitDMA",
        "AIQuit",
        "AIRegisterDMABuffer",
        "AIRegisterDMACallback",
        "AIReset",
        "AISetChannel",
        "AISetCloneMode",
        "AISetDSPSampleRate",
        "AISetFormatChangeState",
        "AIStartDMA",
        "AIStopDMA",
        "AXAcquireMultiVoice",
        "AXAcquireVoice",
        "AXAcquireVoiceEx",
        "AXCheckVoiceOffsets",
        "AXComputeLpfCoefs",
        "AXDecodeAdpcmData",
        "AXDeregisterAppFrameCallback",
        "AXFreeMultiVoice",
        "AXFreeVoice",
        "AXGetAdpcmData",
        "AXGetAdpcmOutputSize",
        "AXGetAdpcmWorkSize",
        "AXGetAuxACallback",
        "AXGetAuxAReturnVolume",
        "AXGetAuxBCallback",
        "AXGetAuxBReturnVolume",
        "AXGetAuxCCallback",
        "AXGetAuxCReturnVolume",
        "AXGetAuxCallback",
        "AXGetAuxDRCCallback",
        "AXGetAuxReturnVolume",
        "AXGetCurrentParams",
        "AXGetDRCAuxReturnVolume",
        "AXGetDRCVSMode",
        "AXGetDSPCyclesFudgeFactor",
        "AXGetDefaultMixerSelect",
        "AXGetDeviceChannelCount",
        "AXGetDeviceFinalMixCallback",
        "AXGetDeviceFinalOutput",
        "AXGetDeviceMode",
        "AXGetDeviceRemixMatrix",
        "AXGetDeviceUpsampleStage",
        "AXGetDeviceVolume",
        "AXGetDroppedVoiceCount",
        "AXGetDspCycles",
        "AXGetDspLoad",
        "AXGetDspLoadLimit",
        "AXGetInputSamplesPerFrame",
        "AXGetInputSamplesPerSec",
        "AXGetLibraryVersion",
        "AXGetMasterDRCVolume",
        "AXGetMasterVolume",
        "AXGetMaxDspCycles",
        "AXGetMaxDspVoices",
        "AXGetMaxVoices",
        "AXGetMode",
        "AXGetMultiVoiceOffsets",
        "AXGetMultiVoiceReformatBufferSize",
        "AXGetMultiVoiceRenderer",
        "AXGetNumDspVoices",
        "AXGetNumVoices",
        "AXGetPostFinalMixCallback",
        "AXGetPpcLoad",
        "AXGetPpcLoadLimit",
        "AXGetProfile",
        "AXGetRendererFreq",
        "AXGetSwapProfile",
        "AXGetVoiceCurrentOffsetEx",
        "AXGetVoiceLoopCount",
        "AXGetVoiceMixerSelect",
        "AXGetVoiceOffsets",
        "AXGetVoiceOffsetsEx",
        "AXGlitch_GetCount",
        "AXGlitch_PrintTrace",
        "AXGlitch_RegisterCatcher",
        "AXGlitch_trace",
        "AXInit",
        "AXInitEx",
        "AXInitProfile",
        "AXInitWithParams",
        "AXIsAudioOutReady",
        "AXIsInit",
        "AXIsMultiVoiceRunning",
        "AXIsTransitionAudioDone",
        "AXIsVoiceRunning",
        "AXMakeCompressorTable",
        "AXMixGetSwapProfile",
        "AXPrepareEfxData",
        "AXQuit",
        "AXRegisterAppFrameCallback",
        "AXRegisterAuxACallback",
        "AXRegisterAuxBCallback",
        "AXRegisterAuxCCallback",
        "AXRegisterAuxCallback",
        "AXRegisterAuxDRCCallback",
        "AXRegisterCallback",
        "AXRegisterDRCCallback",
        "AXRegisterDeviceFinalMixCallback",
        "AXRegisterExceedCallback",
        "AXRegisterFrameCallback",
        "AXRegisterPostFinalMixCallback",
        "AXRmtAdvancePtr",
        "AXRmtGetSamples",
        "AXRmtGetSamplesLeft",
        "AXSetAuxAReturnVolume",
        "AXSetAuxBReturnVolume",
        "AXSetAuxCReturnVolume",
        "AXSetAuxReturnVolume",
        "AXSetDRCAuxReturnVolume",
        "AXSetDRCVSDownmixBalance",
        "AXSetDRCVSLC",
        "AXSetDRCVSLimiter",
        "AXSetDRCVSLimiterThreshold",
        "AXSetDRCVSMode",
        "AXSetDRCVSOutputGain",
        "AXSetDRCVSSpeakerPosition",
        "AXSetDRCVSSurroundDepth",
        "AXSetDRCVSSurroundLevelGain",
        "AXSetDSPCyclesFudgeFactor",
        "AXSetDefaultMixerSelect",
        "AXSetDeviceCompressor",
        "AXSetDeviceCompressorTable",
        "AXSetDeviceLinearUpsampler",
        "AXSetDeviceMode",
        "AXSetDeviceRemixMatrix",
        "AXSetDeviceUpsampleStage",
        "AXSetDeviceVolume",
        "AXSetDspLoadLimit",
        "AXSetMasterDRCVolume",
        "AXSetMasterVolume",
        "AXSetMaxDspCycles",
        "AXSetMaxDspVoices",
        "AXSetMaxVoices",
        "AXSetMode",
        "AXSetMultiVoiceAdpcm",
        "AXSetMultiVoiceAdpcmLoop",
        "AXSetMultiVoiceBiquad",
        "AXSetMultiVoiceBiquadCoefs",
        "AXSetMultiVoiceCurrentOffset",
        "AXSetMultiVoiceDeviceMix",
        "AXSetMultiVoiceEndOffset",
        "AXSetMultiVoiceLoop",
        "AXSetMultiVoiceLoopOffset",
        "AXSetMultiVoiceLpf",
        "AXSetMultiVoiceLpfCoefs",
        "AXSetMultiVoiceOffsets",
        "AXSetMultiVoicePriority",
        "AXSetMultiVoiceRenderer",
        "AXSetMultiVoiceSrc",
        "AXSetMultiVoiceSrcRatio",
        "AXSetMultiVoiceSrcType",
        "AXSetMultiVoiceState",
        "AXSetMultiVoiceType",
        "AXSetMultiVoiceVe",
        "AXSetMultiVoiceVeDelta",
        "AXSetPpcLoadLimit",
        "AXSetStepMode",
        "AXSetUpTransitionAudio",
        "AXSetVoiceAdpcm",
        "AXSetVoiceAdpcmLoop",
        "AXSetVoiceBiquad",
        "AXSetVoiceBiquadCoefs",
        "AXSetVoiceCurrentOffset",
        "AXSetVoiceCurrentOffsetEx",
        "AXSetVoiceDRCMix",
        "AXSetVoiceDeviceMix",
        "AXSetVoiceEndOffset",
        "AXSetVoiceEndOffsetEx",
        "AXSetVoiceInitialTimeDelay",
        "AXSetVoiceItdOn",
        "AXSetVoiceItdTarget",
        "AXSetVoiceLoop",
        "AXSetVoiceLoopOffset",
        "AXSetVoiceLoopOffsetEx",
        "AXSetVoiceLpf",
        "AXSetVoiceLpfCoefs",
        "AXSetVoiceMix",
        "AXSetVoiceMixerSelect",
        "AXSetVoiceOffsets",
        "AXSetVoiceOffsetsEx",
        "AXSetVoicePriority",
        "AXSetVoiceRmtIIR",
        "AXSetVoiceRmtIIRCoefs",
        "AXSetVoiceRmtMix",
        "AXSetVoiceRmtOn",
        "AXSetVoiceRmtSrc",
        "AXSetVoiceSamplesAddr",
        "AXSetVoiceSrc",
        "AXSetVoiceSrcRatio",
        "AXSetVoiceSrcType",
        "AXSetVoiceState",
        "AXSetVoiceType",
        "AXSetVoiceVe",
        "AXSetVoiceVeDelta",
        "AXStartTransitionAudio",
        "AXUpdateDeviceModes",
        "AXUserBegin",
        "AXUserEnd",
        "AXUserIsProtected",
        "AXVoiceBegin",
        "AXVoiceEnd",
        "AXVoiceIsProtected",
        "DRCVS_GetVersion",
        "DRCVS_Initialize",
        "DRCVS_Process",
        "DRCVS_Quit",
        "DRCVS_SetDirectionFilterCoef",
        "DRCVS_SetDownmixBalance",
        "DRCVS_SetEqCoef",
        "DRCVS_SetHeadphoneGain",
        "DRCVS_SetLimiter",
        "DRCVS_SetLimiterThreshold",
        "DRCVS_SetOutputMode",
        "DRCVS_SetSampleFreq",
        "DRCVS_SetSlevGain",
        "DRCVS_SetSpacialFilterCoef",
        "DRCVS_SetSpeakerGain",
        "DRCVS_SetSurroundDepth",
        "DSPAddTask",
        "DSPAssertInt",
        "DSPAssertTask",
        "DSPCancelTask",
        "DSPCheckInit",
        "DSPCheckMailFromDSP",
        "DSPCheckMailToDSP",
        "DSPGetDMAStatus",
        "DSPH_RunTask",
        "DSPH_WaitTaskComplete",
        "DSPHalt",
        "DSPInit",
        "DSPQuit",
        "DSPReadCPUToDSPMbox",
        "DSPReadMailFromDSP",
        "DSPReset",
        "DSPSendMailToDSP",
        "DSPUnhalt",
        "ISRREnable",
        "ISRRGetMsgQ",
        "ISRRGetNewMsg",
        "ISRRInit",
        "ISRRQuit",
        "ISRSendMessage",
        "_CheckMailFromDSP",
        "_CheckMailToDSP",
        "_InitDsp",
        "_ReadMailFromDSP",
        "_SendMailToDSP",
        "__AXGetMixMode",
        "aiBSPHardwareVersion",
        "check_os_audio_transition_flag",
        "dsp_os_switch",
        "dsp_os_switchLength",
        "dump_dsp_data",
        "ex_addr_reg_stat",
        "pcd",
        "ppf_dn_e_coef",
        "ppf_dn_o_coef",
        "ppf_up_0_coef",
        "ppf_up_1_coef",
        "ppf_up_2_coef",
        "s_board_type",
        "set_os_audio_transition_flag",
        "slaveData",
        "slaveLength",
        "start_os_audio_transition",
        "stop_os_audio_transition",
        "vs"
    ],
    []
);
