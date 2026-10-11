//! Auto Connection (AC)

use super::Result;
use crate::ffi;

/// ACStatus
#[doc(alias = "ACStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const FAILED: Self = Self(-1);
    pub const OK: Self = Self(0);
    pub const PROCESSING: Self = Self(1);
}

/// ACErrorCode
#[doc(alias = "ACErrorCode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode(pub u32);

/// ACConfigId
#[doc(alias = "ACConfigId")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigId(pub u32);

impl ConfigId {
    pub const ID_1: Self = Self(1);
    pub const ID_2: Self = Self(2);
    pub const ID_3: Self = Self(3);
    pub const ID_4: Self = Self(4);
    pub const ID_5: Self = Self(5);
    pub const ID_6: Self = Self(6);
}

unsafe extern "C" {
    /// ACIsSuccess
    #[doc(alias = "ACIsSuccess")]
    #[link_name = "ACIsSuccess"]
    pub unsafe fn is_success(result: Result) -> ffi::c_bool;

    /// ACIsFailure
    #[doc(alias = "ACIsFailure")]
    #[link_name = "ACIsFailure"]
    pub unsafe fn is_failure(result: Result) -> ffi::c_bool;

    /// ACIsRetryRequired
    #[doc(alias = "ACIsRetryRequired")]
    #[link_name = "ACIsRetryRequired"]
    pub unsafe fn is_retry_required(result: Result) -> ffi::c_bool;

    /// ACInitialize
    ///
    /// Initializes the AC library.
    ///
    /// If this function has already been called, it does nothing. This function **must** be called before calling any other AC functions.
    #[doc(alias = "ACInitialize")]
    #[link_name = "ACInitialize"]
    pub unsafe fn init() -> Result;

    /// ACFinalize
    ///
    /// Deinitializes the AC library.
    ///
    /// Close any active connections with [close] before calling this function. This function asserts(?) if called more times than the [init] function.
    #[doc(alias = "ACFinalize")]
    #[link_name = "ACFinalize"]
    pub unsafe fn deinit();

    /// ACConnect
    ///
    /// Creates a network connection.
    ///
    /// Uses the default network profile configured in the system settings. Any network related functions (like `socket`) must be used after this function and before [close].
    #[doc(alias = "ACConnect")]
    #[link_name = "ACConnect"]
    pub unsafe fn connect() -> Result;

    /// ACConnectAsync
    ///
    /// Like [connect], but returns immediately. Poll [get_connect_status] and [get_connect_result].
    #[doc(alias = "ACConnectAsync")]
    #[link_name = "ACConnectAsync"]
    pub unsafe fn connect_async() -> Result;

    /// ACConnectWithConfigId
    #[doc(alias = "ACConnectWithConfigId")]
    #[link_name = "ACConnectWithConfigId"]
    pub unsafe fn connect_with_config_id(id: ConfigId) -> Result;

    /// ACConnectWithConfigIdAsync
    #[doc(alias = "ACConnectWithConfigIdAsync")]
    #[link_name = "ACConnectWithConfigIdAsync"]
    pub unsafe fn connect_with_config_id_async(id: ConfigId) -> Result;

    /// ACClose
    ///
    /// Closes a network connection.
    ///
    /// Only closes connections created by this process. Any network related functions (like `socket`) must be used after [connect] function and before this function.
    #[doc(alias = "ACClose")]
    #[link_name = "ACClose"]
    pub unsafe fn close() -> Result;

    /// ACCloseAll
    #[doc(alias = "ACCloseAll")]
    #[link_name = "ACCloseAll"]
    pub unsafe fn close_all() -> Result;

    /// ACGetStatus
    #[doc(alias = "ACGetStatus")]
    #[link_name = "ACGetStatus"]
    pub unsafe fn get_status(status: *mut Status) -> Result;

    /// ACGetConnectStatus
    #[doc(alias = "ACGetConnectStatus")]
    #[link_name = "ACGetConnectStatus"]
    pub unsafe fn get_connect_status(status: *mut Status) -> Result;

    /// ACGetCloseStatus
    #[doc(alias = "ACGetCloseStatus")]
    #[link_name = "ACGetCloseStatus"]
    pub unsafe fn get_close_status(status: *mut Status) -> Result;

    /// ACGetConnectResult
    ///
    /// Result of the last [connect_async].
    #[doc(alias = "ACGetConnectResult")]
    #[link_name = "ACGetConnectResult"]
    pub unsafe fn get_connect_result(result: *mut Result) -> Result;

    /// ACIsApplicationConnected
    #[doc(alias = "ACIsApplicationConnected")]
    #[link_name = "ACIsApplicationConnected"]
    pub unsafe fn is_application_connected(connected: *mut ffi::c_bool) -> Result;

    /// ACGetLastErrorCode
    #[doc(alias = "ACGetLastErrorCode")]
    #[link_name = "ACGetLastErrorCode"]
    pub unsafe fn get_last_error_code(code: *mut ErrorCode) -> Result;

    /// ACGetLastDetailedErrorCode
    #[doc(alias = "ACGetLastDetailedErrorCode")]
    #[link_name = "ACGetLastDetailedErrorCode"]
    pub unsafe fn get_last_detailed_error_code(code: *mut u32, command: *mut u32) -> Result;

    /// ACGetAssignedAddress
    #[doc(alias = "ACGetAssignedAddress")]
    #[link_name = "ACGetAssignedAddress"]
    pub unsafe fn get_assigned_address(address: *mut u32) -> Result;

    /// ACGetAssignedPreferedDns
    #[doc(alias = "ACGetAssignedPreferedDns")]
    #[link_name = "ACGetAssignedPreferedDns"]
    pub unsafe fn get_assigned_preferred_dns(address: *mut u32) -> Result;

    /// ACGetAssignedAlternativeDns
    #[doc(alias = "ACGetAssignedAlternativeDns")]
    #[link_name = "ACGetAssignedAlternativeDns"]
    pub unsafe fn get_assigned_alternative_dns(address: *mut u32) -> Result;

    /// ACGetAssignedGateway
    #[doc(alias = "ACGetAssignedGateway")]
    #[link_name = "ACGetAssignedGateway"]
    pub unsafe fn get_assigned_gateway(address: *mut u32) -> Result;

    /// ACGetAssignedSubnet
    #[doc(alias = "ACGetAssignedSubnet")]
    #[link_name = "ACGetAssignedSubnet"]
    pub unsafe fn get_assigned_subnet(address: *mut u32) -> Result;

    /// ACSetStartupId
    #[doc(alias = "ACSetStartupId")]
    #[link_name = "ACSetStartupId"]
    pub unsafe fn set_startup_id(id: ConfigId) -> Result;

    /// ACGetStartupId
    ///
    /// Config used by [connect].
    #[doc(alias = "ACGetStartupId")]
    #[link_name = "ACGetStartupId"]
    pub unsafe fn get_startup_id(id: *mut ConfigId) -> Result;

    /// ACIsConfigExisting
    #[doc(alias = "ACIsConfigExisting")]
    #[link_name = "ACIsConfigExisting"]
    pub unsafe fn is_config_existing(id: ConfigId, existing: *mut ffi::c_bool) -> Result;
}

imports_section!(
    "nn_ac",
    [
        "ACClearConfig",
        "ACClearConfigCache",
        "ACClose",
        "ACCloseAll",
        "ACConnect",
        "ACConnectAsync",
        "ACConnectWithConfig",
        "ACConnectWithConfigAsync",
        "ACConnectWithConfigId",
        "ACConnectWithConfigIdAsync",
        "ACDeleteConfig",
        "ACFinalize",
        "ACGetAddressObtainMode",
        "ACGetAesKey",
        "ACGetAlternativeDnsServer",
        "ACGetAssignedAddress",
        "ACGetAssignedAlternativeDns",
        "ACGetAssignedGateway",
        "ACGetAssignedPreferedDns",
        "ACGetAssignedSubnet",
        "ACGetCloseResult",
        "ACGetCloseStatus",
        "ACGetConfigStatus",
        "ACGetConnectResult",
        "ACGetConnectStatus",
        "ACGetDefaultGateway",
        "ACGetDeviceIpAddress",
        "ACGetEthernetAutoNegotiation",
        "ACGetEthernetCommunicationMethod",
        "ACGetEthernetSpeed",
        "ACGetLastDetailedErrorCode",
        "ACGetLastErrorCode",
        "ACGetMacAddress",
        "ACGetMtu",
        "ACGetNetworkInterface",
        "ACGetPrimaryDnsServer",
        "ACGetPrivacyMode",
        "ACGetProxyAuthType",
        "ACGetProxyHostName",
        "ACGetProxyNoproxyHosts",
        "ACGetProxyPassword",
        "ACGetProxyPort",
        "ACGetProxyUse",
        "ACGetProxyUserName",
        "ACGetRunningConfig",
        "ACGetRunningMacAddress",
        "ACGetSsid",
        "ACGetStartupId",
        "ACGetStatus",
        "ACGetSubnetMask",
        "ACGetTkipKey",
        "ACGetValidFlag",
        "ACGetWep104Key",
        "ACGetWep104KeyId",
        "ACGetWep40Key",
        "ACGetWep40KeyId",
        "ACGetWifiConfigureMethod",
        "ACInitialize",
        "ACIsAnyKeepingConnect",
        "ACIsApplicationConnected",
        "ACIsApplicationConnectedWithApType",
        "ACIsApplicationConnectedWithNicType",
        "ACIsConfigExisting",
        "ACIsConnectedWithApType",
        "ACIsConnectedWithNicType",
        "ACIsFailure",
        "ACIsKeepingConnect",
        "ACIsRetryRequired",
        "ACIsSuccess",
        "ACIsSystemConnectedWithApType",
        "ACIsSystemConnectedWithNicType",
        "ACReadConfig",
        "ACSetAddressObtainMode",
        "ACSetAesKey",
        "ACSetAlternativeDnsServer",
        "ACSetDefaultGateway",
        "ACSetDeviceIpAddress",
        "ACSetEthernetAutoNegotiation",
        "ACSetEthernetCommunicationMethod",
        "ACSetEthernetSpeed",
        "ACSetMtu",
        "ACSetNetworkInterface",
        "ACSetPrimaryDnsServer",
        "ACSetPrivacyMode",
        "ACSetProxyAuthType",
        "ACSetProxyHostName",
        "ACSetProxyNoproxyHosts",
        "ACSetProxyPassword",
        "ACSetProxyPort",
        "ACSetProxyUse",
        "ACSetProxyUserName",
        "ACSetSsid",
        "ACSetStartupId",
        "ACSetSubnetMask",
        "ACSetTkipKey",
        "ACSetValidFlag",
        "ACSetWep104Key",
        "ACSetWep104KeyId",
        "ACSetWep40Key",
        "ACSetWep40KeyId",
        "ACSetWifiConfigureMethod",
        "ACStoreConfig",
        "ACWriteConfig",
        "BeginLocalConnection__Q2_2nn2acFb",
        "CheckInfraConnectionEnabled__Q2_2nn2acFPQ3_2nn2ac10InfraCheck",
        "ClearAutoConnectionStatus__Q2_2nn2acFv",
        "ClearConfigCache__Q2_2nn2acFv",
        "ClearConfig__Q2_2nn2acFP16netconf_profile_",
        "CloseAll__Q2_2nn2acFv",
        "Close__Q2_2nn2acFv",
        "ConnectAsync__Q2_2nn2acFPC16netconf_profile_",
        "ConnectAsync__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "ConnectAsync__Q2_2nn2acFv",
        "ConnectWithRetry__Q2_2nn2acFv",
        "Connect__Q2_2nn2acFPC16netconf_profile_",
        "Connect__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "Connect__Q2_2nn2acFv",
        "DeleteConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "EndLocalConnection__Q2_2nn2acFv",
        "Finalize__Q2_2nn2acFv",
        "GetAddressObtainMode__Q2_2nn2acFPC16netconf_profile_",
        "GetAesKey__Q2_2nn2acFPC16netconf_profile_Pi",
        "GetAlternativeDnsServer__Q2_2nn2acFPC16netconf_profile_",
        "GetAssignedAddress__Q2_2nn2acFPUl",
        "GetAssignedAlternativeDns__Q2_2nn2acFPUl",
        "GetAssignedGateway__Q2_2nn2acFPUl",
        "GetAssignedPreferedDns__Q2_2nn2acFPUl",
        "GetAssignedSubnet__Q2_2nn2acFPUl",
        "GetAutoConnectionResult__Q2_2nn2acFPQ2_2nn6Result",
        "GetAutoConnectionStatus__Q2_2nn2acFPQ3_2nn2ac20AutoConnectionStatusPUi",
        "GetCloseResult__Q2_2nn2acFPQ2_2nn6Result",
        "GetCloseStatus__Q2_2nn2acFPQ3_2nn2ac6Status",
        "GetCompatId__Q2_2nn2acFPQ3_2nn2ac11ConfigIdNum",
        "GetConfigStatus__Q2_2nn2acFPUiT1",
        "GetConnectResult__Q2_2nn2acFPQ2_2nn6Result",
        "GetConnectStatus__Q2_2nn2acFPQ3_2nn2ac6Status",
        "GetDefaultGateway__Q2_2nn2acFPC16netconf_profile_",
        "GetDeviceIpAddress__Q2_2nn2acFPC16netconf_profile_",
        "GetEthernetAutoNegotiation__Q2_2nn2acFPC16netconf_profile_",
        "GetEthernetCommunicationMethod__Q2_2nn2acFPC16netconf_profile_",
        "GetEthernetSpeed__Q2_2nn2acFPC16netconf_profile_",
        "GetLastDetailedErrorCode__Q2_2nn2acFPUiT1",
        "GetLastErrorCode__Q2_2nn2acFPUi",
        "GetMacAddress__Q2_2nn2acFPQ3_2nn2ac10MacAddress",
        "GetMacAddress__Q2_2nn2acFQ3_2nn2ac7NicTypePQ3_2nn2ac10MacAddress",
        "GetMtu__Q2_2nn2acFPC16netconf_profile_",
        "GetNetworkInterface__Q2_2nn2acFPC16netconf_profile_",
        "GetPrimaryDnsServer__Q2_2nn2acFPC16netconf_profile_",
        "GetPrivacyMode__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyAuthType__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyHostName__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyNoproxyHosts__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyPassword__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyPort__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyUse__Q2_2nn2acFPC16netconf_profile_",
        "GetProxyUserName__Q2_2nn2acFPC16netconf_profile_",
        "GetRunningConfig__Q2_2nn2acFP16netconf_profile_",
        "GetSettingNic__Q2_2nn2acFPQ3_2nn2ac7NicType",
        "GetSsid__Q2_2nn2acFPC16netconf_profile_Pi",
        "GetStartupId__Q2_2nn2acFPQ3_2nn2ac11ConfigIdNum",
        "GetStatus__Q2_2nn2acFPQ3_2nn2ac6Status",
        "GetSubnetMask__Q2_2nn2acFPC16netconf_profile_",
        "GetTkipKey__Q2_2nn2acFPC16netconf_profile_Pi",
        "GetValidFlag__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumPb",
        "GetWep104KeyId__Q2_2nn2acFPC16netconf_profile_",
        "GetWep104Key__Q2_2nn2acFPC16netconf_profile_i",
        "GetWep40KeyId__Q2_2nn2acFPC16netconf_profile_",
        "GetWep40Key__Q2_2nn2acFPC16netconf_profile_i",
        "GetWifiConfigureMethod__Q2_2nn2acFPC16netconf_profile_",
        "Initialize__Q2_2nn2acFv",
        "IsAnyKeepingConnect__Q2_2nn2acFPb",
        "IsApplicationConnected__Q2_2nn2acFPb",
        "IsApplicationConnected__Q2_2nn2acFPbPQ3_2nn2ac6ApType",
        "IsApplicationConnected__Q2_2nn2acFPbPQ3_2nn2ac7NicType",
        "IsAutoConnectionFatallyFailed__Q2_2nn2acFQ2_2nn6Result",
        "IsConfigExisting__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumPb",
        "IsConnected__Q2_2nn2acFPbPQ3_2nn2ac6ApType",
        "IsConnected__Q2_2nn2acFPbPQ3_2nn2ac7NicType",
        "IsInfraConnectionEnabled__Q2_2nn2acFv",
        "IsKeepingConnect__Q2_2nn2acFPb",
        "IsReadyToConnect__Q2_2nn2acFPb",
        "IsSystemConnected__Q2_2nn2acFPbPQ3_2nn2ac6ApType",
        "IsSystemConnected__Q2_2nn2acFPbPQ3_2nn2ac7NicType",
        "ReadAossConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumP12aoss_config_",
        "ReadConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumP16netconf_profile_",
        "SetAddressObtainMode__Q2_2nn2acFP16netconf_profile_Ui",
        "SetAesKey__Q2_2nn2acFP16netconf_profile_PCUci",
        "SetAlternativeDnsServer__Q2_2nn2acFP16netconf_profile_PCc",
        "SetCompatId__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "SetDefaultGateway__Q2_2nn2acFP16netconf_profile_PCc",
        "SetDeviceIpAddress__Q2_2nn2acFP16netconf_profile_PCc",
        "SetEthernetAutoNegotiation__Q2_2nn2acFP16netconf_profile_Us",
        "SetEthernetCommunicationMethod__Q2_2nn2acFP16netconf_profile_Us",
        "SetEthernetSpeed__Q2_2nn2acFP16netconf_profile_Us",
        "SetMtu__Q2_2nn2acFP16netconf_profile_Ui",
        "SetNetworkInterface__Q2_2nn2acFP16netconf_profile_Us",
        "SetPrimaryDnsServer__Q2_2nn2acFP16netconf_profile_PCc",
        "SetPrivacyMode__Q2_2nn2acFP16netconf_profile_Us",
        "SetProxyAuthType__Q2_2nn2acFP16netconf_profile_Ui",
        "SetProxyHostName__Q2_2nn2acFP16netconf_profile_PCc",
        "SetProxyNoproxyHosts__Q2_2nn2acFP16netconf_profile_PCc",
        "SetProxyPassword__Q2_2nn2acFP16netconf_profile_PCc",
        "SetProxyPort__Q2_2nn2acFP16netconf_profile_Us",
        "SetProxyUse__Q2_2nn2acFP16netconf_profile_Us",
        "SetProxyUserName__Q2_2nn2acFP16netconf_profile_PCc",
        "SetSocketCloseSysShutdown__Q2_2nn2acFv",
        "SetSsid__Q2_2nn2acFP16netconf_profile_PCc",
        "SetStartupId__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "SetSubnetMask__Q2_2nn2acFP16netconf_profile_PCc",
        "SetTkipKey__Q2_2nn2acFP16netconf_profile_PCUci",
        "SetValidFlag__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumb",
        "SetWep104KeyId__Q2_2nn2acFP16netconf_profile_i",
        "SetWep104Key__Q2_2nn2acFP16netconf_profile_iPCc",
        "SetWep40KeyId__Q2_2nn2acFP16netconf_profile_i",
        "SetWep40Key__Q2_2nn2acFP16netconf_profile_iPCc",
        "SetWifiConfigureMethod__Q2_2nn2acFP16netconf_profile_Us",
        "StoreConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNum",
        "WriteAossConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumPC12aoss_config_",
        "WriteConfig__Q2_2nn2acFQ3_2nn2ac11ConfigIdNumPC16netconf_profile_"
    ],
    []
);
