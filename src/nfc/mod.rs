//! NFC

use crate::{ffi, vpad::Channel};

pub const MAX_UID_SIZE: usize = 10;

/// NFCError
#[doc(alias = "NFCError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const INIT: Self = Self(-5000);
    pub const NOT_INIT: Self = Self(-4999);
    pub const TIMEOUT: Self = Self(-4998);
    pub const INVALID_COMMAND: Self = Self(-4997);
    pub const INVALID_TAG: Self = Self(-4996);
    pub const GET_TAG_INFO: Self = Self(-4995);
    pub const ABORT: Self = Self(-4994);
    pub const OTHER: Self = Self(-4993);
    pub const GET_TAG_INFO_MULTI: Self = Self(-4992);
}

/// NFCMode
#[doc(alias = "NFCMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mode(pub i32);

impl Mode {
    pub const INVALID: Self = Self(-1);
    pub const IDLE: Self = Self(0);
    /// Discovering or has an active tag.
    pub const ACTIVE: Self = Self(1);
}

/// NFCProtocol
#[doc(alias = "NFCProtocol")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Protocol(pub u8);

impl Protocol {
    pub const UNKNOWN: Self = Self(0x00);
    pub const T1T: Self = Self(0x01);
    pub const T2T: Self = Self(0x02);
    pub const T3T: Self = Self(0x03);
    pub const ISO_DEP: Self = Self(0x04);
    pub const ISO15693: Self = Self(0x83);
}

/// NFCTechnology
#[doc(alias = "NFCTechnology")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Technology(pub u8);

impl Technology {
    pub const A: Self = Self(0x00);
    pub const B: Self = Self(0x01);
    pub const F: Self = Self(0x02);
    pub const ISO15693: Self = Self(0x06);
}

/// NFCTechnologyMask
#[doc(alias = "NFCTechnologyMask")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TechnologyMask(pub u8);

impl TechnologyMask {
    pub const ALL: Self = Self(0);
    pub const A: Self = Self(1 << 0);
    pub const B: Self = Self(1 << 1);
    pub const F: Self = Self(1 << 2);
    pub const ISO15693: Self = Self(1 << 3);
}

impl core::ops::BitOr for TechnologyMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// NFCNDEFFlags
#[doc(alias = "NFCNDEFFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NdefFlags(pub u8);

impl NdefFlags {
    pub const READ_ONLY: Self = Self(1 << 0);
    pub const FORMATTED: Self = Self(1 << 1);
    pub const SUPPORTED: Self = Self(1 << 2);
    pub const UNKNOWN: Self = Self(1 << 3);
    pub const FORMATTABLE: Self = Self(1 << 4);
    pub const SOFT_LOCKABLE: Self = Self(1 << 5);
    pub const HARD_LOCKABLE: Self = Self(1 << 6);
    /// One time programmable.
    pub const OTP: Self = Self(1 << 7);
}

/// NFCUid
#[doc(alias = "NFCUid")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uid {
    pub uid: [u8; 7],
}

/// NFCTagInfo
#[doc(alias = "NFCTagInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagInfo {
    pub uid_size: u8,
    pub uid: [u8; MAX_UID_SIZE],
    pub technology: Technology,
    pub protocol: Protocol,
    pub reserved: ffi::unknown<0x20>,
}

/// NFCNTAGVersion
#[doc(alias = "NFCNTAGVersion")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NtagVersion {
    pub header: u8,
    pub vendor_id: u8,
    pub product_type: u8,
    pub product_subtype: u8,
    pub major_product_version: u8,
    pub minor_product_version: u8,
    pub storage_size: u8,
    pub protocol_type: u8,
}

/// NFCReadT2TResult
#[doc(alias = "NFCReadT2TResult")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadT2TResult {
    pub num_reads: u8,
    pub read0_start: u8,
    pub read0_end: u8,
    pub read1_start: u8,
    pub read1_end: u8,
    pub read2_start: u8,
    pub read2_end: u8,
    pub read3_start: u8,
    pub read3_end: u8,
    pub data: [u8; 0x3A0],
}

/// NFCCallbackFn
#[doc(alias = "NFCCallbackFn")]
pub type Callback =
    Option<unsafe extern "C" fn(chan: Channel, error: Error, context: *mut ffi::c_void)>;

/// NFCTagDetectCallbackFn
#[doc(alias = "NFCTagDetectCallbackFn")]
pub type TagDetectCallback =
    Option<unsafe extern "C" fn(chan: Channel, has_tag: ffi::c_bool, context: *mut ffi::c_void)>;

/// NFCDetectCallbackFn
#[doc(alias = "NFCDetectCallbackFn")]
pub type DetectCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        uid: *mut Uid,
        read_only: ffi::c_bool,
        has_locked_data: ffi::c_bool,
        flags: NdefFlags,
        context: *mut ffi::c_void,
    ),
>;

/// NFCRawDataCallbackFn
#[doc(alias = "NFCRawDataCallbackFn")]
pub type RawDataCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        response_size: u32,
        response: *mut ffi::c_void,
        context: *mut ffi::c_void,
    ),
>;

/// NFCRawDataTwiceCallbackFn
#[doc(alias = "NFCRawDataTwiceCallbackFn")]
pub type RawDataTwiceCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        num_completed: u8,
        response0_size: u32,
        response0: *mut ffi::c_void,
        response1_size: u32,
        response1: *mut ffi::c_void,
        context: *mut ffi::c_void,
    ),
>;

/// NFCReadCallbackFn
#[doc(alias = "NFCReadCallbackFn")]
pub type ReadCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        uid: *mut Uid,
        read_only: ffi::c_bool,
        data_size: u32,
        data: *mut ffi::c_void,
        locked_data_size: u32,
        locked_data: *mut ffi::c_void,
        context: *mut ffi::c_void,
    ),
>;

/// NFCReadT2TCallbackFn
#[doc(alias = "NFCReadT2TCallbackFn")]
pub type ReadT2TCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        rf_disc_id: u8,
        protocol: Protocol,
        technology: Technology,
        uid_size: u8,
        uid: *mut u8,
        version: *mut NtagVersion,
        result: *mut ReadT2TResult,
        context: *mut ffi::c_void,
    ),
>;

/// NFCGetTagInfoCallbackFn
#[doc(alias = "NFCGetTagInfoCallbackFn")]
pub type GetTagInfoCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        tag_info: *mut TagInfo,
        context: *mut ffi::c_void,
    ),
>;

/// NFCGetTagInfoMultiCallbackFn
#[doc(alias = "NFCGetTagInfoMultiCallbackFn")]
pub type GetTagInfoMultiCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        num_tag_infos: u8,
        tag_info0: *mut TagInfo,
        tag_info1: *mut TagInfo,
        tag_info2: *mut TagInfo,
        context: *mut ffi::c_void,
    ),
>;

unsafe extern "C" {
    /// NFCInit
    #[doc(alias = "NFCInit")]
    #[link_name = "NFCInit"]
    pub unsafe fn init(chan: Channel) -> Error;

    /// NFCInitEx
    ///
    /// The meaning of `power_mode` is unknown.
    #[doc(alias = "NFCInitEx")]
    #[link_name = "NFCInitEx"]
    pub unsafe fn init_ex(chan: Channel, power_mode: u32) -> Error;

    /// NFCShutdown
    #[doc(alias = "NFCShutdown")]
    #[link_name = "NFCShutdown"]
    pub unsafe fn shutdown(chan: Channel) -> Error;

    /// NFCIsInit
    #[doc(alias = "NFCIsInit")]
    #[link_name = "NFCIsInit"]
    pub unsafe fn is_init(chan: Channel) -> ffi::c_bool;

    /// NFCProc
    ///
    /// Must be called periodically, runs pending callbacks.
    #[doc(alias = "NFCProc")]
    #[link_name = "NFCProc"]
    pub unsafe fn process(chan: Channel);

    /// NFCSetTagDetectCallback
    #[doc(alias = "NFCSetTagDetectCallback")]
    #[link_name = "NFCSetTagDetectCallback"]
    pub unsafe fn set_tag_detect_callback(
        chan: Channel,
        callback: TagDetectCallback,
        context: *mut ffi::c_void,
    );

    /// NFCGetMode
    #[doc(alias = "NFCGetMode")]
    #[link_name = "NFCGetMode"]
    pub unsafe fn get_mode(chan: Channel) -> Mode;

    /// NFCSetMode
    #[doc(alias = "NFCSetMode")]
    #[link_name = "NFCSetMode"]
    pub unsafe fn set_mode(chan: Channel, mode: Mode) -> Error;

    /// NFCAbort
    #[doc(alias = "NFCAbort")]
    #[link_name = "NFCAbort"]
    pub unsafe fn abort(chan: Channel, callback: Callback, context: *mut ffi::c_void) -> Error;

    /// NFCAntennaCheck
    #[doc(alias = "NFCAntennaCheck")]
    #[link_name = "NFCAntennaCheck"]
    pub unsafe fn antenna_check(
        chan: Channel,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCDetect
    ///
    /// Timeouts in this module are in milliseconds.
    #[doc(alias = "NFCDetect")]
    #[link_name = "NFCDetect"]
    pub unsafe fn detect(
        chan: Channel,
        discovery_timeout: u16,
        callback: DetectCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSetLockBitsForT1T
    #[doc(alias = "NFCSetLockBitsForT1T")]
    #[link_name = "NFCSetLockBitsForT1T"]
    pub unsafe fn set_lock_bits_for_t1t(
        chan: Channel,
        num_blocks: u8,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCRead
    #[doc(alias = "NFCRead")]
    #[link_name = "NFCRead"]
    pub unsafe fn read(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        callback: ReadCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCWrite
    #[doc(alias = "NFCWrite")]
    #[link_name = "NFCWrite"]
    pub unsafe fn write(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        size: u32,
        data: *mut ffi::c_void,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawData
    #[doc(alias = "NFCSendRawData")]
    #[link_name = "NFCSendRawData"]
    pub unsafe fn send_raw_data(
        chan: Channel,
        start_discovery: ffi::c_bool,
        discovery_timeout: u16,
        command_timeout: u32,
        command_size: u32,
        response_size: u32,
        command: *mut ffi::c_void,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawDataEx
    #[doc(alias = "NFCSendRawDataEx")]
    #[link_name = "NFCSendRawDataEx"]
    pub unsafe fn send_raw_data_ex(
        chan: Channel,
        start_discovery: ffi::c_bool,
        discovery_timeout: u16,
        command_timeout: u32,
        command_size: u32,
        response_size: u32,
        command: *mut ffi::c_void,
        disable_crc: ffi::c_bool,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawDataEx2
    #[doc(alias = "NFCSendRawDataEx2")]
    #[link_name = "NFCSendRawDataEx2"]
    pub unsafe fn send_raw_data_ex2(
        chan: Channel,
        start_discovery: ffi::c_bool,
        discovery_timeout: u16,
        command_timeout: u32,
        command_size: u32,
        response_size: u32,
        command: *mut ffi::c_void,
        disable_crc: ffi::c_bool,
        technology_mask: TechnologyMask,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawDataWithPrePolling
    #[doc(alias = "NFCSendRawDataWithPrePolling")]
    #[link_name = "NFCSendRawDataWithPrePolling"]
    pub unsafe fn send_raw_data_with_pre_polling(
        chan: Channel,
        start_discovery: ffi::c_bool,
        discovery_timeout: u16,
        command_timeout: u32,
        command_size: u32,
        response_size: u32,
        command: *mut ffi::c_void,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawDataWithPrePollingEx
    #[doc(alias = "NFCSendRawDataWithPrePollingEx")]
    #[link_name = "NFCSendRawDataWithPrePollingEx"]
    pub unsafe fn send_raw_data_with_pre_polling_ex(
        chan: Channel,
        start_discovery: ffi::c_bool,
        discovery_timeout: u16,
        command_timeout: u32,
        command_size: u32,
        response_size: u32,
        command: *mut ffi::c_void,
        callback: RawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSendRawDataTwice
    #[doc(alias = "NFCSendRawDataTwice")]
    #[link_name = "NFCSendRawDataTwice"]
    pub unsafe fn send_raw_data_twice(
        chan: Channel,
        command0_timeout: u32,
        command0_size: u32,
        response0_size: u32,
        command0: *mut ffi::c_void,
        command1_timeout: u32,
        command1_size: u32,
        response1_size: u32,
        command1: *mut ffi::c_void,
        callback: RawDataTwiceCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCFormat
    #[doc(alias = "NFCFormat")]
    #[link_name = "NFCFormat"]
    pub unsafe fn format(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCGetTagInfo
    #[doc(alias = "NFCGetTagInfo")]
    #[link_name = "NFCGetTagInfo"]
    pub unsafe fn get_tag_info(
        chan: Channel,
        discovery_timeout: u16,
        callback: GetTagInfoCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCGetTagInfoMulti
    #[doc(alias = "NFCGetTagInfoMulti")]
    #[link_name = "NFCGetTagInfoMulti"]
    pub unsafe fn get_tag_info_multi(
        chan: Channel,
        discovery_timeout: u16,
        callback: GetTagInfoMultiCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCIsTagPresent
    #[doc(alias = "NFCIsTagPresent")]
    #[link_name = "NFCIsTagPresent"]
    pub unsafe fn is_tag_present(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCSetReadOnly
    #[doc(alias = "NFCSetReadOnly")]
    #[link_name = "NFCSetReadOnly"]
    pub unsafe fn set_read_only(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        read_only: ffi::c_bool,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCReadT2T
    #[doc(alias = "NFCReadT2T")]
    #[link_name = "NFCReadT2T"]
    pub unsafe fn read_t2t(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        version: *mut NtagVersion,
        command_timeout: u32,
        num_reads: u8,
        read0_start: u8,
        read0_end: u8,
        read1_start: u8,
        read1_end: u8,
        read2_start: u8,
        read2_end: u8,
        read3_start: u8,
        read3_end: u8,
        authenticate: u8,
        callback: ReadT2TCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCWriteT2T
    #[doc(alias = "NFCWriteT2T")]
    #[link_name = "NFCWriteT2T"]
    pub unsafe fn write_t2t(
        chan: Channel,
        discovery_timeout: u16,
        uid: *mut Uid,
        uid_mask: *mut Uid,
        version: *mut NtagVersion,
        command_timeout: u32,
        num_writes: u8,
        write0_start: u8,
        write0_size: u8,
        write0: *mut ffi::c_void,
        write1_start: u8,
        write1_size: u8,
        write1: *mut ffi::c_void,
        write2_start: u8,
        write2_size: u8,
        write2: *mut ffi::c_void,
        write3_start: u8,
        write3_size: u8,
        write3: *mut ffi::c_void,
        activation_start: u8,
        deactivation_data: *mut u32,
        activation_data: *mut u32,
        authenticate: u8,
        activate: u8,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NFCGetUIDFromActivationEventData
    #[doc(alias = "NFCGetUIDFromActivationEventData")]
    #[link_name = "NFCGetUIDFromActivationEventData"]
    pub unsafe fn get_uid_from_activation_event_data(
        data: *mut ffi::c_void,
        protocol: *mut Protocol,
        technology: *mut Technology,
        uid_size: *mut u8,
        uid: *mut u8,
    );

    /// __NFCSystemAbort
    #[doc(alias = "__NFCSystemAbort")]
    #[link_name = "__NFCSystemAbort"]
    pub unsafe fn system_abort(chan: Channel) -> Error;
}

imports_section!(
    "nfc",
    [
        "NFCAbort",
        "NFCAntennaCheck",
        "NFCDetect",
        "NFCFormat",
        "NFCGetMode",
        "NFCGetTagInfo",
        "NFCGetTagInfoMulti",
        "NFCGetUIDFromActivationEventData",
        "NFCInit",
        "NFCInitEx",
        "NFCIsInit",
        "NFCIsTagPresent",
        "NFCProc",
        "NFCRead",
        "NFCReadT2T",
        "NFCSendRawData",
        "NFCSendRawDataEx",
        "NFCSendRawDataEx2",
        "NFCSendRawDataTwice",
        "NFCSendRawDataWithPrePolling",
        "NFCSendRawDataWithPrePollingEx",
        "NFCSetLockBitsForT1T",
        "NFCSetMode",
        "NFCSetReadOnly",
        "NFCSetTagDetectCallback",
        "NFCShutdown",
        "NFCWrite",
        "NFCWriteT2T",
        "__NFCSystemAbort"
    ],
    []
);
