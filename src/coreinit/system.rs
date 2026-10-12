//! System
//!
//! Boot and power state, shutdown, process IDs and system health.

use super::memory::MemoryType;
use crate::ffi;

/// UPID
///
/// Process ID of a system process or the game. Also used as PFID.
#[doc(alias("UPID", "PFID", "SysAppPFID"))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Upid(pub u32);

impl Upid {
    pub const ERROR: Self = Self(u32::MAX);
    pub const KERNEL: Self = Self(0);
    pub const ROOT: Self = Self(1);
    pub const CAFE_MENU: Self = Self(2);
    pub const CAFE_TV: Self = Self(3);
    pub const EMANUAL: Self = Self(4);
    pub const HOME_BUTTON_MENU: Self = Self(5);
    pub const ERROR_DISPLAY: Self = Self(6);
    pub const MINI_MIIVERSE: Self = Self(7);
    pub const BROWSER: Self = Self(8);
    pub const MIIVERSE: Self = Self(9);
    pub const ESHOP: Self = Self(10);
    pub const FLV: Self = Self(11);
    pub const DOWNLOAD_MANAGER: Self = Self(12);
    pub const GAME: Self = Self(15);
    pub const MINT_U: Self = Self(16);
    pub const CABINET_U: Self = Self(17);
    pub const TEST_OVERLAY: Self = Self(31);

    pub const INVALID: Self = Self::ERROR;
    pub const WII_U_MENU: Self = Self::CAFE_MENU;
    pub const TVII: Self = Self::CAFE_TV;
    pub const HOME_MENU: Self = Self::HOME_BUTTON_MENU;
    pub const FRIEND_LIST: Self = Self::FLV;
    pub const DOWNLOAD_MANAGEMENT: Self = Self::DOWNLOAD_MANAGER;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyRequest {
    Exit = 1,
    NotifyNetIoStart = 2,
    NotifyNetIoEnd = 3,
    NotifyHomeButtonDenied = 4,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShutdownReason(pub u32);

impl ShutdownReason {
    pub const NO_SHUTDOWN: Self = Self(0);
    pub const POWER_OFF: Self = Self(1);
    pub const RESTART: Self = Self(2);
    pub const STANDBY: Self = Self(3);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShutdownFlags(pub u32);

impl ShutdownFlags {
    pub const NONE: Self = Self(0);
    /// Shutdown requested by auto power-down.
    pub const APD_REQUEST: Self = Self(1 << 0);
}

/// SH
///
/// Components of [SysHealth].
#[doc(alias = "SH")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SysHealthComponents(pub u32);

impl SysHealthComponents {
    pub const ALARM: Self = Self(1 << 0);
    pub const THREAD: Self = Self(1 << 1);
    pub const MEMORY: Self = Self(1 << 2);
    pub const SYSLOG: Self = Self(1 << 3);
    pub const FG_BG_TRANSITION: Self = Self(1 << 4);
    pub const NETWORK: Self = Self(1 << 5);
    pub const FILE_SYSTEM: Self = Self(1 << 6);
    pub const GRAPHIC: Self = Self(1 << 7);
    pub const ALL: Self = Self(0xFF);
}

impl core::ops::BitOr for SysHealthComponents {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Each component pointer is null unless requested in `component_mask`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SysHealth {
    pub alloc_size: u32,
    pub component_mask: SysHealthComponents,
    pub alarm: *mut ffi::c_void,
    pub thread: *mut ffi::c_void,
    pub memory: *mut ffi::c_void,
    pub syslog: *mut ffi::c_void,
    pub fg_bg_transition: *mut ffi::c_void,
    pub network: *mut ffi::c_void,
    pub file_system: *mut ffi::c_void,
    pub graphic: *mut ffi::c_void,
}

unsafe extern "C" {
    /// OSIsColdBoot
    #[doc(alias = "OSIsColdBoot")]
    #[link_name = "OSIsColdBoot"]
    pub unsafe fn is_cold_boot() -> ffi::c_bool;

    /// OSIsNormalBoot
    #[doc(alias = "OSIsNormalBoot")]
    #[link_name = "OSIsNormalBoot"]
    pub unsafe fn is_normal_boot() -> ffi::c_bool;

    /// OSIsStandbyBoot
    #[doc(alias = "OSIsStandbyBoot")]
    #[link_name = "OSIsStandbyBoot"]
    pub unsafe fn is_standby_boot() -> ffi::c_bool;

    /// OSIsSelfRefreshBoot
    #[doc(alias = "OSIsSelfRefreshBoot")]
    #[link_name = "OSIsSelfRefreshBoot"]
    pub unsafe fn is_self_refresh_boot() -> ffi::c_bool;

    /// OSIsOffBoot
    #[doc(alias = "OSIsOffBoot")]
    #[link_name = "OSIsOffBoot"]
    pub unsafe fn is_off_boot() -> ffi::c_bool;

    /// OSIsCompatBoot
    #[doc(alias = "OSIsCompatBoot")]
    #[link_name = "OSIsCompatBoot"]
    pub unsafe fn is_compat_boot() -> ffi::c_bool;

    /// OSGetBootPMFlags
    #[doc(alias = "OSGetBootPMFlags")]
    #[link_name = "OSGetBootPMFlags"]
    pub unsafe fn get_boot_pm_flags() -> u32;

    /// OSGetLastPMState
    #[doc(alias = "OSGetLastPMState")]
    #[link_name = "OSGetLastPMState"]
    pub unsafe fn get_last_pm_state() -> u32;

    /// OSGetCurrentPMState
    #[doc(alias = "OSGetCurrentPMState")]
    #[link_name = "OSGetCurrentPMState"]
    pub unsafe fn get_current_pm_state() -> u32;

    /// OSIsProdMode
    #[doc(alias = "OSIsProdMode")]
    #[link_name = "OSIsProdMode"]
    pub unsafe fn is_prod_mode() -> ffi::c_bool;

    /// OSIsMainCore
    #[doc(alias = "OSIsMainCore")]
    #[link_name = "OSIsMainCore"]
    pub unsafe fn is_main_core() -> ffi::c_bool;

    /// OSGetMemBound
    ///
    /// Start and size of the memory area of `kind` available to the application. Returns 0 on success.
    #[doc(alias = "OSGetMemBound")]
    #[link_name = "OSGetMemBound"]
    pub unsafe fn get_mem_bound(kind: MemoryType, start: *mut u32, size: *mut u32) -> ffi::c_int;

    /// OSGetShutdownReason
    #[doc(alias = "OSGetShutdownReason")]
    #[link_name = "OSGetShutdownReason"]
    pub unsafe fn get_shutdown_reason() -> ShutdownReason;

    /// OSGetShutdownFlags
    #[doc(alias = "OSGetShutdownFlags")]
    #[link_name = "OSGetShutdownFlags"]
    pub unsafe fn get_shutdown_flags() -> ShutdownFlags;

    /// OSBlockThreadsOnExit
    #[doc(alias = "OSBlockThreadsOnExit")]
    #[link_name = "OSBlockThreadsOnExit"]
    pub unsafe fn block_threads_on_exit();

    /// OSIsBlockThreadsOnExit
    #[doc(alias = "OSIsBlockThreadsOnExit")]
    #[link_name = "OSIsBlockThreadsOnExit"]
    pub unsafe fn is_block_threads_on_exit() -> ffi::c_bool;

    /// OSRunThreadsOnExit
    #[doc(alias = "OSRunThreadsOnExit")]
    #[link_name = "OSRunThreadsOnExit"]
    pub unsafe fn run_threads_on_exit();

    /// OSRequestFastExit
    #[doc(alias = "OSRequestFastExit")]
    #[link_name = "OSRequestFastExit"]
    pub unsafe fn request_fast_exit(flags: u32, run_exit: ffi::c_bool) -> ffi::c_bool;

    /// OSSendPolicyRequest
    #[doc(alias = "OSSendPolicyRequest")]
    #[link_name = "OSSendPolicyRequest"]
    pub unsafe fn send_policy_request(policy: PolicyRequest, target: Upid) -> ffi::c_bool;

    /// OSAllocSysHealth
    #[doc(alias = "OSAllocSysHealth")]
    #[link_name = "OSAllocSysHealth"]
    pub unsafe fn alloc_sys_health(components: SysHealthComponents) -> *mut SysHealth;

    /// OSFreeSysHealth
    #[doc(alias = "OSFreeSysHealth")]
    #[link_name = "OSFreeSysHealth"]
    pub unsafe fn free_sys_health(health: *mut SysHealth);

    /// OSGetSysHealth
    #[doc(alias = "OSGetSysHealth")]
    #[link_name = "OSGetSysHealth"]
    pub unsafe fn get_sys_health(health: *mut SysHealth) -> ffi::c_bool;
}
