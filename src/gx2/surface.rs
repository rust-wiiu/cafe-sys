//! GX2 surfaces

use super::enums::{
    AaMode, CompareFunction, RenderTarget, SurfaceDim, SurfaceFormat, SurfaceUse, TileMode,
};
use super::r::ResourceFlags;
use crate::ffi;

/// Usage of a [Surface], as [SurfaceUse] for GX2 or [ResourceFlags] for GX2R.
#[repr(C)]
#[derive(Clone, Copy)]
pub union SurfaceUsage {
    pub use_: SurfaceUse,
    pub resource_flags: ResourceFlags,
}

impl core::fmt::Debug for SurfaceUsage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SurfaceUsage")
            .field(unsafe { &self.use_.0 })
            .finish()
    }
}

impl Default for SurfaceUsage {
    fn default() -> Self {
        Self {
            use_: SurfaceUse::NONE,
        }
    }
}

/// GX2Surface
#[doc(alias = "GX2Surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub dim: SurfaceDim,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: SurfaceFormat,
    pub aa: AaMode,
    pub usage: SurfaceUsage,
    pub image_size: u32,
    pub image: *mut ffi::c_void,
    pub mipmap_size: u32,
    pub mipmaps: *mut ffi::c_void,
    pub tile_mode: TileMode,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_level_offset: [u32; 13],
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            dim: SurfaceDim::TEXTURE_2D,
            width: 0,
            height: 0,
            depth: 1,
            mip_levels: 1,
            format: SurfaceFormat::INVALID,
            aa: AaMode::MODE1X,
            usage: SurfaceUsage::default(),
            image_size: 0,
            image: core::ptr::null_mut(),
            mipmap_size: 0,
            mipmaps: core::ptr::null_mut(),
            tile_mode: TileMode::DEFAULT,
            swizzle: 0,
            alignment: 0,
            pitch: 0,
            mip_level_offset: [0; 13],
        }
    }
}

/// GX2ColorBuffer
#[doc(alias = "GX2ColorBuffer")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ColorBuffer {
    pub surface: Surface,
    pub view_mip: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    /// Multisample auxiliary buffer.
    pub aa_buffer: *mut ffi::c_void,
    pub aa_size: u32,
    pub regs: [u32; 5],
}

/// GX2DepthBuffer
#[doc(alias = "GX2DepthBuffer")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DepthBuffer {
    pub surface: Surface,
    pub view_mip: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub hi_z_ptr: *mut ffi::c_void,
    pub hi_z_size: u32,
    pub depth_clear: f32,
    pub stencil_clear: u32,
    pub regs: [u32; 7],
}

/// GX2Rect
#[doc(alias = "GX2Rect")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// GX2Point
#[doc(alias = "GX2Point")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// GX2_MAX_COPY_SURFACE_EX_RECTS
///
/// Maximum number of rectangles for [copy_ex].
#[doc(alias = "GX2_MAX_COPY_SURFACE_EX_RECTS")]
pub const MAX_COPY_SURFACE_EX_RECTS: u32 = 25;

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZRangeBase {
    #[default]
    Zmin = 0,
    Zmax = 1,
}

pub const NUM_HI_STENCIL_STATES: usize = 2;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HiStencilState {
    pub function: CompareFunction,
    pub reference: u8,
    pub mask: u8,
    pub enable: ffi::c_bool,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HiStencilInfo {
    pub state: [HiStencilState; NUM_HI_STENCIL_STATES],
    pub regs: [u32; NUM_HI_STENCIL_STATES],
}

/// Sample offsets from the pixel center in 1/16 pixels, from -8 to 7.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AaSampleLoc {
    pub x: [i8; 8],
    pub y: [i8; 8],
}

const _: () = {
    assert!(size_of::<Rect>() == 0x10);
    assert!(size_of::<Point>() == 0x8);
    assert!(size_of::<HiStencilState>() == 0xC);
    assert!(size_of::<HiStencilInfo>() == 0x20);
    assert!(size_of::<AaSampleLoc>() == 0x10);
};

unsafe extern "C" {
    /// GX2CalcSurfaceSizeAndAlignment
    ///
    /// Fills in the size, alignment, pitch, swizzle and mip offsets of `surface`.
    #[doc(alias = "GX2CalcSurfaceSizeAndAlignment")]
    #[link_name = "GX2CalcSurfaceSizeAndAlignment"]
    pub unsafe fn calc_size_and_alignment(surface: *mut Surface);

    /// GX2CalcDepthBufferHiZInfo
    #[doc(alias = "GX2CalcDepthBufferHiZInfo")]
    #[link_name = "GX2CalcDepthBufferHiZInfo"]
    pub unsafe fn calc_depth_buffer_hi_z_info(
        depth_buffer: *const DepthBuffer,
        size: *mut u32,
        alignment: *mut u32,
    );

    /// GX2CalcColorBufferAuxInfo
    ///
    /// Size and alignment of the multisample auxiliary buffer.
    #[doc(alias = "GX2CalcColorBufferAuxInfo")]
    #[link_name = "GX2CalcColorBufferAuxInfo"]
    pub unsafe fn calc_color_buffer_aux_info(
        color_buffer: *const ColorBuffer,
        size: *mut u32,
        alignment: *mut u32,
    );

    /// GX2SetColorBuffer
    #[doc(alias = "GX2SetColorBuffer")]
    #[link_name = "GX2SetColorBuffer"]
    pub unsafe fn set_color_buffer(color_buffer: *const ColorBuffer, target: RenderTarget);

    /// GX2SetDepthBuffer
    #[doc(alias = "GX2SetDepthBuffer")]
    #[link_name = "GX2SetDepthBuffer"]
    pub unsafe fn set_depth_buffer(depth_buffer: *const DepthBuffer);

    /// GX2InitColorBufferRegs
    #[doc(alias = "GX2InitColorBufferRegs")]
    #[link_name = "GX2InitColorBufferRegs"]
    pub unsafe fn init_color_buffer_regs(color_buffer: *mut ColorBuffer);

    /// GX2InitDepthBufferRegs
    #[doc(alias = "GX2InitDepthBufferRegs")]
    #[link_name = "GX2InitDepthBufferRegs"]
    pub unsafe fn init_depth_buffer_regs(depth_buffer: *mut DepthBuffer);

    /// GX2InitDepthBufferHiZEnable
    #[doc(alias = "GX2InitDepthBufferHiZEnable")]
    #[link_name = "GX2InitDepthBufferHiZEnable"]
    pub unsafe fn init_depth_buffer_hi_z_enable(
        depth_buffer: *mut DepthBuffer,
        enable: ffi::c_bool,
    );

    /// GX2InitDepthBufferRangeBase
    #[doc(alias = "GX2InitDepthBufferRangeBase")]
    #[link_name = "GX2InitDepthBufferRangeBase"]
    pub unsafe fn init_depth_buffer_range_base(depth_buffer: *mut DepthBuffer, base: ZRangeBase);

    /// GX2InitHiStencilInfoRegs
    #[doc(alias = "GX2InitHiStencilInfoRegs")]
    #[link_name = "GX2InitHiStencilInfoRegs"]
    pub unsafe fn init_hi_stencil_info_regs(info: *mut HiStencilInfo);

    /// GX2SetHiStencilInfo
    #[doc(alias = "GX2SetHiStencilInfo")]
    #[link_name = "GX2SetHiStencilInfo"]
    pub unsafe fn set_hi_stencil_info(info: *const HiStencilInfo);

    /// GX2GetSurfaceSwizzle
    #[doc(alias = "GX2GetSurfaceSwizzle")]
    #[link_name = "GX2GetSurfaceSwizzle"]
    pub unsafe fn get_swizzle(surface: *const Surface) -> u32;

    /// GX2SetSurfaceSwizzle
    #[doc(alias = "GX2SetSurfaceSwizzle")]
    #[link_name = "GX2SetSurfaceSwizzle"]
    pub unsafe fn set_swizzle(surface: *mut Surface, swizzle: u32);

    /// GX2GetSurfaceSwizzleOffset
    #[doc(alias = "GX2GetSurfaceSwizzleOffset")]
    #[link_name = "GX2GetSurfaceSwizzleOffset"]
    pub unsafe fn get_swizzle_offset(surface: *const Surface, mip_level: u32) -> u32;

    /// GX2GetSurfaceMipPitch
    #[doc(alias = "GX2GetSurfaceMipPitch")]
    #[link_name = "GX2GetSurfaceMipPitch"]
    pub unsafe fn get_mip_pitch(surface: *const Surface, mip_level: u32) -> u32;

    /// GX2GetSurfaceMipSliceSize
    #[doc(alias = "GX2GetSurfaceMipSliceSize")]
    #[link_name = "GX2GetSurfaceMipSliceSize"]
    pub unsafe fn get_mip_slice_size(surface: *const Surface, mip_level: u32) -> u32;

    /// GX2GetSurfaceFormatBits
    ///
    /// Bits per pixel. Compressed formats return the bits of a 4x4 block divided by 16.
    #[doc(alias = "GX2GetSurfaceFormatBits")]
    #[link_name = "GX2GetSurfaceFormatBits"]
    pub unsafe fn get_format_bits(format: SurfaceFormat) -> u32;

    /// GX2SurfaceIsCompressed
    #[doc(alias = "GX2SurfaceIsCompressed")]
    #[link_name = "GX2SurfaceIsCompressed"]
    pub unsafe fn is_compressed(format: SurfaceFormat) -> ffi::c_bool;

    /// GX2CheckSurfaceUseVsFormat
    ///
    /// Whether `format` can be used for `use_`.
    #[doc(alias = "GX2CheckSurfaceUseVsFormat")]
    #[link_name = "GX2CheckSurfaceUseVsFormat"]
    pub unsafe fn check_use_vs_format(use_: SurfaceUse, format: SurfaceFormat) -> ffi::c_bool;

    /// GX2CopySurface
    #[doc(alias = "GX2CopySurface")]
    #[link_name = "GX2CopySurface"]
    pub unsafe fn copy(
        src: *const Surface,
        src_mip: u32,
        src_slice: u32,
        dst: *mut Surface,
        dst_mip: u32,
        dst_slice: u32,
    );

    /// GX2CopySurfaceEx
    ///
    /// Copies up to [MAX_COPY_SURFACE_EX_RECTS] rectangles of `src` to the points in `dst`.
    #[doc(alias = "GX2CopySurfaceEx")]
    #[link_name = "GX2CopySurfaceEx"]
    pub unsafe fn copy_ex(
        src: *const Surface,
        src_mip: u32,
        src_slice: u32,
        dst: *mut Surface,
        dst_mip: u32,
        dst_slice: u32,
        num_rects: u32,
        src_rects: *const Rect,
        dst_points: *const Point,
    );

    /// GX2ResolveAAColorBuffer
    ///
    /// Resolves a multisampled color buffer into a single-sample surface.
    #[doc(alias = "GX2ResolveAAColorBuffer")]
    #[link_name = "GX2ResolveAAColorBuffer"]
    pub unsafe fn resolve_aa_color_buffer(
        src: *const ColorBuffer,
        dst: *mut Surface,
        dst_mip: u32,
        dst_slice: u32,
    );

    /// GX2ExpandAAColorBuffer
    #[doc(alias = "GX2ExpandAAColorBuffer")]
    #[link_name = "GX2ExpandAAColorBuffer"]
    pub unsafe fn expand_aa_color_buffer(buffer: *mut ColorBuffer);

    /// GX2ExpandDepthBuffer
    #[doc(alias = "GX2ExpandDepthBuffer")]
    #[link_name = "GX2ExpandDepthBuffer"]
    pub unsafe fn expand_depth_buffer(buffer: *mut DepthBuffer);

    /// GX2ConvertDepthBufferToTextureSurface
    #[doc(alias = "GX2ConvertDepthBufferToTextureSurface")]
    #[link_name = "GX2ConvertDepthBufferToTextureSurface"]
    pub unsafe fn convert_depth_buffer_to_texture_surface(
        src: *const DepthBuffer,
        dst: *mut Surface,
        dst_mip: u32,
        dst_slice: u32,
    );

    /// GX2SetAAMode
    #[doc(alias = "GX2SetAAMode")]
    #[link_name = "GX2SetAAMode"]
    pub unsafe fn set_aa_mode(aa: AaMode);

    /// GX2SetAAModeEx
    ///
    /// Same as [set_aa_mode] with custom sample positions.
    #[doc(alias = "GX2SetAAModeEx")]
    #[link_name = "GX2SetAAModeEx"]
    pub unsafe fn set_aa_mode_ex(sample_loc: *const AaSampleLoc, aa: AaMode);
}
