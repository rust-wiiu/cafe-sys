//! GX2 draw calls

use super::enums::{IndexType, PrimitiveMode};
use super::shader::StreamOutBuffer;
use crate::ffi;

/// Number of work groups per dimension.
#[repr(C, align(0x100))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DispatchParams {
    pub num_groups_x: u32,
    pub num_groups_y: u32,
    pub num_groups_z: u32,
    _pad: ffi::unknown<4>,
}

impl DispatchParams {
    #[inline]
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            num_groups_x: x,
            num_groups_y: y,
            num_groups_z: z,
            _pad: ffi::unknown::zero(),
        }
    }
}

unsafe extern "C" {
    /// GX2SetAttribBuffer
    #[doc(alias = "GX2SetAttribBuffer")]
    #[link_name = "GX2SetAttribBuffer"]
    pub unsafe fn set_attrib_buffer(index: u32, size: u32, stride: u32, buffer: *const ffi::c_void);

    /// GX2SetPrimitiveRestartIndex
    #[doc(alias = "GX2SetPrimitiveRestartIndex")]
    #[link_name = "GX2SetPrimitiveRestartIndex"]
    pub unsafe fn set_primitive_restart_index(index: u32);

    /// GX2DrawEx
    #[doc(alias = "GX2DrawEx")]
    #[link_name = "GX2DrawEx"]
    pub unsafe fn draw_ex(mode: PrimitiveMode, count: u32, first_vertex: u32, num_instances: u32);

    /// GX2DrawEx2
    #[doc(alias = "GX2DrawEx2")]
    #[link_name = "GX2DrawEx2"]
    pub unsafe fn draw_ex2(
        mode: PrimitiveMode,
        count: u32,
        first_vertex: u32,
        num_instances: u32,
        base_instance: u32,
    );

    /// GX2DrawIndexedEx
    #[doc(alias = "GX2DrawIndexedEx")]
    #[link_name = "GX2DrawIndexedEx"]
    pub unsafe fn draw_indexed_ex(
        mode: PrimitiveMode,
        count: u32,
        index_type: IndexType,
        indices: *const ffi::c_void,
        base_vertex: u32,
        num_instances: u32,
    );

    /// GX2DrawIndexedEx2
    #[doc(alias = "GX2DrawIndexedEx2")]
    #[link_name = "GX2DrawIndexedEx2"]
    pub unsafe fn draw_indexed_ex2(
        mode: PrimitiveMode,
        count: u32,
        index_type: IndexType,
        indices: *const ffi::c_void,
        base_vertex: u32,
        num_instances: u32,
        base_instance: u32,
    );

    /// GX2DrawIndexedImmediateEx
    ///
    /// Same as [draw_indexed_ex], but copies the indices into the command buffer.
    #[doc(alias = "GX2DrawIndexedImmediateEx")]
    #[link_name = "GX2DrawIndexedImmediateEx"]
    pub unsafe fn draw_indexed_immediate_ex(
        mode: PrimitiveMode,
        count: u32,
        index_type: IndexType,
        indices: *const ffi::c_void,
        base_vertex: u32,
        num_instances: u32,
    );

    /// GX2DrawStreamOut
    ///
    /// Draws the vertices written to `buffer`.
    #[doc(alias = "GX2DrawStreamOut")]
    #[link_name = "GX2DrawStreamOut"]
    pub unsafe fn draw_stream_out(mode: PrimitiveMode, buffer: *const StreamOutBuffer);

    /// GX2SaveStreamOutContext
    #[doc(alias = "GX2SaveStreamOutContext")]
    #[link_name = "GX2SaveStreamOutContext"]
    pub unsafe fn save_stream_out_context(target: u32, buffer: *const StreamOutBuffer);

    /// GX2DispatchCompute
    #[doc(alias = "GX2DispatchCompute")]
    #[link_name = "GX2DispatchCompute"]
    pub unsafe fn dispatch_compute(params: *const DispatchParams);
}
