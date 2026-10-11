//! GX2 display lists

use crate::ffi;

/// GX2_DISPLAY_LIST_ALIGNMENT
#[doc(alias = "GX2_DISPLAY_LIST_ALIGNMENT")]
pub const DISPLAY_LIST_ALIGNMENT: u32 = 0x20;

/// GX2PatchType
#[doc(alias = "GX2PatchType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatchType(pub u32);

impl PatchType {
    pub const FETCH_SHADER: Self = Self(1);
    pub const VERTEX_SHADER: Self = Self(2);
    pub const GEOMETRY_COPY_SHADER: Self = Self(3);
    pub const GEOMETRY_SHADER: Self = Self(4);
    pub const PIXEL_SHADER: Self = Self(5);
    pub const COMPUTE_SHADER: Self = Self(6);
}

unsafe extern "C" {
    /// GX2BeginDisplayListEx
    ///
    /// Following GX2 commands are written into `display_list` instead of the command buffer.
    #[doc(alias = "GX2BeginDisplayListEx")]
    #[link_name = "GX2BeginDisplayListEx"]
    pub unsafe fn begin_ex(display_list: *mut ffi::c_void, size: u32, profiling: ffi::c_bool);

    /// GX2EndDisplayList
    ///
    /// Returns the used size in bytes.
    #[doc(alias = "GX2EndDisplayList")]
    #[link_name = "GX2EndDisplayList"]
    pub unsafe fn end(display_list: *mut ffi::c_void) -> u32;

    /// GX2CallDisplayList
    #[doc(alias = "GX2CallDisplayList")]
    #[link_name = "GX2CallDisplayList"]
    pub unsafe fn call(display_list: *const ffi::c_void, size: u32);

    /// GX2DirectCallDisplayList
    ///
    /// Same as [call], but submits the list to the GPU directly.
    #[doc(alias = "GX2DirectCallDisplayList")]
    #[link_name = "GX2DirectCallDisplayList"]
    pub unsafe fn direct_call(display_list: *const ffi::c_void, size: u32);

    /// GX2CopyDisplayList
    ///
    /// Copies the commands of `display_list` into the current command buffer.
    #[doc(alias = "GX2CopyDisplayList")]
    #[link_name = "GX2CopyDisplayList"]
    pub unsafe fn copy(display_list: *const ffi::c_void, size: u32);

    /// GX2GetDisplayListWriteStatus
    ///
    /// Whether a display list is being written.
    #[doc(alias = "GX2GetDisplayListWriteStatus")]
    #[link_name = "GX2GetDisplayListWriteStatus"]
    pub unsafe fn get_write_status() -> ffi::c_bool;

    /// GX2GetCurrentDisplayList
    #[doc(alias = "GX2GetCurrentDisplayList")]
    #[link_name = "GX2GetCurrentDisplayList"]
    pub unsafe fn get_current(
        display_list: *mut *mut ffi::c_void,
        size_max: *mut u32,
    ) -> ffi::c_bool;

    /// GX2PatchDisplayList
    ///
    /// Patches the shader address at `offset` in `display_list` to `object`.
    #[doc(alias = "GX2PatchDisplayList")]
    #[link_name = "GX2PatchDisplayList"]
    pub unsafe fn patch(
        display_list: *mut ffi::c_void,
        kind: PatchType,
        offset: u32,
        object: *mut ffi::c_void,
    );
}
