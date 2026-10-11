//! Amiibo (NFP)

use super::{Result, ffl::StoreData};
use crate::{
    coreinit::event::Event,
    ffi,
    nfc::Technology,
    sysapp::{ArgSlot, StandardArgs},
};

/// Description values of an NFP [Result].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultDescription(pub i32);

impl ResultDescription {
    pub const OUT_OF_RANGE: Self = Self(0x03700);
    pub const INVALID_PARAM: Self = Self(0x03780);
    pub const INVALID_ALIGNMENT: Self = Self(0x03800);
    pub const INVALID_STATE: Self = Self(0x06400);
    pub const INVALID_TAG: Self = Self(0x0c800);
    pub const INVALID_TAG_INFO: Self = Self(0x0ca80);
    pub const NO_BACKUPENTRY: Self = Self(0x0e580);
    pub const NO_REGISTER_INFO: Self = Self(0x10900);
    pub const APP_AREA_MISSING: Self = Self(0x10400);
    pub const APP_AREA_TAGID_MISMATCH: Self = Self(0x11d00);
    pub const APP_AREA_ALREADY_EXISTS: Self = Self(0x10e00);
    pub const APP_AREA_ACCESS_ID_MISMATCH: Self = Self(0x11300);
    pub const NO_BACKUP_SAVEDATA: Self = Self(0x38880);
    pub const SYSTEM_ERROR: Self = Self(0x3e880);
    pub const FATAL: Self = Self(0x5db00);
}

/// NfpState
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NfpState(pub u32);

impl NfpState {
    pub const UNINITIALIZED: Self = Self(0);
    pub const INITIALIZED: Self = Self(1);
    pub const SEARCHING: Self = Self(2);
    pub const FOUND: Self = Self(3);
    pub const REMOVED: Self = Self(4);
    pub const MOUNTED: Self = Self(5);
    pub const UNKNOWN6: Self = Self(6);
    pub const MOUNTED_ROM: Self = Self(7);
}

/// AdminFlags
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdminFlags(pub u8);

impl AdminFlags {
    /// The tag contains register info.
    pub const IS_REGISTERED: Self = Self(1 << 0);
    pub const HAS_APPLICATION_DATA: Self = Self(1 << 1);
}

/// TagType
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagType(pub u8);

impl TagType {
    pub const UNKNOWN: Self = Self(0);
    pub const TYPE1_TAG: Self = Self(1 << 0);
    pub const TYPE2_TAG: Self = Self(1 << 1);
    pub const TYPE3_TAG: Self = Self(1 << 2);
    pub const ISO15693: Self = Self(1 << 5);
}

/// AmiiboSettingsMode
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmiiboSettingsMode(pub u32);

impl AmiiboSettingsMode {
    /// Mii and name registration.
    pub const REGISTER: Self = Self(0);
    pub const DELETE_GAME_DATA: Self = Self(1);
    pub const RESTORE: Self = Self(2);
}

/// Date
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u16,
    /// 0 to 11.
    pub month: u8,
    /// 0 to 30.
    pub day: u8,
}

/// ApplicationAreaCreateInfo
///
/// Initialize with [initialize_create_info].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationAreaCreateInfo {
    pub access_id: u32,
    pub data: *mut ffi::c_void,
    pub size: u32,
    pub reserved: ffi::unknown<0x30>,
}

/// TagId
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId {
    pub size: u8,
    pub uid: [u8; 10],
}

/// TagInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagInfo {
    pub id: TagId,
    pub reserved0: ffi::unknown<0x15>,
    pub technology: Technology,
    pub tag_type: TagType,
    pub reserved1: ffi::unknown<0x32>,
}

/// CommonInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommonInfo {
    pub last_write_date: Date,
    pub writes: u16,
    pub character_id: [u8; 3],
    pub series_id: u8,
    pub numbering_id: u16,
    pub figure_type: u8,
    pub figure_version: u8,
    pub application_area_size: u16,
    pub reserved: ffi::unknown<0x30>,
}

/// RegisterInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegisterInfo {
    pub mii: StoreData,
    /// UTF-16.
    pub name: [u16; 11],
    pub font_region: u8,
    pub country: u8,
    pub register_date: Date,
    pub reserved: ffi::unknown<0x2c>,
}

/// ReadOnlyInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadOnlyInfo {
    pub character_id: [u8; 3],
    pub series_id: u8,
    pub numbering_id: u16,
    pub figure_type: u8,
    pub reserved: ffi::unknown<0x2f>,
}

/// RomInfo
pub type RomInfo = ReadOnlyInfo;

/// RegisterInfoSet
///
/// Initialize with [initialize_register_info_set].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegisterInfoSet {
    pub mii: StoreData,
    /// UTF-16.
    pub name: [u16; 11],
    pub font_region: u8,
    pub reserved: ffi::unknown<0x2d>,
}

/// AdminInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdminInfo {
    /// Title that wrote the application area.
    pub title_id: u64,
    pub access_id: u32,
    pub application_area_writes: u16,
    pub flags: AdminFlags,
    pub format_version: u8,
    /// 1 for Wii U.
    pub platform: u8,
    pub reserved: ffi::unknown<0x2f>,
}

/// AmiiboSettingsArgsIn
///
/// Initialize with [initialize_amiibo_settings_args_in].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmiiboSettingsArgsIn {
    pub mode: AmiiboSettingsMode,
    pub tag_info: TagInfo,
    pub is_registered: bool,
    pub padding: ffi::unknown<3>,
    pub register_info: RegisterInfo,
    pub common_info: CommonInfo,
    pub reserved: ffi::unknown<0x20>,
}

/// AmiiboSettingsArgs
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmiiboSettingsArgs {
    pub standard_args: StandardArgs,
    pub args_in: AmiiboSettingsArgsIn,
}

/// AmiiboSettingsResult
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmiiboSettingsResult {
    /// 0 on success.
    pub result: i32,
    pub tag_info: TagInfo,
    pub register_info: RegisterInfo,
    pub common_info: CommonInfo,
    pub reserved: ffi::unknown<0x20>,
}

unsafe extern "C" {
    /// GetNfpState
    #[doc(alias = "GetNfpState")]
    #[link_name = "GetNfpState__Q2_2nn3nfpFv"]
    pub unsafe fn get_nfp_state() -> NfpState;

    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q2_2nn3nfpFv"]
    pub unsafe fn init() -> Result;

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q2_2nn3nfpFv"]
    pub unsafe fn deinit() -> Result;

    /// AntennaCheck
    #[doc(alias = "AntennaCheck")]
    #[link_name = "AntennaCheck__Q2_2nn3nfpFv"]
    pub unsafe fn antenna_check() -> Result;

    /// SetActivateEvent
    ///
    /// Signaled when a tag is detected.
    #[doc(alias = "SetActivateEvent")]
    #[link_name = "SetActivateEvent__Q2_2nn3nfpFP7OSEvent"]
    pub unsafe fn set_activate_event(event: *mut Event) -> Result;

    /// SetDeactivateEvent
    ///
    /// Signaled when the tag is removed.
    #[doc(alias = "SetDeactivateEvent")]
    #[link_name = "SetDeactivateEvent__Q2_2nn3nfpFP7OSEvent"]
    pub unsafe fn set_deactivate_event(event: *mut Event) -> Result;

    /// StartDetection
    #[doc(alias = "StartDetection")]
    #[link_name = "StartDetection__Q2_2nn3nfpFv"]
    pub unsafe fn start_detection() -> Result;

    /// StopDetection
    #[doc(alias = "StopDetection")]
    #[link_name = "StopDetection__Q2_2nn3nfpFv"]
    pub unsafe fn stop_detection() -> Result;

    /// Mount
    #[doc(alias = "Mount")]
    #[link_name = "Mount__Q2_2nn3nfpFv"]
    pub unsafe fn mount() -> Result;

    /// MountReadOnly
    #[doc(alias = "MountReadOnly")]
    #[link_name = "MountReadOnly__Q2_2nn3nfpFv"]
    pub unsafe fn mount_read_only() -> Result;

    /// MountRom
    #[doc(alias = "MountRom")]
    #[link_name = "MountRom__Q2_2nn3nfpFv"]
    pub unsafe fn mount_rom() -> Result;

    /// Flush
    ///
    /// Writes pending changes to the tag.
    #[doc(alias = "Flush")]
    #[link_name = "Flush__Q2_2nn3nfpFv"]
    pub unsafe fn flush() -> Result;

    /// Unmount
    #[doc(alias = "Unmount")]
    #[link_name = "Unmount__Q2_2nn3nfpFv"]
    pub unsafe fn unmount() -> Result;

    /// IsExistApplicationArea
    #[doc(alias = "IsExistApplicationArea")]
    #[link_name = "IsExistApplicationArea__Q2_2nn3nfpFv"]
    pub unsafe fn is_exist_application_area() -> bool;

    /// InitializeCreateInfo
    #[doc(alias = "InitializeCreateInfo")]
    #[link_name = "InitializeCreateInfo__Q2_2nn3nfpFPQ3_2nn3nfp25ApplicationAreaCreateInfo"]
    pub unsafe fn initialize_create_info(info: *mut ApplicationAreaCreateInfo) -> Result;

    /// CreateApplicationArea
    #[doc(alias = "CreateApplicationArea")]
    #[link_name = "CreateApplicationArea__Q2_2nn3nfpFRCQ3_2nn3nfp25ApplicationAreaCreateInfo"]
    pub unsafe fn create_application_area(info: *const ApplicationAreaCreateInfo) -> Result;

    /// OpenApplicationArea
    #[doc(alias = "OpenApplicationArea")]
    #[link_name = "OpenApplicationArea__Q2_2nn3nfpFUi"]
    pub unsafe fn open_application_area(access_id: u32) -> Result;

    /// DeleteApplicationArea
    #[doc(alias = "DeleteApplicationArea")]
    #[link_name = "DeleteApplicationArea__Q2_2nn3nfpFv"]
    pub unsafe fn delete_application_area() -> Result;

    /// WriteApplicationArea
    #[doc(alias = "WriteApplicationArea")]
    #[link_name = "WriteApplicationArea__Q2_2nn3nfpFPCvUiRCQ3_2nn3nfp5TagId"]
    pub unsafe fn write_application_area(
        data: *const ffi::c_void,
        size: u32,
        id: *const TagId,
    ) -> Result;

    /// ReadApplicationArea
    #[doc(alias = "ReadApplicationArea")]
    #[link_name = "ReadApplicationArea__Q2_2nn3nfpFPvUi"]
    pub unsafe fn read_application_area(data: *mut ffi::c_void, size: u32) -> Result;

    /// InitializeRegisterInfoSet
    #[doc(alias = "InitializeRegisterInfoSet")]
    #[link_name = "InitializeRegisterInfoSet__Q2_2nn3nfpFPQ3_2nn3nfp15RegisterInfoSet"]
    pub unsafe fn initialize_register_info_set(info: *mut RegisterInfoSet) -> Result;

    /// SetNfpRegisterInfo
    #[doc(alias = "SetNfpRegisterInfo")]
    #[link_name = "SetNfpRegisterInfo__Q2_2nn3nfpFRCQ3_2nn3nfp15RegisterInfoSet"]
    pub unsafe fn set_nfp_register_info(info: *const RegisterInfoSet) -> Result;

    /// GetNfpRegisterInfo
    #[doc(alias = "GetNfpRegisterInfo")]
    #[link_name = "GetNfpRegisterInfo__Q2_2nn3nfpFPQ3_2nn3nfp12RegisterInfo"]
    pub unsafe fn get_nfp_register_info(info: *mut RegisterInfo) -> Result;

    /// DeleteNfpRegisterInfo
    #[doc(alias = "DeleteNfpRegisterInfo")]
    #[link_name = "DeleteNfpRegisterInfo__Q2_2nn3nfpFv"]
    pub unsafe fn delete_nfp_register_info() -> Result;

    /// GetNfpCommonInfo
    #[doc(alias = "GetNfpCommonInfo")]
    #[link_name = "GetNfpCommonInfo__Q2_2nn3nfpFPQ3_2nn3nfp10CommonInfo"]
    pub unsafe fn get_nfp_common_info(info: *mut CommonInfo) -> Result;

    /// GetNfpReadOnlyInfo
    #[doc(alias = "GetNfpReadOnlyInfo")]
    #[link_name = "GetNfpReadOnlyInfo__Q2_2nn3nfpFPQ3_2nn3nfp12ReadOnlyInfo"]
    pub unsafe fn get_nfp_read_only_info(info: *mut ReadOnlyInfo) -> Result;

    /// GetNfpAdminInfo
    #[doc(alias = "GetNfpAdminInfo")]
    #[link_name = "GetNfpAdminInfo__Q2_2nn3nfpFPQ3_2nn3nfp9AdminInfo"]
    pub unsafe fn get_nfp_admin_info(info: *mut AdminInfo) -> Result;

    /// GetNfpRomInfo
    #[doc(alias = "GetNfpRomInfo")]
    #[link_name = "GetNfpRomInfo__Q2_2nn3nfpFPQ3_2nn3nfp7RomInfo"]
    pub unsafe fn get_nfp_rom_info(info: *mut RomInfo) -> Result;

    /// GetTagInfo
    #[doc(alias = "GetTagInfo")]
    #[link_name = "GetTagInfo__Q2_2nn3nfpFPQ3_2nn3nfp7TagInfo"]
    pub unsafe fn get_tag_info(info: *mut TagInfo) -> Result;

    /// Format
    #[doc(alias = "Format")]
    #[link_name = "Format__Q2_2nn3nfpFPCUci"]
    pub unsafe fn format(data: *const u8, size: i32) -> Result;

    /// Restore
    #[doc(alias = "Restore")]
    #[link_name = "Restore__Q2_2nn3nfpFv"]
    pub unsafe fn restore() -> Result;

    /// GetErrorCode
    ///
    /// Error code to show to the user.
    #[doc(alias = "GetErrorCode")]
    #[link_name = "GetErrorCode__Q2_2nn3nfpFRCQ2_2nn6Result"]
    pub unsafe fn get_error_code(result: *const Result) -> u32;

    /// InitializeAmiiboSettingsArgsIn
    #[doc(alias = "InitializeAmiiboSettingsArgsIn")]
    #[link_name = "InitializeAmiiboSettingsArgsIn__Q2_2nn3nfpFPQ3_2nn3nfp20AmiiboSettingsArgsIn"]
    pub unsafe fn initialize_amiibo_settings_args_in(args: *mut AmiiboSettingsArgsIn) -> Result;

    /// SwitchToAmiiboSettings
    ///
    /// Opens the amiibo settings applet.
    #[doc(alias = "SwitchToAmiiboSettings")]
    #[link_name = "SwitchToAmiiboSettings__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsArgsIn"]
    pub unsafe fn switch_to_amiibo_settings(args: *const AmiiboSettingsArgsIn) -> Result;

    /// SwitchToAmiiboSettings
    #[doc(alias = "SwitchToAmiiboSettings")]
    #[link_name = "SwitchToAmiiboSettings__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsArgsInPCcUi"]
    pub unsafe fn switch_to_amiibo_settings_with_standard_args(
        args: *const AmiiboSettingsArgsIn,
        standard_arg: *const ffi::c_char,
        standard_arg_size: u32,
    ) -> Result;

    /// GetAmiiboSettingsResult
    #[doc(alias = "GetAmiiboSettingsResult")]
    #[link_name = "GetAmiiboSettingsResult__Q2_2nn3nfpFPQ3_2nn3nfp20AmiiboSettingsResultRC15SysArgDataBlock"]
    pub unsafe fn get_amiibo_settings_result(
        result: *mut AmiiboSettingsResult,
        block: *const ArgSlot,
    ) -> Result;

    /// ReturnToCallerWithAmiiboSettingsResult
    #[doc(alias = "ReturnToCallerWithAmiiboSettingsResult")]
    #[link_name = "ReturnToCallerWithAmiiboSettingsResult__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsResult"]
    pub unsafe fn return_to_caller_with_amiibo_settings_result(
        result: *const AmiiboSettingsResult,
    ) -> Result;

    /// GetAmiiboSettingsArgs
    #[doc(alias = "GetAmiiboSettingsArgs")]
    #[link_name = "GetAmiiboSettingsArgs__Q2_2nn3nfpFPQ3_2nn3nfp18AmiiboSettingsArgs"]
    pub unsafe fn get_amiibo_settings_args(args: *mut AmiiboSettingsArgs) -> Result;
}

imports_section!(
    "nn_nfp",
    [
        "AntennaCheck__Q2_2nn3nfpFv",
        "CheckMovable__Q2_2nn3nfpFRCQ3_2nn3nfp8MoveInfo",
        "CreateApplicationArea__Q2_2nn3nfpFRCQ3_2nn3nfp25ApplicationAreaCreateInfo",
        "DeleteApplicationArea__Q2_2nn3nfpFv",
        "DeleteNfpRegisterInfo__Q2_2nn3nfpFv",
        "Finalize__Q2_2nn3nfpFv",
        "FindCharacterImage__Q2_2nn3nfpFPUiUs",
        "Flush__Q2_2nn3nfpFv",
        "FormatForMove__Q2_2nn3nfpFRCQ3_2nn3nfp8MoveInfoT1",
        "Format__Q2_2nn3nfpFPCUci",
        "GetAmiiboSettingsArgs__Q2_2nn3nfpFPQ3_2nn3nfp18AmiiboSettingsArgs",
        "GetAmiiboSettingsResult__Q2_2nn3nfpFPQ3_2nn3nfp20AmiiboSettingsResultRC15SysArgDataBlock",
        "GetBackupEntryFromMemory__Q2_2nn3nfpFPQ3_2nn3nfp15BackupEntryInfoUsPCvUi",
        "GetBackupHeaderFromMemory__Q2_2nn3nfpFPQ3_2nn3nfp16BackupHeaderInfoPCvUi",
        "GetBackupSaveDataSize__Q2_2nn3nfpFv",
        "GetCharacterImage__Q2_2nn3nfpFPvUiUs",
        "GetCharacterName__Q2_2nn3nfpFPUsUiPCUcUc",
        "GetConnectionStatus__Q2_2nn3nfpFPQ3_2nn3nfp16ConnectionStatus",
        "GetErrorCode__Q2_2nn3nfpFRCQ2_2nn6Result",
        "GetNfpAdminInfo__Q2_2nn3nfpFPQ3_2nn3nfp9AdminInfo",
        "GetNfpCommonInfo__Q2_2nn3nfpFPQ3_2nn3nfp10CommonInfo",
        "GetNfpInfoForMove__Q2_2nn3nfpFPQ3_2nn3nfp8MoveInfo",
        "GetNfpReadOnlyInfo__Q2_2nn3nfpFPQ3_2nn3nfp12ReadOnlyInfo",
        "GetNfpRegisterInfo__Q2_2nn3nfpFPQ3_2nn3nfp12RegisterInfo",
        "GetNfpRomInfo__Q2_2nn3nfpFPQ3_2nn3nfp7RomInfo",
        "GetNfpState__Q2_2nn3nfpFv",
        "GetTagInfo__Q2_2nn3nfpFPQ3_2nn3nfp7TagInfo",
        "InitializeAmiiboSettingsArgsIn__Q2_2nn3nfpFPQ3_2nn3nfp20AmiiboSettingsArgsIn",
        "InitializeCreateInfo__Q2_2nn3nfpFPQ3_2nn3nfp25ApplicationAreaCreateInfo",
        "InitializeRegisterInfoSet__Q2_2nn3nfpFPQ3_2nn3nfp15RegisterInfoSet",
        "Initialize__Q2_2nn3nfpFv",
        "IsExistApplicationArea__Q2_2nn3nfpFv",
        "MountReadOnly__Q2_2nn3nfpFv",
        "MountRom__Q2_2nn3nfpFv",
        "Mount__Q2_2nn3nfpFv",
        "Move__Q2_2nn3nfpFv",
        "OpenApplicationArea__Q2_2nn3nfpFUi",
        "ReadAllBackupSaveData__Q2_2nn3nfpFPvUi",
        "ReadApplicationArea__Q2_2nn3nfpFPvUi",
        "Restore__Q2_2nn3nfpFv",
        "ReturnToCallerWithAmiiboSettingsResult__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsResult",
        "SetActivateEvent__Q2_2nn3nfpFP7OSEvent",
        "SetDeactivateEvent__Q2_2nn3nfpFP7OSEvent",
        "SetNfpRegisterInfo__Q2_2nn3nfpFRCQ3_2nn3nfp15RegisterInfoSet",
        "StartDetection__Q2_2nn3nfpFv",
        "StopDetection__Q2_2nn3nfpFv",
        "SwitchToAmiiboSettings__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsArgsIn",
        "SwitchToAmiiboSettings__Q2_2nn3nfpFRCQ3_2nn3nfp20AmiiboSettingsArgsInPCcUi",
        "Unmount__Q2_2nn3nfpFv",
        "WriteApplicationArea__Q2_2nn3nfpFPCvUiRCQ3_2nn3nfp5TagId"
    ],
    []
);
