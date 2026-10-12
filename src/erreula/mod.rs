//! Error Viewer (erreula)
//!
//! System error and message dialog, drawn by the application each frame.

use crate::coreinit::fs;
use crate::ffi;

pub use crate::swkbd::{ControllerInfo, LanguageType as LangType, RegionType, State};

/// Error code shown by the error viewer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode(pub i32);

impl ErrorCode {
    pub const ERROR_CODE_KPAD_READ_ERR_NO_CONTROLLER: Self = Self(1510101);
    pub const ERROR_CODE_KPAD_WBC_ERR_NO_BATTERY: Self = Self(1510201);
    pub const ERROR_CODE_KPAD_WBC_ERR_WEIGHT_OVER: Self = Self(1510204);
    pub const ERROR_CODE_WPAD_ERR_NO_CONTROLLER: Self = Self(1520100);
    pub const ERROR_CODE_VPAD_READ_ERR_NO_CONTROLLER: Self = Self(1650101);
    pub const ERROR_CODE_GX2_DRC_DETACH: Self = Self(1650101);
    pub const ERROR_CODE_FS_STATUS_STORAGE_FULL: Self = Self(1550100);
    pub const ERROR_CODE_SAVE_STATUS_STORAGE_FULL: Self = Self(1550100);
    pub const ERROR_CODE_SAVE_STATUS_DEVICE_FULL: Self = Self(1550100);
    pub const ERROR_CODE_UDS_DISCONNECTED: Self = Self(1560101);
    pub const ERROR_CODE_UDS_NOT_FOUND_USER: Self = Self(1560102);
    pub const ERROR_CODE_DEVICE_SETTING_READ_FAILURE: Self = Self(1580101);
    pub const ERROR_CODE_DEVICE_SETTING_WRITE_FAILURE: Self = Self(1580102);
    pub const ERROR_CODE_TEMP_CREATE_STORAGE_FULL: Self = Self(1660100);
    pub const ERROR_CODE_TEMP_MODIFY_STORAGE_FULL: Self = Self(1660200);
    pub const CMN_MSG_FATAL: Self = Self(1999999);
    pub const CMN_MSG_JUMP_SETTING_TOP: Self = Self(1990010);
    pub const CMN_MSG_JUMP_SETTING_INTERNET: Self = Self(1990011);
    pub const CMN_MSG_JUMP_SETTING_DATA_MANAGE: Self = Self(1990012);
    pub const CMN_MSG_JUMP_SETTING_DATE_TIME: Self = Self(1990014);
    pub const CMN_MSG_JUMP_SETTING_UPDATE: Self = Self(1990016);
    pub const CMN_MSG_JUMP_ACCOUNT_SETTING: Self = Self(1990020);
    pub const CMN_MSG_JUMP_ACCOUNT_SETTING_2: Self = Self(1990021);
    pub const CMN_MSG_JUMP_APP_REBOOT: Self = Self(1990030);
    pub const CMN_MSG_RELEASE_PARENTAL_CONTROL: Self = Self(1990040);
    pub const CMN_MSG_NGWORD_NOT_ACCEPTABLE: Self = Self(1990101);
    pub const CMN_MSG_NGWORD_ACCEPTABLE: Self = Self(1990102);
    pub const CMN_MSG_TEXT_OVER_LIMIT: Self = Self(1990105);
    pub const CMN_MSG_TEXT_NO_INPUT: Self = Self(1990106);
    pub const CMN_MSG_NUMBER_NOT_ACCEPTABLE: Self = Self(1990110);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_1: Self = Self(1990111);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_2: Self = Self(1990112);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_3: Self = Self(1990113);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_4: Self = Self(1990114);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_5: Self = Self(1990115);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_6: Self = Self(1990116);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_7: Self = Self(1990117);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_8: Self = Self(1990118);
    pub const CMN_MSG_NUMBER_OVER_LIMIT_9: Self = Self(1990119);
    pub const CMN_MSG_NOT_MATCH_PASSWORD: Self = Self(1990120);
    pub const CMN_MSG_NOT_MATCH_EMAIL: Self = Self(1990121);
    pub const CMN_MSG_WRONG_PASSWORD: Self = Self(1990131);
    pub const CMN_MSG_WRONG_PIN_CODE: Self = Self(1990132);
    pub const CMN_MSG_WRONG_FRIEND_CODE: Self = Self(1990133);
    pub const CMN_MSG_WRONG_NNID: Self = Self(1990134);
    pub const CMN_MSG_WRONG_INPUT: Self = Self(1990135);
    pub const CMN_MSG_WIIEMAIL_NOT_ACCEPTABLE: Self = Self(1990136);
    pub const CMN_MSG_FORGET_PIN_CODE: Self = Self(1990140);
    pub const CMN_MSG_NEED_WII_REMOTE: Self = Self(1990200);
    pub const CMN_MSG_NEED_WII_NUNCHUK: Self = Self(1990201);
    pub const CMN_MSG_NEED_WII_REMOTE_PLUS: Self = Self(1990202);
    pub const CMN_MSG_NEED_WII_U_PRO_CONTROLLER: Self = Self(1990203);
    pub const CMN_MSG_NEED_WII_BALANCE_BOARD: Self = Self(1990204);
    pub const CMN_MSG_NEED_WII_CLASSIC_CONTROLLER: Self = Self(1990205);
    pub const CMN_MSG_NEED_WII_U_GAMEPAD: Self = Self(1990210);
    pub const CMN_MSG_WII_REMOTE_NO_BATTERY: Self = Self(1990301);
    pub const CMN_MSG_WII_U_GAMEPAD_NO_BATTERY: Self = Self(1990302);
    pub const CMN_MSG_WII_U_PRO_CONTROLLER_NO_BATTERY: Self = Self(1990303);
    pub const CMN_MSG_WII_BALANCE_BOARD_NO_BATTERY: Self = Self(1990304);
    pub const CMN_MSG_MII_ALREADY_SAVED: Self = Self(1990401);
    pub const CMN_MSG_MII_CANT_SAVE: Self = Self(1990402);
    pub const CMN_MSG_MII_OVER_WRITE_CONFIRM: Self = Self(1990403);
    pub const CMN_MSG_MII_NO_SPACE: Self = Self(1990404);
    pub const CMN_MSG_MII_DATABASE_BROKEN: Self = Self(1990410);
    pub const CMN_MSG_FUNCTION_PROTECTED: Self = Self(1990501);
    pub const CMN_MSG_NETWORK_FUNCTION_PROTECTED: Self = Self(1990502);
    pub const CMN_MSG_INTERNET_BROWSER_PROTECTED: Self = Self(1990503);
    pub const CMN_MSG_PROTECTION_RELEASED: Self = Self(1990510);
    pub const CMN_MSG_AUTO_POWER_DOWN: Self = Self(1990601);
    pub const CMN_MSG_AUTO_POWER_DOWN_NOTICE: Self = Self(1990602);
    pub const CMN_MSG_JUMP_SETTING_TOP_LUN: Self = Self(1991010);
    pub const CMN_MSG_JUMP_SETTING_INTERNET_LUN: Self = Self(1991011);
    pub const CMN_MSG_JUMP_SETTING_DATA_MANAGE_LUN: Self = Self(1991012);
    pub const CMN_MSG_JUMP_SETTING_DATE_TIME_LUN: Self = Self(1991014);
    pub const CMN_MSG_JUMP_SETTING_UPDATE_LUN: Self = Self(1991016);
    pub const CMN_MSG_JUMP_ACCOUNT_SETTING_LUN: Self = Self(1991020);
    pub const CMN_MSG_JUMP_ACCOUNT_SETTING_2_LUN: Self = Self(1991021);
    pub const CMN_MSG_JUMP_APP_REBOOT_LUN: Self = Self(1991030);
    pub const CMN_MSG_ACCOUNT_COUNTRY_INVALID: Self = Self(1991101);
    pub const CMN_MSG_ACCOUNT_DELETED: Self = Self(1991102);
    pub const CMN_MSG_ACCOUNT_CHANGED: Self = Self(1991103);
    pub const CMN_MSG_REREASE_PARENTAL_CONTROLL: Self = Self(1990040);
    pub const CMN_MSG_WORNG_PASSWORD: Self = Self(1990131);
    pub const CMN_MSG_WORNG_PIN_CODE: Self = Self(1990132);
    pub const CMN_MSG_WORNG_FRIEND_CODE: Self = Self(1990133);
    pub const CMN_MSG_WORNG_NNID: Self = Self(1990134);
    pub const CMN_MSG_WORNG_INPUT: Self = Self(1990135);
    pub const CMN_MSG_NEED_WII_NUNCHAK: Self = Self(1990201);
    pub const CMN_MSG_NO_COMMUNITY: Self = Self(1990701);
}

/// nn::erreula::ErrorType
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorType {
    /// Message and buttons are chosen by [ErrorArg::error_code].
    #[default]
    Code = 0,
    /// Free text without buttons.
    Message = 1,
    /// Free text with one button.
    Message1Button = 2,
    /// Free text with two buttons.
    Message2Button = 3,
}

/// nn::erreula::RenderTarget
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderTarget {
    Tv = 0,
    Drc = 1,
    #[default]
    Both = 2,
}

/// nn::erreula::ControllerType
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerType {
    #[default]
    WiiRemote0 = 0,
    WiiRemote1 = 1,
    WiiRemote2 = 2,
    WiiRemote3 = 3,
    /// No Wii Remote, the GamePad is used.
    DrcGamepad = 4,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemoteHoldType {
    #[default]
    Vertical = 0,
    Horizontal = 1,
}

/// nn::erreula::ResultType
///
/// What to do after the user closed the error viewer.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultType(pub u32);

impl ResultType {
    pub const NONE: Self = Self(0);
    /// The user closed the viewer.
    pub const FINISH: Self = Self(1);
    pub const NEXT: Self = Self(2);
    /// Hide the viewer and call [jump].
    pub const JUMP: Self = Self(3);
    pub const PIN_CODE: Self = Self(4);
}

/// nn::erreula::ErrorArg
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorArg {
    pub error_type: ErrorType,
    pub render_target: RenderTarget,
    pub controller_type: ControllerType,
    pub remote_hold_type: RemoteHoldType,
    /// Used with [ErrorType::Code].
    pub error_code: ErrorCode,
    pub frame_rate: i32,
    /// UTF-16.
    pub error_message: *const u16,
    /// Label of the only or left button, UTF-16.
    pub button1_label: *const u16,
    /// Label of the right button, UTF-16.
    pub button2_label: *const u16,
    /// UTF-16.
    pub error_title: *const u16,
    /// Draws the Wii Remote pointer on the TV.
    pub draw_dpd_cursor: bool,
}

impl Default for ErrorArg {
    fn default() -> Self {
        Self {
            error_type: ErrorType::Code,
            render_target: RenderTarget::Both,
            controller_type: ControllerType::WiiRemote0,
            remote_hold_type: RemoteHoldType::Vertical,
            error_code: ErrorCode(0),
            frame_rate: 1,
            error_message: core::ptr::null(),
            button1_label: core::ptr::null(),
            button2_label: core::ptr::null(),
            error_title: core::ptr::null(),
            draw_dpd_cursor: true,
        }
    }
}

/// nn::erreula::AppearArg
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppearArg {
    pub error_arg: ErrorArg,
}

/// nn::erreula::HomeNixSignArg
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HomeNixSignArg {
    pub frame_rate: i32,
}

impl Default for HomeNixSignArg {
    fn default() -> Self {
        Self { frame_rate: 1 }
    }
}

/// nn::erreula::GetWorkMemorySize
///
/// Size of the work memory passed to [create]. Unlike wut, this excludes the memory for loading the RPL, which is already loaded as an import.
#[doc(alias = "GetWorkMemorySize")]
#[inline]
pub const fn get_work_memory_size() -> u32 {
    0x1280000
}

const _: () = {
    assert!(size_of::<HomeNixSignArg>() == 0x4);
};

unsafe extern "C" {
    /// ErrEulaCreate
    ///
    /// `work_memory` must be [get_work_memory_size] bytes. Call [set_version] with 3 first.
    #[doc(alias = "ErrEulaCreate", alias = "Create")]
    #[link_name = "ErrEulaCreate__3RplFPUcQ3_2nn7erreula10RegionTypeQ3_2nn7erreula8LangTypeP8FSClient"]
    pub unsafe fn create(
        work_memory: *mut u8,
        region: RegionType,
        language: LangType,
        fs_client: *mut fs::Client,
    );

    /// ErrEulaDestroy
    #[doc(alias = "ErrEulaDestroy", alias = "Destroy")]
    #[link_name = "ErrEulaDestroy__3RplFv"]
    pub unsafe fn destroy();

    /// ErrEulaSetVersion
    ///
    /// Call with 3 before [create].
    #[doc(alias = "ErrEulaSetVersion")]
    #[link_name = "ErrEulaSetVersion__3RplFi"]
    pub unsafe fn set_version(version: ffi::c_int);

    /// ErrEulaCalc
    ///
    /// Call every frame.
    #[doc(alias = "ErrEulaCalc", alias = "Calc")]
    #[link_name = "ErrEulaCalc__3RplFRCQ3_2nn7erreula14ControllerInfo"]
    pub unsafe fn calc(info: *const ControllerInfo);

    /// ErrEulaDrawTV
    #[doc(alias = "ErrEulaDrawTV", alias = "DrawTV")]
    #[link_name = "ErrEulaDrawTV__3RplFv"]
    pub unsafe fn draw_tv();

    /// ErrEulaDrawDRC
    #[doc(alias = "ErrEulaDrawDRC", alias = "DrawDRC")]
    #[link_name = "ErrEulaDrawDRC__3RplFv"]
    pub unsafe fn draw_drc();

    /// ErrEulaAppearError
    #[doc(alias = "ErrEulaAppearError", alias = "AppearErrorViewer")]
    #[link_name = "ErrEulaAppearError__3RplFRCQ3_2nn7erreula9AppearArg"]
    pub unsafe fn appear_error(arg: *const AppearArg);

    /// ErrEulaDisappearError
    #[doc(alias = "ErrEulaDisappearError", alias = "DisappearErrorViewer")]
    #[link_name = "ErrEulaDisappearError__3RplFv"]
    pub unsafe fn disappear_error();

    /// ErrEulaGetStateErrorViewer
    #[doc(alias = "ErrEulaGetStateErrorViewer", alias = "GetStateErrorViewer")]
    #[link_name = "ErrEulaGetStateErrorViewer__3RplFv"]
    pub unsafe fn get_state_error_viewer() -> State;

    /// ErrEulaChangeLang
    #[doc(alias = "ErrEulaChangeLang", alias = "ChangeLangError")]
    #[link_name = "ErrEulaChangeLang__3RplFQ3_2nn7erreula8LangType"]
    pub unsafe fn change_lang(language: LangType);

    /// ErrEulaIsDecideSelectButtonError
    ///
    /// Whether the single button was pressed.
    #[doc(
        alias = "ErrEulaIsDecideSelectButtonError",
        alias = "IsDecideSelectButtonError"
    )]
    #[link_name = "ErrEulaIsDecideSelectButtonError__3RplFv"]
    pub unsafe fn is_decide_select_button_error() -> bool;

    /// ErrEulaIsDecideSelectLeftButtonError
    #[doc(
        alias = "ErrEulaIsDecideSelectLeftButtonError",
        alias = "IsDecideSelectLeftButtonError"
    )]
    #[link_name = "ErrEulaIsDecideSelectLeftButtonError__3RplFv"]
    pub unsafe fn is_decide_select_left_button_error() -> bool;

    /// ErrEulaIsDecideSelectRightButtonError
    #[doc(
        alias = "ErrEulaIsDecideSelectRightButtonError",
        alias = "IsDecideSelectRightButtonError"
    )]
    #[link_name = "ErrEulaIsDecideSelectRightButtonError__3RplFv"]
    pub unsafe fn is_decide_select_right_button_error() -> bool;

    /// ErrEulaGetSelectButtonNumError
    ///
    /// Index of the pressed button.
    #[doc(
        alias = "ErrEulaGetSelectButtonNumError",
        alias = "GetSelectButtonNumError"
    )]
    #[link_name = "ErrEulaGetSelectButtonNumError__3RplFv"]
    pub unsafe fn get_select_button_num_error() -> i32;

    /// ErrEulaGetResultType
    #[doc(alias = "ErrEulaGetResultType", alias = "GetResultType")]
    #[link_name = "ErrEulaGetResultType__3RplFv"]
    pub unsafe fn get_result_type() -> ResultType;

    /// ErrEulaGetResultCode
    #[doc(alias = "ErrEulaGetResultCode", alias = "GetResultCode")]
    #[link_name = "ErrEulaGetResultCode__3RplFv"]
    pub unsafe fn get_result_code() -> ErrorCode;

    /// ErrEulaJump
    ///
    /// Jumps to the application chosen by the result code after [ResultType::JUMP].
    #[doc(alias = "ErrEulaJump", alias = "Jump")]
    #[link_name = "ErrEulaJump__3RplFPCcUi"]
    pub unsafe fn jump(buffer: *const ffi::c_char, buffer_size: u32) -> bool;

    /// ErrEulaSetControllerRemo
    ///
    /// Switches the controller operating the viewer.
    #[doc(alias = "ErrEulaSetControllerRemo", alias = "SetControllerRemo")]
    #[link_name = "ErrEulaSetControllerRemo__3RplFQ3_2nn7erreula14ControllerType"]
    pub unsafe fn set_controller_remo(controller: ControllerType);

    /// ErrEulaIsSelectCursorActive
    #[doc(alias = "ErrEulaIsSelectCursorActive", alias = "IsSelectCursorActive")]
    #[link_name = "ErrEulaIsSelectCursorActive__3RplFv"]
    pub unsafe fn is_select_cursor_active() -> bool;

    /// ErrEulaPlayAppearSE
    ///
    /// Whether to play the sound effect when the viewer appears.
    #[doc(alias = "ErrEulaPlayAppearSE", alias = "PlayAppearSE")]
    #[link_name = "ErrEulaPlayAppearSE__3RplFb"]
    pub unsafe fn play_appear_se(play: bool);

    /// ErrEulaAppearHomeNixSign
    #[doc(alias = "ErrEulaAppearHomeNixSign", alias = "AppearHomeNixSign")]
    #[link_name = "ErrEulaAppearHomeNixSign__3RplFRCQ3_2nn7erreula14HomeNixSignArg"]
    pub unsafe fn appear_home_nix_sign(arg: *const HomeNixSignArg);

    /// ErrEulaDisappearHomeNixSign
    #[doc(alias = "ErrEulaDisappearHomeNixSign", alias = "DisappearHomeNixSign")]
    #[link_name = "ErrEulaDisappearHomeNixSign__3RplFv"]
    pub unsafe fn disappear_home_nix_sign();

    /// ErrEulaIsAppearHomeNixSign
    #[doc(alias = "ErrEulaIsAppearHomeNixSign", alias = "IsAppearHomeNixSign")]
    #[link_name = "ErrEulaIsAppearHomeNixSign__3RplFv"]
    pub unsafe fn is_appear_home_nix_sign() -> bool;
}

imports_section!(
    "erreula",
    [
        "ErrEulaAppearError__3RplFRCQ3_2nn7erreula9AppearArg",
        "ErrEulaAppearHomeNixSign__3RplFRCQ3_2nn7erreula14HomeNixSignArg",
        "ErrEulaCalc__3RplFRCQ3_2nn7erreula14ControllerInfo",
        "ErrEulaChangeLang__3RplFQ3_2nn7erreula8LangType",
        "ErrEulaCreate__3RplFPUcQ3_2nn7erreula10RegionTypeQ3_2nn7erreula8LangTypeP8FSClient",
        "ErrEulaDestroy__3RplFv",
        "ErrEulaDisappearError__3RplFv",
        "ErrEulaDisappearHomeNixSign__3RplFv",
        "ErrEulaDrawDRC__3RplFv",
        "ErrEulaDrawTV__3RplFv",
        "ErrEulaGetResultCode__3RplFv",
        "ErrEulaGetResultType__3RplFv",
        "ErrEulaGetSelectButtonNumError__3RplFv",
        "ErrEulaGetStateErrorViewer__3RplFv",
        "ErrEulaIsAppearHomeNixSign__3RplFv",
        "ErrEulaIsDecideSelectButtonError__3RplFv",
        "ErrEulaIsDecideSelectLeftButtonError__3RplFv",
        "ErrEulaIsDecideSelectRightButtonError__3RplFv",
        "ErrEulaIsSelectCursorActive__3RplFv",
        "ErrEulaJump__3RplFPCcUi",
        "ErrEulaPlayAppearSE__3RplFb",
        "ErrEulaSetControllerRemo__3RplFQ3_2nn7erreula14ControllerType",
        "ErrEulaSetVersion__3RplFi"
    ],
    []
);
