//! GX2 samplers

use super::enums::{
    CompareFunction, RoundingMode, TexAnisoRatio, TexBorderType, TexClampMode, TexMipFilterMode,
    TexMipPerfMode, TexXYFilterMode, TexZFilterMode, TexZPerfMode,
};
use crate::ffi;

/// GX2Sampler
#[doc(alias = "GX2Sampler")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sampler {
    pub regs: [u32; 3],
}

const _: () = assert!(size_of::<Sampler>() == 0xC);

unsafe extern "C" {
    /// GX2InitSampler
    ///
    /// Initializes all sampler state with defaults.
    #[doc(alias = "GX2InitSampler")]
    #[link_name = "GX2InitSampler"]
    pub unsafe fn init_sampler(
        sampler: *mut Sampler,
        clamp: TexClampMode,
        min_mag_filter: TexXYFilterMode,
    );

    /// GX2InitSamplerClamping
    #[doc(alias = "GX2InitSamplerClamping")]
    #[link_name = "GX2InitSamplerClamping"]
    pub unsafe fn init_sampler_clamping(
        sampler: *mut Sampler,
        clamp_x: TexClampMode,
        clamp_y: TexClampMode,
        clamp_z: TexClampMode,
    );

    /// GX2InitSamplerXYFilter
    #[doc(alias = "GX2InitSamplerXYFilter")]
    #[link_name = "GX2InitSamplerXYFilter"]
    pub unsafe fn init_sampler_xy_filter(
        sampler: *mut Sampler,
        mag_filter: TexXYFilterMode,
        min_filter: TexXYFilterMode,
        max_aniso: TexAnisoRatio,
    );

    /// GX2InitSamplerZMFilter
    #[doc(alias = "GX2InitSamplerZMFilter")]
    #[link_name = "GX2InitSamplerZMFilter"]
    pub unsafe fn init_sampler_zm_filter(
        sampler: *mut Sampler,
        z_filter: TexZFilterMode,
        mip_filter: TexMipFilterMode,
    );

    /// GX2InitSamplerLOD
    #[doc(alias = "GX2InitSamplerLOD")]
    #[link_name = "GX2InitSamplerLOD"]
    pub unsafe fn init_sampler_lod(
        sampler: *mut Sampler,
        min_lod: f32,
        max_lod: f32,
        lod_bias: f32,
    );

    /// GX2InitSamplerLODAdjust
    #[doc(alias = "GX2InitSamplerLODAdjust")]
    #[link_name = "GX2InitSamplerLODAdjust"]
    pub unsafe fn init_sampler_lod_adjust(
        sampler: *mut Sampler,
        aniso_bias: f32,
        lod_uses_minor_axis: ffi::c_bool,
    );

    /// GX2InitSamplerBorderType
    #[doc(alias = "GX2InitSamplerBorderType")]
    #[link_name = "GX2InitSamplerBorderType"]
    pub unsafe fn init_sampler_border_type(sampler: *mut Sampler, border: TexBorderType);

    /// GX2InitSamplerRoundingMode
    #[doc(alias = "GX2InitSamplerRoundingMode")]
    #[link_name = "GX2InitSamplerRoundingMode"]
    pub unsafe fn init_sampler_rounding_mode(sampler: *mut Sampler, mode: RoundingMode);

    /// GX2InitSamplerDepthCompare
    #[doc(alias = "GX2InitSamplerDepthCompare")]
    #[link_name = "GX2InitSamplerDepthCompare"]
    pub unsafe fn init_sampler_depth_compare(sampler: *mut Sampler, compare: CompareFunction);

    /// GX2InitSamplerFilterAdjust
    #[doc(alias = "GX2InitSamplerFilterAdjust")]
    #[link_name = "GX2InitSamplerFilterAdjust"]
    pub unsafe fn init_sampler_filter_adjust(
        sampler: *mut Sampler,
        high_precision: ffi::c_bool,
        perf_mip: TexMipPerfMode,
        perf_z: TexZPerfMode,
    );

    /// GX2SetVertexSampler
    #[doc(alias = "GX2SetVertexSampler")]
    #[link_name = "GX2SetVertexSampler"]
    pub unsafe fn set_vertex_sampler(sampler: *const Sampler, unit: u32);

    /// GX2SetGeometrySampler
    #[doc(alias = "GX2SetGeometrySampler")]
    #[link_name = "GX2SetGeometrySampler"]
    pub unsafe fn set_geometry_sampler(sampler: *const Sampler, unit: u32);

    /// GX2SetPixelSampler
    #[doc(alias = "GX2SetPixelSampler")]
    #[link_name = "GX2SetPixelSampler"]
    pub unsafe fn set_pixel_sampler(sampler: *const Sampler, unit: u32);

    /// GX2SetComputeSampler
    #[doc(alias = "GX2SetComputeSampler")]
    #[link_name = "GX2SetComputeSampler"]
    pub unsafe fn set_compute_sampler(sampler: *const Sampler, unit: u32);

    /// GX2SetVertexSamplerBorderColor
    #[doc(alias = "GX2SetVertexSamplerBorderColor")]
    #[link_name = "GX2SetVertexSamplerBorderColor"]
    pub unsafe fn set_vertex_sampler_border_color(unit: u32, r: f32, g: f32, b: f32, a: f32);

    /// GX2SetGeometrySamplerBorderColor
    #[doc(alias = "GX2SetGeometrySamplerBorderColor")]
    #[link_name = "GX2SetGeometrySamplerBorderColor"]
    pub unsafe fn set_geometry_sampler_border_color(unit: u32, r: f32, g: f32, b: f32, a: f32);

    /// GX2SetPixelSamplerBorderColor
    #[doc(alias = "GX2SetPixelSamplerBorderColor")]
    #[link_name = "GX2SetPixelSamplerBorderColor"]
    pub unsafe fn set_pixel_sampler_border_color(unit: u32, r: f32, g: f32, b: f32, a: f32);

    /// GX2SetComputeSamplerBorderColor
    #[doc(alias = "GX2SetComputeSamplerBorderColor")]
    #[link_name = "GX2SetComputeSamplerBorderColor"]
    pub unsafe fn set_compute_sampler_border_color(unit: u32, r: f32, g: f32, b: f32, a: f32);
}
