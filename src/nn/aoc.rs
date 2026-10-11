//! Add-On Content

use crate::ffi;

pub const MAX_CONTENTS_NUM: u32 = 65536;
pub const MAX_VARIATION_NUM: u32 = 256;
pub const MAX_UNIQUE_ID_NUM: u32 = 32;
pub const MAX_TITLES_NUM: u32 = MAX_UNIQUE_ID_NUM * MAX_VARIATION_NUM;
pub const MAX_CONTENT_PATH_SIZE: usize = 88;
pub const MAX_OPENABLE_TITLE_NUM: u32 = 16;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const OK: Self = Self(0);
    pub const INVALID_PARAM: Self = Self(-1);
    pub const NOT_INITIALIZED: Self = Self(-2);
    pub const ALREADY_INITIALIZED: Self = Self(-3);
    pub const TITLE_STILL_OPENED: Self = Self(-4);
    pub const NOT_OPENED: Self = Self(-8);
    pub const ALREADY_OPENED: Self = Self(-9);
    pub const CLOSE_FAILURE: Self = Self(-10);
    pub const EXCEED_LIMIT: Self = Self(-11);
    pub const NOT_FOUND_TITLE: Self = Self(-16);
    pub const NOT_FOUND_CONTENT: Self = Self(-17);
    pub const ACCESS_DENIED: Self = Self(-18);
    pub const ALREADY_LOCKED: Self = Self(-768);
    pub const NOT_LOCKED: Self = Self(-769);
    pub const INTERNAL_ERROR: Self = Self(-1024);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TitleListType {
    pub title_id: u64,
    pub group_id: u32,
    pub version: u16,
    pub path: [ffi::c_char; MAX_CONTENT_PATH_SIZE],
}

unsafe extern "C" {
    /// AOC_Initialize
    #[doc(alias = "AOC_Initialize")]
    #[link_name = "AOC_Initialize"]
    pub unsafe fn init() -> Status;

    /// AOC_Finalize
    #[doc(alias = "AOC_Finalize")]
    #[link_name = "AOC_Finalize"]
    pub unsafe fn deinit() -> Status;

    /// AOC_CalculateWorkBufferSize
    ///
    /// Pass the same `max_list_num` as to [list_title], or 0 for the other functions.
    #[doc(alias = "AOC_CalculateWorkBufferSize")]
    #[link_name = "AOC_CalculateWorkBufferSize"]
    pub unsafe fn calculate_work_buffer_size(max_list_num: u32) -> u32;

    /// AOC_ListTitle
    ///
    /// Lists the installed add-on content of the calling title.
    /// `work_buffer` must be 64-byte aligned.
    #[doc(alias = "AOC_ListTitle")]
    #[link_name = "AOC_ListTitle"]
    pub unsafe fn list_title(
        num_titles: *mut u32,
        titles: *mut TitleListType,
        max_list_num: u32,
        work_buffer: *mut ffi::c_void,
        buffer_size: u32,
    ) -> Status;

    /// AOC_OpenTitle
    ///
    /// Writes the content root directory to `path`.
    /// `work_buffer` must be 64-byte aligned.
    #[doc(alias = "AOC_OpenTitle")]
    #[link_name = "AOC_OpenTitle"]
    pub unsafe fn open_title(
        path: *mut [ffi::c_char; MAX_CONTENT_PATH_SIZE],
        target: *const TitleListType,
        work_buffer: *mut ffi::c_void,
        buffer_size: u32,
    ) -> Status;

    /// AOC_CloseTitle
    #[doc(alias = "AOC_CloseTitle")]
    #[link_name = "AOC_CloseTitle"]
    pub unsafe fn close_title(target: *const TitleListType) -> Status;

    /// AOC_DeleteContent
    #[doc(alias = "AOC_DeleteContent")]
    #[link_name = "AOC_DeleteContent"]
    pub unsafe fn delete_content(
        title_id: u64,
        content_indexes: *const u16,
        num_contents: u32,
        work_buffer: *mut ffi::c_void,
        buffer_size: u32,
    ) -> Status;

    /// AOC_GetOccupiedSize
    #[doc(alias = "AOC_GetOccupiedSize")]
    #[link_name = "AOC_GetOccupiedSize"]
    pub unsafe fn get_occupied_size(
        sizes: *mut u64,
        title_id: u64,
        content_indexes: *const u16,
        num_contents: u32,
        work_buffer: *mut ffi::c_void,
        buffer_size: u32,
    ) -> Status;

    /// AOC_GetPurchaseInfo
    #[doc(alias = "AOC_GetPurchaseInfo")]
    #[link_name = "AOC_GetPurchaseInfo"]
    pub unsafe fn get_purchase_info(
        purchased: *mut ffi::c_bool,
        title_id: u64,
        content_indexes: *const u16,
        num_contents: u32,
        work_buffer: *mut ffi::c_void,
        buffer_size: u32,
    ) -> Status;

    /// AOC_DebugRealDeviceAccess
    ///
    /// Does nothing on retail consoles.
    #[doc(alias = "AOC_DebugRealDeviceAccess")]
    #[link_name = "AOC_DebugRealDeviceAccess"]
    pub unsafe fn debug_real_device_access(enable: ffi::c_bool);

    /// AOC_GetErrorCode
    ///
    /// Error code to show to the user.
    #[doc(alias = "AOC_GetErrorCode")]
    #[link_name = "AOC_GetErrorCode"]
    pub unsafe fn get_error_code(status: Status) -> u32;
}

imports_section!(
    "nn_aoc",
    [
        "AOC_CalculateWorkBufferSize",
        "AOC_CloseTitle",
        "AOC_DebugRealDeviceAccess",
        "AOC_DeleteContent",
        "AOC_Finalize",
        "AOC_GetErrorCode",
        "AOC_GetOccupiedSize",
        "AOC_GetPurchaseInfo",
        "AOC_Initialize",
        "AOC_ListTitle",
        "AOC_LockTitle",
        "AOC_OpenTitle",
        "AOC_UnlockTitle",
        "AOC_UtilGetValidAocTitles"
    ],
    []
);
