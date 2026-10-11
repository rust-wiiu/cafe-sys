//! Download Play (DLP)
//!
//! Distributes a Download Play child to Nintendo 3DS consoles. Functions are static methods of `nn::dlp::Cafe::Server`, see [private] for `ServerPrivate`.

use super::{
    Result,
    uds::{NodeInformation, UserName},
};
use crate::ffi;

/// PollStateChangeFlags
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PollStateChangeFlags(pub u8);

impl PollStateChangeFlags {
    pub const NONE: Self = Self(0);
    /// Don't block if there is no state change.
    #[doc(alias = "DLP_POLL_NONBLOCK")]
    pub const NONBLOCK: Self = Self(1 << 0);
}

/// ClientState
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientState(pub u32);

impl ClientState {
    pub const ACCEPTED: Self = Self(5);
    pub const RECEIVING: Self = Self(6);
    /// Received the distribution and ready to boot.
    pub const READY: Self = Self(7);
    /// Connected and waiting to be accepted.
    pub const CONNECTING: Self = Self(64);
}

/// ServerState
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerState(pub u32);

impl ServerState {
    pub const UNINITIALIZED: Self = Self(0);
    /// Initialized, but no active network.
    pub const IDLE: Self = Self(1);
    pub const OPEN: Self = Self(2);
    pub const DISTRIBUTING_INTERNAL: Self = Self(7);
    pub const DISTRIBUTED: Self = Self(8);
    /// Clients are booted into the child.
    pub const FINISHED: Self = Self(9);
    pub const FAILED: Self = Self(10);
    /// Public value for the internal distributing states.
    pub const DISTRIBUTING: Self = Self(11);
}

unsafe extern "C" {
    /// AcceptClient
    #[doc(alias = "AcceptClient")]
    #[link_name = "AcceptClient__Q4_2nn3dlp4Cafe6ServerSFUs"]
    pub unsafe fn accept_client(node_id: u16) -> Result;

    /// CloseSessions
    #[doc(alias = "CloseSessions")]
    #[link_name = "CloseSessions__Q4_2nn3dlp4Cafe6ServerSFv"]
    pub unsafe fn close_sessions() -> Result;

    /// DisconnectClient
    #[doc(alias = "DisconnectClient")]
    #[link_name = "DisconnectClient__Q4_2nn3dlp4Cafe6ServerSFUs"]
    pub unsafe fn disconnect_client(node_id: u16) -> Result;

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q4_2nn3dlp4Cafe6ServerSFv"]
    pub unsafe fn deinit() -> Result;

    /// GetClientInfo
    #[doc(alias = "GetClientInfo")]
    #[link_name = "GetClientInfo__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3uds4Cafe15NodeInformationUs"]
    pub unsafe fn get_client_info(info: *mut NodeInformation, node_id: u16) -> Result;

    /// GetClientState
    #[doc(alias = "GetClientState")]
    #[link_name = "GetClientState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ClientStateUs"]
    pub unsafe fn get_client_state(state: *mut ClientState, node_id: u16) -> Result;

    /// GetClientState
    #[doc(alias = "GetClientState")]
    #[link_name = "GetClientState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ClientStatePUiT2Us"]
    pub unsafe fn get_client_state_with_progress(
        state: *mut ClientState,
        units_total: *mut u32,
        units_received: *mut u32,
        node_id: u16,
    ) -> Result;

    /// GetConnectingClients
    #[doc(alias = "GetConnectingClients")]
    #[link_name = "GetConnectingClients__Q4_2nn3dlp4Cafe6ServerSFPUsT1Us"]
    pub unsafe fn get_connecting_clients(
        received_len: *mut u16,
        clients: *mut u16,
        len: u16,
    ) -> Result;

    /// GetState
    #[doc(alias = "GetState")]
    #[link_name = "GetState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ServerState"]
    pub unsafe fn get_state(state: *mut ServerState) -> Result;

    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q4_2nn3dlp4Cafe6ServerSFUcUiT1PQ4_2nn3cfg3CTR8UserName"]
    pub unsafe fn init(
        max_client_num: u8,
        unique_id: u32,
        child_index: u8,
        user_name: *mut UserName,
    ) -> Result;

    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q4_2nn3dlp4Cafe6ServerSFPbUcUiT2PQ4_2nn3cfg3CTR8UserName"]
    pub unsafe fn init_with_dup_notice(
        dup_notice_need: *mut bool,
        max_client_num: u8,
        unique_id: u32,
        child_index: u8,
        user_name: *mut UserName,
    ) -> Result;

    /// OpenSessions
    #[doc(alias = "OpenSessions")]
    #[link_name = "OpenSessions__Q4_2nn3dlp4Cafe6ServerSFbUc"]
    pub unsafe fn open_sessions(manual_accept: bool, channel: u8) -> Result;

    /// PollStateChange
    #[doc(alias = "PollStateChange")]
    #[link_name = "PollStateChange__Q4_2nn3dlp4Cafe6ServerSFUc"]
    pub unsafe fn poll_state_change(flags: PollStateChangeFlags) -> Result;

    /// RebootAllClients
    ///
    /// Boots the clients into the distributed child.
    #[doc(alias = "RebootAllClients")]
    #[link_name = "RebootAllClients__Q4_2nn3dlp4Cafe6ServerSFPCc"]
    pub unsafe fn reboot_all_clients(pass_phrase: *const ffi::c_char) -> Result;

    /// StartDistribute
    #[doc(alias = "StartDistribute")]
    #[link_name = "StartDistribute__Q4_2nn3dlp4Cafe6ServerSFv"]
    pub unsafe fn start_distribute() -> Result;

}

/// Static methods of `nn::dlp::Cafe::ServerPrivate`.
pub mod private {
    use super::{Result, ServerState, UserName};

    unsafe extern "C" {
        /// GetInternalState
        #[doc(alias = "GetInternalState")]
        #[link_name = "GetInternalState__Q4_2nn3dlp4Cafe13ServerPrivateSFPQ4_2nn3dlp4Cafe11ServerState"]
        pub unsafe fn get_internal_state(state: *mut ServerState) -> Result;

        /// Initialize
        #[doc(alias = "Initialize")]
        #[link_name = "Initialize__Q4_2nn3dlp4Cafe13ServerPrivateSFUcUiT1N22PQ4_2nn3cfg3CTR8UserName"]
        pub unsafe fn init(
            max_client_num: u8,
            unique_id: u32,
            child_index: u8,
            block_buffer_size: u32,
            block_buffer_num: u32,
            user_name: *mut UserName,
        ) -> Result;

        /// Initialize
        #[doc(alias = "Initialize")]
        #[link_name = "Initialize__Q4_2nn3dlp4Cafe13ServerPrivateSFPbUcUiT2N23PQ4_2nn3cfg3CTR8UserName"]
        pub unsafe fn init_with_dup_notice(
            dup_notice_need: *mut bool,
            max_client_num: u8,
            unique_id: u32,
            child_index: u8,
            block_buffer_size: u32,
            block_buffer_num: u32,
            user_name: *mut UserName,
        ) -> Result;
    }
}

imports_section!(
    "nn_dlp",
    [
        "AcceptClient__Q4_2nn3dlp4Cafe6ServerSFUs",
        "AcceptClient__Q5_2nn3dlp4Cafe6detail9ServerIpcSFUs",
        "CloseSessions__Q4_2nn3dlp4Cafe6ServerSFv",
        "CloseSessions__Q5_2nn3dlp4Cafe6detail9ServerIpcSFv",
        "DisconnectClient__Q4_2nn3dlp4Cafe6ServerSFUs",
        "DisconnectClient__Q5_2nn3dlp4Cafe6detail9ServerIpcSFUs",
        "FinalizeServerIpc__Q4_2nn3dlp4Cafe6detailFv",
        "Finalize__Q4_2nn3dlp4Cafe6ServerSFv",
        "Finalize__Q5_2nn3dlp4Cafe6detail9ServerIpcSFv",
        "GetClientInfo__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3uds4Cafe15NodeInformationUs",
        "GetClientInfo__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPQ4_2nn3uds4Cafe15NodeInformationUs",
        "GetClientState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ClientStatePUiT2Us",
        "GetClientState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ClientStateUs",
        "GetClientStatus__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPQ4_2nn3dlp4Cafe12ClientStatusUs",
        "GetConnectingClients__Q4_2nn3dlp4Cafe6ServerSFPUsT1Us",
        "GetConnectingClients__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPUsT1Us",
        "GetDupNoticeNeed__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPb",
        "GetInternalState__Q4_2nn3dlp4Cafe13ServerPrivateSFPQ4_2nn3dlp4Cafe11ServerState",
        "GetState__Q4_2nn3dlp4Cafe6ServerSFPQ4_2nn3dlp4Cafe11ServerState",
        "GetState__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPQ4_2nn3dlp4Cafe11ServerState",
        "InitializePrivate__Q5_2nn3dlp4Cafe6detail9ServerIpcSFUcUiT1N22RCQ4_2nn3cfg3CTR8UserNameULb",
        "InitializeServerIpc__Q4_2nn3dlp4Cafe6detailFv",
        "Initialize__Q4_2nn3dlp4Cafe13ServerPrivateSFPbUcUiT2N23PQ4_2nn3cfg3CTR8UserName",
        "Initialize__Q4_2nn3dlp4Cafe13ServerPrivateSFUcUiT1N22PQ4_2nn3cfg3CTR8UserName",
        "Initialize__Q4_2nn3dlp4Cafe6ServerSFPbUcUiT2PQ4_2nn3cfg3CTR8UserName",
        "Initialize__Q4_2nn3dlp4Cafe6ServerSFUcUiT1PQ4_2nn3cfg3CTR8UserName",
        "Initialize__Q5_2nn3dlp4Cafe6detail9ServerIpcSFUcUiT1RCQ4_2nn3cfg3CTR8UserNameULb",
        "OpenSessions__Q4_2nn3dlp4Cafe6ServerSFbUc",
        "OpenSessions__Q5_2nn3dlp4Cafe6detail9ServerIpcSFbUc",
        "PollStateChange__Q4_2nn3dlp4Cafe6ServerSFUc",
        "PollStateChange__Q5_2nn3dlp4Cafe6detail9ServerIpcSFv",
        "RebootAllClients__Q4_2nn3dlp4Cafe6ServerSFPCc",
        "RebootAllClients__Q5_2nn3dlp4Cafe6detail9ServerIpcSFPCc",
        "StartDistribute__Q4_2nn3dlp4Cafe6ServerSFv",
        "StartDistribute__Q5_2nn3dlp4Cafe6detail9ServerIpcSFv"
    ],
    []
);
