//! Human Interface Devices (HID)
//!
//! USB HID devices. The transfer functions block if `callback` is `None` and then return the number of bytes transferred.
//! Otherwise they return immediately and report the result to `callback`.

use crate::ffi;

/// HID_ERROR_CODE
#[doc(alias = "HID_ERROR_CODE")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const NOT_OPEN: Self = Self(-1);
    pub const ALREADY_OPEN: Self = Self(-2);
    pub const FIRMWARE_VERSION: Self = Self(-3);
    pub const FIRMWARE_MALFUNCTION: Self = Self(-4);
    pub const BUSY: Self = Self(-5);
    pub const CANCELLED: Self = Self(-6);
}

/// HIDDeviceHandle
#[doc(alias = "HIDDeviceHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub u32);

/// HIDAttachEvent
#[doc(alias = "HIDAttachEvent")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttachEvent(pub u32);

impl AttachEvent {
    pub const DETACH: Self = Self(0);
    pub const ATTACH: Self = Self(1);
}

/// HIDDevice
#[doc(alias = "HIDDevice")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Device {
    pub handle: Handle,
    pub physical_device_inst: u32,
    pub vid: u16,
    pub pid: u16,
    pub interface_index: u8,
    pub sub_class: u8,
    pub protocol: u8,
    _pad: ffi::unknown<1>,
    pub max_packet_size_rx: u16,
    pub max_packet_size_tx: u16,
}

/// HIDAttachCallback
///
/// Return 1 on [AttachEvent::ATTACH] to take the device. The first client that takes it stops the notification of other clients.
#[doc(alias = "HIDAttachCallback")]
pub type AttachCallback = Option<
    unsafe extern "C" fn(client: *mut Client, device: *mut Device, event: AttachEvent) -> i32,
>;

/// HIDCallback
#[doc(alias = "HIDCallback")]
pub type Callback = Option<
    unsafe extern "C" fn(
        handle: Handle,
        error: Error,
        buf: *mut u8,
        bytes_transferred: u32,
        context: *mut ffi::c_void,
    ),
>;

/// HIDClient
#[doc(alias = "HIDClient")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client {
    pub next: *mut Client,
    pub attach_callback: AttachCallback,
}

unsafe extern "C" {
    /// HIDSetup
    #[doc(alias = "HIDSetup")]
    #[link_name = "HIDSetup"]
    pub unsafe fn init() -> Error;

    /// HIDTeardown
    #[doc(alias = "HIDTeardown")]
    #[link_name = "HIDTeardown"]
    pub unsafe fn deinit() -> Error;

    /// HIDAddClient
    ///
    /// `client` must stay valid until [del_client]. Devices that are already attached are reported during this call.
    #[doc(alias = "HIDAddClient")]
    #[link_name = "HIDAddClient"]
    pub unsafe fn add_client(client: *mut Client, attach_callback: AttachCallback) -> Error;

    /// HIDDelClient
    #[doc(alias = "HIDDelClient")]
    #[link_name = "HIDDelClient"]
    pub unsafe fn del_client(client: *mut Client) -> Error;

    /// HIDGetDescriptor
    #[doc(alias = "HIDGetDescriptor")]
    #[link_name = "HIDGetDescriptor"]
    pub unsafe fn get_descriptor(
        handle: Handle,
        descriptor_type: u8,
        descriptor_index: u8,
        language_id: u16,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDSetDescriptor
    #[doc(alias = "HIDSetDescriptor")]
    #[link_name = "HIDSetDescriptor"]
    pub unsafe fn set_descriptor(
        handle: Handle,
        descriptor_type: u8,
        descriptor_index: u8,
        language_id: u16,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDGetReport
    #[doc(alias = "HIDGetReport")]
    #[link_name = "HIDGetReport"]
    pub unsafe fn get_report(
        handle: Handle,
        report_type: u8,
        report_id: u8,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDSetReport
    #[doc(alias = "HIDSetReport")]
    #[link_name = "HIDSetReport"]
    pub unsafe fn set_report(
        handle: Handle,
        report_type: u8,
        report_id: u8,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDGetProtocol
    #[doc(alias = "HIDGetProtocol")]
    #[link_name = "HIDGetProtocol"]
    pub unsafe fn get_protocol(
        handle: Handle,
        interface_index: u8,
        protocol: *mut u8,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDSetProtocol
    #[doc(alias = "HIDSetProtocol")]
    #[link_name = "HIDSetProtocol"]
    pub unsafe fn set_protocol(
        handle: Handle,
        interface_index: u8,
        protocol: u8,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDGetIdle
    #[doc(alias = "HIDGetIdle")]
    #[link_name = "HIDGetIdle"]
    pub unsafe fn get_idle(
        handle: Handle,
        interface_index: u8,
        report_id: u8,
        duration: *mut u8,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDSetIdle
    #[doc(alias = "HIDSetIdle")]
    #[link_name = "HIDSetIdle"]
    pub unsafe fn set_idle(
        handle: Handle,
        interface_index: u8,
        report_id: u8,
        duration: u8,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDRead
    #[doc(alias = "HIDRead")]
    #[link_name = "HIDRead"]
    pub unsafe fn read(
        handle: Handle,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDWrite
    #[doc(alias = "HIDWrite")]
    #[link_name = "HIDWrite"]
    pub unsafe fn write(
        handle: Handle,
        buf: *mut u8,
        buf_len: u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDResetDevice
    #[doc(alias = "HIDResetDevice")]
    #[link_name = "HIDResetDevice"]
    pub unsafe fn reset_device(
        handle: Handle,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// HIDDecodeError
    ///
    /// Splits an error into category and code.
    #[doc(alias = "HIDDecodeError")]
    #[link_name = "HIDDecodeError"]
    pub unsafe fn decode_error(error: Error, category: *mut u32, code: *mut i32);
}

imports_section!(
    "nsyshid",
    [
        "HIDAddClient",
        "HIDAddPassiveClient",
        "HIDDecodeError",
        "HIDDelClient",
        "HIDDelPassiveClient",
        "HIDGetDescriptor",
        "HIDGetHierarchy",
        "HIDGetIdle",
        "HIDGetProtocol",
        "HIDGetReport",
        "HIDRead",
        "HIDResetDevice",
        "HIDSetDescriptor",
        "HIDSetIdle",
        "HIDSetProtocol",
        "HIDSetReport",
        "HIDSetup",
        "HIDTeardown",
        "HIDWrite"
    ],
    []
);
