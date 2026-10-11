//! Temporary Folders (TEMP)
//!
//! Scratch directories on internal or USB storage. The functions mirror the FS API, with paths relative to a TEMP directory.

use super::save::{
    FsAsyncData, FsClient, FsCmdBlock, FsDirHandle, FsErrorFlag, FsFileHandle, FsMode, FsStat,
};
use crate::ffi;

/// TEMPStatus
#[doc(alias = "TEMPStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const OK: Self = Self(0);
    pub const CANCELED: Self = Self(-1);
    pub const END: Self = Self(-2);
    pub const MAX: Self = Self(-3);
    pub const ALREADY_OPEN: Self = Self(-4);
    pub const EXISTS: Self = Self(-5);
    pub const NOT_FOUND: Self = Self(-6);
    pub const NOT_FILE: Self = Self(-7);
    pub const NOT_DIR: Self = Self(-8);
    pub const ACCESS_ERROR: Self = Self(-9);
    pub const PERMISSION_ERROR: Self = Self(-10);
    pub const FILE_TOO_BIG: Self = Self(-11);
    pub const STORAGE_FULL: Self = Self(-12);
    pub const JOURNAL_FULL: Self = Self(-13);
    pub const UNSUPPORTED_CMD: Self = Self(-14);
    pub const FATAL_ERROR: Self = Self(-1024);
    pub const INVALID_PARAM: Self = Self(-1025);
}

/// TEMPDirId
#[doc(alias = "TEMPDirId")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirId(pub u64);

impl DirId {
    pub const INVALID: Self = Self(0);
}

/// TEMPTargetPreference
///
/// Which device to create the TEMP directory on.
#[doc(alias("TEMPTargetPreference", "TEMPDevicePreference"))]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevicePreference {
    /// The device with the most free space.
    #[default]
    FreeSpace = 0,
    /// The device with the fastest access.
    AccessSpeed = 1,
}

unsafe extern "C" {
    /// TEMPChangeDir
    #[doc(alias = "TEMPChangeDir")]
    #[link_name = "TEMPChangeDir"]
    pub unsafe fn change_dir(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPChangeDirAsync
    #[doc(alias = "TEMPChangeDirAsync")]
    #[link_name = "TEMPChangeDirAsync"]
    pub unsafe fn change_dir_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPChangeOthersMode
    #[doc(alias = "TEMPChangeOthersMode")]
    #[link_name = "TEMPChangeOthersMode"]
    pub unsafe fn change_others_mode(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: FsMode,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPChangeOthersModeAsync
    #[doc(alias = "TEMPChangeOthersModeAsync")]
    #[link_name = "TEMPChangeOthersModeAsync"]
    pub unsafe fn change_others_mode_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: FsMode,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPCreateAndInitTempDir
    ///
    /// `max_size` is in bytes.
    #[doc(alias = "TEMPCreateAndInitTempDir")]
    #[link_name = "TEMPCreateAndInitTempDir"]
    pub unsafe fn create_and_init_temp_dir(
        max_size: u32,
        preference: DevicePreference,
        dir: *mut DirId,
    ) -> Status;

    /// TEMPGetDirGlobalPath
    ///
    /// Absolute path of the TEMP directory.
    #[doc(alias = "TEMPGetDirGlobalPath")]
    #[link_name = "TEMPGetDirGlobalPath"]
    pub unsafe fn get_dir_global_path(dir: DirId, path: *mut ffi::c_char, path_len: u32) -> Status;

    /// TEMPGetDirPath
    ///
    /// Path of the TEMP directory, relative to the mount point.
    #[doc(alias = "TEMPGetDirPath")]
    #[link_name = "TEMPGetDirPath"]
    pub unsafe fn get_dir_path(dir: DirId, path: *mut ffi::c_char, path_len: u32) -> Status;

    /// TEMPGetFreeSpaceSize
    #[doc(alias = "TEMPGetFreeSpaceSize")]
    #[link_name = "TEMPGetFreeSpaceSize"]
    pub unsafe fn get_free_space_size(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        size: *mut u64,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPGetFreeSpaceSizeAsync
    #[doc(alias = "TEMPGetFreeSpaceSizeAsync")]
    #[link_name = "TEMPGetFreeSpaceSizeAsync"]
    pub unsafe fn get_free_space_size_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        size: *mut u64,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPGetStat
    #[doc(alias = "TEMPGetStat")]
    #[link_name = "TEMPGetStat"]
    pub unsafe fn get_stat(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        stat: *mut FsStat,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPGetStatAsync
    #[doc(alias = "TEMPGetStatAsync")]
    #[link_name = "TEMPGetStatAsync"]
    pub unsafe fn get_stat_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        stat: *mut FsStat,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPInit
    #[doc(alias = "TEMPInit")]
    #[link_name = "TEMPInit"]
    pub unsafe fn init() -> Status;

    /// TEMPMakeDir
    #[doc(alias = "TEMPMakeDir")]
    #[link_name = "TEMPMakeDir"]
    pub unsafe fn make_dir(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPMakeDirAsync
    #[doc(alias = "TEMPMakeDirAsync")]
    #[link_name = "TEMPMakeDirAsync"]
    pub unsafe fn make_dir_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPMountTempDir
    #[doc(alias = "TEMPMountTempDir")]
    #[link_name = "TEMPMountTempDir"]
    pub unsafe fn mount_temp_dir(dir: DirId) -> Status;

    /// TEMPOpenDir
    #[doc(alias = "TEMPOpenDir")]
    #[link_name = "TEMPOpenDir"]
    pub unsafe fn open_dir(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        handle: *mut FsDirHandle,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPOpenDirAsync
    #[doc(alias = "TEMPOpenDirAsync")]
    #[link_name = "TEMPOpenDirAsync"]
    pub unsafe fn open_dir_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        handle: *mut FsDirHandle,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPOpenFile
    #[doc(alias = "TEMPOpenFile")]
    #[link_name = "TEMPOpenFile"]
    pub unsafe fn open_file(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut FsFileHandle,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPOpenFileAsync
    #[doc(alias = "TEMPOpenFileAsync")]
    #[link_name = "TEMPOpenFileAsync"]
    pub unsafe fn open_file_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut FsFileHandle,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPOpenNewFile
    ///
    /// Creates a file with a preallocated `size`.
    #[doc(alias = "TEMPOpenNewFile")]
    #[link_name = "TEMPOpenNewFile"]
    pub unsafe fn open_new_file(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        size: u32,
        handle: *mut FsFileHandle,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPOpenNewFileAsync
    #[doc(alias = "TEMPOpenNewFileAsync")]
    #[link_name = "TEMPOpenNewFileAsync"]
    pub unsafe fn open_new_file_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        size: u32,
        handle: *mut FsFileHandle,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPRemove
    #[doc(alias = "TEMPRemove")]
    #[link_name = "TEMPRemove"]
    pub unsafe fn remove(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPRemoveAsync
    #[doc(alias = "TEMPRemoveAsync")]
    #[link_name = "TEMPRemoveAsync"]
    pub unsafe fn remove_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        path: *const ffi::c_char,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPRename
    #[doc(alias = "TEMPRename")]
    #[link_name = "TEMPRename"]
    pub unsafe fn rename(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: FsErrorFlag,
    ) -> Status;

    /// TEMPRenameAsync
    #[doc(alias = "TEMPRenameAsync")]
    #[link_name = "TEMPRenameAsync"]
    pub unsafe fn rename_async(
        client: *mut FsClient,
        block: *mut FsCmdBlock,
        dir: DirId,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: FsErrorFlag,
        async_data: *const FsAsyncData,
    ) -> Status;

    /// TEMPShutdown
    #[doc(alias = "TEMPShutdown")]
    #[link_name = "TEMPShutdown"]
    pub unsafe fn deinit();

    /// TEMPShutdownTempDir
    #[doc(alias = "TEMPShutdownTempDir")]
    #[link_name = "TEMPShutdownTempDir"]
    pub unsafe fn shutdown_temp_dir(dir: DirId) -> Status;

    /// TEMPUnmountTempDir
    #[doc(alias = "TEMPUnmountTempDir")]
    #[link_name = "TEMPUnmountTempDir"]
    pub unsafe fn unmount_temp_dir(dir: DirId) -> Status;
}

imports_section!(
    "nn_temp",
    [
        "TEMPChangeDir",
        "TEMPChangeDirAsync",
        "TEMPChangeOthersMode",
        "TEMPChangeOthersModeAsync",
        "TEMPCreateAndInitTempDir",
        "TEMPGetDirGlobalPath",
        "TEMPGetDirPath",
        "TEMPGetFreeSpaceSize",
        "TEMPGetFreeSpaceSizeAsync",
        "TEMPGetStat",
        "TEMPGetStatAsync",
        "TEMPInit",
        "TEMPMakeDir",
        "TEMPMakeDirAsync",
        "TEMPMountTempDir",
        "TEMPOpenDir",
        "TEMPOpenDirAsync",
        "TEMPOpenFile",
        "TEMPOpenFileAsync",
        "TEMPOpenNewFile",
        "TEMPOpenNewFileAsync",
        "TEMPRemove",
        "TEMPRemoveAsync",
        "TEMPRename",
        "TEMPRenameAsync",
        "TEMPShutdown",
        "TEMPShutdownTempDir",
        "TEMPUnmountTempDir"
    ],
    []
);
