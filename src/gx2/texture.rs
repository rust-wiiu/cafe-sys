//! GX2 textures

use super::shader::ComponentSelection;
use super::surface::Surface;

/// GX2Texture
#[doc(alias = "GX2Texture")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Texture {
    pub surface: Surface,
    pub view_first_mip: u32,
    pub view_num_mips: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub comp_map: ComponentSelection,
    pub regs: [u32; 5],
}

unsafe extern "C" {
    /// GX2InitTextureRegs
    ///
    /// Call after changing the texture.
    #[doc(alias = "GX2InitTextureRegs")]
    #[link_name = "GX2InitTextureRegs"]
    pub unsafe fn init_texture_regs(texture: *mut Texture);

    /// GX2SetVertexTexture
    #[doc(alias = "GX2SetVertexTexture")]
    #[link_name = "GX2SetVertexTexture"]
    pub unsafe fn set_vertex_texture(texture: *const Texture, unit: u32);

    /// GX2SetGeometryTexture
    #[doc(alias = "GX2SetGeometryTexture")]
    #[link_name = "GX2SetGeometryTexture"]
    pub unsafe fn set_geometry_texture(texture: *const Texture, unit: u32);

    /// GX2SetPixelTexture
    #[doc(alias = "GX2SetPixelTexture")]
    #[link_name = "GX2SetPixelTexture"]
    pub unsafe fn set_pixel_texture(texture: *const Texture, unit: u32);

    /// GX2SetComputeTexture
    #[doc(alias = "GX2SetComputeTexture")]
    #[link_name = "GX2SetComputeTexture"]
    pub unsafe fn set_compute_texture(texture: *const Texture, unit: u32);
}
