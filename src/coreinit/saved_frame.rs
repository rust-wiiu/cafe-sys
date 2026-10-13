//! Saved frames
//!
//! Frames saved for the TV and GamePad.

use crate::ffi;

/// OSSavedFrameType
#[doc(alias = "OSSavedFrameType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SavedFrameType {
    A = 0,
    B = 1,
}

/// OSSavedFrameScreen
#[doc(alias = "OSSavedFrameScreen")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SavedFrameScreen {
    Tv = 2,
    Drc = 4,
}

unsafe extern "C" {
    /// __OSClearSavedFrame
    #[doc(alias = "__OSClearSavedFrame")]
    #[link_name = "__OSClearSavedFrame"]
    pub unsafe fn clear(kind: SavedFrameType, screen: SavedFrameScreen) -> u32;

    /// __OSGetSavedFrame
    #[doc(alias = "__OSGetSavedFrame")]
    #[link_name = "__OSGetSavedFrame"]
    pub unsafe fn get(screen: SavedFrameScreen, buffer: *mut ffi::c_void, size: u32);

    /// __OSGetSavedFrameA
    #[doc(alias = "__OSGetSavedFrameA")]
    #[link_name = "__OSGetSavedFrameA"]
    pub unsafe fn get_a(screen: SavedFrameScreen, buffer: *mut ffi::c_void, size: u32);

    /// __OSGetSavedFrameB
    #[doc(alias = "__OSGetSavedFrameB")]
    #[link_name = "__OSGetSavedFrameB"]
    pub unsafe fn get_b(screen: SavedFrameScreen, buffer: *mut ffi::c_void, size: u32);

    /// __OSGetSavedFrameGammaA
    #[doc(alias = "__OSGetSavedFrameGammaA")]
    #[link_name = "__OSGetSavedFrameGammaA"]
    pub unsafe fn get_gamma_a(screen: SavedFrameScreen, gamma: *mut f32) -> ffi::c_bool;

    /// __OSGetSavedFrameGammaB
    #[doc(alias = "__OSGetSavedFrameGammaB")]
    #[link_name = "__OSGetSavedFrameGammaB"]
    pub unsafe fn get_gamma_b(screen: SavedFrameScreen, gamma: *mut f32) -> ffi::c_bool;

    /// __OSGetSavedFramePtr
    #[doc(alias = "__OSGetSavedFramePtr")]
    #[link_name = "__OSGetSavedFramePtr"]
    pub unsafe fn get_ptr(kind: SavedFrameType, screen: SavedFrameScreen) -> *mut ffi::c_void;

    /// __OSGetSavedFramePtrForRead
    #[doc(alias = "__OSGetSavedFramePtrForRead")]
    #[link_name = "__OSGetSavedFramePtrForRead"]
    pub unsafe fn get_ptr_for_read(screen: SavedFrameScreen) -> *mut ffi::c_void;

    /// __OSGetSavedFramePtrForWrite
    #[doc(alias = "__OSGetSavedFramePtrForWrite")]
    #[link_name = "__OSGetSavedFramePtrForWrite"]
    pub unsafe fn get_ptr_for_write(screen: SavedFrameScreen) -> *mut ffi::c_void;

    /// __OSGetSavedFrames
    #[doc(alias = "__OSGetSavedFrames")]
    #[link_name = "__OSGetSavedFrames"]
    pub unsafe fn get_frames() -> u32;

    /// __OSGetSavedFramesA
    #[doc(alias = "__OSGetSavedFramesA")]
    #[link_name = "__OSGetSavedFramesA"]
    pub unsafe fn get_frames_a() -> u32;

    /// __OSGetSavedFramesB
    #[doc(alias = "__OSGetSavedFramesB")]
    #[link_name = "__OSGetSavedFramesB"]
    pub unsafe fn get_frames_b() -> u32;

    /// __OSResetSavedFrame
    #[doc(alias = "__OSResetSavedFrame")]
    #[link_name = "__OSResetSavedFrame"]
    pub unsafe fn reset(screen: SavedFrameScreen);

    /// __OSSetSavedFrame
    #[doc(alias = "__OSSetSavedFrame")]
    #[link_name = "__OSSetSavedFrame"]
    pub unsafe fn set(screen: SavedFrameScreen, data: *const ffi::c_void, size: u32) -> ffi::c_int;

    /// __OSSetSavedFrameGamma
    #[doc(alias = "__OSSetSavedFrameGamma")]
    #[link_name = "__OSSetSavedFrameGamma"]
    pub unsafe fn set_gamma(gamma: f32, screen: SavedFrameScreen);
}
