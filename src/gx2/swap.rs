//! GX2 scan buffer swapping

use super::enums::ScanTarget;
use super::surface::ColorBuffer;
use super::texture::Texture;
use crate::ffi;

unsafe extern "C" {
    /// GX2CopyColorBufferToScanBuffer
    ///
    /// Copies (and converts) `buffer` into the next scan buffer of `target`. Must not be called while writing a display list.
    #[doc(alias = "GX2CopyColorBufferToScanBuffer")]
    #[link_name = "GX2CopyColorBufferToScanBuffer"]
    pub unsafe fn copy_color_buffer_to_scan_buffer(buffer: *const ColorBuffer, target: ScanTarget);

    /// GX2SwapScanBuffers
    #[doc(alias = "GX2SwapScanBuffers")]
    #[link_name = "GX2SwapScanBuffers"]
    pub unsafe fn swap_scan_buffers();

    /// GX2GetSwapStatus
    #[doc(alias = "GX2GetSwapStatus")]
    #[link_name = "GX2GetSwapStatus"]
    pub unsafe fn get_swap_status(
        swap_count: *mut u32,
        flip_count: *mut u32,
        last_flip: *mut crate::coreinit::time::Time,
        last_vsync: *mut crate::coreinit::time::Time,
    );

    /// GX2GetSwapInterval
    #[doc(alias = "GX2GetSwapInterval")]
    #[link_name = "GX2GetSwapInterval"]
    pub unsafe fn get_swap_interval() -> u32;

    /// GX2SetSwapInterval
    ///
    /// Number of vsyncs per swap.
    #[doc(alias = "GX2SetSwapInterval")]
    #[link_name = "GX2SetSwapInterval"]
    pub unsafe fn set_swap_interval(interval: u32);

    /// GX2GetLastFrame
    ///
    /// Sets up `texture` to read the last displayed frame of `target`.
    #[doc(alias = "GX2GetLastFrame")]
    #[link_name = "GX2GetLastFrame"]
    pub unsafe fn get_last_frame(target: ScanTarget, texture: *mut Texture) -> ffi::c_bool;

    /// _GX2GetLastFrameB
    #[doc(alias = "_GX2GetLastFrameB")]
    #[link_name = "_GX2GetLastFrameB"]
    pub unsafe fn get_last_frame_b(target: ScanTarget, texture: *mut Texture) -> ffi::c_bool;

    /// GX2GetLastFrameGamma
    #[doc(alias = "GX2GetLastFrameGamma")]
    #[link_name = "GX2GetLastFrameGamma"]
    pub unsafe fn get_last_frame_gamma(target: ScanTarget, gamma: *mut f32) -> ffi::c_bool;

    /// GX2GetLastFrameGammaA
    #[doc(alias = "GX2GetLastFrameGammaA")]
    #[link_name = "GX2GetLastFrameGammaA"]
    pub unsafe fn get_last_frame_gamma_a(target: ScanTarget, gamma: *mut f32) -> ffi::c_bool;

    /// GX2GetLastFrameGammaB
    #[doc(alias = "GX2GetLastFrameGammaB")]
    #[link_name = "GX2GetLastFrameGammaB"]
    pub unsafe fn get_last_frame_gamma_b(target: ScanTarget, gamma: *mut f32) -> ffi::c_bool;

    /// GX2ProtectLastFrame
    #[doc(alias = "GX2ProtectLastFrame")]
    #[link_name = "GX2ProtectLastFrame"]
    pub unsafe fn protect_last_frame(target: ScanTarget, enable: ffi::c_bool);

    /// GX2GetCurrentScanBuffer
    ///
    /// Copies the color buffer describing the current scan buffer of `target`.
    #[doc(alias = "GX2GetCurrentScanBuffer")]
    #[link_name = "GX2GetCurrentScanBuffer"]
    pub unsafe fn get_current_scan_buffer(target: ScanTarget, buffer: *mut ColorBuffer);

    /// GX2MarkScanBufferCopied
    #[doc(alias = "GX2MarkScanBufferCopied")]
    #[link_name = "GX2MarkScanBufferCopied"]
    pub unsafe fn mark_scan_buffer_copied(target: ScanTarget);

    /// GX2WaitForFreeScanBuffer
    #[doc(alias = "GX2WaitForFreeScanBuffer")]
    #[link_name = "GX2WaitForFreeScanBuffer"]
    pub unsafe fn wait_for_free_scan_buffer(target: ScanTarget);
}
