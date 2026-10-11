//! Coroutine
//!
//! Saved non-volatile register state for cooperative switching on the current thread.

use crate::ffi;

/// OSCoroutine
#[doc(alias = "OSCoroutine")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coroutine {
    /// Resume address.
    pub lr: u32,
    pub cr: u32,
    pub gqr1: u32,
    /// Stack pointer.
    pub r1: u32,
    pub r2: u32,
    pub r13: u32,
    /// r14-r31.
    pub gpr: [u32; 18],
    /// f14-f31.
    pub fpr: [f64; 18],
    /// Paired single second halves of f14-f31.
    pub psr: [f64; 18],
}

const _: () = {
    assert!(size_of::<Coroutine>() == 0x180);
};

unsafe extern "C" {
    /// OSInitCoroutine
    ///
    /// Sets up `coroutine` to start at `pc` with stack pointer `sp`.
    #[doc(alias = "OSInitCoroutine")]
    #[link_name = "OSInitCoroutine"]
    pub unsafe fn init(coroutine: *mut Coroutine, pc: *const ffi::c_void, sp: *mut ffi::c_void);

    /// OSSaveCoroutine
    ///
    /// Returns null when saving, or the value passed to [load] when resumed.
    #[doc(alias = "OSSaveCoroutine")]
    #[link_name = "OSSaveCoroutine"]
    pub unsafe fn save(coroutine: *mut Coroutine) -> *mut ffi::c_void;

    /// OSLoadCoroutine
    ///
    /// Resumes `coroutine`, its [save] returns `value`.
    #[doc(alias = "OSLoadCoroutine")]
    #[link_name = "OSLoadCoroutine"]
    pub unsafe fn load(coroutine: *mut Coroutine, value: *mut ffi::c_void);

    /// OSSwitchCoroutine
    ///
    /// Saves into `from` and resumes `to`.
    #[doc(alias = "OSSwitchCoroutine")]
    #[link_name = "OSSwitchCoroutine"]
    pub unsafe fn switch(from: *mut Coroutine, to: *mut Coroutine);
}
