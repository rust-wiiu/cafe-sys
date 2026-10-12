//! GX2 enums

/// GX2_COMMAND_BUFFER_ALIGNMENT
pub const COMMAND_BUFFER_ALIGNMENT: u32 = 0x40;
/// GX2_CONTEXT_STATE_ALIGNMENT
pub const CONTEXT_STATE_ALIGNMENT: u32 = 0x100;
/// GX2_SCAN_BUFFER_ALIGNMENT
pub const SCAN_BUFFER_ALIGNMENT: u32 = 0x1000;
/// GX2_SHADER_PROGRAM_ALIGNMENT
pub const SHADER_PROGRAM_ALIGNMENT: u32 = 0x100;
/// GX2_VERTEX_BUFFER_ALIGNMENT
pub const VERTEX_BUFFER_ALIGNMENT: u32 = 0x40;
/// GX2_INDEX_BUFFER_ALIGNMENT
pub const INDEX_BUFFER_ALIGNMENT: u32 = 0x20;
/// GX2_UNIFORM_BLOCK_ALIGNMENT
pub const UNIFORM_BLOCK_ALIGNMENT: u32 = 0x100;
/// GX2_COMMAND_BUFFER_SIZE
pub const COMMAND_BUFFER_SIZE: u32 = 0x400000;

/// GX2AAMode
#[doc(alias = "GX2AAMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AaMode(pub u32);

impl AaMode {
    pub const MODE1X: Self = Self(0);
    pub const MODE2X: Self = Self(1);
    pub const MODE4X: Self = Self(2);
    pub const MODE8X: Self = Self(3);
}

/// GX2AlphaToMaskMode
#[doc(alias = "GX2AlphaToMaskMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlphaToMaskMode(pub u32);

impl AlphaToMaskMode {
    pub const NON_DITHERED: Self = Self(0);
    pub const DITHER_0: Self = Self(1);
    pub const DITHER_90: Self = Self(2);
    pub const DITHER_180: Self = Self(3);
    pub const DITHER_270: Self = Self(4);
}

/// GX2AspectRatio
#[doc(alias = "GX2AspectRatio")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AspectRatio(pub u32);

impl AspectRatio {
    pub const RATIO_4_3: Self = Self(0);
    pub const RATIO_16_9: Self = Self(1);
}

/// GX2AttribFormat
#[doc(alias = "GX2AttribFormat")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribFormat(pub u32);

impl AttribFormat {
    pub const TYPE_8: Self = Self(0);
    pub const TYPE_4_4: Self = Self(1);
    pub const TYPE_16: Self = Self(2);
    pub const TYPE_16_FLOAT: Self = Self(3);
    pub const TYPE_8_8: Self = Self(4);
    pub const TYPE_32: Self = Self(5);
    pub const TYPE_32_FLOAT: Self = Self(6);
    pub const TYPE_16_16: Self = Self(7);
    pub const TYPE_16_16_FLOAT: Self = Self(8);
    pub const TYPE_10_11_11_FLOAT: Self = Self(9);
    pub const TYPE_8_8_8_8: Self = Self(0xa);
    pub const TYPE_10_10_10_2: Self = Self(0xb);
    pub const TYPE_32_32: Self = Self(0xc);
    pub const TYPE_32_32_FLOAT: Self = Self(0xd);
    pub const TYPE_16_16_16_16: Self = Self(0xe);
    pub const TYPE_16_16_16_16_FLOAT: Self = Self(0xf);
    pub const TYPE_32_32_32: Self = Self(0x10);
    pub const TYPE_32_32_32_FLOAT: Self = Self(0x11);
    pub const TYPE_32_32_32_32: Self = Self(0x12);
    pub const TYPE_32_32_32_32_FLOAT: Self = Self(0x13);
    pub const FLAG_INTEGER: Self = Self(0x100);
    pub const FLAG_SIGNED: Self = Self(0x200);
    pub const FLAG_DEGAMMA: Self = Self(0x400);
    pub const FLAG_SCALED: Self = Self(0x800);
    pub const FORMAT_UNORM_8: Self = Self(0);
    pub const FORMAT_UNORM_8_8: Self = Self(4);
    pub const FORMAT_UNORM_8_8_8_8: Self = Self(0xa);
    pub const FORMAT_UINT_8: Self = Self(0x100);
    pub const FORMAT_UINT_8_8: Self = Self(0x104);
    pub const FORMAT_UINT_8_8_8_8: Self = Self(0x10a);
    pub const FORMAT_SNORM_8: Self = Self(0x200);
    pub const FORMAT_SNORM_8_8: Self = Self(0x204);
    pub const FORMAT_SNORM_8_8_8_8: Self = Self(0x20a);
    pub const FORMAT_SINT_8: Self = Self(0x300);
    pub const FORMAT_SINT_8_8: Self = Self(0x304);
    pub const FORMAT_SINT_8_8_8_8: Self = Self(0x30a);
    pub const FORMAT_FLOAT_32: Self = Self(0x806);
    pub const FORMAT_FLOAT_32_32: Self = Self(0x80d);
    pub const FORMAT_FLOAT_32_32_32: Self = Self(0x811);
    pub const FORMAT_FLOAT_32_32_32_32: Self = Self(0x813);
}

impl core::ops::BitOr for AttribFormat {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2AttribIndexType
#[doc(alias = "GX2AttribIndexType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribIndexType(pub u32);

impl AttribIndexType {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}

/// GX2BlendMode
#[doc(alias = "GX2BlendMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendMode(pub u32);

impl BlendMode {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const INV_SRC_COLOR: Self = Self(3);
    pub const SRC_ALPHA: Self = Self(4);
    pub const INV_SRC_ALPHA: Self = Self(5);
    pub const DST_ALPHA: Self = Self(6);
    pub const INV_DST_ALPHA: Self = Self(7);
    pub const DST_COLOR: Self = Self(8);
    pub const INV_DST_COLOR: Self = Self(9);
    pub const SRC_ALPHA_SAT: Self = Self(0xa);
    pub const BOTH_SRC_ALPHA: Self = Self(0xb);
    pub const BOTH_INV_SRC_ALPHA: Self = Self(0xc);
    pub const BLEND_FACTOR: Self = Self(0xd);
    pub const INV_BLEND_FACTOR: Self = Self(0xe);
    pub const SRC1_COLOR: Self = Self(0xf);
    pub const INV_SRC1_COLOR: Self = Self(0x10);
    pub const SRC1_ALPHA: Self = Self(0x11);
    pub const INV_SRC1_ALPHA: Self = Self(0x12);
    pub const CONSTANT_ALPHA: Self = Self(0x13);
    pub const INV_CONSTANT_ALPHA: Self = Self(0x14);
}

/// GX2BlendCombineMode
#[doc(alias = "GX2BlendCombineMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendCombineMode(pub u32);

impl BlendCombineMode {
    pub const ADD: Self = Self(0);
    pub const SUB: Self = Self(1);
    pub const MIN: Self = Self(2);
    pub const MAX: Self = Self(3);
    pub const REV_SUB: Self = Self(4);
}

/// GX2BufferingMode
#[doc(alias = "GX2BufferingMode")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferingMode {
    Single = 1,
    Double = 2,
    Triple = 3,
}

/// GX2ChannelMask
#[doc(alias = "GX2ChannelMask")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelMask(pub u32);

impl ChannelMask {
    pub const R: Self = Self(1);
    pub const G: Self = Self(2);
    pub const RG: Self = Self(3);
    pub const B: Self = Self(4);
    pub const RB: Self = Self(5);
    pub const GB: Self = Self(6);
    pub const RGB: Self = Self(7);
    pub const A: Self = Self(8);
    pub const RA: Self = Self(9);
    pub const GA: Self = Self(0xa);
    pub const RGA: Self = Self(0xb);
    pub const BA: Self = Self(0xc);
    pub const RBA: Self = Self(0xd);
    pub const GBA: Self = Self(0xe);
    pub const RGBA: Self = Self(0xf);
}

/// GX2ClearFlags
#[doc(alias = "GX2ClearFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClearFlags(pub u32);

impl ClearFlags {
    pub const DEPTH: Self = Self(1);
    pub const STENCIL: Self = Self(2);
    pub const BOTH: Self = Self(3);
    pub const D_REG: Self = Self(4);
    pub const S_REG: Self = Self(8);
    pub const D_S_REG: Self = Self(0xc);
    pub const DEPTH_D_REG: Self = Self(5);
    pub const STENCIL_S_REG: Self = Self(0xa);
    pub const BOTH_D_S_REG: Self = Self(0xf);
}

impl core::ops::BitOr for ClearFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2CompareFunction
#[doc(alias = "GX2CompareFunction")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompareFunction(pub u32);

impl CompareFunction {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LEQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GEQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}

/// GX2DrcRenderMode
#[doc(alias = "GX2DrcRenderMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcRenderMode(pub u32);

impl DrcRenderMode {
    pub const DISABLED: Self = Self(0);
    pub const SINGLE: Self = Self(1);
    pub const DOUBLE: Self = Self(2);
    pub const SINGLE_30HZ: Self = Self(3);
}

/// GX2EventType
#[doc(alias = "GX2EventType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventType(pub u32);

impl EventType {
    pub const START_OF_PIPE_INTERRUPT: Self = Self(0);
    pub const END_OF_PIPE_INTERRUPT: Self = Self(1);
    pub const VSYNC: Self = Self(2);
    pub const FLIP: Self = Self(3);
    pub const DISPLAY_LIST_OVERRUN: Self = Self(4);
}

/// GX2EndianSwapMode
#[doc(alias = "GX2EndianSwapMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndianSwapMode(pub u32);

impl EndianSwapMode {
    pub const NONE: Self = Self(0);
    pub const SWAP_8_IN_16: Self = Self(1);
    pub const SWAP_8_IN_32: Self = Self(2);
    pub const DEFAULT: Self = Self(3);
}

/// GX2FetchShaderType
#[doc(alias = "GX2FetchShaderType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FetchShaderType(pub u32);

impl FetchShaderType {
    pub const NONE: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const TRIANGLE: Self = Self(2);
    pub const QUAD: Self = Self(3);
}

/// GX2FrontFace
#[doc(alias = "GX2FrontFace")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrontFace(pub u32);

impl FrontFace {
    pub const CCW: Self = Self(0);
    pub const CW: Self = Self(1);
}

/// GX2IndexType
#[doc(alias = "GX2IndexType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexType {
    #[default]
    U16Le = 0,
    U32Le = 1,
    U16 = 4,
    U32 = 9,
}

/// GX2InvalidateMode
#[doc(alias = "GX2InvalidateMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidateMode(pub u32);

impl InvalidateMode {
    pub const NONE: Self = Self(0);
    pub const ATTRIBUTE_BUFFER: Self = Self(1);
    pub const TEXTURE: Self = Self(2);
    pub const UNIFORM_BLOCK: Self = Self(4);
    pub const SHADER: Self = Self(8);
    pub const COLOR_BUFFER: Self = Self(0x10);
    pub const DEPTH_BUFFER: Self = Self(0x20);
    pub const CPU: Self = Self(0x40);
    pub const STREAM_OUT_BUFFER: Self = Self(0x80);
    pub const EXPORT_BUFFER: Self = Self(0x100);
    pub const CPU_ATTRIBUTE_BUFFER: Self = Self(0x41);
    pub const CPU_TEXTURE: Self = Self(0x42);
    pub const CPU_SHADER: Self = Self(0x48);
}

impl core::ops::BitOr for InvalidateMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2InitAttributes
#[doc(alias = "GX2InitAttributes")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitAttributes(pub u32);

impl InitAttributes {
    pub const END: Self = Self(0);
    pub const CMD_BUF_BASE: Self = Self(1);
    pub const CMD_BUF_POOL_SIZE: Self = Self(2);
    pub const UDA_LEVEL: Self = Self(3);
    pub const UDA_TEST_MODE: Self = Self(4);
    pub const UDA_DISABLE_ALERT: Self = Self(5);
    pub const UDA_MAX_FIRE_COUNT: Self = Self(6);
    pub const ARGC: Self = Self(7);
    pub const ARGV: Self = Self(8);
    pub const PROFILE_MODE: Self = Self(9);
    pub const TOSS_STAGE: Self = Self(0xa);
    pub const APP_IO_THREAD_STACK_SIZE: Self = Self(0xb);
}

/// GX2LogicOp
#[doc(alias = "GX2LogicOp")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicOp(pub u32);

impl LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const NOR: Self = Self(0x11);
    pub const INV_AND: Self = Self(0x22);
    pub const INV_COPY: Self = Self(0x33);
    pub const REV_AND: Self = Self(0x44);
    pub const INV: Self = Self(0x55);
    pub const XOR: Self = Self(0x66);
    pub const NOT_AND: Self = Self(0x77);
    pub const AND: Self = Self(0x88);
    pub const EQUIV: Self = Self(0x99);
    pub const NOP: Self = Self(0xaa);
    pub const INV_OR: Self = Self(0xbb);
    pub const COPY: Self = Self(0xcc);
    pub const REV_OR: Self = Self(0xdd);
    pub const OR: Self = Self(0xee);
    pub const SET: Self = Self(0xff);
}

/// GX2PrimitiveMode
#[doc(alias = "GX2PrimitiveMode")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveMode {
    Points = 1,
    Lines = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleFan = 5,
    TriangleStrip = 6,
    LinesAdjacency = 0xa,
    LineStripAdjacency = 0xb,
    TrianglesAdjacency = 0xc,
    TriangleStripAdjacency = 0xd,
    Rects = 0x11,
    LineLoop = 0x12,
    Quads = 0x13,
    QuadStrip = 0x14,
}

/// GX2PolygonMode
#[doc(alias = "GX2PolygonMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolygonMode(pub u32);

impl PolygonMode {
    pub const POINT: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const TRIANGLE: Self = Self(2);
}

/// GX2RenderTarget
#[doc(alias = "GX2RenderTarget")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTarget(pub u32);

impl RenderTarget {
    pub const TARGET_0: Self = Self(0);
    pub const TARGET_1: Self = Self(1);
    pub const TARGET_2: Self = Self(2);
    pub const TARGET_3: Self = Self(3);
    pub const TARGET_4: Self = Self(4);
    pub const TARGET_5: Self = Self(5);
    pub const TARGET_6: Self = Self(6);
    pub const TARGET_7: Self = Self(7);
}

/// GX2RoundingMode
#[doc(alias = "GX2RoundingMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    #[default]
    RoundToEven = 0,
    Truncate = 1,
}

/// GX2SamplerVarType
#[doc(alias = "GX2SamplerVarType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerVarType(pub u32);

impl SamplerVarType {
    pub const SAMPLER_1D: Self = Self(0);
    pub const SAMPLER_2D: Self = Self(1);
    pub const SAMPLER_2D_RECT: Self = Self(2);
    pub const SAMPLER_3D: Self = Self(3);
    pub const CUBE: Self = Self(4);
    pub const SAMPLER_1D_SHADOW: Self = Self(5);
    pub const SAMPLER_2D_SHADOW: Self = Self(6);
    pub const SAMPLER_2D_RECT_SHADOW: Self = Self(7);
    pub const CUBE_SHADOW: Self = Self(8);
    pub const SAMPLER_1D_ARRAY: Self = Self(9);
    pub const SAMPLER_2D_ARRAY: Self = Self(0xa);
    pub const SAMPLER_1D_ARRAY_SHADOW: Self = Self(0xb);
    pub const SAMPLER_2D_ARRAY_SHADOW: Self = Self(0xc);
    pub const CUBE_ARRAY: Self = Self(0xd);
    pub const CUBE_ARRAY_SHADOW: Self = Self(0xe);
    pub const BUFFER: Self = Self(0xf);
    pub const SAMPLER_2D_MS: Self = Self(0x11);
    pub const SAMPLER_2D_MS_ARRAY: Self = Self(0x12);
    pub const INT_1D: Self = Self(0x13);
    pub const INT_2D: Self = Self(0x14);
    pub const INT_2D_RECT: Self = Self(0x15);
    pub const INT_3D: Self = Self(0x16);
    pub const INT_CUBE: Self = Self(0x17);
    pub const INT_1D_ARRAY: Self = Self(0x18);
    pub const INT_2D_ARRAY: Self = Self(0x19);
    pub const INT_CUBE_ARRAY: Self = Self(0x1a);
    pub const INT_BUFFER: Self = Self(0x1b);
    pub const INT_2D_MS: Self = Self(0x1d);
    pub const INT_2D_MS_ARRAY: Self = Self(0x1e);
    pub const UNSIGNED_INT_1D: Self = Self(0x1f);
    pub const UNSIGNED_INT_2D: Self = Self(0x20);
    pub const UNSIGNED_INT_2D_RECT: Self = Self(0x21);
    pub const UNSIGNED_INT_3D: Self = Self(0x22);
    pub const UNSIGNED_INT_CUBE: Self = Self(0x23);
    pub const UNSIGNED_INT_1D_ARRAY: Self = Self(0x24);
    pub const UNSIGNED_INT_2D_ARRAY: Self = Self(0x25);
    pub const UNSIGNED_INT_CUBE_ARRAY: Self = Self(0x26);
    pub const UNSIGNED_INT_BUFFER: Self = Self(0x27);
    pub const UNSIGNED_INT_2D_MS: Self = Self(0x29);
    pub const UNSIGNED_INT_2D_MS_ARRAY: Self = Self(0x2a);
}

/// GX2ScanTarget
#[doc(alias = "GX2ScanTarget")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScanTarget {
    Tv0 = 1,
    Tv1 = 2,
    Drc0 = 4,
    Drc1 = 8,
}

impl ScanTarget {
    pub const TV: Self = Self::Tv0;
    pub const DRC: Self = Self::Drc0;
}

/// GX2SemaphoreAction
#[doc(alias = "GX2SemaphoreAction")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemaphoreAction {
    #[default]
    Wait = 0,
    Signal = 1,
}

/// GX2ShaderMode
#[doc(alias = "GX2ShaderMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderMode(pub u32);

impl ShaderMode {
    pub const UNIFORM_REGISTER: Self = Self(0);
    pub const UNIFORM_BLOCK: Self = Self(1);
    pub const GEOMETRY_SHADER: Self = Self(2);
    pub const COMPUTE_SHADER: Self = Self(3);
}

/// GX2ShaderVarType
#[doc(alias = "GX2ShaderVarType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderVarType(pub u32);

impl ShaderVarType {
    pub const VOID: Self = Self(0);
    pub const BOOL: Self = Self(1);
    pub const INT: Self = Self(2);
    pub const UINT: Self = Self(3);
    pub const FLOAT: Self = Self(4);
    pub const DOUBLE: Self = Self(5);
    pub const DOUBLE2: Self = Self(6);
    pub const DOUBLE3: Self = Self(7);
    pub const DOUBLE4: Self = Self(8);
    pub const FLOAT2: Self = Self(9);
    pub const FLOAT3: Self = Self(0xa);
    pub const FLOAT4: Self = Self(0xb);
    pub const BOOL2: Self = Self(0xc);
    pub const BOOL3: Self = Self(0xd);
    pub const BOOL4: Self = Self(0xe);
    pub const INT2: Self = Self(0xf);
    pub const INT3: Self = Self(0x10);
    pub const INT4: Self = Self(0x11);
    pub const UINT2: Self = Self(0x12);
    pub const UINT3: Self = Self(0x13);
    pub const UINT4: Self = Self(0x14);
    pub const FLOAT2X2: Self = Self(0x15);
    pub const FLOAT2X3: Self = Self(0x16);
    pub const FLOAT2X4: Self = Self(0x17);
    pub const FLOAT3X2: Self = Self(0x18);
    pub const FLOAT3X3: Self = Self(0x19);
    pub const FLOAT3X4: Self = Self(0x1a);
    pub const FLOAT4X2: Self = Self(0x1b);
    pub const FLOAT4X3: Self = Self(0x1c);
    pub const FLOAT4X4: Self = Self(0x1d);
    pub const DOUBLE2X2: Self = Self(0x1e);
    pub const DOUBLE2X3: Self = Self(0x1f);
    pub const DOUBLE2X4: Self = Self(0x20);
    pub const DOUBLE3X2: Self = Self(0x21);
    pub const DOUBLE3X3: Self = Self(0x22);
    pub const DOUBLE3X4: Self = Self(0x23);
    pub const DOUBLE4X2: Self = Self(0x24);
    pub const DOUBLE4X3: Self = Self(0x25);
    pub const DOUBLE4X4: Self = Self(0x26);
}

/// GX2StencilFunction
#[doc(alias = "GX2StencilFunction")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilFunction(pub u32);

impl StencilFunction {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCR_CLAMP: Self = Self(3);
    pub const DECR_CLAMP: Self = Self(4);
    pub const INV: Self = Self(5);
    pub const INCR_WRAP: Self = Self(6);
    pub const DECR_WRAP: Self = Self(7);
}

/// GX2SurfaceDim
#[doc(alias = "GX2SurfaceDim")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceDim(pub u32);

impl SurfaceDim {
    pub const TEXTURE_1D: Self = Self(0);
    pub const TEXTURE_2D: Self = Self(1);
    pub const TEXTURE_3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const TEXTURE_1D_ARRAY: Self = Self(4);
    pub const TEXTURE_2D_ARRAY: Self = Self(5);
    pub const TEXTURE_2D_MSAA: Self = Self(6);
    pub const TEXTURE_2D_MSAA_ARRAY: Self = Self(7);
}

/// GX2SurfaceFormat
#[doc(alias = "GX2SurfaceFormat")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceFormat(pub u32);

impl SurfaceFormat {
    pub const INVALID: Self = Self(0);
    pub const UNORM_R4_G4: Self = Self(2);
    pub const UNORM_R4_G4_B4_A4: Self = Self(0xb);
    pub const UNORM_R8: Self = Self(1);
    pub const UNORM_R8_G8: Self = Self(7);
    pub const UNORM_R8_G8_B8_A8: Self = Self(0x1a);
    pub const UNORM_R16: Self = Self(5);
    pub const UNORM_R16_G16: Self = Self(0xf);
    pub const UNORM_R16_G16_B16_A16: Self = Self(0x1f);
    pub const UNORM_R5_G6_B5: Self = Self(8);
    pub const UNORM_R5_G5_B5_A1: Self = Self(0xa);
    pub const UNORM_A1_B5_G5_R5: Self = Self(0xc);
    pub const UNORM_R24_X8: Self = Self(0x11);
    pub const UNORM_A2_B10_G10_R10: Self = Self(0x1b);
    pub const UNORM_R10_G10_B10_A2: Self = Self(0x19);
    pub const UNORM_BC1: Self = Self(0x31);
    pub const UNORM_BC2: Self = Self(0x32);
    pub const UNORM_BC3: Self = Self(0x33);
    pub const UNORM_BC4: Self = Self(0x34);
    pub const UNORM_BC5: Self = Self(0x35);
    pub const UNORM_NV12: Self = Self(0x81);
    pub const UINT_R8: Self = Self(0x101);
    pub const UINT_R8_G8: Self = Self(0x107);
    pub const UINT_R8_G8_B8_A8: Self = Self(0x11a);
    pub const UINT_R16: Self = Self(0x105);
    pub const UINT_R16_G16: Self = Self(0x10f);
    pub const UINT_R16_G16_B16_A16: Self = Self(0x11f);
    pub const UINT_R32: Self = Self(0x10d);
    pub const UINT_R32_G32: Self = Self(0x11d);
    pub const UINT_R32_G32_B32_A32: Self = Self(0x122);
    pub const UINT_A2_B10_G10_R10: Self = Self(0x11b);
    pub const UINT_R10_G10_B10_A2: Self = Self(0x119);
    pub const UINT_X24_G8: Self = Self(0x111);
    pub const UINT_G8_X24: Self = Self(0x11c);
    pub const SNORM_R8: Self = Self(0x201);
    pub const SNORM_R8_G8: Self = Self(0x207);
    pub const SNORM_R8_G8_B8_A8: Self = Self(0x21a);
    pub const SNORM_R16: Self = Self(0x205);
    pub const SNORM_R16_G16: Self = Self(0x20f);
    pub const SNORM_R16_G16_B16_A16: Self = Self(0x21f);
    pub const SNORM_R10_G10_B10_A2: Self = Self(0x219);
    pub const SNORM_BC4: Self = Self(0x234);
    pub const SNORM_BC5: Self = Self(0x235);
    pub const SINT_R8: Self = Self(0x301);
    pub const SINT_R8_G8: Self = Self(0x307);
    pub const SINT_R8_G8_B8_A8: Self = Self(0x31a);
    pub const SINT_R16: Self = Self(0x305);
    pub const SINT_R16_G16: Self = Self(0x30f);
    pub const SINT_R16_G16_B16_A16: Self = Self(0x31f);
    pub const SINT_R32: Self = Self(0x30d);
    pub const SINT_R32_G32: Self = Self(0x31d);
    pub const SINT_R32_G32_B32_A32: Self = Self(0x322);
    pub const SINT_R10_G10_B10_A2: Self = Self(0x319);
    pub const SRGB_R8_G8_B8_A8: Self = Self(0x41a);
    pub const SRGB_BC1: Self = Self(0x431);
    pub const SRGB_BC2: Self = Self(0x432);
    pub const SRGB_BC3: Self = Self(0x433);
    pub const FLOAT_R32: Self = Self(0x80e);
    pub const FLOAT_R32_G32: Self = Self(0x81e);
    pub const FLOAT_R32_G32_B32_A32: Self = Self(0x823);
    pub const FLOAT_R16: Self = Self(0x806);
    pub const FLOAT_R16_G16: Self = Self(0x810);
    pub const FLOAT_R16_G16_B16_A16: Self = Self(0x820);
    pub const FLOAT_R11_G11_B10: Self = Self(0x816);
    pub const FLOAT_D24_S8: Self = Self(0x811);
    pub const FLOAT_X8_X24: Self = Self(0x81c);
}

/// GX2SurfaceUse
#[doc(alias = "GX2SurfaceUse")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceUse(pub u32);

impl SurfaceUse {
    pub const NONE: Self = Self(0);
    pub const TEXTURE: Self = Self(1);
    pub const COLOR_BUFFER: Self = Self(2);
    pub const DEPTH_BUFFER: Self = Self(4);
    pub const SCAN_BUFFER: Self = Self(8);
    pub const TV: Self = Self(0x80000000);
    pub const TEXTURE_COLOR_BUFFER_TV: Self = Self(0x80000003);
}

impl core::ops::BitOr for SurfaceUse {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2TessellationMode
#[doc(alias = "GX2TessellationMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TessellationMode {
    #[default]
    Discrete = 0,
    Continuous = 1,
    Adaptive = 2,
}

/// GX2TexBorderType
#[doc(alias = "GX2TexBorderType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexBorderType {
    #[default]
    TransparentBlack = 0,
    Black = 1,
    White = 2,
    Variable = 3,
}

/// GX2TexClampMode
#[doc(alias = "GX2TexClampMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexClampMode {
    #[default]
    Wrap = 0,
    Mirror = 1,
    Clamp = 2,
    MirrorOnce = 3,
    ClampHalfBorder = 4,
    MirrorOnceHalfBorder = 5,
    ClampBorder = 6,
    MirrorOnceBorder = 7,
}

/// GX2TexMipFilterMode
#[doc(alias = "GX2TexMipFilterMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexMipFilterMode {
    #[default]
    None = 0,
    Point = 1,
    Linear = 2,
}

/// GX2TexMipPerfMode
#[doc(alias = "GX2TexMipPerfMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexMipPerfMode {
    #[default]
    Disable = 0,
}

/// GX2TexXYFilterMode
#[doc(alias = "GX2TexXYFilterMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexXYFilterMode {
    #[default]
    Point = 0,
    Linear = 1,
    Bicubic = 2,
}

/// GX2TexAnisoRatio
#[doc(alias = "GX2TexAnisoRatio")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexAnisoRatio {
    #[default]
    None = 0,
    Ratio2To1 = 1,
    Ratio4To1 = 2,
    Ratio8To1 = 3,
    Ratio16To1 = 4,
}

/// GX2TexZFilterMode
#[doc(alias = "GX2TexZFilterMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexZFilterMode {
    #[default]
    None = 0,
    Point = 1,
    Linear = 2,
}

/// GX2TexZPerfMode
#[doc(alias = "GX2TexZPerfMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexZPerfMode {
    #[default]
    Disabled = 0,
}

/// GX2TileMode
#[doc(alias = "GX2TileMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileMode(pub u32);

impl TileMode {
    pub const DEFAULT: Self = Self(0);
    pub const LINEAR_ALIGNED: Self = Self(1);
    pub const TILED_1D_THIN1: Self = Self(2);
    pub const TILED_1D_THICK: Self = Self(3);
    pub const TILED_2D_THIN1: Self = Self(4);
    pub const TILED_2D_THIN2: Self = Self(5);
    pub const TILED_2D_THIN4: Self = Self(6);
    pub const TILED_2D_THICK: Self = Self(7);
    pub const TILED_2B_THIN1: Self = Self(8);
    pub const TILED_2B_THIN2: Self = Self(9);
    pub const TILED_2B_THIN4: Self = Self(0xa);
    pub const TILED_2B_THICK: Self = Self(0xb);
    pub const TILED_3D_THIN1: Self = Self(0xc);
    pub const TILED_3D_THICK: Self = Self(0xd);
    pub const TILED_3B_THIN1: Self = Self(0xe);
    pub const TILED_3B_THICK: Self = Self(0xf);
    pub const LINEAR_SPECIAL: Self = Self(0x10);
}

/// GX2TVRenderMode
#[doc(alias = "GX2TVRenderMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TvRenderMode {
    #[default]
    Disabled = 0,
    Standard480p = 1,
    Wide480p = 2,
    Wide720p = 3,
    Wide1080p = 5,
}

/// GX2TVScanMode
#[doc(alias = "GX2TVScanMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TvScanMode(pub u32);

impl TvScanMode {
    pub const NONE: Self = Self(0);
    pub const MODE_576I: Self = Self(1);
    pub const MODE_480I: Self = Self(2);
    pub const MODE_480P: Self = Self(3);
    pub const MODE_720P: Self = Self(4);
    pub const MODE_1080I: Self = Self(6);
    pub const MODE_1080P: Self = Self(7);
}
