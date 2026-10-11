//! Sound pipeline
//!
//! Plays sound effects packed into sound tables by the sound tools.

use crate::sndcore2::voice::{Adpcm, AdpcmLoopData, Voice};

/// SP_TYPE
#[doc(alias = "SP_TYPE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundType(pub u32);

impl SoundType {
    pub const ADPCM_ONESHOT: Self = Self(0);
    pub const ADPCM_LOOPED: Self = Self(1);
    pub const PCM16_ONESHOT: Self = Self(2);
    pub const PCM16_LOOPED: Self = Self(3);
    pub const PCM8_ONESHOT: Self = Self(4);
    pub const PCM8_LOOPED: Self = Self(5);
}

/// SPAdpcmEntry
#[doc(alias = "SPAdpcmEntry")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdpcmEntry {
    pub adpcm: Adpcm,
    pub adpcm_loop: AdpcmLoopData,
}

/// SPSoundEntry
#[doc(alias = "SPSoundEntry")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundEntry {
    pub kind: SoundType,
    pub sample_rate: u32,
    pub loop_addr: u32,
    pub loop_end_addr: u32,
    pub end_addr: u32,
    pub current_addr: u32,
    pub adpcm: *mut AdpcmEntry,
}

/// SPSoundTable
#[doc(alias = "SPSoundTable")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundTable {
    pub entries: u32,
    /// Followed by the remaining `entries - 1` entries.
    pub sound: [SoundEntry; 1],
}

const _: () = {
    assert!(size_of::<AdpcmEntry>() == 0x2E);
};

unsafe extern "C" {
    /// SPInitSoundTable
    ///
    /// Resolves the entries of `table` against `samples`, the 32-byte aligned sample data. `zero_buffer` is unused, pass null.
    #[doc(alias = "SPInitSoundTable")]
    #[link_name = "SPInitSoundTable"]
    pub unsafe fn init_sound_table(table: *mut SoundTable, samples: *mut u8, zero_buffer: *mut u8);

    /// SPGetSoundEntry
    #[doc(alias = "SPGetSoundEntry")]
    #[link_name = "SPGetSoundEntry"]
    pub unsafe fn get_sound_entry(table: *mut SoundTable, index: u32) -> *mut SoundEntry;

    /// SPPrepareSound
    ///
    /// Sets up `voice` to play `sound` at `sample_rate`.
    #[doc(alias = "SPPrepareSound")]
    #[link_name = "SPPrepareSound"]
    pub unsafe fn prepare_sound(sound: *mut SoundEntry, voice: *mut Voice, sample_rate: u32);

    /// SPPrepareEnd
    ///
    /// Lets a playing looped sound play to its end and stop. Has no effect within the same audio frame as [prepare_sound].
    #[doc(alias = "SPPrepareEnd")]
    #[link_name = "SPPrepareEnd"]
    pub unsafe fn prepare_end(sound: *mut SoundEntry, voice: *mut Voice);
}
