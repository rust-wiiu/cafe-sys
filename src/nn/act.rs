//! Accounts

use super::{Result, ffl::StoreData};
use crate::ffi;

pub const ACCOUNT_ID_SIZE: usize = 17;
pub const NFS_PASSWORD_SIZE: usize = 17;
pub const MII_NAME_SIZE: usize = 11;
pub const COUNTRY_SIZE: usize = 3;

/// Number of account slots.
pub const SLOT_NUM: u8 = 12;

/// Account slot, 1 to [SLOT_NUM].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotNo(pub u8);

impl SlotNo {
    pub const INVALID: Self = Self(0);
    /// The current account.
    pub const CURRENT: Self = Self(254);
    /// Data shared by all accounts.
    pub const COMMON: Self = Self(255);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersistentId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrincipalId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleAddressId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransferableId(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalFriendCode(pub u64);

/// ACTLoadOption
#[doc(alias = "ACTLoadOption")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadOption(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiiImageType(pub i32);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    pub data: [u8; 16],
}

unsafe extern "C" {
    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q2_2nn3actFv"]
    pub unsafe fn init() -> Result;

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q2_2nn3actFv"]
    pub unsafe fn deinit() -> Result;

    /// Cancel
    #[doc(alias = "Cancel")]
    #[link_name = "Cancel__Q2_2nn3actFv"]
    pub unsafe fn cancel() -> Result;

    /// GetErrorCode
    ///
    /// Converts a result into the error code shown to the user.
    #[doc(alias = "GetErrorCode")]
    #[link_name = "GetErrorCode__Q2_2nn3actFRCQ2_2nn6Result"]
    pub unsafe fn get_error_code(result: *const Result) -> u32;

    /// GetNumOfAccounts
    #[doc(alias = "GetNumOfAccounts")]
    #[link_name = "GetNumOfAccounts__Q2_2nn3actFv"]
    pub unsafe fn get_num_of_accounts() -> u8;

    /// GetSlotNo
    ///
    /// Slot of the current account.
    #[doc(alias = "GetSlotNo")]
    #[link_name = "GetSlotNo__Q2_2nn3actFv"]
    pub unsafe fn get_slot_no() -> SlotNo;

    #[link_name = "GetSlotNoEx__Q2_2nn3actFRC7ACTUuid"]
    pub unsafe fn get_slot_no_ex(uuid: *const Uuid) -> SlotNo;

    /// GetDefaultAccount
    #[doc(alias = "GetDefaultAccount")]
    #[link_name = "GetDefaultAccount__Q2_2nn3actFv"]
    pub unsafe fn get_default_account() -> SlotNo;

    #[link_name = "GetParentalControlSlotNo__Q2_2nn3actFv"]
    pub unsafe fn get_parental_control_slot_no() -> SlotNo;

    #[link_name = "GetParentalControlSlotNoEx__Q2_2nn3actFPUcUc"]
    pub unsafe fn get_parental_control_slot_no_ex(out: *mut SlotNo, slot: SlotNo) -> Result;

    /// IsSlotOccupied
    #[doc(alias = "IsSlotOccupied")]
    #[link_name = "IsSlotOccupied__Q2_2nn3actFUc"]
    pub unsafe fn is_slot_occupied(slot: SlotNo) -> bool;

    /// LoadConsoleAccount
    #[doc(alias = "LoadConsoleAccount")]
    #[link_name = "LoadConsoleAccount__Q2_2nn3actFUc13ACTLoadOptionPCcb"]
    pub unsafe fn load_console_account(
        slot: SlotNo,
        option: LoadOption,
        unk2: *const ffi::c_char,
        unk3: bool,
    ) -> Result;

    /// IsCommitted
    #[doc(alias = "IsCommitted")]
    #[link_name = "IsCommitted__Q2_2nn3actFv"]
    pub unsafe fn is_committed() -> bool;

    /// IsCommittedEx
    #[doc(alias = "IsCommittedEx")]
    #[link_name = "IsCommittedEx__Q2_2nn3actFUc"]
    pub unsafe fn is_committed_ex(slot: SlotNo) -> bool;

    /// IsNetworkAccount
    #[doc(alias = "IsNetworkAccount")]
    #[link_name = "IsNetworkAccount__Q2_2nn3actFv"]
    pub unsafe fn is_network_account() -> bool;

    /// IsNetworkAccountEx
    #[doc(alias = "IsNetworkAccountEx")]
    #[link_name = "IsNetworkAccountEx__Q2_2nn3actFUc"]
    pub unsafe fn is_network_account_ex(slot: SlotNo) -> bool;

    /// IsPasswordCacheEnabled
    #[doc(alias = "IsPasswordCacheEnabled")]
    #[link_name = "IsPasswordCacheEnabled__Q2_2nn3actFv"]
    pub unsafe fn is_password_cache_enabled() -> bool;

    /// IsPasswordCacheEnabledEx
    #[doc(alias = "IsPasswordCacheEnabledEx")]
    #[link_name = "IsPasswordCacheEnabledEx__Q2_2nn3actFUc"]
    pub unsafe fn is_password_cache_enabled_ex(slot: SlotNo) -> bool;

    /// IsServerAccountActive
    #[doc(alias = "IsServerAccountActive")]
    #[link_name = "IsServerAccountActive__Q2_2nn3actFv"]
    pub unsafe fn is_server_account_active() -> bool;

    /// IsServerAccountActiveEx
    #[doc(alias = "IsServerAccountActiveEx")]
    #[link_name = "IsServerAccountActiveEx__Q2_2nn3actFUc"]
    pub unsafe fn is_server_account_active_ex(slot: SlotNo) -> bool;

    /// IsServerAccountDeleted
    #[doc(alias = "IsServerAccountDeleted")]
    #[link_name = "IsServerAccountDeleted__Q2_2nn3actFv"]
    pub unsafe fn is_server_account_deleted() -> bool;

    /// IsServerAccountDeletedEx
    #[doc(alias = "IsServerAccountDeletedEx")]
    #[link_name = "IsServerAccountDeletedEx__Q2_2nn3actFUc"]
    pub unsafe fn is_server_account_deleted_ex(slot: SlotNo) -> bool;

    /// HasNfsAccount
    #[doc(alias = "HasNfsAccount")]
    #[link_name = "HasNfsAccount__Q2_2nn3actFv"]
    pub unsafe fn has_nfs_account() -> bool;

    /// GetAccountId
    #[doc(alias = "GetAccountId")]
    #[link_name = "GetAccountId__Q2_2nn3actFPc"]
    pub unsafe fn get_account_id(id: *mut [ffi::c_char; ACCOUNT_ID_SIZE]) -> Result;

    /// GetAccountIdEx
    #[doc(alias = "GetAccountIdEx")]
    #[link_name = "GetAccountIdEx__Q2_2nn3actFPcUc"]
    pub unsafe fn get_account_id_ex(
        id: *mut [ffi::c_char; ACCOUNT_ID_SIZE],
        slot: SlotNo,
    ) -> Result;

    /// GetBirthday
    #[doc(alias = "GetBirthday")]
    #[link_name = "GetBirthday__Q2_2nn3actFPUsPUcT2"]
    pub unsafe fn get_birthday(year: *mut u16, month: *mut u8, day: *mut u8) -> Result;

    /// GetBirthdayEx
    #[doc(alias = "GetBirthdayEx")]
    #[link_name = "GetBirthdayEx__Q2_2nn3actFPUsPUcT2Uc"]
    pub unsafe fn get_birthday_ex(
        year: *mut u16,
        month: *mut u8,
        day: *mut u8,
        slot: SlotNo,
    ) -> Result;

    #[link_name = "GetCountry__Q2_2nn3actFPc"]
    pub unsafe fn get_country(country: *mut [ffi::c_char; COUNTRY_SIZE]) -> Result;

    #[link_name = "GetCountryEx__Q2_2nn3actFPcUc"]
    pub unsafe fn get_country_ex(country: *mut [ffi::c_char; COUNTRY_SIZE], slot: SlotNo)
    -> Result;

    /// GetDeviceHash
    #[doc(alias = "GetDeviceHash")]
    #[link_name = "GetDeviceHash__Q2_2nn3actFPUL"]
    pub unsafe fn get_device_hash(hash: *mut u64) -> Result;

    /// GetMii
    #[doc(alias = "GetMii")]
    #[link_name = "GetMii__Q2_2nn3actFP12FFLStoreData"]
    pub unsafe fn get_mii(mii: *mut StoreData) -> Result;

    /// GetMiiEx
    #[doc(alias = "GetMiiEx")]
    #[link_name = "GetMiiEx__Q2_2nn3actFP12FFLStoreDataUc"]
    pub unsafe fn get_mii_ex(mii: *mut StoreData, slot: SlotNo) -> Result;

    /// GetMiiImageEx
    #[doc(alias = "GetMiiImageEx")]
    #[link_name = "GetMiiImageEx__Q2_2nn3actFPUiPvUi15ACTMiiImageTypeUc"]
    pub unsafe fn get_mii_image_ex(
        image_size: *mut u32,
        buf: *mut ffi::c_void,
        buf_size: u32,
        image_type: MiiImageType,
        slot: SlotNo,
    ) -> Result;

    /// GetMiiName
    ///
    /// UTF-16 encoded.
    #[doc(alias = "GetMiiName")]
    #[link_name = "GetMiiName__Q2_2nn3actFPw"]
    pub unsafe fn get_mii_name(name: *mut [u16; MII_NAME_SIZE]) -> Result;

    /// GetMiiNameEx
    ///
    /// UTF-16 encoded.
    #[doc(alias = "GetMiiNameEx")]
    #[link_name = "GetMiiNameEx__Q2_2nn3actFPwUc"]
    pub unsafe fn get_mii_name_ex(name: *mut [u16; MII_NAME_SIZE], slot: SlotNo) -> Result;

    /// GetNfsPassword
    #[doc(alias = "GetNfsPassword")]
    #[link_name = "GetNfsPassword__Q2_2nn3actFPc"]
    pub unsafe fn get_nfs_password(password: *mut [ffi::c_char; NFS_PASSWORD_SIZE]) -> Result;

    /// GetNfsPasswordEx
    #[doc(alias = "GetNfsPasswordEx")]
    #[link_name = "GetNfsPasswordEx__Q2_2nn3actFPcUc"]
    pub unsafe fn get_nfs_password_ex(
        password: *mut [ffi::c_char; NFS_PASSWORD_SIZE],
        slot: SlotNo,
    ) -> Result;

    /// GetPersistentId
    #[doc(alias = "GetPersistentId")]
    #[link_name = "GetPersistentId__Q2_2nn3actFv"]
    pub unsafe fn get_persistent_id() -> PersistentId;

    /// GetPersistentIdEx
    #[doc(alias = "GetPersistentIdEx")]
    #[link_name = "GetPersistentIdEx__Q2_2nn3actFUc"]
    pub unsafe fn get_persistent_id_ex(slot: SlotNo) -> PersistentId;

    /// GetPrincipalId
    #[doc(alias = "GetPrincipalId")]
    #[link_name = "GetPrincipalId__Q2_2nn3actFv"]
    pub unsafe fn get_principal_id() -> PrincipalId;

    /// GetPrincipalIdEx
    #[doc(alias = "GetPrincipalIdEx")]
    #[link_name = "GetPrincipalIdEx__Q2_2nn3actFPUiUc"]
    pub unsafe fn get_principal_id_ex(id: *mut PrincipalId, slot: SlotNo) -> Result;

    /// GetSimpleAddressId
    #[doc(alias = "GetSimpleAddressId")]
    #[link_name = "GetSimpleAddressId__Q2_2nn3actFv"]
    pub unsafe fn get_simple_address_id() -> SimpleAddressId;

    /// GetSimpleAddressIdEx
    #[doc(alias = "GetSimpleAddressIdEx")]
    #[link_name = "GetSimpleAddressIdEx__Q2_2nn3actFPUiUc"]
    pub unsafe fn get_simple_address_id_ex(id: *mut SimpleAddressId, slot: SlotNo) -> Result;

    /// GetTransferableId
    #[doc(alias = "GetTransferableId")]
    #[link_name = "GetTransferableId__Q2_2nn3actFUi"]
    pub unsafe fn get_transferable_id(unk1: u32) -> TransferableId;

    /// GetTransferableIdEx
    #[doc(alias = "GetTransferableIdEx")]
    #[link_name = "GetTransferableIdEx__Q2_2nn3actFPULUiUc"]
    pub unsafe fn get_transferable_id_ex(
        id: *mut TransferableId,
        unk1: u32,
        slot: SlotNo,
    ) -> Result;

    /// GetUuid
    #[doc(alias = "GetUuid")]
    #[link_name = "GetUuid__Q2_2nn3actFP7ACTUuid"]
    pub unsafe fn get_uuid(uuid: *mut Uuid) -> Result;

    /// GetUuid
    #[doc(alias = "GetUuid")]
    #[link_name = "GetUuid__Q2_2nn3actFP7ACTUuidUi"]
    pub unsafe fn get_uuid_unk(uuid: *mut Uuid, unk1: u32) -> Result;

    /// GetUuidEx
    #[doc(alias = "GetUuidEx")]
    #[link_name = "GetUuidEx__Q2_2nn3actFP7ACTUuidUc"]
    pub unsafe fn get_uuid_ex(uuid: *mut Uuid, slot: SlotNo) -> Result;

    /// GetUuidEx
    #[doc(alias = "GetUuidEx")]
    #[link_name = "GetUuidEx__Q2_2nn3actFP7ACTUuidUcUi"]
    pub unsafe fn get_uuid_ex_unk(uuid: *mut Uuid, slot: SlotNo, unk1: u32) -> Result;

    #[link_name = "GenerateUuid__Q2_2nn3actFP7ACTUuid"]
    pub unsafe fn generate_uuid(uuid: *mut Uuid) -> Result;
}

imports_section!(
    "nn_act",
    [
        "AcquireAccountIdByPrincipalId__Q2_2nn3actFPA17_cPCUiUi",
        "AcquireAccountInfo__Q3_2nn3act8shimutilFUiPvT1Q4_2nn3act6detail11ACTInfoType",
        "AcquireAccountTokenEx__Q2_2nn3actFPCc",
        "AcquireAccountToken__Q2_2nn3actFv",
        "AcquireAssociatedPlatformsEx__Q2_2nn3actFPUiUc",
        "AcquireAssociatedPlatforms__Q2_2nn3actFPUi",
        "AcquireEcServiceToken__Q2_2nn3actFPc",
        "AcquireEulaList__Q2_2nn3actFPUcPvUiUc",
        "AcquireIndependentServiceTokenIgnoreParentalControl__Q2_2nn3actFPcPCc",
        "AcquireIndependentServiceTokenV2__Q2_2nn3actFP17ACTServiceTokenV2PCc",
        "AcquireIndependentServiceTokenV2__Q2_2nn3actFP17ACTServiceTokenV2PCcUi",
        "AcquireIndependentServiceTokenV2__Q2_2nn3actFP17ACTServiceTokenV2PCcUibT4",
        "AcquireIndependentServiceToken__Q2_2nn3actFPcPCc",
        "AcquireIndependentServiceToken__Q2_2nn3actFPcPCcUi",
        "AcquireIndependentServiceToken__Q2_2nn3actFPcPCcUibT4",
        "AcquireMailAddress__Q2_2nn3actFPc",
        "AcquireMii__Q2_2nn3actFP12FFLStoreDataPCUiUi",
        "AcquireNetworkTime__Q2_2nn3actFPUL27ACTAcquireNetworkTimeOption",
        "AcquireNexServiceTokenIgnoreParentalControl__Q2_2nn3actFP26ACTNexAuthenticationResultUi",
        "AcquireNexServiceToken__Q2_2nn3actFP26ACTNexAuthenticationResultUi",
        "AcquirePrincipalIdByAccountId__Q2_2nn3actFPUiPA17_CcUi",
        "AcquireTimeZoneList__Q2_2nn3actFPUcP11ACTTimeZoneUcN23",
        "AgreeEula__Q2_2nn3actFPCcRC11ACTEulaInfoRC14OSCalendarTime",
        "AgreeEula__Q2_2nn3actFPCcUcUs",
        "ApproveByCreditCard__Q2_2nn3actFPUicPCcT3UcT5N23",
        "AuthenticateDevice__Q2_2nn3actFv",
        "BindToExistentServerAccount__Q2_2nn3actFPCcT1",
        "BindToExistentServerAccount__Q2_2nn3actFPUiPCcN22",
        "BindToNewServerAccount__Q2_2nn3actFPCcN21bT4UsUcT79ACTGenderUiRC11ACTTimeZoneRC11ACTEulaInfoRC14OSCalendarTimeUi",
        "BindToNewServerAccount__Q2_2nn3actFPCcN21bT4UsUcT79ACTGenderUiRC11ACTTimeZoneRC11ACTEulaInfoRC14OSCalendarTimeUiT4",
        "BindToNewServerAccount__Q2_2nn3actFPCcN21bUsUcT69ACTGenderUiRC11ACTTimeZoneT5",
        "CancelTransfer__Q2_2nn3actFv",
        "Cancel__Q2_2nn3actFv",
        "CheckAccountId__Q2_2nn3actFPCc",
        "CheckAccountPassword__Q2_2nn3actFPCcT1",
        "CheckBirthday__Q2_2nn3actFUsUcT2",
        "CheckMailAddress__Q2_2nn3actFPCc",
        "CheckNnasNfsEnv__Q2_2nn3actFPCc",
        "CheckSubDomain__Q2_2nn3actFPCc",
        "CommitConsoleAccount__Q2_2nn3actFUc",
        "CompleteTransfer__Q2_2nn3actFv",
        "CreateConsoleAccount__Q2_2nn3actFv",
        "DeleteAccountDeviceAssociation__Q2_2nn3actFv",
        "DeleteConsoleAccount__Q2_2nn3actFUc",
        "DeleteDeviceAssociation__Q2_2nn3actFv",
        "DeleteServerAccount__Q2_2nn3actFv",
        "EmulateAccountSalvage__Q2_2nn3actFv",
        "EnableAccountPasswordCache__Q2_2nn3actFb",
        "EnableParentalControlCheck__Q2_2nn3actFb",
        "ExpireAccountToken__Q2_2nn3actFv",
        "Finalize__Q2_2nn3actFv",
        "GenerateSupportPasscode__Q3_2nn3act8shimutilFUiT1Us",
        "GenerateUuid__Q2_2nn3actFP7ACTUuid",
        "GenerateUuid__Q2_2nn3actFP7ACTUuidUi",
        "GetAccountIdEx__Q2_2nn3actFPcUc",
        "GetAccountId__Q2_2nn3actFPc",
        "GetAccountInfoEx__Q2_2nn3actFP14ACTAccountInfoUc",
        "GetAccountInfo__Q3_2nn3act8shimutilFUcPvUiQ4_2nn3act6detail11ACTInfoType",
        "GetAccountTokenStatus__Q2_2nn3actFP21ACTAccountTokenStatus",
        "GetAccountToken__Q2_2nn3actFP15ACTAccountToken",
        "GetAccountToken__Q2_2nn3actFPc",
        "GetBirthdayEx__Q2_2nn3actFPUsPUcT2Uc",
        "GetBirthday__Q2_2nn3actFPUsPUcT2",
        "GetCommonInfo__Q3_2nn3act8shimutilFPvUiQ4_2nn3act6detail11ACTInfoType",
        "GetCountryEx__Q2_2nn3actFPcUc",
        "GetCountry__Q2_2nn3actFPc",
        "GetDefaultAccount__Q2_2nn3actFv",
        "GetDefaultHostServerSettings__Q2_2nn3actFP11ACTNnasTypeP10ACTNfsTypePUc",
        "GetDefaultHostServerSettings__Q2_2nn3actFPcT1",
        "GetDeviceHash__Q2_2nn3actFPUL",
        "GetErrorCode__Q2_2nn3actFRCQ2_2nn6Result",
        "GetGenderEx__Q2_2nn3actFP9ACTGenderUc",
        "GetGender__Q2_2nn3actFv",
        "GetHostServerSettings__Q2_2nn3actFP11ACTNnasTypeP10ACTNfsTypePUcUc",
        "GetHostServerSettings__Q2_2nn3actFPcT1Uc",
        "GetLastAuthenticationResultEx__Q2_2nn3actFUc",
        "GetLastAuthenticationResult__Q2_2nn3actFv",
        "GetLocalFriendCodeEx__Q2_2nn3actFPULUc",
        "GetLocalFriendCode__Q2_2nn3actFv",
        "GetMailAddressEx__Q2_2nn3actFPcUc",
        "GetMailAddress__Q2_2nn3actFPc",
        "GetMiiEx__Q2_2nn3actFP12FFLStoreDataUc",
        "GetMiiImageEx__Q2_2nn3actFPUiPvUi15ACTMiiImageTypeUc",
        "GetMiiImageUrlEx__Q2_2nn3actFPcUc",
        "GetMiiNameEx__Q2_2nn3actFPwUc",
        "GetMiiName__Q2_2nn3actFPw",
        "GetMii__Q2_2nn3actFP12FFLStoreData",
        "GetNetworkTimeDifference__Q2_2nn3actFPL",
        "GetNextAccountId__Q2_2nn3actFPc",
        "GetNfsPasswordEx__Q2_2nn3actFPcUc",
        "GetNfsPassword__Q2_2nn3actFPc",
        "GetNumOfAccounts__Q2_2nn3actFv",
        "GetParentalControlSlotNoEx__Q2_2nn3actFPUcUc",
        "GetParentalControlSlotNo__Q2_2nn3actFv",
        "GetPersistentIdEx__Q2_2nn3actFUc",
        "GetPersistentId__Q2_2nn3actFv",
        "GetPrincipalIdEx__Q2_2nn3actFPUiUc",
        "GetPrincipalId__Q2_2nn3actFv",
        "GetSimpleAddressIdEx__Q2_2nn3actFPUiUc",
        "GetSimpleAddressId__Q2_2nn3actFv",
        "GetSlotNoEx__Q2_2nn3actFRC7ACTUuid",
        "GetSlotNoEx__Q2_2nn3actFRC7ACTUuidUi",
        "GetSlotNoEx__Q2_2nn3actFUi",
        "GetSlotNo__Q2_2nn3actFv",
        "GetStickyAccountIdEx__Q2_2nn3actFPcUc",
        "GetStickyAccountId__Q2_2nn3actFPc",
        "GetStickyPrincipalIdEx__Q2_2nn3actFPUiUc",
        "GetSupportContext__Q2_2nn3actFP17ACTSupportContext",
        "GetTimeZoneIdEx__Q2_2nn3actFPcUc",
        "GetTimeZoneId__Q2_2nn3actFPc",
        "GetTransferableIdEx__Q2_2nn3actFPULUiUc",
        "GetTransferableId__Q2_2nn3actFUi",
        "GetUtcOffsetEx__Q2_2nn3actFPLUc",
        "GetUtcOffset__Q2_2nn3actFv",
        "GetUuidEx__Q2_2nn3actFP7ACTUuidUc",
        "GetUuidEx__Q2_2nn3actFP7ACTUuidUcUi",
        "GetUuid__Q2_2nn3actFP7ACTUuid",
        "GetUuid__Q2_2nn3actFP7ACTUuidUi",
        "HasEciVirtualAccountEx__Q2_2nn3actFUc",
        "HasEciVirtualAccount__Q2_2nn3actFv",
        "HasNfsAccount__Q2_2nn3actFv",
        "HasStickyAccountIdEx__Q2_2nn3actFUc",
        "HasStickyAccountId__Q2_2nn3actFv",
        "InactivateAccountDeviceAssociation__Q2_2nn3actFv",
        "InactivateDeviceAssociation__Q2_2nn3actFv",
        "Initialize__Q2_2nn3actFv",
        "InquireAccountIdAvailability__Q2_2nn3actFPCc",
        "InquireBindingToExistentServerAccount__Q2_2nn3actFPbP12FFLStoreDataPUiT1PcT5PCcN27",
        "InquireBindingToExistentServerAccount__Q2_2nn3actFPbP12FFLStoreDataPUiT1PcT5PUsPUcT8PCcPCcPCc",
        "InquireMailAddressAvailability__Q2_2nn3actFPCc",
        "InquireNetworkTime__Q2_2nn3actFv",
        "InvalidateAccountToken__Q2_2nn3actFb",
        "IsApplicationUpdateRequired__Q2_2nn3actFv",
        "IsCommittedEx__Q2_2nn3actFUc",
        "IsCommitted__Q2_2nn3actFv",
        "IsMailAddressValidatedEx__Q2_2nn3actFUc",
        "IsMailAddressValidated__Q2_2nn3actFv",
        "IsMiiUpdatedEx__Q2_2nn3actFUc",
        "IsMiiUpdated__Q2_2nn3actFv",
        "IsNetworkAccountEx__Q2_2nn3actFUc",
        "IsNetworkAccount__Q2_2nn3actFv",
        "IsParentalControlCheckEnabled__Q2_2nn3actFv",
        "IsPasswordCacheEnabledEx__Q2_2nn3actFUc",
        "IsPasswordCacheEnabled__Q2_2nn3actFv",
        "IsServerAccountActiveEx__Q2_2nn3actFUc",
        "IsServerAccountActive__Q2_2nn3actFv",
        "IsServerAccountDeletedEx__Q2_2nn3actFUc",
        "IsServerAccountDeleted__Q2_2nn3actFv",
        "IsSlotOccupied__Q2_2nn3actFUc",
        "IsValidSupportPasscode__Q2_2nn3actFPCcRC17ACTSupportContext",
        "LoadConsoleAccount__Q2_2nn3actFUc13ACTLoadOptionPCcb",
        "MyStrlcpy__Q3_2nn3act8shimutilFPcPCcUi",
        "ReissueAccountPassword__Q2_2nn3actFv",
        "ReplaceAccountId__Q2_2nn3actFv",
        "ReserveServerAccountDeletion__Q2_2nn3actFv",
        "ReserveTransfer__Q2_2nn3actFPC13ACTDeviceInfoPCc",
        "Save__Q2_2nn3actFv",
        "SendConfirmationMailForPin__Q2_2nn3actFPCc",
        "SendConfirmationMail__Q2_2nn3actFv",
        "SendCoppaCodeMail__Q2_2nn3actFUiPcT2",
        "SendMasterKeyMailForPin__Q2_2nn3actFPCcUi",
        "SendPostingApprovalMail__Q2_2nn3actFPCc",
        "SetAccountPasswordInput__Q2_2nn3actFPCc",
        "SetApplicationUpdateRequired__Q2_2nn3actFb",
        "SetDefaultAccount__Q2_2nn3actFUc",
        "SetDefaultHostServerSettings__Q2_2nn3actF11ACTNnasType10ACTNfsTypeUc",
        "SetDefaultHostServerSettings__Q2_2nn3actFPCcT1",
        "SetHostServerSettings__Q2_2nn3actF11ACTNnasType10ACTNfsTypeUcT3",
        "SetHostServerSettings__Q2_2nn3actFPCcT1Uc",
        "SetNfsPassword__Q2_2nn3actFPCc",
        "SetPersistentIdHead__Q2_2nn3actFUi",
        "SetTransferableIdCounter__Q2_2nn3actFUs",
        "Strlcpy__tm__2_c__Q3_2nn3act6detailFPZ1ZPCZ1ZUi_Ui",
        "StrnlenChar16__Q3_2nn3act8shimutilFPCwUi",
        "SwapAccounts__Q2_2nn3actFUcT1",
        "SyncAccountInfo__Q2_2nn3actFv",
        "UnbindServerAccount__Q2_2nn3actFUcb",
        "UnloadConsoleAccount__Q2_2nn3actFv",
        "UpdateAccountInfo__Q3_2nn3act8shimutilFQ4_2nn3act6detail11ACTInfoTypePCvUi",
        "UpdateAccountPassword__Q2_2nn3actFPCc",
        "UpdateMiiData__Q2_2nn3actFUcRC12FFLStoreDataPCw",
        "UpdateMiiImage__Q3_2nn3act8shimutilFUc15ACTMiiImageTypePCvUi",
        "UpdateMii__Q2_2nn3actFUcRC12FFLStoreDataPCwPCvUiT4T5T4T5T4T5T4T5T4T5T4T5T4T5T4T5",
        "UploadMii__Q2_2nn3actFv",
        "ValidateMailAddress__Q2_2nn3actFUi"
    ],
    [
        "s_InitializeCount__Q3_2nn3act8shimutil",
        "s_IosFd__Q3_2nn3act8shimutil",
        "s_IpcClientSender__Q3_2nn3act8shimutil",
        "s_IsParentalControlCheckEnabled__Q3_2nn3act8shimutil",
        "s_Mutex__Q3_2nn3act8shimutil",
        "s_TransferableBufferAllocator__Q3_2nn3act8shimutil",
        "s_TransferableBuffer__Q3_2nn3act8shimutil"
    ]
);
