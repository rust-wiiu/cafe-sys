//! NETS2
//!
//! Socket memory, ICMP and NSSL helpers used together with nsysnet.

use crate::{ffi, nsys::net::socket::Address};

// TODO: replace with NSSLContextHandle from nsysnet nssl
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NsslContextHandle(pub i32);

// TODO: replace with NSSLError from nsysnet nssl
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NsslError(pub i32);

// TODO: replace with NSSLCrlType from nsysnet nssl
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NsslCrlType {
    Der = 0,
}

/// SOMemOptRequest
#[doc(alias = "SOMemOptRequest")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOptRequest {
    /// Hands the buffer to nsysnet. Blocks until nsysnet shuts down.
    Init = 1,
    /// Number of bytes used for buffer management.
    GetBytesUsed = 2,
    /// Waits until [MemOptRequest::Init] is done.
    WaitForInit = 3,
    CancelWait = 4,
}

/// SOMemOptFlags
#[doc(alias = "SOMemOptFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemOptFlags(pub i32);

impl MemOptFlags {
    pub const NONE: Self = Self(0);
    /// Splits the memory 50/50 between small and big buffers instead of 80/20.
    pub const BIG_BUFFERS: Self = Self(1 << 0);
}

/// NSSL_CTX_FLAG_*
#[doc(alias = "NSSL_CTX_FLAG")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextFlags(pub u32);

impl ContextFlags {
    pub const CRL_CHECK_LEAF: Self = Self(0x1);
    pub const CRL_CHECK_CHAIN: Self = Self(0x2);
}

impl core::ops::BitOr for ContextFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

unsafe extern "C" {
    /// somemopt
    #[doc(alias("somemopt", "SOMemOpt"))]
    #[link_name = "somemopt"]
    pub unsafe fn mem_opt(
        request: MemOptRequest,
        buf: *mut ffi::c_void,
        size: u32,
        flags: MemOptFlags,
    ) -> ffi::c_int;

    /// icmp_create_handle
    ///
    /// Returns a handle, or a negative value on error.
    #[doc(alias = "SOIcmpCreateHandle")]
    #[link_name = "icmp_create_handle"]
    pub unsafe fn icmp_create_handle(family: ffi::c_int) -> ffi::c_int;

    /// icmp_close_handle
    #[doc(alias = "SOIcmpCloseHandle")]
    #[link_name = "icmp_close_handle"]
    pub unsafe fn icmp_close_handle(handle: ffi::c_int) -> ffi::c_int;

    /// icmp_ping
    #[doc(alias = "SOIcmpPing")]
    #[link_name = "icmp_ping"]
    pub unsafe fn icmp_ping(
        handle: ffi::c_int,
        data: *const ffi::c_void,
        len: ffi::c_int,
        dest: *const Address,
        dest_len: ffi::c_int,
        timeout: ffi::c_uint,
        flags: ffi::c_int,
    ) -> ffi::c_int;

    /// icmp_cancel
    #[doc(alias = "SOIcmpCancel")]
    #[link_name = "icmp_cancel"]
    pub unsafe fn icmp_cancel(handle: ffi::c_int) -> ffi::c_int;

    /// icmp_last_code_type
    #[doc(alias = "SOIcmpLastCodeType")]
    #[link_name = "icmp_last_code_type"]
    pub unsafe fn icmp_last_code_type(handle: ffi::c_int) -> ffi::c_int;

    /// NSSLAddCRLExternal
    ///
    /// Adds a certificate revocation list to the context.
    #[doc(alias = "NSSLAddCRLExternal")]
    #[link_name = "NSSLAddCRLExternal"]
    pub unsafe fn add_crl_external(
        context: NsslContextHandle,
        crl: *const u8,
        crl_len: ffi::c_int,
        crl_type: NsslCrlType,
    ) -> NsslError;

    /// NSSLContextSetFlags
    #[doc(alias = "NSSLContextSetFlags")]
    #[link_name = "NSSLContextSetFlags"]
    pub unsafe fn context_set_flags(context: NsslContextHandle, flags: ContextFlags) -> NsslError;

    /// NSSLContextClearFlags
    #[doc(alias = "NSSLContextClearFlags")]
    #[link_name = "NSSLContextClearFlags"]
    pub unsafe fn context_clear_flags(context: NsslContextHandle, flags: ContextFlags)
    -> NsslError;

    /// NSSLContextGetFlags
    #[doc(alias = "NSSLContextGetFlags")]
    #[link_name = "NSSLContextGetFlags"]
    pub unsafe fn context_get_flags(
        context: NsslContextHandle,
        flags: *mut ContextFlags,
    ) -> NsslError;
}

imports_section!(
    "nn_nets2",
    [
        "NSSLAddCRLExternal",
        "NSSLContextClearFlags",
        "NSSLContextGetFlags",
        "NSSLContextSetFlags",
        "NSSLContextSetMode",
        "icmp_cancel",
        "icmp_close_handle",
        "icmp_create_handle",
        "icmp_last_code_type",
        "icmp_ping",
        "somemopt"
    ],
    []
);
