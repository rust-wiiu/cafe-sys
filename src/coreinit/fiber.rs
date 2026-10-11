//! Fiber
//!
//! Runs a function on another stack of the current thread.

use crate::ffi;

/// OSFiberEntryFn
#[doc(alias = "OSFiberEntryFn")]
pub type FiberEntry = Option<unsafe extern "C" fn() -> ffi::c_int>;

/// OSFiberExEntryFn
#[doc(alias = "OSFiberExEntryFn")]
pub type FiberExEntry =
    Option<unsafe extern "C" fn(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> ffi::c_int>;

unsafe extern "C" {
    /// OSSwitchFiber
    ///
    /// Calls `entry` on the stack `sp` and returns its result.
    #[doc(alias = "OSSwitchFiber")]
    #[link_name = "OSSwitchFiber"]
    pub unsafe fn switch(entry: FiberEntry, sp: *mut ffi::c_void) -> ffi::c_int;

    /// OSSwitchFiberEx
    ///
    /// Same as [switch] with four arguments for `entry`.
    #[doc(alias = "OSSwitchFiberEx")]
    #[link_name = "OSSwitchFiberEx"]
    pub unsafe fn switch_ex(
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        entry: FiberExEntry,
        sp: *mut ffi::c_void,
    ) -> ffi::c_int;
}
