use crate::{UnsafeInit, ffi::*};
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Format {
    #[default]
    UnormR8G8B8A8 = 0x01A,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ColorBuffer {
    pub surface: Surface,
    pub view_mip: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub aa_ptr: *mut c_void,
    pub aa_size: u32,
    pub _regs: [u32; 5],
}

impl UnsafeInit for ColorBuffer {}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DepthBuffer {
    pub surface: Surface,
    pub view_mip: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub z_ptr: *mut c_void,
    pub z_size: u32,
    pub clear_depth: f32,
    pub clear_stencil: u32,
    pub _regs: [u32; 7],
}

impl UnsafeInit for DepthBuffer {}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Dimension {
    /// 1 Dimensional
    D1 = 0,
    /// 2 Dimensional
    D2 = 1,
    /// 3 Dimensional
    D3 = 2,
    Cube = 3,
    D1Array = 4,
    D2Array = 5,
    D2Msaa = 6,
    D2MsaaArray = 7,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum AntiAliasing {
    X1 = 0,
    X2 = 1,
    X4 = 2,
    X8 = 3,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum TileMode {
    #[default]
    Default = 0x00,
    LinearAligned = 0x01,
    Tiled1DThin1 = 0x02,
    Tiled1DThinK = 0x03,
    Tiled2DThin1 = 0x04,
    Tiled2DThin2 = 0x05,
    Tiled2DThin4 = 0x06,
    Tiled2DThick = 0x07,
    Tiled2BThin1 = 0x08,
    Tiled2BThin2 = 0x09,
    Tiled2BThin4 = 0x0a,
    Tiled2BThick = 0x0b,
    Tiled3DThin1 = 0x0c,
    Tiled3DThick = 0x0d,
    Tiled3BThin1 = 0x0e,
    Tiled3BThick = 0x0f,
    LinearSpecial = 0x10,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum MipLevel {
    L0 = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
    L6 = 6,
    L7 = 7,
    L8 = 8,
    L9 = 9,
    L10 = 10,
    L11 = 11,
    L12 = 12,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum SurfaceData {
    #[default]
    Image = 0,
    Mip0 = 1,
    Mip1 = 2,
    Mip2 = 3,
    Mip3 = 4,
    Mip4 = 5,
    Mip5 = 6,
    Mip6 = 7,
    Mip7 = 8,
    Mip8 = 9,
    Mip9 = 10,
    Mip10 = 11,
    Mip11 = 12,
    Mip12 = 13,
    MipAll = -1,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ResourceFlags : u32 {
        const Texture = 1 << 0;
        const ColorBuffer = 1 << 1;
        const DepthBuffer = 1 << 2;
        const ScanBuffer = 1 << 3;
        const VertexBuffer = 1 << 4;
        const IndexBuffer = 1 << 5;
        const UniformBlock = 1 << 6;
        const ShaderProgram = 1 << 7;
        const StreamOutput = 1 << 8;
        const DisplayList = 1 << 9;
        const GsRing = 1 << 10;
        const CpuRead = 1 << 11;
        const CpuWrite = 1 << 12;
        const Cpu = Self::CpuRead.bits() | Self::CpuWrite.bits();
        const GpuRead = 1 << 13;
        const GpuWrite = 1 << 14;
        const Gpu = Self::GpuRead.bits() | Self::GpuWrite.bits();
        const DmaRead = 1 << 15;
        const DmaWrite = 1 << 16;
        const Dma = Self::DmaRead.bits() | Self::DmaWrite.bits();
        const ForceMem1 = 1 << 17;
        const ForceMem2 = 1 << 18;
        const NoCpuInvalidate = 1 << 20;
        const NoGpuInvalidate = 1 << 21;
        const Readonly = 1 << 22;
        const Tv = 1 << 31;
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Surface {
    pub dim: Dimension,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub num_mips: u32,
    pub format: Format,
    pub aa: AntiAliasing,
    pub flags: ResourceFlags,
    pub image_size: u32,
    pub image: *mut c_void,
    pub mip_size: u32,
    pub mip: *mut c_void,
    pub tile_mode: TileMode,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_offset: [u32; 13],
}

impl UnsafeInit for Surface {}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2CalcSurfaceSizeAndAlignment
    #[link_name = "GX2CalcSurfaceSizeAndAlignment"]
    pub unsafe fn calc_size_alignment(surface: *mut Surface);

    /// GX2InitColorBufferRegs
    #[link_name = "GX2InitColorBufferRegs"]
    pub unsafe fn init_colorbuffer_regs(buffer: *mut ColorBuffer);

    /// GX2InitDepthBufferRegs
    #[link_name = "GX2InitDepthBufferRegs"]
    pub unsafe fn init_depthbuffer_regs(buffer: *mut DepthBuffer);

    #[doc(alias = "GX2RCreateSurface")]
    #[link_name = "GX2RCreateSurface"]
    pub unsafe fn create_surface(surface: *mut Surface, flags: ResourceFlags) -> c_bool;

    #[doc(alias = "GX2RDestroySurfaceEx")]
    #[link_name = "GX2RDestroySurfaceEx"]
    pub unsafe fn destroy_surface(surface: *mut Surface, flags: ResourceFlags);

    #[doc(alias = "GX2RLockSurfaceEx")]
    #[link_name = "GX2RLockSurfaceEx"]
    pub unsafe fn lock_surface(
        surface: *const Surface,
        data: SurfaceData,
        flags: ResourceFlags,
    ) -> *mut c_void;

    #[doc(alias = "GX2RUnlockSurfaceEx")]
    #[link_name = "GX2RUnlockSurfaceEx"]
    pub unsafe fn unlock_surface(surface: *const Surface, data: SurfaceData, flags: ResourceFlags);

    #[doc(alias = "GX2GetSurfaceMipSliceSize")]
    #[link_name = "GX2GetSurfaceMipSliceSize"]
    pub unsafe fn surface_mip_size(surface: *const Surface, level: MipLevel) -> u32;
}
