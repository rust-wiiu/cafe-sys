use crate::ffi::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive, PartialEq, Eq)]
pub enum Font {
    Chinese = 0,
    Korean = 1,
    #[default]
    Latin = 2,
    Taiwanese = 3,
}

unsafe extern "C" {
    /// The data is owned by the system and MUST NOT BE FREED.
    #[doc(alias = "OSGetSharedData")]
    #[link_name = "OSGetSharedData"]
    pub unsafe fn get_shared_font(
        font: Font,
        _unused: u32,
        data: *mut *const u8,
        len: *mut usize,
    ) -> c_bool;
}
