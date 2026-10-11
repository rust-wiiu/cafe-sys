//! COS report
//!
//! Leveled console output per system module.

use crate::ffi;

/// COSReportLevel
#[doc(alias = "COSReportLevel")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Level(pub u32);

impl Level {
    pub const ERROR: Self = Self(0);
    pub const WARN: Self = Self(1);
    pub const INFO: Self = Self(2);
    pub const VERBOSE: Self = Self(3);
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
