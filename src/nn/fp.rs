//! Friends Presence (FP)

use super::{
    Result,
    act::{ACCOUNT_ID_SIZE, MII_NAME_SIZE, PrincipalId},
    ffl::StoreData,
};
use crate::ffi;

/// NotificationType
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationType(pub u32);

impl NotificationType {
    pub const ONLINE: Self = Self(1);
    pub const OFFLINE: Self = Self(2);
    pub const PREFERENCE: Self = Self(3);
    pub const FRIEND_ONLINE: Self = Self(4);
    pub const FRIEND_OFFLINE: Self = Self(5);
    pub const FRIEND_PRESENCE: Self = Self(6);
    pub const FRIEND_MII: Self = Self(7);
    pub const FRIEND_PROFILE: Self = Self(8);
    pub const FRIEND_ADDED: Self = Self(9);
    pub const FRIEND_REMOVED: Self = Self(10);
    pub const MY_REQUEST_ADDED: Self = Self(11);
    pub const MY_REQUEST_REMOVED: Self = Self(12);
    pub const MY_REQUEST_UPDATED: Self = Self(13);
    pub const BLACKLIST_ADDED: Self = Self(14);
    pub const BLACKLIST_REMOVED: Self = Self(15);
    pub const BLACKLIST_UPDATED: Self = Self(16);
    pub const REQUEST_ADDED: Self = Self(17);
    pub const REQUEST_REMOVED: Self = Self(18);
}

/// DateTime
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u8,
}

/// GameKey
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameKey {
    pub title_id: u64,
    pub unk_0x8: u16,
    pub unk_0xc: u32,
}

/// RecentPlayRecord
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecentPlayRecord {
    pub principal_id: PrincipalId,
    pub unk_0x04: u16,
    pub in_game_name: [u16; ACCOUNT_ID_SIZE],
    pub my_mii_name: [u16; ACCOUNT_ID_SIZE],
}

/// RecentPlayRecordEx
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecentPlayRecordEx {
    pub play_record: RecentPlayRecord,
    pub game: GameKey,
    pub date: DateTime,
}

/// Preference
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Preference {
    pub unk_0x00: bool,
    pub unk_0x01: bool,
    pub unk_0x02: bool,
    _pad: ffi::unknown<1>,
}

/// GameMode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameMode {
    pub join_availability_flag: ffi::c_bool,
    pub matchmake_system_type: u32,
    pub join_game_id: u32,
    pub join_game_mode: u32,
    /// Only set if `join_availability_flag` is set.
    pub owner_principal_id: PrincipalId,
    pub join_group_id: u32,
    pub application_arg: [u8; 0x14],
}

/// Presence
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Presence {
    pub game_mode: GameMode,
    pub region: u8,
    pub language: u8,
    pub platform: u8,
}

/// FriendPresence
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FriendPresence {
    pub presence: Presence,
    pub is_online: bool,
    pub is_valid: bool,
}

/// MyPresence
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MyPresence {
    pub presence: Presence,
    pub presence_text: [u16; 64],
}

/// BasicInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicInfo(pub ffi::unknown<0x68>);

/// BlackListedPrincipal
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackListedPrincipal(pub ffi::unknown<0xB0>);

/// Profile
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Profile(pub ffi::unknown<0x4>);

/// Comment
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment {
    pub unk_0x00: u8,
    pub unk_0x01: u8,
    pub comment: [u16; 17],
}

/// FriendData
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FriendData(pub ffi::unknown<0x228>);

/// FriendRequest
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FriendRequest(pub ffi::unknown<0x164>);

/// GameModeDescription
///
/// Size unknown.
#[repr(C)]
pub struct GameModeDescription {
    _private: [u8; 0],
}

/// FPAsyncCallback
#[doc(alias = "FPAsyncCallback")]
pub type AsyncCallback =
    Option<unsafe extern "C" fn(result: *const Result, context: *mut ffi::c_void)>;

/// NotificationHandlerFn
#[doc(alias = "NotificationHandlerFn")]
pub type NotificationHandler = Option<
    unsafe extern "C" fn(kind: NotificationType, principal: PrincipalId, context: *mut ffi::c_void),
>;

unsafe extern "C" {
    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q2_2nn2fpFv"]
    pub unsafe fn init() -> Result;

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q2_2nn2fpFv"]
    pub unsafe fn deinit() -> Result;

    /// InitializeAdmin
    #[doc(alias = "InitializeAdmin")]
    #[link_name = "InitializeAdmin__Q2_2nn2fpFv"]
    pub unsafe fn init_admin() -> Result;

    /// FinalizeAdmin
    #[doc(alias = "FinalizeAdmin")]
    #[link_name = "FinalizeAdmin__Q2_2nn2fpFv"]
    pub unsafe fn deinit_admin() -> Result;

    /// IsInitialized
    #[doc(alias = "IsInitialized")]
    #[link_name = "IsInitialized__Q2_2nn2fpFv"]
    pub unsafe fn is_initialized() -> bool;

    /// IsInitializedAdmin
    #[doc(alias = "IsInitializedAdmin")]
    #[link_name = "IsInitializedAdmin__Q2_2nn2fpFv"]
    pub unsafe fn is_initialized_admin() -> bool;

    /// LoginAsync
    #[doc(alias = "LoginAsync")]
    #[link_name = "LoginAsync__Q2_2nn2fpFPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn login_async(callback: AsyncCallback, context: *mut ffi::c_void) -> Result;

    /// Logout
    #[doc(alias = "Logout")]
    #[link_name = "Logout__Q2_2nn2fpFv"]
    pub unsafe fn logout() -> Result;

    /// HasLoggedIn
    #[doc(alias = "HasLoggedIn")]
    #[link_name = "HasLoggedIn__Q2_2nn2fpFv"]
    pub unsafe fn has_logged_in() -> bool;

    /// IsOnline
    #[doc(alias = "IsOnline")]
    #[link_name = "IsOnline__Q2_2nn2fpFv"]
    pub unsafe fn is_online() -> bool;

    /// RegisterAccountAsync
    #[doc(alias = "RegisterAccountAsync")]
    #[link_name = "RegisterAccountAsync__Q2_2nn2fpFPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn register_account_async(
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// ResultToErrorCode
    #[doc(alias = "ResultToErrorCode")]
    #[link_name = "ResultToErrorCode__Q2_2nn2fpFQ2_2nn6Result"]
    pub unsafe fn result_to_error_code(result: *const Result) -> u32;

    /// SetNotificationHandler
    #[doc(alias = "SetNotificationHandler")]
    #[link_name = "SetNotificationHandler__Q2_2nn2fpFUiPFQ3_2nn2fp16NotificationTypeUiPv_vPv"]
    pub unsafe fn set_notification_handler(
        mask: u32,
        handler: NotificationHandler,
        context: *mut ffi::c_void,
    ) -> Result;

    /// GetMyPrincipalId
    #[doc(alias = "GetMyPrincipalId")]
    #[link_name = "GetMyPrincipalId__Q2_2nn2fpFv"]
    pub unsafe fn get_my_principal_id() -> PrincipalId;

    /// GetMyAccountId
    #[doc(alias = "GetMyAccountId")]
    #[link_name = "GetMyAccountId__Q2_2nn2fpFPc"]
    pub unsafe fn get_my_account_id(account_id: *mut [ffi::c_char; ACCOUNT_ID_SIZE]) -> Result;

    /// GetMyScreenName
    #[doc(alias = "GetMyScreenName")]
    #[link_name = "GetMyScreenName__Q2_2nn2fpFPw"]
    pub unsafe fn get_my_screen_name(name: *mut [u16; MII_NAME_SIZE]) -> Result;

    /// GetMyMii
    #[doc(alias = "GetMyMii")]
    #[link_name = "GetMyMii__Q2_2nn2fpFP12FFLStoreData"]
    pub unsafe fn get_my_mii(mii: *mut StoreData) -> Result;

    /// GetMyProfile
    #[doc(alias = "GetMyProfile")]
    #[link_name = "GetMyProfile__Q2_2nn2fpFPQ3_2nn2fp7Profile"]
    pub unsafe fn get_my_profile(profile: *mut Profile) -> Result;

    /// GetMyComment
    #[doc(alias = "GetMyComment")]
    #[link_name = "GetMyComment__Q2_2nn2fpFPQ3_2nn2fp7Comment"]
    pub unsafe fn get_my_comment(comment: *mut Comment) -> Result;

    /// GetMyPreference
    #[doc(alias = "GetMyPreference")]
    #[link_name = "GetMyPreference__Q2_2nn2fpFPQ3_2nn2fp10Preference"]
    pub unsafe fn get_my_preference(preference: *mut Preference) -> Result;

    /// GetMyPresence
    #[doc(alias = "GetMyPresence")]
    #[link_name = "GetMyPresence__Q2_2nn2fpFPQ3_2nn2fp10MyPresence"]
    pub unsafe fn get_my_presence(presence: *mut MyPresence) -> Result;

    /// GetMyPlayingGame
    #[doc(alias = "GetMyPlayingGame")]
    #[link_name = "GetMyPlayingGame__Q2_2nn2fpFPQ3_2nn2fp7GameKey"]
    pub unsafe fn get_my_playing_game(game: *mut GameKey) -> Result;

    /// IsPreferenceValid
    #[doc(alias = "IsPreferenceValid")]
    #[link_name = "IsPreferenceValid__Q2_2nn2fpFv"]
    pub unsafe fn is_preference_valid() -> bool;

    /// UpdatePreferenceAsync
    #[doc(alias = "UpdatePreferenceAsync")]
    #[link_name = "UpdatePreferenceAsync__Q2_2nn2fpFPCQ3_2nn2fp10PreferencePFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn update_preference_async(
        preference: *const Preference,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// UpdateCommentAsync
    #[doc(alias = "UpdateCommentAsync")]
    #[link_name = "UpdateCommentAsync__Q2_2nn2fpFPCwPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn update_comment_async(
        comment: *const u16,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// UpdateMiiAsync
    #[doc(alias = "UpdateMiiAsync")]
    #[link_name = "UpdateMiiAsync__Q2_2nn2fpFPC12FFLStoreDataPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn update_mii_async(
        mii: *const StoreData,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// UpdateMiiAsync
    #[doc(alias = "UpdateMiiAsync")]
    #[link_name = "UpdateMiiAsync__Q2_2nn2fpFPC12FFLStoreDataPCwPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn update_mii_with_name_async(
        mii: *const StoreData,
        name: *const u16,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// UpdateGameMode
    #[doc(alias = "UpdateGameMode")]
    #[link_name = "UpdateGameMode__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw"]
    pub unsafe fn update_game_mode(game_mode: *const GameMode, description: *const u16) -> Result;

    /// UpdateGameMode
    #[doc(alias = "UpdateGameMode")]
    #[link_name = "UpdateGameMode__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCwUi"]
    pub unsafe fn update_game_mode_with_unk(
        game_mode: *const GameMode,
        description: *const u16,
        unk: u32,
    ) -> Result;

    /// UpdateGameModeEx
    #[doc(alias = "UpdateGameModeEx")]
    #[link_name = "UpdateGameModeEx__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw"]
    pub unsafe fn update_game_mode_ex(
        game_mode: *const GameMode,
        description: *const u16,
    ) -> Result;

    /// UpdateGameModeForOverlayApplication
    #[doc(alias = "UpdateGameModeForOverlayApplication")]
    #[link_name = "UpdateGameModeForOverlayApplication__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw"]
    pub unsafe fn update_game_mode_for_overlay_application(
        game_mode: *const GameMode,
        description: *const u16,
    ) -> Result;

    /// UpdateGameModeDescription
    #[doc(alias = "UpdateGameModeDescription")]
    #[link_name = "UpdateGameModeDescription__Q2_2nn2fpFPCw"]
    pub unsafe fn update_game_mode_description(description: *const u16) -> Result;

    /// UpdatePlayingGame
    #[doc(alias = "UpdatePlayingGame")]
    #[link_name = "UpdatePlayingGame__Q2_2nn2fpFPCQ3_2nn2fp7GameKeyUi"]
    pub unsafe fn update_playing_game(game: *const GameKey, unk: u32) -> Result;

    /// UpdatePlayingOverlayApplication
    #[doc(alias = "UpdatePlayingOverlayApplication")]
    #[link_name = "UpdatePlayingOverlayApplication__Q2_2nn2fpFPCQ3_2nn2fp7GameKeyUi"]
    pub unsafe fn update_playing_overlay_application(game: *const GameKey, unk: u32) -> Result;

    /// SetInvitationParameter
    #[doc(alias = "SetInvitationParameter")]
    #[link_name = "SetInvitationParameter__Q2_2nn2fpFPQ3_2nn2fp8GameModePCUiUib"]
    pub unsafe fn set_invitation_parameter(
        game_mode: *mut GameMode,
        principals: *const PrincipalId,
        count: u32,
        unk: bool,
    ) -> Result;

    /// IsInvitation
    #[doc(alias = "IsInvitation")]
    #[link_name = "IsInvitation__Q2_2nn2fpFPCQ3_2nn2fp8GameModeUiT2"]
    pub unsafe fn is_invitation(
        game_mode: *const GameMode,
        my_principal_id: PrincipalId,
        unk: u32,
    ) -> bool;

    /// IsJoinable
    #[doc(alias = "IsJoinable")]
    #[link_name = "IsJoinable__Q2_2nn2fpFPCQ3_2nn2fp14FriendPresenceUL"]
    pub unsafe fn is_joinable(presence: *const FriendPresence, unk: u64) -> bool;

    /// IsJoinableForFriendListViewer
    #[doc(alias = "IsJoinableForFriendListViewer")]
    #[link_name = "IsJoinableForFriendListViewer__Q2_2nn2fpFPCQ3_2nn2fp14FriendPresenceUiUL"]
    pub unsafe fn is_joinable_for_friend_list_viewer(
        presence: *const FriendPresence,
        unk1: u32,
        unk2: u64,
    ) -> bool;

    /// IsJoinableForFriendListViewer
    #[doc(alias = "IsJoinableForFriendListViewer")]
    #[link_name = "IsJoinableForFriendListViewer__Q2_2nn2fpFPCQ3_2nn2fp8PresenceUiUL"]
    pub unsafe fn is_joinable_for_friend_list_viewer_presence(
        presence: *const Presence,
        unk1: u32,
        unk2: u64,
    ) -> bool;

    /// GetFriendList
    #[doc(alias = "GetFriendList")]
    #[link_name = "GetFriendList__Q2_2nn2fpFPUiT1UiT3"]
    pub unsafe fn get_friend_list(
        principals: *mut PrincipalId,
        count: *mut u32,
        unk: u32,
        max_count: u32,
    ) -> Result;

    /// GetFriendListAll
    #[doc(alias = "GetFriendListAll")]
    #[link_name = "GetFriendListAll__Q2_2nn2fpFPUiT1UiT3"]
    pub unsafe fn get_friend_list_all(
        principals: *mut PrincipalId,
        count: *mut u32,
        unk: u32,
        max_count: u32,
    ) -> Result;

    /// GetFriendListEx
    #[doc(alias = "GetFriendListEx")]
    #[link_name = "GetFriendListEx__Q2_2nn2fpFPQ3_2nn2fp10FriendDataPCUiUi"]
    pub unsafe fn get_friend_list_ex(
        data: *mut FriendData,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendAccountId
    #[doc(alias = "GetFriendAccountId")]
    #[link_name = "GetFriendAccountId__Q2_2nn2fpFPA17_cPCUiUi"]
    pub unsafe fn get_friend_account_id(
        account_ids: *mut [ffi::c_char; ACCOUNT_ID_SIZE],
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendScreenName
    #[doc(alias = "GetFriendScreenName")]
    #[link_name = "GetFriendScreenName__Q2_2nn2fpFPA11_wPCUiUibPUc"]
    pub unsafe fn get_friend_screen_name(
        names: *mut [u16; MII_NAME_SIZE],
        principals: *const PrincipalId,
        count: u32,
        unk1: bool,
        unk2: *mut u8,
    ) -> Result;

    /// GetFriendMii
    #[doc(alias = "GetFriendMii")]
    #[link_name = "GetFriendMii__Q2_2nn2fpFP12FFLStoreDataPCUiUi"]
    pub unsafe fn get_friend_mii(
        miis: *mut StoreData,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendPresence
    #[doc(alias = "GetFriendPresence")]
    #[link_name = "GetFriendPresence__Q2_2nn2fpFPQ3_2nn2fp14FriendPresencePCUiUi"]
    pub unsafe fn get_friend_presence(
        presences: *mut FriendPresence,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendPresenceEx
    #[doc(alias = "GetFriendPresenceEx")]
    #[link_name = "GetFriendPresenceEx__Q2_2nn2fpFPQ3_2nn2fp14FriendPresencePCUiUi"]
    pub unsafe fn get_friend_presence_ex(
        presences: *mut FriendPresence,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendProfile
    #[doc(alias = "GetFriendProfile")]
    #[link_name = "GetFriendProfile__Q2_2nn2fpFPQ3_2nn2fp7ProfilePCUiUi"]
    pub unsafe fn get_friend_profile(
        profiles: *mut Profile,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendComment
    #[doc(alias = "GetFriendComment")]
    #[link_name = "GetFriendComment__Q2_2nn2fpFPQ3_2nn2fp7CommentPCUiUi"]
    pub unsafe fn get_friend_comment(
        comments: *mut Comment,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendApprovalTime
    #[doc(alias = "GetFriendApprovalTime")]
    #[link_name = "GetFriendApprovalTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi"]
    pub unsafe fn get_friend_approval_time(
        times: *mut DateTime,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendSortTime
    #[doc(alias = "GetFriendSortTime")]
    #[link_name = "GetFriendSortTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi"]
    pub unsafe fn get_friend_sort_time(
        times: *mut DateTime,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendRelationship
    #[doc(alias = "GetFriendRelationship")]
    #[link_name = "GetFriendRelationship__Q2_2nn2fpFPUcPCUiUi"]
    pub unsafe fn get_friend_relationship(
        relationships: *mut u8,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendPlayingGame
    #[doc(alias = "GetFriendPlayingGame")]
    #[link_name = "GetFriendPlayingGame__Q2_2nn2fpFPQ3_2nn2fp7GameKeyPQ3_2nn2fp19GameModeDescriptionPCUiUi"]
    pub unsafe fn get_friend_playing_game(
        games: *mut GameKey,
        descriptions: *mut GameModeDescription,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetBasicInfoAsync
    #[doc(alias = "GetBasicInfoAsync")]
    #[link_name = "GetBasicInfoAsync__Q2_2nn2fpFPQ3_2nn2fp9BasicInfoPCUiUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn get_basic_info_async(
        infos: *mut BasicInfo,
        principals: *const PrincipalId,
        count: u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AddFriendAsync
    #[doc(alias = "AddFriendAsync")]
    #[link_name = "AddFriendAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_friend_async(
        principal: PrincipalId,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AddFriendAsync
    #[doc(alias = "AddFriendAsync")]
    #[link_name = "AddFriendAsync__Q2_2nn2fpFPCcPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_friend_by_account_id_async(
        account_id: *const ffi::c_char,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// RemoveFriendAsync
    #[doc(alias = "RemoveFriendAsync")]
    #[link_name = "RemoveFriendAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn remove_friend_async(
        principal: PrincipalId,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// DeleteFriendFlagsAsync
    #[doc(alias = "DeleteFriendFlagsAsync")]
    #[link_name = "DeleteFriendFlagsAsync__Q2_2nn2fpFPCUiUiT2PFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn delete_friend_flags_async(
        principals: *const PrincipalId,
        count: u32,
        flags: u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// IsFriendRequestAllowed
    #[doc(alias = "IsFriendRequestAllowed")]
    #[link_name = "IsFriendRequestAllowed__Q2_2nn2fpFv"]
    pub unsafe fn is_friend_request_allowed() -> bool;

    /// IsRequestBlockForced
    #[doc(alias = "IsRequestBlockForced")]
    #[link_name = "IsRequestBlockForced__Q2_2nn2fpFv"]
    pub unsafe fn is_request_block_forced() -> bool;

    /// GetRequestBlockSettingAsync
    #[doc(alias = "GetRequestBlockSettingAsync")]
    #[link_name = "GetRequestBlockSettingAsync__Q2_2nn2fpFPUcPCUiUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn get_request_block_setting_async(
        settings: *mut u8,
        principals: *const PrincipalId,
        count: u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// GetFriendRequestList
    #[doc(alias = "GetFriendRequestList")]
    #[link_name = "GetFriendRequestList__Q2_2nn2fpFPUiT1UiT3"]
    pub unsafe fn get_friend_request_list(
        principals: *mut PrincipalId,
        count: *mut u32,
        unk: u32,
        max_count: u32,
    ) -> Result;

    /// GetFriendRequestListEx
    #[doc(alias = "GetFriendRequestListEx")]
    #[link_name = "GetFriendRequestListEx__Q2_2nn2fpFPQ3_2nn2fp13FriendRequestPCUiUi"]
    pub unsafe fn get_friend_request_list_ex(
        requests: *mut FriendRequest,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendRequestAccountId
    #[doc(alias = "GetFriendRequestAccountId")]
    #[link_name = "GetFriendRequestAccountId__Q2_2nn2fpFPA17_cPCUiUi"]
    pub unsafe fn get_friend_request_account_id(
        account_ids: *mut [ffi::c_char; ACCOUNT_ID_SIZE],
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetFriendRequestMessageId
    #[doc(alias = "GetFriendRequestMessageId")]
    #[link_name = "GetFriendRequestMessageId__Q2_2nn2fpFPULPCUiUi"]
    pub unsafe fn get_friend_request_message_id(
        message_ids: *mut u64,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// AddFriendRequestAsync
    #[doc(alias = "AddFriendRequestAsync")]
    #[link_name = "AddFriendRequestAsync__Q2_2nn2fpFUiUcPCwT2T3PFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_friend_request_async(
        principal: PrincipalId,
        unk1: u8,
        message: *const u16,
        unk2: u8,
        unk3: *const u16,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AddFriendRequestAsync
    #[doc(alias = "AddFriendRequestAsync")]
    #[link_name = "AddFriendRequestAsync__Q2_2nn2fpFPCQ3_2nn2fp18RecentPlayRecordExPCwPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_friend_request_from_record_async(
        record: *const RecentPlayRecordEx,
        message: *const u16,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AddFriendRequestNoTitleAsync
    #[doc(alias = "AddFriendRequestNoTitleAsync")]
    #[link_name = "AddFriendRequestNoTitleAsync__Q2_2nn2fpFUiUcPCwT2T3PFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_friend_request_no_title_async(
        principal: PrincipalId,
        unk1: u8,
        message: *const u16,
        unk2: u8,
        unk3: *const u16,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AcceptFriendRequestAsync
    #[doc(alias = "AcceptFriendRequestAsync")]
    #[link_name = "AcceptFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn accept_friend_request_async(
        message_id: u64,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// DenyFriendRequestAsync
    #[doc(alias = "DenyFriendRequestAsync")]
    #[link_name = "DenyFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn deny_friend_request_async(
        message_id: u64,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// CancelFriendRequestAsync
    #[doc(alias = "CancelFriendRequestAsync")]
    #[link_name = "CancelFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn cancel_friend_request_async(
        message_id: u64,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// DeleteFriendRequestAsync
    #[doc(alias = "DeleteFriendRequestAsync")]
    #[link_name = "DeleteFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn delete_friend_request_async(
        message_id: u64,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// MarkFriendRequestsAsReceivedAsync
    #[doc(alias = "MarkFriendRequestsAsReceivedAsync")]
    #[link_name = "MarkFriendRequestsAsReceivedAsync__Q2_2nn2fpFPCULUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn mark_friend_requests_as_received_async(
        message_ids: *const u64,
        count: u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// GetBlackList
    #[doc(alias = "GetBlackList")]
    #[link_name = "GetBlackList__Q2_2nn2fpFPUiT1UiT3"]
    pub unsafe fn get_black_list(
        principals: *mut PrincipalId,
        count: *mut u32,
        unk: u32,
        max_count: u32,
    ) -> Result;

    /// GetBlackListEx
    #[doc(alias = "GetBlackListEx")]
    #[link_name = "GetBlackListEx__Q2_2nn2fpFPQ3_2nn2fp20BlackListedPrincipalPCUiUi"]
    pub unsafe fn get_black_list_ex(
        entries: *mut BlackListedPrincipal,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetBlackListAccountId
    #[doc(alias = "GetBlackListAccountId")]
    #[link_name = "GetBlackListAccountId__Q2_2nn2fpFPA17_cPCUiUi"]
    pub unsafe fn get_black_list_account_id(
        account_ids: *mut [ffi::c_char; ACCOUNT_ID_SIZE],
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// GetBlackListAdditionalTime
    #[doc(alias = "GetBlackListAdditionalTime")]
    #[link_name = "GetBlackListAdditionalTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi"]
    pub unsafe fn get_black_list_additional_time(
        times: *mut DateTime,
        principals: *const PrincipalId,
        count: u32,
    ) -> Result;

    /// AddBlackListAsync
    #[doc(alias = "AddBlackListAsync")]
    #[link_name = "AddBlackListAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_black_list_async(
        principal: PrincipalId,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// AddBlackListAsync
    #[doc(alias = "AddBlackListAsync")]
    #[link_name = "AddBlackListAsync__Q2_2nn2fpFUiPCQ3_2nn2fp7GameKeyPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn add_black_list_with_game_async(
        principal: PrincipalId,
        game: *const GameKey,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// RemoveBlackListAsync
    #[doc(alias = "RemoveBlackListAsync")]
    #[link_name = "RemoveBlackListAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn remove_black_list_async(
        principal: PrincipalId,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// GetRecentPlayRecord
    #[doc(alias = "GetRecentPlayRecord")]
    #[link_name = "GetRecentPlayRecord__Q2_2nn2fpFPQ3_2nn2fp18RecentPlayRecordExPUiUiT3"]
    pub unsafe fn get_recent_play_record(
        records: *mut RecentPlayRecordEx,
        count: *mut u32,
        unk: u32,
        max_count: u32,
    ) -> Result;

    /// AddRecentPlayRecord
    #[doc(alias = "AddRecentPlayRecord")]
    #[link_name = "AddRecentPlayRecord__Q2_2nn2fpFPCQ3_2nn2fp16RecentPlayRecordUi"]
    pub unsafe fn add_recent_play_record(records: *const RecentPlayRecord, count: u32) -> Result;

    /// AddRecentPlayRecordEx
    #[doc(alias = "AddRecentPlayRecordEx")]
    #[link_name = "AddRecentPlayRecordEx__Q2_2nn2fpFPCQ3_2nn2fp18RecentPlayRecordExUi"]
    pub unsafe fn add_recent_play_record_ex(
        records: *const RecentPlayRecordEx,
        count: u32,
    ) -> Result;

    /// DeleteRecentPlayRecordAll
    #[doc(alias = "DeleteRecentPlayRecordAll")]
    #[link_name = "DeleteRecentPlayRecordAll__Q2_2nn2fpFv"]
    pub unsafe fn delete_recent_play_record_all() -> Result;

    /// IsRecentPlayRecordCorrupted
    #[doc(alias = "IsRecentPlayRecordCorrupted")]
    #[link_name = "IsRecentPlayRecordCorrupted__Q2_2nn2fpFv"]
    pub unsafe fn is_recent_play_record_corrupted() -> bool;

    /// CheckSettingStatusAsync
    #[doc(alias = "CheckSettingStatusAsync")]
    #[link_name = "CheckSettingStatusAsync__Q2_2nn2fpFPUcPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn check_setting_status_async(
        status: *mut u8,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// DetectNatPropertiesAsync
    #[doc(alias = "DetectNatPropertiesAsync")]
    #[link_name = "DetectNatPropertiesAsync__Q2_2nn2fpFPUcT1PUiPFQ2_2nn6ResultPv_vPv"]
    pub unsafe fn detect_nat_properties_async(
        unk1: *mut u8,
        unk2: *mut u8,
        unk3: *mut u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Result;

    /// DeleteSaveDirectory
    #[doc(alias = "DeleteSaveDirectory")]
    #[link_name = "DeleteSaveDirectory__Q2_2nn2fpFUi"]
    pub unsafe fn delete_save_directory(unk: u32) -> Result;

    /// UnlockParentalControlTemporarily
    #[doc(alias = "UnlockParentalControlTemporarily")]
    #[link_name = "UnlockParentalControlTemporarily__Q2_2nn2fpFPCc"]
    pub unsafe fn unlock_parental_control_temporarily(pin: *const [ffi::c_char; 5]) -> Result;

    /// SetLedEventMask
    #[doc(alias = "SetLedEventMask")]
    #[link_name = "SetLedEventMask__Q2_2nn2fpFUi"]
    pub unsafe fn set_led_event_mask(mask: u32) -> Result;

    /// GetLastLedEvent
    #[doc(alias = "GetLastLedEvent")]
    #[link_name = "GetLastLedEvent__Q2_2nn2fpFPUiT1"]
    pub unsafe fn get_last_led_event(unk1: *mut u32, unk2: *mut u32) -> Result;

    /// ClearLedEvent
    #[doc(alias = "ClearLedEvent")]
    #[link_name = "ClearLedEvent__Q2_2nn2fpFv"]
    pub unsafe fn clear_led_event() -> Result;
}

imports_section!(
    "nn_fp",
    [
        "AcceptFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv",
        "AddBlackListAsync__Q2_2nn2fpFUiPCQ3_2nn2fp7GameKeyPFQ2_2nn6ResultPv_vPv",
        "AddBlackListAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv",
        "AddFriendAsync__Q2_2nn2fpFPCcPFQ2_2nn6ResultPv_vPv",
        "AddFriendAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv",
        "AddFriendRequestAsync__Q2_2nn2fpFPCQ3_2nn2fp18RecentPlayRecordExPCwPFQ2_2nn6ResultPv_vPv",
        "AddFriendRequestAsync__Q2_2nn2fpFUiUcPCwT2T3PFQ2_2nn6ResultPv_vPv",
        "AddFriendRequestNoTitleAsync__Q2_2nn2fpFUiUcPCwT2T3PFQ2_2nn6ResultPv_vPv",
        "AddRecentPlayRecordEx__Q2_2nn2fpFPCQ3_2nn2fp18RecentPlayRecordExUi",
        "AddRecentPlayRecord__Q2_2nn2fpFPCQ3_2nn2fp16RecentPlayRecordUi",
        "CancelFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv",
        "CheckSettingStatusAsync__Q2_2nn2fpFPUcPFQ2_2nn6ResultPv_vPv",
        "ClearLedEvent__Q2_2nn2fpFv",
        "DeleteFriendFlagsAsync__Q2_2nn2fpFPCUiUiT2PFQ2_2nn6ResultPv_vPv",
        "DeleteFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv",
        "DeleteRecentPlayRecordAll__Q2_2nn2fpFv",
        "DeleteSaveDirectory__Q2_2nn2fpFUi",
        "DenyFriendRequestAsync__Q2_2nn2fpFULPFQ2_2nn6ResultPv_vPv",
        "DetectNatPropertiesAsync__Q2_2nn2fpFPUcT1PUiPFQ2_2nn6ResultPv_vPv",
        "FinalizeAdmin__Q2_2nn2fpFv",
        "Finalize__Q2_2nn2fpFv",
        "GetBasicInfoAsync__Q2_2nn2fpFPQ3_2nn2fp9BasicInfoPCUiUiPFQ2_2nn6ResultPv_vPv",
        "GetBlackListAccountId__Q2_2nn2fpFPA17_cPCUiUi",
        "GetBlackListAdditionalTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi",
        "GetBlackListEx__Q2_2nn2fpFPQ3_2nn2fp20BlackListedPrincipalPCUiUi",
        "GetBlackList__Q2_2nn2fpFPUiT1UiT3",
        "GetFriendAccountId__Q2_2nn2fpFPA17_cPCUiUi",
        "GetFriendApprovalTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi",
        "GetFriendComment__Q2_2nn2fpFPQ3_2nn2fp7CommentPCUiUi",
        "GetFriendListAll__Q2_2nn2fpFPUiT1UiT3",
        "GetFriendListEx__Q2_2nn2fpFPQ3_2nn2fp10FriendDataPCUiUi",
        "GetFriendList__Q2_2nn2fpFPUiT1UiT3",
        "GetFriendMii__Q2_2nn2fpFP12FFLStoreDataPCUiUi",
        "GetFriendPlayingGame__Q2_2nn2fpFPQ3_2nn2fp7GameKeyPQ3_2nn2fp19GameModeDescriptionPCUiUi",
        "GetFriendPresenceEx__Q2_2nn2fpFPQ3_2nn2fp14FriendPresencePCUiUi",
        "GetFriendPresence__Q2_2nn2fpFPQ3_2nn2fp14FriendPresencePCUiUi",
        "GetFriendProfile__Q2_2nn2fpFPQ3_2nn2fp7ProfilePCUiUi",
        "GetFriendRelationship__Q2_2nn2fpFPUcPCUiUi",
        "GetFriendRequestAccountId__Q2_2nn2fpFPA17_cPCUiUi",
        "GetFriendRequestListEx__Q2_2nn2fpFPQ3_2nn2fp13FriendRequestPCUiUi",
        "GetFriendRequestList__Q2_2nn2fpFPUiT1UiT3",
        "GetFriendRequestMessageId__Q2_2nn2fpFPULPCUiUi",
        "GetFriendScreenName__Q2_2nn2fpFPA11_wPCUiUibPUc",
        "GetFriendSortTime__Q2_2nn2fpFPQ3_2nn2fp8DateTimePCUiUi",
        "GetLastLedEvent__Q2_2nn2fpFPUiT1",
        "GetMyAccountId__Q2_2nn2fpFPc",
        "GetMyComment__Q2_2nn2fpFPQ3_2nn2fp7Comment",
        "GetMyMii__Q2_2nn2fpFP12FFLStoreData",
        "GetMyPlayingGame__Q2_2nn2fpFPQ3_2nn2fp7GameKey",
        "GetMyPreference__Q2_2nn2fpFPQ3_2nn2fp10Preference",
        "GetMyPresence__Q2_2nn2fpFPQ3_2nn2fp10MyPresence",
        "GetMyPrincipalId__Q2_2nn2fpFv",
        "GetMyProfile__Q2_2nn2fpFPQ3_2nn2fp7Profile",
        "GetMyScreenName__Q2_2nn2fpFPw",
        "GetRecentPlayRecord__Q2_2nn2fpFPQ3_2nn2fp18RecentPlayRecordExPUiUiT3",
        "GetRequestBlockSettingAsync__Q2_2nn2fpFPUcPCUiUiPFQ2_2nn6ResultPv_vPv",
        "HasLoggedIn__Q2_2nn2fpFv",
        "InitializeAdmin__Q2_2nn2fpFv",
        "Initialize__Q2_2nn2fpFv",
        "IsFriendRequestAllowed__Q2_2nn2fpFv",
        "IsInitializedAdmin__Q2_2nn2fpFv",
        "IsInitialized__Q2_2nn2fpFv",
        "IsInvitation__Q2_2nn2fpFPCQ3_2nn2fp8GameModeUiT2",
        "IsJoinableForFriendListViewer__Q2_2nn2fpFPCQ3_2nn2fp14FriendPresenceUiUL",
        "IsJoinableForFriendListViewer__Q2_2nn2fpFPCQ3_2nn2fp8PresenceUiUL",
        "IsJoinable__Q2_2nn2fpFPCQ3_2nn2fp14FriendPresenceUL",
        "IsOnline__Q2_2nn2fpFv",
        "IsPreferenceValid__Q2_2nn2fpFv",
        "IsRecentPlayRecordCorrupted__Q2_2nn2fpFv",
        "IsRequestBlockForced__Q2_2nn2fpFv",
        "LoginAsync__Q2_2nn2fpFPFQ2_2nn6ResultPv_vPv",
        "Logout__Q2_2nn2fpFv",
        "MarkFriendRequestsAsReceivedAsync__Q2_2nn2fpFPCULUiPFQ2_2nn6ResultPv_vPv",
        "RegisterAccountAsync__Q2_2nn2fpFPFQ2_2nn6ResultPv_vPv",
        "RemoveBlackListAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv",
        "RemoveFriendAsync__Q2_2nn2fpFUiPFQ2_2nn6ResultPv_vPv",
        "ResultToErrorCode__Q2_2nn2fpFQ2_2nn6Result",
        "SetInvitationParameter__Q2_2nn2fpFPQ3_2nn2fp8GameModePCUiUib",
        "SetLedEventMask__Q2_2nn2fpFUi",
        "SetNotificationHandler__Q2_2nn2fpFUiPFQ3_2nn2fp16NotificationTypeUiPv_vPv",
        "UnlockParentalControlTemporarily__Q2_2nn2fpFPCc",
        "UpdateCommentAsync__Q2_2nn2fpFPCwPFQ2_2nn6ResultPv_vPv",
        "UpdateGameModeDescription__Q2_2nn2fpFPCw",
        "UpdateGameModeEx__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw",
        "UpdateGameModeForOverlayApplication__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw",
        "UpdateGameMode__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCw",
        "UpdateGameMode__Q2_2nn2fpFPCQ3_2nn2fp8GameModePCwUi",
        "UpdateMiiAsync__Q2_2nn2fpFPC12FFLStoreDataPCwPFQ2_2nn6ResultPv_vPv",
        "UpdateMiiAsync__Q2_2nn2fpFPC12FFLStoreDataPFQ2_2nn6ResultPv_vPv",
        "UpdatePlayingGame__Q2_2nn2fpFPCQ3_2nn2fp7GameKeyUi",
        "UpdatePlayingOverlayApplication__Q2_2nn2fpFPCQ3_2nn2fp7GameKeyUi",
        "UpdatePreferenceAsync__Q2_2nn2fpFPCQ3_2nn2fp10PreferencePFQ2_2nn6ResultPv_vPv"
    ],
    []
);
