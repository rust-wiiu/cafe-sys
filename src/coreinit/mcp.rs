//! MCP
//!
//! Installed titles, devices, installation and system settings.

use super::ios::Handle;
use crate::ffi;

/// MCPError
#[doc(alias = "MCPError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// MCPAppType
#[doc(alias = "MCPAppType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppType(pub u32);

impl AppType {
    pub const GAME_UPDATE: Self = Self(0x0800001B);
    pub const GAME_DLC: Self = Self(0x0800000E);
    pub const BOOT1: Self = Self(0x10000009);
    pub const SYSTEM_LIBRARIES: Self = Self(0x1000000A);
    pub const BLUETOOTH_FIRMWARE: Self = Self(0x10000012);
    pub const DRC_FIRMWARE: Self = Self(0x10000013);
    pub const DRH_FIRMWARE: Self = Self(0x10000014);
    pub const SYSTEM_VERSION: Self = Self(0x10000015);
    pub const DRC_LANGUAGE: Self = Self(0x1000001A);
    pub const EXCEPTIONS_DATA: Self = Self(0x18000010);
    pub const SHARED_DATA: Self = Self(0x1800001C);
    pub const CERT_STORE: Self = Self(0x1800001E);
    pub const PATCH_MAP_DATA: Self = Self(0x18000023);
    pub const WAGONU_MIGRATION_LIST: Self = Self(0x18000029);
    pub const CAFFEINE_TITLE_LIST: Self = Self(0x18000030);
    pub const MCP_TITLE_LIST: Self = Self(0x18000031);
    pub const GAME: Self = Self(0x80000000);
    pub const GAME_WII: Self = Self(0x8000002E);
    pub const SYSTEM_MENU: Self = Self(0x90000001);
    pub const SYSTEM_UPDATER: Self = Self(0x9000000B);
    pub const SYSTEM_APPS: Self = Self(0x90000020);
    pub const ACCOUNT_APPS: Self = Self(0x90000021);
    pub const SYSTEM_SETTINGS: Self = Self(0x90000022);
    pub const ECO_PROCESS: Self = Self(0x9000002F);
    pub const EMANUAL: Self = Self(0xD0000003);
    pub const HOME_MENU: Self = Self(0xD0000004);
    pub const ERROR_DISPLAY: Self = Self(0xD0000005);
    pub const BROWSER: Self = Self(0xD0000006);
    pub const MIIVERSE_POST: Self = Self(0xD000000D);
    pub const MIIVERSE: Self = Self(0xD0000016);
    pub const ESHOP: Self = Self(0xD0000017);
    pub const FRIEND_LIST: Self = Self(0xD0000018);
    pub const DOWNLOAD_MANAGEMENT: Self = Self(0xD0000019);
    pub const AOC_OVERLAY: Self = Self(0xD000002C);
    pub const AMIIBO_SETTINGS: Self = Self(0xD0000033);
}

/// MCPDeviceType
///
/// Also used as ACPDeviceType.
#[doc(alias("MCPDeviceType", "ACPDeviceType"))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Optical disc, MLC and USB. Values above [DeviceType::Usb] behave the same.
    Auto = 1,
    /// Disc drive, or emulated disc (HFIO).
    Odd = 2,
    /// Internal storage, or emulated internal storage (HFIO).
    Mlc = 3,
    Usb = 4,
}

/// MCPDeviceFlags
#[doc(alias = "MCPDeviceFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceFlags(pub u32);

/// MCPInstallTarget
#[doc(alias = "MCPInstallTarget")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstallTarget(pub u32);

impl InstallTarget {
    pub const MLC: Self = Self(0);
    pub const USB: Self = Self(1);
}

/// MCPRegion
#[doc(alias = "MCPRegion")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Region(pub u32);

impl Region {
    pub const JAPAN: Self = Self(0x01);
    pub const USA: Self = Self(0x02);
    pub const EUROPE: Self = Self(0x04);
    pub const CHINA: Self = Self(0x10);
    pub const KOREA: Self = Self(0x20);
    pub const TAIWAN: Self = Self(0x40);
}

impl core::ops::BitOr for Region {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MCPCompatAVFile
#[doc(alias = "MCPCompatAVFile")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatAvFile {
    /// DMCU firmware.
    #[default]
    Dmcu = 0x00,
    /// Deinterlacer parameters.
    Deint = 0x01,
}

/// MCPSystemMode
#[doc(alias = "MCPSystemMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemMode(pub u32);

impl SystemMode {
    pub const PRODUCTION: Self = Self(0x00);
    pub const DEVELOPMENT: Self = Self(0x01);
    pub const TEST: Self = Self(0x02);
}

/// MCPDevice
#[doc(alias = "MCPDevice")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Device {
    pub kind: [ffi::c_char; 8],
    _unk0x08: ffi::unknown<128>,
    pub filesystem: [ffi::c_char; 8],
    pub path: [ffi::c_char; 0x27F],
    pub flags: DeviceFlags,
    pub uid: u32,
    pub index: u32,
}

/// MCPInstallInfo
#[doc(alias = "MCPInstallInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstallInfo(pub ffi::unknown<0x27F>);

/// MCPInstallTitleInfo
#[doc(alias = "MCPInstallTitleInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstallTitleInfo(pub ffi::unknown<0x27F>);

/// MCPInstallProgress
#[doc(alias = "MCPInstallProgress")]
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstallProgress {
    pub in_progress: u32,
    pub title_id: u64,
    pub size_total: u64,
    pub size_progress: u64,
    pub contents_total: u32,
    pub contents_progress: u32,
}

/// MCPSysProdSettings
#[doc(alias = "MCPSysProdSettings")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SysProdSettings {
    pub product_area: Region,
    pub eeprom_version: u16,
    _pad: ffi::unknown<2>,
    pub game_region: Region,
    _unk: ffi::unknown<4>,
    pub ntsc_pal: [ffi::c_char; 5],
    pub wifi_5ghz_country_code: [ffi::c_char; 4],
    pub wifi_5ghz_country_code_revision: u8,
    pub code_id: [ffi::c_char; 8],
    pub serial_id: [ffi::c_char; 12],
    _unk2: ffi::unknown<4>,
    pub model_number: [ffi::c_char; 16],
    pub version: u32,
}

/// MCPSystemVersion
#[doc(alias = "MCPSystemVersion")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub region: ffi::c_char,
}

/// MCPTitleListType
#[doc(alias = "MCPTitleListType")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TitleListType {
    pub title_id: u64,
    pub group_id: u32,
    pub path: [ffi::c_char; 56],
    pub app_type: AppType,
    pub title_version: u16,
    pub os_version: u64,
    pub sdk_version: u32,
    pub indexed_device: [ffi::c_char; 10],
    pub unk0x60: u8,
}

const _: () = {
    assert!(size_of::<Device>() == 0x31B);
    assert!(size_of::<InstallInfo>() == 0x27F);
    assert!(size_of::<InstallTitleInfo>() == 0x27F);
    assert!(size_of::<InstallProgress>() == 0x24);
    assert!(size_of::<SysProdSettings>() == 0x46);
    assert!(core::mem::offset_of!(SysProdSettings, model_number) == 0x32);
    assert!(size_of::<SystemVersion>() == 0x10);
    assert!(size_of::<TitleListType>() == 0x61);
    assert!(core::mem::offset_of!(TitleListType, indexed_device) == 0x56);
};

unsafe extern "C" {
    /// MCP_Open
    ///
    /// Returns the handle, or a negative error.
    #[doc(alias = "MCP_Open")]
    #[link_name = "MCP_Open"]
    pub unsafe fn open() -> Handle;

    /// MCP_Close
    #[doc(alias = "MCP_Close")]
    #[link_name = "MCP_Close"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// MCP_DeviceList
    #[doc(alias = "MCP_DeviceList")]
    #[link_name = "MCP_DeviceList"]
    pub unsafe fn device_list(
        handle: Handle,
        num_devices: *mut ffi::c_int,
        devices: *mut Device,
        devices_size: u32,
    ) -> Error;

    /// MCP_FullDeviceList
    #[doc(alias = "MCP_FullDeviceList")]
    #[link_name = "MCP_FullDeviceList"]
    pub unsafe fn full_device_list(
        handle: Handle,
        num_devices: *mut ffi::c_int,
        devices: *mut Device,
        devices_size: u32,
    ) -> Error;

    /// MCP_GetOwnTitleInfo
    #[doc(alias = "MCP_GetOwnTitleInfo")]
    #[link_name = "MCP_GetOwnTitleInfo"]
    pub unsafe fn get_own_title_info(handle: Handle, info: *mut TitleListType) -> Error;

    /// MCP_GetSystemMode
    #[doc(alias = "MCP_GetSystemMode")]
    #[link_name = "MCP_GetSystemMode"]
    pub unsafe fn get_system_mode(handle: Handle, mode: *mut SystemMode) -> Error;

    /// MCP_GetSysProdSettings
    #[doc(alias = "MCP_GetSysProdSettings")]
    #[link_name = "MCP_GetSysProdSettings"]
    pub unsafe fn get_sys_prod_settings(handle: Handle, settings: *mut SysProdSettings) -> Error;

    /// MCP_GetSystemVersion
    #[doc(alias = "MCP_GetSystemVersion")]
    #[link_name = "MCP_GetSystemVersion"]
    pub unsafe fn get_system_version(handle: Handle, version: *mut SystemVersion) -> Error;

    /// MCP_GetTitleId
    #[doc(alias = "MCP_GetTitleId")]
    #[link_name = "MCP_GetTitleId"]
    pub unsafe fn get_title_id(handle: Handle, title_id: *mut u64) -> Error;

    /// MCP_GetTitleInfo
    #[doc(alias = "MCP_GetTitleInfo")]
    #[link_name = "MCP_GetTitleInfo"]
    pub unsafe fn get_title_info(handle: Handle, title_id: u64, info: *mut TitleListType) -> Error;

    /// MCP_InstallSetTargetDevice
    #[doc(alias = "MCP_InstallSetTargetDevice")]
    #[link_name = "MCP_InstallSetTargetDevice"]
    pub unsafe fn install_set_target_device(handle: Handle, device: InstallTarget) -> Error;

    /// MCP_InstallGetTargetDevice
    #[doc(alias = "MCP_InstallGetTargetDevice")]
    #[link_name = "MCP_InstallGetTargetDevice"]
    pub unsafe fn install_get_target_device(handle: Handle, device: *mut InstallTarget) -> Error;

    /// MCP_InstallSetTargetUsb
    #[doc(alias = "MCP_InstallSetTargetUsb")]
    #[link_name = "MCP_InstallSetTargetUsb"]
    pub unsafe fn install_set_target_usb(handle: Handle, usb: ffi::c_int) -> Error;

    /// MCP_InstallGetInfo
    #[doc(alias = "MCP_InstallGetInfo")]
    #[link_name = "MCP_InstallGetInfo"]
    pub unsafe fn install_get_info(
        handle: Handle,
        path: *const ffi::c_char,
        info: *mut InstallInfo,
    ) -> Error;

    /// MCP_InstallTitleAsync
    #[doc(alias = "MCP_InstallTitleAsync")]
    #[link_name = "MCP_InstallTitleAsync"]
    pub unsafe fn install_title_async(
        handle: Handle,
        path: *const ffi::c_char,
        info: *mut InstallTitleInfo,
    ) -> Error;

    /// MCP_InstallGetProgress
    #[doc(alias = "MCP_InstallGetProgress")]
    #[link_name = "MCP_InstallGetProgress"]
    pub unsafe fn install_get_progress(handle: Handle, progress: *mut InstallProgress) -> Error;

    /// MCP_InstallTitleAbort
    #[doc(alias = "MCP_InstallTitleAbort")]
    #[link_name = "MCP_InstallTitleAbort"]
    pub unsafe fn install_title_abort(handle: Handle) -> Error;

    /// MCP_UninstallTitleAsync
    #[doc(alias = "MCP_UninstallTitleAsync")]
    #[link_name = "MCP_UninstallTitleAsync"]
    pub unsafe fn uninstall_title_async(
        handle: Handle,
        path: *const ffi::c_char,
        info: *mut InstallTitleInfo,
    ) -> Error;

    /// MCP_TitleCount
    ///
    /// Returns the number of installed titles, or a negative error.
    #[doc(alias = "MCP_TitleCount")]
    #[link_name = "MCP_TitleCount"]
    pub unsafe fn title_count(handle: Handle) -> i32;

    /// MCP_TitleList
    #[doc(alias = "MCP_TitleList")]
    #[link_name = "MCP_TitleList"]
    pub unsafe fn title_list(
        handle: Handle,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByAppType
    #[doc(alias = "MCP_TitleListByAppType")]
    #[link_name = "MCP_TitleListByAppType"]
    pub unsafe fn title_list_by_app_type(
        handle: Handle,
        app_type: AppType,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByUniqueId
    #[doc(alias = "MCP_TitleListByUniqueId")]
    #[link_name = "MCP_TitleListByUniqueId"]
    pub unsafe fn title_list_by_unique_id(
        handle: Handle,
        unique_id: u32,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByDevice
    #[doc(alias = "MCP_TitleListByDevice")]
    #[link_name = "MCP_TitleListByDevice"]
    pub unsafe fn title_list_by_device(
        handle: Handle,
        device_name: *const ffi::c_char,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByDeviceType
    #[doc(alias = "MCP_TitleListByDeviceType")]
    #[link_name = "MCP_TitleListByDeviceType"]
    pub unsafe fn title_list_by_device_type(
        handle: Handle,
        device_type: DeviceType,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByAppAndDevice
    #[doc(alias = "MCP_TitleListByAppAndDevice")]
    #[link_name = "MCP_TitleListByAppAndDevice"]
    pub unsafe fn title_list_by_app_and_device(
        handle: Handle,
        app_type: AppType,
        device_name: *const ffi::c_char,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByAppAndDeviceType
    #[doc(alias = "MCP_TitleListByAppAndDeviceType")]
    #[link_name = "MCP_TitleListByAppAndDeviceType"]
    pub unsafe fn title_list_by_app_and_device_type(
        handle: Handle,
        app_type: AppType,
        device_type: DeviceType,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_TitleListByUniqueIdAndIndexedDeviceAndAppType
    #[doc(alias = "MCP_TitleListByUniqueIdAndIndexedDeviceAndAppType")]
    #[link_name = "MCP_TitleListByUniqueIdAndIndexedDeviceAndAppType"]
    pub unsafe fn title_list_by_unique_id_and_indexed_device_and_app_type(
        handle: Handle,
        unique_id: u32,
        indexed_device: *const ffi::c_char,
        unk0x60: u8,
        app_type: AppType,
        count: *mut u32,
        titles: *mut TitleListType,
        titles_size: u32,
    ) -> Error;

    /// MCP_CompatLoadAVFile
    #[doc(alias = "MCP_CompatLoadAVFile")]
    #[link_name = "MCP_CompatLoadAVFile"]
    pub unsafe fn compat_load_av_file(
        handle: Handle,
        buffer: *mut ffi::c_void,
        size: *mut u32,
        file: CompatAvFile,
    ) -> Error;

    /// MCP_TriggerCrashLogCollection
    #[doc(alias = "MCP_TriggerCrashLogCollection")]
    #[link_name = "MCP_TriggerCrashLogCollection"]
    pub unsafe fn trigger_crash_log_collection(handle: Handle) -> Error;

    /// MCP_ChangeEcoSettings
    #[doc(alias = "MCP_ChangeEcoSettings")]
    #[link_name = "MCP_ChangeEcoSettings"]
    pub unsafe fn change_eco_settings(
        handle: Handle,
        enable: u32,
        max_on_time: u32,
        default_off_time: u16,
    ) -> Error;
}
