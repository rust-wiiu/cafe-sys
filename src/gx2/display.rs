use crate::gx2::surface::{self, DepthBuffer};
use crate::{ffi::*, gx2::surface::ColorBuffer};
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum DrcMode {
    Disabled = 0,
    Single = 1,
    Double = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ScanMode {
    /// 720x576 interlaced | 576I
    PAL = 1,
    /// 720x480 interlaced | 480I
    NTSC = 2,
    /// 720x480 progressive | 480P
    NTSCp = 3,
    /// 1280x720 progressive | 720P
    HD = 4,
    /// 1920x1080 interlaced | 1080I
    FHDi = 6,
    /// 1920x1080 progressive | 1080P
    FHD = 7,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum AspectRatio {
    /// 4:3
    Standard = 0,
    /// 16:9
    Widescreen = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum TvMode {
    Disabled = 0,
    Standard480 = 1,
    Wide480 = 2,
    Wide720 = 3,
    Wide1080 = 5,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Buffering {
    /// DEBUG ONLY - DO NOT USE
    #[deprecated]
    Single = 1,
    #[default]
    Double = 2,
    #[deprecated]
    Triple = 3,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ClearMode : u32 {
        const None = 0;
        const Depth = 1;
        const Stencil = 2;
        const DReg = 4;
        const SReg = 8;
        const Both = Self::Depth.bits() | Self::Stencil.bits();
        const DSReg = Self::DReg.bits() | Self::SReg.bits();
        const DepthDReg = Self::Depth.bits() | Self::DReg.bits();
        const StencilSReg = Self::Stencil.bits() | Self::SReg.bits();
        const BothDSReg = Self::Both.bits() | Self::DSReg.bits();
    }
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum SwapInterval {
    #[default]
    VSync60Hz = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ScanTarget {
    Tv = 1,
    Drc = 4,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// GX2GetSystemDRCMode
    #[link_name = "GX2GetSystemDRCMode"]
    pub unsafe fn drc_mode() -> DrcMode;

    /// GX2GetSystemTVScanMode
    #[link_name = "GX2GetSystemTVScanMode"]
    pub unsafe fn scan_mode() -> ScanMode;

    /// GX2GetSystemTVAspectRatio
    #[link_name = "GX2GetSystemTVAspectRatio"]
    pub unsafe fn aspect_ratio() -> AspectRatio;

    /// GX2CalcTVSize
    #[link_name = "GX2CalcTVSize"]
    pub unsafe fn tv_framebuffer_size(
        mode: TvMode,
        format: surface::Format,
        buffering: Buffering,
        size: *mut u32,
        scale_needed: *mut c_bool,
    );

    /// GX2CalcDRCSize
    #[link_name = "GX2CalcDRCSize"]
    pub unsafe fn drc_framebuffer_size(
        mode: DrcMode,
        format: surface::Format,
        buffering: Buffering,
        size: *mut u32,
        scale_needed: *mut c_bool,
    );

    /// GX2SetTVBuffer
    #[link_name = "GX2SetTVBuffer"]
    pub unsafe fn set_tv_buffer(
        buffer: *mut c_void,
        size: u32,
        mode: TvMode,
        format: surface::Format,
        buffering: Buffering,
    );

    /// GX2SetDRCBuffer
    #[link_name = "GX2SetDRCBuffer"]
    pub unsafe fn set_drc_buffer(
        buffer: *mut c_void,
        size: u32,
        mode: DrcMode,
        format: surface::Format,
        buffering: Buffering,
    );

    /// GX2SetSwapInterval
    #[link_name = "GX2SetSwapInterval"]
    pub unsafe fn set_swap_interval(swap_interval: SwapInterval);

    /// GX2WaitForVsync
    #[link_name = "GX2WaitForVsync"]
    pub unsafe fn wait_for_vsync();

    /// GX2DrawDone
    #[link_name = "GX2DrawDone"]
    pub unsafe fn draw_done() -> bool;

    /// GX2ClearColor
    #[link_name = "GX2ClearColor"]
    pub unsafe fn clear_color(buffer: *mut ColorBuffer, r: f32, g: f32, b: f32, a: f32);

    /// GX2ClearDepthStencilEx
    #[link_name = "GX2ClearDepthStencilEx"]
    pub unsafe fn clear_depth_stencil_ex(
        buffer: *mut DepthBuffer,
        depth: f32,
        stencil: u8,
        mode: ClearMode,
    );

    /// GX2CopyColorBufferToScanBuffer
    #[link_name = "GX2CopyColorBufferToScanBuffer"]
    pub unsafe fn copy_color_to_scan_buffer(buffer: *const ColorBuffer, target: ScanTarget);

    /// GX2SwapScanBuffers
    #[link_name = "GX2SwapScanBuffers"]
    pub unsafe fn swap_scan_buffers();

    /// GX2SetTVEnable
    #[link_name = "GX2SetTVEnable"]
    pub unsafe fn enable_tv(enable: bool);

    /// GX2SetDRCEnable
    #[link_name = "GX2SetDRCEnable"]
    pub unsafe fn enable_drc(enable: bool);

    /// GX2BeginDisplayListEx
    #[link_name = "GX2BeginDisplayListEx"]
    pub unsafe fn begin_display_list(buffer: *mut c_void, size: u32, profiling: c_bool);

    /// GX2EndDisplayList
    #[link_name = "GX2EndDisplayList"]
    pub unsafe fn end_display_list(buffer: *mut c_void) -> u32;
}
