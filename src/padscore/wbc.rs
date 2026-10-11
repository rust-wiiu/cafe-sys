//! WBC
//!
//! Wii Balance Board weight calculation. Read the raw data with [wpad::BlStatus](super::wpad::BlStatus).

use super::wpad::BlStatus;
use crate::ffi;

/// WBCError
#[doc(alias("WBCError", "WBC_ERR"))]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const NO_CONTROLLER: Self = Self(-1);
    pub const BUSY: Self = Self(-2);
    pub const WRONG_TEMP: Self = Self(-3);
    pub const WRONG_PARAM: Self = Self(-4);
}

unsafe extern "C" {
    /// WBCSetupCalibration
    #[doc(alias = "WBCSetupCalibration")]
    #[link_name = "WBCSetupCalibration"]
    pub unsafe fn setup_calibration() -> ffi::c_bool;

    /// WBCGetCalibrationStatus
    #[doc(alias = "WBCGetCalibrationStatus")]
    #[link_name = "WBCGetCalibrationStatus"]
    pub unsafe fn get_calibration_status() -> ffi::c_bool;

    /// WBCRead
    ///
    /// Converts the raw pressure values of `status` into weights. Requires [setup_calibration]. `size` must be at least 4.
    #[doc(alias = "WBCRead")]
    #[link_name = "WBCRead"]
    pub unsafe fn read(status: *mut BlStatus, weight: *mut f64, size: u32) -> Error;

    /// WBCGetBatteryLevel
    #[doc(alias = "WBCGetBatteryLevel")]
    #[link_name = "WBCGetBatteryLevel"]
    pub unsafe fn get_battery_level(battery: u8) -> i32;

    /// WBCSetZEROPoint
    #[doc(alias = "WBCSetZEROPoint")]
    #[link_name = "WBCSetZEROPoint"]
    pub unsafe fn set_zero_point(press_ave: *mut f64, size: u32) -> Error;

    /// WBCGetZEROPoint
    #[doc(alias = "WBCGetZEROPoint")]
    #[link_name = "WBCGetZEROPoint"]
    pub unsafe fn get_zero_point(zeros: *mut f64, size: u32) -> Error;

    /// WBCGetTGCWeight
    ///
    /// Applies temperature and gravity correction.
    #[doc(alias = "WBCGetTGCWeight")]
    #[link_name = "WBCGetTGCWeight"]
    pub unsafe fn get_tgc_weight(
        total_weight_ave: f64,
        tgc_weight: *mut f64,
        status: *mut BlStatus,
    ) -> Error;

    /// WBCGetTCWeight
    ///
    /// Applies temperature correction.
    #[doc(alias = "WBCGetTCWeight")]
    #[link_name = "WBCGetTCWeight"]
    pub unsafe fn get_tc_weight(weight: f64, corrected: *mut f64, status: *const BlStatus)
    -> Error;

    /// WBCGetGCWeight
    ///
    /// Applies gravity correction.
    #[doc(alias = "WBCGetGCWeight")]
    #[link_name = "WBCGetGCWeight"]
    pub unsafe fn get_gc_weight(weight: f64) -> f64;

    /// WBCGetGravCoff
    #[doc(alias = "WBCGetGravCoff")]
    #[link_name = "WBCGetGravCoff"]
    pub unsafe fn get_grav_coff() -> f64;

    /// WBCGetABSWeight
    #[doc(alias = "WBCGetABSWeight")]
    #[link_name = "WBCGetABSWeight"]
    pub unsafe fn get_abs_weight() -> u32;

    /// WBCGetProductArea
    #[doc(alias = "WBCGetProductArea")]
    #[link_name = "WBCGetProductArea"]
    pub unsafe fn get_product_area() -> u32;
}
