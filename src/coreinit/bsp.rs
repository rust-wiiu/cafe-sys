//! BSP
//!
//! Board support package: hardware version and board configuration entities.

use crate::ffi;

/// BSPError
#[doc(alias = "BSPError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub u32);

impl Error {
    pub const SUCCESS: Self = Self(0x0);
    pub const UNKNOWN_ENTITY: Self = Self(0x1);
    pub const UNKNOWN_ATTRIBUTE: Self = Self(0x2);
    pub const INVALID_INSTANCE: Self = Self(0x4);
    pub const CFG_CORRUPTED: Self = Self(0x8);
    pub const DEVICE_ERROR: Self = Self(0x10);
    pub const DEVICE_BUSY: Self = Self(0x20);
    pub const IOS_ERROR: Self = Self(0x40);
    pub const SPECIFIED_SIZE_INVALID: Self = Self(0x80);
    pub const HEAP_ERROR: Self = Self(0x100);
    pub const ENTITY_LIST_FULL: Self = Self(0x200);
    pub const UNSUPPORTED_METHOD: Self = Self(0x400);
    pub const UNKNOWN_HARDWARE_VERSION: Self = Self(0x800);
    pub const QUERY_UNAVAILABLE: Self = Self(0x1000);
    pub const UNKNOWN_DEVICE: Self = Self(0x2000);
    pub const INVALID_PARAMETER: Self = Self(0x4000);
    pub const DEVICE_NOT_FOUND: Self = Self(0x8000);
    pub const REQUEST_DENIED: Self = Self(0x10000);
    pub const UNSUPPORTED_PARAMETER: Self = Self(0x20000);
    pub const BOARD_CONFIG_INVALID: Self = Self(0x40000);
}

/// BSPHardwareVersion
#[doc(alias = "BSPHardwareVersion", alias = "BSPHardwareVersions")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareVersion(pub u32);

impl HardwareVersion {
    pub const UNKNOWN: Self = Self(0x00000000);
    pub const HOLLYWOOD_ENG_SAMPLE_1: Self = Self(0x00000001);
    pub const HOLLYWOOD_ENG_SAMPLE_2: Self = Self(0x10000001);
    pub const HOLLYWOOD_PROD_FOR_WII: Self = Self(0x10100001);
    pub const HOLLYWOOD_CORTADO: Self = Self(0x10100008);
    pub const HOLLYWOOD_CORTADO_ESPRESSO: Self = Self(0x1010000C);
    pub const BOLLYWOOD: Self = Self(0x20000001);
    pub const BOLLYWOOD_PROD_FOR_WII: Self = Self(0x20100001);
    pub const LATTE_A11_EV: Self = Self(0x21100010);
    pub const LATTE_A11_CAT: Self = Self(0x21100020);
    pub const LATTE_A12_EV: Self = Self(0x21200010);
    pub const LATTE_A12_CAT: Self = Self(0x21200020);
    pub const LATTE_A2X_EV: Self = Self(0x22100010);
    pub const LATTE_A2X_CAT: Self = Self(0x22100020);
    pub const LATTE_A3X_EV: Self = Self(0x23100010);
    pub const LATTE_A3X_CAT: Self = Self(0x23100020);
    pub const LATTE_A3X_CAFE: Self = Self(0x23100028);
    pub const LATTE_A4X_EV: Self = Self(0x24100010);
    pub const LATTE_A4X_CAT: Self = Self(0x24100020);
    pub const LATTE_A4X_CAFE: Self = Self(0x24100028);
    pub const LATTE_A5X_EV: Self = Self(0x25100010);
    pub const LATTE_A5X_EV_Y: Self = Self(0x25100011);
    pub const LATTE_A5X_CAT: Self = Self(0x25100020);
    pub const LATTE_A5X_CAFE: Self = Self(0x25100028);
    pub const LATTE_B1X_EV: Self = Self(0x26100010);
    pub const LATTE_B1X_EV_Y: Self = Self(0x26100011);
    pub const LATTE_B1X_CAT: Self = Self(0x26100020);
    pub const LATTE_B1X_CAFE: Self = Self(0x26100028);
}

/// BSPConsoleTypeRaw
#[doc(alias = "BSPConsoleTypeRaw")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsoleTypeRaw(pub u32);

unsafe extern "C" {
    /// bspInitializeShimInterface
    #[doc(alias = "bspInitializeShimInterface")]
    #[link_name = "bspInitializeShimInterface"]
    pub unsafe fn initialize_shim_interface() -> Error;

    /// bspGetHardwareVersion
    #[doc(alias = "bspGetHardwareVersion")]
    #[link_name = "bspGetHardwareVersion"]
    pub unsafe fn get_hardware_version(version: *mut HardwareVersion) -> Error;

    /// bspGetConsoleTypeRaw
    #[doc(alias = "bspGetConsoleTypeRaw")]
    #[link_name = "bspGetConsoleTypeRaw"]
    pub unsafe fn get_console_type_raw(console_type: *mut ConsoleTypeRaw) -> Error;

    /// bspInitialize
    #[doc(alias = "bspInitialize")]
    #[link_name = "bspInitialize"]
    pub unsafe fn initialize(
        entity: *const ffi::c_char,
        instance: u32,
        attribute: *const ffi::c_char,
        options_size: u32,
        options: *const ffi::c_void,
    ) -> Error;

    /// bspShutdown
    #[doc(alias = "bspShutdown")]
    #[link_name = "bspShutdown"]
    pub unsafe fn shutdown(
        entity: *const ffi::c_char,
        instance: u8,
        attribute: *const ffi::c_char,
    ) -> Error;

    /// bspGetEntityVersion
    #[doc(alias = "bspGetEntityVersion")]
    #[link_name = "bspGetEntityVersion"]
    pub unsafe fn get_entity_version(entity: *const ffi::c_char, version: *mut u32) -> Error;

    /// bspRead
    #[doc(alias = "bspRead")]
    #[link_name = "bspRead"]
    pub unsafe fn read(
        entity: *const ffi::c_char,
        instance: u8,
        attribute: *const ffi::c_char,
        size: usize,
        data: *mut ffi::c_void,
    ) -> Error;

    /// bspWrite
    #[doc(alias = "bspWrite")]
    #[link_name = "bspWrite"]
    pub unsafe fn write(
        entity: *const ffi::c_char,
        instance: u8,
        attribute: *const ffi::c_char,
        size: usize,
        data: *const ffi::c_void,
    ) -> Error;

    /// bspQuery
    #[doc(alias = "bspQuery")]
    #[link_name = "bspQuery"]
    pub unsafe fn query(
        entity: *const ffi::c_char,
        instance: u8,
        attribute: *const ffi::c_char,
        size: usize,
        data: *mut ffi::c_void,
    ) -> Error;
}
