//! GX2 usage alerts
//!
//! Runtime alerts about inefficient API usage.

use crate::ffi;

/// GX2UDAAlertID
#[doc(alias = "GX2UDAAlertID")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UdaAlertId(pub i32);

impl UdaAlertId {
    pub const NO_ALERT: Self = Self(-1);
    pub const ENABLE_STATE_SHADOWING: Self = Self(0);
}

/// GX2UDAAlertLevel
#[doc(alias = "GX2UDAAlertLevel")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UdaAlertLevel(pub u32);

impl UdaAlertLevel {
    pub const NONE: Self = Self(0);
    pub const SEVERE: Self = Self(1);
    pub const ORDINARY: Self = Self(2);
    pub const PEDANTIC: Self = Self(3);
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
