//! Exception

use super::context::Context;
use super::thread::Thread;
use crate::ffi;

/// OSExceptionType
#[doc(alias = "OSExceptionType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExceptionType(pub u32);

impl ExceptionType {
    pub const SYSTEM_RESET: Self = Self(0);
    pub const MACHINE_CHECK: Self = Self(1);
    pub const DSI: Self = Self(2);
    pub const ISI: Self = Self(3);
    pub const EXTERNAL_INTERRUPT: Self = Self(4);
    pub const ALIGNMENT: Self = Self(5);
    pub const PROGRAM: Self = Self(6);
    pub const FLOATING_POINT: Self = Self(7);
    pub const DECREMENTER: Self = Self(8);
    pub const SYSTEM_CALL: Self = Self(9);
    pub const TRACE: Self = Self(10);
    pub const PERFORMANCE_MONITOR: Self = Self(11);
    pub const BREAKPOINT: Self = Self(12);
    pub const SYSTEM_INTERRUPT: Self = Self(13);
    pub const ICI: Self = Self(14);
}

/// OSExceptionMode
#[doc(alias = "OSExceptionMode")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExceptionMode {
    System = 0,
    /// Current thread, on the current core.
    Thread = 1,
    /// Current core.
    Global = 2,
    ThreadAllCores = 3,
    GlobalAllCores = 4,
}

/// FPSCR exception enable bits, for the FPU exception functions.
#[doc(alias = "FPSCR")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FpuExceptionMask(pub u32);

impl FpuExceptionMask {
    /// Invalid operation.
    pub const VE: Self = Self(0x80);
    /// Overflow.
    pub const OE: Self = Self(0x40);
    /// Underflow.
    pub const UE: Self = Self(0x20);
    /// Division by zero.
    pub const ZE: Self = Self(0x10);
    /// Inexact.
    pub const XE: Self = Self(0x08);
    pub const ALL: Self = Self(0xF8);
}

impl core::ops::BitOr for FpuExceptionMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSExceptionCallbackFn
#[doc(alias = "OSExceptionCallbackFn")]
pub type ExceptionCallback =
    Option<unsafe extern "C" fn(interrupted_context: *mut Context) -> ffi::c_bool>;

unsafe extern "C" {
    /// OSSetExceptionCallback
    ///
    /// Same as [set_callback_ex] with [ExceptionMode::Thread]. Returns the previous callback.
    #[doc(alias = "OSSetExceptionCallback")]
    #[link_name = "OSSetExceptionCallback"]
    pub unsafe fn set_callback(
        kind: ExceptionType,
        callback: ExceptionCallback,
    ) -> ExceptionCallback;

    /// OSSetExceptionCallbackEx
    ///
    /// Returns the previous callback. Only [ExceptionType::DSI], [ExceptionType::ISI],
    /// [ExceptionType::PROGRAM] and [ExceptionType::PERFORMANCE_MONITOR] are supported, other
    /// types are ignored and return `None`.
    #[doc(alias = "OSSetExceptionCallbackEx")]
    #[link_name = "OSSetExceptionCallbackEx"]
    pub unsafe fn set_callback_ex(
        mode: ExceptionMode,
        kind: ExceptionType,
        callback: ExceptionCallback,
    ) -> ExceptionCallback;

    /// OSSetDABR
    ///
    /// Sets a data breakpoint, raising [ExceptionType::DSI].
    #[doc(alias = "OSSetDABR")]
    #[link_name = "OSSetDABR"]
    pub unsafe fn set_dabr(
        all_cores: ffi::c_bool,
        address: *mut ffi::c_void,
        match_reads: ffi::c_bool,
        match_writes: ffi::c_bool,
    );

    /// OSSetIABR
    ///
    /// Sets an instruction breakpoint, raising [ExceptionType::BREAKPOINT].
    #[doc(alias = "OSSetIABR")]
    #[link_name = "OSSetIABR"]
    pub unsafe fn set_iabr(all_cores: ffi::c_bool, address: *mut ffi::c_void);

    /// OSEnableThreadFPUException
    ///
    /// Only changes threads in the ready state, returns false otherwise.
    #[doc(alias = "OSEnableThreadFPUException")]
    #[link_name = "OSEnableThreadFPUException"]
    pub unsafe fn enable_thread_fpu_exception(
        thread: *mut Thread,
        mask: FpuExceptionMask,
    ) -> ffi::c_bool;

    /// OSEnableAllThreadFPUException
    ///
    /// Returns the number of threads that were changed, see [enable_thread_fpu_exception].
    #[doc(alias = "OSEnableAllThreadFPUException")]
    #[link_name = "OSEnableAllThreadFPUException"]
    pub unsafe fn enable_all_thread_fpu_exception(mask: FpuExceptionMask) -> u32;

    /// OSDisableThreadFPUException
    ///
    /// Only changes threads in the ready state, returns false otherwise.
    #[doc(alias = "OSDisableThreadFPUException")]
    #[link_name = "OSDisableThreadFPUException"]
    pub unsafe fn disable_thread_fpu_exception(thread: *mut Thread) -> ffi::c_bool;

    /// OSDisableAllThreadFPUException
    ///
    /// Returns the number of threads that were changed, see [disable_thread_fpu_exception].
    #[doc(alias = "OSDisableAllThreadFPUException")]
    #[link_name = "OSDisableAllThreadFPUException"]
    pub unsafe fn disable_all_thread_fpu_exception() -> u32;

    /// OSDisableContextFPUException
    #[doc(alias = "OSDisableContextFPUException")]
    #[link_name = "OSDisableContextFPUException"]
    pub unsafe fn disable_context_fpu_exception(context: *mut Context);
}
