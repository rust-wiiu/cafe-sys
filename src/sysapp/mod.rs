//! System Application (sysapp)
//!
//! Launching and switching to other titles and system applications, and passing arguments between them.

use crate::ffi;

use crate::nn::ffl::StoreData;

pub use crate::coreinit::system::Upid as Pfid;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    /// No arguments were passed by the caller.
    pub const NO_ARGS: Self = Self(1);
    pub const FATAL: Self = Self(-100000);
    pub const SERIALIZE_ANCHOR_SIZE_MAX: Self = Self(-40000);
    pub const SERIALIZE_URL_SIZE_MAX: Self = Self(-50000);
}

/// SYSTEM_APP_ID
#[doc(alias = "SYSTEM_APP_ID")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemAppId {
    WiiUMenu = 0,
    SystemSettings = 1,
    ParentalControls = 2,
    UserSettings = 3,
    MiiMaker = 4,
    AccountSettings = 5,
    DailyLog = 6,
    Notifications = 7,
    HealthAndSafety = 8,
    ElectronicManual = 9,
    WiiUChat = 10,
    SoftwareDataTransfer = 11,
}

/// SYSStandardArgsIn
#[doc(alias = "SYSStandardArgsIn")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardArgsIn {
    /// Null-terminated, up to 4096 characters.
    pub anchor: *const ffi::c_char,
    pub anchor_size: u32,
}

/// SYSStandardArgs
#[doc(alias = "SYSStandardArgs")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardArgs {
    /// Anchor passed when jumping away.
    pub anchor: *const ffi::c_char,
    pub anchor_size: u32,
    /// Result passed by the application that was jumped to.
    pub result: *mut ffi::c_char,
    pub result_size: u32,
}

/// SYSStandardArgsOut
#[doc(alias = "SYSStandardArgsOut")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardArgsOut {
    pub data: *const ffi::c_void,
    pub size: u32,
}

/// SysAppBrowserArgs
#[doc(alias = "SysAppBrowserArgs", alias = "SysBrowserArgsIn")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrowserArgs {
    pub std_args: StandardArgsIn,
    /// Null-terminated URL, up to 1023 characters.
    pub url: *const ffi::c_char,
    pub url_size: u32,
}

/// SysAppBrowserArgsWithCallback
#[doc(alias = "SysAppBrowserArgsWithCallback")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrowserArgsWithCallback {
    pub browser_args: BrowserArgs,
    pub callback_url: *const ffi::c_char,
    pub callback_url_size: u32,
    /// Disables the HOME Menu in the browser.
    pub hbm_disable: ffi::c_bool,
}

/// SysAppEShopArgs
#[doc(alias = "SysAppEShopArgs")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EShopArgs {
    pub std_args: StandardArgsIn,
    pub query: *const ffi::c_char,
    pub query_size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountArgs {
    pub std_args: StandardArgsIn,
    /// Account slot to change.
    pub slot_id: u32,
}

/// SYSSettingsJumpToTarget
#[doc(alias = "SYSSettingsJumpToTarget")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsJumpTo {
    #[default]
    None = 0,
    Internet = 1,
    DataManagement = 2,
    TvRemote = 3,
    DateTime = 4,
    Country = 5,
    SystemUpdate = 6,
    /// Initial console setup.
    InitialSettings = 100,
    /// Same as [SettingsJumpTo::None].
    Unknown = 101,
    /// Wipes the console without confirmation.
    WipeConsole = 102,
    QuickStartSettings = 103,
    TvConnectionType = 104,
    /// Same as [SettingsJumpTo::DataManagement].
    DataManagement2 = 105,
    SoftwareTransfer = 255,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SettingsArgs {
    pub std_args: StandardArgsIn,
    pub jump_to: SettingsJumpTo,
}

/// SysAppMiiMakerArgs
#[doc(alias = "SysAppMiiMakerArgs")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiiMakerArgs {
    pub std_args: StandardArgsIn,
    pub slot_id: u32,
    pub mii: *mut StoreData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VodArgs {
    pub std_args: StandardArgs,
    pub url: *const ffi::c_char,
    pub url_size: u32,
}

/// SYSArgID
#[doc(alias = "SYSArgID")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArgId(pub u32);

impl ArgId {
    /// Marks the last slot.
    pub const END: Self = Self(0);
    pub const ANCHOR: Self = Self(100);
    pub const RESULT: Self = Self(101);
    pub const VOD_URL: Self = Self(200);
    pub const MIIVERSE_DATA: Self = Self(300);
    pub const JOIN_PID: Self = Self(400);
}

/// SYSArgType
#[doc(alias = "SYSArgType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArgType(pub u32);

impl ArgType {
    pub const NO_DATA: Self = Self(0);
    pub const U32: Self = Self(1);
    pub const U64: Self = Self(2);
    pub const DATA: Self = Self(3);
    pub const CONST_DATA: Self = Self(4);
}

/// Data block of an [ArgSlot].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArgData {
    pub ptr: *mut ffi::c_void,
    pub size: u32,
}

/// Value of an [ArgSlot], selected by [ArgSlot::kind].
#[repr(C)]
#[derive(Clone, Copy)]
pub union ArgValue {
    pub u32: u32,
    pub u64: u64,
    pub data: ArgData,
}

/// SYSArgDataBlock
#[doc(alias = "SYSArgDataBlock")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArgSlot {
    pub id: ArgId,
    pub kind: ArgType,
    pub value: ArgValue,
}

/// SYSCallerInfo
#[doc(alias = "SYSCallerInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallerInfo {
    pub caller: Pfid,
    pub title_id: u64,
}

/// SYSDeserializeArg
#[doc(alias = "SYSDeserializeArg")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeserializeArg {
    pub arg_name: *const ffi::c_char,
    pub size: u32,
    pub data: *mut ffi::c_void,
}

/// SYSDeserializeCallback
#[doc(alias = "SYSDeserializeCallback")]
pub type DeserializeCallback =
    Option<unsafe extern "C" fn(arg: *mut DeserializeArg, user_arg: *mut ffi::c_void)>;

const _: () = {
    assert!(size_of::<CallerInfo>() == 0x10);
};

unsafe extern "C" {
    /// SYSLaunchTitle
    #[doc(alias = "SYSLaunchTitle")]
    #[link_name = "SYSLaunchTitle"]
    pub unsafe fn launch_title(title_id: u64) -> Error;

    /// SYSRelaunchTitle
    #[doc(alias = "SYSRelaunchTitle")]
    #[link_name = "SYSRelaunchTitle"]
    pub unsafe fn relaunch_title(argc: u32, argv: *mut *mut ffi::c_char) -> Error;

    /// SYSLaunchMenu
    ///
    /// Exits to the Wii U Menu.
    #[doc(alias = "SYSLaunchMenu")]
    #[link_name = "SYSLaunchMenu"]
    pub unsafe fn launch_menu() -> Error;

    /// SYSLaunchSettings
    ///
    /// Exits to System Settings.
    #[doc(alias = "SYSLaunchSettings")]
    #[link_name = "SYSLaunchSettings"]
    pub unsafe fn launch_settings(args: *const SettingsArgs) -> Error;

    /// SYSLaunchAccount
    ///
    /// Exits to the account settings.
    #[doc(alias = "SYSLaunchAccount")]
    #[link_name = "SYSLaunchAccount"]
    pub unsafe fn launch_account(args: *const AccountArgs) -> Error;

    /// SYSLaunchMiiStudio
    ///
    /// Exits to Mii Maker.
    #[doc(alias = "SYSLaunchMiiStudio")]
    #[link_name = "SYSLaunchMiiStudio"]
    pub unsafe fn launch_mii_studio(args: *const MiiMakerArgs) -> Error;

    /// SYSSwitchTo
    #[doc(alias = "SYSSwitchTo")]
    #[link_name = "SYSSwitchTo"]
    pub unsafe fn switch_to(pfid: Pfid) -> Error;

    /// SYSSwitchToBrowser
    #[doc(alias = "SYSSwitchToBrowser")]
    #[link_name = "SYSSwitchToBrowser"]
    pub unsafe fn switch_to_browser(args: *const BrowserArgs) -> Error;

    /// SYSSwitchToBrowserForViewer
    #[doc(alias = "SYSSwitchToBrowserForViewer")]
    #[link_name = "SYSSwitchToBrowserForViewer"]
    pub unsafe fn switch_to_browser_for_viewer(args: *const BrowserArgs) -> Error;

    /// SYSSwitchToBrowserForCallbackURL
    #[doc(alias = "SYSSwitchToBrowserForCallbackURL")]
    #[link_name = "SYSSwitchToBrowserForCallbackURL"]
    pub unsafe fn switch_to_browser_for_callback_url(args: *const BrowserArgsWithCallback)
    -> Error;

    /// SYSSwitchToEManual
    #[doc(alias = "SYSSwitchToEManual")]
    #[link_name = "SYSSwitchToEManual"]
    pub unsafe fn switch_to_emanual() -> Error;

    /// SYSSwitchToEShop
    #[doc(alias = "SYSSwitchToEShop")]
    #[link_name = "SYSSwitchToEShop"]
    pub unsafe fn switch_to_eshop(args: *const EShopArgs) -> Error;

    /// SYSSwitchToEShopAocList
    ///
    /// Opens the add-on content page of the title.
    #[doc(alias = "SYSSwitchToEShopAocList")]
    #[link_name = "SYSSwitchToEShopAocList"]
    pub unsafe fn switch_to_eshop_aoc_list(args: *const StandardArgsIn) -> Error;

    /// SYSSwitchToEShopTicketList
    ///
    /// Opens the ticket list page of the title.
    #[doc(alias = "SYSSwitchToEShopTicketList")]
    #[link_name = "SYSSwitchToEShopTicketList"]
    pub unsafe fn switch_to_eshop_ticket_list(args: *const StandardArgsIn) -> Error;

    /// SYSSwitchToSyncControllerOnHBM
    ///
    /// Opens the controller pairing screen of the HOME Menu.
    #[doc(alias = "SYSSwitchToSyncControllerOnHBM")]
    #[link_name = "SYSSwitchToSyncControllerOnHBM"]
    pub unsafe fn switch_to_sync_controller_on_hbm() -> Error;

    /// SYSIsReturnable
    ///
    /// Whether [return_to_caller] can return to the caller.
    #[doc(alias = "SYSIsReturnable")]
    #[link_name = "SYSIsReturnable"]
    pub unsafe fn is_returnable() -> ffi::c_bool;

    /// SYSReturnToCaller
    #[doc(alias = "SYSReturnToCaller")]
    #[link_name = "SYSReturnToCaller"]
    pub unsafe fn return_to_caller(args: *const StandardArgsOut) -> Error;

    /// SYSGetArguments
    ///
    /// `slots` is terminated by [ArgId::END].
    #[doc(alias = "SYSGetArguments")]
    #[link_name = "SYSGetArguments"]
    pub unsafe fn get_arguments(slots: *mut ArgSlot, caller: *mut CallerInfo) -> Error;

    /// SYSGetStandardArgs
    #[doc(alias = "SYSGetStandardArgs")]
    #[link_name = "SYSGetStandardArgs"]
    pub unsafe fn get_standard_args(args: *mut StandardArgs) -> Error;

    /// SYSGetVodArgs
    #[doc(alias = "SYSGetVodArgs")]
    #[link_name = "SYSGetVodArgs"]
    pub unsafe fn get_vod_args(args: *mut VodArgs) -> Error;

    /// SYSClearSysArgs
    #[doc(alias = "SYSClearSysArgs")]
    #[link_name = "SYSClearSysArgs"]
    pub unsafe fn clear_sys_args();

    /// SYSPackArgs
    #[doc(alias = "SYSPackArgs")]
    #[link_name = "SYSPackArgs"]
    pub unsafe fn pack_args() -> Error;

    /// SYSSerializeSysArgs
    #[doc(alias = "SYSSerializeSysArgs")]
    #[link_name = "SYSSerializeSysArgs"]
    pub unsafe fn serialize_sys_args(
        arg_name: *const ffi::c_char,
        data: *const ffi::c_void,
        size: u32,
    ) -> Error;

    /// SYSSerializeSysArgsToBuffer
    #[doc(alias = "SYSSerializeSysArgsToBuffer")]
    #[link_name = "SYSSerializeSysArgsToBuffer"]
    pub unsafe fn serialize_sys_args_to_buffer(
        buffer: *mut ffi::c_char,
        bytes_written: *mut u32,
        buffer_size: u32,
        arg_name: *const ffi::c_char,
        data: *const ffi::c_void,
        data_size: u32,
    ) -> Error;

    /// SYSDeserializeSysArgs
    #[doc(alias = "SYSDeserializeSysArgs")]
    #[link_name = "SYSDeserializeSysArgs"]
    pub unsafe fn deserialize_sys_args(
        callback: DeserializeCallback,
        user_arg: *mut ffi::c_void,
    ) -> Error;

    /// SYSDeserializeSysArgsFromBlock
    #[doc(alias = "SYSDeserializeSysArgsFromBlock")]
    #[link_name = "SYSDeserializeSysArgsFromBlock"]
    pub unsafe fn deserialize_sys_args_from_block(
        block: *mut ffi::c_void,
        block_size: u32,
        callback: DeserializeCallback,
        user_arg: *mut ffi::c_void,
    ) -> Error;

    /// SYSGetCallerTitleId
    #[doc(alias = "SYSGetCallerTitleId")]
    #[link_name = "SYSGetCallerTitleId"]
    pub unsafe fn get_caller_title_id() -> u64;

    /// SYSGetCallerPFID
    #[doc(alias = "SYSGetCallerPFID")]
    #[link_name = "SYSGetCallerPFID"]
    pub unsafe fn get_caller_pfid() -> Pfid;

    /// SYSGetCallerUPID
    #[doc(alias = "SYSGetCallerUPID")]
    #[link_name = "SYSGetCallerUPID"]
    pub unsafe fn get_caller_upid() -> i32;

    /// SYSGetPFIDFromTitleID
    #[doc(alias = "SYSGetPFIDFromTitleID")]
    #[link_name = "SYSGetPFIDFromTitleID"]
    pub unsafe fn get_pfid_from_title_id(title_id: u64) -> Pfid;

    /// SYSGetUPIDFromTitleID
    #[doc(alias = "SYSGetUPIDFromTitleID")]
    #[link_name = "SYSGetUPIDFromTitleID"]
    pub unsafe fn get_upid_from_title_id(title_id: u64) -> i32;

    /// SYSCheckTitleExists
    #[doc(alias = "SYSCheckTitleExists")]
    #[link_name = "SYSCheckTitleExists"]
    pub unsafe fn check_title_exists(title_id: u64) -> ffi::c_bool;

    /// SYSCheckSystemApplicationExists
    ///
    /// Only supports [Pfid::EMANUAL], [Pfid::MINI_MIIVERSE], [Pfid::BROWSER], [Pfid::MIIVERSE] and [Pfid::ESHOP], returns false otherwise.
    #[doc(alias = "SYSCheckSystemApplicationExists")]
    #[link_name = "SYSCheckSystemApplicationExists"]
    pub unsafe fn check_system_application_exists(pfid: Pfid) -> ffi::c_bool;
}

/// Functions meant for system applications.
pub mod private {
    use super::{
        BrowserArgsWithCallback, DeserializeArg, EShopArgs, Error, Pfid, SettingsJumpTo,
        StandardArgs, StandardArgsIn, StandardArgsOut, SystemAppId,
    };
    use crate::ffi;
    use crate::nn::ffl::StoreData;

    /// _SysAppMiiMakerArgs
    #[doc(alias = "_SysAppMiiMakerArgs")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MiiMakerArgs {
        pub std_args: StandardArgsIn,
        pub mode: u32,
        pub slot_id: u32,
        pub mii: *mut StoreData,
    }

    /// SysAppSettingsArgs
    #[doc(alias = "SysAppSettingsArgs")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SettingsArgs {
        pub std_args: StandardArgsIn,
        pub jump_to: SettingsJumpTo,
        pub first_boot_kind: u32,
    }

    /// SysAppParentalArgs
    #[doc(alias = "SysAppParentalArgs")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ParentalArgs {
        pub std_args: StandardArgsIn,
        pub mode: u32,
    }

    /// SysAppNotificationArgs
    #[doc(alias = "SysAppNotificationArgs")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NotificationArgs {
        pub std_args: StandardArgsIn,
        pub notification_file: u32,
    }

    /// SysAppEManualArgs
    #[doc(alias = "SysAppEManualArgs")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EManualArgs {
        pub std_args: StandardArgsIn,
        pub title_id: u64,
    }

    unsafe extern "C" {
        /// _SYSLaunchTitleWithStdArgsInNoSplash
        #[doc(alias = "_SYSLaunchTitleWithStdArgsInNoSplash")]
        #[link_name = "_SYSLaunchTitleWithStdArgsInNoSplash"]
        pub unsafe fn launch_title_with_std_args_in_no_splash(
            title_id: u64,
            args: *const StandardArgsIn,
        ) -> Error;

        /// _SYSLaunchTitleByPathFromLauncher
        #[doc(alias = "_SYSLaunchTitleByPathFromLauncher")]
        #[link_name = "_SYSLaunchTitleByPathFromLauncher"]
        pub unsafe fn launch_title_by_path_from_launcher(
            path: *const ffi::c_char,
            unused: u32,
        ) -> Error;

        /// _SYSLaunchMenuWithCheckingAccount
        #[doc(alias = "_SYSLaunchMenuWithCheckingAccount")]
        #[link_name = "_SYSLaunchMenuWithCheckingAccount"]
        pub unsafe fn launch_menu_with_checking_account(slot: u8) -> Error;

        /// _SYSLaunchSettings
        #[doc(alias = "_SYSLaunchSettings")]
        #[link_name = "_SYSLaunchSettings"]
        pub unsafe fn launch_settings(args: *const SettingsArgs) -> Error;

        /// _SYSLaunchParental
        #[doc(alias = "_SYSLaunchParental")]
        #[link_name = "_SYSLaunchParental"]
        pub unsafe fn launch_parental(args: *const ParentalArgs) -> Error;

        /// _SYSLaunchNotifications
        #[doc(alias = "_SYSLaunchNotifications")]
        #[link_name = "_SYSLaunchNotifications"]
        pub unsafe fn launch_notifications(args: *const NotificationArgs) -> Error;

        /// _SYSLaunchMiiStudio
        #[doc(alias = "_SYSLaunchMiiStudio")]
        #[link_name = "_SYSLaunchMiiStudio"]
        pub unsafe fn launch_mii_studio(args: *const MiiMakerArgs) -> Error;

        /// _SYSSwitchTo
        #[doc(alias = "_SYSSwitchTo")]
        #[link_name = "_SYSSwitchTo"]
        pub unsafe fn switch_to(pfid: Pfid) -> Error;

        /// _SYSDirectlySwitchTo
        #[doc(alias = "_SYSDirectlySwitchTo")]
        #[link_name = "_SYSDirectlySwitchTo"]
        pub unsafe fn directly_switch_to(pfid: Pfid) -> Error;

        /// _SYSSwitchToMainApp
        #[doc(alias = "_SYSSwitchToMainApp")]
        #[link_name = "_SYSSwitchToMainApp"]
        pub unsafe fn switch_to_main_app() -> Error;

        /// _SYSSwitchToHBMWithMode
        #[doc(alias = "_SYSSwitchToHBMWithMode")]
        #[link_name = "_SYSSwitchToHBMWithMode"]
        pub unsafe fn switch_to_hbm_with_mode(mode: i32) -> Error;

        /// _SYSSwitchToOverlayFromHBM
        #[doc(alias = "_SYSSwitchToOverlayFromHBM")]
        #[link_name = "_SYSSwitchToOverlayFromHBM"]
        pub unsafe fn switch_to_overlay_from_hbm(pfid: Pfid) -> Error;

        /// _SYSSwitchToEManual
        #[doc(alias = "_SYSSwitchToEManual")]
        #[link_name = "_SYSSwitchToEManual"]
        pub unsafe fn switch_to_emanual(args: *const EManualArgs) -> Error;

        /// _SYSSwitchToEManualFromHBM
        #[doc(alias = "_SYSSwitchToEManualFromHBM")]
        #[link_name = "_SYSSwitchToEManualFromHBM"]
        pub unsafe fn switch_to_emanual_from_hbm(args: *const EManualArgs) -> Error;

        /// _SYSSwitchToEShopFromHBM
        #[doc(alias = "_SYSSwitchToEShopFromHBM")]
        #[link_name = "_SYSSwitchToEShopFromHBM"]
        pub unsafe fn switch_to_eshop_from_hbm(args: *const EShopArgs) -> Error;

        /// _SYSSwitchToBrowserForCallbackURLFromHBM
        #[doc(alias = "_SYSSwitchToBrowserForCallbackURLFromHBM")]
        #[link_name = "_SYSSwitchToBrowserForCallbackURLFromHBM"]
        pub unsafe fn switch_to_browser_for_callback_url_from_hbm(
            args: *const BrowserArgsWithCallback,
        ) -> Error;

        /// _SYSDirectlyReturnToCaller
        #[doc(alias = "_SYSDirectlyReturnToCaller")]
        #[link_name = "_SYSDirectlyReturnToCaller"]
        pub unsafe fn directly_return_to_caller(args: *const StandardArgsOut) -> Error;

        /// _SYSSerializeStandardArgsIn
        #[doc(alias = "_SYSSerializeStandardArgsIn")]
        #[link_name = "_SYSSerializeStandardArgsIn"]
        pub unsafe fn serialize_standard_args_in(args: *const StandardArgsIn) -> Error;

        /// _SYSDeserializeStandardArg
        #[doc(alias = "_SYSDeserializeStandardArg")]
        #[link_name = "_SYSDeserializeStandardArg"]
        pub unsafe fn deserialize_standard_arg(
            arg: *mut DeserializeArg,
            args: *mut StandardArgs,
        ) -> ffi::c_bool;

        /// _SYSGetSystemApplicationTitleId
        #[doc(alias = "_SYSGetSystemApplicationTitleId")]
        #[link_name = "_SYSGetSystemApplicationTitleId"]
        pub unsafe fn get_system_application_title_id(id: SystemAppId) -> u64;

        /// _SYSGetSystemApplicationTitleIdByProdArea
        #[doc(alias = "_SYSGetSystemApplicationTitleIdByProdArea")]
        #[link_name = "_SYSGetSystemApplicationTitleIdByProdArea"]
        pub unsafe fn get_system_application_title_id_by_prod_area(
            id: SystemAppId,
            prod_area: crate::coreinit::mcp::Region,
        ) -> u64;
    }
}

imports_section!(
    "sysapp",
    [
        "SYSCheckSystemApplicationExists",
        "SYSCheckTitleExists",
        "SYSClearSysArgs",
        "SYSDeserializeSysArgs",
        "SYSDeserializeSysArgsFromBlock",
        "SYSGetArguments",
        "SYSGetCallerPFID",
        "SYSGetCallerTitleId",
        "SYSGetCallerUPID",
        "SYSGetInvitationArgs",
        "SYSGetPFIDFromTitleID",
        "SYSGetStandardArgs",
        "SYSGetStandardResult",
        "SYSGetUPIDFromTitleID",
        "SYSGetVodArgs",
        "SYSIsReturnable",
        "SYSLaunchAccount",
        "SYSLaunchMenu",
        "SYSLaunchMiiStudio",
        "SYSLaunchSettings",
        "SYSLaunchTitle",
        "SYSLaunchTitleViaLauncher",
        "SYSLaunchVod",
        "SYSPackArgs",
        "SYSPreloadMiniMiiverse",
        "SYSRelaunchTitle",
        "SYSReturnToCaller",
        "SYSSerializeSysArgs",
        "SYSSerializeSysArgsToBuffer",
        "SYSSwitchTo",
        "SYSSwitchToBrowser",
        "SYSSwitchToBrowserForCallbackURL",
        "SYSSwitchToBrowserForViewer",
        "SYSSwitchToEManual",
        "SYSSwitchToEShop",
        "SYSSwitchToEShopAocList",
        "SYSSwitchToEShopTicketList",
        "SYSSwitchToSyncControllerOnHBM",
        "_SYSAppendCallerInfo",
        "_SYSDeserializeStandardArg",
        "_SYSDirectlyReturnToCaller",
        "_SYSDirectlySwitchTo",
        "_SYSGetAccountArgs",
        "_SYSGetBrowserArgs",
        "_SYSGetDebugOption",
        "_SYSGetEManualArgs",
        "_SYSGetEShopArgs",
        "_SYSGetHBMArgs",
        "_SYSGetLauncherArgs",
        "_SYSGetMiiStudioArgs",
        "_SYSGetNetAccountArgs",
        "_SYSGetNotificationsArgs",
        "_SYSGetParentalArgs",
        "_SYSGetSettingsArgs",
        "_SYSGetSystemApplicationTitleId",
        "_SYSGetSystemApplicationTitleIdByProdArea",
        "_SYSLaunchAccount",
        "_SYSLaunchAccountByProdArea",
        "_SYSLaunchAccountDirect",
        "_SYSLaunchMenuFromHBM",
        "_SYSLaunchMenuWithCheckingAccount",
        "_SYSLaunchMenuWithCheckingAccountFromHBM",
        "_SYSLaunchMenuWithPackFromHBM",
        "_SYSLaunchMiiStudio",
        "_SYSLaunchMiiStudioByProdArea",
        "_SYSLaunchMiiStudioDirect",
        "_SYSLaunchNetAccount",
        "_SYSLaunchNetAccountByProdArea",
        "_SYSLaunchNetAccountDirect",
        "_SYSLaunchNotifications",
        "_SYSLaunchNotificationsByProdArea",
        "_SYSLaunchNotificationsDirect",
        "_SYSLaunchParental",
        "_SYSLaunchParentalByProdArea",
        "_SYSLaunchParentalDirect",
        "_SYSLaunchSettings",
        "_SYSLaunchSettingsByProdArea",
        "_SYSLaunchSettingsDirect",
        "_SYSLaunchTitleByPathFromLauncher",
        "_SYSLaunchTitleDirect",
        "_SYSLaunchTitleFromLauncher",
        "_SYSLaunchTitleWithMiiverseData",
        "_SYSLaunchTitleWithPackFromHBM",
        "_SYSLaunchTitleWithPackFromLauncher",
        "_SYSLaunchTitleWithStdArgsInNoSplash",
        "_SYSLaunchVodFromLauncher",
        "_SYSLaunchWithInvitation",
        "_SYSLaunchWithInvitationFromHBM",
        "_SYSReturnToCallerWithStandardResult",
        "_SYSSerializeStandardArgsIn",
        "_SYSSetDebugOption",
        "_SYSSwitchTo",
        "_SYSSwitchToBrowserForCallbackURLFromHBM",
        "_SYSSwitchToEManual",
        "_SYSSwitchToEManualFromHBM",
        "_SYSSwitchToEShopFromHBM",
        "_SYSSwitchToHBMWithMode",
        "_SYSSwitchToMainApp",
        "_SYSSwitchToOverlayFromHBM",
        "_SYSSwitchToOverlayWithPackFromHBM"
    ],
    []
);
