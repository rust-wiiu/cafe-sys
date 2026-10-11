//! Environment

use crate::ffi;

pub const MAX_VALUE_SIZE: u32 = 256;

unsafe extern "C" {
    /// ENVGetEnvironmentVariable
    #[doc(alias = "ENVGetEnvironmentVariable")]
    #[link_name = "ENVGetEnvironmentVariable"]
    pub unsafe fn get_variable(
        key: *const ffi::c_char,
        value: *mut ffi::c_char,
        value_size: u32,
    ) -> ffi::c_int;
}
