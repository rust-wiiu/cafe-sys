//! GX2R buffers

use super::super::enums::{FetchShaderType, TessellationMode};
use super::super::shader::{AttribStream, FetchShader, StreamOutBuffer};
use super::{Buffer, ResourceFlags};
use crate::ffi;

unsafe extern "C" {
    /// GX2RCreateBuffer
    ///
    /// Allocates memory for `buffer` from its `flags`, `elem_size` and `elem_count`.
    #[doc(alias = "GX2RCreateBuffer")]
    #[link_name = "GX2RCreateBuffer"]
    pub unsafe fn create(buffer: *mut Buffer) -> ffi::c_bool;

    /// GX2RCreateBufferUserMemory
    #[doc(alias = "GX2RCreateBufferUserMemory")]
    #[link_name = "GX2RCreateBufferUserMemory"]
    pub unsafe fn create_user_memory(
        buffer: *mut Buffer,
        memory: *mut ffi::c_void,
        size: u32,
    ) -> ffi::c_bool;

    /// GX2RDestroyBufferEx
    #[doc(alias = "GX2RDestroyBufferEx")]
    #[link_name = "GX2RDestroyBufferEx"]
    pub unsafe fn destroy_ex(buffer: *mut Buffer, flags: ResourceFlags);

    /// GX2RBufferExists
    #[doc(alias = "GX2RBufferExists")]
    #[link_name = "GX2RBufferExists"]
    pub unsafe fn exists(buffer: *const Buffer) -> ffi::c_bool;

    /// GX2RLockBufferEx
    #[doc(alias = "GX2RLockBufferEx")]
    #[link_name = "GX2RLockBufferEx"]
    pub unsafe fn lock_ex(buffer: *const Buffer, flags: ResourceFlags) -> *mut ffi::c_void;

    /// GX2RLockBufferRegionEx
    #[doc(alias = "GX2RLockBufferRegionEx")]
    #[link_name = "GX2RLockBufferRegionEx"]
    pub unsafe fn lock_region_ex(
        buffer: *const Buffer,
        offset: u32,
        size: u32,
        flags: ResourceFlags,
    ) -> *mut ffi::c_void;

    /// GX2RUnlockBufferEx
    #[doc(alias = "GX2RUnlockBufferEx")]
    #[link_name = "GX2RUnlockBufferEx"]
    pub unsafe fn unlock_ex(buffer: *const Buffer, flags: ResourceFlags);

    /// GX2RInvalidateBuffer
    #[doc(alias = "GX2RInvalidateBuffer")]
    #[link_name = "GX2RInvalidateBuffer"]
    pub unsafe fn invalidate(buffer: *const Buffer, flags: ResourceFlags);

    /// GX2RGetBufferAlignment
    #[doc(alias = "GX2RGetBufferAlignment")]
    #[link_name = "GX2RGetBufferAlignment"]
    pub unsafe fn get_alignment(flags: ResourceFlags) -> u32;

    /// GX2RGetBufferAllocationSize
    #[doc(alias = "GX2RGetBufferAllocationSize")]
    #[link_name = "GX2RGetBufferAllocationSize"]
    pub unsafe fn get_allocation_size(buffer: *const Buffer) -> u32;

    /// GX2RGetBufferGuardBandSize
    #[doc(alias = "GX2RGetBufferGuardBandSize")]
    #[link_name = "GX2RGetBufferGuardBandSize"]
    pub unsafe fn get_guard_band_size(flags: ResourceFlags) -> u32;

    /// GX2RGetBufferName
    #[doc(alias = "GX2RGetBufferName")]
    #[link_name = "GX2RGetBufferName"]
    pub unsafe fn get_name(buffer: *const Buffer) -> *const ffi::c_char;

    /// GX2RSetBufferName
    #[doc(alias = "GX2RSetBufferName")]
    #[link_name = "GX2RSetBufferName"]
    pub unsafe fn set_name(buffer: *mut Buffer, name: *const ffi::c_char);

    /// GX2RSetVertexUniformBlock
    #[doc(alias = "GX2RSetVertexUniformBlock")]
    #[link_name = "GX2RSetVertexUniformBlock"]
    pub unsafe fn set_vertex_uniform_block(buffer: *const Buffer, location: u32, offset: u32);

    /// GX2RSetPixelUniformBlock
    #[doc(alias = "GX2RSetPixelUniformBlock")]
    #[link_name = "GX2RSetPixelUniformBlock"]
    pub unsafe fn set_pixel_uniform_block(buffer: *const Buffer, location: u32, offset: u32);

    /// GX2RSetGeometryUniformBlock
    #[doc(alias = "GX2RSetGeometryUniformBlock")]
    #[link_name = "GX2RSetGeometryUniformBlock"]
    pub unsafe fn set_geometry_uniform_block(buffer: *const Buffer, location: u32, offset: u32);

    /// GX2RSetGeometryShaderInputRingBuffer
    #[doc(alias = "GX2RSetGeometryShaderInputRingBuffer")]
    #[link_name = "GX2RSetGeometryShaderInputRingBuffer"]
    pub unsafe fn set_geometry_shader_input_ring_buffer(buffer: *const Buffer);

    /// GX2RSetGeometryShaderOutputRingBuffer
    #[doc(alias = "GX2RSetGeometryShaderOutputRingBuffer")]
    #[link_name = "GX2RSetGeometryShaderOutputRingBuffer"]
    pub unsafe fn set_geometry_shader_output_ring_buffer(buffer: *const Buffer);

    /// GX2RSetStreamOutBuffer
    #[doc(alias = "GX2RSetStreamOutBuffer")]
    #[link_name = "GX2RSetStreamOutBuffer"]
    pub unsafe fn set_stream_out_buffer(target: u32, buffer: *const StreamOutBuffer);

    /// GX2RInitFetchShader
    ///
    /// Not supported, does nothing.
    #[doc(alias = "GX2RInitFetchShader")]
    #[link_name = "GX2RInitFetchShader"]
    pub unsafe fn init_fetch_shader(
        fetch_shader: *mut FetchShader,
        program: *const Buffer,
        attrib_count: u32,
        attribs: *const AttribStream,
        kind: FetchShaderType,
        tess_mode: TessellationMode,
    );
}
