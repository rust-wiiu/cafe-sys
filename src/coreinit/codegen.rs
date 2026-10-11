//! Code generation
//!
//! The JIT area: memory that can be switched between writable and executable.

use crate::ffi;

/// OSCodegenSecMode
#[doc(alias = "OSCodegenSecMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecMode(pub u32);

impl SecMode {
    /// Readable and writable, not executable.
    pub const READ_WRITE: Self = Self(0);
    /// Readable and executable, not writable.
    pub const READ_EXECUTE: Self = Self(1);
}

unsafe extern "C" {
    /// OSGetCodegenVirtAddrRange
    #[doc(alias = "OSGetCodegenVirtAddrRange")]
    #[link_name = "OSGetCodegenVirtAddrRange"]
    pub unsafe fn get_virt_addr_range(address: *mut *mut ffi::c_void, size: *mut usize);

    /// OSGetCodegenCore
    ///
    /// The core allowed to switch the mode.
    #[doc(alias = "OSGetCodegenCore")]
    #[link_name = "OSGetCodegenCore"]
    pub unsafe fn get_core() -> u32;

    /// OSGetCodegenMode
    #[doc(alias = "OSGetCodegenMode")]
    #[link_name = "OSGetCodegenMode"]
    pub unsafe fn get_mode() -> u32;

    /// OSSwitchSecCodeGenMode
    #[doc(alias = "OSSwitchSecCodeGenMode")]
    #[link_name = "OSSwitchSecCodeGenMode"]
    pub unsafe fn switch_sec_mode(mode: SecMode) -> ffi::c_bool;

    /// OSGetSecCodeGenMode
    #[doc(alias = "OSGetSecCodeGenMode")]
    #[link_name = "OSGetSecCodeGenMode"]
    pub unsafe fn get_sec_mode() -> SecMode;

    /// OSCodegenCopy
    ///
    /// Copies into the codegen area regardless of its mode.
    #[doc(alias = "OSCodegenCopy")]
    #[link_name = "OSCodegenCopy"]
    pub unsafe fn copy(dst: *mut ffi::c_void, src: *const ffi::c_void, size: usize) -> ffi::c_bool;
}
