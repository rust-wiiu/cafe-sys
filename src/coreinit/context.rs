//! Context
//!
//! Saved CPU state of a thread or exception.

use crate::ffi;

/// OS_CONTEXT_TAG
///
/// Value of [Context::tag].
pub const CONTEXT_TAG: u64 = 0x4F53436F_6E747874;

/// OSContextState
#[doc(alias = "OSContextState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextState(pub u16);

impl ContextState {
    pub const EXCEPTION: Self = Self(1 << 1);
    pub const PMC_SAVED: Self = Self(1 << 2);
    pub const OS_CALLBACK: Self = Self(1 << 3);
    pub const USER_MODE_SAVED: Self = Self(1 << 4);
}

impl core::ops::BitOr for ContextState {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSContext
#[doc(alias = "OSContext")]
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    /// [CONTEXT_TAG].
    pub tag: u64,
    pub gpr: [u32; 32],
    pub cr: u32,
    pub lr: u32,
    pub ctr: u32,
    pub xer: u32,
    pub srr0: u32,
    pub srr1: u32,
    /// DSISR for DSI exceptions.
    pub exception_specific0: u32,
    /// DAR for DSI exceptions.
    pub exception_specific1: u32,
    pub crash_exception_type: u32,
    _reserved: ffi::unknown<4>,
    _fpscr_pad: ffi::unknown<4>,
    pub fpscr: u32,
    pub fpr: [f64; 32],
    pub spin_lock_count: u16,
    pub state: ContextState,
    pub gqr: [u32; 8],
    /// Index of the current core.
    pub pir: u32,
    /// Paired single second halves.
    pub psf: [f64; 32],
    pub core_time: [u64; 3],
    pub start_time: u64,
    pub error: u32,
    pub attributes: u32,
    pub pmc1: u32,
    pub pmc2: u32,
    pub pmc3: u32,
    pub pmc4: u32,
    pub mmcr0: u32,
    pub mmcr1: u32,
}

const _: () = {
    assert!(size_of::<Context>() == 0x320);
    assert!(core::mem::offset_of!(Context, fpscr) == 0xB4);
    assert!(core::mem::offset_of!(Context, fpr) == 0xB8);
    assert!(core::mem::offset_of!(Context, spin_lock_count) == 0x1B8);
    assert!(core::mem::offset_of!(Context, pir) == 0x1DC);
    assert!(core::mem::offset_of!(Context, core_time) == 0x2E0);
    assert!(core::mem::offset_of!(Context, error) == 0x300);
    assert!(core::mem::offset_of!(Context, mmcr1) == 0x31C);
};

unsafe extern "C" {
    /// OSInitContext
    ///
    /// Sets up `context` to start at `pc` with stack pointer `sp`.
    #[doc(alias = "OSInitContext")]
    #[link_name = "OSInitContext"]
    pub unsafe fn init(context: *mut Context, pc: *const ffi::c_void, sp: *mut ffi::c_void);

    /// OSClearContext
    #[doc(alias = "OSClearContext")]
    #[link_name = "OSClearContext"]
    pub unsafe fn clear(context: *mut Context);

    /// OSSaveContext
    ///
    /// Returns 0 when saving and 1 when resumed through [load].
    #[doc(alias = "OSSaveContext")]
    #[link_name = "OSSaveContext"]
    pub unsafe fn save(context: *mut Context) -> u32;

    /// OSLoadContext
    #[doc(alias = "OSLoadContext")]
    #[link_name = "OSLoadContext"]
    pub unsafe fn load(context: *mut Context);

    /// OSGetCurrentContext
    #[doc(alias = "OSGetCurrentContext")]
    #[link_name = "OSGetCurrentContext"]
    pub unsafe fn get_current() -> *mut Context;

    /// OSSetCurrentContext
    #[doc(alias = "OSSetCurrentContext")]
    #[link_name = "OSSetCurrentContext"]
    pub unsafe fn set_current(context: *mut Context);

    /// OSGetCurrentFPUContext
    #[doc(alias = "OSGetCurrentFPUContext")]
    #[link_name = "OSGetCurrentFPUContext"]
    pub unsafe fn get_current_fpu() -> *mut Context;

    /// OSSetCurrentFPUContext
    #[doc(alias = "OSSetCurrentFPUContext")]
    #[link_name = "OSSetCurrentFPUContext"]
    pub unsafe fn set_current_fpu(context: *mut Context);

    /// OSLoadFPUContext
    #[doc(alias = "OSLoadFPUContext")]
    #[link_name = "OSLoadFPUContext"]
    pub unsafe fn load_fpu(context: *mut Context);

    /// OSSaveFPUContext
    #[doc(alias = "OSSaveFPUContext")]
    #[link_name = "OSSaveFPUContext"]
    pub unsafe fn save_fpu(context: *mut Context);

    /// OSDumpContext
    ///
    /// Prints the registers of `context`.
    #[doc(alias = "OSDumpContext")]
    #[link_name = "OSDumpContext"]
    pub unsafe fn dump(context: *const Context);

    /// OSSwitchStack
    ///
    /// Switches the current stack and returns the previous stack pointer.
    #[doc(alias = "OSSwitchStack")]
    #[link_name = "OSSwitchStack"]
    pub unsafe fn switch_stack(new_sp: *mut ffi::c_void) -> *mut ffi::c_void;

    /// __OSSetCurrentUserContext
    #[doc(alias = "__OSSetCurrentUserContext")]
    #[link_name = "__OSSetCurrentUserContext"]
    pub unsafe fn set_current_user(context: *mut Context);

    /// __OSSetAndLoadContext
    #[doc(alias = "__OSSetAndLoadContext")]
    #[link_name = "__OSSetAndLoadContext"]
    pub unsafe fn set_and_load(context: *mut Context);
}
