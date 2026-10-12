//! COS report
//!
//! Leveled console output per system module.

use crate::ffi;

/// COSReportLevel
#[doc(alias = "COSReportLevel")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Error = 0,
    Warn = 1,
    Info = 2,
    Verbose = 3,
}

/// COSReportModule
#[doc(alias = "COSReportModule")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Module(pub u32);

unsafe extern "C" {
    /// COSVReport
    #[doc(alias = "COSVReport")]
    #[link_name = "COSVReport"]
    pub unsafe fn report(module: Module, level: Level, format: *const ffi::c_char, ...);

    /// COSError
    #[doc(alias = "COSError")]
    #[link_name = "COSError"]
    pub unsafe fn error(module: Module, format: *const ffi::c_char, ...);

    /// COSWarn
    #[doc(alias = "COSWarn")]
    #[link_name = "COSWarn"]
    pub unsafe fn warn(module: Module, format: *const ffi::c_char, ...);

    /// COSInfo
    #[doc(alias = "COSInfo")]
    #[link_name = "COSInfo"]
    pub unsafe fn info(module: Module, format: *const ffi::c_char, ...);

    /// COSVerbose
    #[doc(alias = "COSVerbose")]
    #[link_name = "COSVerbose"]
    pub unsafe fn verbose(module: Module, format: *const ffi::c_char, ...);
}
