//! GX2 tiling apertures
//!
//! Linear CPU access to tiled surfaces.

use super::enums::EndianSwapMode;
use super::surface::Surface;
use crate::ffi;

/// GX2ApertureHandle
#[doc(alias = "GX2ApertureHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApertureHandle(pub u32);

unsafe extern "C" {
    /// GX2AllocateTilingApertureEx
    ///
    /// Maps one mip slice of a tiled surface to `address` for linear access.
    #[doc(alias = "GX2AllocateTilingApertureEx")]
    #[link_name = "GX2AllocateTilingApertureEx"]
    pub unsafe fn allocate_tiling_aperture_ex(
        surface: *const Surface,
        mip: u32,
        slice: u32,
        endian: EndianSwapMode,
        handle: *mut ApertureHandle,
        address: *mut *mut ffi::c_void,
    );

    /// GX2FreeTilingAperture
    #[doc(alias = "GX2FreeTilingAperture")]
    #[link_name = "GX2FreeTilingAperture"]
    pub unsafe fn free_tiling_aperture(handle: ApertureHandle);
}
