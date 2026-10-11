//! GX2 clears

use super::enums::ClearFlags;
use super::surface::{ColorBuffer, DepthBuffer};

unsafe extern "C" {
    /// GX2ClearColor
    #[doc(alias = "GX2ClearColor")]
    #[link_name = "GX2ClearColor"]
    pub unsafe fn clear_color(
        color_buffer: *mut ColorBuffer,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );

    /// GX2ClearDepthStencilEx
    #[doc(alias = "GX2ClearDepthStencilEx")]
    #[link_name = "GX2ClearDepthStencilEx"]
    pub unsafe fn clear_depth_stencil_ex(
        depth_buffer: *mut DepthBuffer,
        depth: f32,
        stencil: u8,
        flags: ClearFlags,
    );

    /// GX2ClearBuffersEx
    #[doc(alias = "GX2ClearBuffersEx")]
    #[link_name = "GX2ClearBuffersEx"]
    pub unsafe fn clear_buffers_ex(
        color_buffer: *mut ColorBuffer,
        depth_buffer: *mut DepthBuffer,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        depth: f32,
        stencil: u8,
        flags: ClearFlags,
    );

    /// GX2SetClearDepth
    ///
    /// Sets the depth clear value of the buffer, which must match the cleared value for HiZ to work.
    #[doc(alias = "GX2SetClearDepth")]
    #[link_name = "GX2SetClearDepth"]
    pub unsafe fn set_clear_depth(depth_buffer: *mut DepthBuffer, depth: f32);

    /// GX2SetClearStencil
    #[doc(alias = "GX2SetClearStencil")]
    #[link_name = "GX2SetClearStencil"]
    pub unsafe fn set_clear_stencil(depth_buffer: *mut DepthBuffer, stencil: u8);

    /// GX2SetClearDepthStencil
    #[doc(alias = "GX2SetClearDepthStencil")]
    #[link_name = "GX2SetClearDepthStencil"]
    pub unsafe fn set_clear_depth_stencil(depth_buffer: *mut DepthBuffer, depth: f32, stencil: u8);
}
