//! netconf
//!
//! Network configuration and interface state.

use crate::ffi;

pub const PROFILE_COUNT: usize = 6;
pub const SSID_SIZE: usize = 32;
pub const DEFAULT_INTERFACE_MTU: u32 = 1400;
pub const MAX_MTU: u32 = 1500;
pub const MIN_MTU: u32 = 576;
pub const MAX_PROXY_HOST_LEN: usize = 128;
pub const MAX_NO_PROXY_HOSTS_LEN: usize = 128;
pub const MAX_PROXY_USERNAME_LEN: usize = 128;
pub const MAX_PROXY_PASSWORD_LEN: usize = 64;

/// NETCONF_ERROR
#[doc(alias = "NETCONF_ERROR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const OK: Self = Self(0);
    pub const NG: Self = Self(-1);
    pub const INVALID: Self = Self(-2);
    pub const UNKNOWN: Self = Self(-3);
    pub const RETRY: Self = Self(-4);
    pub const NEEDINIT: Self = Self(-5);
    pub const ADDR: Self = Self(-6);
    pub const NETMASK: Self = Self(-7);
    pub const NEXTHOP: Self = Self(-8);
    pub const IF_STATE: Self = Self(-9);
    pub const CONFIG_METHOD: Self = Self(-10);
    pub const SSID_LEN: Self = Self(-11);
    pub const KEYLEN: Self = Self(-12);
    pub const KEYID: Self = Self(-13);
    pub const PRIVACY_MODE: Self = Self(-14);
    pub const CONFIG_FILE: Self = Self(-15);
    pub const CONFIG_FORMAT_INVALID: Self = Self(-16);
    pub const MCP: Self = Self(-17);
    pub const NAMESERVER: Self = Self(-18);
    pub const IF_INDEX: Self = Self(-19);
    pub const SPEED: Self = Self(-20);
    pub const DUPLEX: Self = Self(-21);
    pub const NEGOTIATION: Self = Self(-22);
    pub const PROFILE_NUMBER: Self = Self(-23);
    pub const MTU: Self = Self(-24);
    pub const VALID_FLAG: Self = Self(-25);
    pub const PROXY_STATE: Self = Self(-26);
    pub const PROXY_HOST: Self = Self(-27);
    pub const PROXY_PORT: Self = Self(-28);
    pub const USBETH_NOT_FOUND: Self = Self(-29);
    pub const PROXY_AUTH_TYPE: Self = Self(-30);
    pub const SSID_STRING: Self = Self(-31);
    pub const PROXY_NO_HOSTS: Self = Self(-32);
    pub const PROXY_USERNAME: Self = Self(-33);
    pub const PROXY_PASSWORD: Self = Self(-34);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileId(pub i32);

impl ProfileId {
    pub const DEFAULT: Self = Self(0);
    pub const P0: Self = Self(0);
    pub const P1: Self = Self(1);
    pub const P2: Self = Self(2);
    pub const P3: Self = Self(3);
    pub const P4: Self = Self(4);
    pub const P5: Self = Self(5);
}

/// NetConfInterfaceType
///
/// Network interface.
#[doc(alias = "NetConfInterfaceType")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nic {
    Wifi = 0,
    Eth = 1,
}

/// NetConfAdminState
#[doc(alias = "NetConfAdminState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdminState(pub u16);

impl AdminState {
    pub const UNUSED: Self = Self(0);
    pub const UP: Self = Self(1);
    pub const DOWN: Self = Self(2);
}

/// NetConfLinkState
#[doc(alias = "NetConfLinkState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkState(pub u16);

impl LinkState {
    pub const UP: Self = Self(1);
    pub const DOWN: Self = Self(2);
    pub const NEGOTIATE: Self = Self(3);
}

/// NetConfOperState
#[doc(alias = "NetConfOperState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperState(pub u16);

impl OperState {
    pub const UP: Self = Self(1);
    pub const DOWN: Self = Self(2);
}

/// NetConfIPv4Mode
#[doc(alias = "NetConfIPv4Mode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddrMethod(pub u32);

impl AddrMethod {
    pub const DHCP: Self = Self(0);
    pub const STICKY: Self = Self(1);
    pub const MANUAL: Self = Self(2);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigMethod(pub u16);

impl ConfigMethod {
    pub const MANUAL: Self = Self(0);
    pub const WPS: Self = Self(1);
    pub const USBAP: Self = Self(2);
    pub const AOSS: Self = Self(3);
    pub const RAKURAKU: Self = Self(4);
}

/// NetConfWifiPrivacyMode
#[doc(alias = "NetConfWifiPrivacyMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrivacyMode(pub u16);

impl PrivacyMode {
    pub const NONE: Self = Self(0);
    pub const WEP40: Self = Self(1);
    pub const WEP104: Self = Self(2);
    pub const WPA2_PSK_TKIP: Self = Self(3);
    pub const WPA_PSK_TKIP: Self = Self(4);
    pub const WPA2_PSK_AES: Self = Self(5);
    pub const WPA_PSK_AES: Self = Self(6);
    pub const STATIC_AES: Self = Self(7);
    pub const WEP: Self = Self(8);
}

/// NetConfEthCfgSpeed
///
/// Link speed in Mbit/s.
#[doc(alias = "NetConfEthCfgSpeed")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthSpeed(pub u16);

impl EthSpeed {
    pub const M10: Self = Self(10);
    pub const M100: Self = Self(100);
}

/// NetConfEthCfgDuplex
#[doc(alias = "NetConfEthCfgDuplex")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthDuplex(pub u16);

impl EthDuplex {
    pub const HALF: Self = Self(1);
    pub const FULL: Self = Self(2);
}

/// NetConfEthCfgNegotiation
#[doc(alias = "NetConfEthCfgNegotiation")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthNegotiation(pub u16);

impl EthNegotiation {
    pub const MANUAL: Self = Self(1);
    pub const AUTO: Self = Self(2);
}

/// NetConfProxyStatus
#[doc(alias = "NetConfProxyStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProxyState(pub u16);

impl ProxyState {
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
}

/// NetConfProxyAuthType
#[doc(alias = "NetConfProxyAuthType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProxyAuthType(pub u32);

impl ProxyAuthType {
    pub const NONE: Self = Self(0);
    pub const BASIC: Self = Self(1);
    pub const DIGEST: Self = Self(2);
    pub const NTLM: Self = Self(8);
    pub const ANY: Self = Self(11);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileState(pub u32);

impl ProfileState {
    pub const DEFAULT: Self = Self(0);
    pub const CONFIGURED: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidFlag(pub u32);

impl ValidFlag {
    pub const INVALID: Self = Self(0);
    pub const VALID: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LastWifiLinkError(pub u32);

impl LastWifiLinkError {
    pub const OK: Self = Self(0);
    pub const ASSOC_FAILED: Self = Self(1);
    pub const REASSOC_DENIED: Self = Self(2);
    pub const KEY_INVALID: Self = Self(3);
    pub const SELF_DISCONNECT: Self = Self(4);
    pub const INVALID_WD_MODE: Self = Self(5);
    pub const LOST_AP_SIGNAL: Self = Self(6);
    pub const UNKNOWN: Self = Self(7);
}

/// NetConfMACAddr
#[doc(alias = "NetConfMACAddr")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddr(pub [u8; 6]);

/// NetConfIPv4Info
///
/// Addresses are in network byte order.
#[doc(alias = "NetConfIPv4Info")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Info {
    pub mode: AddrMethod,
    pub addr: u32,
    pub netmask: u32,
    /// Gateway.
    pub nexthop: u32,
    pub ns1: u32,
    pub ns2: u32,
}

/// NetConfInterface
#[doc(alias = "NetConfInterface")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interface {
    pub if_index: u16,
    pub if_state: u16,
    pub if_mtu: u32,
    pub ipv4_info: Ipv4Info,
}

/// WEP 40 bit keys.
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wep40Key {
    pub key_id: u16,
    pub key: [[u8; 5]; 4],
    _reserved: ffi::unknown<2>,
}

/// WEP 104 bit keys.
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wep104Key {
    pub key_id: u16,
    pub key: [[u8; 13]; 4],
    _reserved: ffi::unknown<2>,
}

/// TKIP or AES key.
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PskKey {
    pub key_len: u16,
    pub key: [u8; 64],
    _reserved: ffi::unknown<2>,
}

/// Key, depending on [PrivacyMode].
#[repr(C)]
#[derive(Clone, Copy)]
pub union WifiKey {
    pub wep40: Wep40Key,
    pub wep104: Wep104Key,
    pub psk: PskKey,
}

/// NetConfWifiConfigDataPrivacy
#[doc(alias = "NetConfWifiConfigDataPrivacy")]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct WifiPrivacy {
    pub mode: PrivacyMode,
    _reserved: ffi::unknown<2>,
    pub key: WifiKey,
}

/// NetConfWifiConfigData
#[doc(alias = "NetConfWifiConfigData")]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct WifiApConfig {
    pub ssid: [u8; SSID_SIZE],
    pub ssid_len: u16,
    _reserved: ffi::unknown<2>,
    pub privacy: WifiPrivacy,
}

/// NetConfWifiConfig
#[doc(alias = "NetConfWifiConfig")]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct WifiConfig {
    pub config_method: ConfigMethod,
    _reserved: ffi::unknown<2>,
    pub config: WifiApConfig,
}

/// NetConfEthCfg
#[doc(alias = "NetConfEthCfg")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthConfig {
    pub speed: EthSpeed,
    pub duplex: EthDuplex,
    pub negotiation: EthNegotiation,
    _reserved: ffi::unknown<2>,
}

/// NetConfProxyConfig
#[doc(alias = "NetConfProxyConfig")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProxyConfig {
    pub use_proxy: ProxyState,
    pub port: u16,
    pub auth_type: ProxyAuthType,
    pub host: [u8; MAX_PROXY_HOST_LEN],
    pub username: [u8; MAX_PROXY_USERNAME_LEN],
    pub password: [u8; MAX_PROXY_PASSWORD_LEN],
    pub noproxy_hosts: [u8; MAX_NO_PROXY_HOSTS_LEN],
}

/// NetConfCfg
#[doc(alias = "NetConfCfg")]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Profile {
    pub wl0: Interface,
    pub wifi: WifiConfig,
    pub eth0: Interface,
    pub eth_cfg: EthConfig,
    pub proxy: ProxyConfig,
}

/// NetConfAOSSConfig
#[doc(alias = "NetConfAOSSConfig")]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct AossConfig {
    pub config: [WifiApConfig; 4],
}

/// NetConfIfState
#[doc(alias = "NetConfIfState")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IfState {
    pub if_state: u16,
    pub link_state: LinkState,
    pub oper_state: OperState,
    _reserved: ffi::unknown<2>,
}

/// NetConfAllProfileState
#[doc(alias = "NetConfAllProfileState")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileStates {
    pub state: [ProfileState; PROFILE_COUNT],
}

/// NetConfValidFlags
#[doc(alias = "NetConfValidFlags")]
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidFlags {
    pub flag: [ValidFlag; PROFILE_COUNT],
}

/// NetConfOpt
#[doc(alias = "NetConfOpt")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Opt(pub ffi::unknown<0x2c1>);

const _: () = {
    assert!(size_of::<WifiPrivacy>() == 0x48);
    assert!(size_of::<WifiApConfig>() == 0x6C);
    assert!(size_of::<WifiConfig>() == 0x70);
    assert!(size_of::<ProxyConfig>() == 0x1C8);
    assert!(size_of::<Profile>() == 0x280);
    assert!(size_of::<AossConfig>() == 0x1B0);
};

unsafe extern "C" {
    /// netconf_init
    #[link_name = "netconf_init"]
    pub unsafe fn init() -> Error;

    /// netconf_close
    #[link_name = "netconf_close"]
    pub unsafe fn close() -> Error;

    /// netconf_get_running
    ///
    /// Configuration currently in use.
    #[link_name = "netconf_get_running"]
    pub unsafe fn get_running(config: *mut Profile) -> Error;

    /// netconf_set_running
    #[link_name = "netconf_set_running"]
    pub unsafe fn set_running(config: *mut Profile) -> Error;

    /// netconf_get_startup_profile_id
    ///
    /// Profile used on startup, or a negative [Error].
    #[link_name = "netconf_get_startup_profile_id"]
    pub unsafe fn get_startup_profile_id() -> ffi::c_int;

    /// netconf_set_startup_profile_id
    #[link_name = "netconf_set_startup_profile_id"]
    pub unsafe fn set_startup_profile_id(id: ProfileId) -> Error;

    /// netconf_delete_profile
    #[link_name = "netconf_delete_profile"]
    pub unsafe fn delete_profile(id: ProfileId) -> Error;

    /// netconf_get_all_profile_state
    #[link_name = "netconf_get_all_profile_state"]
    pub unsafe fn get_all_profile_state(states: *mut ProfileStates) -> Error;

    /// netconf_get_profile_state
    #[link_name = "netconf_get_profile_state"]
    pub unsafe fn get_profile_state(nic: Nic, state: *mut ProfileState) -> Error;

    /// netconf_get_valid_flags
    #[link_name = "netconf_get_valid_flags"]
    pub unsafe fn get_valid_flags(flags: *mut ValidFlags) -> Error;

    /// netconf_set_valid_flag
    #[link_name = "netconf_set_valid_flag"]
    pub unsafe fn set_valid_flag(unk1: u32, unk2: u32) -> Error;

    /// netconf_nv_read
    ///
    /// Reads a stored profile.
    #[link_name = "netconf_nv_read"]
    pub unsafe fn nv_read(id: ProfileId, config: *mut Profile) -> Error;

    /// netconf_nv_write
    ///
    /// Writes a stored profile.
    #[link_name = "netconf_nv_write"]
    pub unsafe fn nv_write(id: ProfileId, config: *mut Profile) -> Error;

    /// netconf_nv_load
    #[link_name = "netconf_nv_load"]
    pub unsafe fn nv_load(id: ProfileId) -> Error;

    /// netconf_nv_store
    #[link_name = "netconf_nv_store"]
    pub unsafe fn nv_store(id: ProfileId) -> Error;

    /// netconf_read_aoss_config
    #[link_name = "netconf_read_aoss_config"]
    pub unsafe fn read_aoss_config(id: ProfileId, config: *mut AossConfig) -> Error;

    /// netconf_write_aoss_config
    #[link_name = "netconf_write_aoss_config"]
    pub unsafe fn write_aoss_config(id: ProfileId, config: *mut AossConfig) -> Error;

    /// netconf_read_compat_profile_id
    #[link_name = "netconf_read_compat_profile_id"]
    pub unsafe fn read_compat_profile_id(id: *mut u32) -> Error;

    /// netconf_write_compat_profile_id
    #[link_name = "netconf_write_compat_profile_id"]
    pub unsafe fn write_compat_profile_id(id: u32) -> Error;

    /// netconf_get_ifstate
    #[link_name = "netconf_get_ifstate"]
    pub unsafe fn get_ifstate(nic: Nic, state: *mut IfState) -> Error;

    /// netconf_get_if_adminstate
    #[link_name = "netconf_get_if_adminstate"]
    pub unsafe fn get_if_adminstate(nic: Nic) -> AdminState;

    /// netconf_set_if_admin_state
    #[link_name = "netconf_set_if_admin_state"]
    pub unsafe fn set_if_admin_state(nic: Nic, state: AdminState) -> Error;

    /// netconf_get_if_linkstate
    #[link_name = "netconf_get_if_linkstate"]
    pub unsafe fn get_if_linkstate(nic: Nic) -> LinkState;

    /// netconf_get_if_operstate
    #[link_name = "netconf_get_if_operstate"]
    pub unsafe fn get_if_operstate(nic: Nic) -> OperState;

    /// netconf_get_if_macaddr
    #[link_name = "netconf_get_if_macaddr"]
    pub unsafe fn get_if_macaddr(nic: Nic, mac: *mut MacAddr) -> Error;

    /// netconf_get_if_ipv4_info
    #[link_name = "netconf_get_if_ipv4_info"]
    pub unsafe fn get_if_ipv4_info(info: *mut Ipv4Info) -> Error;

    /// netconf_set_if_ipv4_info
    #[link_name = "netconf_set_if_ipv4_info"]
    pub unsafe fn set_if_ipv4_info(info: *mut Ipv4Info) -> Error;

    /// netconf_get_if_ipv4_info_ex
    #[link_name = "netconf_get_if_ipv4_info_ex"]
    pub unsafe fn get_if_ipv4_info_ex(nic: Nic, info: *mut Ipv4Info) -> Error;

    /// netconf_set_if_ipv4_info_ex
    #[link_name = "netconf_set_if_ipv4_info_ex"]
    pub unsafe fn set_if_ipv4_info_ex(nic: Nic, info: *mut Ipv4Info) -> Error;

    /// netconf_get_interface_mtu
    #[link_name = "netconf_get_interface_mtu"]
    pub unsafe fn get_interface_mtu(nic: Nic, mtu: *mut u32) -> Error;

    /// netconf_set_interface_mtu
    #[link_name = "netconf_set_interface_mtu"]
    pub unsafe fn set_interface_mtu(nic: Nic, mtu: u32) -> Error;

    /// netconf_get_assigned_address
    #[link_name = "netconf_get_assigned_address"]
    pub unsafe fn get_assigned_address(nic: Nic, addr: *mut u32) -> Error;

    /// netconf_get_assigned_subnet
    #[link_name = "netconf_get_assigned_subnet"]
    pub unsafe fn get_assigned_subnet(nic: Nic, subnet: *mut u32) -> Error;

    /// netconf_get_assigned_gateway
    #[link_name = "netconf_get_assigned_gateway"]
    pub unsafe fn get_assigned_gateway(nic: Nic, gateway: *mut u32) -> Error;

    /// netconf_get_assigned_dns
    #[link_name = "netconf_get_assigned_dns"]
    pub unsafe fn get_assigned_dns(nic: Nic, dns1: *mut u32, dns2: *mut u32) -> Error;

    /// netconf_get_dns
    #[link_name = "netconf_get_dns"]
    pub unsafe fn get_dns(nic: Nic, dns1: *mut u32, dns2: *mut u32) -> Error;

    /// netconf_set_dns
    #[link_name = "netconf_set_dns"]
    pub unsafe fn set_dns(nic: Nic, dns1: *mut u32, dns2: *mut u32) -> Error;

    /// netconf_get_eth_cfg
    #[link_name = "netconf_get_eth_cfg"]
    pub unsafe fn get_eth_cfg(config: *mut EthConfig) -> Error;

    /// netconf_set_eth_cfg
    #[link_name = "netconf_set_eth_cfg"]
    pub unsafe fn set_eth_cfg(config: *mut EthConfig) -> Error;

    /// netconf_get_wifi_cfg
    #[link_name = "netconf_get_wifi_cfg"]
    pub unsafe fn get_wifi_cfg(config: *mut WifiConfig) -> Error;

    /// netconf_set_wifi_cfg
    #[link_name = "netconf_set_wifi_cfg"]
    pub unsafe fn set_wifi_cfg(config: *mut WifiConfig) -> Error;

    /// netconf_get_last_wifi_link_error
    #[link_name = "netconf_get_last_wifi_link_error"]
    pub unsafe fn get_last_wifi_link_error(error: *mut LastWifiLinkError) -> Error;

    /// netconf_get_proxy_config
    #[link_name = "netconf_get_proxy_config"]
    pub unsafe fn get_proxy_config(config: *mut ProxyConfig) -> Error;

    /// netconf_set_proxy_config
    #[link_name = "netconf_set_proxy_config"]
    pub unsafe fn set_proxy_config(config: *mut ProxyConfig) -> Error;

    /// SOGetProxyConfig
    #[link_name = "SOGetProxyConfig"]
    pub unsafe fn so_get_proxy_config(config: *mut ProxyConfig) -> Error;

    /// netconf_getopt
    #[link_name = "netconf_getopt"]
    pub unsafe fn getopt(unk: u16, opt: *mut Opt, size: *mut u32) -> Error;

    /// netconf_setopt
    #[link_name = "netconf_setopt"]
    pub unsafe fn setopt(unk: u16, opt: *mut Opt, size: *mut u32) -> Error;
}
