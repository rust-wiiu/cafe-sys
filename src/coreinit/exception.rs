use super::context::Context;
use crate::ffi;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// OSExceptionCallbackFn
pub type ExceptionCallbackFn = Option<unsafe extern "C" fn(context: *mut Context) -> ffi::c_bool>;

/// OSExceptionMode
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum ExceptionMode {
    System = 0,
    Thread = 1,
    Global = 2,
    ThreadAllCores = 3,
    GlobalAllCores = 4,
}

/// OSExceptionType
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum ExceptionType {
    SystemReset = 0,
    MachineCheck = 1,
    Dsi = 2,
    Isi = 3,
    ExternalInterrupt = 4,
    Alignment = 5,
    Program = 6,
    FloatingPoint = 7,
    Decrementer = 8,
    SystemCall = 9,
    Trace = 10,
    PerformanceMonitor = 11,
    Breakpoint = 12,
    SystemInterrupt = 13,
    Ici = 14,
}

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSSetExceptionCallback
    #[link_name = "OSSetExceptionCallback"]
    pub unsafe fn set_callback(
        r#type: ExceptionType,
        callback: ExceptionCallbackFn,
    ) -> ExceptionCallbackFn;

    /// OSSetExceptionCallbackEx
    #[link_name = "OSSetExceptionCallbackEx"]
    pub unsafe fn set_callback_ex(
        mode: ExceptionMode,
        r#type: ExceptionType,
        callback: ExceptionCallbackFn,
    ) -> ExceptionCallbackFn;
}
