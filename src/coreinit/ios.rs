//! IOS
//!
//! IPC with the IOS processor: devices are opened by path and controlled with ioctls.

use crate::ffi;

/// IOSHandle
///
/// Negative values are [Error] codes.
#[doc(alias = "IOSHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub i32);

/// IOSError
#[doc(alias = "IOSError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const ACCESS: Self = Self(-1);
    pub const EXISTS: Self = Self(-2);
    pub const INTERRUPTED: Self = Self(-3);
    pub const INVALID: Self = Self(-4);
    pub const MAX: Self = Self(-5);
    pub const NO_EXISTS: Self = Self(-6);
    pub const QUEUE_EMPTY: Self = Self(-7);
    pub const QUEUE_FULL: Self = Self(-8);
    pub const UNKNOWN: Self = Self(-9);
    pub const NOT_READY: Self = Self(-10);
    pub const ECC: Self = Self(-11);
    pub const ECC_CRITICAL: Self = Self(-12);
    pub const BAD_BLOCK: Self = Self(-13);
    pub const INVALID_OBJECT_TYPE: Self = Self(-14);
    pub const INVALID_RNG: Self = Self(-15);
    pub const INVALID_FLAG: Self = Self(-16);
    pub const INVALID_FORMAT: Self = Self(-17);
    pub const INVALID_VERSION: Self = Self(-18);
    pub const INVALID_SIGNER: Self = Self(-19);
    pub const FAIL_CHECK_VALUE: Self = Self(-20);
    pub const FAIL_INTERNAL: Self = Self(-21);
    pub const FAIL_ALLOC: Self = Self(-22);
    pub const INVALID_SIZE: Self = Self(-23);
    pub const NO_LINK: Self = Self(-24);
    pub const AN_FAILED: Self = Self(-25);
    pub const MAX_SEM_COUNT: Self = Self(-26);
    pub const SEM_UNAVAILABLE: Self = Self(-27);
    pub const INVALID_HANDLE: Self = Self(-28);
    pub const INVALID_ARG: Self = Self(-29);
    pub const NO_RESOURCE: Self = Self(-30);
    pub const BUSY: Self = Self(-31);
    pub const TIMEOUT: Self = Self(-32);
    pub const ALIGNMENT: Self = Self(-33);
    pub const BSP: Self = Self(-34);
    pub const DATA_PENDING: Self = Self(-35);
    pub const EXPIRED: Self = Self(-36);
    pub const NO_READ_ACCESS: Self = Self(-37);
    pub const NO_WRITE_ACCESS: Self = Self(-38);
    pub const NO_READ_WRITE_ACCESS: Self = Self(-39);
    pub const CLIENT_TXN_LIMIT: Self = Self(-40);
    pub const STALE_HANDLE: Self = Self(-41);
    pub const UNKNOWN_VALUE: Self = Self(-42);
}

/// IOSOpenMode
#[doc(alias = "IOSOpenMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpenMode(pub u32);

impl OpenMode {
    pub const NONE: Self = Self(0);
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const READ_WRITE: Self = Self(0b11);
}

impl core::ops::BitOr for OpenMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// IOSVec
#[doc(alias = "IOSVec")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoctlVec {
    pub vaddr: *mut ffi::c_void,
    pub len: u32,
    pub paddr: *mut ffi::c_void,
}

impl Default for IoctlVec {
    fn default() -> Self {
        Self {
            vaddr: core::ptr::null_mut(),
            len: 0,
            paddr: core::ptr::null_mut(),
        }
    }
}

/// IOSAsyncCallbackFn
#[doc(alias = "IOSAsyncCallbackFn")]
pub type AsyncCallback = Option<unsafe extern "C" fn(error: Error, context: *mut ffi::c_void)>;

unsafe extern "C" {
    /// IOS_Open
    ///
    /// Returns the handle, or a negative [Error].
    #[doc(alias = "IOS_Open")]
    #[link_name = "IOS_Open"]
    pub unsafe fn open(device: *const ffi::c_char, mode: OpenMode) -> Handle;

    /// IOS_OpenAsync
    #[doc(alias = "IOS_OpenAsync")]
    #[link_name = "IOS_OpenAsync"]
    pub unsafe fn open_async(
        device: *const ffi::c_char,
        mode: OpenMode,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// IOS_Close
    #[doc(alias = "IOS_Close")]
    #[link_name = "IOS_Close"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// IOS_CloseAsync
    #[doc(alias = "IOS_CloseAsync")]
    #[link_name = "IOS_CloseAsync"]
    pub unsafe fn close_async(
        handle: Handle,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// IOS_Ioctl
    #[doc(alias = "IOS_Ioctl")]
    #[link_name = "IOS_Ioctl"]
    pub unsafe fn ioctl(
        handle: Handle,
        request: u32,
        in_buf: *const ffi::c_void,
        in_len: u32,
        out_buf: *mut ffi::c_void,
        out_len: u32,
    ) -> Error;

    /// IOS_IoctlAsync
    #[doc(alias = "IOS_IoctlAsync")]
    #[link_name = "IOS_IoctlAsync"]
    pub unsafe fn ioctl_async(
        handle: Handle,
        request: u32,
        in_buf: *const ffi::c_void,
        in_len: u32,
        out_buf: *mut ffi::c_void,
        out_len: u32,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// IOS_Ioctlv
    ///
    /// `vec` holds `vec_in` input vectors followed by `vec_out` output vectors.
    #[doc(alias = "IOS_Ioctlv")]
    #[link_name = "IOS_Ioctlv"]
    pub unsafe fn ioctlv(
        handle: Handle,
        request: u32,
        vec_in: u32,
        vec_out: u32,
        vec: *mut IoctlVec,
    ) -> Error;

    /// IOS_IoctlvAsync
    #[doc(alias = "IOS_IoctlvAsync")]
    #[link_name = "IOS_IoctlvAsync"]
    pub unsafe fn ioctlv_async(
        handle: Handle,
        request: u32,
        vec_in: u32,
        vec_out: u32,
        vec: *mut IoctlVec,
        callback: AsyncCallback,
        context: *mut ffi::c_void,
    ) -> Error;
}
