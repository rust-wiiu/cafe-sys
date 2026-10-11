//! Energy saver
//!
//! Screen dimming and auto power-down (APD).

use super::im::Error;
use crate::ffi;

unsafe extern "C" {
    /// IMEnableDim
    #[doc(alias = "IMEnableDim")]
    #[link_name = "IMEnableDim"]
    pub unsafe fn enable_dim() -> Error;

    /// IMDisableDim
    #[doc(alias = "IMDisableDim")]
    #[link_name = "IMDisableDim"]
    pub unsafe fn disable_dim() -> Error;

    /// IMIsDimEnabled
    #[doc(alias = "IMIsDimEnabled")]
    #[link_name = "IMIsDimEnabled"]
    pub unsafe fn is_dim_enabled(enabled: *mut u32) -> Error;

    /// IMGetDimPeriod
    ///
    /// Seconds of inactivity before dimming.
    #[doc(alias = "IMGetDimPeriod")]
    #[link_name = "IMGetDimPeriod"]
    pub unsafe fn get_dim_period(seconds: *mut u32) -> Error;

    /// IMGetTimeBeforeDimming
    #[doc(alias = "IMGetTimeBeforeDimming")]
    #[link_name = "IMGetTimeBeforeDimming"]
    pub unsafe fn get_time_before_dimming(seconds: *mut u32) -> Error;

    /// IMSetDimEnableTV
    #[doc(alias = "IMSetDimEnableTV")]
    #[link_name = "IMSetDimEnableTV"]
    pub unsafe fn set_dim_enable_tv(enable: ffi::c_bool) -> Error;

    /// IMSetDimEnableDRC
    #[doc(alias = "IMSetDimEnableDRC")]
    #[link_name = "IMSetDimEnableDRC"]
    pub unsafe fn set_dim_enable_drc(enable: ffi::c_bool) -> Error;

    /// IMGetDimEnableTV
    #[doc(alias = "IMGetDimEnableTV")]
    #[link_name = "IMGetDimEnableTV"]
    pub unsafe fn get_dim_enable_tv(enabled: *mut u32) -> Error;

    /// IMGetDimEnableDRC
    #[doc(alias = "IMGetDimEnableDRC")]
    #[link_name = "IMGetDimEnableDRC"]
    pub unsafe fn get_dim_enable_drc(enabled: *mut u32) -> Error;

    /// IMEnableAPD
    #[doc(alias = "IMEnableAPD")]
    #[link_name = "IMEnableAPD"]
    pub unsafe fn enable_apd() -> Error;

    /// IMDisableAPD
    #[doc(alias = "IMDisableAPD")]
    #[link_name = "IMDisableAPD"]
    pub unsafe fn disable_apd() -> Error;

    /// IMIsAPDEnabled
    #[doc(alias = "IMIsAPDEnabled")]
    #[link_name = "IMIsAPDEnabled"]
    pub unsafe fn is_apd_enabled(enabled: *mut u32) -> Error;

    /// IMIsAPDEnabledBySysSettings
    #[doc(alias = "IMIsAPDEnabledBySysSettings")]
    #[link_name = "IMIsAPDEnabledBySysSettings"]
    pub unsafe fn is_apd_enabled_by_sys_settings(enabled: *mut u32) -> Error;

    /// IMGetTimeBeforeAPD
    #[doc(alias = "IMGetTimeBeforeAPD")]
    #[link_name = "IMGetTimeBeforeAPD"]
    pub unsafe fn get_time_before_apd(seconds: *mut u32) -> Error;

    /// IMStartAPDVideoMode
    #[doc(alias = "IMStartAPDVideoMode")]
    #[link_name = "IMStartAPDVideoMode"]
    pub unsafe fn start_apd_video_mode() -> Error;
}
