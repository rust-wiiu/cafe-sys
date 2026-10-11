//! vWii compatibility (CMPT)

use crate::ffi;

/// CMPT error code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// CmptScreenType
#[doc(alias = "CmptScreenType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenType {
    Tv = 1,
    Drc = 2,
    Both = 3,
}

unsafe extern "C" {
    /// CMPTGetDataSize
    ///
    /// Size of the buffer needed by the launch functions. The buffer must be 64-byte aligned.
    #[doc(alias = "CMPTGetDataSize")]
    #[link_name = "CMPTGetDataSize"]
    pub unsafe fn get_data_size(size: *mut u32) -> Error;

    /// CMPTLaunchTitle
    ///
    /// Launches a vWii title.
    #[doc(alias = "CMPTLaunchTitle")]
    #[link_name = "CMPTLaunchTitle"]
    pub unsafe fn launch_title(buf: *mut ffi::c_void, buf_size: u32, title_id: u64) -> Error;

    /// CMPTLaunchMenu
    ///
    /// Launches the vWii System Menu.
    #[doc(alias = "CMPTLaunchMenu")]
    #[link_name = "CMPTLaunchMenu"]
    pub unsafe fn launch_menu(buf: *mut ffi::c_void, buf_size: u32) -> Error;

    /// CMPTLaunchDataManager
    ///
    /// Launches the vWii Data Management.
    #[doc(alias = "CMPTLaunchDataManager")]
    #[link_name = "CMPTLaunchDataManager"]
    pub unsafe fn launch_data_manager(buf: *mut ffi::c_void, buf_size: u32) -> Error;

    /// CMPTAcctSetScreenType
    #[doc(alias = "CMPTAcctSetScreenType")]
    #[link_name = "CMPTAcctSetScreenType"]
    pub unsafe fn acct_set_screen_type(screen: ScreenType) -> Error;

    /// CMPTCheckScreenState
    ///
    /// Checks whether the current screen configuration is valid.
    #[doc(alias = "CMPTCheckScreenState")]
    #[link_name = "CMPTCheckScreenState"]
    pub unsafe fn check_screen_state() -> Error;
}

imports_section!(
    "nn_cmpt",
    [
        "CMPTAcctClearInternalState",
        "CMPTAcctGetDrcCtrlEnabled",
        "CMPTAcctGetEula",
        "CMPTAcctGetPcConf",
        "CMPTAcctGetProfile",
        "CMPTAcctGetScreenType",
        "CMPTAcctSetDrcCtrlEnabled",
        "CMPTAcctSetEula",
        "CMPTAcctSetEulaBySlotNo",
        "CMPTAcctSetPcConf",
        "CMPTAcctSetProfile",
        "CMPTAcctSetScreenType",
        "CMPTCheckScreenState",
        "CMPTExLaunch",
        "CMPTExPrepareLaunch",
        "CMPTExPrepareLaunchTest",
        "CMPTExSetWorkBuffer",
        "CMPTGetDataSize",
        "CMPTGetSystemVersion",
        "CMPTInitSystem",
        "CMPTIsClean",
        "CMPTIsPlayableDisc",
        "CMPTLaunchDataManager",
        "CMPTLaunchMenu",
        "CMPTLaunchTest",
        "CMPTLaunchTitle",
        "CMPTSetDirty"
    ],
    []
);
