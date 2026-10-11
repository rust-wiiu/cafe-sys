//! GX2 queries and GPU timing

use crate::ffi;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryType(pub u32);

impl QueryType {
    /// Occlusion query read by the CPU.
    pub const OCCLUSION_CPU: Self = Self(0);
    pub const STREAM_OUT_STATS_CPU: Self = Self(1);
    /// Occlusion query used by the GPU for conditional rendering.
    pub const OCCLUSION_GPU: Self = Self(2);
    pub const STREAM_OUT_STATS_GPU: Self = Self(3);
}

/// Written by the GPU, little-endian.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryInfo {
    pub reg: [u64; 8],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamOutStats {
    pub primitives_written: u64,
    pub storage_needed: u64,
}

const _: () = {
    assert!(size_of::<QueryInfo>() == 0x40);
    assert!(size_of::<StreamOutStats>() == 0x10);
};

unsafe extern "C" {
    /// GX2QueryBegin
    #[doc(alias = "GX2QueryBegin")]
    #[link_name = "GX2QueryBegin"]
    pub unsafe fn begin(kind: QueryType, info: *mut QueryInfo);

    /// GX2QueryEnd
    #[doc(alias = "GX2QueryEnd")]
    #[link_name = "GX2QueryEnd"]
    pub unsafe fn end(kind: QueryType, info: *mut QueryInfo);

    /// GX2QueryGetOcclusionResult
    ///
    /// Returns false if the GPU has not written the result yet.
    #[doc(alias = "GX2QueryGetOcclusionResult")]
    #[link_name = "GX2QueryGetOcclusionResult"]
    pub unsafe fn get_occlusion_result(info: *const QueryInfo, z_pass: *mut u64) -> ffi::c_bool;

    /// GX2QueryGetStreamOutStatsResult
    ///
    /// Returns false if the GPU has not written the result yet.
    #[doc(alias = "GX2QueryGetStreamOutStatsResult")]
    #[link_name = "GX2QueryGetStreamOutStatsResult"]
    pub unsafe fn get_stream_out_stats_result(
        info: *const QueryInfo,
        stats: *mut StreamOutStats,
    ) -> ffi::c_bool;

    /// GX2QueryBeginConditionalRender
    ///
    /// Following draws are skipped depending on the GPU query result and `predicate`. `kind` must be a GPU query type.
    #[doc(alias = "GX2QueryBeginConditionalRender")]
    #[link_name = "GX2QueryBeginConditionalRender"]
    pub unsafe fn begin_conditional_render(
        kind: QueryType,
        info: *const QueryInfo,
        hint: ffi::c_bool,
        predicate: ffi::c_bool,
    );

    /// GX2QueryEndConditionalRender
    #[doc(alias = "GX2QueryEndConditionalRender")]
    #[link_name = "GX2QueryEndConditionalRender"]
    pub unsafe fn end_conditional_render();

    /// GX2CPUTimeToGPUTime
    #[doc(alias = "GX2CPUTimeToGPUTime")]
    #[link_name = "GX2CPUTimeToGPUTime"]
    pub unsafe fn cpu_time_to_gpu_time(cpu_time: u64) -> u64;

    /// GX2GPUTimeToCPUTime
    #[doc(alias = "GX2GPUTimeToCPUTime")]
    #[link_name = "GX2GPUTimeToCPUTime"]
    pub unsafe fn gpu_time_to_cpu_time(gpu_time: u64) -> u64;

    /// GX2SampleTopGPUCycle
    ///
    /// Has the GPU write its cycle counter to `addr` at the top of the pipeline. `*addr` is set to all ones until then.
    #[doc(alias = "GX2SampleTopGPUCycle")]
    #[link_name = "GX2SampleTopGPUCycle"]
    pub unsafe fn sample_top_gpu_cycle(addr: *mut u64);

    /// GX2SampleBottomGPUCycle
    ///
    /// Same as [sample_top_gpu_cycle], but at the bottom of the pipeline.
    #[doc(alias = "GX2SampleBottomGPUCycle")]
    #[link_name = "GX2SampleBottomGPUCycle"]
    pub unsafe fn sample_bottom_gpu_cycle(addr: *mut u64);
}
