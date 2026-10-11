//! USB host stack (UHS)
//!
//! Class drivers, interfaces and transfers.

use crate::ffi;

pub const MAX_ENDPOINT_PAIRS: usize = 16;
pub const MAX_HUB_HIERARCHY: usize = 8;
pub const MAX_ISOC_FRAMES: usize = 32;
pub const CLIENT_MIN_ASYNC_TXN: u32 = 3;

/// UHSStatus
#[doc(alias("UHSStatus", "UhsStatus"))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const OK: Self = Self(0);
    pub const HANDLE_INVALID_ARGS: Self = Self(0xFFDEFFFDu32 as i32);
    pub const HANDLE_INVALID_STATE: Self = Self(0xFFDEFFFCu32 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DevRegHandle(pub i32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceHandle(pub i32);

/// Timeout, -1 for none.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time(pub i32);

impl Time {
    pub const FOREVER: Self = Self(-1);
}

/// Bit `n` is OUT endpoint `n`, bit `n + 16` is IN endpoint `n`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointMask(pub u32);

impl core::ops::BitOr for EndpointMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointOptions(pub u32);

impl EndpointOptions {
    pub const NONE: Self = Self(0);
    pub const PRESERVE_MAX_LIMITS: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceOptions(pub u32);

impl DeviceOptions {
    pub const NONE: Self = Self(0);
    pub const UNCONDITIONAL: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceSpeed(pub u32);

impl DeviceSpeed {
    pub const INVALID: Self = Self(0);
    pub const LOW: Self = Self(1);
    pub const FULL: Self = Self(2);
    pub const HIGH: Self = Self(3);
}

/// UHSAdminEpType
#[doc(alias("UHSAdminEpType", "UhsEpCommandType"))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointCommand {
    Enable = 1,
    Disable = 2,
    Cancel = 3,
    CancelAndReset = 4,
}

/// UHSAdminDevType
#[doc(alias("UHSAdminDevType", "UhsDeviceCommandType"))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceCommand {
    Reset = 1,
    Disable = 2,
    Suspend = 3,
    Resume = 4,
    Destroy = 5,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direction(pub u32);

impl Direction {
    pub const INVALID: Self = Self(0);
    pub const TO_DEVICE: Self = Self(1);
    pub const TO_HOST: Self = Self(2);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceIndication(pub u32);

impl InterfaceIndication {
    pub const INVALID: Self = Self(0);
    pub const SUSPEND: Self = Self(1);
    pub const RESUME: Self = Self(2);
    pub const PROBE: Self = Self(3);
    pub const DISCONNECT: Self = Self(4);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsocStartType {
    Absolute = 0,
    Relative = 1,
    Continue = 2,
}

/// UHSHandleState
#[doc(alias("UHSHandleState", "UhsCosClientState"))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientState(pub u32);

impl ClientState {
    pub const INIT: Self = Self(0);
    pub const OPENING: Self = Self(1);
    pub const OPENED: Self = Self(2);
    pub const CLOSING: Self = Self(3);
    pub const CLOSED: Self = Self(4);
    pub const ERROR: Self = Self(5);
}

/// UhsConfig
#[doc(alias("UhsConfig", "UhsCosClientConfig"))]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config {
    /// USB host controller.
    pub controller: i32,
    /// Transaction memory, 64-byte aligned.
    pub buf: *mut ffi::c_void,
    /// At least 4991 bytes.
    pub buf_size: u32,
}

/// UhsHandle
#[doc(alias("UhsHandle", "UhsCosClientInstance"))]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client {
    pub state: ClientState,
    pub buf_pool: *mut ffi::c_void,
    pub state_sequence: i32,
    pub handle: i32,
    pub config: *mut Config,
    pub last_probed_interface: InterfaceHandle,
}

/// UHS_DEVICE_ID_MATCH_*
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchFlags(pub u16);

impl MatchFlags {
    pub const VENDOR: Self = Self(0x0001);
    pub const PRODUCT: Self = Self(0x0002);
    pub const DEV_LO: Self = Self(0x0004);
    pub const DEV_HI: Self = Self(0x0008);
    pub const DEV_CLASS: Self = Self(0x0010);
    pub const DEV_SUBCLASS: Self = Self(0x0020);
    pub const DEV_PROTOCOL: Self = Self(0x0040);
    pub const INT_CLASS: Self = Self(0x0080);
    pub const INT_SUBCLASS: Self = Self(0x0100);
    pub const INT_PROTOCOL: Self = Self(0x0200);
}

impl core::ops::BitOr for MatchFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// UhsInterfaceFilter
#[doc(alias("UhsInterfaceFilter", "UhsDeviceId"))]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceFilter {
    pub match_flags: MatchFlags,
    pub vendor: u16,
    pub product: u16,
    pub bcd_device_lo: u16,
    pub bcd_device_hi: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
}

/// UhsDeviceDescriptor
#[doc(alias = "UhsDeviceDescriptor")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub vendor: u16,
    pub product: u16,
    pub bcd_device: u16,
    pub manufacturer: u8,
    pub product_string: u8,
    pub serial_number: u8,
    pub num_configurations: u8,
}

/// UhsConfigDescriptor
#[doc(alias = "UhsConfigDescriptor")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub total_length: u16,
    pub num_interfaces: u8,
    pub configuration_value: u8,
    pub configuration: u8,
    pub attributes: u8,
    pub max_power: u8,
}

/// UhsInterfaceDescriptor
#[doc(alias = "UhsInterfaceDescriptor")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
    pub interface: u8,
}

/// UhsEndpointDescriptor
#[doc(alias = "UhsEndpointDescriptor")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub endpoint_address: u8,
    pub attributes: u8,
    pub max_packet_size: u16,
    pub interval: u8,
    pub refresh: u8,
    pub synch_address: u8,
}

/// UhsInterfaceProfile
#[doc(alias = "UhsInterfaceProfile")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceProfile {
    pub interface: InterfaceHandle,
    pub speed: DeviceSpeed,
    pub controller_group: i32,
    pub controller_index: i32,
    pub ports: [i16; MAX_HUB_HIERARCHY],
    pub device_uid: u32,
    pub alt_settings: i32,
    pub device: DeviceDescriptor,
    pub config: ConfigDescriptor,
    pub interface_descriptor: InterfaceDescriptor,
    pub in_endpoints: [EndpointDescriptor; MAX_ENDPOINT_PAIRS],
    pub out_endpoints: [EndpointDescriptor; MAX_ENDPOINT_PAIRS],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsocFrame {
    pub buf_size: u32,
    pub status: Status,
    pub xfer_size: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsocPayload {
    pub num_frames: u32,
    pub buf: *mut ffi::c_void,
    pub total_buf_size: u32,
    pub frames: [IsocFrame; MAX_ISOC_FRAMES],
}

const _: () = {
    assert!(size_of::<InterfaceFilter>() == 0x10);
    assert!(size_of::<DeviceDescriptor>() == 0x12);
    assert!(size_of::<EndpointDescriptor>() == 0x09);
    assert!(size_of::<InterfaceProfile>() == 0x16C);
};

/// UHSDrvRegCallback
#[doc(alias("UHSDrvRegCallback", "UhsIfProbeFunction"))]
pub type ProbeCallback =
    Option<unsafe extern "C" fn(context: *mut ffi::c_void, profile: *mut InterfaceProfile)>;

/// UhsAcquireInterfaceCallback
#[doc(alias("UhsAcquireInterfaceCallback", "UhsIfIndicationFunction"))]
pub type IndicationCallback = Option<
    unsafe extern "C" fn(
        context: *mut ffi::c_void,
        interface: InterfaceHandle,
        indication: InterfaceIndication,
    ),
>;

pub type ControlCallback = Option<
    unsafe extern "C" fn(
        context: *mut ffi::c_void,
        status: Status,
        interface: InterfaceHandle,
        request: u8,
        request_type: u8,
        value: u16,
        index: u16,
        data: *mut ffi::c_void,
        xfer_size: u32,
    ),
>;

#[doc(alias("UhsBulkUrbCompletionCallback", "UhsInterruptUrbCompletionCallback"))]
pub type TransferCallback = Option<
    unsafe extern "C" fn(
        context: *mut ffi::c_void,
        status: Status,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        xfer_size: u32,
    ),
>;

pub type IsocCallback = Option<
    unsafe extern "C" fn(
        context: *mut ffi::c_void,
        status: Status,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        num_entries: u32,
        frames: *mut IsocFrame,
    ),
>;

unsafe extern "C" {
    /// UhsClientOpen
    #[link_name = "UhsClientOpen"]
    pub unsafe fn client_open(client: *mut Client, config: *const Config) -> Status;

    /// UhsClientClose
    #[link_name = "UhsClientClose"]
    pub unsafe fn client_close(client: *mut Client) -> Status;

    /// UhsClassDrvReg
    ///
    /// Registers a class driver. `probe` is called for every matching interface.
    #[link_name = "UhsClassDrvReg"]
    pub unsafe fn class_drv_reg(
        client: *mut Client,
        filter: *mut InterfaceFilter,
        context: *mut ffi::c_void,
        probe: ProbeCallback,
    ) -> DevRegHandle;

    /// UhsClassDrvUnReg
    #[link_name = "UhsClassDrvUnReg"]
    pub unsafe fn class_drv_unreg(client: *mut Client, handle: DevRegHandle) -> Status;

    /// UhsQueryInterfaces
    #[link_name = "UhsQueryInterfaces"]
    pub unsafe fn query_interfaces(
        client: *mut Client,
        filter: *mut InterfaceFilter,
        profiles: *mut InterfaceProfile,
        max_profiles: u32,
    ) -> Status;

    /// UhsAcquireInterface
    #[link_name = "UhsAcquireInterface"]
    pub unsafe fn acquire_interface(
        client: *mut Client,
        interface: InterfaceHandle,
        context: *mut ffi::c_void,
        indication: IndicationCallback,
    ) -> Status;

    /// UhsReleaseInterface
    #[link_name = "UhsReleaseInterface"]
    pub unsafe fn release_interface(
        client: *mut Client,
        interface: InterfaceHandle,
        lock_out: ffi::c_bool,
    ) -> Status;

    /// UhsSetInterface
    #[link_name = "UhsSetInterface"]
    pub unsafe fn set_interface(
        client: *mut Client,
        interface: InterfaceHandle,
        alternate_setting: u8,
    ) -> Status;

    /// UhsGetInterface
    #[link_name = "UhsGetInterface"]
    pub unsafe fn get_interface(
        client: *mut Client,
        interface: InterfaceHandle,
        profile: *mut InterfaceProfile,
    ) -> Status;

    /// UhsGetAlternateInterface
    #[link_name = "UhsGetAlternateInterface"]
    pub unsafe fn get_alternate_interface(
        client: *mut Client,
        interface: InterfaceHandle,
        alternate_setting: u8,
        profile: *mut InterfaceProfile,
    ) -> Status;

    /// UhsAdministerEndpoint
    #[link_name = "UhsAdministerEndpoint"]
    pub unsafe fn administer_endpoint(
        client: *mut Client,
        interface: InterfaceHandle,
        command: EndpointCommand,
        endpoints: EndpointMask,
        max_urb: i32,
        max_xfer_size: u32,
    ) -> Status;

    /// UhsAdministerEndpointOpt
    #[link_name = "UhsAdministerEndpointOpt"]
    pub unsafe fn administer_endpoint_opt(
        client: *mut Client,
        interface: InterfaceHandle,
        command: EndpointCommand,
        endpoints: EndpointMask,
        max_urb: i32,
        max_xfer_size: u32,
        options: EndpointOptions,
    ) -> Status;

    /// UhsClearEndpointHalt
    #[link_name = "UhsClearEndpointHalt"]
    pub unsafe fn clear_endpoint_halt(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
    ) -> Status;

    /// UhsAdministerDevice
    #[link_name = "UhsAdministerDevice"]
    pub unsafe fn administer_device(
        client: *mut Client,
        interface: InterfaceHandle,
        command: DeviceCommand,
        options: DeviceOptions,
    ) -> Status;

    /// UhsGetDescriptorString
    #[link_name = "UhsGetDescriptorString"]
    pub unsafe fn get_descriptor_string(
        client: *mut Client,
        interface: InterfaceHandle,
        index: u8,
        unicode: ffi::c_bool,
        buf: *mut ffi::c_char,
        size: u32,
    ) -> Status;

    /// UhsGetFullConfigDescriptor
    #[link_name = "UhsGetFullConfigDescriptor"]
    pub unsafe fn get_full_config_descriptor(
        client: *mut Client,
        interface: InterfaceHandle,
        buf: *mut ffi::c_void,
        size: u32,
    ) -> Status;

    /// UhsSubmitControlRequest
    #[link_name = "UhsSubmitControlRequest"]
    pub unsafe fn submit_control_request(
        client: *mut Client,
        interface: InterfaceHandle,
        data: *mut ffi::c_void,
        request: u8,
        request_type: u8,
        value: u16,
        index: u16,
        length: u16,
        timeout: Time,
    ) -> Status;

    /// UhsSubmitControlRequestAsync
    #[link_name = "UhsSubmitControlRequestAsync"]
    pub unsafe fn submit_control_request_async(
        client: *mut Client,
        interface: InterfaceHandle,
        data: *mut ffi::c_void,
        request: u8,
        request_type: u8,
        value: u16,
        index: u16,
        length: u16,
        context: *mut ffi::c_void,
        callback: ControlCallback,
    ) -> Status;

    /// UhsSubmitInterruptRequest
    #[link_name = "UhsSubmitInterruptRequest"]
    pub unsafe fn submit_interrupt_request(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        size: u32,
        timeout: Time,
    ) -> Status;

    /// UhsSubmitInterruptRequestAsync
    #[link_name = "UhsSubmitInterruptRequestAsync"]
    pub unsafe fn submit_interrupt_request_async(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        size: u32,
        context: *mut ffi::c_void,
        callback: TransferCallback,
    ) -> Status;

    /// UhsSubmitBulkRequest
    #[link_name = "UhsSubmitBulkRequest"]
    pub unsafe fn submit_bulk_request(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        size: u32,
        timeout: Time,
    ) -> Status;

    /// UhsSubmitBulkRequestAsync
    #[link_name = "UhsSubmitBulkRequestAsync"]
    pub unsafe fn submit_bulk_request_async(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        data: *mut ffi::c_void,
        size: u32,
        context: *mut ffi::c_void,
        callback: TransferCallback,
    ) -> Status;

    /// UhsSubmitIsocRequest
    #[link_name = "UhsSubmitIsocRequest"]
    pub unsafe fn submit_isoc_request(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        start_frame: u32,
        start_type: IsocStartType,
        payload: *mut IsocPayload,
        timeout: Time,
    ) -> Status;

    /// UhsSubmitIsocRequestAsync
    #[link_name = "UhsSubmitIsocRequestAsync"]
    pub unsafe fn submit_isoc_request_async(
        client: *mut Client,
        interface: InterfaceHandle,
        endpoint: i8,
        direction: Direction,
        start_frame: u32,
        start_type: IsocStartType,
        payload: *mut IsocPayload,
        context: *mut ffi::c_void,
        callback: IsocCallback,
    ) -> Status;

    /// UhsGetCurrentFrame
    #[link_name = "UhsGetCurrentFrame"]
    pub unsafe fn get_current_frame(
        client: *mut Client,
        interface: InterfaceHandle,
        frame: *mut u32,
    ) -> Status;
}

imports_section!(
    "nsysuhs",
    [
        "UhsAcquireInterface",
        "UhsAdministerDevice",
        "UhsAdministerEndpoint",
        "UhsAdministerEndpointOpt",
        "UhsClassDrvReg",
        "UhsClassDrvUnReg",
        "UhsClearEndpointHalt",
        "UhsClientClose",
        "UhsClientOpen",
        "UhsGetAlternateInterface",
        "UhsGetCurrentFrame",
        "UhsGetDescriptorString",
        "UhsGetFullConfigDescriptor",
        "UhsGetInterface",
        "UhsQueryInterfaces",
        "UhsReleaseInterface",
        "UhsSetInterface",
        "UhsSubmitBulkRequest",
        "UhsSubmitBulkRequestAsync",
        "UhsSubmitControlRequest",
        "UhsSubmitControlRequestAsync",
        "UhsSubmitInterruptRequest",
        "UhsSubmitInterruptRequestAsync",
        "UhsSubmitIsocRequest",
        "UhsSubmitIsocRequestAsync"
    ],
    []
);
