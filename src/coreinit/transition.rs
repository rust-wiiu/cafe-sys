//! Transition audio
//!
//! Audio played while switching between processes.

use crate::ffi;

unsafe extern "C" {
    /// __OSGetSavedAudioFlags
    #[doc(alias = "__OSGetSavedAudioFlags")]
    #[link_name = "__OSGetSavedAudioFlags"]
    pub unsafe fn get_saved_audio_flags() -> ffi::c_int;

    /// __OSGetTransitionAudioBuffer
    #[doc(alias = "__OSGetTransitionAudioBuffer")]
    #[link_name = "__OSGetTransitionAudioBuffer"]
    pub unsafe fn get_audio_buffer(buffer: *mut *mut ffi::c_void, size: *mut u32) -> ffi::c_int;

    /// __OSSetTransitionAudioSize
    #[doc(alias = "__OSSetTransitionAudioSize")]
    #[link_name = "__OSSetTransitionAudioSize"]
    pub unsafe fn set_audio_size(size: u32);
}
