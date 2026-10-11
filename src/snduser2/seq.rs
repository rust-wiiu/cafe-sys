//! MIDI sequencer

use super::syn::Synth;
use crate::sndcore2::voice::Renderer;

/// SEQ_MAX_TRACKS
#[doc(alias = "SEQ_MAX_TRACKS")]
pub const MAX_TRACKS: usize = 64;
/// SEQ_ALL_TRACKS
#[doc(alias = "SEQ_ALL_TRACKS")]
pub const ALL_TRACKS: u32 = 0xFFFF_FFFF;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(pub u32);

impl State {
    pub const STOP: Self = Self(0);
    pub const RUN: Self = Self(1);
    pub const RUN_LOOPED: Self = Self(2);
    pub const PAUSE: Self = Self(3);
}

/// Called on a registered MIDI controller event.
pub type ControllerCallback = Option<unsafe extern "C" fn(track: *mut Track, controller: u8)>;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Track {
    pub sequence: *mut Sequence,
    pub start: *mut u8,
    pub end: *mut u8,
    pub current: *mut u8,
    /// MIDI running status.
    pub status: u8,
    pub beats_per_sec: f32,
    pub default_ticks_per_frame: u32,
    pub ticks_per_frame: u32,
    /// Ticks until the next event.
    pub delay: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Sequence {
    pub next: *mut Sequence,
    pub state: State,
    pub num_tracks: u16,
    /// MIDI file time format.
    pub time_format: i16,
    pub tracks_running: u32,
    pub end: u32,
    pub synth: Synth,
    pub callback: [ControllerCallback; 128],
    pub track: [Track; MAX_TRACKS],
}

unsafe extern "C" {
    /// SEQInit
    #[doc(alias = "SEQInit")]
    #[link_name = "SEQInit"]
    pub unsafe fn init();

    /// SEQQuit
    #[doc(alias = "SEQQuit")]
    #[link_name = "SEQQuit"]
    pub unsafe fn quit();

    /// SEQRunAudioFrame
    #[doc(alias = "SEQRunAudioFrame")]
    #[link_name = "SEQRunAudioFrame"]
    pub unsafe fn run_audio_frame();

    /// SEQAddSequence
    ///
    /// Initializes `sequence`, including its synth, and adds it to the sequence list. `zero_buffer` is unused.
    #[doc(alias = "SEQAddSequence")]
    #[link_name = "SEQAddSequence"]
    pub unsafe fn add_sequence(
        sequence: *mut Sequence,
        midi_stream: *mut u8,
        wavetable: *mut u8,
        samples: *mut u8,
        zero_buffer: *mut u8,
        priority_voice_alloc: u32,
        priority_note_on: u32,
        priority_note_release: u32,
    );

    /// SEQRemoveSequence
    #[doc(alias = "SEQRemoveSequence")]
    #[link_name = "SEQRemoveSequence"]
    pub unsafe fn remove_sequence(sequence: *mut Sequence);

    /// SEQRegisterControllerCallback
    #[doc(alias = "SEQRegisterControllerCallback")]
    #[link_name = "SEQRegisterControllerCallback"]
    pub unsafe fn register_controller_callback(
        sequence: *mut Sequence,
        controller: u8,
        callback: ControllerCallback,
    );

    /// SEQSetState
    #[doc(alias = "SEQSetState")]
    #[link_name = "SEQSetState"]
    pub unsafe fn set_state(sequence: *mut Sequence, state: State);

    /// SEQGetState
    #[doc(alias = "SEQGetState")]
    #[link_name = "SEQGetState"]
    pub unsafe fn get_state(sequence: *mut Sequence) -> State;

    /// SEQSetTempo
    ///
    /// `tempo` is in BPM. `track` may be [ALL_TRACKS].
    #[doc(alias = "SEQSetTempo")]
    #[link_name = "SEQSetTempo"]
    pub unsafe fn set_tempo(sequence: *mut Sequence, track: u32, tempo: f32);

    /// SEQGetTempo
    #[doc(alias = "SEQGetTempo")]
    #[link_name = "SEQGetTempo"]
    pub unsafe fn get_tempo(sequence: *mut Sequence, track: u32) -> f32;

    /// SEQSetVolume
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "SEQSetVolume")]
    #[link_name = "SEQSetVolume"]
    pub unsafe fn set_volume(sequence: *mut Sequence, db: i32);

    /// SEQGetVolume
    #[doc(alias = "SEQGetVolume")]
    #[link_name = "SEQGetVolume"]
    pub unsafe fn get_volume(sequence: *mut Sequence) -> i32;

    /// SEQDRCSetVolume
    ///
    /// `db` is in 0.1 dB.
    #[doc(alias = "SEQDRCSetVolume")]
    #[link_name = "SEQDRCSetVolume"]
    pub unsafe fn drc_set_volume(sequence: *mut Sequence, db: i32);

    /// SEQDRCGetVolume
    #[doc(alias = "SEQDRCGetVolume")]
    #[link_name = "SEQDRCGetVolume"]
    pub unsafe fn drc_get_volume(sequence: *mut Sequence) -> i32;

    /// SEQSetMixerSelect
    #[doc(alias = "SEQSetMixerSelect")]
    #[link_name = "SEQSetMixerSelect"]
    pub unsafe fn set_renderer(sequence: *mut Sequence, renderer: Renderer);

    /// SEQGetMixerSelect
    #[doc(alias = "SEQGetMixerSelect")]
    #[link_name = "SEQGetMixerSelect"]
    pub unsafe fn get_renderer(sequence: *mut Sequence) -> Renderer;
}
