//! GX2R surfaces

use super::super::surface::Surface;
use super::ResourceFlags;
use crate::ffi;

unsafe extern "C" {
    /// GX2RCreateSurface
    ///
    /// Calculates the size of `surface` and allocates its image and mipmaps.
    #[doc(alias = "GX2RCreateSurface")]
    #[link_name = "GX2RCreateSurface"]
    pub unsafe fn create(surface: *mut Surface, flags: ResourceFlags) -> ffi::c_bool;

    /// GX2RCreateSurfaceUserMemory
    #[doc(alias = "GX2RCreateSurfaceUserMemory")]
    #[link_name = "GX2RCreateSurfaceUserMemory"]
    pub unsafe fn create_user_memory(
        surface: *mut Surface,
        image: *mut ffi::c_void,
        mipmaps: *mut ffi::c_void,
        flags: ResourceFlags,
    ) -> ffi::c_bool;

    /// GX2RDestroySurfaceEx
    #[doc(alias = "GX2RDestroySurfaceEx")]
    #[link_name = "GX2RDestroySurfaceEx"]
    pub unsafe fn destroy_ex(surface: *mut Surface, flags: ResourceFlags);

    /// GX2RSurfaceExists
    #[doc(alias = "GX2RSurfaceExists")]
    #[link_name = "GX2RSurfaceExists"]
    pub unsafe fn exists(surface: *const Surface) -> ffi::c_bool;

    /// GX2RLockSurfaceEx
    ///
    /// Returns `image`. `mip_level` selects the range to invalidate: 0 the image, 1 and above a mipmap, -1 all mipmaps.
    #[doc(alias = "GX2RLockSurfaceEx")]
    #[link_name = "GX2RLockSurfaceEx"]
    pub unsafe fn lock_ex(
        surface: *const Surface,
        mip_level: i32,
        flags: ResourceFlags,
    ) -> *mut ffi::c_void;

    /// GX2RUnlockSurfaceEx
    #[doc(alias = "GX2RUnlockSurfaceEx")]
    #[link_name = "GX2RUnlockSurfaceEx"]
    pub unsafe fn unlock_ex(surface: *const Surface, mip_level: i32, flags: ResourceFlags);

    /// GX2RInvalidateSurface
    #[doc(alias = "GX2RInvalidateSurface")]
    #[link_name = "GX2RInvalidateSurface"]
    pub unsafe fn invalidate(surface: *const Surface, mip_level: i32, flags: ResourceFlags);

    /// GX2RGetSurfaceName
    #[doc(alias = "GX2RGetSurfaceName")]
    #[link_name = "GX2RGetSurfaceName"]
    pub unsafe fn get_name(surface: *const Surface) -> *const ffi::c_char;

    /// GX2RSetSurfaceName
    #[doc(alias = "GX2RSetSurfaceName")]
    #[link_name = "GX2RSetSurfaceName"]
    pub unsafe fn set_name(surface: *const Surface, name: *const ffi::c_char);
}
