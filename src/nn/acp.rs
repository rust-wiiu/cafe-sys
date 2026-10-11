//! Application Control Process

use crate::ffi;

// TODO: replace with MCPTitleListType from coreinit::mcp
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TitleListType(pub ffi::unknown<0x61>);

/// ACPResult
#[doc(alias = "ACPResult")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID: Self = Self(-200);
    pub const INVALID_PARAMETER: Self = Self(-201);
    pub const INVALID_FILE: Self = Self(-202);
    pub const INVALID_XML_FILE: Self = Self(-203);
    pub const FILE_ACCESS_MODE: Self = Self(-204);
    pub const INVALID_NETWORK_TIME: Self = Self(-205);
    pub const NOT_FOUND: Self = Self(-500);
    pub const FILE_NOT_FOUND: Self = Self(-501);
    pub const DIR_NOT_FOUND: Self = Self(-502);
    pub const DEVICE_NOT_FOUND: Self = Self(-503);
    pub const TITLE_NOT_FOUND: Self = Self(-504);
    pub const APPLICATION_NOT_FOUND: Self = Self(-505);
    pub const SYSTEM_CONFIG_NOT_FOUND: Self = Self(-506);
    pub const XML_ITEM_NOT_FOUND: Self = Self(-507);
    pub const ALREADY_EXISTS: Self = Self(-600);
    pub const FILE_ALREADY_EXISTS: Self = Self(-601);
    pub const DIR_ALREADY_EXISTS: Self = Self(-602);
    pub const ALREADY_DONE: Self = Self(-700);
    pub const AUTHENTICATION: Self = Self(-1000);
    pub const INVALID_REGION: Self = Self(-1001);
    pub const RESTRICTED_RATING: Self = Self(-1002);
    pub const NOT_PRESENT_RATING: Self = Self(-1003);
    pub const PENDING_RATING: Self = Self(-1004);
    pub const NET_SETTING_REQUIRED: Self = Self(-1005);
    pub const NET_ACCOUNT_REQUIRED: Self = Self(-1006);
    pub const NET_ACCOUNT_ERROR: Self = Self(-1007);
    pub const BROWSER_REQUIRED: Self = Self(-1008);
    pub const OLV_REQUIRED: Self = Self(-1009);
    pub const PINCODE_REQUIRED: Self = Self(-1010);
    pub const INCORRECT_PINCODE: Self = Self(-1011);
    pub const INVALID_LOGO: Self = Self(-1012);
    pub const DEMO_EXPIRED_NUMBER: Self = Self(-1013);
    pub const DRC_REQUIRED: Self = Self(-1014);
    pub const NO_PERMISSION: Self = Self(-1100);
    pub const NO_FILE_PERMISSION: Self = Self(-1101);
    pub const NO_DIR_PERMISSION: Self = Self(-1102);
    pub const BUSY: Self = Self(-1300);
    pub const USB_STORAGE_NOT_READY: Self = Self(-1301);
    pub const CANCELLED: Self = Self(-1400);
    pub const RESOURCE: Self = Self(-1500);
    pub const DEVICE_FULL: Self = Self(-1501);
    pub const JOURNAL_FULL: Self = Self(-1502);
    pub const SYSTEM_MEMORY: Self = Self(-1503);
    pub const FS_RESOURCE: Self = Self(-1504);
    pub const IPC_RESOURCE: Self = Self(-1505);
    pub const NOT_INITIALISED: Self = Self(-1600);
    pub const ACCOUNT_ERROR: Self = Self(-1700);
    pub const UNSUPPORTED: Self = Self(-1800);
    pub const DATA_CORRUPTED: Self = Self(-2000);
    pub const DEVICE: Self = Self(-2001);
    pub const SLC_DATA_CORRUPTED: Self = Self(-2002);
    pub const MLC_DATA_CORRUPTED: Self = Self(-2003);
    pub const USB_DATA_CORRUPTED: Self = Self(-2004);
    pub const MEDIA: Self = Self(-2100);
    pub const MEDIA_NOT_READY: Self = Self(-2101);
    pub const MEDIA_BROKEN: Self = Self(-2102);
    pub const ODD_MEDIA_NOT_READY: Self = Self(-2103);
    pub const ODD_MEDIA_BROKEN: Self = Self(-2104);
    pub const USB_MEDIA_NOT_READY: Self = Self(-2105);
    pub const USB_MEDIA_BROKEN: Self = Self(-2106);
    pub const MEDIA_WRITE_PROTECTED: Self = Self(-2107);
    pub const USB_WRITE_PROTECTED: Self = Self(-2108);
    pub const MII: Self = Self(-2200);
    pub const ENCRYPTION_ERROR: Self = Self(-2201);
    pub const GENERIC_ERROR: Self = Self(-4096);
}

/// ACPDeviceType
#[doc(alias = "ACPDeviceType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Auto = 1,
    /// Disc drive, or emulated disc (HFIO).
    Odd = 2,
    /// Internal storage, or emulated internal storage (HFIO).
    Mlc = 3,
    Usb = 4,
}

/// ACPSaveDirInfo
#[doc(alias = "ACPSaveDirInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SaveDirInfo {
    _unk0: ffi::unknown<0x8>,
    pub persistent_id: u32,
    _unk1: ffi::unknown<0x14>,
    pub path: [ffi::c_char; 0x40],
    _unk2: ffi::unknown<0x20>,
}

/// ACPMetaXml
#[doc(alias = "ACPMetaXml")]
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetaXml {
    pub title_id: u64,
    pub boss_id: u64,
    pub os_version: u64,
    pub app_size: u64,
    pub common_save_size: u64,
    pub account_save_size: u64,
    pub common_boss_size: u64,
    pub account_boss_size: u64,
    pub join_game_mode_mask: u64,
    pub version: u32,
    pub product_code: [ffi::c_char; 32],
    pub content_platform: [ffi::c_char; 32],
    pub company_code: [ffi::c_char; 8],
    pub mastering_date: [ffi::c_char; 32],
    pub logo_type: u32,
    pub app_launch_type: u32,
    pub invisible_flag: u32,
    pub no_managed_flag: u32,
    pub no_event_log: u32,
    pub no_icon_database: u32,
    pub launching_flag: u32,
    pub install_flag: u32,
    pub closing_msg: u32,
    pub title_version: u32,
    pub group_id: u32,
    pub save_no_rollback: u32,
    pub bg_daemon_enable: u32,
    pub join_game_id: u32,
    pub olv_accesskey: u32,
    pub wood_tin: u32,
    pub e_manual: u32,
    pub e_manual_version: u32,
    pub region: u32,
    pub pc_cero: u32,
    pub pc_esrb: u32,
    pub pc_bbfc: u32,
    pub pc_usk: u32,
    pub pc_pegi_gen: u32,
    pub pc_pegi_fin: u32,
    pub pc_pegi_prt: u32,
    pub pc_pegi_bbfc: u32,
    pub pc_cob: u32,
    pub pc_grb: u32,
    pub pc_cgsrr: u32,
    pub pc_oflc: u32,
    pub pc_reserved0: u32,
    pub pc_reserved1: u32,
    pub pc_reserved2: u32,
    pub pc_reserved3: u32,
    pub ext_dev_nunchaku: u32,
    pub ext_dev_classic: u32,
    pub ext_dev_urcc: u32,
    pub ext_dev_board: u32,
    pub ext_dev_usb_keyboard: u32,
    pub ext_dev_etc: u32,
    pub ext_dev_etc_name: [ffi::c_char; 512],
    pub eula_version: u32,
    pub drc_use: u32,
    pub network_use: u32,
    pub online_account_use: u32,
    pub direct_boot: u32,
    pub reserved_flag0: u32,
    pub reserved_flag1: u32,
    pub reserved_flag2: u32,
    pub reserved_flag3: u32,
    pub reserved_flag4: u32,
    pub reserved_flag5: u32,
    pub reserved_flag6: u32,
    pub reserved_flag7: u32,
    pub longname_ja: [ffi::c_char; 512],
    pub longname_en: [ffi::c_char; 512],
    pub longname_fr: [ffi::c_char; 512],
    pub longname_de: [ffi::c_char; 512],
    pub longname_it: [ffi::c_char; 512],
    pub longname_es: [ffi::c_char; 512],
    pub longname_zhs: [ffi::c_char; 512],
    pub longname_ko: [ffi::c_char; 512],
    pub longname_nl: [ffi::c_char; 512],
    pub longname_pt: [ffi::c_char; 512],
    pub longname_ru: [ffi::c_char; 512],
    pub longname_zht: [ffi::c_char; 512],
    pub shortname_ja: [ffi::c_char; 256],
    pub shortname_en: [ffi::c_char; 256],
    pub shortname_fr: [ffi::c_char; 256],
    pub shortname_de: [ffi::c_char; 256],
    pub shortname_it: [ffi::c_char; 256],
    pub shortname_es: [ffi::c_char; 256],
    pub shortname_zhs: [ffi::c_char; 256],
    pub shortname_ko: [ffi::c_char; 256],
    pub shortname_nl: [ffi::c_char; 256],
    pub shortname_pt: [ffi::c_char; 256],
    pub shortname_ru: [ffi::c_char; 256],
    pub shortname_zht: [ffi::c_char; 256],
    pub publisher_ja: [ffi::c_char; 256],
    pub publisher_en: [ffi::c_char; 256],
    pub publisher_fr: [ffi::c_char; 256],
    pub publisher_de: [ffi::c_char; 256],
    pub publisher_it: [ffi::c_char; 256],
    pub publisher_es: [ffi::c_char; 256],
    pub publisher_zhs: [ffi::c_char; 256],
    pub publisher_ko: [ffi::c_char; 256],
    pub publisher_nl: [ffi::c_char; 256],
    pub publisher_pt: [ffi::c_char; 256],
    pub publisher_ru: [ffi::c_char; 256],
    pub publisher_zht: [ffi::c_char; 256],
    pub add_on_unique_id: [u32; 32],
    _unk: ffi::unknown<52>,
}

/// ACPDrcLedStatus
#[doc(alias = "ACPDrcLedStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcLedStatus(pub u8);

/// ACPDrcLedPattern
#[doc(alias = "ACPDrcLedPattern")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcLedPattern(pub u32);

unsafe extern "C" {
    /// ACPInitialize
    #[doc(alias = "ACPInitialize")]
    #[link_name = "ACPInitialize"]
    pub unsafe fn init();

    /// ACPFinalize
    #[doc(alias = "ACPFinalize")]
    #[link_name = "ACPFinalize"]
    pub unsafe fn deinit();

    /// ACPCheckApplicationDeviceEmulation
    #[doc(alias = "ACPCheckApplicationDeviceEmulation")]
    #[link_name = "ACPCheckApplicationDeviceEmulation"]
    pub unsafe fn check_application_device_emulation(emulation: *mut ffi::c_bool) -> Error;

    /// ACPCreateSaveDir
    #[doc(alias = "ACPCreateSaveDir")]
    #[link_name = "ACPCreateSaveDir"]
    pub unsafe fn create_save_dir(persistent_id: u32, device: DeviceType) -> Error;

    /// ACPIsExternalStorageRequired
    #[doc(alias = "ACPIsExternalStorageRequired")]
    #[link_name = "ACPIsExternalStorageRequired"]
    pub unsafe fn is_external_storage_required(required: *mut ffi::c_bool) -> Error;

    /// ACPMountExternalStorage
    #[doc(alias = "ACPMountExternalStorage")]
    #[link_name = "ACPMountExternalStorage"]
    pub unsafe fn mount_external_storage() -> Error;

    /// ACPMountSaveDir
    #[doc(alias = "ACPMountSaveDir")]
    #[link_name = "ACPMountSaveDir"]
    pub unsafe fn mount_save_dir() -> Error;

    /// ACPRemoveSaveDir
    #[doc(alias = "ACPRemoveSaveDir")]
    #[link_name = "ACPRemoveSaveDir"]
    pub unsafe fn remove_save_dir(persistent_id: u32, title_id: u64, device: DeviceType) -> Error;

    /// ACPRemoveSaveDirWithoutFlush
    #[doc(alias = "ACPRemoveSaveDirWithoutFlush")]
    #[link_name = "ACPRemoveSaveDirWithoutFlush"]
    pub unsafe fn remove_save_dir_without_flush(
        persistent_id: u32,
        title_id: u64,
        device: DeviceType,
    ) -> Error;

    /// ACPRemoveSaveDirWithoutMetaCheck
    #[doc(alias = "ACPRemoveSaveDirWithoutMetaCheck")]
    #[link_name = "ACPRemoveSaveDirWithoutMetaCheck"]
    pub unsafe fn remove_save_dir_without_meta_check(
        persistent_id: u32,
        title_id: u64,
        device: DeviceType,
    ) -> Error;

    /// ACPRepairSaveMetaDir
    #[doc(alias = "ACPRepairSaveMetaDir")]
    #[link_name = "ACPRepairSaveMetaDir"]
    pub unsafe fn repair_save_meta_dir() -> Error;

    /// ACPUnmountExternalStorage
    #[doc(alias = "ACPUnmountExternalStorage")]
    #[link_name = "ACPUnmountExternalStorage"]
    pub unsafe fn unmount_external_storage() -> Error;

    /// ACPUnmountSaveDir
    #[doc(alias = "ACPUnmountSaveDir")]
    #[link_name = "ACPUnmountSaveDir"]
    pub unsafe fn unmount_save_dir() -> Error;

    /// ACPGetSaveDataTitleIdList
    #[doc(alias = "ACPGetSaveDataTitleIdList")]
    #[link_name = "ACPGetSaveDataTitleIdList"]
    pub unsafe fn get_save_data_title_id_list(
        device: DeviceType,
        title_ids: *mut u64,
        max_count: u32,
        count: *mut u32,
    ) -> Error;

    /// ACPGetTitleSaveDirEx
    #[doc(alias = "ACPGetTitleSaveDirEx")]
    #[link_name = "ACPGetTitleSaveDirEx"]
    pub unsafe fn get_title_save_dir_ex(
        title_id: u64,
        device: DeviceType,
        unk: u32,
        infos: *mut SaveDirInfo,
        max_count: u32,
        count: *mut u32,
    ) -> Error;

    /// ACPGetTitleSaveDirExWithoutMetaCheck
    #[doc(alias = "ACPGetTitleSaveDirExWithoutMetaCheck")]
    #[link_name = "ACPGetTitleSaveDirExWithoutMetaCheck"]
    pub unsafe fn get_title_save_dir_ex_without_meta_check(
        title_id: u64,
        device: DeviceType,
        unk: u32,
        infos: *mut SaveDirInfo,
        max_count: u32,
        count: *mut u32,
    ) -> Error;

    /// ACPAssignTitlePatch
    #[doc(alias = "ACPAssignTitlePatch")]
    #[link_name = "ACPAssignTitlePatch"]
    pub unsafe fn assign_title_patch(title: *mut TitleListType) -> Error;

    /// ACPGetTitleIdOfMainApplication
    #[doc(alias = "ACPGetTitleIdOfMainApplication")]
    #[link_name = "ACPGetTitleIdOfMainApplication"]
    pub unsafe fn get_title_id_of_main_application(title_id: *mut u64) -> Error;

    /// ACPGetTitleMetaXml
    ///
    /// Reads the `meta.xml` of an installed title.
    #[doc(alias = "ACPGetTitleMetaXml")]
    #[link_name = "__rplwrap_ACPGetTitleMetaXml"]
    pub unsafe fn get_title_meta_xml(title_id: u64, meta: *mut MetaXml) -> Error;

    /// ACPGetTitleSaveMetaXml
    #[doc(alias = "ACPGetTitleSaveMetaXml")]
    #[link_name = "__rplwrap_ACPGetTitleSaveMetaXml"]
    pub unsafe fn get_title_save_meta_xml(
        title_id: u64,
        meta: *mut MetaXml,
        device: DeviceType,
    ) -> Error;

    /// ACPGetTitleMetaDir
    #[doc(alias = "ACPGetTitleMetaDir")]
    #[link_name = "ACPGetTitleMetaDir"]
    pub unsafe fn get_title_meta_dir(title_id: u64, dir: *mut ffi::c_char, dir_len: usize)
    -> Error;

    /// ACPGetTitleMetaDirByDevice
    #[doc(alias = "ACPGetTitleMetaDirByDevice")]
    #[link_name = "ACPGetTitleMetaDirByDevice"]
    pub unsafe fn get_title_meta_dir_by_device(
        title_id: u64,
        dir: *mut ffi::c_char,
        dir_len: usize,
        device: DeviceType,
    ) -> Error;

    /// ACPGetTitleMetaDirByTitleListType
    #[doc(alias = "ACPGetTitleMetaDirByTitleListType")]
    #[link_name = "ACPGetTitleMetaDirByTitleListType"]
    pub unsafe fn get_title_meta_dir_by_title_list_type(
        title: TitleListType,
        dir: *mut ffi::c_char,
        dir_len: usize,
    ) -> Error;

    /// ACPIsOverAgeEx
    ///
    /// Whether the account in `slot` is at least `age` years old.
    #[doc(alias = "ACPIsOverAgeEx")]
    #[link_name = "ACPIsOverAgeEx"]
    pub unsafe fn is_over_age_ex(age: u8, slot: u8) -> ffi::c_bool;

    /// ACPDrcLedStartTest
    #[doc(alias = "ACPDrcLedStartTest")]
    #[link_name = "ACPDrcLedStartTest"]
    pub unsafe fn drc_led_start_test();

    /// ACPDrcLedStopTest
    #[doc(alias = "ACPDrcLedStopTest")]
    #[link_name = "ACPDrcLedStopTest"]
    pub unsafe fn drc_led_stop_test();

    /// ACPGetDrcLedStat
    #[doc(alias = "ACPGetDrcLedStat")]
    #[link_name = "ACPGetDrcLedStat"]
    pub unsafe fn get_drc_led_stat(status: *mut DrcLedStatus);

    /// ACPGetDrcLedStatusOfPattern
    #[doc(alias = "ACPGetDrcLedStatusOfPattern")]
    #[link_name = "ACPGetDrcLedStatusOfPattern"]
    pub unsafe fn get_drc_led_status_of_pattern(status: *mut DrcLedStatus, pattern: DrcLedPattern);

    /// ACPSetDrcLedDummyPowerStat
    #[doc(alias = "ACPSetDrcLedDummyPowerStat")]
    #[link_name = "ACPSetDrcLedDummyPowerStat"]
    pub unsafe fn set_drc_led_dummy_power_stat(unk1: u8);

    /// ACPSetDrcLedTimerLength
    #[doc(alias = "ACPSetDrcLedTimerLength")]
    #[link_name = "ACPSetDrcLedTimerLength"]
    pub unsafe fn set_drc_led_timer_length(unk1: u64, unk2: u64);

    /// ACPSetDrcLedTimerSpeed
    #[doc(alias = "ACPSetDrcLedTimerSpeed")]
    #[link_name = "ACPSetDrcLedTimerSpeed"]
    pub unsafe fn set_drc_led_timer_speed(speed: u32);

    /// ACPTurnOffDrcLed
    #[doc(alias = "ACPTurnOffDrcLed")]
    #[link_name = "ACPTurnOffDrcLed"]
    pub unsafe fn turn_off_drc_led();

    /// ACPTurnOffDrcLedTest
    #[doc(alias = "ACPTurnOffDrcLedTest")]
    #[link_name = "ACPTurnOffDrcLedTest"]
    pub unsafe fn turn_off_drc_led_test(unk1: u8);

    /// ACPTurnOnDrcLed
    #[doc(alias = "ACPTurnOnDrcLed")]
    #[link_name = "ACPTurnOnDrcLed"]
    pub unsafe fn turn_on_drc_led(unk1: u32, pattern: DrcLedPattern);

    /// ACPTurnOnDrcLedTest
    #[doc(alias = "ACPTurnOnDrcLedTest")]
    #[link_name = "ACPTurnOnDrcLedTest"]
    pub unsafe fn turn_on_drc_led_test(unk1: u8, unk2: u32, pattern: DrcLedPattern);
}

imports_section!(
    "nn_acp",
    [
        "ACPAssignTitlePatch",
        "ACPCheckApplicationDeviceEmulation",
        "ACPCheckPreOrderTitle",
        "ACPCheckSaveDirRequiredUpdate",
        "ACPCheckSelfTitleNotReferAccountLaunch",
        "ACPCheckSysTitleLaunchFromGame",
        "ACPCheckTitleLaunch",
        "ACPCheckTitleLaunchByTitleListType",
        "ACPCheckTitleLaunchByTitleListTypeEx",
        "ACPCheckTitleLaunchEx",
        "ACPCheckTitleNotReferAccountLaunch",
        "ACPCheckTitleNotReferAccountLaunchAndDRC",
        "ACPCheckTitleNotReferAccountLaunchAndDRCByTitleListType",
        "ACPCheckTitleNotReferAccountLaunchByTitleListType",
        "ACPClearOwnApplicationTitleId",
        "ACPClearRtcResetFlag",
        "ACPCmpNandCheck",
        "ACPCmpNandGetAvailableUserArea",
        "ACPCmpNandGetSaveSize",
        "ACPConvertNetworkTimeToOSCalendarTime",
        "ACPCopySaveDirByDeviceIndex",
        "ACPCreateAccountSaveDir",
        "ACPCreateBaristaSaveQuotaOnUsb",
        "ACPCreateCabinetBackupSaveDir",
        "ACPCreateDiscPairingInfoDir",
        "ACPCreateNoDeleteGroupSaveDir",
        "ACPCreateNoDeleteGroupSaveDirForLauncher",
        "ACPCreateSaveDir",
        "ACPCreateSaveDirByAppMeta",
        "ACPCreateSaveDirEx",
        "ACPDebugCreateSaveDir",
        "ACPDebugCreateSaveDirEx",
        "ACPDebugResetForceEmulatedFlg",
        "ACPDebugSetForceEmulatedFlg",
        "ACPDeleteUserData",
        "ACPDrcLedStartTest",
        "ACPDrcLedStopTest",
        "ACPExistsBaristaSaveQuotaOnUsb",
        "ACPExportSaveDataToBuffer",
        "ACPExportSaveDirOfAccountWithEncryption",
        "ACPExportSaveDirOfMetaWithEncryption",
        "ACPExportSaveMetaToBuffer",
        "ACPFinalize",
        "ACPGetAddOnUniqueId",
        "ACPGetAgeEx",
        "ACPGetApplicationBox",
        "ACPGetBossStorageList",
        "ACPGetConsoleStatusCode",
        "ACPGetDiscPairingInfoDir",
        "ACPGetDrcLedStat",
        "ACPGetDrcLedStatusOfPattern",
        "ACPGetFreeSpaceSizeOfDevice",
        "ACPGetFsaStat",
        "ACPGetLaunchMetaData",
        "ACPGetLaunchMetaXml",
        "ACPGetNetworkTime",
        "ACPGetNoDeleteGroupSaveDir",
        "ACPGetNoDeleteGroupSaveDirForLauncher",
        "ACPGetOlvAccesskey",
        "ACPGetRtcCount",
        "ACPGetSaveDataTitleIdList",
        "ACPGetSaveDirList",
        "ACPGetSystemInformation",
        "ACPGetTitleCmpTitleId",
        "ACPGetTitleIdOfBackgroundApplication",
        "ACPGetTitleIdOfMainApplication",
        "ACPGetTitleInfoOfMainApplication",
        "ACPGetTitleMetaDir",
        "ACPGetTitleMetaDirByDevice",
        "ACPGetTitleMetaDirByTitleListType",
        "ACPGetTitleMetaXmlByDevice",
        "ACPGetTitleMetaXmlByPath",
        "ACPGetTitleMetaXmlByTitleListType",
        "ACPGetTitleSaveDir",
        "ACPGetTitleSaveDirEx",
        "ACPGetTitleSaveDirExWithoutMetaCheck",
        "ACPGetWoodTin",
        "ACPImportSaveDataFromBuffer",
        "ACPImportSaveDirOfAccountWithEncryption",
        "ACPImportSaveDirOfMetaWithEncryption",
        "ACPImportSaveMetaFromBuffer",
        "ACPInitialize",
        "ACPInvalidateApplicationBoxCache",
        "ACPInvalidateNetworkTimeForDebug",
        "ACPIsExternalStorageRequired",
        "ACPIsNetworkTimeOffsetSet",
        "ACPIsOverAgeEx",
        "ACPLaunchTitlel",
        "ACPLaunchTitlelEx",
        "ACPLaunchTitlev",
        "ACPLaunchTitlevEx",
        "ACPMigrateSaveDir",
        "ACPMiiUnwrap",
        "ACPMiiWrap",
        "ACPMountExternalStorage",
        "ACPMountSaveDir",
        "ACPMoveSaveDir",
        "ACPMoveSaveDirByDeviceIndex",
        "ACPRemoveAllDiscPairingInfo",
        "ACPRemoveAllNoDeleteGroupSaveDir",
        "ACPRemoveAllSaveDirOfAccount",
        "ACPRemoveBaristaSaveQuotaOnUsb",
        "ACPRemoveNoDeleteGroupSaveDir",
        "ACPRemoveNoDeleteGroupSaveDirForLauncher",
        "ACPRemoveNoDeleteTitleSaveDir",
        "ACPRemoveSaveDir",
        "ACPRemoveSaveDirWithoutFlush",
        "ACPRemoveSaveDirWithoutMetaCheck",
        "ACPRepairSaveMetaDir",
        "ACPSetBootFlagWithoutLaunchCheck",
        "ACPSetDrcLedDummyPowerStat",
        "ACPSetDrcLedTimerLength",
        "ACPSetDrcLedTimerSpeed",
        "ACPSetNetworkTimeForDebug",
        "ACPSetOwnApplicationTitleId",
        "ACPSetRtcResetFlag",
        "ACPStartTest",
        "ACPTurnOffDrcLed",
        "ACPTurnOffDrcLedTest",
        "ACPTurnOnDrcLed",
        "ACPTurnOnDrcLedTest",
        "ACPUnmountExternalStorage",
        "ACPUnmountSaveDir",
        "ACPUpdateSaveDir",
        "ACPUpdateSaveMetaDir",
        "ACPUpdateSaveTimeStamp",
        "CheckImportSaveDirSizeOfAccount__Q2_2nn3acpFPbUiUL13ACPDeviceTypePCc",
        "CheckImportSaveDirSizeOfMeta__Q2_2nn3acpFPbUL13ACPDeviceTypePCc",
        "CheckTitleLaunchByTitleListType__Q2_2nn3acpFPC17MCP_TitleListTypePC13ACPPCAuthInfo",
        "CheckTitleLaunch__Q2_2nn3acpFULPC13ACPPCAuthInfo",
        "CreateBaristaSaveQuotaOnUsb__Q2_2nn3acpFUi",
        "CreateSaveDirEx__Q2_2nn3acpFUiUL13ACPDeviceType",
        "CreateSaveDir__Q2_2nn3acpFUi13ACPDeviceType",
        "DrcLedStartTest__Q2_2nn3acpFv",
        "DrcLedStopTest__Q2_2nn3acpFv",
        "ExistsBaristaSaveQuotaOnUsb__Q2_2nn3acpFPbUi",
        "ExportSaveDirOfAccount__Q2_2nn3acpFUiUL13ACPDeviceTypePCcPb",
        "ExportSaveDirOfMeta__Q2_2nn3acpFUL13ACPDeviceTypePCcPb",
        "GetBossStorageList__Q2_2nn3acpFPiP18ACPBossStorageInfoUiT3UL",
        "GetDrcLedStat__Q2_2nn3acpFPi",
        "GetDrcLedStatusOfPattern__Q2_2nn3acpFPQ3_2nn3acp12DrcLedStatusQ3_2nn3acp13DrcLedPattern",
        "GetMetaData__Q2_2nn3acpFP12_ACPMetaData",
        "GetMetaXml__Q2_2nn3acpFP11_ACPMetaXml",
        "ImportSaveDirOfAccount__Q2_2nn3acpFUiUL13ACPDeviceTypePCc",
        "ImportSaveDirOfMeta__Q2_2nn3acpFUL13ACPDeviceTypePCc",
        "RemoveBaristaSaveQuotaOnUsb__Q2_2nn3acpFUi",
        "RepairSaveMetaDir__Q2_2nn3acpFv",
        "SetDrcLedDummyPowerStat__Q2_2nn3acpFUc",
        "SetDrcLedTimerLength__Q2_2nn3acpFULT1",
        "SetDrcLedTimerSpeed__Q2_2nn3acpFUi",
        "SetSaveExportFormat__Q2_2nn3acpF19ACPSaveExportFormat",
        "TurnOffDrcLedTest__Q2_2nn3acpFUc",
        "TurnOffDrcLed__Q2_2nn3acpFv",
        "TurnOnDrcLedTest__Q2_2nn3acpFUcUiQ3_2nn3acp13DrcLedPattern",
        "TurnOnDrcLed__Q2_2nn3acpFUiQ3_2nn3acp13DrcLedPattern",
        "WaitExternalStorage__Q2_2nn3acpFv",
        "__rplwrap_ACPGetTitleMetaXml",
        "__rplwrap_ACPGetTitleSaveMetaXml"
    ],
    []
);
