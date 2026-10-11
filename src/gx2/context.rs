//! GX2 context state

use crate::ffi;

/// GX2ShadowState
#[doc(alias = "GX2ShadowState")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowState {
    pub config: [u32; 0xB00],
    pub context: [u32; 0x400],
    pub alu: [u32; 0x800],
    pub loop_: [u32; 0x60],
    _pad0: ffi::unknown<{ (0x80 - 0x60) * 4 }>,
    pub resource: [u32; 0xD9E],
    _pad1: ffi::unknown<{ (0xDC0 - 0xD9E) * 4 }>,
    pub sampler: [u32; 0xA2],
    _pad2: ffi::unknown<{ (0xC0 - 0xA2) * 4 }>,
}

/// GX2ContextState
///
/// Shadowed GPU registers, see [setup_context_state_ex].
#[doc(alias = "GX2ContextState")]
#[repr(C, align(0x100))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextState {
    pub shadow_state: ShadowState,
    _unk0: ffi::unknown<4>,
    pub shadow_display_list_size: u32,
    _unk1: ffi::unknown<{ 0x9E00 - 0x9808 }>,
    pub shadow_display_list: [u32; 192],
}

const _: () = {
    assert!(size_of::<ShadowState>() == 0x9800);
    assert!(core::mem::offset_of!(ShadowState, resource) == 0x5E00);
    assert!(core::mem::offset_of!(ShadowState, sampler) == 0x9500);
    assert!(size_of::<ContextState>() == 0xA100);
    assert!(core::mem::offset_of!(ContextState, shadow_display_list_size) == 0x9804);
    assert!(core::mem::offset_of!(ContextState, shadow_display_list) == 0x9E00);
};

unsafe extern "C" {
    /// GX2SetupContextStateEx
    ///
    /// Initializes `state` with the default state.
    #[doc(alias = "GX2SetupContextStateEx")]
    #[link_name = "GX2SetupContextStateEx"]
    pub unsafe fn setup_context_state_ex(state: *mut ContextState, profiling: ffi::c_bool);

    /// GX2SetContextState
    ///
    /// Restores `state` and makes it the active context.
    #[doc(alias = "GX2SetContextState")]
    #[link_name = "GX2SetContextState"]
    pub unsafe fn set_context_state(state: *const ContextState);

    /// GX2GetContextStateDisplayList
    ///
    /// Display list that restores `state`.
    #[doc(alias = "GX2GetContextStateDisplayList")]
    #[link_name = "GX2GetContextStateDisplayList"]
    pub unsafe fn get_context_state_display_list(
        state: *const ContextState,
        display_list: *mut *mut ffi::c_void,
        size: *mut u32,
    );

    /// GX2SetDefaultState
    #[doc(alias = "GX2SetDefaultState")]
    #[link_name = "GX2SetDefaultState"]
    pub unsafe fn set_default_state();
}
