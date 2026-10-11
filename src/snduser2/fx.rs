//! AX effects
//!
//! Aux bus effects, run from aux callbacks. Effect memory is allocated through [set_hooks] / [set_mem_alloc_fns].

use crate::ffi;
use crate::sndcore2::device::AuxCallbackInfo;

pub const MULTI_CH_MAX_CHANNELS: usize = 6;

pub type Alloc = Option<unsafe extern "C" fn(size: u32) -> *mut ffi::c_void>;

pub type Free = Option<unsafe extern "C" fn(ptr: *mut ffi::c_void)>;

/// Paired single.
#[repr(C, align(8))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct F32x2(pub [f32; 2]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferUpdate {
    pub left: *mut i32,
    pub right: *mut i32,
    pub surround: *mut i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferUpdateDpl2 {
    pub left: *mut i32,
    pub right: *mut i32,
    pub left_surround: *mut i32,
    pub right_surround: *mut i32,
}

/// Each buffer holds one aux block of samples.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferUpdate6ch {
    pub left: *mut i32,
    pub right: *mut i32,
    pub left_surround: *mut i32,
    pub right_surround: *mut i32,
    pub center: *mut i32,
    pub subwoofer: *mut i32,
}

/// Send bus for chaining effects.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bus {
    pub left: *mut i32,
    pub right: *mut i32,
    pub surround: *mut i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BusDpl2 {
    pub left: *mut i32,
    pub right: *mut i32,
    pub left_surround: *mut i32,
    pub right_surround: *mut i32,
}

/// Early reflection mode.
#[doc()]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReverbEarlyMode(pub u32);

impl ReverbEarlyMode {
    pub const MS_5: Self = Self(0);
    pub const MS_10: Self = Self(1);
    pub const MS_15: Self = Self(2);
    pub const MS_20: Self = Self(3);
    pub const MS_25: Self = Self(4);
    pub const MS_30: Self = Self(5);
    pub const MS_35: Self = Self(6);
    pub const MS_40: Self = Self(7);
}

/// Fused reverb mode.
#[doc()]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReverbFusedMode(pub u32);

impl ReverbFusedMode {
    pub const OLD_AXFX: Self = Self(0);
    pub const METAL_TANK: Self = Self(1);
    pub const SMALL_ROOM: Self = Self(2);
    pub const LARGE_ROOM: Self = Self(3);
    pub const HALL: Self = Self(4);
    pub const CAVERNOUS: Self = Self(5);
    pub const LONGEST: Self = Self(6);
}

/// High quality reverb.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbHiExp {
    pub early_line: [*mut f32; 3],
    pub early_pos: [u32; 3],
    pub early_length: u32,
    pub early_max_length: u32,
    pub early_coef: [f32; 3],
    pub pre_delay_line: [*mut f32; 3],
    pub pre_delay_pos: u32,
    pub pre_delay_length: u32,
    pub pre_delay_max_length: u32,
    pub comb_line: [[*mut f32; 3]; 3],
    pub comb_pos: [u32; 3],
    pub comb_length: [u32; 3],
    pub comb_max_length: [u32; 3],
    pub comb_coef: [f32; 3],
    pub allpass_line: [[*mut f32; 2]; 3],
    pub allpass_pos: [u32; 2],
    pub allpass_length: [u32; 2],
    pub allpass_max_length: [u32; 2],
    pub last_allpass_line: [*mut f32; 3],
    pub last_allpass_pos: [u32; 3],
    pub last_allpass_length: [u32; 3],
    pub last_allpass_max_length: [u32; 3],
    pub allpass_coef: f32,
    pub last_lpf_out: [f32; 3],
    pub lpf_coef: f32,
    pub active: u32,
    pub early_mode: ReverbEarlyMode,
    pub pre_delay_time_max: f32,
    pub pre_delay_time: f32,
    pub fused_mode: ReverbFusedMode,
    pub fused_time: f32,
    pub coloration: f32,
    pub damping: f32,
    pub crosstalk: f32,
    pub early_gain: f32,
    pub fused_gain: f32,
    pub bus_in: *mut Bus,
    pub bus_out: *mut Bus,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbHiExpDpl2 {
    pub early_line: [*mut f32; 4],
    pub early_pos: [u32; 3],
    pub early_length: u32,
    pub early_max_length: u32,
    pub early_coef: [f32; 3],
    pub pre_delay_line: [*mut f32; 4],
    pub pre_delay_pos: u32,
    pub pre_delay_length: u32,
    pub pre_delay_max_length: u32,
    pub comb_line: [[*mut f32; 3]; 4],
    pub comb_pos: [u32; 3],
    pub comb_length: [u32; 3],
    pub comb_max_length: [u32; 3],
    pub comb_coef: [f32; 3],
    pub allpass_line: [[*mut f32; 2]; 4],
    pub allpass_pos: [u32; 2],
    pub allpass_length: [u32; 2],
    pub allpass_max_length: [u32; 2],
    pub last_allpass_line: [*mut f32; 4],
    pub last_allpass_pos: [u32; 4],
    pub last_allpass_length: [u32; 4],
    pub last_allpass_max_length: [u32; 4],
    pub allpass_coef: f32,
    pub last_lpf_out: [f32; 4],
    pub lpf_coef: f32,
    pub active: u32,
    pub early_mode: ReverbEarlyMode,
    pub pre_delay_time_max: f32,
    pub pre_delay_time: f32,
    pub fused_mode: ReverbFusedMode,
    pub fused_time: f32,
    pub coloration: f32,
    pub damping: f32,
    pub crosstalk: f32,
    pub early_gain: f32,
    pub fused_gain: f32,
    pub bus_in: *mut BusDpl2,
    pub bus_out: *mut BusDpl2,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbHi {
    pub reverb_inner: ReverbHiExp,
    pub coloration: f32,
    pub mix: f32,
    pub time: f32,
    pub damping: f32,
    pub pre_delay: f32,
    pub crosstalk: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbHiDpl2 {
    pub reverb_inner: ReverbHiExpDpl2,
    pub coloration: f32,
    pub mix: f32,
    pub time: f32,
    pub damping: f32,
    pub pre_delay: f32,
    pub crosstalk: f32,
}

/// Standard reverb.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbStdExp {
    pub early_line: [*mut f32; 3],
    pub early_pos: u32,
    pub early_length: u32,
    pub early_max_length: u32,
    pub early_coef: f32,
    pub pre_delay_line: [*mut f32; 3],
    pub pre_delay_pos: u32,
    pub pre_delay_length: u32,
    pub pre_delay_max_length: u32,
    pub comb_line: [[*mut f32; 2]; 3],
    pub comb_pos: [u32; 2],
    pub comb_length: [u32; 2],
    pub comb_max_length: [u32; 2],
    pub comb_coef: [f32; 2],
    pub allpass_line: [[*mut f32; 2]; 3],
    pub allpass_pos: [u32; 2],
    pub allpass_length: [u32; 2],
    pub allpass_max_length: [u32; 2],
    pub allpass_coef: f32,
    pub last_lpf_out: [f32; 3],
    pub lpf_coef: f32,
    pub active: u32,
    pub early_mode: ReverbEarlyMode,
    pub pre_delay_time_max: f32,
    pub pre_delay_time: f32,
    pub fused_mode: ReverbFusedMode,
    pub fused_time: f32,
    pub coloration: f32,
    pub damping: f32,
    pub early_gain: f32,
    pub fused_gain: f32,
    pub bus_in: *mut Bus,
    pub bus_out: *mut Bus,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbStdExpDpl2 {
    pub early_line: [*mut f32; 4],
    pub early_pos: u32,
    pub early_length: u32,
    pub early_max_length: u32,
    pub early_coef: f32,
    pub pre_delay_line: [*mut f32; 4],
    pub pre_delay_pos: u32,
    pub pre_delay_length: u32,
    pub pre_delay_max_length: u32,
    pub comb_line: [[*mut f32; 2]; 4],
    pub comb_pos: [u32; 2],
    pub comb_length: [u32; 2],
    pub comb_max_length: [u32; 2],
    pub comb_coef: [f32; 2],
    pub allpass_line: [[*mut f32; 2]; 4],
    pub allpass_pos: [u32; 2],
    pub allpass_length: [u32; 2],
    pub allpass_max_length: [u32; 2],
    pub allpass_coef: f32,
    pub last_lpf_out: [f32; 4],
    pub lpf_coef: f32,
    pub active: u32,
    pub early_mode: ReverbEarlyMode,
    pub pre_delay_time_max: f32,
    pub pre_delay_time: f32,
    pub fused_mode: ReverbFusedMode,
    pub fused_time: f32,
    pub coloration: f32,
    pub damping: f32,
    pub early_gain: f32,
    pub fused_gain: f32,
    pub bus_in: *mut BusDpl2,
    pub bus_out: *mut BusDpl2,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbStd {
    pub reverb_inner: ReverbStdExp,
    pub coloration: f32,
    pub mix: f32,
    pub time: f32,
    pub damping: f32,
    pub pre_delay: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbStdDpl2 {
    pub reverb_inner: ReverbStdExpDpl2,
    pub coloration: f32,
    pub mix: f32,
    pub time: f32,
    pub damping: f32,
    pub pre_delay: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayExp {
    pub line_left: *mut i32,
    pub line_right: *mut i32,
    pub line_surround: *mut i32,
    pub cur_pos: u32,
    pub length: u32,
    pub max_length: u32,
    pub feedback_gain: i32,
    pub last_lpf_out: [i32; 3],
    pub lpf_coef: i32,
    pub out_gain_calc: i32,
    pub send_gain_calc: i32,
    pub active: u32,
    pub max_delay: f32,
    pub delay: f32,
    pub feedback: f32,
    pub lpf: f32,
    pub bus_in: *mut Bus,
    pub bus_out: *mut Bus,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayExpDpl2 {
    pub line: [*mut i32; 4],
    pub cur_pos: u32,
    pub length: u32,
    pub max_length: u32,
    pub feedback_gain: i32,
    pub last_lpf_out: [i32; 4],
    pub lpf_coef: i32,
    pub out_gain_calc: i32,
    pub send_gain_calc: i32,
    pub active: u32,
    pub max_delay: f32,
    pub delay: f32,
    pub feedback: f32,
    pub lpf: f32,
    pub bus_in: *mut BusDpl2,
    pub bus_out: *mut BusDpl2,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Delay {
    pub line: [*mut i32; 3],
    pub cur_pos: [u32; 3],
    pub length: [u32; 3],
    pub feedback_gain: [i32; 3],
    pub out_gain: [i32; 3],
    pub active: u32,
    pub delay: [u32; 3],
    pub feedback: [u32; 3],
    pub output: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DelayDpl2 {
    pub line: [*mut i32; 4],
    pub cur_pos: [u32; 4],
    pub length: [u32; 4],
    pub feedback_gain: [i32; 4],
    pub out_gain: [i32; 4],
    pub active: u32,
    pub delay: [u32; 4],
    pub feedback: [u32; 4],
    pub output: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusExpDelay {
    pub line: [*mut f32; 3],
    pub in_pos: u32,
    pub out_pos: u32,
    pub last_pos: u32,
    pub size_fp: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusExpDelayDpl2 {
    pub line: [*mut f32; 4],
    pub in_pos: u32,
    pub out_pos: u32,
    pub last_pos: u32,
    pub size_fp: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusDelay6ch {
    pub line: [*mut f32; MULTI_CH_MAX_CHANNELS],
    pub in_pos: u32,
    pub out_pos: u32,
    pub last_pos: u32,
    pub size_fp: u32,
    pub size: u32,
}

#[doc()]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusLfo {
    pub table: *mut i32,
    pub phase_add: i32,
    pub step_samp: i32,
    pub depth_samp: i32,
    pub phase: u32,
    pub sign: u32,
    pub last_num: u32,
    pub last_value: i32,
    pub grad: i32,
    pub grad_factor: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChorusExp {
    pub delay: ChorusExpDelay,
    pub lfo: ChorusLfo,
    pub history: [[f32; 4]; 3],
    pub hist_index: u32,
    pub active: u32,
    pub delay_time: f32,
    pub depth: f32,
    pub rate: f32,
    pub feedback: f32,
    pub bus_in: *mut Bus,
    pub bus_out: *mut Bus,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChorusExpDpl2 {
    pub delay: ChorusExpDelayDpl2,
    pub lfo: ChorusLfo,
    pub history: [[f32; 4]; 4],
    pub hist_index: u32,
    pub active: u32,
    pub delay_time: f32,
    pub depth: f32,
    pub rate: f32,
    pub feedback: f32,
    pub bus_in: *mut BusDpl2,
    pub bus_out: *mut BusDpl2,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chorus {
    pub chorus_inner: ChorusExp,
    pub base_delay: u32,
    pub variation: u32,
    pub period: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChorusDpl2 {
    pub chorus_inner: ChorusExpDpl2,
    pub base_delay: u32,
    pub variation: u32,
    pub period: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleRate {
    Hz32000 = 1,
    Hz48000 = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReverbPreset {
    SmallRoom = 1,
    LargeRoom = 2,
    Hall = 3,
    CavernousCathedral = 4,
    MetalCorridor = 5,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReverbMode(pub u32);

impl ReverbMode {
    pub const INVALID: Self = Self(0);
    pub const CH6_MONO_MIXED: Self = Self(1);
    pub const CH2: Self = Self(2);
    pub const CH4: Self = Self(3);
    pub const CH6: Self = Self(4);
    pub const CH2_MONO_MIXED: Self = Self(5);
    pub const CH4_MONO_MIXED: Self = Self(6);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusMode(pub u32);

impl ChorusMode {
    pub const INVALID: Self = Self(0);
    pub const CH2: Self = Self(1);
    pub const CH4: Self = Self(2);
    pub const CH6: Self = Self(3);
    pub const CH2_2: Self = Self(4);
    pub const CH4_2: Self = Self(5);
    pub const CH6_2: Self = Self(6);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DelayMode(pub u32);

impl DelayMode {
    pub const INVALID: Self = Self(0);
    pub const CH2: Self = Self(1);
    pub const CH4: Self = Self(2);
    pub const CH6: Self = Self(3);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlangerMode {
    Ch2 = 1,
    Ch4 = 2,
    Ch6 = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverdriveMode {
    Ch2 = 1,
    Ch4 = 2,
    Ch6 = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompressorMode {
    Ch2 = 1,
    Ch4 = 2,
    Ch6 = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchshiftMode {
    Ch2 = 1,
    Ch4 = 2,
    Ch6 = 3,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DelayType(pub u32);

impl DelayType {
    pub const NONE: Self = Self(0);
    pub const DELAY: Self = Self(1);
    pub const ALLPASS: Self = Self(2);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayLine {
    pub kind: DelayType,
    /// Buffer length in samples, at least the maximum delay + 1.
    pub size: i32,
    pub buffer: *mut f32,
    /// Delay in samples, at most `size - 1`.
    pub delay: i32,
    pub input: i32,
    pub output: i32,
    /// Only used by all-pass lines.
    pub coef: f32,
}

/// Delay line processing two channels at once.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PsDelayLine {
    pub kind: DelayType,
    pub size: i32,
    pub buffer: *mut F32x2,
    pub delay: i32,
    pub input: i32,
    pub output: i32,
    pub coef: F32x2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MultiChReverb {
    pub early_mode: u32,
    pub late_mode: u32,
    pub predelay_time: f32,
    pub decay_time: f32,
    pub hf_decay_ratio: f32,
    pub coloration: f32,
    pub early_gain: f32,
    pub late_gain: f32,
    pub reverb_gain: f32,
    pub mode: ReverbMode,
    pub fs: u32,
    pub early_level: f32,
    pub late_level: f32,
    pub reverb_level: f32,
    pub early_delay: [DelayLine; MULTI_CH_MAX_CHANNELS],
    pub early_tap: [u32; 10],
    pub early_coef: [f32; 10],
    pub late_tap: u32,
    pub cos_w_hf: f32,
    pub ap_coef: f32,
    pub center_delay: DelayLine,
    pub fdn_delay: [DelayLine; 4],
    pub ap: [DelayLine; 4],
    pub fdn_fb: [f32; 4],
    pub fdn_lp_coef: [f32; 4],
    pub fdn_lp_z: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MultiChDelay {
    pub delay_time: f32,
    pub feedback: f32,
    pub low_pass_amount: f32,
    pub channel_spread: f32,
    pub out_gain: f32,
    pub mode: DelayMode,
    pub fs: u32,
    pub ps_delay: [PsDelayLine; MULTI_CH_MAX_CHANNELS / 2],
    pub fb: f32,
    pub fb_direct: f32,
    pub fb_cross: f32,
    pub lpf_b0: f32,
    pub lpf_a1: f32,
    pub ps_lpf_z: [F32x2; MULTI_CH_MAX_CHANNELS / 2],
    pub gain: f32,
    pub max_delay: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MultiChChorus {
    pub mode: ChorusMode,
    pub fs: u32,
    pub delay: ChorusDelay6ch,
    pub lfo: ChorusLfo,
    pub history: [[f32; 4]; MULTI_CH_MAX_CHANNELS],
    pub hist_index: u32,
    pub active: u32,
    pub delay_time: f32,
    pub depth: f32,
    pub rate: f32,
    pub feedback: f32,
    pub bus_in: *mut BufferUpdate6ch,
    pub bus_out: *mut BufferUpdate6ch,
    pub out_gain: f32,
    pub send_gain: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reverb2 {
    pub reverb_gain: f32,
    pub early_mode: u32,
    pub late_mode: u32,
    pub predelay_time: f32,
    pub decay_time: f32,
    pub hf_decay_ratio: f32,
    pub coloration: f32,
    pub early_gain: f32,
    pub late_gain: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reverb2I3dl2 {
    /// -10000 to 0 millibels.
    pub room: i32,
    /// -10000 to 0 millibels.
    pub room_hf: i32,
    /// 0.1-20.0 seconds.
    pub decay_time: f32,
    /// 0.1-2.0.
    pub decay_hf_ratio: f32,
    /// Millibels.
    pub reflections: i32,
    /// 0.0-0.3 seconds.
    pub reflections_delay: f32,
    /// Millibels.
    pub reverb: i32,
    /// 0.0-0.1 seconds.
    pub reverb_delay: f32,
    /// 0.0-100.0 percent.
    pub diffusion: f32,
    /// 0.0-100.0 percent.
    pub density: f32,
    /// 20.0-20000.0 Hz.
    pub hf_reference: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Delay2 {
    pub delay_time: f32,
    pub gain: f32,
    pub feedback: f32,
    pub low_pass_amount: f32,
    pub channel_spread: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    pub max_delay_time: f32,
    _pad: ffi::unknown<12>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chorus2 {
    pub delay_time: f32,
    pub mod_depth: f32,
    pub mod_rate: f32,
    pub mod_spread: f32,
    pub feedback: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flanger2 {
    pub delay_time: f32,
    pub mod_depth: f32,
    pub mod_rate: f32,
    pub feedback: f32,
    pub feedforward: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

/// AXFX2 overdrive distortion type
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistortionType(pub u32);

impl DistortionType {
    pub const SOFT_CLIP: Self = Self(0);
    pub const MEDIUM_CURVE: Self = Self(1);
    pub const HARD_CLIP: Self = Self(2);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overdrive2 {
    pub drive: f32,
    pub distortion_type: DistortionType,
    pub out_gain: f32,
    pub tone: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Compressor2 {
    pub threshold: f32,
    pub ratio: f32,
    pub attack_time: f32,
    pub release_time: f32,
    /// Boosts the gain to make up for the compression.
    pub makeup_gain: ffi::c_bool,
    pub out_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pitchshift2 {
    pub pitch_shift: f32,
    pub out_gain: f32,
    pub dry_gain: f32,
    pub private: *mut ffi::c_void,
    _pad: ffi::unknown<16>,
}

const _: () = {
    assert!(size_of::<F32x2>() == 0x8);
};

unsafe extern "C" {
    /// AXFXSetHooks
    #[doc(alias = "AXFXSetHooks")]
    #[link_name = "AXFXSetHooks"]
    pub unsafe fn set_hooks(alloc: Alloc, free: Free);

    /// AXFXGetHooks
    #[doc(alias = "AXFXGetHooks")]
    #[link_name = "AXFXGetHooks"]
    pub unsafe fn get_hooks(alloc: *mut Alloc, free: *mut Free);

    /// AXFX2SetMemAllocFns
    #[doc(alias = "AXFX2SetMemAllocFns")]
    #[link_name = "AXFX2SetMemAllocFns"]
    pub unsafe fn set_mem_alloc_fns(alloc: Alloc, free: Free) -> ffi::c_bool;

    /// AXFX2GetMemAllocFns
    #[doc(alias = "AXFX2GetMemAllocFns")]
    #[link_name = "AXFX2GetMemAllocFns"]
    pub unsafe fn get_mem_alloc_fns(alloc: *mut Alloc, free: *mut Free) -> ffi::c_bool;

    /// AXFXReverbHiExpGetMemSize
    #[doc(alias = "AXFXReverbHiExpGetMemSize")]
    #[link_name = "AXFXReverbHiExpGetMemSize"]
    pub unsafe fn reverb_hi_exp_get_mem_size(reverb: *mut ReverbHiExp) -> i32;

    /// AXFXReverbHiExpInit
    #[doc(alias = "AXFXReverbHiExpInit")]
    #[link_name = "AXFXReverbHiExpInit"]
    pub unsafe fn reverb_hi_exp_init(reverb: *mut ReverbHiExp) -> ffi::c_bool;

    /// AXFXReverbHiExpShutdown
    #[doc(alias = "AXFXReverbHiExpShutdown")]
    #[link_name = "AXFXReverbHiExpShutdown"]
    pub unsafe fn reverb_hi_exp_shutdown(reverb: *mut ReverbHiExp);

    /// AXFXReverbHiExpSettings
    #[doc(alias = "AXFXReverbHiExpSettings")]
    #[link_name = "AXFXReverbHiExpSettings"]
    pub unsafe fn reverb_hi_exp_settings(reverb: *mut ReverbHiExp) -> ffi::c_bool;

    /// AXFXReverbHiExpSettingsUpdate
    #[doc(alias = "AXFXReverbHiExpSettingsUpdate")]
    #[link_name = "AXFXReverbHiExpSettingsUpdate"]
    pub unsafe fn reverb_hi_exp_settings_update(reverb: *mut ReverbHiExp) -> ffi::c_bool;

    /// AXFXReverbHiExpCallback
    #[doc(alias = "AXFXReverbHiExpCallback")]
    #[link_name = "AXFXReverbHiExpCallback"]
    pub unsafe fn reverb_hi_exp_callback(
        buffer_update: *mut BufferUpdate,
        reverb: *mut ReverbHiExp,
    );

    /// AXFXReverbHiExpGetMemSizeDpl2
    #[doc(alias = "AXFXReverbHiExpGetMemSizeDpl2")]
    #[link_name = "AXFXReverbHiExpGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_get_mem_size_dpl2(reverb: *mut ReverbHiExpDpl2) -> i32;

    /// AXFXReverbHiExpInitDpl2
    #[doc(alias = "AXFXReverbHiExpInitDpl2")]
    #[link_name = "AXFXReverbHiExpInitDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_init_dpl2(reverb: *mut ReverbHiExpDpl2) -> ffi::c_bool;

    /// AXFXReverbHiExpShutdownDpl2
    #[doc(alias = "AXFXReverbHiExpShutdownDpl2")]
    #[link_name = "AXFXReverbHiExpShutdownDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_shutdown_dpl2(reverb: *mut ReverbHiExpDpl2);

    /// AXFXReverbHiExpSettingsDpl2
    #[doc(alias = "AXFXReverbHiExpSettingsDpl2")]
    #[link_name = "AXFXReverbHiExpSettingsDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_settings_dpl2(reverb: *mut ReverbHiExpDpl2) -> ffi::c_bool;

    /// AXFXReverbHiExpSettingsUpdateDpl2
    #[doc(alias = "AXFXReverbHiExpSettingsUpdateDpl2")]
    #[link_name = "AXFXReverbHiExpSettingsUpdateDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_settings_update_dpl2(reverb: *mut ReverbHiExpDpl2) -> ffi::c_bool;

    /// AXFXReverbHiExpCallbackDpl2
    #[doc(alias = "AXFXReverbHiExpCallbackDpl2")]
    #[link_name = "AXFXReverbHiExpCallbackDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_exp_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        reverb: *mut ReverbHiExpDpl2,
    );

    /// AXFXReverbHiGetMemSize
    #[doc(alias = "AXFXReverbHiGetMemSize")]
    #[link_name = "AXFXReverbHiGetMemSize"]
    pub unsafe fn reverb_hi_get_mem_size(reverb: *mut ReverbHi) -> i32;

    /// AXFXReverbHiInit
    #[doc(alias = "AXFXReverbHiInit")]
    #[link_name = "AXFXReverbHiInit"]
    pub unsafe fn reverb_hi_init(reverb: *mut ReverbHi) -> ffi::c_bool;

    /// AXFXReverbHiShutdown
    #[doc(alias = "AXFXReverbHiShutdown")]
    #[link_name = "AXFXReverbHiShutdown"]
    pub unsafe fn reverb_hi_shutdown(reverb: *mut ReverbHi) -> ffi::c_bool;

    /// AXFXReverbHiSettings
    #[doc(alias = "AXFXReverbHiSettings")]
    #[link_name = "AXFXReverbHiSettings"]
    pub unsafe fn reverb_hi_settings(reverb: *mut ReverbHi) -> ffi::c_bool;

    /// AXFXReverbHiCallback
    #[doc(alias = "AXFXReverbHiCallback")]
    #[link_name = "AXFXReverbHiCallback"]
    pub unsafe fn reverb_hi_callback(buffer_update: *mut BufferUpdate, reverb: *mut ReverbHi);

    /// AXFXReverbHiGetMemSizeDpl2
    #[doc(alias = "AXFXReverbHiGetMemSizeDpl2")]
    #[link_name = "AXFXReverbHiGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_get_mem_size_dpl2(reverb: *mut ReverbHiDpl2) -> i32;

    /// AXFXReverbHiInitDpl2
    #[doc(alias = "AXFXReverbHiInitDpl2")]
    #[link_name = "AXFXReverbHiInitDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_init_dpl2(reverb: *mut ReverbHiDpl2) -> ffi::c_bool;

    /// AXFXReverbHiShutdownDpl2
    #[doc(alias = "AXFXReverbHiShutdownDpl2")]
    #[link_name = "AXFXReverbHiShutdownDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_shutdown_dpl2(reverb: *mut ReverbHiDpl2) -> ffi::c_bool;

    /// AXFXReverbHiSettingsDpl2
    #[doc(alias = "AXFXReverbHiSettingsDpl2")]
    #[link_name = "AXFXReverbHiSettingsDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_settings_dpl2(reverb: *mut ReverbHiDpl2) -> ffi::c_bool;

    /// AXFXReverbHiCallbackDpl2
    #[doc(alias = "AXFXReverbHiCallbackDpl2")]
    #[link_name = "AXFXReverbHiCallbackDpl2"]
    #[deprecated]
    pub unsafe fn reverb_hi_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        reverb: *mut ReverbHiDpl2,
    );

    /// AXFXReverbStdExpGetMemSize
    #[doc(alias = "AXFXReverbStdExpGetMemSize")]
    #[link_name = "AXFXReverbStdExpGetMemSize"]
    pub unsafe fn reverb_std_exp_get_mem_size(reverb: *mut ReverbStdExp) -> i32;

    /// AXFXReverbStdExpInit
    #[doc(alias = "AXFXReverbStdExpInit")]
    #[link_name = "AXFXReverbStdExpInit"]
    pub unsafe fn reverb_std_exp_init(reverb: *mut ReverbStdExp) -> ffi::c_bool;

    /// AXFXReverbStdExpShutdown
    #[doc(alias = "AXFXReverbStdExpShutdown")]
    #[link_name = "AXFXReverbStdExpShutdown"]
    pub unsafe fn reverb_std_exp_shutdown(reverb: *mut ReverbStdExp);

    /// AXFXReverbStdExpSettings
    #[doc(alias = "AXFXReverbStdExpSettings")]
    #[link_name = "AXFXReverbStdExpSettings"]
    pub unsafe fn reverb_std_exp_settings(reverb: *mut ReverbStdExp) -> ffi::c_bool;

    /// AXFXReverbStdExpSettingsUpdate
    #[doc(alias = "AXFXReverbStdExpSettingsUpdate")]
    #[link_name = "AXFXReverbStdExpSettingsUpdate"]
    pub unsafe fn reverb_std_exp_settings_update(reverb: *mut ReverbStdExp) -> ffi::c_bool;

    /// AXFXReverbStdExpCallback
    #[doc(alias = "AXFXReverbStdExpCallback")]
    #[link_name = "AXFXReverbStdExpCallback"]
    pub unsafe fn reverb_std_exp_callback(
        buffer_update: *mut BufferUpdate,
        reverb: *mut ReverbStdExp,
    );

    /// AXFXReverbStdExpGetMemSizeDpl2
    #[doc(alias = "AXFXReverbStdExpGetMemSizeDpl2")]
    #[link_name = "AXFXReverbStdExpGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_get_mem_size_dpl2(reverb: *mut ReverbStdExpDpl2) -> i32;

    /// AXFXReverbStdExpInitDpl2
    #[doc(alias = "AXFXReverbStdExpInitDpl2")]
    #[link_name = "AXFXReverbStdExpInitDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_init_dpl2(reverb: *mut ReverbStdExpDpl2) -> ffi::c_bool;

    /// AXFXReverbStdExpShutdownDpl2
    #[doc(alias = "AXFXReverbStdExpShutdownDpl2")]
    #[link_name = "AXFXReverbStdExpShutdownDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_shutdown_dpl2(reverb: *mut ReverbStdExpDpl2);

    /// AXFXReverbStdExpSettingsDpl2
    #[doc(alias = "AXFXReverbStdExpSettingsDpl2")]
    #[link_name = "AXFXReverbStdExpSettingsDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_settings_dpl2(reverb: *mut ReverbStdExpDpl2) -> ffi::c_bool;

    /// AXFXReverbStdExpSettingsUpdateDpl2
    #[doc(alias = "AXFXReverbStdExpSettingsUpdateDpl2")]
    #[link_name = "AXFXReverbStdExpSettingsUpdateDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_settings_update_dpl2(reverb: *mut ReverbStdExpDpl2)
    -> ffi::c_bool;

    /// AXFXReverbStdExpCallbackDpl2
    #[doc(alias = "AXFXReverbStdExpCallbackDpl2")]
    #[link_name = "AXFXReverbStdExpCallbackDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_exp_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        reverb: *mut ReverbStdExpDpl2,
    );

    /// AXFXReverbStdGetMemSize
    #[doc(alias = "AXFXReverbStdGetMemSize")]
    #[link_name = "AXFXReverbStdGetMemSize"]
    pub unsafe fn reverb_std_get_mem_size(reverb: *mut ReverbStd) -> i32;

    /// AXFXReverbStdInit
    #[doc(alias = "AXFXReverbStdInit")]
    #[link_name = "AXFXReverbStdInit"]
    pub unsafe fn reverb_std_init(reverb: *mut ReverbStd) -> ffi::c_bool;

    /// AXFXReverbStdShutdown
    #[doc(alias = "AXFXReverbStdShutdown")]
    #[link_name = "AXFXReverbStdShutdown"]
    pub unsafe fn reverb_std_shutdown(reverb: *mut ReverbStd) -> ffi::c_bool;

    /// AXFXReverbStdSettings
    #[doc(alias = "AXFXReverbStdSettings")]
    #[link_name = "AXFXReverbStdSettings"]
    pub unsafe fn reverb_std_settings(reverb: *mut ReverbStd) -> ffi::c_bool;

    /// AXFXReverbStdCallback
    #[doc(alias = "AXFXReverbStdCallback")]
    #[link_name = "AXFXReverbStdCallback"]
    pub unsafe fn reverb_std_callback(buffer_update: *mut BufferUpdate, reverb: *mut ReverbStd);

    /// AXFXReverbStdGetMemSizeDpl2
    #[doc(alias = "AXFXReverbStdGetMemSizeDpl2")]
    #[link_name = "AXFXReverbStdGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_get_mem_size_dpl2(reverb: *mut ReverbStdDpl2) -> i32;

    /// AXFXReverbStdInitDpl2
    #[doc(alias = "AXFXReverbStdInitDpl2")]
    #[link_name = "AXFXReverbStdInitDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_init_dpl2(reverb: *mut ReverbStdDpl2) -> ffi::c_bool;

    /// AXFXReverbStdShutdownDpl2
    #[doc(alias = "AXFXReverbStdShutdownDpl2")]
    #[link_name = "AXFXReverbStdShutdownDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_shutdown_dpl2(reverb: *mut ReverbStdDpl2) -> ffi::c_bool;

    /// AXFXReverbStdSettingsDpl2
    #[doc(alias = "AXFXReverbStdSettingsDpl2")]
    #[link_name = "AXFXReverbStdSettingsDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_settings_dpl2(reverb: *mut ReverbStdDpl2) -> ffi::c_bool;

    /// AXFXReverbStdCallbackDpl2
    #[doc(alias = "AXFXReverbStdCallbackDpl2")]
    #[link_name = "AXFXReverbStdCallbackDpl2"]
    #[deprecated]
    pub unsafe fn reverb_std_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        reverb: *mut ReverbStdDpl2,
    );

    /// AXFXDelayExpGetMemSize
    #[doc(alias = "AXFXDelayExpGetMemSize")]
    #[link_name = "AXFXDelayExpGetMemSize"]
    pub unsafe fn delay_exp_get_mem_size(delay: *mut DelayExp) -> i32;

    /// AXFXDelayExpInit
    #[doc(alias = "AXFXDelayExpInit")]
    #[link_name = "AXFXDelayExpInit"]
    pub unsafe fn delay_exp_init(delay: *mut DelayExp) -> ffi::c_bool;

    /// AXFXDelayExpShutdown
    #[doc(alias = "AXFXDelayExpShutdown")]
    #[link_name = "AXFXDelayExpShutdown"]
    pub unsafe fn delay_exp_shutdown(delay: *mut DelayExp);

    /// AXFXDelayExpSettings
    #[doc(alias = "AXFXDelayExpSettings")]
    #[link_name = "AXFXDelayExpSettings"]
    pub unsafe fn delay_exp_settings(delay: *mut DelayExp) -> ffi::c_bool;

    /// AXFXDelayExpSettingsUpdate
    #[doc(alias = "AXFXDelayExpSettingsUpdate")]
    #[link_name = "AXFXDelayExpSettingsUpdate"]
    pub unsafe fn delay_exp_settings_update(delay: *mut DelayExp) -> ffi::c_bool;

    /// AXFXDelayExpCallback
    #[doc(alias = "AXFXDelayExpCallback")]
    #[link_name = "AXFXDelayExpCallback"]
    pub unsafe fn delay_exp_callback(buffer_update: *mut BufferUpdate, delay: *mut DelayExp);

    /// AXFXDelayExpGetMemSizeDpl2
    #[doc(alias = "AXFXDelayExpGetMemSizeDpl2")]
    #[link_name = "AXFXDelayExpGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_get_mem_size_dpl2(delay: *mut DelayExpDpl2) -> i32;

    /// AXFXDelayExpInitDpl2
    #[doc(alias = "AXFXDelayExpInitDpl2")]
    #[link_name = "AXFXDelayExpInitDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_init_dpl2(delay: *mut DelayExpDpl2) -> ffi::c_bool;

    /// AXFXDelayExpShutdownDpl2
    #[doc(alias = "AXFXDelayExpShutdownDpl2")]
    #[link_name = "AXFXDelayExpShutdownDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_shutdown_dpl2(delay: *mut DelayExpDpl2);

    /// AXFXDelayExpSettingsDpl2
    #[doc(alias = "AXFXDelayExpSettingsDpl2")]
    #[link_name = "AXFXDelayExpSettingsDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_settings_dpl2(delay: *mut DelayExpDpl2) -> ffi::c_bool;

    /// AXFXDelayExpSettingsUpdateDpl2
    #[doc(alias = "AXFXDelayExpSettingsUpdateDpl2")]
    #[link_name = "AXFXDelayExpSettingsUpdateDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_settings_update_dpl2(delay: *mut DelayExpDpl2) -> ffi::c_bool;

    /// AXFXDelayExpCallbackDpl2
    #[doc(alias = "AXFXDelayExpCallbackDpl2")]
    #[link_name = "AXFXDelayExpCallbackDpl2"]
    #[deprecated]
    pub unsafe fn delay_exp_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        delay: *mut DelayExpDpl2,
    );

    /// AXFXDelayGetMemSize
    #[doc(alias = "AXFXDelayGetMemSize")]
    #[link_name = "AXFXDelayGetMemSize"]
    pub unsafe fn delay_get_mem_size(delay: *mut Delay) -> i32;

    /// AXFXDelayInit
    #[doc(alias = "AXFXDelayInit")]
    #[link_name = "AXFXDelayInit"]
    pub unsafe fn delay_init(delay: *mut Delay) -> ffi::c_bool;

    /// AXFXDelayShutdown
    #[doc(alias = "AXFXDelayShutdown")]
    #[link_name = "AXFXDelayShutdown"]
    pub unsafe fn delay_shutdown(delay: *mut Delay);

    /// AXFXDelaySettings
    #[doc(alias = "AXFXDelaySettings")]
    #[link_name = "AXFXDelaySettings"]
    pub unsafe fn delay_settings(delay: *mut Delay) -> ffi::c_bool;

    /// AXFXDelayCallback
    #[doc(alias = "AXFXDelayCallback")]
    #[link_name = "AXFXDelayCallback"]
    pub unsafe fn delay_callback(buffer_update: *mut BufferUpdate, delay: *mut Delay);

    /// AXFXDelayGetMemSizeDpl2
    #[doc(alias = "AXFXDelayGetMemSizeDpl2")]
    #[link_name = "AXFXDelayGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn delay_get_mem_size_dpl2(delay: *mut DelayDpl2) -> i32;

    /// AXFXDelayInitDpl2
    #[doc(alias = "AXFXDelayInitDpl2")]
    #[link_name = "AXFXDelayInitDpl2"]
    #[deprecated]
    pub unsafe fn delay_init_dpl2(delay: *mut DelayDpl2) -> ffi::c_bool;

    /// AXFXDelayShutdownDpl2
    #[doc(alias = "AXFXDelayShutdownDpl2")]
    #[link_name = "AXFXDelayShutdownDpl2"]
    #[deprecated]
    pub unsafe fn delay_shutdown_dpl2(delay: *mut DelayDpl2);

    /// AXFXDelaySettingsDpl2
    #[doc(alias = "AXFXDelaySettingsDpl2")]
    #[link_name = "AXFXDelaySettingsDpl2"]
    #[deprecated]
    pub unsafe fn delay_settings_dpl2(delay: *mut DelayDpl2) -> ffi::c_bool;

    /// AXFXDelayCallbackDpl2
    #[doc(alias = "AXFXDelayCallbackDpl2")]
    #[link_name = "AXFXDelayCallbackDpl2"]
    #[deprecated]
    pub unsafe fn delay_callback_dpl2(buffer_update: *mut BufferUpdateDpl2, delay: *mut DelayDpl2);

    /// AXFXChorusExpGetMemSize
    #[doc(alias = "AXFXChorusExpGetMemSize")]
    #[link_name = "AXFXChorusExpGetMemSize"]
    pub unsafe fn chorus_exp_get_mem_size(chorus: *mut ChorusExp) -> i32;

    /// AXFXChorusExpInit
    #[doc(alias = "AXFXChorusExpInit")]
    #[link_name = "AXFXChorusExpInit"]
    pub unsafe fn chorus_exp_init(chorus: *mut ChorusExp) -> ffi::c_bool;

    /// AXFXChorusExpSettings
    #[doc(alias = "AXFXChorusExpSettings")]
    #[link_name = "AXFXChorusExpSettings"]
    pub unsafe fn chorus_exp_settings(chorus: *mut ChorusExp) -> ffi::c_bool;

    /// AXFXChorusExpSettingsUpdate
    #[doc(alias = "AXFXChorusExpSettingsUpdate")]
    #[link_name = "AXFXChorusExpSettingsUpdate"]
    pub unsafe fn chorus_exp_settings_update(chorus: *mut ChorusExp) -> ffi::c_bool;

    /// AXFXChorusExpShutdown
    #[doc(alias = "AXFXChorusExpShutdown")]
    #[link_name = "AXFXChorusExpShutdown"]
    pub unsafe fn chorus_exp_shutdown(chorus: *mut ChorusExp);

    /// AXFXChorusExpCallback
    #[doc(alias = "AXFXChorusExpCallback")]
    #[link_name = "AXFXChorusExpCallback"]
    pub unsafe fn chorus_exp_callback(buffer_update: *mut BufferUpdate, chorus: *mut ChorusExp);

    /// AXFXChorusExpGetMemSizeDpl2
    #[doc(alias = "AXFXChorusExpGetMemSizeDpl2")]
    #[link_name = "AXFXChorusExpGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_get_mem_size_dpl2(chorus: *mut ChorusExpDpl2) -> i32;

    /// AXFXChorusExpInitDpl2
    #[doc(alias = "AXFXChorusExpInitDpl2")]
    #[link_name = "AXFXChorusExpInitDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_init_dpl2(chorus: *mut ChorusExpDpl2) -> ffi::c_bool;

    /// AXFXChorusExpSettingsDpl2
    #[doc(alias = "AXFXChorusExpSettingsDpl2")]
    #[link_name = "AXFXChorusExpSettingsDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_settings_dpl2(chorus: *mut ChorusExpDpl2) -> ffi::c_bool;

    /// AXFXChorusExpSettingsUpdateDpl2
    #[doc(alias = "AXFXChorusExpSettingsUpdateDpl2")]
    #[link_name = "AXFXChorusExpSettingsUpdateDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_settings_update_dpl2(chorus: *mut ChorusExpDpl2) -> ffi::c_bool;

    /// AXFXChorusExpShutdownDpl2
    #[doc(alias = "AXFXChorusExpShutdownDpl2")]
    #[link_name = "AXFXChorusExpShutdownDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_shutdown_dpl2(chorus: *mut ChorusExpDpl2);

    /// AXFXChorusExpCallbackDpl2
    #[doc(alias = "AXFXChorusExpCallbackDpl2")]
    #[link_name = "AXFXChorusExpCallbackDpl2"]
    #[deprecated]
    pub unsafe fn chorus_exp_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        chorus: *mut ChorusExpDpl2,
    );

    /// AXFXChorusGetMemSize
    #[doc(alias = "AXFXChorusGetMemSize")]
    #[link_name = "AXFXChorusGetMemSize"]
    pub unsafe fn chorus_get_mem_size(chorus: *mut Chorus) -> i32;

    /// AXFXChorusInit
    #[doc(alias = "AXFXChorusInit")]
    #[link_name = "AXFXChorusInit"]
    pub unsafe fn chorus_init(chorus: *mut Chorus) -> ffi::c_bool;

    /// AXFXChorusShutdown
    #[doc(alias = "AXFXChorusShutdown")]
    #[link_name = "AXFXChorusShutdown"]
    pub unsafe fn chorus_shutdown(chorus: *mut Chorus) -> ffi::c_bool;

    /// AXFXChorusSettings
    #[doc(alias = "AXFXChorusSettings")]
    #[link_name = "AXFXChorusSettings"]
    pub unsafe fn chorus_settings(chorus: *mut Chorus) -> ffi::c_bool;

    /// AXFXChorusCallback
    #[doc(alias = "AXFXChorusCallback")]
    #[link_name = "AXFXChorusCallback"]
    pub unsafe fn chorus_callback(buffer_update: *mut BufferUpdate, chorus: *mut Chorus);

    /// AXFXChorusGetMemSizeDpl2
    #[doc(alias = "AXFXChorusGetMemSizeDpl2")]
    #[link_name = "AXFXChorusGetMemSizeDpl2"]
    #[deprecated]
    pub unsafe fn chorus_get_mem_size_dpl2(chorus: *mut ChorusDpl2) -> i32;

    /// AXFXChorusInitDpl2
    #[doc(alias = "AXFXChorusInitDpl2")]
    #[link_name = "AXFXChorusInitDpl2"]
    #[deprecated]
    pub unsafe fn chorus_init_dpl2(chorus: *mut ChorusDpl2) -> ffi::c_bool;

    /// AXFXChorusShutdownDpl2
    #[doc(alias = "AXFXChorusShutdownDpl2")]
    #[link_name = "AXFXChorusShutdownDpl2"]
    #[deprecated]
    pub unsafe fn chorus_shutdown_dpl2(chorus: *mut ChorusDpl2) -> ffi::c_bool;

    /// AXFXChorusSettingsDpl2
    #[doc(alias = "AXFXChorusSettingsDpl2")]
    #[link_name = "AXFXChorusSettingsDpl2"]
    #[deprecated]
    pub unsafe fn chorus_settings_dpl2(chorus: *mut ChorusDpl2) -> ffi::c_bool;

    /// AXFXChorusCallbackDpl2
    #[doc(alias = "AXFXChorusCallbackDpl2")]
    #[link_name = "AXFXChorusCallbackDpl2"]
    #[deprecated]
    pub unsafe fn chorus_callback_dpl2(
        buffer_update: *mut BufferUpdateDpl2,
        chorus: *mut ChorusDpl2,
    );

    /// AXFXMultiChReverbGetMemSize
    #[doc(alias = "AXFXMultiChReverbGetMemSize")]
    #[link_name = "AXFXMultiChReverbGetMemSize"]
    pub unsafe fn multi_ch_reverb_get_mem_size(reverb: *mut MultiChReverb) -> i32;

    /// AXFXMultiChReverbInit
    #[doc(alias = "AXFXMultiChReverbInit")]
    #[link_name = "AXFXMultiChReverbInit"]
    pub unsafe fn multi_ch_reverb_init(
        reverb: *mut MultiChReverb,
        mode: ReverbMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFXMultiChReverbShutdown
    #[doc(alias = "AXFXMultiChReverbShutdown")]
    #[link_name = "AXFXMultiChReverbShutdown"]
    pub unsafe fn multi_ch_reverb_shutdown(reverb: *mut MultiChReverb);

    /// AXFXMultiChReverbParametersPreset
    #[doc(alias = "AXFXMultiChReverbParametersPreset")]
    #[link_name = "AXFXMultiChReverbParametersPreset"]
    pub unsafe fn multi_ch_reverb_parameters_preset(
        reverb: *mut MultiChReverb,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFXMultiChReverbSettingsUpdate
    #[doc(alias = "AXFXMultiChReverbSettingsUpdate")]
    #[link_name = "AXFXMultiChReverbSettingsUpdate"]
    pub unsafe fn multi_ch_reverb_settings_update(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFXMultiChReverbSettingsUpdateNoReset
    #[doc(alias = "AXFXMultiChReverbSettingsUpdateNoReset")]
    #[link_name = "AXFXMultiChReverbSettingsUpdateNoReset"]
    pub unsafe fn multi_ch_reverb_settings_update_no_reset(
        reverb: *mut MultiChReverb,
    ) -> ffi::c_bool;

    /// AXFXMultiChReverbCallback
    #[doc(alias = "AXFXMultiChReverbCallback")]
    #[link_name = "AXFXMultiChReverbCallback"]
    pub unsafe fn multi_ch_reverb_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut MultiChReverb,
        info: *mut AuxCallbackInfo,
    );

    /// AXFXMultiChDelayGetMemSize
    #[doc(alias = "AXFXMultiChDelayGetMemSize")]
    #[link_name = "AXFXMultiChDelayGetMemSize"]
    pub unsafe fn multi_ch_delay_get_mem_size(delay: *mut MultiChDelay) -> i32;

    /// AXFXMultiChDelayInit
    #[doc(alias = "AXFXMultiChDelayInit")]
    #[link_name = "AXFXMultiChDelayInit"]
    pub unsafe fn multi_ch_delay_init(
        delay: *mut MultiChDelay,
        mode: DelayMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFXMultiChDelayShutdown
    #[doc(alias = "AXFXMultiChDelayShutdown")]
    #[link_name = "AXFXMultiChDelayShutdown"]
    pub unsafe fn multi_ch_delay_shutdown(delay: *mut MultiChDelay);

    /// AXFXMultiChDelaySettingsUpdate
    #[doc(alias = "AXFXMultiChDelaySettingsUpdate")]
    #[link_name = "AXFXMultiChDelaySettingsUpdate"]
    pub unsafe fn multi_ch_delay_settings_update(delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFXMultiChDelaySettingsUpdateNoReset
    #[doc(alias = "AXFXMultiChDelaySettingsUpdateNoReset")]
    #[link_name = "AXFXMultiChDelaySettingsUpdateNoReset"]
    pub unsafe fn multi_ch_delay_settings_update_no_reset(delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFXMultiChDelayCallback
    #[doc(alias = "AXFXMultiChDelayCallback")]
    #[link_name = "AXFXMultiChDelayCallback"]
    pub unsafe fn multi_ch_delay_callback(
        buffer_update: *mut BufferUpdate6ch,
        delay: *mut MultiChDelay,
        info: *mut AuxCallbackInfo,
    );

    /// AXFXMultiChChorusGetMemSize
    #[doc(alias = "AXFXMultiChChorusGetMemSize")]
    #[link_name = "AXFXMultiChChorusGetMemSize"]
    pub unsafe fn multi_ch_chorus_get_mem_size(chorus: *mut MultiChChorus) -> i32;

    /// AXFXMultiChChorusInit
    #[doc(alias = "AXFXMultiChChorusInit")]
    #[link_name = "AXFXMultiChChorusInit"]
    pub unsafe fn multi_ch_chorus_init(
        chorus: *mut MultiChChorus,
        mode: ChorusMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFXMultiChChorusShutdown
    #[doc(alias = "AXFXMultiChChorusShutdown")]
    #[link_name = "AXFXMultiChChorusShutdown"]
    pub unsafe fn multi_ch_chorus_shutdown(chorus: *mut MultiChChorus);

    /// AXFXMultiChChorusSettings
    #[doc(alias = "AXFXMultiChChorusSettings")]
    #[link_name = "AXFXMultiChChorusSettings"]
    pub unsafe fn multi_ch_chorus_settings(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFXMultiChChorusSettingsUpdate
    #[doc(alias = "AXFXMultiChChorusSettingsUpdate")]
    #[link_name = "AXFXMultiChChorusSettingsUpdate"]
    pub unsafe fn multi_ch_chorus_settings_update(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFXMultiChChorusSettingsUpdateNoReset
    #[doc(alias = "AXFXMultiChChorusSettingsUpdateNoReset")]
    #[link_name = "AXFXMultiChChorusSettingsUpdateNoReset"]
    pub unsafe fn multi_ch_chorus_settings_update_no_reset(
        chorus: *mut MultiChChorus,
    ) -> ffi::c_bool;

    /// AXFXMultiChChorusCallback
    #[doc(alias = "AXFXMultiChChorusCallback")]
    #[link_name = "AXFXMultiChChorusCallback"]
    pub unsafe fn multi_ch_chorus_callback(
        buffer_update: *mut BufferUpdate6ch,
        chorus: *mut MultiChChorus,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2chReverbGetMemSize
    #[doc(alias = "AXFX2chReverbGetMemSize")]
    #[link_name = "AXFX2chReverbGetMemSize"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_get_mem_size(reverb: *mut MultiChReverb) -> i32;

    /// AXFX2chReverbInit
    #[doc(alias = "AXFX2chReverbInit")]
    #[link_name = "AXFX2chReverbInit"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_init(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX2chReverbShutdown
    #[doc(alias = "AXFX2chReverbShutdown")]
    #[link_name = "AXFX2chReverbShutdown"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_shutdown(reverb: *mut MultiChReverb);

    /// AXFX2chReverbParametersPreset
    #[doc(alias = "AXFX2chReverbParametersPreset")]
    #[link_name = "AXFX2chReverbParametersPreset"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_parameters_preset(
        params: *mut MultiChReverb,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX2chReverbSettingsUpdate
    #[doc(alias = "AXFX2chReverbSettingsUpdate")]
    #[link_name = "AXFX2chReverbSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_settings_update(params: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX2chReverbCallback
    #[doc(alias = "AXFX2chReverbCallback")]
    #[link_name = "AXFX2chReverbCallback"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_2ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut MultiChReverb,
    );

    /// AXFX4chReverbGetMemSize
    #[doc(alias = "AXFX4chReverbGetMemSize")]
    #[link_name = "AXFX4chReverbGetMemSize"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_get_mem_size(reverb: *mut MultiChReverb) -> i32;

    /// AXFX4chReverbInit
    #[doc(alias = "AXFX4chReverbInit")]
    #[link_name = "AXFX4chReverbInit"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_init(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX4chReverbShutdown
    #[doc(alias = "AXFX4chReverbShutdown")]
    #[link_name = "AXFX4chReverbShutdown"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_shutdown(reverb: *mut MultiChReverb);

    /// AXFX4chReverbParametersPreset
    #[doc(alias = "AXFX4chReverbParametersPreset")]
    #[link_name = "AXFX4chReverbParametersPreset"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_parameters_preset(
        params: *mut MultiChReverb,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX4chReverbSettingsUpdate
    #[doc(alias = "AXFX4chReverbSettingsUpdate")]
    #[link_name = "AXFX4chReverbSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_settings_update(params: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX4chReverbCallback
    #[doc(alias = "AXFX4chReverbCallback")]
    #[link_name = "AXFX4chReverbCallback"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_4ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut MultiChReverb,
    );

    /// AXFX6chReverbGetMemSize
    #[doc(alias = "AXFX6chReverbGetMemSize")]
    #[link_name = "AXFX6chReverbGetMemSize"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_get_mem_size(reverb: *mut MultiChReverb) -> i32;

    /// AXFX6chReverbInit
    #[doc(alias = "AXFX6chReverbInit")]
    #[link_name = "AXFX6chReverbInit"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_init(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX6chReverbShutdown
    #[doc(alias = "AXFX6chReverbShutdown")]
    #[link_name = "AXFX6chReverbShutdown"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_shutdown(reverb: *mut MultiChReverb);

    /// AXFX6chReverbParametersPreset
    #[doc(alias = "AXFX6chReverbParametersPreset")]
    #[link_name = "AXFX6chReverbParametersPreset"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_parameters_preset(
        params: *mut MultiChReverb,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX6chReverbSettingsUpdate
    #[doc(alias = "AXFX6chReverbSettingsUpdate")]
    #[link_name = "AXFX6chReverbSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_settings_update(params: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX6chReverbCallback
    #[doc(alias = "AXFX6chReverbCallback")]
    #[link_name = "AXFX6chReverbCallback"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut MultiChReverb,
    );

    /// AXFX6ch6chReverbGetMemSize
    #[doc(alias = "AXFX6ch6chReverbGetMemSize")]
    #[link_name = "AXFX6ch6chReverbGetMemSize"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_get_mem_size(reverb: *mut MultiChReverb) -> i32;

    /// AXFX6ch6chReverbInit
    #[doc(alias = "AXFX6ch6chReverbInit")]
    #[link_name = "AXFX6ch6chReverbInit"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_init(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX6ch6chReverbShutdown
    #[doc(alias = "AXFX6ch6chReverbShutdown")]
    #[link_name = "AXFX6ch6chReverbShutdown"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_shutdown(reverb: *mut MultiChReverb);

    /// AXFX6ch6chReverbParametersPreset
    #[doc(alias = "AXFX6ch6chReverbParametersPreset")]
    #[link_name = "AXFX6ch6chReverbParametersPreset"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_parameters_preset(
        params: *mut MultiChReverb,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX6ch6chReverbSettingsUpdate
    #[doc(alias = "AXFX6ch6chReverbSettingsUpdate")]
    #[link_name = "AXFX6ch6chReverbSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_settings_update(params: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX6ch6chReverbCallback
    #[doc(alias = "AXFX6ch6chReverbCallback")]
    #[link_name = "AXFX6ch6chReverbCallback"]
    #[deprecated(note = "Use multi_ch_reverb_* instead")]
    pub unsafe fn reverb_6ch_6ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut MultiChReverb,
    );

    /// AXFX6chMultiDelayGetMemSize
    #[doc(alias = "AXFX6chMultiDelayGetMemSize")]
    #[link_name = "AXFX6chMultiDelayGetMemSize"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_6ch_get_mem_size(multi_delay: *mut MultiChDelay) -> i32;

    /// AXFX6chMultiDelayInit
    #[doc(alias = "AXFX6chMultiDelayInit")]
    #[link_name = "AXFX6chMultiDelayInit"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_6ch_init(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX6chMultiDelayShutdown
    #[doc(alias = "AXFX6chMultiDelayShutdown")]
    #[link_name = "AXFX6chMultiDelayShutdown"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_6ch_shutdown(multi_delay: *mut MultiChDelay);

    /// AXFX6chMultiDelaySettingsUpdate
    #[doc(alias = "AXFX6chMultiDelaySettingsUpdate")]
    #[link_name = "AXFX6chMultiDelaySettingsUpdate"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_6ch_settings_update(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX6chMultiDelayCallback
    #[doc(alias = "AXFX6chMultiDelayCallback")]
    #[link_name = "AXFX6chMultiDelayCallback"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_6ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        multi_delay: *mut MultiChDelay,
    );

    /// AXFX2chMultiDelayGetMemSize
    #[doc(alias = "AXFX2chMultiDelayGetMemSize")]
    #[link_name = "AXFX2chMultiDelayGetMemSize"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_2ch_get_mem_size(multi_delay: *mut MultiChDelay) -> i32;

    /// AXFX2chMultiDelayInit
    #[doc(alias = "AXFX2chMultiDelayInit")]
    #[link_name = "AXFX2chMultiDelayInit"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_2ch_init(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX2chMultiDelayShutdown
    #[doc(alias = "AXFX2chMultiDelayShutdown")]
    #[link_name = "AXFX2chMultiDelayShutdown"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_2ch_shutdown(multi_delay: *mut MultiChDelay);

    /// AXFX2chMultiDelaySettingsUpdate
    #[doc(alias = "AXFX2chMultiDelaySettingsUpdate")]
    #[link_name = "AXFX2chMultiDelaySettingsUpdate"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_2ch_settings_update(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX2chMultiDelayCallback
    #[doc(alias = "AXFX2chMultiDelayCallback")]
    #[link_name = "AXFX2chMultiDelayCallback"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_2ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        multi_delay: *mut MultiChDelay,
    );

    /// AXFX4chMultiDelayGetMemSize
    #[doc(alias = "AXFX4chMultiDelayGetMemSize")]
    #[link_name = "AXFX4chMultiDelayGetMemSize"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_4ch_get_mem_size(multi_delay: *mut MultiChDelay) -> i32;

    /// AXFX4chMultiDelayInit
    #[doc(alias = "AXFX4chMultiDelayInit")]
    #[link_name = "AXFX4chMultiDelayInit"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_4ch_init(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX4chMultiDelayShutdown
    #[doc(alias = "AXFX4chMultiDelayShutdown")]
    #[link_name = "AXFX4chMultiDelayShutdown"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_4ch_shutdown(multi_delay: *mut MultiChDelay);

    /// AXFX4chMultiDelaySettingsUpdate
    #[doc(alias = "AXFX4chMultiDelaySettingsUpdate")]
    #[link_name = "AXFX4chMultiDelaySettingsUpdate"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_4ch_settings_update(multi_delay: *mut MultiChDelay) -> ffi::c_bool;

    /// AXFX4chMultiDelayCallback
    #[doc(alias = "AXFX4chMultiDelayCallback")]
    #[link_name = "AXFX4chMultiDelayCallback"]
    #[deprecated(note = "Use multi_ch_delay_* instead")]
    pub unsafe fn multi_delay_4ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        multi_delay: *mut MultiChDelay,
    );

    /// AXFX6chChorusGetMemSize
    #[doc(alias = "AXFX6chChorusGetMemSize")]
    #[link_name = "AXFX6chChorusGetMemSize"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_get_mem_size(chorus: *mut MultiChChorus) -> i32;

    /// AXFX6chChorusInit
    #[doc(alias = "AXFX6chChorusInit")]
    #[link_name = "AXFX6chChorusInit"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_init(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX6chChorusShutdown
    #[doc(alias = "AXFX6chChorusShutdown")]
    #[link_name = "AXFX6chChorusShutdown"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_shutdown(chorus: *mut MultiChChorus);

    /// AXFX6chChorusSettings
    #[doc(alias = "AXFX6chChorusSettings")]
    #[link_name = "AXFX6chChorusSettings"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_settings(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX6chChorusSettingsUpdate
    #[doc(alias = "AXFX6chChorusSettingsUpdate")]
    #[link_name = "AXFX6chChorusSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_settings_update(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX6chChorusCallback
    #[doc(alias = "AXFX6chChorusCallback")]
    #[link_name = "AXFX6chChorusCallback"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_6ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        chorus: *mut MultiChChorus,
    );

    /// AXFX2chChorusGetMemSize
    #[doc(alias = "AXFX2chChorusGetMemSize")]
    #[link_name = "AXFX2chChorusGetMemSize"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_get_mem_size(chorus: *mut MultiChChorus) -> i32;

    /// AXFX2chChorusInit
    #[doc(alias = "AXFX2chChorusInit")]
    #[link_name = "AXFX2chChorusInit"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_init(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX2chChorusShutdown
    #[doc(alias = "AXFX2chChorusShutdown")]
    #[link_name = "AXFX2chChorusShutdown"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_shutdown(chorus: *mut MultiChChorus);

    /// AXFX2chChorusSettings
    #[doc(alias = "AXFX2chChorusSettings")]
    #[link_name = "AXFX2chChorusSettings"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_settings(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX2chChorusSettingsUpdate
    #[doc(alias = "AXFX2chChorusSettingsUpdate")]
    #[link_name = "AXFX2chChorusSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_settings_update(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX2chChorusCallback
    #[doc(alias = "AXFX2chChorusCallback")]
    #[link_name = "AXFX2chChorusCallback"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_2ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        chorus: *mut MultiChChorus,
    );

    /// AXFX4chChorusGetMemSize
    #[doc(alias = "AXFX4chChorusGetMemSize")]
    #[link_name = "AXFX4chChorusGetMemSize"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_get_mem_size(chorus: *mut MultiChChorus) -> i32;

    /// AXFX4chChorusInit
    #[doc(alias = "AXFX4chChorusInit")]
    #[link_name = "AXFX4chChorusInit"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_init(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX4chChorusShutdown
    #[doc(alias = "AXFX4chChorusShutdown")]
    #[link_name = "AXFX4chChorusShutdown"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_shutdown(chorus: *mut MultiChChorus);

    /// AXFX4chChorusSettings
    #[doc(alias = "AXFX4chChorusSettings")]
    #[link_name = "AXFX4chChorusSettings"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_settings(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX4chChorusSettingsUpdate
    #[doc(alias = "AXFX4chChorusSettingsUpdate")]
    #[link_name = "AXFX4chChorusSettingsUpdate"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_settings_update(chorus: *mut MultiChChorus) -> ffi::c_bool;

    /// AXFX4chChorusCallback
    #[doc(alias = "AXFX4chChorusCallback")]
    #[link_name = "AXFX4chChorusCallback"]
    #[deprecated(note = "Use multi_ch_chorus_* instead")]
    pub unsafe fn chorus_4ch_callback(
        buffer_update: *mut BufferUpdate6ch,
        chorus: *mut MultiChChorus,
    );

    /// AXFXReverbSettingsUpdate
    #[doc(alias = "AXFXReverbSettingsUpdate")]
    #[link_name = "AXFXReverbSettingsUpdate"]
    pub unsafe fn reverb_settings_update(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFXReverbSettingsUpdateNoReset
    #[doc(alias = "AXFXReverbSettingsUpdateNoReset")]
    #[link_name = "AXFXReverbSettingsUpdateNoReset"]
    pub unsafe fn reverb_settings_update_no_reset(reverb: *mut MultiChReverb) -> ffi::c_bool;

    /// AXFX2ReverbGetMemSize
    #[doc(alias = "AXFX2ReverbGetMemSize")]
    #[link_name = "AXFX2ReverbGetMemSize"]
    pub unsafe fn reverb2_get_mem_size(mode: ReverbMode, sample_rate: SampleRate) -> i32;

    /// AXFX2ReverbInit
    #[doc(alias = "AXFX2ReverbInit")]
    #[link_name = "AXFX2ReverbInit"]
    pub unsafe fn reverb2_init(
        reverb: *mut Reverb2,
        mode: ReverbMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2ReverbShutdown
    #[doc(alias = "AXFX2ReverbShutdown")]
    #[link_name = "AXFX2ReverbShutdown"]
    pub unsafe fn reverb2_shutdown(reverb: *mut Reverb2);

    /// AXFX2ReverbParametersPreset
    #[doc(alias = "AXFX2ReverbParametersPreset")]
    #[link_name = "AXFX2ReverbParametersPreset"]
    pub unsafe fn reverb2_parameters_preset(
        reverb: *mut Reverb2,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX2ReverbSettingsUpdate
    #[doc(alias = "AXFX2ReverbSettingsUpdate")]
    #[link_name = "AXFX2ReverbSettingsUpdate"]
    pub unsafe fn reverb2_settings_update(reverb: *mut Reverb2) -> ffi::c_bool;

    /// AXFX2ReverbSettingsUpdateNoReset
    #[doc(alias = "AXFX2ReverbSettingsUpdateNoReset")]
    #[link_name = "AXFX2ReverbSettingsUpdateNoReset"]
    pub unsafe fn reverb2_settings_update_no_reset(reverb: *mut Reverb2) -> ffi::c_bool;

    /// AXFX2ReverbCallback
    #[doc(alias = "AXFX2ReverbCallback")]
    #[link_name = "AXFX2ReverbCallback"]
    pub unsafe fn reverb2_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb: *mut Reverb2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2ReverbI3dl2GetMemSize
    #[doc(alias = "AXFX2ReverbI3dl2GetMemSize")]
    #[link_name = "AXFX2ReverbI3dl2GetMemSize"]
    pub unsafe fn reverb2_i3dl2_get_mem_size(mode: ReverbMode, sample_rate: SampleRate) -> i32;

    /// AXFX2ReverbI3dl2Init
    #[doc(alias = "AXFX2ReverbI3dl2Init")]
    #[link_name = "AXFX2ReverbI3dl2Init"]
    pub unsafe fn reverb2_i3dl2_init(
        reverb_i3dl2: *mut Reverb2I3dl2,
        mode: ReverbMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2ReverbI3dl2Shutdown
    #[doc(alias = "AXFX2ReverbI3dl2Shutdown")]
    #[link_name = "AXFX2ReverbI3dl2Shutdown"]
    pub unsafe fn reverb2_i3dl2_shutdown(reverb_i3dl2: *mut Reverb2I3dl2);

    /// AXFX2ReverbI3dl2ParametersPreset
    #[doc(alias = "AXFX2ReverbI3dl2ParametersPreset")]
    #[link_name = "AXFX2ReverbI3dl2ParametersPreset"]
    pub unsafe fn reverb2_i3dl2_parameters_preset(
        reverb_i3dl2: *mut Reverb2I3dl2,
        preset: ReverbPreset,
    ) -> ffi::c_bool;

    /// AXFX2ReverbI3dl2SettingsUpdate
    #[doc(alias = "AXFX2ReverbI3dl2SettingsUpdate")]
    #[link_name = "AXFX2ReverbI3dl2SettingsUpdate"]
    pub unsafe fn reverb2_i3dl2_settings_update(reverb_i3dl2: *mut Reverb2I3dl2) -> ffi::c_bool;

    /// AXFX2ReverbI3dl2SettingsUpdateNoReset
    #[doc(alias = "AXFX2ReverbI3dl2SettingsUpdateNoReset")]
    #[link_name = "AXFX2ReverbI3dl2SettingsUpdateNoReset"]
    pub unsafe fn reverb2_i3dl2_settings_update_no_reset(
        reverb_i3dl2: *mut Reverb2I3dl2,
    ) -> ffi::c_bool;

    /// AXFX2ReverbI3dl2Callback
    #[doc(alias = "AXFX2ReverbI3dl2Callback")]
    #[link_name = "AXFX2ReverbI3dl2Callback"]
    pub unsafe fn reverb2_i3dl2_callback(
        buffer_update: *mut BufferUpdate6ch,
        reverb_i3dl2: *mut Reverb2I3dl2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2DelayGetMemSize
    #[doc(alias = "AXFX2DelayGetMemSize")]
    #[link_name = "AXFX2DelayGetMemSize"]
    pub unsafe fn delay2_get_mem_size(
        delay: *mut Delay2,
        mode: DelayMode,
        sample_rate: SampleRate,
    ) -> i32;

    /// AXFX2DelayInit
    #[doc(alias = "AXFX2DelayInit")]
    #[link_name = "AXFX2DelayInit"]
    pub unsafe fn delay2_init(
        delay: *mut Delay2,
        mode: DelayMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2DelayShutdown
    #[doc(alias = "AXFX2DelayShutdown")]
    #[link_name = "AXFX2DelayShutdown"]
    pub unsafe fn delay2_shutdown(delay: *mut Delay2);

    /// AXFX2DelaySettings
    #[doc(alias = "AXFX2DelaySettings")]
    #[link_name = "AXFX2DelaySettings"]
    pub unsafe fn delay2_settings(delay: *mut Delay2) -> ffi::c_bool;

    /// AXFX2DelaySettingsUpdate
    #[doc(alias = "AXFX2DelaySettingsUpdate")]
    #[link_name = "AXFX2DelaySettingsUpdate"]
    pub unsafe fn delay2_settings_update(delay: *mut Delay2) -> ffi::c_bool;

    /// AXFX2DelaySettingsUpdateNoReset
    #[doc(alias = "AXFX2DelaySettingsUpdateNoReset")]
    #[link_name = "AXFX2DelaySettingsUpdateNoReset"]
    pub unsafe fn delay2_settings_update_no_reset(delay: *mut Delay2) -> ffi::c_bool;

    /// AXFX2DelayCallback
    #[doc(alias = "AXFX2DelayCallback")]
    #[link_name = "AXFX2DelayCallback"]
    pub unsafe fn delay2_callback(
        buffer_update: *mut BufferUpdate6ch,
        delay: *mut Delay2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2ChorusGetMemSize
    #[doc(alias = "AXFX2ChorusGetMemSize")]
    #[link_name = "AXFX2ChorusGetMemSize"]
    pub unsafe fn chorus2_get_mem_size(mode: ChorusMode, sample_rate: SampleRate) -> i32;

    /// AXFX2ChorusInit
    #[doc(alias = "AXFX2ChorusInit")]
    #[link_name = "AXFX2ChorusInit"]
    pub unsafe fn chorus2_init(
        chorus: *mut Chorus2,
        mode: ChorusMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2ChorusShutdown
    #[doc(alias = "AXFX2ChorusShutdown")]
    #[link_name = "AXFX2ChorusShutdown"]
    pub unsafe fn chorus2_shutdown(chorus: *mut Chorus2);

    /// AXFX2ChorusSettingsUpdate
    #[doc(alias = "AXFX2ChorusSettingsUpdate")]
    #[link_name = "AXFX2ChorusSettingsUpdate"]
    pub unsafe fn chorus2_settings_update(chorus: *mut Chorus2) -> ffi::c_bool;

    /// AXFX2ChorusSettingsUpdateNoReset
    #[doc(alias = "AXFX2ChorusSettingsUpdateNoReset")]
    #[link_name = "AXFX2ChorusSettingsUpdateNoReset"]
    pub unsafe fn chorus2_settings_update_no_reset(chorus: *mut Chorus2) -> ffi::c_bool;

    /// AXFX2ChorusCallback
    #[doc(alias = "AXFX2ChorusCallback")]
    #[link_name = "AXFX2ChorusCallback"]
    pub unsafe fn chorus2_callback(
        buffer_update: *mut BufferUpdate6ch,
        chorus: *mut Chorus2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2FlangerGetMemSize
    #[doc(alias = "AXFX2FlangerGetMemSize")]
    #[link_name = "AXFX2FlangerGetMemSize"]
    pub unsafe fn flanger2_get_mem_size(mode: FlangerMode, sample_rate: SampleRate) -> i32;

    /// AXFX2FlangerInit
    #[doc(alias = "AXFX2FlangerInit")]
    #[link_name = "AXFX2FlangerInit"]
    pub unsafe fn flanger2_init(
        flanger: *mut Flanger2,
        mode: FlangerMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2FlangerShutdown
    #[doc(alias = "AXFX2FlangerShutdown")]
    #[link_name = "AXFX2FlangerShutdown"]
    pub unsafe fn flanger2_shutdown(flanger: *mut Flanger2);

    /// AXFX2FlangerSettingsUpdate
    #[doc(alias = "AXFX2FlangerSettingsUpdate")]
    #[link_name = "AXFX2FlangerSettingsUpdate"]
    pub unsafe fn flanger2_settings_update(flanger: *mut Flanger2) -> ffi::c_bool;

    /// AXFX2FlangerSettingsUpdateNoReset
    #[doc(alias = "AXFX2FlangerSettingsUpdateNoReset")]
    #[link_name = "AXFX2FlangerSettingsUpdateNoReset"]
    pub unsafe fn flanger2_settings_update_no_reset(flanger: *mut Flanger2) -> ffi::c_bool;

    /// AXFX2FlangerCallback
    #[doc(alias = "AXFX2FlangerCallback")]
    #[link_name = "AXFX2FlangerCallback"]
    pub unsafe fn flanger2_callback(
        buffer_update: *mut BufferUpdate6ch,
        flanger: *mut Flanger2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2OverdriveGetMemSize
    #[doc(alias = "AXFX2OverdriveGetMemSize")]
    #[link_name = "AXFX2OverdriveGetMemSize"]
    pub unsafe fn overdrive2_get_mem_size(mode: OverdriveMode, sample_rate: SampleRate) -> i32;

    /// AXFX2OverdriveInit
    #[doc(alias = "AXFX2OverdriveInit")]
    #[link_name = "AXFX2OverdriveInit"]
    pub unsafe fn overdrive2_init(
        overdrive: *mut Overdrive2,
        mode: OverdriveMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2OverdriveSettingsUpdate
    #[doc(alias = "AXFX2OverdriveSettingsUpdate")]
    #[link_name = "AXFX2OverdriveSettingsUpdate"]
    pub unsafe fn overdrive2_settings_update(overdrive: *mut Overdrive2) -> ffi::c_bool;

    /// AXFX2OverdriveShutdown
    #[doc(alias = "AXFX2OverdriveShutdown")]
    #[link_name = "AXFX2OverdriveShutdown"]
    pub unsafe fn overdrive2_shutdown(overdrive: *mut Overdrive2);

    /// AXFX2OverdriveCallback
    #[doc(alias = "AXFX2OverdriveCallback")]
    #[link_name = "AXFX2OverdriveCallback"]
    pub unsafe fn overdrive2_callback(
        buffer_update: *mut BufferUpdate6ch,
        overdrive: *mut Overdrive2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2CompressorGetMemSize
    #[doc(alias = "AXFX2CompressorGetMemSize")]
    #[link_name = "AXFX2CompressorGetMemSize"]
    pub unsafe fn compressor2_get_mem_size(mode: CompressorMode, sample_rate: SampleRate) -> i32;

    /// AXFX2CompressorInit
    #[doc(alias = "AXFX2CompressorInit")]
    #[link_name = "AXFX2CompressorInit"]
    pub unsafe fn compressor2_init(
        compressor: *mut Compressor2,
        mode: CompressorMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2CompressorSettingsUpdate
    #[doc(alias = "AXFX2CompressorSettingsUpdate")]
    #[link_name = "AXFX2CompressorSettingsUpdate"]
    pub unsafe fn compressor2_settings_update(compressor: *mut Compressor2) -> ffi::c_bool;

    /// AXFX2CompressorShutdown
    #[doc(alias = "AXFX2CompressorShutdown")]
    #[link_name = "AXFX2CompressorShutdown"]
    pub unsafe fn compressor2_shutdown(compressor: *mut Compressor2);

    /// AXFX2CompressorCallback
    #[doc(alias = "AXFX2CompressorCallback")]
    #[link_name = "AXFX2CompressorCallback"]
    pub unsafe fn compressor2_callback(
        buffer_update: *mut BufferUpdate6ch,
        compressor: *mut Compressor2,
        info: *mut AuxCallbackInfo,
    );

    /// AXFX2PitchshiftGetMemSize
    #[doc(alias = "AXFX2PitchshiftGetMemSize")]
    #[link_name = "AXFX2PitchshiftGetMemSize"]
    pub unsafe fn pitchshift2_get_mem_size(mode: PitchshiftMode, sample_rate: SampleRate) -> i32;

    /// AXFX2PitchshiftInit
    #[doc(alias = "AXFX2PitchshiftInit")]
    #[link_name = "AXFX2PitchshiftInit"]
    pub unsafe fn pitchshift2_init(
        pitchshift: *mut Pitchshift2,
        mode: PitchshiftMode,
        sample_rate: SampleRate,
    ) -> ffi::c_bool;

    /// AXFX2PitchshiftSettingsUpdate
    #[doc(alias = "AXFX2PitchshiftSettingsUpdate")]
    #[link_name = "AXFX2PitchshiftSettingsUpdate"]
    pub unsafe fn pitchshift2_settings_update(pitchshift: *mut Pitchshift2) -> ffi::c_bool;

    /// AXFX2PitchshiftSettingsUpdateNoReset
    #[doc(alias = "AXFX2PitchshiftSettingsUpdateNoReset")]
    #[link_name = "AXFX2PitchshiftSettingsUpdateNoReset"]
    pub unsafe fn pitchshift2_settings_update_no_reset(pitchshift: *mut Pitchshift2)
    -> ffi::c_bool;

    /// AXFX2PitchshiftShutdown
    #[doc(alias = "AXFX2PitchshiftShutdown")]
    #[link_name = "AXFX2PitchshiftShutdown"]
    pub unsafe fn pitchshift2_shutdown(pitchshift: *mut Pitchshift2);

    /// AXFX2PitchshiftCallback
    #[doc(alias = "AXFX2PitchshiftCallback")]
    #[link_name = "AXFX2PitchshiftCallback"]
    pub unsafe fn pitchshift2_callback(
        buffer_update: *mut BufferUpdate6ch,
        pitchshift: *mut Pitchshift2,
        info: *mut AuxCallbackInfo,
    );
}
