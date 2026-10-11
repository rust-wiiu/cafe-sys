//! GX2 shaders

use super::enums::{
    AttribFormat, AttribIndexType, EndianSwapMode, FetchShaderType, SamplerVarType, ShaderMode,
    ShaderVarType, TessellationMode,
};
use super::r::Buffer;
use crate::ffi;

/// GX2_UNIFORM_BLOCK_INDEX_INVALID
///
/// [UniformVar::block_index] of uniform registers.
#[doc(alias = "GX2_UNIFORM_BLOCK_INDEX_INVALID")]
pub const UNIFORM_BLOCK_INDEX_INVALID: u32 = 0xFFFF_FFFF;

/// GX2_MAX_STREAMOUT_BUFFERS
#[doc(alias = "GX2_MAX_STREAMOUT_BUFFERS")]
pub const MAX_STREAMOUT_BUFFERS: usize = 4;

/// GX2_SQ_SEL
#[doc(alias = "GX2_SQ_SEL", alias = "GX2Component")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Component(pub u32);

impl Component {
    pub const X: Self = Self(0);
    pub const Y: Self = Self(1);
    pub const Z: Self = Self(2);
    pub const W: Self = Self(3);
    pub const R: Self = Self(0);
    pub const G: Self = Self(1);
    pub const B: Self = Self(2);
    pub const A: Self = Self(3);
    pub const ZERO: Self = Self(4);
    pub const ONE: Self = Self(5);
}

/// GX2CompSel
///
/// Source component for each of the x, y, z and w destination components.
#[doc(alias = "GX2CompSel")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentSelection(pub u32);

impl ComponentSelection {
    /// GX2_COMP_MAP
    #[doc(alias = "GX2_COMP_MAP")]
    #[inline]
    pub const fn new(x: Component, y: Component, z: Component, w: Component) -> Self {
        Self(((x.0 & 0xFF) << 24) | ((y.0 & 0xFF) << 16) | ((z.0 & 0xFF) << 8) | (w.0 & 0xFF))
    }
}

/// Registers of a [FetchShader].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FetchShaderRegisters {
    pub sq_pgm_resources_fs: u32,
}

impl FetchShaderRegisters {
    /// # Safety
    ///
    /// `regs` must be valid register values.
    #[inline]
    pub const unsafe fn from_array(regs: [u32; 1]) -> Self {
        unsafe { core::mem::transmute(regs) }
    }
}

/// Registers of a [VertexShader].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexShaderRegisters {
    pub sq_pgm_resources_vs: u32,
    pub vgt_primitiveid_en: u32,
    pub spi_vs_out_config: u32,
    pub num_spi_vs_out_id: u32,
    pub spi_vs_out_id: [u32; 10],
    pub pa_cl_vs_out_cntl: u32,
    pub sq_vtx_semantic_clear: u32,
    pub num_sq_vtx_semantic: u32,
    pub sq_vtx_semantic: [u32; 32],
    pub vgt_strmout_buffer_en: u32,
    pub vgt_vertex_reuse_block_cntl: u32,
    pub vgt_hos_reuse_depth: u32,
}

impl VertexShaderRegisters {
    /// # Safety
    ///
    /// `regs` must be valid register values.
    #[inline]
    pub const unsafe fn from_array(regs: [u32; 52]) -> Self {
        unsafe { core::mem::transmute(regs) }
    }
}

/// Registers of a [GeometryShader].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometryShaderRegisters {
    pub sq_pgm_resources_gs: u32,
    pub vgt_gs_out_prim_type: u32,
    pub vgt_gs_mode: u32,
    pub pa_cl_vs_out_cntl: u32,
    pub sq_pgm_resources_vs: u32,
    pub sq_gs_vert_itemsize: u32,
    pub spi_vs_out_config: u32,
    pub num_spi_vs_out_id: u32,
    pub spi_vs_out_id: [u32; 10],
    pub vgt_strmout_buffer_en: u32,
}

impl GeometryShaderRegisters {
    /// # Safety
    ///
    /// `regs` must be valid register values.
    #[inline]
    pub const unsafe fn from_array(regs: [u32; 19]) -> Self {
        unsafe { core::mem::transmute(regs) }
    }
}

/// Registers of a [PixelShader].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelShaderRegisters {
    pub sq_pgm_resources_ps: u32,
    pub sq_pgm_exports_ps: u32,
    pub spi_ps_in_control_0: u32,
    pub spi_ps_in_control_1: u32,
    pub num_spi_ps_input_cntl: u32,
    pub spi_ps_input_cntls: [u32; 32],
    pub cb_shader_mask: u32,
    pub cb_shader_control: u32,
    pub db_shader_control: u32,
    pub spi_input_z: u32,
}

impl PixelShaderRegisters {
    /// # Safety
    ///
    /// `regs` must be valid register values.
    #[inline]
    pub const unsafe fn from_array(regs: [u32; 41]) -> Self {
        unsafe { core::mem::transmute(regs) }
    }
}

/// GX2UniformBlock
#[doc(alias = "GX2UniformBlock")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniformBlock {
    pub name: *const ffi::c_char,
    /// Hardware location, chosen by the shader compiler.
    pub location: u32,
    /// Size in bytes.
    pub size: u32,
}

/// GX2UniformVar
#[doc(alias = "GX2UniformVar")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniformVar {
    pub name: *const ffi::c_char,
    pub kind: ShaderVarType,
    /// 1 for non-arrays.
    pub count: u32,
    /// Offset in u32s into the uniform block or register file.
    pub offset: u32,
    /// Index into the shader's uniform blocks, or [UNIFORM_BLOCK_INDEX_INVALID] for uniform registers.
    pub block_index: u32,
}

/// GX2UniformInitialValue
#[doc(alias = "GX2UniformInitialValue")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UniformInitialValue {
    pub value: [f32; 4],
    pub offset: u32,
}

/// GX2LoopVar
#[doc(alias = "GX2LoopVar")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopVar {
    pub offset: u32,
    pub value: u32,
}

/// GX2SamplerVar
#[doc(alias = "GX2SamplerVar")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerVar {
    pub name: *const ffi::c_char,
    pub kind: SamplerVarType,
    /// Hardware location of the texture and sampler.
    pub location: u32,
}

/// GX2AttribVar
#[doc(alias = "GX2AttribVar")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribVar {
    pub name: *const ffi::c_char,
    pub kind: ShaderVarType,
    /// 1 for non-arrays.
    pub count: u32,
    /// Location in the hardware semantic table.
    pub location: u32,
}

/// GX2AttribStream
///
/// A vertex attribute as read by the fetch shader.
#[doc(alias = "GX2AttribStream")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribStream {
    /// Destination, see [AttribVar::location].
    pub location: u32,
    /// Source attribute buffer index.
    pub buffer: u32,
    /// Byte offset into the buffer.
    pub offset: u32,
    pub format: AttribFormat,
    pub index_type: AttribIndexType,
    /// Divides the instance index if greater than 1.
    pub alu_divisor: u32,
    pub mask: ComponentSelection,
    pub endian_swap: EndianSwapMode,
}

/// GX2FetchShader
#[doc(alias = "GX2FetchShader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FetchShader {
    pub kind: FetchShaderType,
    pub regs: FetchShaderRegisters,
    pub size: u32,
    /// 256-byte aligned.
    pub program: *mut ffi::c_void,
    pub attrib_count: u32,
    pub num_divisors: u32,
    pub divisors: [u32; 2],
}

/// GX2VertexShader
#[doc(alias = "GX2VertexShader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexShader {
    pub regs: VertexShaderRegisters,
    pub size: u32,
    /// 256-byte aligned. Null if `gx2r_buffer` is used.
    pub program: *mut ffi::c_void,
    pub mode: ShaderMode,
    pub uniform_block_count: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub uniform_var_count: u32,
    pub uniform_vars: *mut UniformVar,
    pub initial_value_count: u32,
    pub initial_values: *mut UniformInitialValue,
    pub loop_var_count: u32,
    pub loop_vars: *mut LoopVar,
    pub sampler_var_count: u32,
    pub sampler_vars: *mut SamplerVar,
    pub attrib_var_count: u32,
    pub attrib_vars: *mut AttribVar,
    /// u32s per vertex in the geometry shader input ring buffer.
    pub ring_item_size: u32,
    pub has_stream_out: ffi::c_bool,
    pub stream_out_stride: [u32; MAX_STREAMOUT_BUFFERS],
    pub gx2r_buffer: Buffer,
}

/// GX2GeometryShader
#[doc(alias = "GX2GeometryShader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometryShader {
    pub regs: GeometryShaderRegisters,
    pub size: u32,
    /// 256-byte aligned. Null if `gx2r_buffer` is used.
    pub program: *mut ffi::c_void,
    /// Size of the copy shader.
    pub vertex_program_size: u32,
    /// Copy shader, 256-byte aligned. Null if `vertex_gx2r_buffer` is used.
    pub vertex_program: *mut ffi::c_void,
    pub mode: ShaderMode,
    pub uniform_block_count: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub uniform_var_count: u32,
    pub uniform_vars: *mut UniformVar,
    pub initial_value_count: u32,
    pub initial_values: *mut UniformInitialValue,
    pub loop_var_count: u32,
    pub loop_vars: *mut LoopVar,
    pub sampler_var_count: u32,
    pub sampler_vars: *mut SamplerVar,
    /// u32s per primitive in the geometry shader output ring buffer.
    pub ring_item_size: u32,
    pub has_stream_out: ffi::c_bool,
    pub stream_out_stride: [u32; MAX_STREAMOUT_BUFFERS],
    pub gx2r_buffer: Buffer,
    pub vertex_gx2r_buffer: Buffer,
}

/// GX2PixelShader
#[doc(alias = "GX2PixelShader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelShader {
    pub regs: PixelShaderRegisters,
    pub size: u32,
    /// 256-byte aligned. Null if `gx2r_buffer` is used.
    pub program: *mut ffi::c_void,
    pub mode: ShaderMode,
    pub uniform_block_count: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub uniform_var_count: u32,
    pub uniform_vars: *mut UniformVar,
    pub initial_value_count: u32,
    pub initial_values: *mut UniformInitialValue,
    pub loop_var_count: u32,
    pub loop_vars: *mut LoopVar,
    pub sampler_var_count: u32,
    pub sampler_vars: *mut SamplerVar,
    pub gx2r_buffer: Buffer,
}

/// GX2ComputeShader
#[doc(alias = "GX2ComputeShader")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputeShader {
    pub regs: [u32; 12],
    pub size: u32,
    /// 256-byte aligned. Null if `gx2r_buffer` is used.
    pub program: *mut ffi::c_void,
    pub uniform_block_count: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub uniform_var_count: u32,
    pub uniform_vars: *mut UniformVar,
    pub initial_value_count: u32,
    pub initial_values: *mut UniformInitialValue,
    pub loop_var_count: u32,
    pub loop_vars: *mut LoopVar,
    pub sampler_var_count: u32,
    pub sampler_vars: *mut SamplerVar,
    /// Work-items per work-group.
    pub layout_size: [u32; 3],
    /// The work-group size exceeds 64.
    pub over64_mode: ffi::c_bool,
    /// Maximum wavefronts per SIMD, 0 for unrestricted.
    pub num_waves_per_simd: u32,
    pub gx2r_buffer: Buffer,
}

/// GX2StreamOutContext
#[doc(alias = "GX2StreamOutContext")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamOutContext(pub ffi::unknown<4>);

/// GX2StreamOutBuffer
#[doc(alias = "GX2StreamOutBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamOutBuffer {
    /// Size in bytes.
    pub size: u32,
    /// Null if `gx2r_buffer` is used.
    pub data: *mut ffi::c_void,
    /// Bytes between vertices, at most 2048.
    pub vertex_stride: u32,
    pub gx2r_buffer: Buffer,
    pub context: *mut StreamOutContext,
}

const _: () = {
    assert!(size_of::<FetchShaderRegisters>() == 4);
    assert!(size_of::<VertexShaderRegisters>() == 52 * 4);
    assert!(size_of::<GeometryShaderRegisters>() == 19 * 4);
    assert!(size_of::<PixelShaderRegisters>() == 41 * 4);
    assert!(size_of::<UniformInitialValue>() == 0x14);
    assert!(size_of::<LoopVar>() == 0x8);
    assert!(size_of::<AttribStream>() == 0x20);
    assert!(size_of::<StreamOutContext>() == 0x4);
};

unsafe extern "C" {
    /// GX2CalcFetchShaderSizeEx
    ///
    /// Size of the program buffer for [init_fetch_shader_ex].
    #[doc(alias = "GX2CalcFetchShaderSizeEx")]
    #[link_name = "GX2CalcFetchShaderSizeEx"]
    pub unsafe fn calc_fetch_shader_size_ex(
        num_attribs: u32,
        kind: FetchShaderType,
        tess_mode: TessellationMode,
    ) -> u32;

    /// GX2InitFetchShaderEx
    ///
    /// `program` must be 256-byte aligned and [calc_fetch_shader_size_ex] bytes.
    #[doc(alias = "GX2InitFetchShaderEx")]
    #[link_name = "GX2InitFetchShaderEx"]
    pub unsafe fn init_fetch_shader_ex(
        shader: *mut FetchShader,
        program: *mut ffi::c_void,
        count: u32,
        attribs: *const AttribStream,
        kind: FetchShaderType,
        tess_mode: TessellationMode,
    );

    /// GX2GetAttribFormatBits
    #[doc(alias = "GX2GetAttribFormatBits")]
    #[link_name = "GX2GetAttribFormatBits"]
    pub unsafe fn get_attrib_format_bits(format: AttribFormat) -> u32;

    /// GX2SetFetchShader
    #[doc(alias = "GX2SetFetchShader")]
    #[link_name = "GX2SetFetchShader"]
    pub unsafe fn set_fetch_shader(shader: *const FetchShader);

    /// GX2SetVertexShader
    #[doc(alias = "GX2SetVertexShader")]
    #[link_name = "GX2SetVertexShader"]
    pub unsafe fn set_vertex_shader(shader: *const VertexShader);

    /// GX2SetGeometryShader
    #[doc(alias = "GX2SetGeometryShader")]
    #[link_name = "GX2SetGeometryShader"]
    pub unsafe fn set_geometry_shader(shader: *const GeometryShader);

    /// GX2SetPixelShader
    #[doc(alias = "GX2SetPixelShader")]
    #[link_name = "GX2SetPixelShader"]
    pub unsafe fn set_pixel_shader(shader: *const PixelShader);

    /// GX2SetComputeShader
    #[doc(alias = "GX2SetComputeShader")]
    #[link_name = "GX2SetComputeShader"]
    pub unsafe fn set_compute_shader(shader: *const ComputeShader);

    /// GX2SetShaderModeEx
    ///
    /// Splits the GPRs and stack entries between the shader stages.
    #[doc(alias = "GX2SetShaderModeEx")]
    #[link_name = "GX2SetShaderModeEx"]
    pub unsafe fn set_shader_mode_ex(
        mode: ShaderMode,
        vs_gprs: u32,
        vs_stack_size: u32,
        gs_gprs: u32,
        gs_stack_size: u32,
        ps_gprs: u32,
        ps_stack_size: u32,
    );

    /// GX2GetVertexShaderGPRs
    #[doc(alias = "GX2GetVertexShaderGPRs")]
    #[link_name = "GX2GetVertexShaderGPRs"]
    pub unsafe fn get_vertex_shader_gprs(shader: *const VertexShader) -> u32;

    /// GX2GetVertexShaderStackEntries
    #[doc(alias = "GX2GetVertexShaderStackEntries")]
    #[link_name = "GX2GetVertexShaderStackEntries"]
    pub unsafe fn get_vertex_shader_stack_entries(shader: *const VertexShader) -> u32;

    /// GX2GetGeometryShaderGPRs
    #[doc(alias = "GX2GetGeometryShaderGPRs")]
    #[link_name = "GX2GetGeometryShaderGPRs"]
    pub unsafe fn get_geometry_shader_gprs(shader: *const GeometryShader) -> u32;

    /// GX2GetGeometryShaderStackEntries
    #[doc(alias = "GX2GetGeometryShaderStackEntries")]
    #[link_name = "GX2GetGeometryShaderStackEntries"]
    pub unsafe fn get_geometry_shader_stack_entries(shader: *const GeometryShader) -> u32;

    /// GX2GetPixelShaderGPRs
    #[doc(alias = "GX2GetPixelShaderGPRs")]
    #[link_name = "GX2GetPixelShaderGPRs"]
    pub unsafe fn get_pixel_shader_gprs(shader: *const PixelShader) -> u32;

    /// GX2GetPixelShaderStackEntries
    #[doc(alias = "GX2GetPixelShaderStackEntries")]
    #[link_name = "GX2GetPixelShaderStackEntries"]
    pub unsafe fn get_pixel_shader_stack_entries(shader: *const PixelShader) -> u32;

    /// GX2SetVertexUniformReg
    ///
    /// `offset` and `count` are in u32s.
    #[doc(alias = "GX2SetVertexUniformReg")]
    #[link_name = "GX2SetVertexUniformReg"]
    pub unsafe fn set_vertex_uniform_reg(offset: u32, count: u32, values: *const ffi::c_void);

    /// GX2SetPixelUniformReg
    ///
    /// `offset` and `count` are in u32s.
    #[doc(alias = "GX2SetPixelUniformReg")]
    #[link_name = "GX2SetPixelUniformReg"]
    pub unsafe fn set_pixel_uniform_reg(offset: u32, count: u32, values: *const ffi::c_void);

    /// GX2SetVertexUniformBlock
    #[doc(alias = "GX2SetVertexUniformBlock")]
    #[link_name = "GX2SetVertexUniformBlock"]
    pub unsafe fn set_vertex_uniform_block(location: u32, size: u32, data: *const ffi::c_void);

    /// GX2SetGeometryUniformBlock
    #[doc(alias = "GX2SetGeometryUniformBlock")]
    #[link_name = "GX2SetGeometryUniformBlock"]
    pub unsafe fn set_geometry_uniform_block(location: u32, size: u32, data: *const ffi::c_void);

    /// GX2SetPixelUniformBlock
    #[doc(alias = "GX2SetPixelUniformBlock")]
    #[link_name = "GX2SetPixelUniformBlock"]
    pub unsafe fn set_pixel_uniform_block(location: u32, size: u32, data: *const ffi::c_void);

    /// GX2SetComputeUniformBlock
    #[doc(alias = "GX2SetComputeUniformBlock")]
    #[link_name = "GX2SetComputeUniformBlock"]
    pub unsafe fn set_compute_uniform_block(location: u32, size: u32, data: *const ffi::c_void);

    /// GX2CalcGeometryShaderInputRingBufferSize
    #[doc(alias = "GX2CalcGeometryShaderInputRingBufferSize")]
    #[link_name = "GX2CalcGeometryShaderInputRingBufferSize"]
    pub unsafe fn calc_geometry_shader_input_ring_buffer_size(ring_item_size: u32) -> u32;

    /// GX2CalcGeometryShaderOutputRingBufferSize
    #[doc(alias = "GX2CalcGeometryShaderOutputRingBufferSize")]
    #[link_name = "GX2CalcGeometryShaderOutputRingBufferSize"]
    pub unsafe fn calc_geometry_shader_output_ring_buffer_size(ring_item_size: u32) -> u32;

    /// GX2SetGeometryShaderInputRingBuffer
    #[doc(alias = "GX2SetGeometryShaderInputRingBuffer")]
    #[link_name = "GX2SetGeometryShaderInputRingBuffer"]
    pub unsafe fn set_geometry_shader_input_ring_buffer(buffer: *mut ffi::c_void, size: u32);

    /// GX2SetGeometryShaderOutputRingBuffer
    #[doc(alias = "GX2SetGeometryShaderOutputRingBuffer")]
    #[link_name = "GX2SetGeometryShaderOutputRingBuffer"]
    pub unsafe fn set_geometry_shader_output_ring_buffer(buffer: *mut ffi::c_void, size: u32);

    /// GX2SetStreamOutBuffer
    #[doc(alias = "GX2SetStreamOutBuffer")]
    #[link_name = "GX2SetStreamOutBuffer"]
    pub unsafe fn set_stream_out_buffer(target: u32, buffer: *const StreamOutBuffer);

    /// GX2SetStreamOutContext
    ///
    /// Restores the context of `buffer`, or resets it if `reset` is true.
    #[doc(alias = "GX2SetStreamOutContext")]
    #[link_name = "GX2SetStreamOutContext"]
    pub unsafe fn set_stream_out_context(
        target: u32,
        buffer: *const StreamOutBuffer,
        reset: ffi::c_bool,
    );

    /// GX2SetStreamOutEnable
    #[doc(alias = "GX2SetStreamOutEnable")]
    #[link_name = "GX2SetStreamOutEnable"]
    pub unsafe fn set_stream_out_enable(enable: ffi::c_bool);

    /// GX2SetShaderExportBuffer
    #[doc(alias = "GX2SetShaderExportBuffer")]
    #[link_name = "GX2SetShaderExportBuffer"]
    pub unsafe fn set_shader_export_buffer(buffer: *mut ffi::c_void, size: u32);
}
