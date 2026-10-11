//! GX2 display setup

use super::enums::{
    AspectRatio, BufferingMode, DrcRenderMode, SurfaceFormat, TvRenderMode, TvScanMode,
};
use crate::ffi;

/// Reason passed to a [DrcConnectCallback].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcConnectReason(pub i32);

impl DrcConnectReason {
    pub const DETACH: Self = Self(0);
    pub const ATTACH: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrcEncodingHint(pub u32);

impl DrcEncodingHint {
    pub const INVALIDATE: Self = Self(0x1);
    pub const MOTION_VECTOR: Self = Self(0x4);
}

impl core::ops::BitOr for DrcEncodingHint {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// GX2DRCConnectCallback
///
/// Called when the DRC on `channel` connects or disconnects.
#[doc(alias = "GX2DRCConnectCallback")]
pub type DrcConnectCallback = Option<unsafe extern "C" fn(channel: i32, reason: DrcConnectReason)>;

unsafe extern "C" {
    /// GX2SetTVEnable
    #[doc(alias = "GX2SetTVEnable")]
    #[link_name = "GX2SetTVEnable"]
    pub unsafe fn set_tv_enable(enable: ffi::c_bool);

    /// GX2SetDRCEnable
    #[doc(alias = "GX2SetDRCEnable")]
    #[link_name = "GX2SetDRCEnable"]
    pub unsafe fn set_drc_enable(enable: ffi::c_bool);

    /// GX2CalcTVSize
    ///
    /// Size of the TV scan buffer, and whether the system has to scale it to the TV resolution.
    #[doc(alias = "GX2CalcTVSize")]
    #[link_name = "GX2CalcTVSize"]
    pub unsafe fn calc_tv_size(
        mode: TvRenderMode,
        format: SurfaceFormat,
        buffering: BufferingMode,
        size: *mut u32,
        scale_needed: *mut ffi::c_bool,
    );

    /// GX2CalcDRCSize
    ///
    /// Size of the DRC scan buffer, and whether the system has to scale it.
    #[doc(alias = "GX2CalcDRCSize")]
    #[link_name = "GX2CalcDRCSize"]
    pub unsafe fn calc_drc_size(
        mode: DrcRenderMode,
        format: SurfaceFormat,
        buffering: BufferingMode,
        size: *mut u32,
        scale_needed: *mut ffi::c_bool,
    );

    /// GX2SetTVBuffer
    ///
    /// `buffer` must be aligned to [SCAN_BUFFER_ALIGNMENT][super::enums::SCAN_BUFFER_ALIGNMENT] and [calc_tv_size] bytes.
    #[doc(alias = "GX2SetTVBuffer")]
    #[link_name = "GX2SetTVBuffer"]
    pub unsafe fn set_tv_buffer(
        buffer: *mut ffi::c_void,
        size: u32,
        mode: TvRenderMode,
        format: SurfaceFormat,
        buffering: BufferingMode,
    );

    /// GX2SetDRCBuffer
    ///
    /// `buffer` must be aligned to [SCAN_BUFFER_ALIGNMENT][super::enums::SCAN_BUFFER_ALIGNMENT] and [calc_drc_size] bytes.
    #[doc(alias = "GX2SetDRCBuffer")]
    #[link_name = "GX2SetDRCBuffer"]
    pub unsafe fn set_drc_buffer(
        buffer: *mut ffi::c_void,
        size: u32,
        mode: DrcRenderMode,
        format: SurfaceFormat,
        buffering: BufferingMode,
    );

    /// GX2SetTVScale
    #[doc(alias = "GX2SetTVScale")]
    #[link_name = "GX2SetTVScale"]
    pub unsafe fn set_tv_scale(width: u32, height: u32);

    /// GX2SetDRCScale
    #[doc(alias = "GX2SetDRCScale")]
    #[link_name = "GX2SetDRCScale"]
    pub unsafe fn set_drc_scale(width: u32, height: u32);

    /// GX2GetSystemTVAspectRatio
    #[doc(alias = "GX2GetSystemTVAspectRatio")]
    #[link_name = "GX2GetSystemTVAspectRatio"]
    pub unsafe fn get_system_tv_aspect_ratio() -> AspectRatio;

    /// GX2GetSystemTVScanMode
    #[doc(alias = "GX2GetSystemTVScanMode")]
    #[link_name = "GX2GetSystemTVScanMode"]
    pub unsafe fn get_system_tv_scan_mode() -> TvScanMode;

    /// GX2GetSystemDRCMode
    #[doc(alias = "GX2GetSystemDRCMode")]
    #[link_name = "GX2GetSystemDRCMode"]
    pub unsafe fn get_system_drc_mode() -> DrcRenderMode;

    /// GX2GetSystemDRCScanMode
    #[doc(alias = "GX2GetSystemDRCScanMode")]
    #[link_name = "GX2GetSystemDRCScanMode"]
    pub unsafe fn get_system_drc_scan_mode() -> DrcRenderMode;

    /// GX2GetSystemTVStereoDisplayCapability
    #[doc(alias = "GX2GetSystemTVStereoDisplayCapability")]
    #[link_name = "GX2GetSystemTVStereoDisplayCapability"]
    pub unsafe fn get_system_tv_stereo_display_capability() -> u32;

    /// GX2SetTVStereoMode
    #[doc(alias = "GX2SetTVStereoMode")]
    #[link_name = "GX2SetTVStereoMode"]
    pub unsafe fn set_tv_stereo_mode(mode: u32);

    /// GX2SetDRCConnectCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "GX2SetDRCConnectCallback")]
    #[link_name = "GX2SetDRCConnectCallback"]
    pub unsafe fn set_drc_connect_callback(
        channel: i32,
        callback: DrcConnectCallback,
    ) -> DrcConnectCallback;

    /// GX2IsVideoOutReady
    #[doc(alias = "GX2IsVideoOutReady")]
    #[link_name = "GX2IsVideoOutReady"]
    pub unsafe fn is_video_out_ready() -> ffi::c_bool;

    /// GX2SetVideoEncodingHint
    ///
    /// Hints the DRC video encoder, e.g. with a global motion vector.
    #[doc(alias = "GX2SetVideoEncodingHint")]
    #[link_name = "GX2SetVideoEncodingHint"]
    pub unsafe fn set_video_encoding_hint(
        hint: DrcEncodingHint,
        motion_vector_x: i8,
        motion_vector_y: i8,
    ) -> ffi::c_bool;

    /// GX2GetDRCVerticalInfo
    ///
    /// Current vertical count and phase of the DRC.
    #[doc(alias = "GX2GetDRCVerticalInfo")]
    #[link_name = "GX2GetDRCVerticalInfo"]
    pub unsafe fn get_drc_vertical_info(vcount: *mut u32, vphase: *mut u32);

    /// GX2SetTVGamma
    #[doc(alias = "GX2SetTVGamma")]
    #[link_name = "GX2SetTVGamma"]
    pub unsafe fn set_tv_gamma(gamma: f32);

    /// GX2GetTVGamma
    #[doc(alias = "GX2GetTVGamma")]
    #[link_name = "GX2GetTVGamma"]
    pub unsafe fn get_tv_gamma(gamma: *mut f32);

    /// GX2GetTVGammaEx
    #[doc(alias = "GX2GetTVGammaEx")]
    #[link_name = "GX2GetTVGammaEx"]
    pub unsafe fn get_tv_gamma_ex(gamma: *mut f32);

    /// GX2SetDRCGamma
    #[doc(alias = "GX2SetDRCGamma")]
    #[link_name = "GX2SetDRCGamma"]
    pub unsafe fn set_drc_gamma(gamma: f32);

    /// GX2GetDRCGamma
    #[doc(alias = "GX2GetDRCGamma")]
    #[link_name = "GX2GetDRCGamma"]
    pub unsafe fn get_drc_gamma(gamma: *mut f32);

    /// GX2GetDRCGammaEx
    #[doc(alias = "GX2GetDRCGammaEx")]
    #[link_name = "GX2GetDRCGammaEx"]
    pub unsafe fn get_drc_gamma_ex(gamma: *mut f32);
}
