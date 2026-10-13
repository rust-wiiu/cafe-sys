//! NTAG
//!
//! NFC tag library used for amiibo figures. Builds on [nfc]. Timeouts are in milliseconds.

use crate::{ffi, nfc, nn::ffl::StoreData, vpad::Channel};

/// NTAGError
#[doc(alias = "NTAGError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const INIT: Self = Self(-1000);
    pub const NOT_INIT: Self = Self(-999);
    pub const INVALID_COMMAND: Self = Self(-998);
    pub const TIMEOUT: Self = Self(-997);
    pub const INVALID_TAG: Self = Self(-996);
    pub const INVALID_FORMAT: Self = Self(-995);
    pub const READ: Self = Self(-994);
    pub const WRITE: Self = Self(-993);
    pub const FORMAT: Self = Self(-992);
    pub const DETECT: Self = Self(-991);
    pub const WRITE_READ_ONLY: Self = Self(-990);
    pub const NEED_FORMAT: Self = Self(-989);
    pub const SET_READ_ONLY: Self = Self(-988);
    pub const INVALID_UID: Self = Self(-987);
    pub const ABORT: Self = Self(-986);
    pub const SHUTDOWN: Self = Self(-985);
    pub const OTHER: Self = Self(-984);
}

/// NTAGFormatSettings
#[doc(alias = "NTAGFormatSettings")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatSettings {
    pub version: u8,
    _pad: ffi::unknown<0x3>,
    pub maker_code: u32,
    pub identify_code: u32,
    pub reserved: ffi::unknown<0x1C>,
}

/// NTAGNoftHeader
#[doc(alias = "NTAGNoftHeader")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoftHeader {
    pub magic: u32,
    pub version: u8,
    pub write_count: u16,
    pub unknown: u8,
}

/// NTAGInfoHeader
#[doc(alias = "NTAGInfoHeader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfoHeader {
    pub rw_header_offset: u16,
    pub rw_size: u16,
    pub ro_header_offset: u16,
    pub ro_size: u16,
    pub uid: nfc::Uid,
    pub format_version: u8,
}

/// NTAGAreaHeader
#[doc(alias = "NTAGAreaHeader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AreaHeader {
    pub magic: u16,
    pub offset: u16,
    pub size: u16,
    _pad: ffi::unknown<0x2>,
    pub maker_code: u32,
    pub identify_code: u32,
}

/// NTAGAreaInfo
#[doc(alias = "NTAGAreaInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AreaInfo {
    pub data: *mut ffi::c_void,
    pub size: u16,
    _pad: ffi::unknown<0x2>,
    pub maker_code: u32,
    pub identify_code: u32,
    pub reserved: ffi::unknown<0x20>,
}

/// NTAGData
#[doc(alias("NTAGData", "NTAGInfo"))]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Data {
    pub uid: nfc::Uid,
    pub read_only: u8,
    pub format_version: u8,
    _pad: ffi::unknown<0x3>,
    pub rw_info: AreaInfo,
    pub ro_info: AreaInfo,
    pub reserved: ffi::unknown<0x20>,
}

/// NTAGInfoT2T
#[doc(alias = "NTAGInfoT2T")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfoT2T {
    pub magic: u8,
    pub writes: u16,
    pub figure_version: u8,
    pub flags: u8,
    pub character_id: [u8; 3],
    pub numbering_id: u16,
    pub figure_type: u8,
    pub series_id: u8,
    pub unknown: u32,
    pub application_area_writes: u16,
    pub crc_counter: u16,
    pub crc: u32,
    pub font_region: u8,
    /// UTF-16.
    pub name: [u16; 10],
    pub mii: StoreData,
    pub country: u8,
    pub setup_date: u16,
    pub last_write_date: u16,
    pub access_id: u32,
    pub title_id: u64,
    pub reserved: ffi::unknown<0x1FC>,
}

/// NTAGApplicationDataT2T
#[doc(alias = "NTAGApplicationDataT2T")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationDataT2T {
    pub size: u16,
    pub data: [u8; 0xD8],
}

/// First section of [RawDataT2T].
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawSection0 {
    pub magic: u8,
    pub writes: u16,
    pub figure_version: u8,
    pub flags: u8,
    pub country: u8,
    pub crc_counter: u16,
    pub setup_date: u16,
    pub last_write_date: u16,
    pub crc: u32,
    /// UTF-16.
    pub name: [u16; 10],
}

/// Second section of [RawDataT2T].
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawSection1 {
    pub tag_hmac: [u8; 0x20],
    pub character_id: [u8; 3],
    pub figure_type: u8,
    pub numbering_id: u16,
    pub series_id: u8,
    pub format_version: u8,
    pub unknown: u32,
    pub keygen_salt: [u8; 0x20],
    pub data_hmac: [u8; 0x20],
}

/// Third section of [RawDataT2T].
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawSection2 {
    pub mii: StoreData,
    pub title_id: u64,
    pub application_area_writes: u16,
    pub access_id: u32,
    pub reserved: ffi::unknown<0x22>,
}

/// NTAGRawDataT2T
///
/// Raw tag memory.
#[doc(alias = "NTAGRawDataT2T")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawDataT2T {
    pub uid: [u8; 9],
    pub internal: u8,
    pub lock_bytes: [u8; 2],
    pub capability_container: [u8; 4],
    pub section0: RawSection0,
    pub section1: RawSection1,
    pub section2: RawSection2,
    pub application_data: [u8; 0xD8],
    pub dynamic_lock: [u8; 3],
    pub reserved0: u8,
    pub cfg0: [u8; 4],
    pub cfg1: [u8; 4],
    pub pwd: [u8; 4],
    pub pack: [u8; 2],
    pub reserved1: [u8; 2],
}

/// NTAGRawDataContainerT2T
#[doc(alias = "NTAGRawDataContainerT2T")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawDataContainerT2T {
    pub size: u16,
    pub data: RawDataT2T,
}

/// NTAGDataT2T
#[doc(alias = "NTAGDataT2T")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataT2T {
    pub tag_info: nfc::TagInfo,
    pub format_version: u8,
    pub info: InfoT2T,
    pub app_data: ApplicationDataT2T,
    pub raw: RawDataContainerT2T,
    pub reserved: ffi::unknown<0x20>,
}

const _: () = {
    assert!(size_of::<NoftHeader>() == 0x8);
    assert!(size_of::<InfoHeader>() == 0x10);
    assert!(size_of::<InfoT2T>() == 0x29A);
    assert!(size_of::<ApplicationDataT2T>() == 0xDA);
    assert!(size_of::<RawDataT2T>() == 0x21C);
    assert!(size_of::<RawDataContainerT2T>() == 0x21E);
    assert!(size_of::<DataT2T>() == 0x5E0);
};

/// NTAGCallbackFn
#[doc(alias("NTAGCallbackFn", "NTAGResultCheckCallback"))]
pub type Callback =
    Option<unsafe extern "C" fn(chan: Channel, error: Error, context: *mut ffi::c_void)>;

/// NTAGReadCallbackFn
#[doc(alias("NTAGReadCallbackFn", "NTAGReadCallback"))]
pub type ReadCallback = Option<
    unsafe extern "C" fn(chan: Channel, error: Error, data: *mut Data, context: *mut ffi::c_void),
>;

/// NTAGReadT2TCallbackFn
#[doc(alias = "NTAGReadT2TCallbackFn")]
pub type ReadT2TCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        data: *mut DataT2T,
        context: *mut ffi::c_void,
    ),
>;

/// NTAGReadT2TRawDataCallbackFn
#[doc(alias = "NTAGReadT2TRawDataCallbackFn")]
pub type ReadT2TRawDataCallback = Option<
    unsafe extern "C" fn(
        chan: Channel,
        error: Error,
        tag_info: *mut nfc::TagInfo,
        raw: *mut RawDataContainerT2T,
        context: *mut ffi::c_void,
    ),
>;

unsafe extern "C" {
    /// NTAGInit
    #[doc(alias = "NTAGInit")]
    #[link_name = "NTAGInit"]
    pub unsafe fn init(chan: Channel) -> Error;

    /// NTAGInitEx
    #[doc(alias = "NTAGInitEx")]
    #[link_name = "NTAGInitEx"]
    pub unsafe fn init_ex(chan: Channel) -> Error;

    /// NTAGIsInit
    #[doc(alias = "NTAGIsInit")]
    #[link_name = "NTAGIsInit"]
    pub unsafe fn is_init(chan: Channel) -> ffi::c_bool;

    /// NTAGProc
    ///
    /// Must be called periodically, runs pending callbacks.
    #[doc(alias = "NTAGProc")]
    #[link_name = "NTAGProc"]
    pub unsafe fn process(chan: Channel);

    /// NTAGShutdown
    #[doc(alias = "NTAGShutdown")]
    #[link_name = "NTAGShutdown"]
    pub unsafe fn shutdown(chan: Channel) -> Error;

    /// NTAGAbort
    #[doc(alias = "NTAGAbort")]
    #[link_name = "NTAGAbort"]
    pub unsafe fn abort(chan: Channel, callback: Callback, context: *mut ffi::c_void) -> Error;

    /// NTAGSetTagDetectCallback
    #[doc(alias = "NTAGSetTagDetectCallback")]
    #[link_name = "NTAGSetTagDetectCallback"]
    pub unsafe fn set_tag_detect_callback(
        chan: Channel,
        callback: nfc::TagDetectCallback,
        context: *mut ffi::c_void,
    );

    /// NTAGSetFormatSettings
    #[doc(alias = "NTAGSetFormatSettings")]
    #[link_name = "NTAGSetFormatSettings"]
    pub unsafe fn set_format_settings(settings: *const FormatSettings);

    /// NTAGFormat
    #[doc(alias = "NTAGFormat")]
    #[link_name = "NTAGFormat"]
    pub unsafe fn format(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        rw_size: u16,
        rw_data: *const ffi::c_void,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGSetReadOnly
    #[doc(alias = "NTAGSetReadOnly")]
    #[link_name = "NTAGSetReadOnly"]
    pub unsafe fn set_read_only(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        uid_mask: *const nfc::Uid,
        read_only: ffi::c_bool,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGRead
    #[doc(alias = "NTAGRead")]
    #[link_name = "NTAGRead"]
    pub unsafe fn read(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        uid_mask: *const nfc::Uid,
        callback: ReadCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWrite
    #[doc(alias = "NTAGWrite")]
    #[link_name = "NTAGWrite"]
    pub unsafe fn write(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        rw_size: u16,
        rw_data: *const ffi::c_void,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGReadT2T
    #[doc(alias = "NTAGReadT2T")]
    #[link_name = "NTAGReadT2T"]
    pub unsafe fn read_t2t(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        uid_mask: *const nfc::Uid,
        callback: ReadT2TCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGReadT2TRawData
    #[doc(alias = "NTAGReadT2TRawData")]
    #[link_name = "NTAGReadT2TRawData"]
    pub unsafe fn read_t2t_raw_data(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        uid_mask: *const nfc::Uid,
        callback: ReadT2TRawDataCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWriteT2T
    #[doc(alias = "NTAGWriteT2T")]
    #[link_name = "NTAGWriteT2T"]
    pub unsafe fn write_t2t(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        data: *const RawDataContainerT2T,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWriteT2TConfigArea
    #[doc(alias = "NTAGWriteT2TConfigArea")]
    #[link_name = "NTAGWriteT2TConfigArea"]
    pub unsafe fn write_t2t_config_area(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        cfg0: *const u32,
        cfg1: *const u32,
        pwd: *const u32,
        pack: *const u16,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWriteT2TLockArea
    #[doc(alias = "NTAGWriteT2TLockArea")]
    #[link_name = "NTAGWriteT2TLockArea"]
    pub unsafe fn write_t2t_lock_area(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        lock_bytes: *const u32,
        dynamic_lock: *const u32,
        cc: *const u32,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWriteT2TRawData
    #[doc(alias = "NTAGWriteT2TRawData")]
    #[link_name = "NTAGWriteT2TRawData"]
    pub unsafe fn write_t2t_raw_data(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        data: *const RawDataT2T,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGWriteT2TRawDataEx
    #[doc(alias = "NTAGWriteT2TRawDataEx")]
    #[link_name = "NTAGWriteT2TRawDataEx"]
    pub unsafe fn write_t2t_raw_data_ex(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        data: *const RawDataT2T,
        callback: Callback,
        context: *mut ffi::c_void,
        authenticate: u8,
    ) -> Error;

    /// NTAGWriteT2TWithConvert
    #[doc(alias = "NTAGWriteT2TWithConvert")]
    #[link_name = "NTAGWriteT2TWithConvert"]
    pub unsafe fn write_t2t_with_convert(
        chan: Channel,
        timeout: u16,
        uid: *const nfc::Uid,
        data: *const DataT2T,
        callback: Callback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// NTAGConvertT2T
    #[doc(alias = "NTAGConvertT2T")]
    #[link_name = "NTAGConvertT2T"]
    pub unsafe fn convert_t2t(out: *mut DataT2T, data: *const DataT2T) -> Error;

    /// NTAGParseHeader
    #[doc(alias = "NTAGParseHeader")]
    #[link_name = "NTAGParseHeader"]
    pub unsafe fn parse_header(
        data: *const ffi::c_void,
        noft: *mut NoftHeader,
        info: *mut InfoHeader,
        rw: *mut AreaHeader,
        ro: *mut AreaHeader,
    ) -> Error;
}

imports_section!(
    "ntag",
    [
        "NTAGAbort",
        "NTAGConvertMasterDataToWriteData",
        "NTAGConvertMasterDataToWriteDataForT2T",
        "NTAGConvertT2T",
        "NTAGCreateAllWriteData",
        "NTAGCreateAllWriteDataForT2T",
        "NTAGFormat",
        "NTAGInit",
        "NTAGInitEx",
        "NTAGIsInit",
        "NTAGParseHeader",
        "NTAGProc",
        "NTAGRead",
        "NTAGReadT2T",
        "NTAGReadT2TRawData",
        "NTAGSetFormatSettings",
        "NTAGSetReadOnly",
        "NTAGSetTagDetectCallback",
        "NTAGShutdown",
        "NTAGWrite",
        "NTAGWriteT2T",
        "NTAGWriteT2TConfigArea",
        "NTAGWriteT2TLockArea",
        "NTAGWriteT2TRawData",
        "NTAGWriteT2TRawDataEx",
        "NTAGWriteT2TWithConvert",
        "NTAGDetect"
    ],
    []
);
