//! Local wireless 3DS network (UDS)
//!
//! Local wireless networks with Nintendo 3DS systems. Functions are in `nn::uds::Cafe`.

use super::Result;
use crate::ffi;

pub use super::cfg::ctr::UserName;

/// Size of the work memory for [init].
pub const WORK_MEMORY_SIZE: u32 = 0x14000;

/// UDS_MAX_NODES
#[doc(alias = "UDS_MAX_NODES")]
pub const MAX_NODES: usize = 16;

/// ScrambledLocalFriendCode
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScrambledLocalFriendCode {
    /// Already descrambled on the Wii U.
    pub local_friend_code: u64,
    pub network_node_id: u16,
    pub xor_key: u16,
}

/// NodeInformation
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeInformation {
    pub scrambled_local_friend_code: ScrambledLocalFriendCode,
    pub user_name: UserName,
    pub network_node_id: u16,
    _unk: ffi::unknown<0x2>,
}

/// ConnectionStatus
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionStatus {
    pub status: u32,
    /// Why the status changed.
    pub reason: u32,
    pub network_node_id: u16,
    /// Bitmask of changed nodes.
    pub changed_nodes: u16,
    pub nodes: [u16; MAX_NODES],
    pub total_nodes: u8,
    /// Including the host.
    pub max_nodes: u8,
    pub node_bitmask: u16,
}

/// EndpointDescriptor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointDescriptor(pub u32);

/// PollStateChangeFlags
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PollStateChangeFlags(pub u8);

impl PollStateChangeFlags {
    pub const NONE: Self = Self(0);
    /// Don't block if there is no state change.
    #[doc(alias = "UDS_POLL_NONBLOCK")]
    pub const NONBLOCK: Self = Self(1 << 0);
}

/// ReceiveFlags
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReceiveFlags(pub u8);

impl ReceiveFlags {
    pub const NONE: Self = Self(0);
    /// Don't block if no data is available.
    #[doc(alias = "UDS_RECEIVE_NONBLOCK")]
    pub const NONBLOCK: Self = Self(1 << 0);
}

unsafe extern "C" {
    /// Initialize
    ///
    /// `work_memory` must be [WORK_MEMORY_SIZE] bytes.
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q3_2nn3uds4CafeFPvUi"]
    pub unsafe fn init(work_memory: *mut ffi::c_void, work_memory_size: u32) -> Result;

    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q3_2nn3uds4CafeFPvUiPQ4_2nn3cfg3CTR8UserName"]
    pub unsafe fn init_with_user_name(
        work_memory: *mut ffi::c_void,
        work_memory_size: u32,
        user_name: *mut UserName,
    ) -> Result;

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q3_2nn3uds4CafeFv"]
    pub unsafe fn deinit();

    /// CreateLocalCommunicationId
    #[doc(alias = "CreateLocalCommunicationId")]
    #[link_name = "CreateLocalCommunicationId__Q3_2nn3uds4CafeFUib"]
    pub unsafe fn create_local_communication_id(unique_id: u32, unk: bool) -> u32;

    /// CreateNetwork
    #[doc(alias = "CreateNetwork")]
    #[link_name = "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3T1"]
    pub unsafe fn create_network(
        id8: u8,
        max_nodes: u8,
        local_communication_id: u32,
        passphrase: *const ffi::c_char,
        passphrase_size: u32,
        channel: u8,
    ) -> Result;

    /// CreateNetwork
    #[doc(alias = "CreateNetwork")]
    #[link_name = "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3T1PCvT3"]
    pub unsafe fn create_network_with_app_data(
        id8: u8,
        max_nodes: u8,
        local_communication_id: u32,
        passphrase: *const ffi::c_char,
        passphrase_size: u32,
        channel: u8,
        app_data: *const ffi::c_void,
        app_data_size: u32,
    ) -> Result;

    /// CreateNetwork
    #[doc(alias = "CreateNetwork")]
    #[link_name = "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3bT1PCvT3"]
    pub unsafe fn create_network_with_app_data_unk(
        id8: u8,
        max_nodes: u8,
        local_communication_id: u32,
        passphrase: *const ffi::c_char,
        passphrase_size: u32,
        unused: bool,
        channel: u8,
        app_data: *const ffi::c_void,
        app_data_size: u32,
    ) -> Result;

    /// DestroyNetwork
    #[doc(alias = "DestroyNetwork")]
    #[link_name = "DestroyNetwork__Q3_2nn3uds4CafeFv"]
    pub unsafe fn destroy_network() -> Result;

    /// AllowToConnect
    #[doc(alias = "AllowToConnect")]
    #[link_name = "AllowToConnect__Q3_2nn3uds4CafeFv"]
    pub unsafe fn allow_to_connect() -> Result;

    /// DisallowToConnect
    #[doc(alias = "DisallowToConnect")]
    #[link_name = "DisallowToConnect__Q3_2nn3uds4CafeFb"]
    pub unsafe fn disallow_to_connect(unk: bool) -> Result;

    /// AllowToSpectate
    #[doc(alias = "AllowToSpectate")]
    #[link_name = "AllowToSpectate__Q3_2nn3uds4CafeFv"]
    pub unsafe fn allow_to_spectate() -> Result;

    /// EjectClient
    #[doc(alias = "EjectClient")]
    #[link_name = "EjectClient__Q3_2nn3uds4CafeFUs"]
    pub unsafe fn eject_client(node_id: u16) -> Result;

    /// EjectSpectator
    #[doc(alias = "EjectSpectator")]
    #[link_name = "EjectSpectator__Q3_2nn3uds4CafeFv"]
    pub unsafe fn eject_spectator() -> Result;

    /// GetChannel
    #[doc(alias = "GetChannel")]
    #[link_name = "GetChannel__Q3_2nn3uds4CafeFPUc"]
    pub unsafe fn get_channel(channel: *mut u8) -> Result;

    /// GetConnectionStatus
    #[doc(alias = "GetConnectionStatus")]
    #[link_name = "GetConnectionStatus__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe16ConnectionStatus"]
    pub unsafe fn get_connection_status(status: *mut ConnectionStatus) -> Result;

    /// GetNodeInformation
    #[doc(alias = "GetNodeInformation")]
    #[link_name = "GetNodeInformation__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe15NodeInformationUs"]
    pub unsafe fn get_node_information(info: *mut NodeInformation, node_id: u16) -> Result;

    /// PollStateChange
    #[doc(alias = "PollStateChange")]
    #[link_name = "PollStateChange__Q3_2nn3uds4CafeFUc"]
    pub unsafe fn poll_state_change(flags: PollStateChangeFlags) -> Result;

    /// SetApplicationDataToBeacon
    #[doc(alias = "SetApplicationDataToBeacon")]
    #[link_name = "SetApplicationDataToBeacon__Q3_2nn3uds4CafeFPCvUi"]
    pub unsafe fn set_application_data_to_beacon(app_data: *const ffi::c_void, size: u32)
    -> Result;

    /// GetApplicationDataFromBeacon
    #[doc(alias = "GetApplicationDataFromBeacon")]
    #[link_name = "GetApplicationDataFromBeacon__Q3_2nn3uds4CafeFPvPUiUi"]
    pub unsafe fn get_application_data_from_beacon(
        app_data: *mut ffi::c_void,
        read_size: *mut u32,
        size: u32,
    ) -> Result;

    /// CreateEndpoint
    #[doc(alias = "CreateEndpoint")]
    #[link_name = "CreateEndpoint__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptor"]
    pub unsafe fn create_endpoint(endpoint: *mut EndpointDescriptor) -> Result;

    /// DestroyEndpoint
    #[doc(alias = "DestroyEndpoint")]
    #[link_name = "DestroyEndpoint__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptor"]
    pub unsafe fn destroy_endpoint(endpoint: *mut EndpointDescriptor) -> Result;

    /// Attach
    #[doc(alias = "Attach")]
    #[link_name = "Attach__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptorUsUcUi"]
    pub unsafe fn attach(
        endpoint: *mut EndpointDescriptor,
        node_id: u16,
        data_channel: u8,
        recv_buffer_size: u32,
    ) -> Result;

    /// SendTo
    #[doc(alias = "SendTo")]
    #[link_name = "SendTo__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPCvUiUsUcT5"]
    pub unsafe fn send_to(
        endpoint: *const EndpointDescriptor,
        buf: *const ffi::c_void,
        size: u32,
        dest_node_id: u16,
        data_channel: u8,
        flags: u8,
    ) -> Result;

    /// Receive
    #[doc(alias = "Receive")]
    #[link_name = "Receive__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvPUiUiUc"]
    pub unsafe fn receive(
        endpoint: *const EndpointDescriptor,
        buf: *mut ffi::c_void,
        received_size: *mut u32,
        size: u32,
        flags: ReceiveFlags,
    ) -> Result;

    /// ReceiveFrom
    #[doc(alias = "ReceiveFrom")]
    #[link_name = "ReceiveFrom__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvPUiPUsUiUc"]
    pub unsafe fn receive_from(
        endpoint: *const EndpointDescriptor,
        buf: *mut ffi::c_void,
        received_size: *mut u32,
        node_id: *mut u16,
        size: u32,
        flags: ReceiveFlags,
    ) -> Result;
}

imports_section!(
    "nn_uds",
    [
        "AddOutsideClient__Q3_2nn3uds4CafeFRQ4_2nn3uds4Cafe24NodeInformationForBridge",
        "AddOutsideClient__Q5_2nn3uds4Cafe6detail3UdsFRQ4_2nn3uds4Cafe24NodeInformationForBridge",
        "AllowToConnect__Q3_2nn3uds4CafeFv",
        "AllowToSpectate__Q3_2nn3uds4CafeFv",
        "AttachForBridge__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptor",
        "Attach__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptorUsUcUi",
        "BindForBridge__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorPvUi",
        "Bind__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorPviUcUs",
        "Close__Q5_2nn3uds4Cafe6detail3UdsFv",
        "CompareWith__Q4_2nn3uds4Cafe18NetworkDescriptionCFPUcPQ5_2nn3uds4Cafe6detail25NetworkDescriptionElement",
        "CreateBridgeNetwork__Q3_2nn3uds4CafeFPCQ4_2nn3uds4Cafe24BridgeNetworkInformation",
        "CreateBridgeNetwork__Q3_2nn3uds4CafeFPCQ4_2nn3uds4Cafe24BridgeNetworkInformationUc",
        "CreateBridgeNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3UsPCvT3PCQ4_2nn3uds4Cafe24NodeInformationForBridgeT1PCUsN31",
        "CreateBridgeNetwork__Q5_2nn3uds4Cafe6detail3UdsFRCQ4_2nn3uds4Cafe18NetworkDescriptionPCciPCQ4_2nn3uds4Cafe24NodeInformationForBridgeUcPCUsN25",
        "CreateEndpoint__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptor",
        "CreateLocalCommunicationId__Q3_2nn3uds4CafeFUib",
        "CreateNetworkBasicCheck__Q3_2nn3uds4CafeFUcT1UiPCcT3T1",
        "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3",
        "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3PCvT3",
        "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3T1",
        "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3T1PCvT3",
        "CreateNetwork__Q3_2nn3uds4CafeFUcT1UiPCcT3bT1PCvT3",
        "CreateNetwork__Q5_2nn3uds4Cafe6detail3UdsFRCQ4_2nn3uds4Cafe18NetworkDescriptionPCci",
        "DeleteOutsideClient__Q3_2nn3uds4CafeFUs",
        "DeleteOutsideClient__Q5_2nn3uds4Cafe6detail3UdsFUs",
        "DestroyBridgeNetwork__Q3_2nn3uds4CafeFv",
        "DestroyEndpoint__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptor",
        "DestroyEndpoint__Q4_2nn3uds4Cafe6detailFPQ4_2nn3uds4Cafe18EndpointDescriptorPQ5_2nn3uds4Cafe6detail13ReceiveReport",
        "DestroyNetwork__Q3_2nn3uds4CafeFv",
        "DestroyNetwork__Q5_2nn3uds4Cafe6detail3UdsFv",
        "DisallowToConnect__Q3_2nn3uds4CafeFb",
        "DumpNetworkDescriptionElement__Q4_2nn3uds4Cafe6detailFRCQ5_2nn3uds4Cafe6detail25NetworkDescriptionElement",
        "EjectClient__Q3_2nn3uds4CafeFUs",
        "EjectClient__Q5_2nn3uds4Cafe6detail3UdsFUs",
        "EjectSpectator__Q3_2nn3uds4CafeFv",
        "EjectSpectator__Q5_2nn3uds4Cafe6detail3UdsFv",
        "FinalizeCore__Q4_2nn3uds4Cafe6detailFv",
        "Finalize__Q3_2nn3uds4CafeFv",
        "Finalize__Q5_2nn3uds4Cafe6detail3UdsFv",
        "Flush__Q4_2nn3uds4Cafe6detailFQ4_2nn3uds4Cafe14DataFrameIndex",
        "Flush__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe14DataFrameIndex",
        "GetApplicationDataFromBeacon__Q3_2nn3uds4CafeFPvPUiUi",
        "GetApplicationData__Q4_2nn3uds4Cafe18NetworkDescriptionCFPUcUi",
        "GetApplicationData__Q5_2nn3uds4Cafe6detail3UdsFPUcPUiUi",
        "GetChannel__Q3_2nn3uds4CafeFPUc",
        "GetChannel__Q5_2nn3uds4Cafe6detail3UdsFPUc",
        "GetConnectionStatusForBridge__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe25ConnectionStatusForBridge",
        "GetConnectionStatusForBridge__Q5_2nn3uds4Cafe6detail3UdsFPQ4_2nn3uds4Cafe25ConnectionStatusForBridge",
        "GetConnectionStatus__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe16ConnectionStatus",
        "GetConnectionStatus__Q5_2nn3uds4Cafe6detail3UdsFPQ4_2nn3uds4Cafe16ConnectionStatus",
        "GetIobPoolsUtilization__Q4_2nn3uds4Cafe6detailFPiPQ5_2nn3uds4Cafe6detail21AllIobPoolUtilization",
        "GetIobPoolsUtilization__Q5_2nn3uds4Cafe6detail3UdsFPiPQ5_2nn3uds4Cafe6detail21AllIobPoolUtilization",
        "GetIopUtilizationRate__Q4_2nn3uds4Cafe6detailFPi",
        "GetIopUtilizationRate__Q5_2nn3uds4Cafe6detail3UdsFPi",
        "GetMacAddress__Q4_2nn3uds4Cafe6detailFPUc",
        "GetNodeInformationForBridge__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe24NodeInformationForBridgeUs",
        "GetNodeInformationForBridge__Q5_2nn3uds4Cafe6detail3UdsFPQ4_2nn3uds4Cafe24NodeInformationForBridgeUs",
        "GetNodeInformationRaw__Q4_2nn3uds4Cafe6detailFPQ5_2nn3uds4Cafe6detail18NodeInformationRawUs",
        "GetNodeInformation__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe15NodeInformationUs",
        "GetNodeInformation__Q5_2nn3uds4Cafe6detail3UdsFPQ5_2nn3uds4Cafe6detail18NodeInformationRawUs",
        "InitializeCore__Q4_2nn3uds4Cafe6detailFPvUiPQ4_2nn3cfg3CTR8UserName",
        "InitializeWithVersion__Q5_2nn3uds4Cafe6detail3UdsFiRCQ5_2nn3uds4Cafe6detail18NodeInformationRawUs",
        "Initialize__Q3_2nn3uds4CafeFPvUi",
        "Initialize__Q3_2nn3uds4CafeFPvUiPQ4_2nn3cfg3CTR8UserName",
        "Initialize__Q4_2nn3uds4Cafe18NetworkDescriptionFPCQ5_2nn3uds4Cafe6detail25NetworkDescriptionElementUcPCUc",
        "Initialize__Q4_2nn3uds4Cafe18NetworkDescriptionFUiUcN22",
        "Initialize__Q4_2nn3uds4Cafe18NetworkDescriptionFUiUcN22PCvT1T2Us",
        "Ioctlv__Q5_2nn3uds4Cafe6detail3UdsFQ6_2nn3uds4Cafe6detail3Uds3TagUiT2Pv",
        "Open__Q5_2nn3uds4Cafe6detail3UdsFv",
        "PollStateChange__Q3_2nn3uds4CafeFUc",
        "PollStateChange__Q5_2nn3uds4Cafe6detail3UdsFUc",
        "PullBridgedPacket__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorPUiUiT2PUsT5PUcUc",
        "PullPacket__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorPUiUiT2PUsUc",
        "ReceiveBasicCheck__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvUib",
        "ReceiveBridgedPacket__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvPUiPUsT4PUcUiUc",
        "ReceiveFrom__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvPUiPUsUiUc",
        "Receive__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPvPUiUiUc",
        "ResetForcedNetworkChannel__Q4_2nn3uds4Cafe6detailFv",
        "ScrambleLocalFriendCode__Q4_2nn3uds4Cafe6detailFPQ4_2nn3uds4Cafe24ScrambledLocalFriendCodeULUs",
        "Scrap__Q5_2nn3uds4Cafe6detail3UdsFv",
        "SendBridgedPacket__Q3_2nn3uds4CafeFQ4_2nn3uds4Cafe18EndpointDescriptorPCvUiUsT4UcT6",
        "SendBridgedPacket__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorUsT2UcPCUiUiT6T4",
        "SendPacketBasicCheck__Q3_2nn3uds4CafeFQ4_2nn3uds4Cafe18EndpointDescriptorPCvUiUsUc",
        "SendTo__Q3_2nn3uds4CafeFRCQ4_2nn3uds4Cafe18EndpointDescriptorPCvUiUsUcT5",
        "SendTo__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorUsUcPCUiUiT5T3",
        "SetApplicationDataToBeacon__Q3_2nn3uds4CafeFPCvUi",
        "SetApplicationData__Q5_2nn3uds4Cafe6detail3UdsFPCUcUi",
        "SetForcedNetworkChannel__Q4_2nn3uds4Cafe6detailFUc",
        "SetMaxSendDelay__Q3_2nn3uds4CafeFQ3_2nn3fnd8TimeSpan",
        "SetMaxSendDelay__Q5_2nn3uds4Cafe6detail3UdsFL",
        "Unbind__Q3_2nn3uds4CafeFPQ4_2nn3uds4Cafe18EndpointDescriptorPQ5_2nn3uds4Cafe6detail13ReceiveReportPQ4_2nn3uds4Cafe8EndPoint",
        "Unbind__Q5_2nn3uds4Cafe6detail3UdsFQ4_2nn3uds4Cafe18EndpointDescriptorPQ5_2nn3uds4Cafe6detail13ReceiveReport",
        "UnscrambleLocalFriendCode__Q4_2nn3uds4Cafe6detailFRCQ4_2nn3uds4Cafe24ScrambledLocalFriendCode",
        "UpdateNetworkAttribute__Q5_2nn3uds4Cafe6detail3UdsFUsb",
        "__CPR111__Ioctlv__Q5_2nn3uds4Cafe6detail3UdsFQ6_2nn3udsJ18JJ23J3Uds3TagRQJ36JdsJ18JJ23J3Uds17IpcBufferOnStack0",
        "__CPR78____eq__Q4_2nn3uds4Cafe18NetworkDescriptionFRQJ7J3udsJ16JJ21J",
        "__CPR78____ne__Q4_2nn3uds4Cafe18NetworkDescriptionFRQJ7J3udsJ16JJ21J",
        "__CPR87__CompareWith__Q4_2nn3uds4Cafe18NetworkDescriptionCFRCQJ14J3udsJ23JJ28J",
        "__sti___11_uds_Api_cpp_f5d9abb2"
    ],
    [
        "__TID_Q3_2nn3fnd10IAllocator",
        "__TID_Q3_2nn3fnd77ExpHeapTemplate__tm__54_Q4_2nn2os10LockPolicy30Object__tm__16_Q3_2nn2os5Mutex",
        "__TID_Q3_2nn3fnd8HeapBase",
        "__TID_Q4_2nn3fnd48IntrusiveLinkedList__tm__21_Q3_2nn3fnd8HeapBasev4Item",
        "__TID_Q4_2nn3fnd77ExpHeapTemplate__tm__54_Q4_2nn2os10LockPolicy30Object__tm__16_Q3_2nn2os5Mutex9Allocator",
        "__TID_Q4_2nn4util11ADLFireWall85NonCopyable__tm__66_Q4_2nn3fnd48IntrusiveLinkedList__tm__21_Q3_2nn3fnd8HeapBasev4Item",
        "__TID_Q5_2nn2os10LockPolicy30Object__tm__16_Q3_2nn2os5Mutex10LockObject",
        "s_EndPointCounter__Q3_2nn3uds4Cafe",
        "s_EndPoint__Q3_2nn3uds4Cafe",
        "s_ForcedNetworkChannel__Q3_2nn3uds4Cafe",
        "s_IsInitialized__Q3_2nn3uds4Cafe",
        "s_Lock__Q3_2nn3uds4Cafe",
        "s_NetDesc__Q3_2nn3uds4Cafe",
        "s_UdsIpc__Q3_2nn3uds4Cafe"
    ]
);
