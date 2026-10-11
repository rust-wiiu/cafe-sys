//! System information

use super::time::Time;
use crate::ffi;

/// OSSystemInfo
#[doc(alias = "OSSystemInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemInfo {
    /// The timer runs at a quarter of this.
    pub bus_clock_speed: u32,
    pub core_clock_speed: u32,
    /// Ticks from 2000-01-01 until the system reset.
    pub base_time: Time,
    pub l2_size: [u32; 3],
    pub cpu_freq_ratio: u32,
}

/// OS_GETINFO
///
/// Selector of [get_info] and [set_info].
#[doc(alias = "OS_GETINFO", alias = "OS_SETINFO")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfoSelector(pub u32);

impl InfoSelector {
    pub const PMC_THREADS: Self = Self(0);
    /// Get only, [InfoHdp].
    pub const HDP: Self = Self(1);
}

/// OSGetInfo_HDP
#[doc(alias = "OSGetInfo_HDP")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfoHdp {
    pub start: *mut ffi::c_void,
    pub size: u32,
}

const _: () = {
    assert!(size_of::<SystemInfo>() == 0x20);
};

unsafe extern "C" {
    /// OSGetSystemInfo
    #[doc(alias = "OSGetSystemInfo")]
    #[link_name = "OSGetSystemInfo"]
    pub unsafe fn get_system_info() -> *const SystemInfo;

    /// OSEnableHomeButtonMenu
    ///
    /// Returns the previous state.
    #[doc(alias = "OSEnableHomeButtonMenu")]
    #[link_name = "OSEnableHomeButtonMenu"]
    pub unsafe fn enable_home_button_menu(enable: ffi::c_bool) -> ffi::c_bool;

    /// OSIsHomeButtonMenuEnabled
    #[doc(alias = "OSIsHomeButtonMenuEnabled")]
    #[link_name = "OSIsHomeButtonMenuEnabled"]
    pub unsafe fn is_home_button_menu_enabled() -> ffi::c_bool;

    /// OSGetOSID
    #[doc(alias = "OSGetOSID")]
    #[link_name = "OSGetOSID"]
    pub unsafe fn get_os_id() -> u64;

    /// __OSGetProcessSDKVersion
    #[doc(alias = "__OSGetProcessSDKVersion")]
    #[link_name = "__OSGetProcessSDKVersion"]
    pub unsafe fn get_process_sdk_version() -> u32;

    /// OSGetInfo
    #[doc(alias = "OSGetInfo")]
    #[link_name = "OSGetInfo"]
    pub unsafe fn get_info(
        selector: InfoSelector,
        info: *mut ffi::c_void,
        size: u32,
        options: u32,
    ) -> ffi::c_bool;

    /// OSSetInfo
    #[doc(alias = "OSSetInfo")]
    #[link_name = "OSSetInfo"]
    pub unsafe fn set_info(
        selector: InfoSelector,
        info: *mut ffi::c_void,
        size: u32,
        options: u32,
    ) -> ffi::c_bool;
}
