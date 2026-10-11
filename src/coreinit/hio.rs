//! HIO
//!
//! Host I/O channels to a PC on development hardware.

use super::message_queue::MessageQueue;
use crate::ffi;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    pub const NEW_CONNECTION: Self = Self(1);
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_CHANNEL_NAME: Self = Self(-1);
    pub const NO_CONNECTIONS: Self = Self(-2);
    pub const NO_CHANNELS: Self = Self(-3);
    pub const INVALID_HANDLE: Self = Self(-4);
    pub const NO_CLIENT_TXN_BUF_AVAILABLE: Self = Self(-5);
    pub const OUT_OF_MEMORY: Self = Self(-6);
    pub const INVALID_PARAM: Self = Self(-7);
    pub const NOT_INITIALIZED: Self = Self(-8);
    pub const INVALID_OPERATION: Self = Self(-9);
    pub const HEADLESS_MODE: Self = Self(-10);
    pub const ALREADY_OPEN: Self = Self(-100);
    pub const ALREADY_CLOSED: Self = Self(-101);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub i32);

impl Handle {
    pub const INVALID: Self = Self(-1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelOptions(pub u32);

impl ChannelOptions {
    pub const READ: Self = Self(1);
    pub const WRITE: Self = Self(2);
    pub const READ_WRITE: Self = Self(3);
}

pub type Callback = Option<unsafe extern "C" fn(status: Status, context: *mut ffi::c_void)>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncStruct {
    pub queue: *mut MessageQueue,
    pub user_data: *mut ffi::c_void,
    pub callback: Callback,
}

unsafe extern "C" {
    /// HIOInit
    #[doc(alias = "HIOInit")]
    #[link_name = "HIOInit"]
    pub unsafe fn init() -> Status;

    /// HIOShutdown
    #[doc(alias = "HIOShutdown")]
    #[link_name = "HIOShutdown"]
    pub unsafe fn shutdown() -> Status;

    /// HIOOpenEx
    #[doc(alias = "HIOOpenEx")]
    #[link_name = "HIOOpenEx"]
    pub unsafe fn open_ex(
        name: *const ffi::c_char,
        async_struct: *mut AsyncStruct,
        options: ChannelOptions,
        flags: u32,
    ) -> Handle;

    /// HIOClose
    #[doc(alias = "HIOClose")]
    #[link_name = "HIOClose"]
    pub unsafe fn close(handle: Handle) -> Status;

    /// HIORead
    ///
    /// Returns the bytes read or an error.
    #[doc(alias = "HIORead")]
    #[link_name = "HIORead"]
    pub unsafe fn read(handle: Handle, size: u32, data: *mut ffi::c_void) -> Status;

    /// HIOReadAsyncEx
    #[doc(alias = "HIOReadAsyncEx")]
    #[link_name = "HIOReadAsyncEx"]
    pub unsafe fn read_async_ex(
        handle: Handle,
        size: u32,
        data: *mut ffi::c_void,
        async_struct: *mut AsyncStruct,
    ) -> Status;

    /// HIOWrite
    ///
    /// Returns the bytes written or an error.
    #[doc(alias = "HIOWrite")]
    #[link_name = "HIOWrite"]
    pub unsafe fn write(handle: Handle, size: u32, data: *const ffi::c_void) -> Status;

    /// HIOWriteAsyncEx
    #[doc(alias = "HIOWriteAsyncEx")]
    #[link_name = "HIOWriteAsyncEx"]
    pub unsafe fn write_async_ex(
        handle: Handle,
        size: u32,
        data: *const ffi::c_void,
        async_struct: *mut AsyncStruct,
    ) -> Status;
}
