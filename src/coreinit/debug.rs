use crate::ffi;

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// OSFatal
    #[link_name = "OSFatal"]
    pub unsafe fn fatal(msg: *const ffi::c_char);

    /// OSPanic
    #[link_name = "OSPanic"]
    pub unsafe fn panic(file: *const ffi::c_char, line: u32, fmt: *const ffi::c_char);

    /// OSConsoleWrite
    #[link_name = "OSConsoleWrite"]
    pub unsafe fn console_write(msg: *const ffi::c_char, size: u32);

    /// __OSConsoleWrite
    #[link_name = "__OSConsoleWrite"]
    pub unsafe fn __console_write(msg: *const ffi::c_char, size: u32);

    /// OSReport
    #[link_name = "OSReport"]
    pub unsafe fn report(msg: *const ffi::c_char);

    /// OSReportVerbose
    #[link_name = "OSReportVerbose"]
    pub unsafe fn report_verbose(msg: *const ffi::c_char);

    /// OSReportInfo
    #[link_name = "OSReportInfo"]
    pub unsafe fn report_info(msg: *const ffi::c_char);

    /// OSReportWarn
    #[link_name = "OSReportWarn"]
    pub unsafe fn report_warn(msg: *const ffi::c_char);
}
