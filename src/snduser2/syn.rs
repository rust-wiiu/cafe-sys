//! MIDI synthesizer

use crate::ffi;
use crate::sndcore2::voice as ax;

/// SYN_INPUT_BUFFER_SIZE
#[doc(alias = "SYN_INPUT_BUFFER_SIZE")]
pub const INPUT_BUFFER_SIZE: usize = 256;

/// WTART
///
/// Translated DLS articulation block.
#[doc(alias = "WTART")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtArt {
    pub lfo_freq: i32,
    pub lfo_delay: i32,
    pub lfo_atten: i32,
    pub lfo_pitch: i32,
    pub lfo_mod2_atten: i32,
    pub lfo_mod2_pitch: i32,
    pub eg1_attack: i32,
    pub eg1_decay: i32,
    pub eg1_sustain: i32,
    pub eg1_release: i32,
    pub eg1_vel2_attack: i32,
    pub eg1_key2_decay: i32,
    pub eg2_attack: i32,
    pub eg2_decay: i32,
    pub eg2_sustain: i32,
    pub eg2_release: i32,
    pub eg2_vel2_attack: i32,
    pub eg2_key2_decay: i32,
    pub eg2_pitch: i32,
    pub pan: i32,
}

/// WTREGION
#[doc(alias = "WTREGION")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtRegion {
    pub unity_note: u8,
    pub key_group: u8,
    pub fine_tune: i16,
    pub attn: i32,
    pub loop_start: u32,
    pub loop_length: u32,
    pub articulation_index: u32,
    pub sample_index: u32,
}

/// WT_FORMAT
#[doc(alias = "WT_FORMAT")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtFormat(pub u16);

impl WtFormat {
    pub const ADPCM: Self = Self(0);
    pub const PCM16: Self = Self(1);
    pub const PCM8: Self = Self(2);
}

/// WTSAMPLE
#[doc(alias = "WTSAMPLE")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtSample {
    pub format: WtFormat,
    /// In Hz.
    pub sample_rate: u16,
    /// In samples, from the start of the sample file.
    pub offset: u32,
    /// In samples.
    pub length: u32,
    pub adpcm_index: u16,
}

/// WTADPCM
#[doc(alias = "WTADPCM")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtAdpcm {
    pub coefficients: [i16; 16],
    pub gain: u16,
    pub pred_scale: u16,
    pub yn1: i16,
    pub yn2: i16,
    pub loop_pred_scale: u16,
    pub loop_yn1: i16,
    pub loop_yn2: i16,
}

/// WTINST
///
/// Instrument, region index per key.
#[doc(alias = "WTINST")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtInst {
    pub key_region: [u16; 128],
}

/// WTFILEHEADER
///
/// Offsets are from the start of the file.
#[doc(alias = "WTFILEHEADER")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WtFileHeader {
    pub offset_percussive_inst: u32,
    pub offset_melodic_inst: u32,
    pub offset_regions: u32,
    pub offset_articulations: u32,
    pub offset_samples: u32,
    pub offset_adpcm_context: u32,
}

/// SYNCallback
#[doc(alias = "SYNCallback")]
pub type Callback =
    Option<unsafe extern "C" fn(voice: *mut ax::Voice, synth: *mut Synth, midi_channel: u8)>;

/// SYNSYNTH
#[doc(alias = "SYNSYNTH")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Synth {
    pub next: *mut Synth,
    pub percussive_inst: *mut WtInst,
    pub melodic_inst: *mut WtInst,
    pub region: *mut WtRegion,
    pub art: *mut WtArt,
    pub sample: *mut WtSample,
    pub adpcm: *mut WtAdpcm,
    pub samples: *mut u8,
    pub priority_voice_alloc: u32,
    pub priority_note_on: u32,
    pub priority_note_release: u32,
    /// Instrument per MIDI channel.
    pub inst: [*mut WtInst; 16],
    /// In 0.1 dB.
    pub master_volume: i32,
    /// In 0.1 dB.
    pub drc_master_volume: i32,
    pub renderer: ax::Renderer,
    pub controller: [[u8; 128]; 16],
    pub rpn: [u8; 16],
    pub data_entry: [i16; 16],
    pub pw_max_cents: [i32; 16],
    pub pw_cents: [i32; 16],
    pub vol_attn: [i32; 16],
    pub exp_attn: [i32; 16],
    pub aux_a_attn: [i32; 16],
    pub aux_b_attn: [i32; 16],
    pub input: [[u8; 3]; INPUT_BUFFER_SIZE],
    pub input_position: *mut u8,
    pub input_counter: u32,
    pub notes: u32,
    pub key_group: [[*mut ffi::c_void; 16]; 16],
    pub voice: [[*mut ffi::c_void; 128]; 16],
    pub init_callback: Callback,
    pub update_callback: Callback,
}

/// SYN_VOICE_TYPE
#[doc(alias = "SYN_VOICE_TYPE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoiceType(pub u32);

impl VoiceType {
    pub const ONESHOT: Self = Self(0);
    pub const LOOPED: Self = Self(1);
}

/// SYN_VOICE_STATE
#[doc(alias = "SYN_VOICE_STATE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoiceState(pub u32);

impl VoiceState {
    pub const ATTACK: Self = Self(0);
    pub const DECAY: Self = Self(1);
    pub const SUSTAIN: Self = Self(2);
    pub const RELEASE: Self = Self(3);
    pub const DONE: Self = Self(4);
}

/// SYNVOICE
///
/// [init_specify_mem] needs one per AX voice.
#[doc(alias = "SYNVOICE")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Voice {
    pub next: *mut Voice,
    pub ax_voice: *mut ax::Voice,
    pub synth: *mut Synth,
    pub midi_channel: u8,
    pub key_num: u8,
    pub key_vel: u8,
    pub pan: u8,
    pub key_group: u8,
    pub region: *mut WtRegion,
    pub art: *mut WtArt,
    pub sample: *mut WtSample,
    pub adpcm: *mut WtAdpcm,
    pub hold: u32,
    pub kind: VoiceType,
    pub src_ratio: f32,
    pub cents: i32,
    pub attn: i32,
    pub lfo_state: i32,
    pub lfo_attn: i32,
    pub lfo_cents: i32,
    pub lfo_freq: i32,
    pub lfo_delay: i32,
    pub lfo_attn_max: i32,
    pub lfo_cents_max: i32,
    pub lfo_mod_attn: i32,
    pub lfo_mod_cents: i32,
    pub ve_state: VoiceState,
    pub ve_attn: i32,
    pub ve_attack: i32,
    pub ve_attack_delta: i32,
    pub ve_decay: i32,
    pub ve_sustain: i32,
    pub ve_release: i32,
    pub pe_state: VoiceState,
    pub pe_cents: i32,
    pub pe_attack: i32,
    pub pe_decay: i32,
    pub pe_sustain: i32,
    pub pe_release: i32,
    pub pe_pitch: i32,
}

const _: () = {
    assert!(size_of::<WtArt>() == 0x50);
    assert!(size_of::<WtRegion>() == 0x18);
    assert!(size_of::<WtSample>() == 0x10);
    assert!(size_of::<WtAdpcm>() == 0x2E);
    assert!(size_of::<WtInst>() == 0x100);
    assert!(size_of::<WtFileHeader>() == 0x18);
};

unsafe extern "C" {
    /// SYNInit
    #[doc(alias = "SYNInit")]
    #[link_name = "SYNInit"]
    pub unsafe fn init();

    /// SYNInitSpecifyMem
    ///
    /// Same as [init], but uses `mem` for the [Voice]s, one per AX voice.
    #[doc(alias = "SYNInitSpecifyMem")]
    #[link_name = "SYNInitSpecifyMem"]
    pub unsafe fn init_specify_mem(mem: *mut ffi::c_void);

    /// SYNQuit
    #[doc(alias = "SYNQuit")]
    #[link_name = "SYNQuit"]
    pub unsafe fn quit();

    /// SYNRunAudioFrame
    ///
    /// Call from the AX frame callback.
    #[doc(alias = "SYNRunAudioFrame")]
    #[link_name = "SYNRunAudioFrame"]
    pub unsafe fn run_audio_frame();

    /// SYNInitSynth
    ///
    /// `wavetable` and `samples` point to the loaded wavetable and sample files. `zero_buffer` is unused. Priorities are 1-31.
    #[doc(alias = "SYNInitSynth")]
    #[link_name = "SYNInitSynth"]
    pub unsafe fn init_synth(
        synth: *mut Synth,
        wavetable: *mut u8,
        samples: *mut u8,
        zero_buffer: *mut u8,
        priority_voice_alloc: u32,
        priority_note_on: u32,
        priority_note_release: u32,
    );

    /// SYNQuitSynth
    #[doc(alias = "SYNQuitSynth")]
    #[link_name = "SYNQuitSynth"]
    pub unsafe fn quit_synth(synth: *mut Synth);

    /// SYNMidiInput
    ///
    /// Sends one MIDI command. Call from the AX frame callback.
    #[doc(alias = "SYNMidiInput")]
    #[link_name = "SYNMidiInput"]
    pub unsafe fn midi_input(synth: *mut Synth, input: *mut u8);

    /// SYNGetMidiController
    #[doc(alias = "SYNGetMidiController")]
    #[link_name = "SYNGetMidiController"]
    pub unsafe fn get_midi_controller(synth: *mut Synth, midi_channel: u8, function: u8) -> u8;

    /// SYNSetMasterVolume
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "SYNSetMasterVolume")]
    #[link_name = "SYNSetMasterVolume"]
    pub unsafe fn set_master_volume(synth: *mut Synth, db: i32);

    /// SYNGetMasterVolume
    #[doc(alias = "SYNGetMasterVolume")]
    #[link_name = "SYNGetMasterVolume"]
    pub unsafe fn get_master_volume(synth: *mut Synth) -> i32;

    /// SYNDRCSetMasterVolume
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "SYNDRCSetMasterVolume", alias = "SYNMOVSetMasterVolume")]
    #[link_name = "SYNDRCSetMasterVolume"]
    pub unsafe fn drc_set_master_volume(synth: *mut Synth, db: i32);

    /// SYNDRCGetMasterVolume
    #[doc(alias = "SYNDRCGetMasterVolume", alias = "SYNMOVGetMasterVolume")]
    #[link_name = "SYNDRCGetMasterVolume"]
    pub unsafe fn drc_get_master_volume(synth: *mut Synth) -> i32;

    /// SYNGetActiveNotes
    #[doc(alias = "SYNGetActiveNotes")]
    #[link_name = "SYNGetActiveNotes"]
    pub unsafe fn get_active_notes(synth: *mut Synth) -> u32;

    /// SYNSetMixerSelect
    #[doc(alias = "SYNSetMixerSelect")]
    #[link_name = "SYNSetMixerSelect"]
    pub unsafe fn set_renderer(synth: *mut Synth, renderer: ax::Renderer);

    /// SYNGetMixerSelect
    #[doc(alias = "SYNGetMixerSelect")]
    #[link_name = "SYNGetMixerSelect"]
    pub unsafe fn get_renderer(synth: *mut Synth) -> ax::Renderer;

    /// SYNSetInitCallback
    ///
    /// Called at note-on. Returns the previous callback.
    #[doc(alias = "SYNSetInitCallback")]
    #[link_name = "SYNSetInitCallback"]
    pub unsafe fn set_init_callback(synth: *mut Synth, callback: Callback) -> Callback;

    /// SYNSetUpdateCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "SYNSetUpdateCallback")]
    #[link_name = "SYNSetUpdateCallback"]
    pub unsafe fn set_update_callback(synth: *mut Synth, callback: Callback) -> Callback;
}
