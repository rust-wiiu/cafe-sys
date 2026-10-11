//! Play Data Manager (PDM)
//!
//! Dates are days since 2000-01-01, play times are in minutes.

use super::Result;
use crate::ffi;

/// PlayDiaryFlags
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayDiaryFlags(pub u16);

impl PlayDiaryFlags {
    pub const DEFAULT: Self = Self(0x0000);
    /// Played in Wii mode. The title id is `u64::MAX` in that case.
    pub const PLAYED_IN_WII_MODE: Self = Self(0x0100);
    pub const UNKNOWN_0A00: Self = Self(0x0a00);
}

/// PlayDiary
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayDiary {
    pub title_id: u64,
    pub playtime: u32,
    pub date: u16,
    pub flags: PlayDiaryFlags,
}

/// PlayStats
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayStats {
    pub title_id: u64,
    pub playtime: u32,
    pub times_played: u16,
    pub first_time_played: u16,
    pub last_time_played: u16,
    _unk: ffi::unknown<2>,
}

/// PlayEvent
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayEvent {
    pub title_id: u64,
    _unk: ffi::unknown<12>,
}

/// PlayLog
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayLog {
    pub title_id: u64,
    _unk: ffi::unknown<16>,
}

unsafe extern "C" {
    /// PDMInitialize
    #[doc(alias = "PDMInitialize")]
    #[link_name = "PDMInitialize"]
    pub unsafe fn init() -> Result;

    /// PDMFinalize
    #[doc(alias = "PDMFinalize")]
    #[link_name = "PDMFinalize"]
    pub unsafe fn deinit();

    /// PDMCloseAllFiles
    #[doc(alias = "PDMCloseAllFiles")]
    #[link_name = "PDMCloseAllFiles"]
    pub unsafe fn close_all_files();

    /// PDMNotifySetTimeBeginEvent
    #[doc(alias = "PDMNotifySetTimeBeginEvent")]
    #[link_name = "PDMNotifySetTimeBeginEvent"]
    pub unsafe fn notify_set_time_begin_event();

    /// PDMNotifySetTimeEndEvent
    #[doc(alias = "PDMNotifySetTimeEndEvent")]
    #[link_name = "PDMNotifySetTimeEndEvent"]
    pub unsafe fn notify_set_time_end_event();

    /// Convert
    #[doc(alias = "Convert")]
    #[link_name = "Convert__Q2_2nn3pdmFi"]
    pub unsafe fn convert(user_id: i32) -> Result;

    /// WaitForConvertDone
    #[doc(alias = "WaitForConvertDone")]
    #[link_name = "WaitForConvertDone__Q2_2nn3pdmFv"]
    pub unsafe fn wait_for_convert_done() -> Result;

    /// GetPlayDiaryMaxLength
    #[doc(alias = "GetPlayDiaryMaxLength")]
    #[link_name = "GetPlayDiaryMaxLength__Q2_2nn3pdmFPi"]
    pub unsafe fn get_play_diary_max_length(max_len: *mut i32) -> Result;

    /// GetPlayDiaryLength
    #[doc(alias = "GetPlayDiaryLength")]
    #[link_name = "GetPlayDiaryLength__Q2_2nn3pdmFPii"]
    pub unsafe fn get_play_diary_length(len: *mut i32, user_id: i32) -> Result;

    /// GetPlayDiaryStart
    #[doc(alias = "GetPlayDiaryStart")]
    #[link_name = "GetPlayDiaryStart__Q2_2nn3pdmFPii"]
    pub unsafe fn get_play_diary_start(start: *mut i32, user_id: i32) -> Result;

    /// GetPlayDiary
    #[doc(alias = "GetPlayDiary")]
    #[link_name = "GetPlayDiary__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayDiaryiT3"]
    pub unsafe fn get_play_diary(
        count: *mut i32,
        diaries: *mut PlayDiary,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayDiary
    ///
    /// Like [get_play_diary], without returning the number of entries read.
    #[doc(alias = "GetPlayDiary")]
    #[link_name = "GetPlayDiary__Q2_2nn3pdmFPQ3_2nn3pdm9PlayDiaryiT2"]
    pub unsafe fn get_play_diary_without_count(
        diaries: *mut PlayDiary,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayEventMaxLength
    #[doc(alias = "GetPlayEventMaxLength")]
    #[link_name = "GetPlayEventMaxLength__Q2_2nn3pdmFPi"]
    pub unsafe fn get_play_event_max_length(max_len: *mut i32) -> Result;

    /// GetPlayEvent
    #[doc(alias = "GetPlayEvent")]
    #[link_name = "GetPlayEvent__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayEventiT3"]
    pub unsafe fn get_play_event(
        count: *mut i32,
        events: *mut PlayEvent,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayLogMaxLength
    #[doc(alias = "GetPlayLogMaxLength")]
    #[link_name = "GetPlayLogMaxLength__Q2_2nn3pdmFPi"]
    pub unsafe fn get_play_log_max_length(max_len: *mut i32) -> Result;

    /// GetPlayLogLength
    #[doc(alias = "GetPlayLogLength")]
    #[link_name = "GetPlayLogLength__Q2_2nn3pdmFPii"]
    pub unsafe fn get_play_log_length(len: *mut i32, user_id: i32) -> Result;

    /// GetPlayLogStart
    #[doc(alias = "GetPlayLogStart")]
    #[link_name = "GetPlayLogStart__Q2_2nn3pdmFPii"]
    pub unsafe fn get_play_log_start(start: *mut i32, user_id: i32) -> Result;

    /// GetPlayLog
    #[doc(alias = "GetPlayLog")]
    #[link_name = "GetPlayLog__Q2_2nn3pdmFPiPQ3_2nn3pdm7PlayLogiT3"]
    pub unsafe fn get_play_log(
        count: *mut i32,
        logs: *mut PlayLog,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayLog
    ///
    /// Like [get_play_log], without returning the number of entries read.
    #[doc(alias = "GetPlayLog")]
    #[link_name = "GetPlayLog__Q2_2nn3pdmFPQ3_2nn3pdm7PlayLogiT2"]
    pub unsafe fn get_play_log_without_count(
        logs: *mut PlayLog,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayStatsMaxLength
    #[doc(alias = "GetPlayStatsMaxLength")]
    #[link_name = "GetPlayStatsMaxLength__Q2_2nn3pdmFPi"]
    pub unsafe fn get_play_stats_max_length(max_len: *mut i32) -> Result;

    /// GetPlayStatsLength
    #[doc(alias = "GetPlayStatsLength")]
    #[link_name = "GetPlayStatsLength__Q2_2nn3pdmFPii"]
    pub unsafe fn get_play_stats_length(len: *mut i32, user_id: i32) -> Result;

    /// GetPlayStatsOfTitleId
    #[doc(alias = "GetPlayStatsOfTitleId")]
    #[link_name = "GetPlayStatsOfTitleId__Q2_2nn3pdmFPQ3_2nn3pdm9PlayStatsiUL"]
    pub unsafe fn get_play_stats_of_title_id(
        stats: *mut PlayStats,
        user_id: i32,
        title_id: u64,
    ) -> Result;

    /// GetPlayStats
    #[doc(alias = "GetPlayStats")]
    #[link_name = "GetPlayStats__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayStatsiT3"]
    pub unsafe fn get_play_stats(
        count: *mut i32,
        stats: *mut PlayStats,
        user_id: i32,
        max_count: i32,
    ) -> Result;

    /// GetPlayStats
    ///
    /// Like [get_play_stats], without returning the number of entries read.
    #[doc(alias = "GetPlayStats")]
    #[link_name = "GetPlayStats__Q2_2nn3pdmFPQ3_2nn3pdm9PlayStatsiT2"]
    pub unsafe fn get_play_stats_without_count(
        stats: *mut PlayStats,
        user_id: i32,
        max_count: i32,
    ) -> Result;
}

imports_section!(
    "nn_pdm",
    [
        "ClearPlayData__Q2_2nn3pdmFUi",
        "CloseAllFiles__Q2_2nn3pdmFv",
        "Convert__Q2_2nn3pdmFi",
        "CreateWiiReturnSaveData__Q2_2nn3pdmFi",
        "Finalize__Q2_2nn3pdmFv",
        "GetPlayDiaryLength__Q2_2nn3pdmFPii",
        "GetPlayDiaryMaxLength__Q2_2nn3pdmFPi",
        "GetPlayDiaryStart__Q2_2nn3pdmFPii",
        "GetPlayDiary__Q2_2nn3pdmFPQ3_2nn3pdm9PlayDiaryiT2",
        "GetPlayDiary__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayDiaryiT3",
        "GetPlayEventMaxLength__Q2_2nn3pdmFPi",
        "GetPlayEvent__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayEventiT3",
        "GetPlayLogEx__Q2_2nn3pdmFPiPQ3_2nn3pdm7PlayLogiT3Q3_2nn3pdm12TitleIdStyle",
        "GetPlayLogLength__Q2_2nn3pdmFPii",
        "GetPlayLogMaxLength__Q2_2nn3pdmFPi",
        "GetPlayLogStart__Q2_2nn3pdmFPii",
        "GetPlayLog__Q2_2nn3pdmFPQ3_2nn3pdm7PlayLogiT2",
        "GetPlayLog__Q2_2nn3pdmFPiPQ3_2nn3pdm7PlayLogiT3",
        "GetPlayStatsLength__Q2_2nn3pdmFPii",
        "GetPlayStatsMaxLength__Q2_2nn3pdmFPi",
        "GetPlayStatsOfTitleId__Q2_2nn3pdmFPQ3_2nn3pdm9PlayStatsiUL",
        "GetPlayStats__Q2_2nn3pdmFPQ3_2nn3pdm9PlayStatsiT2",
        "GetPlayStats__Q2_2nn3pdmFPiPQ3_2nn3pdm9PlayStatsiT3",
        "Initialize__Q2_2nn3pdmFv",
        "NotifyChangeAccountBeginEvent__Q2_2nn3pdmFi",
        "NotifyChangeAccountBeginEvent__Q2_2nn3pdmFv",
        "NotifyChangeAccountEndEvent__Q2_2nn3pdmFi",
        "NotifyChangeAccountEndEvent__Q2_2nn3pdmFv",
        "NotifyEnterCompatibleModeEvent__Q2_2nn3pdmFv",
        "NotifyEnterHaiModeEvent__Q2_2nn3pdmFv",
        "NotifyPlayEvent__Q2_2nn3pdmFUi",
        "NotifyPlayEvent__Q2_2nn3pdmFUiN21UL",
        "NotifyPlayEvent__Q2_2nn3pdmFUii",
        "NotifySetTimeBeginEvent__Q2_2nn3pdmFv",
        "NotifySetTimeEndEvent__Q2_2nn3pdmFv",
        "PDMCloseAllFiles",
        "PDMFinalize",
        "PDMInitialize",
        "PDMNotifyChangeAccountBeginEvent",
        "PDMNotifyChangeAccountEndEvent",
        "PDMNotifyEnterCompatibleModeEvent",
        "PDMNotifyEnterHaiModeEvent",
        "PDMNotifyPlayEvent",
        "PDMNotifySetTimeBeginEvent",
        "PDMNotifySetTimeEndEvent",
        "SetPlayDiary__Q2_2nn3pdmFPCQ3_2nn3pdm9PlayDiaryiN22",
        "SetPlayEvent__Q2_2nn3pdmFPCQ3_2nn3pdm9PlayEventiT2",
        "SetPlayLog__Q2_2nn3pdmFPCQ3_2nn3pdm7PlayLogiN22",
        "SetPlayStats__Q2_2nn3pdmFPCQ3_2nn3pdm9PlayStatsiT2",
        "WaitForConvertDone__Q2_2nn3pdmFv"
    ],
    []
);
