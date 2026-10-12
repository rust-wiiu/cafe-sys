//! GX2 usage alerts
//!
//! Runtime usage alerts.

use crate::ffi;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UdaAlertId(pub i32);

impl UdaAlertId {
    pub const NO_ALERT: Self = Self(-1);
    pub const ENABLE_STATE_SHADOWING: Self = Self(0);
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UdaAlertLevel {
    #[default]
    None = 0,
    Severe = 1,
    Ordinary = 2,
    Pedantic = 3,
}

unsafe extern "C" {
    /// GX2UDAGetLastAlert
    #[doc(alias = "GX2UDAGetLastAlert")]
    #[link_name = "GX2UDAGetLastAlert"]
    pub unsafe fn get_last_alert() -> UdaAlertId;

    /// GX2UDAResetAlertFireCount
    #[doc(alias = "GX2UDAResetAlertFireCount")]
    #[link_name = "GX2UDAResetAlertFireCount"]
    pub unsafe fn reset_alert_fire_count(id: UdaAlertId);

    /// GX2UDASetAlertEnable
    #[doc(alias = "GX2UDASetAlertEnable")]
    #[link_name = "GX2UDASetAlertEnable"]
    pub unsafe fn set_alert_enable(id: UdaAlertId, enable: ffi::c_bool);

    /// GX2UDASetAlertLevel
    #[doc(alias = "GX2UDASetAlertLevel")]
    #[link_name = "GX2UDASetAlertLevel"]
    pub unsafe fn set_alert_level(level: UdaAlertLevel);
}
