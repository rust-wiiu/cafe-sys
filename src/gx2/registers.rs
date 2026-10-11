//! GX2 render state
//!
//! The `set_*` functions write the state directly. The `*Reg` structs hold the same state as precomputed register values: fill them with `init_*_reg`, read them back with `get_*_reg` and apply them with `set_*_reg`.

use super::enums::{
    AlphaToMaskMode, BlendCombineMode, BlendMode, ChannelMask, CompareFunction, FrontFace, LogicOp,
    PolygonMode, RenderTarget, StencilFunction,
};
use crate::ffi;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialState(pub u32);

impl SpecialState {
    /// Accelerated surface clears.
    pub const CLEAR: Self = Self(0);
    /// Fast HiZ clears.
    pub const CLEAR_HIZ: Self = Self(1);
    /// Accelerated surface copies.
    pub const COPY: Self = Self(2);
    /// Expansion of compressed AA color buffers.
    pub const EXPAND_COLOR: Self = Self(3);
    /// Expansion of compressed depth buffers.
    pub const EXPAND_DEPTH: Self = Self(4);
    /// Depth buffer to color conversion.
    pub const CONVERT_DEPTH: Self = Self(5);
    /// AA depth buffer to color conversion.
    pub const CONVERT_AA_DEPTH: Self = Self(6);
    /// Multisample to single sample resolves.
    pub const RESOLVE_COLOR: Self = Self(7);
    /// Color clears as depth.
    pub const CLEAR_COLOR_AS_DEPTH: Self = Self(8);
}

/// GX2AAMaskReg
#[doc(alias = "GX2AAMaskReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AaMaskReg {
    pub pa_sc_aa_mask: u32,
}

/// GX2AlphaTestReg
#[doc(alias = "GX2AlphaTestReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlphaTestReg {
    pub sx_alpha_test_control: u32,
    pub sx_alpha_ref: u32,
}

/// GX2AlphaToMaskReg
#[doc(alias = "GX2AlphaToMaskReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlphaToMaskReg {
    pub db_alpha_to_mask: u32,
}

/// GX2BlendControlReg
#[doc(alias = "GX2BlendControlReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendControlReg {
    pub target: u32,
    pub cb_blend_control: u32,
}

/// GX2BlendConstantColorReg
#[doc(alias = "GX2BlendConstantColorReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BlendConstantColorReg {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

/// GX2ColorControlReg
#[doc(alias = "GX2ColorControlReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorControlReg {
    pub cb_color_control: u32,
}

/// GX2DepthStencilControlReg
#[doc(alias = "GX2DepthStencilControlReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DepthStencilControlReg {
    pub db_depth_control: u32,
}

/// GX2StencilMaskReg
#[doc(alias = "GX2StencilMaskReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilMaskReg {
    pub db_stencilrefmask: u32,
    pub db_stencilrefmask_bf: u32,
}

/// GX2LineWidthReg
#[doc(alias = "GX2LineWidthReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineWidthReg {
    pub pa_su_line_cntl: u32,
}

/// GX2PointSizeReg
#[doc(alias = "GX2PointSizeReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointSizeReg {
    pub pa_su_point_size: u32,
}

/// GX2PointLimitsReg
#[doc(alias = "GX2PointLimitsReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointLimitsReg {
    pub pa_su_point_minmax: u32,
}

/// GX2PolygonControlReg
#[doc(alias = "GX2PolygonControlReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolygonControlReg {
    pub pa_su_sc_mode_cntl: u32,
}

/// GX2PolygonOffsetReg
#[doc(alias = "GX2PolygonOffsetReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolygonOffsetReg {
    pub pa_su_poly_offset_front_scale: u32,
    pub pa_su_poly_offset_front_offset: u32,
    pub pa_su_poly_offset_back_scale: u32,
    pub pa_su_poly_offset_back_offset: u32,
    pub pa_su_poly_offset_clamp: u32,
}

/// GX2ScissorReg
#[doc(alias = "GX2ScissorReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScissorReg {
    pub pa_sc_generic_scissor_tl: u32,
    pub pa_sc_generic_scissor_br: u32,
}

/// GX2TargetChannelMaskReg
#[doc(alias = "GX2TargetChannelMaskReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetChannelMaskReg {
    pub cb_target_mask: u32,
}

/// GX2ViewportReg
#[doc(alias = "GX2ViewportReg")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewportReg {
    pub pa_cl_vport_xscale: u32,
    pub pa_cl_vport_xoffset: u32,
    pub pa_cl_vport_yscale: u32,
    pub pa_cl_vport_yoffset: u32,
    pub pa_cl_vport_zscale: u32,
    pub pa_cl_vport_zoffset: u32,
    pub pa_cl_gb_vert_clip_adj: u32,
    pub pa_cl_gb_vert_disc_adj: u32,
    pub pa_cl_gb_horz_clip_adj: u32,
    pub pa_cl_gb_horz_disc_adj: u32,
    pub pa_sc_vport_zmin: u32,
    pub pa_sc_vport_zmax: u32,
}

const _: () = {
    assert!(size_of::<AaMaskReg>() == 4);
    assert!(size_of::<AlphaTestReg>() == 8);
    assert!(size_of::<AlphaToMaskReg>() == 4);
    assert!(size_of::<BlendControlReg>() == 8);
    assert!(size_of::<BlendConstantColorReg>() == 0x10);
    assert!(size_of::<ColorControlReg>() == 4);
    assert!(size_of::<DepthStencilControlReg>() == 4);
    assert!(size_of::<StencilMaskReg>() == 8);
    assert!(size_of::<LineWidthReg>() == 4);
    assert!(size_of::<PointSizeReg>() == 4);
    assert!(size_of::<PointLimitsReg>() == 4);
    assert!(size_of::<PolygonControlReg>() == 4);
    assert!(size_of::<PolygonOffsetReg>() == 20);
    assert!(size_of::<ScissorReg>() == 8);
    assert!(size_of::<TargetChannelMaskReg>() == 4);
    assert!(size_of::<ViewportReg>() == 48);
};

unsafe extern "C" {
    /// GX2SetDepthStencilControl
    #[doc(alias = "GX2SetDepthStencilControl")]
    #[link_name = "GX2SetDepthStencilControl"]
    pub unsafe fn set_depth_stencil_control(
        depth_test: ffi::c_bool,
        depth_write: ffi::c_bool,
        depth_compare: CompareFunction,
        stencil_test: ffi::c_bool,
        back_stencil: ffi::c_bool,
        front_stencil_func: CompareFunction,
        front_stencil_z_pass: StencilFunction,
        front_stencil_z_fail: StencilFunction,
        front_stencil_fail: StencilFunction,
        back_stencil_func: CompareFunction,
        back_stencil_z_pass: StencilFunction,
        back_stencil_z_fail: StencilFunction,
        back_stencil_fail: StencilFunction,
    );

    /// GX2InitDepthStencilControlReg
    #[doc(alias = "GX2InitDepthStencilControlReg")]
    #[link_name = "GX2InitDepthStencilControlReg"]
    pub unsafe fn init_depth_stencil_control_reg(
        reg: *mut DepthStencilControlReg,
        depth_test: ffi::c_bool,
        depth_write: ffi::c_bool,
        depth_compare: CompareFunction,
        stencil_test: ffi::c_bool,
        back_stencil: ffi::c_bool,
        front_stencil_func: CompareFunction,
        front_stencil_z_pass: StencilFunction,
        front_stencil_z_fail: StencilFunction,
        front_stencil_fail: StencilFunction,
        back_stencil_func: CompareFunction,
        back_stencil_z_pass: StencilFunction,
        back_stencil_z_fail: StencilFunction,
        back_stencil_fail: StencilFunction,
    );

    /// GX2GetDepthStencilControlReg
    #[doc(alias = "GX2GetDepthStencilControlReg")]
    #[link_name = "GX2GetDepthStencilControlReg"]
    pub unsafe fn get_depth_stencil_control_reg(
        reg: *const DepthStencilControlReg,
        depth_test: *mut ffi::c_bool,
        depth_write: *mut ffi::c_bool,
        depth_compare: *mut CompareFunction,
        stencil_test: *mut ffi::c_bool,
        back_stencil: *mut ffi::c_bool,
        front_stencil_func: *mut CompareFunction,
        front_stencil_z_pass: *mut StencilFunction,
        front_stencil_z_fail: *mut StencilFunction,
        front_stencil_fail: *mut StencilFunction,
        back_stencil_func: *mut CompareFunction,
        back_stencil_z_pass: *mut StencilFunction,
        back_stencil_z_fail: *mut StencilFunction,
        back_stencil_fail: *mut StencilFunction,
    );

    /// GX2SetDepthStencilControlReg
    #[doc(alias = "GX2SetDepthStencilControlReg")]
    #[link_name = "GX2SetDepthStencilControlReg"]
    pub unsafe fn set_depth_stencil_control_reg(reg: *const DepthStencilControlReg);

    /// GX2SetDepthOnlyControl
    ///
    /// Same as [set_depth_stencil_control] with the stencil test disabled.
    #[doc(alias = "GX2SetDepthOnlyControl")]
    #[link_name = "GX2SetDepthOnlyControl"]
    pub unsafe fn set_depth_only_control(
        depth_test: ffi::c_bool,
        depth_write: ffi::c_bool,
        depth_compare: CompareFunction,
    );

    /// GX2SetPolygonControl
    #[doc(alias = "GX2SetPolygonControl")]
    #[link_name = "GX2SetPolygonControl"]
    pub unsafe fn set_polygon_control(
        front_face: FrontFace,
        cull_front: ffi::c_bool,
        cull_back: ffi::c_bool,
        polygon_modes: ffi::c_bool,
        polygon_mode_front: PolygonMode,
        polygon_mode_back: PolygonMode,
        poly_offset_front: ffi::c_bool,
        poly_offset_back: ffi::c_bool,
        point_line_offset: ffi::c_bool,
    );

    /// GX2InitPolygonControlReg
    #[doc(alias = "GX2InitPolygonControlReg")]
    #[link_name = "GX2InitPolygonControlReg"]
    pub unsafe fn init_polygon_control_reg(
        reg: *mut PolygonControlReg,
        front_face: FrontFace,
        cull_front: ffi::c_bool,
        cull_back: ffi::c_bool,
        polygon_modes: ffi::c_bool,
        polygon_mode_front: PolygonMode,
        polygon_mode_back: PolygonMode,
        poly_offset_front: ffi::c_bool,
        poly_offset_back: ffi::c_bool,
        point_line_offset: ffi::c_bool,
    );

    /// GX2GetPolygonControlReg
    #[doc(alias = "GX2GetPolygonControlReg")]
    #[link_name = "GX2GetPolygonControlReg"]
    pub unsafe fn get_polygon_control_reg(
        reg: *const PolygonControlReg,
        front_face: *mut FrontFace,
        cull_front: *mut ffi::c_bool,
        cull_back: *mut ffi::c_bool,
        polygon_modes: *mut ffi::c_bool,
        polygon_mode_front: *mut PolygonMode,
        polygon_mode_back: *mut PolygonMode,
        poly_offset_front: *mut ffi::c_bool,
        poly_offset_back: *mut ffi::c_bool,
        point_line_offset: *mut ffi::c_bool,
    );

    /// GX2SetPolygonControlReg
    #[doc(alias = "GX2SetPolygonControlReg")]
    #[link_name = "GX2SetPolygonControlReg"]
    pub unsafe fn set_polygon_control_reg(reg: *const PolygonControlReg);

    /// GX2SetCullOnlyControl
    ///
    /// Same as [set_polygon_control] with polygon modes and offsets disabled.
    #[doc(alias = "GX2SetCullOnlyControl")]
    #[link_name = "GX2SetCullOnlyControl"]
    pub unsafe fn set_cull_only_control(
        front_face: FrontFace,
        cull_front: ffi::c_bool,
        cull_back: ffi::c_bool,
    );

    /// GX2SetStencilMask
    #[doc(alias = "GX2SetStencilMask")]
    #[link_name = "GX2SetStencilMask"]
    pub unsafe fn set_stencil_mask(
        pre_mask_front: u8,
        write_mask_front: u8,
        ref_front: u8,
        pre_mask_back: u8,
        write_mask_back: u8,
        ref_back: u8,
    );

    /// GX2InitStencilMaskReg
    #[doc(alias = "GX2InitStencilMaskReg")]
    #[link_name = "GX2InitStencilMaskReg"]
    pub unsafe fn init_stencil_mask_reg(
        reg: *mut StencilMaskReg,
        pre_mask_front: u8,
        write_mask_front: u8,
        ref_front: u8,
        pre_mask_back: u8,
        write_mask_back: u8,
        ref_back: u8,
    );

    /// GX2GetStencilMaskReg
    #[doc(alias = "GX2GetStencilMaskReg")]
    #[link_name = "GX2GetStencilMaskReg"]
    pub unsafe fn get_stencil_mask_reg(
        reg: *const StencilMaskReg,
        pre_mask_front: *mut u8,
        write_mask_front: *mut u8,
        ref_front: *mut u8,
        pre_mask_back: *mut u8,
        write_mask_back: *mut u8,
        ref_back: *mut u8,
    );

    /// GX2SetStencilMaskReg
    #[doc(alias = "GX2SetStencilMaskReg")]
    #[link_name = "GX2SetStencilMaskReg"]
    pub unsafe fn set_stencil_mask_reg(reg: *const StencilMaskReg);

    /// GX2SetSpecialState
    #[doc(alias = "GX2SetSpecialState")]
    #[link_name = "GX2SetSpecialState"]
    pub unsafe fn set_special_state(state: SpecialState, enable: ffi::c_bool);

    /// GX2SetAlphaTest
    #[doc(alias = "GX2SetAlphaTest")]
    #[link_name = "GX2SetAlphaTest"]
    pub unsafe fn set_alpha_test(enable: ffi::c_bool, func: CompareFunction, reference: f32);

    /// GX2InitAlphaTestReg
    #[doc(alias = "GX2InitAlphaTestReg")]
    #[link_name = "GX2InitAlphaTestReg"]
    pub unsafe fn init_alpha_test_reg(
        reg: *mut AlphaTestReg,
        enable: ffi::c_bool,
        func: CompareFunction,
        reference: f32,
    );

    /// GX2GetAlphaTestReg
    #[doc(alias = "GX2GetAlphaTestReg")]
    #[link_name = "GX2GetAlphaTestReg"]
    pub unsafe fn get_alpha_test_reg(
        reg: *const AlphaTestReg,
        enable: *mut ffi::c_bool,
        func: *mut CompareFunction,
        reference: *mut f32,
    );

    /// GX2SetAlphaTestReg
    #[doc(alias = "GX2SetAlphaTestReg")]
    #[link_name = "GX2SetAlphaTestReg"]
    pub unsafe fn set_alpha_test_reg(reg: *const AlphaTestReg);

    /// GX2SetAlphaToMask
    #[doc(alias = "GX2SetAlphaToMask")]
    #[link_name = "GX2SetAlphaToMask"]
    pub unsafe fn set_alpha_to_mask(enable: ffi::c_bool, mode: AlphaToMaskMode);

    /// GX2InitAlphaToMaskReg
    #[doc(alias = "GX2InitAlphaToMaskReg")]
    #[link_name = "GX2InitAlphaToMaskReg"]
    pub unsafe fn init_alpha_to_mask_reg(
        reg: *mut AlphaToMaskReg,
        enable: ffi::c_bool,
        mode: AlphaToMaskMode,
    );

    /// GX2GetAlphaToMaskReg
    #[doc(alias = "GX2GetAlphaToMaskReg")]
    #[link_name = "GX2GetAlphaToMaskReg"]
    pub unsafe fn get_alpha_to_mask_reg(
        reg: *const AlphaToMaskReg,
        enable: *mut ffi::c_bool,
        mode: *mut AlphaToMaskMode,
    );

    /// GX2SetAlphaToMaskReg
    #[doc(alias = "GX2SetAlphaToMaskReg")]
    #[link_name = "GX2SetAlphaToMaskReg"]
    pub unsafe fn set_alpha_to_mask_reg(reg: *const AlphaToMaskReg);

    /// GX2SetPolygonOffset
    #[doc(alias = "GX2SetPolygonOffset")]
    #[link_name = "GX2SetPolygonOffset"]
    pub unsafe fn set_polygon_offset(
        front_offset: f32,
        front_slope_scale: f32,
        back_offset: f32,
        back_slope_scale: f32,
        clamp: f32,
    );

    /// GX2InitPolygonOffsetReg
    #[doc(alias = "GX2InitPolygonOffsetReg")]
    #[link_name = "GX2InitPolygonOffsetReg"]
    pub unsafe fn init_polygon_offset_reg(
        reg: *mut PolygonOffsetReg,
        front_offset: f32,
        front_slope_scale: f32,
        back_offset: f32,
        back_slope_scale: f32,
        clamp: f32,
    );

    /// GX2GetPolygonOffsetReg
    #[doc(alias = "GX2GetPolygonOffsetReg")]
    #[link_name = "GX2GetPolygonOffsetReg"]
    pub unsafe fn get_polygon_offset_reg(
        reg: *const PolygonOffsetReg,
        front_offset: *mut f32,
        front_slope_scale: *mut f32,
        back_offset: *mut f32,
        back_slope_scale: *mut f32,
        clamp: *mut f32,
    );

    /// GX2SetPolygonOffsetReg
    #[doc(alias = "GX2SetPolygonOffsetReg")]
    #[link_name = "GX2SetPolygonOffsetReg"]
    pub unsafe fn set_polygon_offset_reg(reg: *const PolygonOffsetReg);

    /// GX2SetTargetChannelMasks
    #[doc(alias = "GX2SetTargetChannelMasks")]
    #[link_name = "GX2SetTargetChannelMasks"]
    pub unsafe fn set_target_channel_masks(
        target0: ChannelMask,
        target1: ChannelMask,
        target2: ChannelMask,
        target3: ChannelMask,
        target4: ChannelMask,
        target5: ChannelMask,
        target6: ChannelMask,
        target7: ChannelMask,
    );

    /// GX2InitTargetChannelMasksReg
    #[doc(alias = "GX2InitTargetChannelMasksReg")]
    #[link_name = "GX2InitTargetChannelMasksReg"]
    pub unsafe fn init_target_channel_masks_reg(
        reg: *mut TargetChannelMaskReg,
        target0: ChannelMask,
        target1: ChannelMask,
        target2: ChannelMask,
        target3: ChannelMask,
        target4: ChannelMask,
        target5: ChannelMask,
        target6: ChannelMask,
        target7: ChannelMask,
    );

    /// GX2GetTargetChannelMasksReg
    #[doc(alias = "GX2GetTargetChannelMasksReg")]
    #[link_name = "GX2GetTargetChannelMasksReg"]
    pub unsafe fn get_target_channel_masks_reg(
        reg: *const TargetChannelMaskReg,
        target0: *mut ChannelMask,
        target1: *mut ChannelMask,
        target2: *mut ChannelMask,
        target3: *mut ChannelMask,
        target4: *mut ChannelMask,
        target5: *mut ChannelMask,
        target6: *mut ChannelMask,
        target7: *mut ChannelMask,
    );

    /// GX2SetTargetChannelMasksReg
    #[doc(alias = "GX2SetTargetChannelMasksReg")]
    #[link_name = "GX2SetTargetChannelMasksReg"]
    pub unsafe fn set_target_channel_masks_reg(reg: *const TargetChannelMaskReg);

    /// GX2SetAAMask
    #[doc(alias = "GX2SetAAMask")]
    #[link_name = "GX2SetAAMask"]
    pub unsafe fn set_aa_mask(upper_left: u8, upper_right: u8, lower_left: u8, lower_right: u8);

    /// GX2InitAAMaskReg
    #[doc(alias = "GX2InitAAMaskReg")]
    #[link_name = "GX2InitAAMaskReg"]
    pub unsafe fn init_aa_mask_reg(
        reg: *mut AaMaskReg,
        upper_left: u8,
        upper_right: u8,
        lower_left: u8,
        lower_right: u8,
    );

    /// GX2GetAAMaskReg
    #[doc(alias = "GX2GetAAMaskReg")]
    #[link_name = "GX2GetAAMaskReg"]
    pub unsafe fn get_aa_mask_reg(
        reg: *const AaMaskReg,
        upper_left: *mut u8,
        upper_right: *mut u8,
        lower_left: *mut u8,
        lower_right: *mut u8,
    );

    /// GX2SetAAMaskReg
    #[doc(alias = "GX2SetAAMaskReg")]
    #[link_name = "GX2SetAAMaskReg"]
    pub unsafe fn set_aa_mask_reg(reg: *const AaMaskReg);

    /// GX2SetColorControl
    ///
    /// `blend_enable_mask` has one bit per render target.
    #[doc(alias = "GX2SetColorControl")]
    #[link_name = "GX2SetColorControl"]
    pub unsafe fn set_color_control(
        logic_op: LogicOp,
        blend_enable_mask: u8,
        multi_write: ffi::c_bool,
        color_buffer: ffi::c_bool,
    );

    /// GX2InitColorControlReg
    #[doc(alias = "GX2InitColorControlReg")]
    #[link_name = "GX2InitColorControlReg"]
    pub unsafe fn init_color_control_reg(
        reg: *mut ColorControlReg,
        logic_op: LogicOp,
        blend_enable_mask: u8,
        multi_write: ffi::c_bool,
        color_buffer: ffi::c_bool,
    );

    /// GX2GetColorControlReg
    #[doc(alias = "GX2GetColorControlReg")]
    #[link_name = "GX2GetColorControlReg"]
    pub unsafe fn get_color_control_reg(
        reg: *const ColorControlReg,
        logic_op: *mut LogicOp,
        blend_enable_mask: *mut u8,
        multi_write: *mut ffi::c_bool,
        color_buffer: *mut ffi::c_bool,
    );

    /// GX2SetColorControlReg
    #[doc(alias = "GX2SetColorControlReg")]
    #[link_name = "GX2SetColorControlReg"]
    pub unsafe fn set_color_control_reg(reg: *const ColorControlReg);

    /// GX2SetBlendControl
    #[doc(alias = "GX2SetBlendControl")]
    #[link_name = "GX2SetBlendControl"]
    pub unsafe fn set_blend_control(
        target: RenderTarget,
        color_src_blend: BlendMode,
        color_dst_blend: BlendMode,
        color_combine: BlendCombineMode,
        separate_alpha_blend: ffi::c_bool,
        alpha_src_blend: BlendMode,
        alpha_dst_blend: BlendMode,
        alpha_combine: BlendCombineMode,
    );

    /// GX2InitBlendControlReg
    #[doc(alias = "GX2InitBlendControlReg")]
    #[link_name = "GX2InitBlendControlReg"]
    pub unsafe fn init_blend_control_reg(
        reg: *mut BlendControlReg,
        target: RenderTarget,
        color_src_blend: BlendMode,
        color_dst_blend: BlendMode,
        color_combine: BlendCombineMode,
        separate_alpha_blend: ffi::c_bool,
        alpha_src_blend: BlendMode,
        alpha_dst_blend: BlendMode,
        alpha_combine: BlendCombineMode,
    );

    /// GX2GetBlendControlReg
    #[doc(alias = "GX2GetBlendControlReg")]
    #[link_name = "GX2GetBlendControlReg"]
    pub unsafe fn get_blend_control_reg(
        reg: *const BlendControlReg,
        target: *mut RenderTarget,
        color_src_blend: *mut BlendMode,
        color_dst_blend: *mut BlendMode,
        color_combine: *mut BlendCombineMode,
        separate_alpha_blend: *mut ffi::c_bool,
        alpha_src_blend: *mut BlendMode,
        alpha_dst_blend: *mut BlendMode,
        alpha_combine: *mut BlendCombineMode,
    );

    /// GX2SetBlendControlReg
    #[doc(alias = "GX2SetBlendControlReg")]
    #[link_name = "GX2SetBlendControlReg"]
    pub unsafe fn set_blend_control_reg(reg: *const BlendControlReg);

    /// GX2SetBlendConstantColor
    #[doc(alias = "GX2SetBlendConstantColor")]
    #[link_name = "GX2SetBlendConstantColor"]
    pub unsafe fn set_blend_constant_color(red: f32, green: f32, blue: f32, alpha: f32);

    /// GX2InitBlendConstantColorReg
    #[doc(alias = "GX2InitBlendConstantColorReg")]
    #[link_name = "GX2InitBlendConstantColorReg"]
    pub unsafe fn init_blend_constant_color_reg(
        reg: *mut BlendConstantColorReg,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );

    /// GX2GetBlendConstantColorReg
    #[doc(alias = "GX2GetBlendConstantColorReg")]
    #[link_name = "GX2GetBlendConstantColorReg"]
    pub unsafe fn get_blend_constant_color_reg(
        reg: *const BlendConstantColorReg,
        red: *mut f32,
        green: *mut f32,
        blue: *mut f32,
        alpha: *mut f32,
    );

    /// GX2SetBlendConstantColorReg
    #[doc(alias = "GX2SetBlendConstantColorReg")]
    #[link_name = "GX2SetBlendConstantColorReg"]
    pub unsafe fn set_blend_constant_color_reg(reg: *const BlendConstantColorReg);

    /// GX2SetPointSize
    #[doc(alias = "GX2SetPointSize")]
    #[link_name = "GX2SetPointSize"]
    pub unsafe fn set_point_size(width: f32, height: f32);

    /// GX2InitPointSizeReg
    #[doc(alias = "GX2InitPointSizeReg")]
    #[link_name = "GX2InitPointSizeReg"]
    pub unsafe fn init_point_size_reg(reg: *mut PointSizeReg, width: f32, height: f32);

    /// GX2GetPointSizeReg
    #[doc(alias = "GX2GetPointSizeReg")]
    #[link_name = "GX2GetPointSizeReg"]
    pub unsafe fn get_point_size_reg(reg: *const PointSizeReg, width: *mut f32, height: *mut f32);

    /// GX2SetPointSizeReg
    #[doc(alias = "GX2SetPointSizeReg")]
    #[link_name = "GX2SetPointSizeReg"]
    pub unsafe fn set_point_size_reg(reg: *const PointSizeReg);

    /// GX2SetPointLimits
    #[doc(alias = "GX2SetPointLimits")]
    #[link_name = "GX2SetPointLimits"]
    pub unsafe fn set_point_limits(min: f32, max: f32);

    /// GX2InitPointLimitsReg
    #[doc(alias = "GX2InitPointLimitsReg")]
    #[link_name = "GX2InitPointLimitsReg"]
    pub unsafe fn init_point_limits_reg(reg: *mut PointLimitsReg, min: f32, max: f32);

    /// GX2GetPointLimitsReg
    #[doc(alias = "GX2GetPointLimitsReg")]
    #[link_name = "GX2GetPointLimitsReg"]
    pub unsafe fn get_point_limits_reg(reg: *const PointLimitsReg, min: *mut f32, max: *mut f32);

    /// GX2SetPointLimitsReg
    #[doc(alias = "GX2SetPointLimitsReg")]
    #[link_name = "GX2SetPointLimitsReg"]
    pub unsafe fn set_point_limits_reg(reg: *const PointLimitsReg);

    /// GX2SetLineWidth
    #[doc(alias = "GX2SetLineWidth")]
    #[link_name = "GX2SetLineWidth"]
    pub unsafe fn set_line_width(width: f32);

    /// GX2InitLineWidthReg
    #[doc(alias = "GX2InitLineWidthReg")]
    #[link_name = "GX2InitLineWidthReg"]
    pub unsafe fn init_line_width_reg(reg: *mut LineWidthReg, width: f32);

    /// GX2GetLineWidthReg
    #[doc(alias = "GX2GetLineWidthReg")]
    #[link_name = "GX2GetLineWidthReg"]
    pub unsafe fn get_line_width_reg(reg: *const LineWidthReg, width: *mut f32);

    /// GX2SetLineWidthReg
    #[doc(alias = "GX2SetLineWidthReg")]
    #[link_name = "GX2SetLineWidthReg"]
    pub unsafe fn set_line_width_reg(reg: *const LineWidthReg);

    /// GX2SetViewport
    #[doc(alias = "GX2SetViewport")]
    #[link_name = "GX2SetViewport"]
    pub unsafe fn set_viewport(x: f32, y: f32, width: f32, height: f32, near_z: f32, far_z: f32);

    /// GX2InitViewportReg
    #[doc(alias = "GX2InitViewportReg")]
    #[link_name = "GX2InitViewportReg"]
    pub unsafe fn init_viewport_reg(
        reg: *mut ViewportReg,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        near_z: f32,
        far_z: f32,
    );

    /// GX2GetViewportReg
    #[doc(alias = "GX2GetViewportReg")]
    #[link_name = "GX2GetViewportReg"]
    pub unsafe fn get_viewport_reg(
        reg: *const ViewportReg,
        x: *mut f32,
        y: *mut f32,
        width: *mut f32,
        height: *mut f32,
        near_z: *mut f32,
        far_z: *mut f32,
    );

    /// GX2SetViewportReg
    #[doc(alias = "GX2SetViewportReg")]
    #[link_name = "GX2SetViewportReg"]
    pub unsafe fn set_viewport_reg(reg: *const ViewportReg);

    /// GX2SetScissor
    #[doc(alias = "GX2SetScissor")]
    #[link_name = "GX2SetScissor"]
    pub unsafe fn set_scissor(x: u32, y: u32, width: u32, height: u32);

    /// GX2InitScissorReg
    #[doc(alias = "GX2InitScissorReg")]
    #[link_name = "GX2InitScissorReg"]
    pub unsafe fn init_scissor_reg(reg: *mut ScissorReg, x: u32, y: u32, width: u32, height: u32);

    /// GX2GetScissorReg
    #[doc(alias = "GX2GetScissorReg")]
    #[link_name = "GX2GetScissorReg"]
    pub unsafe fn get_scissor_reg(
        reg: *const ScissorReg,
        x: *mut u32,
        y: *mut u32,
        width: *mut u32,
        height: *mut u32,
    );

    /// GX2SetScissorReg
    #[doc(alias = "GX2SetScissorReg")]
    #[link_name = "GX2SetScissorReg"]
    pub unsafe fn set_scissor_reg(reg: *const ScissorReg);

    /// GX2SetRasterizerClipControl
    #[doc(alias = "GX2SetRasterizerClipControl")]
    #[link_name = "GX2SetRasterizerClipControl"]
    pub unsafe fn set_rasterizer_clip_control(rasterizer: ffi::c_bool, z_clip: ffi::c_bool);

    /// GX2SetRasterizerClipControlEx
    #[doc(alias = "GX2SetRasterizerClipControlEx")]
    #[link_name = "GX2SetRasterizerClipControlEx"]
    pub unsafe fn set_rasterizer_clip_control_ex(
        rasterizer: ffi::c_bool,
        z_clip: ffi::c_bool,
        half_z: ffi::c_bool,
    );

    /// GX2SetRasterizerClipControlHalfZ
    #[doc(alias = "GX2SetRasterizerClipControlHalfZ")]
    #[link_name = "GX2SetRasterizerClipControlHalfZ"]
    #[deprecated(note = "Use set_rasterizer_clip_control_ex instead")]
    pub unsafe fn set_rasterizer_clip_control_half_z(
        rasterizer: ffi::c_bool,
        z_clip: ffi::c_bool,
        half_z: ffi::c_bool,
    );
}
