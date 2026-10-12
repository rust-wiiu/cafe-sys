//! Kernel
//!
//! Low-level kernel calls.

use super::context::Context;
use super::exception::ExceptionType;
use crate::ffi;

/// OSICICommand
///
/// Inter-core interrupt commands.
#[doc(alias = "OSICICommand")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IciCommand {
    InvalidIcRange = 1,
    RescheduleCore = 2,
    HaltCore = 3,
    ProcExit = 4,
    SetDabr = 5,
    ProcSched = 6,
    FastBgExit = 7,
    IopShellCoreTrace = 8,
    SystemFatal = 9,
    SetIabr = 10,
    Panic0x15 = 0xB,
    ProcKill = 0xC,
    ProcCrash = 0xD,
    Unknown = 0xE,
    OverlayArena = 0xF,
}

/// OSExceptionCallbackExFn
#[doc(alias = "OSExceptionCallbackExFn")]
pub type ExceptionCallbackEx = Option<
    unsafe extern "C" fn(
        kind: ExceptionType,
        interrupted_context: *mut Context,
        callback_context: *mut Context,
    ),
>;

/// KernelTimerCallbackFn
#[doc(alias = "KernelTimerCallbackFn")]
pub type TimerCallback = Option<
    unsafe extern "C" fn(
        kind: ExceptionType,
        interrupted_context: *mut Context,
        current_context: *mut Context,
    ),
>;

/// KernelTimerHandle
#[doc(alias = "KernelTimerHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimerHandle(pub u32);

/// OSExceptionChainInfo
#[doc(alias = "OSExceptionChainInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionChainInfo {
    pub callback: ExceptionCallbackEx,
    pub stack: *mut ffi::c_void,
    pub context: *mut Context,
}

/// Coreinit section of [KernelInfo0].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoreinitInfo {
    pub loader_handle: *mut ffi::c_void,
    pub text_addr: *mut ffi::c_void,
    pub text_offset: u32,
    pub text_size: u32,
    pub data_addr: *mut ffi::c_void,
    pub data_offset: u32,
    pub data_size: u32,
    pub load_addr: *mut ffi::c_void,
    pub load_offset: u32,
    pub load_size: u32,
}

/// KernelInfo0
///
/// Info type 0 of [get_info].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInfo0 {
    pub upid: i32,
    pub rampid: i32,
    pub app_flags: u32,
    pub data_area_start: *mut ffi::c_void,
    pub data_area_end: *mut ffi::c_void,
    pub phys_data_area_start: *mut ffi::c_void,
    pub phys_data_area_end: *mut ffi::c_void,
    pub phys_avail_start: *mut ffi::c_void,
    pub phys_avail_end: *mut ffi::c_void,
    pub phys_code_gen_start: *mut ffi::c_void,
    pub phys_code_gen_end: *mut ffi::c_void,
    pub sda_base: *mut ffi::c_void,
    pub sda2_base: *mut ffi::c_void,
    pub system_heap_size: u32,
    pub stack_end: [*mut ffi::c_void; 3],
    pub stack_base: [*mut ffi::c_void; 3],
    pub exception_stack_end: [*mut ffi::c_void; 3],
    pub exception_stack_base: [*mut ffi::c_void; 3],
    pub locked_cache_base: [*mut ffi::c_void; 3],
    pub coreinit: CoreinitInfo,
    _unk0x9c: ffi::unknown<4>,
    pub title_id: u64,
}

/// KernelInfo6
///
/// Info type 6 of [get_info].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInfo6 {
    pub os_title_id: u64,
    _unk0x08: ffi::unknown<4>,
    _pad: ffi::unknown<0xFC>,
}

const _: () = {
    assert!(size_of::<KernelInfo6>() == 0x108);
};

unsafe extern "C" {
    /// __KernelSetUserModeExHandler
    #[doc(alias = "__KernelSetUserModeExHandler")]
    #[link_name = "__KernelSetUserModeExHandler"]
    pub unsafe fn set_user_mode_ex_handler(
        kind: ExceptionType,
        chain_info: *mut ExceptionChainInfo,
        prev_chain_info: *mut ExceptionChainInfo,
    );

    /// __KernelAllocateTimer
    #[doc(alias = "__KernelAllocateTimer")]
    #[link_name = "__KernelAllocateTimer"]
    pub unsafe fn allocate_timer(
        callback: TimerCallback,
        exception_stack: *mut ffi::c_void,
        context: *mut Context,
    ) -> TimerHandle;

    /// __KernelPrimeTimer
    #[doc(alias = "__KernelPrimeTimer")]
    #[link_name = "__KernelPrimeTimer"]
    pub unsafe fn prime_timer(
        handle: TimerHandle,
        start_ticks: u64,
        interval_ticks: u64,
        unknown: u32,
    ) -> u32;

    /// __KernelSendICI
    #[doc(alias = "__KernelSendICI")]
    #[link_name = "__KernelSendICI"]
    pub unsafe fn send_ici(
        command: IciCommand,
        arg: *mut ffi::c_void,
        unknown1: u32,
        unknown2: u32,
    );

    /// __KernelGetInfo
    ///
    /// Type 0 fills a [KernelInfo0], type 6 a [KernelInfo6].
    #[doc(alias = "__KernelGetInfo")]
    #[link_name = "__KernelGetInfo"]
    pub unsafe fn get_info(kind: u32, buffer: *mut ffi::c_void, size: u32, core: u32);
}
