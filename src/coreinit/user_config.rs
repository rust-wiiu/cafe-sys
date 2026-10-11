//! User config
//!
//! Reading and writing system configuration values by name.

use super::ios::IoctlVec;
use crate::ffi;

/// UCHandle
#[doc(alias = "UCHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub i32);

/// UCError
#[doc(alias = "UCError", alias = "UCErrors")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const ERROR: Self = Self(-1);
    pub const OTHER: Self = Self(-0x200001);
    pub const SYSTEM: Self = Self(-0x200002);
    pub const ALLOC: Self = Self(-0x200003);
    pub const OPCODE: Self = Self(-0x200004);
    pub const INVALID_PARAM: Self = Self(-0x200005);
    pub const INVALID_TYPE: Self = Self(-0x200006);
    pub const UNSUPPORTED: Self = Self(-0x200007);
    pub const NON_LEAF_NODE: Self = Self(-0x200008);
    pub const KEY_NOT_FOUND: Self = Self(-0x200009);
    pub const MODIFY: Self = Self(-0x20000A);
    pub const STRING_TOO_LONG: Self = Self(-0x20000B);
    pub const ROOT_KEYS_DIFFER: Self = Self(-0x20000C);
    pub const INVALID_LOCATION: Self = Self(-0x20000D);
    pub const BAD_COMMENT: Self = Self(-0x20000E);
    pub const READ_ACCESS: Self = Self(-0x20000F);
    pub const WRITE_ACCESS: Self = Self(-0x200010);
    pub const CREATE_ACCESS: Self = Self(-0x200011);
    pub const FILE_SYS_NAME: Self = Self(-0x200012);
    pub const FILE_SYS_INIT: Self = Self(-0x200013);
    pub const FILE_SYS_MOUNT: Self = Self(-0x200014);
    pub const FILE_OPEN: Self = Self(-0x200015);
    pub const FILE_STAT: Self = Self(-0x200016);
    pub const FILE_READ: Self = Self(-0x200017);
    pub const FILE_WRITE: Self = Self(-0x200018);
    pub const FILE_TOO_BIG: Self = Self(-0x200019);
    pub const FILE_REMOVE: Self = Self(-0x20001A);
    pub const FILE_RENAME: Self = Self(-0x20001B);
    pub const FILE_CLOSE: Self = Self(-0x20001C);
    pub const FILE_SEEK: Self = Self(-0x20001D);
    pub const FILE_CONFIRM: Self = Self(-0x20001E);
    pub const FILE_BACKUP: Self = Self(-0x20001F);
    pub const MALFORMED_XML: Self = Self(-0x200020);
    pub const VERSION: Self = Self(-0x200021);
    pub const NO_IPC_BUFFERS: Self = Self(-0x200022);
    pub const FILE_LOCK_NEEDED: Self = Self(-0x200024);
    pub const SYS_PROT: Self = Self(-0x200028);
}

/// UCCommand
#[doc(alias = "UCCommand")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Command(pub u32);

impl Command {
    pub const READ_SYS_CONFIG: Self = Self(0x30);
    pub const WRITE_SYS_CONFIG: Self = Self(0x31);
    pub const DELETE_SYS_CONFIG: Self = Self(0x32);
    pub const QUERY_SYS_CONFIG: Self = Self(0x33);
    pub const LIST_SYS_CONFIG: Self = Self(0x34);
}

/// UCDataType
#[doc(alias = "UCDataType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataType(pub u32);

impl DataType {
    pub const UNDEFINED: Self = Self(0x00);
    pub const UNSIGNED_BYTE: Self = Self(0x01);
    pub const UNSIGNED_SHORT: Self = Self(0x02);
    pub const UNSIGNED_INT: Self = Self(0x03);
    pub const SIGNED_INT: Self = Self(0x04);
    pub const FLOAT: Self = Self(0x05);
    pub const STRING: Self = Self(0x06);
    pub const HEX_BINARY: Self = Self(0x07);
    pub const COMPLEX: Self = Self(0x08);
    pub const INVALID: Self = Self(0xFF);
}

/// UCSysConfig
///
/// A configuration value, `name` is a dotted path such as `cafe.language`.
#[doc(alias = "UCSysConfig")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SysConfig {
    pub name: [ffi::c_char; 64],
    pub access: u32,
    pub data_type: DataType,
    pub error: Error,
    pub data_size: u32,
    pub data: *mut ffi::c_void,
}

/// UCAsyncCallbackFn
#[doc(alias = "UCAsyncCallbackFn")]
pub type AsyncCallback = Option<
    unsafe extern "C" fn(
        result: Error,
        command: Command,
        count: u32,
        settings: *mut SysConfig,
        context: *mut ffi::c_void,
    ),
>;

/// UCAsyncParams
#[doc(alias = "UCAsyncParams")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncParams {
    pub callback: AsyncCallback,
    pub context: *mut ffi::c_void,
    pub command: Command,
    _unk: ffi::unknown<4>,
    pub count: u32,
    pub settings: *mut SysConfig,
    pub vecs: *mut IoctlVec,
}

unsafe extern "C" {
    /// UCOpen
    ///
    /// Negative on failure.
    #[doc(alias = "UCOpen")]
    #[link_name = "UCOpen"]
    pub unsafe fn open() -> Handle;

    /// UCClose
    #[doc(alias = "UCClose")]
    #[link_name = "UCClose"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// UCReadSysConfig
    #[doc(alias = "UCReadSysConfig")]
    #[link_name = "UCReadSysConfig"]
    pub unsafe fn read_sys_config(handle: Handle, count: u32, settings: *mut SysConfig) -> Error;

    /// UCReadSysConfigAsync
    #[doc(alias = "UCReadSysConfigAsync")]
    #[link_name = "UCReadSysConfigAsync"]
    pub unsafe fn read_sys_config_async(
        handle: Handle,
        count: u32,
        settings: *mut SysConfig,
        params: *mut AsyncParams,
    ) -> Error;

    /// UCWriteSysConfig
    #[doc(alias = "UCWriteSysConfig")]
    #[link_name = "UCWriteSysConfig"]
    pub unsafe fn write_sys_config(handle: Handle, count: u32, settings: *mut SysConfig) -> Error;

    /// UCWriteSysConfigAsync
    #[doc(alias = "UCWriteSysConfigAsync")]
    #[link_name = "UCWriteSysConfigAsync"]
    pub unsafe fn write_sys_config_async(
        handle: Handle,
        count: u32,
        settings: *mut SysConfig,
        params: *mut AsyncParams,
    ) -> Error;

    /// UCDeleteSysConfig
    #[doc(alias = "UCDeleteSysConfig")]
    #[link_name = "UCDeleteSysConfig"]
    pub unsafe fn delete_sys_config(handle: Handle, count: u32, settings: *mut SysConfig) -> Error;

    /// UCDeleteSysConfigAsync
    #[doc(alias = "UCDeleteSysConfigAsync")]
    #[link_name = "UCDeleteSysConfigAsync"]
    pub unsafe fn delete_sys_config_async(
        handle: Handle,
        count: u32,
        settings: *mut SysConfig,
        params: *mut AsyncParams,
    ) -> Error;
}
