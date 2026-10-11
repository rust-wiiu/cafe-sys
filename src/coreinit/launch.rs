//! Launch
//!
//! Launching titles and the arguments passed between processes.

use super::system::Upid;
use crate::ffi;

pub const ARGS_SIZE: u32 = 4096;
pub const MAX_NUM_ARGS: u32 = 32;

unsafe extern "C" {
    /// OSLaunchTitlev
    #[doc(alias = "OSLaunchTitlev")]
    #[link_name = "OSLaunchTitlev"]
    pub unsafe fn launch_title_v(
        title_id: u64,
        argc: u32,
        argv: *mut *mut ffi::c_char,
    ) -> ffi::c_int;

    /// OSLaunchTitlel
    ///
    /// Same as [launch_title_v] with `argc` string arguments.
    #[doc(alias = "OSLaunchTitlel")]
    #[link_name = "OSLaunchTitlel"]
    pub unsafe fn launch_title_l(title_id: u64, argc: u32, ...) -> ffi::c_int;

    /// OSLaunchTitleByPathv
    #[doc(alias = "OSLaunchTitleByPathv")]
    #[link_name = "OSLaunchTitleByPathv"]
    pub unsafe fn launch_title_by_path_v(
        path: *const ffi::c_char,
        path_size: u32,
        argc: u32,
        argv: *mut *mut ffi::c_char,
    ) -> ffi::c_int;

    /// OSLaunchTitleByPathl
    #[doc(alias = "OSLaunchTitleByPathl")]
    #[link_name = "OSLaunchTitleByPathl"]
    pub unsafe fn launch_title_by_path_l(
        path: *const ffi::c_char,
        path_size: u32,
        argc: u32,
        ...
    ) -> ffi::c_int;

    /// OSRestartGame
    #[doc(alias = "OSRestartGame")]
    #[link_name = "OSRestartGame"]
    #[deprecated(note = "Use sysapp::relaunch_title instead")]
    pub unsafe fn restart_game(argc: u32, argv: *mut *mut ffi::c_char) -> ffi::c_int;

    /// OSForceFullRelaunch
    #[doc(alias = "OSForceFullRelaunch")]
    #[link_name = "OSForceFullRelaunch"]
    pub unsafe fn force_full_relaunch();

    /// __OSSetLaunchTimeout
    ///
    /// Returns the previous timeout.
    #[doc(alias = "__OSSetLaunchTimeout")]
    #[link_name = "__OSSetLaunchTimeout"]
    pub unsafe fn set_launch_timeout(timeout_ms: u32) -> u32;

    /// OSShutdown
    #[doc(alias = "OSShutdown")]
    #[link_name = "OSShutdown"]
    pub unsafe fn shutdown() -> ffi::c_bool;

    /// OSGetArgcArgv
    #[doc(alias = "OSGetArgcArgv")]
    #[link_name = "OSGetArgcArgv"]
    pub unsafe fn get_argc_argv(argc: *mut ffi::c_int, argv: *mut *const *const ffi::c_char);

    /// OSGetCallArgs
    ///
    /// Arguments passed by the calling process.
    #[doc(alias = "OSGetCallArgs")]
    #[link_name = "OSGetCallArgs"]
    pub unsafe fn get_call_args(
        caller: *mut Upid,
        buffer: *mut u8,
        buffer_size: u32,
    ) -> ffi::c_bool;

    /// OSSendAppSwitchRequest
    #[doc(alias = "OSSendAppSwitchRequest")]
    #[link_name = "OSSendAppSwitchRequest"]
    #[deprecated(note = "Use sysapp instead")]
    pub unsafe fn send_app_switch_request(
        target: Upid,
        args: *mut ffi::c_void,
        args_size: u32,
    ) -> ffi::c_bool;
}
