// use crate::ffi::*;
use super::wpad::*;

#[doc(alias = "KPADVec2D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[doc(alias = "KPADVec3D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe extern "C" {
    #[doc(alias = "KPADInit")]
    #[link_name = "KPADInit"]
    pub unsafe fn init();

    #[doc(alias = "KPADShutdown")]
    #[link_name = "KPADShutdown"]
    pub unsafe fn deinit();

    // #[doc(alias = "KPADReadEx")]
    // #[link_name = "KPADReadEx"]
    // pub unsafe fn poll(
    //     channel: Channel,
    //     buffers: *mut Status,
    //     samples: u32,
    //     error: *mut Error,
    // ) -> i32;
}
