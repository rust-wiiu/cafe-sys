use super::surface;
use crate::{UnsafeInit, ffi::*};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(C, align(0x100))]
#[derive(Debug)]
pub struct Context([u8; 41216]);

impl UnsafeInit for Context {}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum RenderTarget {
    #[default]
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
}

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2Init
    #[link_name = "GX2Init"]
    pub unsafe fn init(attributes: *const u32);

    /// GX2Shutdown
    #[link_name = "GX2Shutdown"]
    pub unsafe fn deinit();

    /// GX2SetupContextStateEx
    #[link_name = "GX2SetupContextStateEx"]
    pub unsafe fn init_context(ctx: *mut Context, enable_profiling: c_bool);

    /// GX2SetContextState
    #[link_name = "GX2SetContextState"]
    pub unsafe fn set_context(ctx: *const Context);

    /// GX2SetColorBuffer
    #[link_name = "GX2SetColorBuffer"]
    pub unsafe fn set_colorbuffer(buffer: *const surface::ColorBuffer, target: RenderTarget);

    /// GX2SetDepthBuffer
    #[link_name = "GX2SetDepthBuffer"]
    pub unsafe fn set_depthbuffer(buffer: *const surface::DepthBuffer);

    /// GX2SetViewport
    #[link_name = "GX2SetViewport"]
    pub unsafe fn set_viewport(
        origin_x: f32,
        origin_y: f32,
        width: f32,
        height: f32,
        near_z: f32,
        far_z: f32,
    );

    /// GX2SetScissor
    #[link_name = "GX2SetScissor"]
    pub unsafe fn set_scissor(origin_x: u32, origin_y: u32, width: u32, height: u32);

    /// GX2SetTVScale
    #[link_name = "GX2SetTVScale"]
    pub unsafe fn set_tv_scale(width: u32, height: u32);

    /// GX2SetDRCScale
    #[link_name = "GX2SetDRCScale"]
    pub unsafe fn set_drc_scale(width: u32, height: u32);

    /// GX2Flush
    #[link_name = "GX2Flush"]
    pub unsafe fn flush();
}
