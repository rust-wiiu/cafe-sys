use crate::ffi;
use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextState : u16 {
        const OsCallback = 1 << 3;
        const UserModeSaved = 1 << 3;
    }
}

/// OSContext
#[repr(C, align(8))]
#[derive(Clone)]
pub struct Context {
    pub tag: u64,
    pub gpr: [u32; 32],
    pub cr: u32,
    pub lr: u32,
    pub ctr: u32,
    pub xer: u32,
    pub ssr0: u32,
    pub ssr1: u32,
    pub dsisr: u32,
    pub dar: u32,
    _unk0: ffi::unknown<0xC>,
    pub fpscr: u32,
    pub fpr: [f64; 32],
    pub spin_lock_count: u16,
    pub state: ContextState,
    pub gqr: [u32; 8],
    pub upir: u32,
    pub psf: [f64; 32],
    pub coretime: [u64; 3],
    pub start_time: u64,
    pub error: u32,
    _unk1: ffi::unknown<0x4>,
    pub pmc1: u32,
    pub pmc2: u32,
    pub pmc3: u32,
    pub pmc4: u32,
    pub mmcr0: u32,
    pub mmcr1: u32,
}

impl Context {
    pub const TAG: u64 = 0x4F53436F6E747874;
}

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSInitContext
    #[link_name = "OSInitContext"]
    pub unsafe fn OSInitContext(
        context: *mut Context,
        entry: *mut ffi::c_void,
        stack: *mut ffi::c_void,
    );

    /// OSDumpContext
    #[link_name = "OSDumpContext"]
    pub unsafe fn OSDumpContext(context: *mut Context);

    /// OSLoadContext
    #[link_name = "OSLoadContext"]
    pub unsafe fn OSLoadContext(context: *mut Context);

    /// OSLoadFPUContext
    #[link_name = "OSLoadFPUContext"]
    pub unsafe fn OSLoadFPUContext(context: *mut Context);

    /// OSSaveFPUContext
    #[link_name = "OSSaveFPUContext"]
    pub unsafe fn OSSaveFPUContext(context: *mut Context);

    /// OSGetCurrentContext
    #[link_name = "OSGetCurrentContext"]
    pub unsafe fn OSGetCurrentContext() -> *mut Context;

    /// OSSetCurrentContext
    #[link_name = "OSSetCurrentContext"]
    pub unsafe fn OSSetCurrentContext(context: *mut Context);

    /// OSSwitchStack
    #[link_name = "OSSwitchStack"]
    pub unsafe fn OSSwitchStack(stack: *mut ffi::c_void);

    /// __OSSetCurrentUserContext
    #[link_name = "__OSSetCurrentUserContext"]
    pub unsafe fn __OSSetCurrentUserContext(context: *mut Context);

    /// __OSSetAndLoadContext
    #[link_name = "__OSSetAndLoadContext"]
    pub unsafe fn __OSSetAndLoadContext(context: *mut Context);
}
