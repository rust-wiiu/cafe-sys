use crate::ffi::*;

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSFatal
    #[link_name = "OSFatal"]
    pub unsafe fn fatal(msg: *const c_char);

    /// OSPanic
    #[link_name = "OSPanic"]
    pub unsafe fn panic(file: *const c_char, line: u32, fmt: *const c_char) -> !;

    /// OSConsoleWrite
    #[link_name = "OSConsoleWrite"]
    pub unsafe fn console_write(msg: *const c_char, size: u32);

    /// __OSConsoleWrite
    #[link_name = "__OSConsoleWrite"]
    pub unsafe fn __console_write(msg: *const c_char, size: u32);

    /// OSReport
    #[link_name = "OSReport"]
    pub unsafe fn report(msg: *const c_char);

    /// OSReportVerbose
    #[link_name = "OSReportVerbose"]
    pub unsafe fn report_verbose(msg: *const c_char);

    /// OSReportInfo
    #[link_name = "OSReportInfo"]
    pub unsafe fn report_info(msg: *const c_char);

    /// OSReportWarn
    #[link_name = "OSReportWarn"]
    pub unsafe fn report_warn(msg: *const c_char);

    #[link_name = "exit"]
    pub unsafe fn exit(code: i32) -> !;

    #[doc(alias = "OSIsDebuggerInitialized")]
    #[link_name = "OSIsDebuggerInitialized"]
    pub unsafe fn debugger_initialized() -> c_bool;
}
