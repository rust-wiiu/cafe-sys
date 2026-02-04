use super::mem::Buffer;
use crate::{UnsafeInit, ffi::*};
use bitfields::bitfield;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// https://www.x.org/docs/AMD/old/R6xx_3D_Registers.pdf
#[allow(non_camel_case_types)]
pub mod registers {
    use super::*;

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SQ_PGM_RESOURCES_FS {
        /// Number of GPRs required to run this program [0..=127]
        #[bits(8)]
        pub num_gprs: u8,
        #[bits(8)]
        pub stack_size: u8,
        #[bits(5)]
        _pad: u32,
        #[bits(1)]
        pub dx10_clamp: u8,
        #[bits(10)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SQ_PGM_RESOURCES_VS {
        #[bits(8)]
        pub num_gprs: u8,
        #[bits(8)]
        pub stack_size: u8,
        #[bits(5)]
        _pad: u32,
        #[bits(1)]
        pub dx10_clamp: bool,
        #[bits(2)]
        _pad: u32,
        #[bits(3)]
        pub fetch_cache_lines: u8,
        #[bits(1)]
        _pad: u32,
        #[bits(1)]
        pub uncached_first_inst: bool,
        #[bits(3)]
        _pad: u32,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum VGT_PRIMITIVEID_EN {
        Disabled = 0,
        Enabled = 1,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SPI_VS_OUT_CONFIG {
        #[bits(1)]
        pub vs_per_component: bool,
        #[bits(5)]
        pub vs_export_count: u8,
        #[bits(2)]
        _pad: u32,
        #[bits(1)]
        pub vs_exports_fog: bool,
        #[bits(5)]
        pub vs_out_fog_vec_addr: u8,
        #[bits(18)]
        _pad: u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct SPI_VS_OUT_ID {
        pub semantic_0: u8,
        pub semantic_1: u8,
        pub semantic_2: u8,
        pub semantic_3: u8,
    }

    #[bitfield(u32, default = false)]
    #[derive(Clone, Copy)]
    pub struct PA_CL_VS_OUT_CNTL {
        #[bits(1)]
        pub clip_dist_ena_0: bool,
        #[bits(1)]
        pub clip_dist_ena_1: bool,
        #[bits(1)]
        pub clip_dist_ena_2: bool,
        #[bits(1)]
        pub clip_dist_ena_3: bool,
        #[bits(1)]
        pub clip_dist_ena_4: bool,
        #[bits(1)]
        pub clip_dist_ena_5: bool,
        #[bits(1)]
        pub clip_dist_ena_6: bool,
        #[bits(1)]
        pub clip_dist_ena_7: bool,
        #[bits(1)]
        pub cull_dist_ena_0: bool,
        #[bits(1)]
        pub cull_dist_ena_1: bool,
        #[bits(1)]
        pub cull_dist_ena_2: bool,
        #[bits(1)]
        pub cull_dist_ena_3: bool,
        #[bits(1)]
        pub cull_dist_ena_4: bool,
        #[bits(1)]
        pub cull_dist_ena_5: bool,
        #[bits(1)]
        pub cull_dist_ena_6: bool,
        #[bits(1)]
        pub cull_dist_ena_7: bool,
        #[bits(1)]
        pub use_vtx_point_size: bool,
        #[bits(1)]
        pub use_vtx_edge_flag: bool,
        #[bits(1)]
        use_vtx_render_target_indx: bool,
        #[bits(13)]
        _pad: u32,
    }

    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct SQ_VTX_SEMANTIC_CLEAR(u32);

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SQ_VTX_SEMANTIC {
        #[bits(8)]
        pub semantic_id: u8,
        #[bits(24)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct VGT_STRMOUT_BUFFER_EN {
        #[bits(1)]
        pub buffer_0_en: bool,
        #[bits(1)]
        pub buffer_1_en: bool,
        #[bits(1)]
        pub buffer_2_en: bool,
        #[bits(1)]
        pub buffer_3_en: bool,
        #[bits(28)]
        _pad: u32,
    }

    #[bitfield(u32, default = false)]
    #[derive(Clone, Copy)]
    pub struct VGT_VERTEX_REUSE_BLOCK_CNTL {
        #[bits(8)]
        pub vtx_reuse_depth: u8,
        #[bits(24)]
        _pad: u32,
    }

    #[bitfield(u32, default = false)]
    #[derive(Clone, Copy)]
    pub struct VGT_HOS_REUSE_DEPTH {
        #[bits(8)]
        pub reuse_depth: u8,
        #[bits(24)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SQ_PGM_RESOURCES_PS {
        #[bits(8)]
        pub num_gprs: u8,
        #[bits(8)]
        pub stack_size: u8,
        #[bits(5)]
        _pad: u32,
        #[bits(1)]
        pub dx10_clamp: bool,
        #[bits(2)]
        _pad: u32,
        #[bits(3)]
        pub fetch_cache_lines: u8,
        #[bits(1)]
        _pad: u32,
        #[bits(1)]
        pub uncached_first_inst: bool,
        #[bits(2)]
        _pad: u32,
        #[bits(1)]
        pub clamp_consts: bool,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SQ_PGM_EXPORTS_PS {
        #[bits(4)]
        pub export_mode: u8,
        #[bits(28)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SPI_PS_IN_CONTROL_0 {
        #[bits(6)]
        pub num_interp: u8,
        #[bits(2)]
        _pad: u32,
        #[bits(1)]
        pub position_ena: bool,
        #[bits(1)]
        pub position_centroid: bool,
        #[bits(5)]
        pub position_addr: u8,
        #[bits(4)]
        pub param_gen: u8,
        #[bits(7)]
        pub param_gen_addr: u8,
        #[bits(2)]
        pub baryc_sample_cntl: u8,
        #[bits(1)]
        pub persp_gradient_ena: bool,
        #[bits(1)]
        pub linear_gradient_ena: bool,
        #[bits(1)]
        pub position_sample: bool,
        #[bits(1)]
        pub baryc_at_sample_ena: bool,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SPI_PS_IN_CONTROL_1 {
        #[bits(1)]
        pub gen_index_pix: bool,
        #[bits(7)]
        pub gen_index_pix_addr: u8,
        #[bits(1)]
        pub front_face_ena: bool,
        #[bits(2)]
        pub front_face_chan: u8,
        #[bits(1)]
        pub front_face_all_bits: bool,
        #[bits(5)]
        pub front_face_addr: u8,
        #[bits(7)]
        pub fog_addr: u8,
        #[bits(1)]
        pub fixed_pt_position_ena: bool,
        #[bits(5)]
        pub fixed_pt_position_addr: u8,
        #[bits(2)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SPI_PS_INPUT_CNTL {
        #[bits(8)]
        pub semantic: u8,
        #[bits(2)]
        pub default_val: u8,
        #[bits(1)]
        pub flat_shade: bool,
        #[bits(1)]
        pub sel_centroid: bool,
        #[bits(1)]
        pub sel_linear: bool,
        #[bits(4)]
        pub cyl_wrap: u8,
        #[bits(1)]
        pub pt_sprite_tex: bool,
        #[bits(1)]
        pub sel_sample: bool,
        #[bits(13)]
        _pad: u32,
    }

    #[bitfield(u32, default = false)]
    #[derive(Clone, Copy)]
    pub struct CB_SHADER_MASK {
        #[bits(4)]
        pub output0_enable: u8,
        #[bits(4)]
        pub output1_enable: u8,
        #[bits(4)]
        pub output2_enable: u8,
        #[bits(4)]
        pub output3_enable: u8,
        #[bits(4)]
        pub output4_enable: u8,
        #[bits(4)]
        pub output5_enable: u8,
        #[bits(4)]
        pub output6_enable: u8,
        #[bits(4)]
        pub output7_enable: u8,
    }

    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct CB_SHADER_CONTROL(pub u32);

    #[bitfield(u32, default = false)]
    #[derive(Clone, Copy)]
    pub struct DB_SHADER_CONTROL {
        #[bits(1)]
        pub z_export_enable: bool,
        #[bits(1)]
        pub stencil_ref_export_enable: bool,
        #[bits(2)]
        pub z_order: u8,
        #[bits(1)]
        pub kill_enable: bool,
        #[bits(1)]
        pub coverage_to_mask_enable: bool,
        #[bits(1)]
        pub mask_export_enable: bool,
        #[bits(1)]
        pub dual_export_enable: bool,
        #[bits(1)]
        pub exec_on_hier_fail: bool,
        #[bits(1)]
        pub exec_on_noop: bool,
        #[bits(22)]
        _pad: u32,
    }

    #[bitfield(u32)]
    #[derive(Clone, Copy)]
    pub struct SPI_INPUT_Z {
        #[bits(1)]
        pub provide_z_to_spi: bool,
        #[bits(31)]
        _pad: u32,
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FetchShaderRegisters {
    pub sq_pgm_resources_fs: registers::SQ_PGM_RESOURCES_FS,
}

#[repr(C)]
#[derive(Debug)]
pub struct VertexShaderRegisters {
    pub sq_pgm_resources_vs: registers::SQ_PGM_RESOURCES_VS,
    pub vgt_primitiveid_en: registers::VGT_PRIMITIVEID_EN,
    pub spi_vs_out_config: registers::SPI_VS_OUT_CONFIG,
    pub num_spi_vs_out_id: u32,
    pub spi_vs_out_id: [registers::SPI_VS_OUT_ID; 10],
    pub pa_cl_vs_out_cntl: registers::PA_CL_VS_OUT_CNTL,
    pub sq_vtx_semantic_clear: registers::SQ_VTX_SEMANTIC_CLEAR,
    pub num_sq_vtx_semantic: u32,
    pub sq_vtx_semantic: [registers::SQ_VTX_SEMANTIC; 32],
    pub vgt_strmout_buffer_en: registers::VGT_STRMOUT_BUFFER_EN,
    pub vgt_vertex_reuse_block_cntl: registers::VGT_VERTEX_REUSE_BLOCK_CNTL,
    pub vgt_hos_reuse_depth: registers::VGT_HOS_REUSE_DEPTH,
}

#[repr(C)]
#[derive(Debug)]
pub struct PixelShaderRegisters {
    pub sq_pgm_resources_ps: registers::SQ_PGM_RESOURCES_PS,
    pub sq_pgm_exports_ps: registers::SQ_PGM_EXPORTS_PS,
    pub spi_ps_in_control_0: registers::SPI_PS_IN_CONTROL_0,
    pub spi_ps_in_control_1: registers::SPI_PS_IN_CONTROL_1,
    pub num_spi_ps_input_cntl: u32,
    pub spi_ps_input_cntls: [registers::SPI_PS_INPUT_CNTL; 32],
    pub cb_shader_mask: registers::CB_SHADER_MASK,
    pub cb_shader_control: registers::CB_SHADER_CONTROL,
    pub db_shader_control: registers::DB_SHADER_CONTROL,
    pub spi_input_z: registers::SPI_INPUT_Z,
}

/// GX2FetchShader
#[repr(C, align(64))]
#[derive(Debug)]
pub struct FetchShader {
    pub r#type: FetchShaderType,
    pub regs: FetchShaderRegisters,
    pub size: u32,
    pub program: *mut c_void,
    pub num_attribs: u32,
    _num_divisor: u32,
    _divisiors: [u32; 2],
}

impl UnsafeInit for FetchShader {}

/// GX2ShaderMode
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ShaderMode {
    UniformRegister = 0,
    UniformBlock = 1,
    Geometry = 2,
    Compute = 3,
}

/// GX2PixelShader
#[repr(C, align(64))]
#[derive(Debug)]
#[doc(alias = "FragmentShader")]
pub struct PixelShader {
    pub regs: PixelShaderRegisters,
    pub shader_size: u32,
    pub shader_ptr: *mut c_void,
    pub shader_mode: ShaderMode,
    pub num_uniform_blocks: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub num_uniforms: u32,
    pub uniform_vars: *mut UniformVar,
    pub num_initial_values: u32,
    pub initial_values: *mut UniformInitialValue,
    pub num_loops: u32,
    pub loop_vars: *mut c_void,
    pub num_samplers: u32,
    pub sampler_vars: *mut SamplerVar,
    pub program: Buffer,
}

impl UnsafeInit for PixelShader {}

/// GX2VertexShader
#[repr(C, align(64))]
#[derive(Debug)]
pub struct VertexShader {
    pub regs: VertexShaderRegisters,
    pub shader_size: u32,
    pub shader_ptr: *mut c_void,
    pub shader_mode: ShaderMode,
    pub num_uniform_blocks: u32,
    pub uniform_blocks: *mut UniformBlock,
    pub num_uniforms: u32,
    pub uniform_vars: *mut UniformVar,
    pub num_initial_values: u32,
    pub initial_values: *mut UniformInitialValue,
    pub num_loops: u32,
    pub loop_vars: *mut LoopVar,
    pub num_samplers: u32,
    pub sampler_vars: *mut SamplerVar,
    pub num_attribs: u32,
    pub attrib_vars: *mut AttribVar,
    pub ring_itemsize: u32,
    pub has_stream_output: c_bool,
    pub stream_out_vertex_stride: [u32; 4],
    pub program: Buffer,
}

impl UnsafeInit for VertexShader {}

/// GX2UniformBlock
#[repr(C)]
#[derive(Debug)]
pub struct UniformBlock {
    pub name: *const c_char,
    pub location: u32,
    pub size: u32,
}

impl UnsafeInit for UniformBlock {}

/// GX2UniformVar
#[repr(C)]
#[derive(Debug)]
pub struct UniformVar {
    pub name: *const c_char,
    pub r#type: VarType,
    pub array_count: u32,
    pub offset: u32,
    pub block_index: u32,
}

impl UnsafeInit for UniformVar {}

/// GX2VarType
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum VarType {
    Void = 0,
    Bool = 1,
    Int = 2,
    Uint = 3,
    Float = 4,
    Double = 5,
    Double2 = 6,
    Double3 = 7,
    Double4 = 8,
    Float2 = 9,
    Float3 = 10,
    Float4 = 11,
    Bool2 = 12,
    Bool3 = 13,
    Bool4 = 14,
    Int2 = 15,
    Int3 = 16,
    Int4 = 17,
    Uint2 = 18,
    Uint3 = 19,
    Uint4 = 20,
    Float2x2 = 21,
    Float2x3 = 22,
    Float2x4 = 23,
    Float3x2 = 24,
    Float3x3 = 25,
    Float3x4 = 26,
    Float4x2 = 27,
    Float4x3 = 28,
    Float4x4 = 29,
    Double2x2 = 30,
    Double2x3 = 31,
    Double2x4 = 32,
    Double3x2 = 33,
    Double3x3 = 34,
    Double3x4 = 35,
    Double4x2 = 36,
    Double4x3 = 37,
    Double4x4 = 38,
}

/// GX2UniformInitialValue
#[repr(C)]
#[derive(Debug)]
pub struct UniformInitialValue {
    pub value: [f32; 4],
    pub offset: u32,
}

impl UnsafeInit for UniformInitialValue {}

/// GX2LoopVar
#[repr(C)]
#[derive(Debug)]
pub struct LoopVar {
    pub offset: u32,
    pub value: u32,
}

/// GX2SamplerVar
#[repr(C)]
#[derive(Debug)]
pub struct SamplerVar {
    pub name: *const c_char,
    pub r#type: SamplerType,
    pub location: u32,
}

impl UnsafeInit for SamplerVar {}

/// GX2SamplerType
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum SamplerType {
    D1 = 0,
    D2 = 1,
    D2Rect = 2,
    D3 = 3,
    Cube = 4,
    D1Shadow = 5,
    D2Shadow = 6,
    D2RectShadow = 7,
    CubeShadow = 8,
    D1Array = 9,
    D2Array = 10,
    D1ArrayShadow = 11,
    D2ArrayShadow = 12,
    CubeArray = 13,
    CubeArrayShadow = 14,
    Buffer = 15,
    // RESERVED_1 = 16,
    D2Ms = 17,
    D2MsArray = 18,
    Int1d = 19,
    Int2d = 20,
    Int2dRect = 21,
    Int3d = 22,
    IntCube = 23,
    Int1dArray = 24,
    Int2dArray = 25,
    IntCubeArray = 26,
    IntBuffer = 27,
    // RESERVED_2 = 28,
    Int2dMs = 29,
    Int2dMsArray = 30,
    UnsignedInt1d = 31,
    UnsignedInt2d = 32,
    UnsignedInt2dRect = 33,
    UnsignedInt3d = 34,
    UnsignedIntCube = 35,
    UnsignedInt1dArray = 36,
    UnsignedInt2dArray = 37,
    UnsignedIntCubeArray = 38,
    UnsignedIntBuffer = 39,
    // RESERVED_3 = 40,
    UnsignedInt2dMs = 41,
    UnsignedInt2dMsArray = 42,
}

/// GX2AttribVar
#[repr(C)]
#[derive(Debug)]
pub struct AttribVar {
    pub name: *const c_char,
    pub r#type: VarType,
    pub array_count: u32,
    pub location: u32,
}

impl UnsafeInit for AttribVar {}

/// GX2AttribStream
#[repr(C)]
#[derive(Debug)]
pub struct AttribStream {
    pub location: u32,
    pub buffer: u32,
    pub offset: u32,
    pub format: AttribFormat,
    pub index_type: AttribIndexType,
    pub alu_divisor: u32,
    pub mask: ComponentSelection,
    pub endian_swap: EndianSwapMode,
}

impl UnsafeInit for AttribStream {}

/// GX2AttribFormat
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum AttribFormat {
    // 8 bits (8 x 1)
    Unorm8 = 0x000,
    Uint8 = 0x100,
    Snorm8 = 0x200,
    Sint8 = 0x300,
    UintToFloat8 = 0x800,
    SintToFloat8 = 0xa00,

    // 8 bits (4 x 2)
    Unorm4_4 = 0x001,

    // 16 bits (16 x 1)
    Unorm16 = 0x002,
    Uint16 = 0x102,
    Snorm16 = 0x202,
    Sint16 = 0x302,
    Float16 = 0x803,
    UintToFloat16 = 0x802,
    SintToFloat16 = 0xa02,

    // 16 bits (8 x 2)
    Unorm8_8 = 0x004,
    Uint8_8 = 0x104,
    Snorm8_8 = 0x204,
    Sint8_8 = 0x304,
    UintToFloat8_8 = 0x804,
    SintToFloat8_8 = 0xa04,

    // 32 bits (32 x 1)
    Uint32 = 0x105,
    Sint32 = 0x305,
    Float32 = 0x806,

    // 32 bits (16 x 2)
    Unorm16_16 = 0x007,
    Uint16_16 = 0x107,
    Snorm16_16 = 0x207,
    Sint16_16 = 0x307,
    Float16_16 = 0x808,
    UintToFloat16_16 = 0x807,
    SintToFloat16_16 = 0xa07,

    // 32 bits (10/11 x 3)
    Float10_11_11 = 0x809,

    // 32 bits (8 x 4)
    Unorm8_8_8_8 = 0x00a,
    Uint8_8_8_8 = 0x10a,
    Snorm8_8_8_8 = 0x20a,
    Sint8_8_8_8 = 0x30a,
    UintToFloat8_8_8_8 = 0x80a,
    SintToFloat8_8_8_8 = 0xa0a,

    // 32 bits (10 x 3 + 2)
    Unorm10_10_10_2 = 0x00b,
    Uint10_10_10_2 = 0x10b,
    Snorm10_10_10_2 = 0x20b, // "2" part is UNORM
    Sint10_10_10_2 = 0x30b,

    // 64 bits (32 x 2)
    Uint32_32 = 0x10c,
    Sint32_32 = 0x30c,
    Float32_32 = 0x80d,

    // 64 bits (16 x 4)
    Unorm16_16_16_16 = 0x00e,
    Uint16_16_16_16 = 0x10e,
    Snorm16_16_16_16 = 0x20e,
    Sint16_16_16_16 = 0x30e,
    Float16_16_16_16 = 0x80f,
    UintToFloat16_16_16_16 = 0x80e,
    SintToFloat16_16_16_16 = 0xa0e,

    // 96 bits (32 x 3)
    Uint32_32_32 = 0x110,
    Sint32_32_32 = 0x310,
    Float32_32_32 = 0x811,

    // 128 bits (32 x 4)
    Uint32_32_32_32 = 0x112,
    Sint32_32_32_32 = 0x312,
    Float32_32_32_32 = 0x813,
}

/// GX2AttribIndexType
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum AttribIndexType {
    /// Per-Vertex Index
    PerVertex = 0,
    /// Per-Instance Index
    PerInstance = 1,
}

/// GX2EndianSwapMode
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum EndianSwapMode {
    None = 0,
    Swap8in16 = 1,
    Swap8in32 = 2,
    #[default]
    Default = 3,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Component(u32);

impl Component {
    pub const X: Self = Self(0);
    pub const R: Self = Self(0);
    pub const Y: Self = Self(1);
    pub const G: Self = Self(1);
    pub const Z: Self = Self(2);
    pub const B: Self = Self(2);
    pub const W: Self = Self(3);
    pub const A: Self = Self(3);
    /// Constant 0
    pub const C0: Self = Self(4);
    /// Constant 1
    pub const C1: Self = Self(5);
}

/// GX2CompSel
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ComponentSelection(u32);

impl ComponentSelection {
    pub const fn new(x: Component, y: Component, z: Component, w: Component) -> Self {
        Self(
            (((x.0) & 0xff) << 24)
                | (((y.0) & 0xff) << 16)
                | (((z.0) & 0xff) << 8)
                | ((w.0) & 0xff),
        )
    }

    pub const fn none() -> Self {
        Self::new(Component::C0, Component::C0, Component::C0, Component::C0)
    }

    pub const fn x001() -> Self {
        Self::new(Component::X, Component::C0, Component::C0, Component::C1)
    }

    pub const fn xy01() -> Self {
        Self::new(Component::X, Component::Y, Component::C0, Component::C1)
    }

    pub const fn xyz1() -> Self {
        Self::new(Component::X, Component::Y, Component::Z, Component::C1)
    }

    pub const fn xyzw() -> Self {
        Self::new(Component::X, Component::Y, Component::Z, Component::W)
    }

    pub const fn xxxx() -> Self {
        Self::new(Component::X, Component::X, Component::X, Component::X)
    }

    pub const fn yyyy() -> Self {
        Self::new(Component::Y, Component::Y, Component::Y, Component::Y)
    }

    pub const fn zzzz() -> Self {
        Self::new(Component::Z, Component::Z, Component::Z, Component::Z)
    }

    pub const fn wwww() -> Self {
        Self::new(Component::W, Component::W, Component::W, Component::W)
    }

    pub const fn wzyx() -> Self {
        Self::new(Component::W, Component::Z, Component::Y, Component::X)
    }

    pub const fn wxyz() -> Self {
        Self::new(Component::W, Component::X, Component::Y, Component::Z)
    }
}

impl From<AttribFormat> for ComponentSelection {
    fn from(value: AttribFormat) -> Self {
        match value {
            AttribFormat::Unorm8
            | AttribFormat::Uint8
            | AttribFormat::Snorm8
            | AttribFormat::Sint8
            | AttribFormat::Float32 => ComponentSelection::x001(),
            AttribFormat::Unorm8_8
            | AttribFormat::Uint8_8
            | AttribFormat::Snorm8_8
            | AttribFormat::Sint8_8
            | AttribFormat::Float32_32 => ComponentSelection::xy01(),
            AttribFormat::Float32_32_32 => ComponentSelection::xyz1(),
            AttribFormat::Unorm8_8_8_8
            | AttribFormat::Uint8_8_8_8
            | AttribFormat::Snorm8_8_8_8
            | AttribFormat::Sint8_8_8_8
            | AttribFormat::Float32_32_32_32 => ComponentSelection::xyzw(),
            _ => {
                ComponentSelection::new(Component::C0, Component::C0, Component::C0, Component::C1)
            }
        }
    }
}

/// GX2FetchShaderType
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum FetchShaderType {
    None,
    Lines,
    Triangles,
    Quads,
}

/// GX2TessellationMode
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum TessellationMode {
    Discrete,
    Countinuous,
    Adaptive,
}

/// GX2PrimitiveType
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum PrimitiveMode {
    Points = 0x01,
    Lines = 0x02,
    LineStrip = 0x03,
    Triangles = 0x04,
    TriangleFan = 0x05,
    TriangleStrip = 0x06,
    LinesAdjacency = 0x0a,
    LineStripAdjacency = 0x0b,
    TrianglesAdjacency = 0x0c,
    TriangleStripAdjacency = 0x0d,
    Rects = 0x11,
    LineLoop = 0x12,
    Quads = 0x13,
    QuadStrip = 0x14,
    TessellateLines = 0x82,
    TessellateLineStrip = 0x83,
    TessellateTriangles = 0x84,
    TessellateTriangleStrip = 0x86,
    TessellateQuads = 0x93,
    TessellateQuadStrip = 0x94,
}

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2CalcFetchShaderSizeEx
    #[link_name = "GX2CalcFetchShaderSizeEx"]
    pub unsafe fn fetch_shader_program_size(
        num_attrib: u32,
        fs_type: FetchShaderType,
        tess_mode: TessellationMode,
    ) -> u32;

    /// GX2InitFetchShaderEx
    #[link_name = "GX2InitFetchShaderEx"]
    pub unsafe fn init_fetch_shader_ex(
        fs: *mut FetchShader,
        fs_buffer: *mut c_void,
        count: u32,
        attibs: *const AttribStream,
        fs_type: FetchShaderType,
        tess_mode: TessellationMode,
    );

    /// GX2SetFetchShader
    #[link_name = "GX2SetFetchShader"]
    pub unsafe fn set_fetch_shader(fs: *const FetchShader);

    /// GX2SetVertexShader
    #[link_name = "GX2SetVertexShader"]
    pub unsafe fn set_vertex_shader(fs: *const VertexShader);

    /// GX2SetPixelShader
    #[link_name = "GX2SetPixelShader"]
    pub unsafe fn set_pixel_shader(fs: *const PixelShader);

    /// GX2RSetAttributeBuffer
    #[link_name = "GX2RSetAttributeBuffer"]
    pub unsafe fn set_attribute_buffer(buffer: *const Buffer, slot: u32, stride: u32, offset: u32);

    /// GX2DrawEx
    #[link_name = "GX2DrawEx"]
    pub unsafe fn draw_ex(mode: PrimitiveMode, count: u32, first_vertex: u32, instances: u32);
}
