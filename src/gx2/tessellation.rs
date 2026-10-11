//! GX2 tessellation

use super::enums::{IndexType, PrimitiveMode, TessellationMode};

unsafe extern "C" {
    /// GX2SetTessellation
    #[doc(alias = "GX2SetTessellation")]
    #[link_name = "GX2SetTessellation"]
    pub unsafe fn set_tessellation(
        mode: TessellationMode,
        primitive_mode: PrimitiveMode,
        index_type: IndexType,
    );

    /// GX2SetMinTessellationLevel
    #[doc(alias = "GX2SetMinTessellationLevel")]
    #[link_name = "GX2SetMinTessellationLevel"]
    pub unsafe fn set_min_tessellation_level(min: f32);

    /// GX2SetMaxTessellationLevel
    #[doc(alias = "GX2SetMaxTessellationLevel")]
    #[link_name = "GX2SetMaxTessellationLevel"]
    pub unsafe fn set_max_tessellation_level(max: f32);
}
