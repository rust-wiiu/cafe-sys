//! NETS2
//!
//! Socket memory, ICMP and NSSL helpers used together with nsysnet.

use crate::{
    ffi,
    nsys::net::{nssl, socket::SockAddr},
};

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
    /// Returns a handle, or -1 on error.
    #[link_name = "icmp_create_handle"]
    pub unsafe fn icmp_create_handle(family: ffi::c_int) -> ffi::c_int;

    /// icmp_close_handle
    #[link_name = "icmp_close_handle"]
    pub unsafe fn icmp_close_handle(handle: ffi::c_int) -> ffi::c_int;

    /// icmp_ping
    #[link_name = "icmp_ping"]
    pub unsafe fn icmp_ping(
        handle: ffi::c_int,
        data: *const ffi::c_void,
        len: ffi::c_int,
        dest: *const SockAddr,
        dest_len: ffi::c_int,
        timeout: ffi::c_uint,
        flags: ffi::c_int,
    ) -> ffi::c_int;

    /// icmp_cancel
    #[link_name = "icmp_cancel"]
    pub unsafe fn icmp_cancel(handle: ffi::c_int) -> ffi::c_int;

    /// icmp_last_code_type
    #[link_name = "icmp_last_code_type"]
    pub unsafe fn icmp_last_code_type(handle: ffi::c_int) -> ffi::c_int;

    /// NSSLAddCRLExternal
    ///
    /// Adds a certificate revocation list to the context.
    #[doc(alias = "NSSLAddCRLExternal")]
    #[link_name = "NSSLAddCRLExternal"]
    pub unsafe fn add_crl_external(
        context: nssl::ContextHandle,
        crl: *const u8,
        crl_len: ffi::c_int,
        crl_type: nssl::CrlType,
    ) -> nssl::Error;

    /// NSSLContextSetFlags
    #[doc(alias = "NSSLContextSetFlags")]
    #[link_name = "NSSLContextSetFlags"]
    pub unsafe fn context_set_flags(
        context: nssl::ContextHandle,
        flags: nssl::ContextFlags,
    ) -> nssl::Error;

    /// NSSLContextClearFlags
    #[doc(alias = "NSSLContextClearFlags")]
    #[link_name = "NSSLContextClearFlags"]
    pub unsafe fn context_clear_flags(
        context: nssl::ContextHandle,
        flags: nssl::ContextFlags,
    ) -> nssl::Error;

    /// NSSLContextGetFlags
    #[doc(alias = "NSSLContextGetFlags")]
    #[link_name = "NSSLContextGetFlags"]
    pub unsafe fn context_get_flags(
        context: nssl::ContextHandle,
        flags: *mut nssl::ContextFlags,
    ) -> nssl::Error;
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
