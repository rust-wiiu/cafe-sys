//! Exit

use crate::ffi;

unsafe extern "C" {
    /// exit
    ///
    /// Runs exit handlers and terminates the process.
    #[link_name = "exit"]
    pub unsafe fn exit(code: ffi::c_int) -> !;

    /// _Exit
    ///
    /// Terminates the process without running exit handlers.
    #[doc(alias = "_Exit")]
    #[link_name = "_Exit"]
    pub unsafe fn immediate_exit(code: ffi::c_int) -> !;
}
