//! GX2R drawing

use super::super::enums::{IndexType, PrimitiveMode};
use super::Buffer;

unsafe extern "C" {
    /// GX2RSetAttributeBuffer
    #[doc(alias = "GX2RSetAttributeBuffer")]
    #[link_name = "GX2RSetAttributeBuffer"]
    pub unsafe fn set_attribute_buffer(buffer: *const Buffer, index: u32, stride: u32, offset: u32);

    /// GX2RDrawIndexed
    #[doc(alias = "GX2RDrawIndexed")]
    #[link_name = "GX2RDrawIndexed"]
    pub unsafe fn draw_indexed(
        mode: PrimitiveMode,
        index_buffer: *const Buffer,
        index_type: IndexType,
        count: u32,
        index_offset: u32,
        vertex_offset: u32,
        num_instances: u32,
    );

    /// GX2RDrawIndexedImmediate
    ///
    /// Same as [draw_indexed], but copies the indices into the command buffer.
    #[doc(alias = "GX2RDrawIndexedImmediate")]
    #[link_name = "GX2RDrawIndexedImmediate"]
    pub unsafe fn draw_indexed_immediate(
        mode: PrimitiveMode,
        index_buffer: *const Buffer,
        index_type: IndexType,
        count: u32,
        index_offset: u32,
        vertex_offset: u32,
        num_instances: u32,
    );
}
