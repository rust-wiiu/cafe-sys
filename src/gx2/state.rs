//! GX2 initialization and GPU state

use crate::ffi;

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiscType {
    /// [HangState].
    #[default]
    HangState = 0,
    /// [HangResponse].
    HangResponse = 1,
    HangResetSwapTimeout = 2,
    HangResetSwapCount = 3,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HangState(pub u32);

impl HangState {
    pub const OK: Self = Self(0);
    /// Timed out waiting for a timestamp.
    pub const TIME_STAMP: Self = Self(1);
    /// Timed out waiting for command buffer space.
    pub const COMMAND_BUFFER: Self = Self(2);
    /// Timed out waiting for ring buffer space.
    pub const RING_BUFFER: Self = Self(3);
    /// Timed out waiting for a flip.
    pub const WAIT_FLIP: Self = Self(4);
    pub const ETC: Self = Self(5);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HangResponse(pub u32);

impl HangResponse {
    pub const NONE: Self = Self(0);
    /// Dumps the GPU state.
    pub const DEBUG: Self = Self(1);
    /// Resets the GPU in [swap_scan_buffers][super::swap::swap_scan_buffers].
    pub const RESET: Self = Self(2);
}

/// Value of [InitAttributes::PROFILE_MODE][super::enums::InitAttributes::PROFILE_MODE].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileMode(pub u32);

impl ProfileMode {
    pub const NONE: Self = Self(0x00000);
    pub const INFINITELY_FAST_HARDWARE: Self = Self(0x00001);
    pub const WIREFRAME: Self = Self(0x00002);
    pub const DISABLE_ALPHABLEND: Self = Self(0x00004);
    pub const DISABLE_COLORWRITES: Self = Self(0x00008);
    pub const DISABLE_COLOR: Self = Self(0x00010);
    pub const MIN_VERTEX_SHADER: Self = Self(0x00020);
    pub const MIN_GEOMETRY_SHADER: Self = Self(0x00040);
    pub const MIN_PIXEL_SHADER: Self = Self(0x00080);
    pub const MIN_VERTEX_FETCH: Self = Self(0x00100);
    pub const MIN_TEXTURE_FETCH: Self = Self(0x00200);
    pub const DISABLE_Z: Self = Self(0x00400);
    pub const DISABLE_ZWRITES: Self = Self(0x00800);
    pub const DISABLE_STENCIL: Self = Self(0x01000);
    pub const DISABLE_TRILINEAR: Self = Self(0x02000);
    pub const DISABLE_ANISO: Self = Self(0x04000);
    pub const DISABLE_COLOR_CLEARS: Self = Self(0x08000);
    pub const DISABLE_Z_CLEARS: Self = Self(0x10000);
    pub const DISABLE_STENCIL_CLEARS: Self = Self(0x20000);
    pub const DISABLE_EARLY_Z: Self = Self(0x40000);
}

impl core::ops::BitOr for ProfileMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Value of [InitAttributes::TOSS_STAGE][super::enums::InitAttributes::TOSS_STAGE]. Disables the pipeline after a stage.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TossStage(pub u32);

impl TossStage {
    pub const NONE: Self = Self(0);
    pub const AFTER_VERTEX_FETCH: Self = Self(1);
    pub const AFTER_VERTEX_SHADER: Self = Self(2);
    pub const AFTER_GEOMETRY_SHADER: Self = Self(3);
    pub const AFTER_CLIPPING: Self = Self(4);
    pub const AFTER_PRIMITIVE_SETUP: Self = Self(5);
    pub const AFTER_SCAN_CONVERSION: Self = Self(6);
    pub const AFTER_RASTERIZATION: Self = Self(7);
    pub const AFTER_PIXEL_SHADER: Self = Self(8);
}

unsafe extern "C" {
    /// GX2Init
    ///
    /// `attributes` is null or a list of [InitAttributes][super::enums::InitAttributes] and value pairs, terminated by [InitAttributes::END][super::enums::InitAttributes::END]. The calling core becomes the main GX2 core.
    #[doc(alias = "GX2Init")]
    #[link_name = "GX2Init"]
    pub unsafe fn init(attributes: *const u32);

    /// GX2Shutdown
    #[doc(alias = "GX2Shutdown")]
    #[link_name = "GX2Shutdown"]
    pub unsafe fn shutdown();

    /// GX2Flush
    ///
    /// Submits the current command buffer to the GPU.
    #[doc(alias = "GX2Flush")]
    #[link_name = "GX2Flush"]
    pub unsafe fn flush();

    /// GX2ResetGPU
    #[doc(alias = "GX2ResetGPU")]
    #[link_name = "GX2ResetGPU"]
    pub unsafe fn reset_gpu(reserved: u32);

    /// GX2GetMainCoreId
    #[doc(alias = "GX2GetMainCoreId")]
    #[link_name = "GX2GetMainCoreId"]
    pub unsafe fn get_main_core_id() -> i32;

    /// GX2GetMiscParam
    #[doc(alias = "GX2GetMiscParam")]
    #[link_name = "GX2GetMiscParam"]
    pub unsafe fn get_misc_param(param: MiscType) -> u32;

    /// GX2SetMiscParam
    #[doc(alias = "GX2SetMiscParam")]
    #[link_name = "GX2SetMiscParam"]
    pub unsafe fn set_misc_param(param: MiscType, value: u32) -> ffi::c_bool;

    /// GX2GetGPUTimeout
    ///
    /// In milliseconds.
    #[doc(alias = "GX2GetGPUTimeout")]
    #[link_name = "GX2GetGPUTimeout"]
    pub unsafe fn get_gpu_timeout() -> u32;

    /// GX2SetGPUTimeout
    ///
    /// In milliseconds.
    #[doc(alias = "GX2SetGPUTimeout")]
    #[link_name = "GX2SetGPUTimeout"]
    pub unsafe fn set_gpu_timeout(millis: u32);

    /// GX2SetInterruptCountLimit
    #[doc(alias = "GX2SetInterruptCountLimit")]
    #[link_name = "GX2SetInterruptCountLimit"]
    pub unsafe fn set_interrupt_count_limit(limit: u32);
}
