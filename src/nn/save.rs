//! Save Data Management (SAVE)
//!
//! Save data access. The functions mirror the FS API, with paths relative to the save directory of an account slot.

use super::{acp::DeviceType, act::SlotNo};
use crate::coreinit::fs;
use crate::ffi;

/// SAVEStatus
#[doc(alias = "SAVEStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const OK: Self = Self(0);
    pub const NOT_FOUND: Self = Self(-6);
    pub const STORAGE_FULL: Self = Self(-12);
    pub const FATAL_ERROR: Self = Self(-1024);
    pub const BUFFER_TOO_SHORT: Self = Self(-1025);
}

unsafe extern "C" {
    /// SAVEChangeDir
    #[doc(alias = "SAVEChangeDir")]
    #[link_name = "SAVEChangeDir"]
    pub unsafe fn change_dir(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEChangeDirAsync
    #[doc(alias = "SAVEChangeDirAsync")]
    #[link_name = "SAVEChangeDirAsync"]
    pub unsafe fn change_dir_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEChangeGroupAndOthersMode
    #[doc(alias = "SAVEChangeGroupAndOthersMode")]
    #[link_name = "SAVEChangeGroupAndOthersMode"]
    pub unsafe fn change_group_and_others_mode(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: fs::Mode,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEChangeGroupAndOthersModeAsync
    #[doc(alias = "SAVEChangeGroupAndOthersModeAsync")]
    #[link_name = "SAVEChangeGroupAndOthersModeAsync"]
    pub unsafe fn change_group_and_others_mode_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: fs::Mode,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEChangeGroupMode
    #[doc(alias = "SAVEChangeGroupMode")]
    #[link_name = "SAVEChangeGroupMode"]
    pub unsafe fn change_group_mode(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: fs::Mode,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEChangeGroupModeAsync
    #[doc(alias = "SAVEChangeGroupModeAsync")]
    #[link_name = "SAVEChangeGroupModeAsync"]
    pub unsafe fn change_group_mode_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: fs::Mode,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVECheckSaveDirRequiredUpdate
    #[doc(alias = "SAVECheckSaveDirRequiredUpdate")]
    #[link_name = "SAVECheckSaveDirRequiredUpdate"]
    pub unsafe fn check_save_dir_required_update(unk1: *mut ffi::c_int, unk2: *mut u64) -> Status;

    /// SAVEFlushQuota
    #[doc(alias = "SAVEFlushQuota")]
    #[link_name = "SAVEFlushQuota"]
    pub unsafe fn flush_quota(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEFlushQuotaAsync
    #[doc(alias = "SAVEFlushQuotaAsync")]
    #[link_name = "SAVEFlushQuotaAsync"]
    pub unsafe fn flush_quota_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetFreeSpaceSize
    #[doc(alias = "SAVEGetFreeSpaceSize")]
    #[link_name = "SAVEGetFreeSpaceSize"]
    pub unsafe fn get_free_space_size(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        size: *mut u64,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetFreeSpaceSizeAsync
    #[doc(alias = "SAVEGetFreeSpaceSizeAsync")]
    #[link_name = "SAVEGetFreeSpaceSizeAsync"]
    pub unsafe fn get_free_space_size_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        size: *mut u64,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetFreeSpaceSizeOfDevice
    #[doc(alias = "SAVEGetFreeSpaceSizeOfDevice")]
    #[link_name = "SAVEGetFreeSpaceSizeOfDevice"]
    pub unsafe fn get_free_space_size_of_device(device: DeviceType) -> Status;

    /// SAVEGetNoDeleteGroupSaveDirPath
    #[doc(alias = "SAVEGetNoDeleteGroupSaveDirPath")]
    #[link_name = "SAVEGetNoDeleteGroupSaveDirPath"]
    pub unsafe fn get_no_delete_group_save_dir_path(
        buffer: *mut ffi::c_char,
        buffer_size: u32,
    ) -> Status;

    /// SAVEGetNoDeleteSaveDataPath
    #[doc(alias = "SAVEGetNoDeleteSaveDataPath")]
    #[link_name = "SAVEGetNoDeleteSaveDataPath"]
    pub unsafe fn get_no_delete_save_data_path(
        title_id: u64,
        path: *const ffi::c_char,
        buffer: *mut ffi::c_char,
        buffer_size: u32,
    ) -> Status;

    /// SAVEGetSharedDataTitlePath
    #[doc(alias = "SAVEGetSharedDataTitlePath")]
    #[link_name = "SAVEGetSharedDataTitlePath"]
    pub unsafe fn get_shared_data_title_path(
        title_id: u64,
        path: *const ffi::c_char,
        buffer: *mut ffi::c_char,
        buffer_size: u32,
    ) -> Status;

    /// SAVEGetSharedSaveDataPath
    #[doc(alias = "SAVEGetSharedSaveDataPath")]
    #[link_name = "SAVEGetSharedSaveDataPath"]
    pub unsafe fn get_shared_save_data_path(
        title_id: u64,
        path: *const ffi::c_char,
        buffer: *mut ffi::c_char,
        buffer_size: u32,
    ) -> Status;

    /// SAVEGetStat
    #[doc(alias = "SAVEGetStat")]
    #[link_name = "SAVEGetStat"]
    pub unsafe fn get_stat(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatAsync
    #[doc(alias = "SAVEGetStatAsync")]
    #[link_name = "SAVEGetStatAsync"]
    pub unsafe fn get_stat_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetStatOtherApplication
    #[doc(alias = "SAVEGetStatOtherApplication")]
    #[link_name = "SAVEGetStatOtherApplication"]
    pub unsafe fn get_stat_other_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatOtherApplicationAsync
    #[doc(alias = "SAVEGetStatOtherApplicationAsync")]
    #[link_name = "SAVEGetStatOtherApplicationAsync"]
    pub unsafe fn get_stat_other_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetStatOtherDemoApplication
    #[doc(alias = "SAVEGetStatOtherDemoApplication")]
    #[link_name = "SAVEGetStatOtherDemoApplication"]
    pub unsafe fn get_stat_other_demo_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatOtherDemoApplicationAsync
    #[doc(alias = "SAVEGetStatOtherDemoApplicationAsync")]
    #[link_name = "SAVEGetStatOtherDemoApplicationAsync"]
    pub unsafe fn get_stat_other_demo_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetStatOtherDemoApplicationVariation
    #[doc(alias = "SAVEGetStatOtherDemoApplicationVariation")]
    #[link_name = "SAVEGetStatOtherDemoApplicationVariation"]
    pub unsafe fn get_stat_other_demo_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatOtherDemoApplicationVariationAsync
    #[doc(alias = "SAVEGetStatOtherDemoApplicationVariationAsync")]
    #[link_name = "SAVEGetStatOtherDemoApplicationVariationAsync"]
    pub unsafe fn get_stat_other_demo_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetStatOtherNormalApplication
    #[doc(alias = "SAVEGetStatOtherNormalApplication")]
    #[link_name = "SAVEGetStatOtherNormalApplication"]
    pub unsafe fn get_stat_other_normal_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatOtherNormalApplicationAsync
    #[doc(alias = "SAVEGetStatOtherNormalApplicationAsync")]
    #[link_name = "SAVEGetStatOtherNormalApplicationAsync"]
    pub unsafe fn get_stat_other_normal_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEGetStatOtherNormalApplicationVariation
    #[doc(alias = "SAVEGetStatOtherNormalApplicationVariation")]
    #[link_name = "SAVEGetStatOtherNormalApplicationVariation"]
    pub unsafe fn get_stat_other_normal_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEGetStatOtherNormalApplicationVariationAsync
    #[doc(alias = "SAVEGetStatOtherNormalApplicationVariationAsync")]
    #[link_name = "SAVEGetStatOtherNormalApplicationVariationAsync"]
    pub unsafe fn get_stat_other_normal_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        stat: *mut fs::Stat,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEInit
    #[doc(alias = "SAVEInit")]
    #[link_name = "SAVEInit"]
    pub unsafe fn init() -> Status;

    /// SAVEInitAccountSaveDir
    #[doc(alias = "SAVEInitAccountSaveDir")]
    #[link_name = "SAVEInitAccountSaveDir"]
    pub unsafe fn init_account_save_dir(slot: SlotNo) -> Status;

    /// SAVEInitCommonSaveDir
    #[doc(alias = "SAVEInitCommonSaveDir")]
    #[link_name = "SAVEInitCommonSaveDir"]
    pub unsafe fn init_common_save_dir() -> Status;

    /// SAVEInitNoDeleteGroupSaveDir
    #[doc(alias = "SAVEInitNoDeleteGroupSaveDir")]
    #[link_name = "SAVEInitNoDeleteGroupSaveDir"]
    pub unsafe fn init_no_delete_group_save_dir();

    /// SAVEInitSaveDir
    #[doc(alias = "SAVEInitSaveDir")]
    #[link_name = "SAVEInitSaveDir"]
    pub unsafe fn init_save_dir(slot: SlotNo) -> Status;

    /// SAVEInitSaveDirByAppMeta
    #[doc(alias = "SAVEInitSaveDirByAppMeta")]
    #[link_name = "SAVEInitSaveDirByAppMeta"]
    pub unsafe fn init_save_dir_by_app_meta(
        slot: SlotNo,
        meta_xml_path: *const ffi::c_char,
        meta_xml_buffer: *mut u8,
        meta_xml_buffer_size: u32,
        icon_path: *const ffi::c_char,
        icon_buffer: *mut u8,
        icon_buffer_size: u32,
    ) -> Status;

    /// SAVEMakeDir
    #[doc(alias = "SAVEMakeDir")]
    #[link_name = "SAVEMakeDir"]
    pub unsafe fn make_dir(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEMakeDirAsync
    #[doc(alias = "SAVEMakeDirAsync")]
    #[link_name = "SAVEMakeDirAsync"]
    pub unsafe fn make_dir_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDir
    #[doc(alias = "SAVEOpenDir")]
    #[link_name = "SAVEOpenDir"]
    pub unsafe fn open_dir(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirAsync
    #[doc(alias = "SAVEOpenDirAsync")]
    #[link_name = "SAVEOpenDirAsync"]
    pub unsafe fn open_dir_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDirOtherApplication
    #[doc(alias = "SAVEOpenDirOtherApplication")]
    #[link_name = "SAVEOpenDirOtherApplication"]
    pub unsafe fn open_dir_other_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirOtherApplicationAsync
    #[doc(alias = "SAVEOpenDirOtherApplicationAsync")]
    #[link_name = "SAVEOpenDirOtherApplicationAsync"]
    pub unsafe fn open_dir_other_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDirOtherDemoApplication
    #[doc(alias = "SAVEOpenDirOtherDemoApplication")]
    #[link_name = "SAVEOpenDirOtherDemoApplication"]
    pub unsafe fn open_dir_other_demo_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirOtherDemoApplicationAsync
    #[doc(alias = "SAVEOpenDirOtherDemoApplicationAsync")]
    #[link_name = "SAVEOpenDirOtherDemoApplicationAsync"]
    pub unsafe fn open_dir_other_demo_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDirOtherDemoApplicationVariation
    #[doc(alias = "SAVEOpenDirOtherDemoApplicationVariation")]
    #[link_name = "SAVEOpenDirOtherDemoApplicationVariation"]
    pub unsafe fn open_dir_other_demo_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirOtherDemoApplicationVariationAsync
    #[doc(alias = "SAVEOpenDirOtherDemoApplicationVariationAsync")]
    #[link_name = "SAVEOpenDirOtherDemoApplicationVariationAsync"]
    pub unsafe fn open_dir_other_demo_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDirOtherNormalApplication
    #[doc(alias = "SAVEOpenDirOtherNormalApplication")]
    #[link_name = "SAVEOpenDirOtherNormalApplication"]
    pub unsafe fn open_dir_other_normal_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirOtherNormalApplicationAsync
    #[doc(alias = "SAVEOpenDirOtherNormalApplicationAsync")]
    #[link_name = "SAVEOpenDirOtherNormalApplicationAsync"]
    pub unsafe fn open_dir_other_normal_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenDirOtherNormalApplicationVariation
    #[doc(alias = "SAVEOpenDirOtherNormalApplicationVariation")]
    #[link_name = "SAVEOpenDirOtherNormalApplicationVariation"]
    pub unsafe fn open_dir_other_normal_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenDirOtherNormalApplicationVariationAsync
    #[doc(alias = "SAVEOpenDirOtherNormalApplicationVariationAsync")]
    #[link_name = "SAVEOpenDirOtherNormalApplicationVariationAsync"]
    pub unsafe fn open_dir_other_normal_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        handle: *mut fs::DirHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFile
    #[doc(alias = "SAVEOpenFile")]
    #[link_name = "SAVEOpenFile"]
    pub unsafe fn open_file(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileAsync
    #[doc(alias = "SAVEOpenFileAsync")]
    #[link_name = "SAVEOpenFileAsync"]
    pub unsafe fn open_file_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFileOtherApplication
    #[doc(alias = "SAVEOpenFileOtherApplication")]
    #[link_name = "SAVEOpenFileOtherApplication"]
    pub unsafe fn open_file_other_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileOtherApplicationAsync
    #[doc(alias = "SAVEOpenFileOtherApplicationAsync")]
    #[link_name = "SAVEOpenFileOtherApplicationAsync"]
    pub unsafe fn open_file_other_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        title_id: u64,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFileOtherDemoApplication
    #[doc(alias = "SAVEOpenFileOtherDemoApplication")]
    #[link_name = "SAVEOpenFileOtherDemoApplication"]
    pub unsafe fn open_file_other_demo_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileOtherDemoApplicationAsync
    #[doc(alias = "SAVEOpenFileOtherDemoApplicationAsync")]
    #[link_name = "SAVEOpenFileOtherDemoApplicationAsync"]
    pub unsafe fn open_file_other_demo_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFileOtherDemoApplicationVariation
    #[doc(alias = "SAVEOpenFileOtherDemoApplicationVariation")]
    #[link_name = "SAVEOpenFileOtherDemoApplicationVariation"]
    pub unsafe fn open_file_other_demo_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileOtherDemoApplicationVariationAsync
    #[doc(alias = "SAVEOpenFileOtherDemoApplicationVariationAsync")]
    #[link_name = "SAVEOpenFileOtherDemoApplicationVariationAsync"]
    pub unsafe fn open_file_other_demo_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFileOtherNormalApplication
    #[doc(alias = "SAVEOpenFileOtherNormalApplication")]
    #[link_name = "SAVEOpenFileOtherNormalApplication"]
    pub unsafe fn open_file_other_normal_application(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileOtherNormalApplicationAsync
    #[doc(alias = "SAVEOpenFileOtherNormalApplicationAsync")]
    #[link_name = "SAVEOpenFileOtherNormalApplicationAsync"]
    pub unsafe fn open_file_other_normal_application_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEOpenFileOtherNormalApplicationVariation
    #[doc(alias = "SAVEOpenFileOtherNormalApplicationVariation")]
    #[link_name = "SAVEOpenFileOtherNormalApplicationVariation"]
    pub unsafe fn open_file_other_normal_application_variation(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVEOpenFileOtherNormalApplicationVariationAsync
    #[doc(alias = "SAVEOpenFileOtherNormalApplicationVariationAsync")]
    #[link_name = "SAVEOpenFileOtherNormalApplicationVariationAsync"]
    pub unsafe fn open_file_other_normal_application_variation_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        unique_id: u32,
        variation: u8,
        slot: SlotNo,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut fs::FileHandle,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVERemove
    #[doc(alias = "SAVERemove")]
    #[link_name = "SAVERemove"]
    pub unsafe fn remove(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVERemoveAsync
    #[doc(alias = "SAVERemoveAsync")]
    #[link_name = "SAVERemoveAsync"]
    pub unsafe fn remove_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVERename
    #[doc(alias = "SAVERename")]
    #[link_name = "SAVERename"]
    pub unsafe fn rename(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVERenameAsync
    #[doc(alias = "SAVERenameAsync")]
    #[link_name = "SAVERenameAsync"]
    pub unsafe fn rename_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVERollbackQuota
    #[doc(alias = "SAVERollbackQuota")]
    #[link_name = "SAVERollbackQuota"]
    pub unsafe fn rollback_quota(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        error_mask: fs::ErrorFlag,
    ) -> fs::Status;

    /// SAVERollbackQuotaAsync
    #[doc(alias = "SAVERollbackQuotaAsync")]
    #[link_name = "SAVERollbackQuotaAsync"]
    pub unsafe fn rollback_quota_async(
        client: *mut fs::Client,
        block: *mut fs::CmdBlock,
        slot: SlotNo,
        error_mask: fs::ErrorFlag,
        async_data: *const fs::AsyncData,
    ) -> fs::Status;

    /// SAVEShutdown
    #[doc(alias = "SAVEShutdown")]
    #[link_name = "SAVEShutdown"]
    pub unsafe fn deinit();

    /// SAVEUpdateSaveDir
    #[doc(alias = "SAVEUpdateSaveDir")]
    #[link_name = "SAVEUpdateSaveDir"]
    pub unsafe fn update_save_dir() -> Status;
}

imports_section!(
    "nn_save",
    [
        "SAVEChangeDir",
        "SAVEChangeDirAsync",
        "SAVEChangeGroupAndOthersMode",
        "SAVEChangeGroupAndOthersModeAsync",
        "SAVEChangeGroupMode",
        "SAVEChangeGroupModeAsync",
        "SAVECheckSaveDirRequiredUpdate",
        "SAVEFlushQuota",
        "SAVEFlushQuotaAsync",
        "SAVEGetFreeSpaceSize",
        "SAVEGetFreeSpaceSizeAsync",
        "SAVEGetFreeSpaceSizeOfDevice",
        "SAVEGetNoDeleteGroupSaveDirPath",
        "SAVEGetNoDeleteSaveDataPath",
        "SAVEGetSharedDataTitlePath",
        "SAVEGetSharedSaveDataPath",
        "SAVEGetStat",
        "SAVEGetStatAsync",
        "SAVEGetStatOtherApplication",
        "SAVEGetStatOtherApplicationAsync",
        "SAVEGetStatOtherDemoApplication",
        "SAVEGetStatOtherDemoApplicationAsync",
        "SAVEGetStatOtherDemoApplicationVariation",
        "SAVEGetStatOtherDemoApplicationVariationAsync",
        "SAVEGetStatOtherNormalApplication",
        "SAVEGetStatOtherNormalApplicationAsync",
        "SAVEGetStatOtherNormalApplicationVariation",
        "SAVEGetStatOtherNormalApplicationVariationAsync",
        "SAVEInit",
        "SAVEInitAccountSaveDir",
        "SAVEInitCommonSaveDir",
        "SAVEInitNoDeleteGroupSaveDir",
        "SAVEInitSaveDir",
        "SAVEInitSaveDirByAppMeta",
        "SAVEMakeDir",
        "SAVEMakeDirAsync",
        "SAVEOpenDir",
        "SAVEOpenDirAsync",
        "SAVEOpenDirOtherApplication",
        "SAVEOpenDirOtherApplicationAsync",
        "SAVEOpenDirOtherDemoApplication",
        "SAVEOpenDirOtherDemoApplicationAsync",
        "SAVEOpenDirOtherDemoApplicationVariation",
        "SAVEOpenDirOtherDemoApplicationVariationAsync",
        "SAVEOpenDirOtherNormalApplication",
        "SAVEOpenDirOtherNormalApplicationAsync",
        "SAVEOpenDirOtherNormalApplicationVariation",
        "SAVEOpenDirOtherNormalApplicationVariationAsync",
        "SAVEOpenFile",
        "SAVEOpenFileAsync",
        "SAVEOpenFileOtherApplication",
        "SAVEOpenFileOtherApplicationAsync",
        "SAVEOpenFileOtherDemoApplication",
        "SAVEOpenFileOtherDemoApplicationAsync",
        "SAVEOpenFileOtherDemoApplicationVariation",
        "SAVEOpenFileOtherDemoApplicationVariationAsync",
        "SAVEOpenFileOtherNormalApplication",
        "SAVEOpenFileOtherNormalApplicationAsync",
        "SAVEOpenFileOtherNormalApplicationVariation",
        "SAVEOpenFileOtherNormalApplicationVariationAsync",
        "SAVERemove",
        "SAVERemoveAsync",
        "SAVERename",
        "SAVERenameAsync",
        "SAVERollbackQuota",
        "SAVERollbackQuotaAsync",
        "SAVEShutdown",
        "SAVEUpdateSaveDir"
    ],
    []
);
