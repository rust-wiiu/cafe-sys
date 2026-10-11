//! AX articulation
//!
//! Applies articulators (3D position, panning, pitch, volume, envelopes, LFO modulation, filters) to AX voices each audio frame.

use crate::ffi;
use crate::sndcore2::voice::Voice;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtType(pub u32);

impl ArtType {
    pub const THREE_D: Self = Self(1);
    pub const PANNING: Self = Self(2);
    pub const ITD: Self = Self(3);
    pub const SRC_TYPE: Self = Self(4);
    pub const PITCH: Self = Self(5);
    pub const PITCH_ENV: Self = Self(6);
    pub const PITCH_MOD: Self = Self(7);
    pub const VOLUME: Self = Self(8);
    pub const AUX_A_VOLUME: Self = Self(9);
    pub const AUX_B_VOLUME: Self = Self(10);
    pub const AUX_C_VOLUME: Self = Self(11);
    pub const VOLUME_ENV: Self = Self(12);
    pub const AUX_A_VOLUME_ENV: Self = Self(13);
    pub const AUX_B_VOLUME_ENV: Self = Self(14);
    pub const AUX_C_VOLUME_ENV: Self = Self(15);
    pub const VOLUME_MOD: Self = Self(16);
    pub const AUX_A_VOLUME_MOD: Self = Self(17);
    pub const AUX_B_VOLUME_MOD: Self = Self(18);
    pub const AUX_C_VOLUME_MOD: Self = Self(19);
    pub const LPF: Self = Self(20);
    pub const FADER: Self = Self(21);
    pub const RMT: Self = Self(22);
    pub const RMT_FADER: Self = Self(24);
    pub const RMT_AUX_VOLUME: Self = Self(25);
    pub const DRC: Self = Self(26);
}

/// Header of every articulator, maintained by the library.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Art {
    pub next: *mut Art,
    pub kind: ArtType,
}

/// LFO of the modulation articulators, see [init_lfo].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lfo {
    pub lfo: *mut f32,
    pub length: u32,
    pub delta: f32,
    pub sample_index: u32,
    pub counter: f32,
    pub sample1: f32,
    pub sample: f32,
    pub output: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Art3d {
    pub art: Art,
    /// Horizontal angle in radians.
    pub h_angle: f32,
    /// Vertical angle in radians.
    pub v_angle: f32,
    pub dist: f32,
    /// Closing speed towards the camera, for doppler.
    pub closing_speed: f32,
    /// Set to true after changing the fields above.
    pub update: ffi::c_bool,
    pub pan: u8,
    pub span: u8,
    pub src: u8,
    pub itd_left: u16,
    pub itd_right: u16,
    pub pitch: f32,
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Panning {
    pub art: Art,
    pub pan: u8,
    pub span: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SrcType {
    pub art: Art,
    /// A [SrcType][crate::sndcore2::voice::SrcType] value.
    pub src: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Itd {
    pub art: Art,
    pub itd_left: u16,
    pub itd_right: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    pub art: Art,
    /// 0x10000 is 1 cent.
    pub cents: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PitchEnv {
    pub art: Art,
    /// Per audio frame, 0x10000 is 1 cent.
    pub delta: i32,
    /// 0x10000 is 1 cent.
    pub target: i32,
    /// Current output.
    pub cents: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PitchMod {
    pub art: Art,
    pub lfo: Lfo,
    /// 0x10000 is 1 cent.
    pub cents: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Volume {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxAVolume {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxBVolume {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxCVolume {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VolumeEnv {
    pub art: Art,
    /// Per audio frame, 0x10000 is 0.1 dB.
    pub delta: i32,
    /// 0x10000 is 0.1 dB.
    pub target: i32,
    /// Current output.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxAVolumeEnv {
    pub art: Art,
    /// Per audio frame, 0x10000 is 0.1 dB.
    pub delta: i32,
    /// 0x10000 is 0.1 dB.
    pub target: i32,
    /// Current output.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxBVolumeEnv {
    pub art: Art,
    /// Per audio frame, 0x10000 is 0.1 dB.
    pub delta: i32,
    /// 0x10000 is 0.1 dB.
    pub target: i32,
    /// Current output.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxCVolumeEnv {
    pub art: Art,
    /// Per audio frame, 0x10000 is 0.1 dB.
    pub delta: i32,
    /// 0x10000 is 0.1 dB.
    pub target: i32,
    /// Current output.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeMod {
    pub art: Art,
    pub lfo: Lfo,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AuxAVolumeMod {
    pub art: Art,
    pub lfo: Lfo,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AuxBVolumeMod {
    pub art: Art,
    pub lfo: Lfo,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AuxCVolumeMod {
    pub art: Art,
    pub lfo: Lfo,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LpfFrequency(pub u32);

impl LpfFrequency {
    pub const HZ_16000: Self = Self(0);
    pub const HZ_12800: Self = Self(1);
    pub const HZ_10240: Self = Self(2);
    pub const HZ_8000: Self = Self(3);
    pub const HZ_6400: Self = Self(4);
    pub const HZ_5120: Self = Self(5);
    pub const HZ_4000: Self = Self(6);
    pub const HZ_3200: Self = Self(7);
    pub const HZ_2560: Self = Self(8);
    pub const HZ_2000: Self = Self(9);
    pub const HZ_1600: Self = Self(10);
    pub const HZ_1280: Self = Self(11);
    pub const HZ_1000: Self = Self(12);
    pub const HZ_800: Self = Self(13);
    pub const HZ_640: Self = Self(14);
    pub const HZ_500: Self = Self(15);
    pub const HZ_400: Self = Self(16);
    pub const HZ_320: Self = Self(17);
    pub const HZ_256: Self = Self(18);
    pub const HZ_200: Self = Self(19);
    pub const HZ_160: Self = Self(20);
    pub const HZ_128: Self = Self(21);
    pub const HZ_100: Self = Self(22);
    pub const HZ_80: Self = Self(23);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lpf {
    pub art: Art,
    pub init_lpf: u32,
    /// Roll-off frequency.
    pub frequency: LpfFrequency,
    /// Set to true after changing `frequency`.
    pub update: ffi::c_bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fader {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: i32,
}

/// Enables the controller speaker output.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rmt {
    pub art: Art,
    pub on: u16,
}

/// Enables the DRC output.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Drc {
    pub art: Art,
    pub on: u16,
}

/// Fader of each controller speaker.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtFader {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: [i32; 4],
}

/// Aux send level of each controller speaker.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RmtAuxVolume {
    pub art: Art,
    /// 0x10000 is 0.1 dB.
    pub attenuation: [i32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sound {
    pub next: *mut Sound,
    pub prev: *mut Sound,
    pub voice: *mut Voice,
    pub sample_rate: f32,
    pub articulators: *mut Art,
}

unsafe extern "C" {
    /// AXARTInit
    #[doc(alias = "AXARTInit")]
    #[link_name = "AXARTInit"]
    pub unsafe fn init();

    /// AXARTQuit
    #[doc(alias = "AXARTQuit")]
    #[link_name = "AXARTQuit"]
    pub unsafe fn quit();

    /// AXARTServiceSounds
    ///
    /// Runs the articulators of all added sounds for one audio frame.
    #[doc(alias = "AXARTServiceSounds")]
    #[link_name = "AXARTServiceSounds"]
    pub unsafe fn service_sounds();

    /// AXARTInitSound
    ///
    /// `sample_rate` is in Hz.
    #[doc(alias = "AXARTInitSound")]
    #[link_name = "AXARTInitSound"]
    pub unsafe fn init_sound(sound: *mut Sound, voice: *mut Voice, sample_rate: u32);

    /// AXARTAddSound
    #[doc(alias = "AXARTAddSound")]
    #[link_name = "AXARTAddSound"]
    pub unsafe fn add_sound(sound: *mut Sound);

    /// AXARTRemoveSound
    #[doc(alias = "AXARTRemoveSound")]
    #[link_name = "AXARTRemoveSound"]
    pub unsafe fn remove_sound(sound: *mut Sound);

    /// AXARTSet3DDistanceScale
    #[doc(alias = "AXARTSet3DDistanceScale")]
    #[link_name = "AXARTSet3DDistanceScale"]
    pub unsafe fn set_3d_distance_scale(scale: f32);

    /// AXARTSet3DDopplerScale
    ///
    /// The doppler pitch is `closing_speed / scale`.
    #[doc(alias = "AXARTSet3DDopplerScale")]
    #[link_name = "AXARTSet3DDopplerScale"]
    pub unsafe fn set_3d_doppler_scale(scale: f32);

    /// AXARTAddArticulator
    ///
    /// `articulator` points to the [Art] header of an initialized articulator.
    #[doc(alias = "AXARTAddArticulator")]
    #[link_name = "AXARTAddArticulator"]
    pub unsafe fn add_articulator(sound: *mut Sound, articulator: *mut Art);

    /// AXARTRemoveArticulator
    ///
    /// Returns false if `articulator` was not added to `sound`.
    #[doc(alias = "AXARTRemoveArticulator")]
    #[link_name = "AXARTRemoveArticulator"]
    pub unsafe fn remove_articulator(sound: *mut Sound, articulator: *mut Art) -> ffi::c_bool;

    /// AXARTInitLfo
    ///
    /// `samples` is one period of `length` samples.
    #[doc(alias = "AXARTInitLfo")]
    #[link_name = "AXARTInitLfo"]
    pub unsafe fn init_lfo(lfo: *mut Lfo, samples: *mut f32, length: u32, delta: f32);

    /// AXARTInitArt3D
    #[doc(alias = "AXARTInitArt3D")]
    #[link_name = "AXARTInitArt3D"]
    pub unsafe fn init_art_3d(articulator: *mut Art3d);

    /// AXARTInitArtPanning
    #[doc(alias = "AXARTInitArtPanning")]
    #[link_name = "AXARTInitArtPanning"]
    pub unsafe fn init_art_panning(articulator: *mut Panning);

    /// AXARTInitArtItd
    #[doc(alias = "AXARTInitArtItd")]
    #[link_name = "AXARTInitArtItd"]
    pub unsafe fn init_art_itd(articulator: *mut Itd);

    /// AXARTInitArtSrctype
    #[doc(alias = "AXARTInitArtSrctype")]
    #[link_name = "AXARTInitArtSrctype"]
    pub unsafe fn init_art_src_type(articulator: *mut SrcType);

    /// AXARTInitArtPitch
    #[doc(alias = "AXARTInitArtPitch")]
    #[link_name = "AXARTInitArtPitch"]
    pub unsafe fn init_art_pitch(articulator: *mut Pitch);

    /// AXARTInitArtPitchEnv
    #[doc(alias = "AXARTInitArtPitchEnv")]
    #[link_name = "AXARTInitArtPitchEnv"]
    pub unsafe fn init_art_pitch_env(articulator: *mut PitchEnv);

    /// AXARTInitArtPitchMod
    #[doc(alias = "AXARTInitArtPitchMod")]
    #[link_name = "AXARTInitArtPitchMod"]
    pub unsafe fn init_art_pitch_mod(articulator: *mut PitchMod);

    /// AXARTInitArtVolume
    #[doc(alias = "AXARTInitArtVolume")]
    #[link_name = "AXARTInitArtVolume"]
    pub unsafe fn init_art_volume(articulator: *mut Volume);

    /// AXARTInitArtAuxAVolume
    #[doc(alias = "AXARTInitArtAuxAVolume")]
    #[link_name = "AXARTInitArtAuxAVolume"]
    pub unsafe fn init_art_aux_a_volume(articulator: *mut AuxAVolume);

    /// AXARTInitArtAuxBVolume
    #[doc(alias = "AXARTInitArtAuxBVolume")]
    #[link_name = "AXARTInitArtAuxBVolume"]
    pub unsafe fn init_art_aux_b_volume(articulator: *mut AuxBVolume);

    /// AXARTInitArtAuxCVolume
    #[doc(alias = "AXARTInitArtAuxCVolume")]
    #[link_name = "AXARTInitArtAuxCVolume"]
    pub unsafe fn init_art_aux_c_volume(articulator: *mut AuxCVolume);

    /// AXARTInitArtVolumeEnv
    #[doc(alias = "AXARTInitArtVolumeEnv")]
    #[link_name = "AXARTInitArtVolumeEnv"]
    pub unsafe fn init_art_volume_env(articulator: *mut VolumeEnv);

    /// AXARTInitArtAuxAVolumeEnv
    #[doc(alias = "AXARTInitArtAuxAVolumeEnv")]
    #[link_name = "AXARTInitArtAuxAVolumeEnv"]
    pub unsafe fn init_art_aux_a_volume_env(articulator: *mut AuxAVolumeEnv);

    /// AXARTInitArtAuxBVolumeEnv
    #[doc(alias = "AXARTInitArtAuxBVolumeEnv")]
    #[link_name = "AXARTInitArtAuxBVolumeEnv"]
    pub unsafe fn init_art_aux_b_volume_env(articulator: *mut AuxBVolumeEnv);

    /// AXARTInitArtAuxCVolumeEnv
    #[doc(alias = "AXARTInitArtAuxCVolumeEnv")]
    #[link_name = "AXARTInitArtAuxCVolumeEnv"]
    pub unsafe fn init_art_aux_c_volume_env(articulator: *mut AuxCVolumeEnv);

    /// AXARTInitArtVolumeMod
    #[doc(alias = "AXARTInitArtVolumeMod")]
    #[link_name = "AXARTInitArtVolumeMod"]
    pub unsafe fn init_art_volume_mod(articulator: *mut VolumeMod);

    /// AXARTInitArtAuxAVolumeMod
    #[doc(alias = "AXARTInitArtAuxAVolumeMod")]
    #[link_name = "AXARTInitArtAuxAVolumeMod"]
    pub unsafe fn init_art_aux_a_volume_mod(articulator: *mut AuxAVolumeMod);

    /// AXARTInitArtAuxBVolumeMod
    #[doc(alias = "AXARTInitArtAuxBVolumeMod")]
    #[link_name = "AXARTInitArtAuxBVolumeMod"]
    pub unsafe fn init_art_aux_b_volume_mod(articulator: *mut AuxBVolumeMod);

    /// AXARTInitArtAuxCVolumeMod
    #[doc(alias = "AXARTInitArtAuxCVolumeMod")]
    #[link_name = "AXARTInitArtAuxCVolumeMod"]
    pub unsafe fn init_art_aux_c_volume_mod(articulator: *mut AuxCVolumeMod);

    /// AXARTInitArtLpf
    #[doc(alias = "AXARTInitArtLpf")]
    #[link_name = "AXARTInitArtLpf"]
    pub unsafe fn init_art_lpf(articulator: *mut Lpf);

    /// AXARTInitArtFader
    #[doc(alias = "AXARTInitArtFader")]
    #[link_name = "AXARTInitArtFader"]
    pub unsafe fn init_art_fader(articulator: *mut Fader);

    /// AXARTInitArtDRC
    #[doc(alias = "AXARTInitArtDRC")]
    #[link_name = "AXARTInitArtDRC"]
    pub unsafe fn init_art_drc(articulator: *mut Drc);

    /// AXARTInitArtRmt
    #[doc(alias = "AXARTInitArtRmt")]
    #[link_name = "AXARTInitArtRmt"]
    pub unsafe fn init_art_rmt(articulator: *mut Rmt);

    /// AXARTInitArtRmtFader
    #[doc(alias = "AXARTInitArtRmtFader")]
    #[link_name = "AXARTInitArtRmtFader"]
    pub unsafe fn init_art_rmt_fader(articulator: *mut RmtFader);

    /// AXARTInitArtRmtAuxVolume
    #[doc(alias = "AXARTInitArtRmtAuxVolume")]
    #[link_name = "AXARTInitArtRmtAuxVolume"]
    pub unsafe fn init_art_rmt_aux_volume(articulator: *mut RmtAuxVolume);
}
