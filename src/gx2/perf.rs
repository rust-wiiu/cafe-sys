//! GX2 performance counters
//!
//! Development hardware only.

use crate::ffi;

// TODO: replace with MEMAllocator from coreinit::mem
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemAllocator(pub ffi::unknown<0x10>);

/// GX2_INVALID_COUNTER_VALUE_U64
#[doc(alias = "GX2_INVALID_COUNTER_VALUE_U64")]
pub const INVALID_COUNTER_VALUE_U64: u64 = u64::MAX;
/// GX2_NUM_COUNTER_SQ
#[doc(alias = "GX2_NUM_COUNTER_SQ")]
pub const NUM_COUNTER_SQ: usize = 4;
/// GX2_NUM_COUNTER_PIPELINE
#[doc(alias = "GX2_NUM_COUNTER_PIPELINE")]
pub const NUM_COUNTER_PIPELINE: usize = 8;

/// GX2PerfTag
#[doc(alias = "GX2PerfTag")]
pub type Tag = u32;

/// GX2PerfTag2StringFunc
///
/// Returns the name of a tag for printing.
#[doc(alias = "GX2PerfTag2StringFunc")]
pub type Tag2StringFunc = Option<unsafe extern "C" fn(tag: Tag) -> *const ffi::c_char>;

/// _GX2CounterId
///
/// Low-level hardware counter. Counters with two instances (`_0`/`_1` suffix) are configured through instance 0.
#[doc(alias = "_GX2CounterId")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterId(pub u32);

impl CounterId {
    pub const CP_0: Self = Self(0x00);
    pub const GRBM_0: Self = Self(0x01);
    pub const GRBM_1: Self = Self(0x02);
    pub const PA_SU_0: Self = Self(0x03);
    pub const PA_SU_1: Self = Self(0x04);
    pub const PA_SU_2: Self = Self(0x05);
    pub const PA_SU_3: Self = Self(0x06);
    pub const PA_SC_0: Self = Self(0x07);
    pub const PA_SC_1: Self = Self(0x08);
    pub const PA_SC_2: Self = Self(0x09);
    pub const PA_SC_3: Self = Self(0x0a);
    pub const VGT_0: Self = Self(0x0b);
    pub const VGT_1: Self = Self(0x0c);
    pub const VGT_2: Self = Self(0x0d);
    pub const VGT_3: Self = Self(0x0e);
    pub const SQ_0: Self = Self(0x0f);
    pub const SQ_1: Self = Self(0x10);
    pub const SQ_2: Self = Self(0x11);
    pub const SQ_3: Self = Self(0x12);
    pub const SPI_0: Self = Self(0x13);
    pub const SPI_1: Self = Self(0x14);
    pub const SPI_2: Self = Self(0x15);
    pub const SPI_3: Self = Self(0x16);
    pub const SX_0: Self = Self(0x17);
    pub const SX_1: Self = Self(0x18);
    pub const SX_2: Self = Self(0x19);
    pub const SX_3: Self = Self(0x1a);
    pub const SMX_0: Self = Self(0x1b);
    pub const SMX_1: Self = Self(0x1c);
    pub const SMX_2: Self = Self(0x1d);
    pub const SMX_3: Self = Self(0x1e);
    pub const TD_0_0: Self = Self(0x1f);
    pub const TD_0_1: Self = Self(0x20);
    pub const TA_0_0: Self = Self(0x21);
    pub const TA_0_1: Self = Self(0x22);
    pub const TA_1_0: Self = Self(0x23);
    pub const TA_1_1: Self = Self(0x24);
    pub const TCP_0_0: Self = Self(0x25);
    pub const TCP_0_1: Self = Self(0x26);
    pub const TCP_1_0: Self = Self(0x27);
    pub const TCP_1_1: Self = Self(0x28);
    pub const TCP_2_0: Self = Self(0x29);
    pub const TCP_2_1: Self = Self(0x2a);
    pub const TCP_3_0: Self = Self(0x2b);
    pub const TCP_3_1: Self = Self(0x2c);
    pub const TCC_0_0: Self = Self(0x2d);
    pub const TCC_0_1: Self = Self(0x2e);
    pub const TCC_1_0: Self = Self(0x2f);
    pub const TCC_1_1: Self = Self(0x30);
    pub const TCC_2_0: Self = Self(0x31);
    pub const TCC_2_1: Self = Self(0x32);
    pub const TCC_3_0: Self = Self(0x33);
    pub const TCC_3_1: Self = Self(0x34);
    pub const CB_0_0: Self = Self(0x3d);
    pub const CB_0_1: Self = Self(0x3e);
    pub const CB_1_0: Self = Self(0x3f);
    pub const CB_1_1: Self = Self(0x40);
    pub const CB_2_0: Self = Self(0x41);
    pub const CB_2_1: Self = Self(0x42);
    pub const CB_3_0: Self = Self(0x43);
    pub const CB_3_1: Self = Self(0x44);
    pub const DB_0_0: Self = Self(0x35);
    pub const DB_0_1: Self = Self(0x36);
    pub const DB_1_0: Self = Self(0x37);
    pub const DB_1_1: Self = Self(0x38);
    pub const DB_2_0: Self = Self(0x39);
    pub const DB_2_1: Self = Self(0x3a);
    pub const DB_3_0: Self = Self(0x3b);
    pub const DB_3_1: Self = Self(0x3c);
    pub const PIPELINE: Self = Self(0x45);
}

/// _GX2StatId
///
/// Stat counted by a low-level counter, specific to the counter block.
#[doc(alias = "_GX2StatId")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatId(pub u32);

/// _GX2SqType
///
/// Shader type of an SQ counter.
#[doc(alias = "_GX2SqType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SqType(pub u32);

impl SqType {
    pub const PS: Self = Self(0x100);
    pub const VS: Self = Self(0x200);
    pub const GS: Self = Self(0x400);
    pub const ES: Self = Self(0x800);
}

/// GX2PerfMetric
#[doc(alias = "GX2PerfMetric")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Metric(pub u32);

impl Metric {
    pub const U64_TIME: Self = Self(0);
    pub const U64_GPU_TIME: Self = Self(1);
    pub const F32_GPU_BUSY: Self = Self(2);
    pub const F32_SHADER_BUSY: Self = Self(3);
    pub const U64_REUSED_INDICES_VS: Self = Self(4);
    pub const F32_SHADER_BUSY_VS: Self = Self(5);
    pub const F32_SHADER_BUSY_GS: Self = Self(6);
    pub const F32_SHADER_BUSY_PS: Self = Self(7);
    pub const F32_ALU_BUSY: Self = Self(8);
    pub const F32_TEX_BUSY: Self = Self(9);
    pub const U64_VS_VERTICES_IN: Self = Self(10);
    pub const F32_VS_TEX_INST_COUNT: Self = Self(11);
    pub const F32_VS_TEX_BUSY: Self = Self(12);
    pub const F32_VS_ALU_INST_COUNT: Self = Self(13);
    pub const F32_VS_ALU_BUSY: Self = Self(14);
    pub const F32_VS_ALU_EFFICIENCY: Self = Self(15);
    pub const F32_VS_ALU_TEX_RATIO: Self = Self(16);
    pub const F32_GS_TEX_INST_COUNT: Self = Self(17);
    pub const F32_GS_TEX_BUSY: Self = Self(18);
    pub const F32_GS_ALU_INST_COUNT: Self = Self(19);
    pub const F32_GS_ALU_BUSY: Self = Self(20);
    pub const F32_GS_ALU_EFFICIENCY: Self = Self(21);
    pub const F32_GS_ALU_TEX_RATIO: Self = Self(22);
    pub const F32_PRIMITIVE_ASSEMBLY_BUSY: Self = Self(23);
    pub const U64_PRIMITIVES_IN: Self = Self(24);
    pub const F32_PA_STALLED_ON_RASTERIZER: Self = Self(25);
    pub const F32_INTERP_BUSY: Self = Self(26);
    pub const U64_PS_PIXELS_IN: Self = Self(27);
    pub const F32_PS_TEX_INST_COUNT: Self = Self(28);
    pub const F32_PS_TEX_BUSY: Self = Self(29);
    pub const F32_PS_ALU_INST_COUNT: Self = Self(30);
    pub const F32_PS_ALU_BUSY: Self = Self(31);
    pub const F32_PS_ALU_EFFICIENCY: Self = Self(32);
    pub const F32_PS_ALU_TEX_RATIO: Self = Self(33);
    pub const U64_PS_PIXELS_OUT: Self = Self(34);
    pub const F32_PS_EXPORT_STALLS: Self = Self(35);
    pub const F32_TEX_UNIT_BUSY: Self = Self(36);
    pub const U64_TEXEL_FETCH_COUNT: Self = Self(37);
    pub const F32_TEX_CACHE_STALLED: Self = Self(38);
    pub const F32_TEX_MISS_RATE: Self = Self(39);
    pub const U64_TEX_MEM_BYTES_READ: Self = Self(40);
    pub const F32_DEPTH_STENCIL_TEST_BUSY: Self = Self(41);
    pub const F32_HIZ_TRIVIAL_ACCEPT: Self = Self(42);
    pub const F32_HIZ_REJECT: Self = Self(43);
    pub const U64_PRE_Z_SAMPLES_PASSING: Self = Self(44);
    pub const U64_PRE_Z_SAMPLES_FAILING_S: Self = Self(45);
    pub const U64_PRE_Z_SAMPLES_FAILING_Z: Self = Self(46);
    pub const U64_POST_Z_SAMPLES_PASSING: Self = Self(47);
    pub const U64_POST_Z_SAMPLES_FAILING_S: Self = Self(48);
    pub const U64_POST_Z_SAMPLES_FAILING_Z: Self = Self(49);
    pub const F32_Z_UNIT_STALLED: Self = Self(50);
    pub const U64_PIXELS_AT_CB: Self = Self(51);
    pub const U64_PIXELS_CB_MEM_WRITTEN: Self = Self(52);
    pub const U64_IA_VERTICES: Self = Self(53);
    pub const U64_IA_PRIMITIVES: Self = Self(54);
    pub const U64_VS_INVOCATIONS: Self = Self(55);
    pub const U64_GS_INVOCATIONS: Self = Self(56);
    pub const U64_GS_PRIMITIVES: Self = Self(57);
    pub const U64_C_INVOCATIONS: Self = Self(58);
    pub const U64_C_PRIMITIVES: Self = Self(59);
    pub const U64_PS_INVOCATIONS: Self = Self(60);
    pub const U64_PA_INPUT_PRIM: Self = Self(61);
}

/// GX2PerfMetricType
#[doc(alias = "GX2PerfMetricType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetricType(pub u32);

impl MetricType {
    pub const U64: Self = Self(0);
    pub const F32: Self = Self(1);
}

/// GX2PerfCollectionMethod
#[doc(alias = "GX2PerfCollectionMethod")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectionMethod(pub u32);

impl CollectionMethod {
    /// Collection off.
    pub const NONE: Self = Self(0);
    /// Results of all enabled tags are accumulated per pass.
    pub const TAGS_ACCUMULATE: Self = Self(1);
    /// Results are collected at each tag start and end.
    pub const TAGS_INDIVIDUAL: Self = Self(2);
}

/// GX2PerfType
#[doc(alias = "GX2PerfType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Type(pub u32);

impl Type {
    /// High-level [Metric].
    pub const GPU_METRIC: Self = Self(0);
    /// Low-level [StatId].
    pub const GPU_STAT: Self = Self(1);
    pub const MEM_STAT: Self = Self(2);
}

/// GX2MetricResult
///
/// Selected by the [MetricType] of the metric.
#[doc(alias = "GX2MetricResult")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union MetricResult {
    pub f32: f32,
    pub u64: u64,
}

/// GX2PerfData
///
/// Perf collection state, see [init].
#[doc(alias = "GX2PerfData")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Data(ffi::unknown<0x8A0>);

/// GX2CounterInfo
///
/// Low-level counter configuration followed by the results.
#[doc(alias = "GX2CounterInfo")]
#[repr(C, align(32))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterInfo {
    config: ffi::unknown<1184>,
    results: ffi::unknown<832>,
}

const _: () = {
    assert!(size_of::<MetricResult>() == 0x8);
    assert!(size_of::<Data>() == 0x8A0);
    assert!(size_of::<CounterInfo>() == 0x7E0);
    assert!(align_of::<CounterInfo>() == 32);
};

unsafe extern "C" {
    /// GX2GetPerfMetricType
    #[doc(alias = "GX2GetPerfMetricType", alias = "GX2TempGetPerfMetricType")]
    #[link_name = "GX2GetPerfMetricType"]
    pub unsafe fn get_metric_type(metric: Metric) -> MetricType;

    /// GX2PerfInit
    ///
    /// `max_tags` is the size of the tag list, allocated with `allocator`.
    #[doc(alias = "GX2PerfInit")]
    #[link_name = "GX2PerfInit"]
    pub unsafe fn init(data: *mut Data, max_tags: u32, allocator: *mut MemAllocator);

    /// GX2PerfFree
    #[doc(alias = "GX2PerfFree")]
    #[link_name = "GX2PerfFree"]
    pub unsafe fn free(data: *mut Data);

    /// GX2PerfSetCollectionMethod
    #[doc(alias = "GX2PerfSetCollectionMethod")]
    #[link_name = "GX2PerfSetCollectionMethod"]
    pub unsafe fn set_collection_method(data: *mut Data, method: CollectionMethod);

    /// GX2PerfGetCollectionMethod
    #[doc(alias = "GX2PerfGetCollectionMethod")]
    #[link_name = "GX2PerfGetCollectionMethod"]
    pub unsafe fn get_collection_method(data: *mut Data) -> CollectionMethod;

    /// GX2PerfMetricsClear
    #[doc(alias = "GX2PerfMetricsClear")]
    #[link_name = "GX2PerfMetricsClear"]
    pub unsafe fn metrics_clear(data: *mut Data);

    /// GX2PerfMetricEnable
    ///
    /// `id` is a [Metric] or [StatId], depending on `kind`.
    #[doc(alias = "GX2PerfMetricEnable")]
    #[link_name = "GX2PerfMetricEnable"]
    pub unsafe fn metric_enable(data: *mut Data, kind: Type, id: u32) -> ffi::c_bool;

    /// GX2PerfMetricIsEnabled
    #[doc(alias = "GX2PerfMetricIsEnabled")]
    #[link_name = "GX2PerfMetricIsEnabled"]
    pub unsafe fn metric_is_enabled(data: *mut Data, kind: Type, id: u32) -> ffi::c_bool;

    /// GX2PerfMetricGetEnabled
    ///
    /// Gets the `index`th enabled metric.
    #[doc(alias = "GX2PerfMetricGetEnabled")]
    #[link_name = "GX2PerfMetricGetEnabled"]
    pub unsafe fn metric_get_enabled(
        data: *mut Data,
        index: u32,
        kind: *mut Type,
        id: *mut u32,
    ) -> ffi::c_bool;

    /// GX2PerfTagEnable
    #[doc(alias = "GX2PerfTagEnable")]
    #[link_name = "GX2PerfTagEnable"]
    pub unsafe fn tag_enable(data: *mut Data, tag: Tag, enable: ffi::c_bool);

    /// GX2PerfTagEnableAll
    #[doc(alias = "GX2PerfTagEnableAll")]
    #[link_name = "GX2PerfTagEnableAll"]
    pub unsafe fn tag_enable_all(data: *mut Data);

    /// GX2PerfTagDisableAll
    #[doc(alias = "GX2PerfTagDisableAll")]
    #[link_name = "GX2PerfTagDisableAll"]
    pub unsafe fn tag_disable_all(data: *mut Data);

    /// GX2PerfTagIsEnabled
    #[doc(alias = "GX2PerfTagIsEnabled")]
    #[link_name = "GX2PerfTagIsEnabled"]
    pub unsafe fn tag_is_enabled(data: *mut Data, tag: Tag) -> ffi::c_bool;

    /// GX2PerfFrameStart
    #[doc(alias = "GX2PerfFrameStart")]
    #[link_name = "GX2PerfFrameStart"]
    pub unsafe fn frame_start(data: *mut Data);

    /// GX2PerfFrameEnd
    #[doc(alias = "GX2PerfFrameEnd")]
    #[link_name = "GX2PerfFrameEnd"]
    pub unsafe fn frame_end(data: *mut Data);

    /// GX2PerfGetNumPasses
    ///
    /// Number of passes needed to collect all enabled metrics.
    #[doc(alias = "GX2PerfGetNumPasses")]
    #[link_name = "GX2PerfGetNumPasses"]
    pub unsafe fn get_num_passes(data: *mut Data) -> u32;

    /// GX2PerfPassStart
    #[doc(alias = "GX2PerfPassStart")]
    #[link_name = "GX2PerfPassStart"]
    pub unsafe fn pass_start(data: *mut Data);

    /// GX2PerfPassEnd
    #[doc(alias = "GX2PerfPassEnd")]
    #[link_name = "GX2PerfPassEnd"]
    pub unsafe fn pass_end(data: *mut Data);

    /// GX2PerfTagStart
    #[doc(alias = "GX2PerfTagStart")]
    #[link_name = "GX2PerfTagStart"]
    pub unsafe fn tag_start(data: *mut Data, tag: Tag);

    /// GX2PerfTagEnd
    #[doc(alias = "GX2PerfTagEnd")]
    #[link_name = "GX2PerfTagEnd"]
    pub unsafe fn tag_end(data: *mut Data, tag: Tag);

    /// GX2PerfGetResultByFrame
    #[doc(alias = "GX2PerfGetResultByFrame")]
    #[link_name = "GX2PerfGetResultByFrame"]
    pub unsafe fn get_result_by_frame(
        data: *mut Data,
        kind: Type,
        id: u32,
        result: *mut MetricResult,
    ) -> ffi::c_bool;

    /// GX2PerfGetResultByTagId
    ///
    /// `number` is the occurrence of `tag`.
    #[doc(alias = "GX2PerfGetResultByTagId")]
    #[link_name = "GX2PerfGetResultByTagId"]
    pub unsafe fn get_result_by_tag_id(
        data: *mut Data,
        kind: Type,
        id: u32,
        tag: Tag,
        number: u32,
        result: *mut MetricResult,
    ) -> ffi::c_bool;

    /// GX2PerfGetResultByTagSequence
    ///
    /// Gets the result of the `sequence`th tag occurrence along with its tag, occurrence number and nesting depth.
    #[doc(alias = "GX2PerfGetResultByTagSequence")]
    #[link_name = "GX2PerfGetResultByTagSequence"]
    pub unsafe fn get_result_by_tag_sequence(
        data: *mut Data,
        kind: Type,
        id: u32,
        sequence: u32,
        tag: *mut Tag,
        number: *mut u32,
        depth: *mut u32,
        result: *mut MetricResult,
    ) -> ffi::c_bool;

    /// GX2PerfPrintFrameResults
    #[doc(alias = "GX2PerfPrintFrameResults")]
    #[link_name = "GX2PerfPrintFrameResults"]
    pub unsafe fn print_frame_results(data: *mut Data);

    /// GX2PerfPrintTagResults
    #[doc(alias = "GX2PerfPrintTagResults")]
    #[link_name = "GX2PerfPrintTagResults"]
    pub unsafe fn print_tag_results(data: *mut Data, tag_to_string: Tag2StringFunc);

    /// GX2PerfSetPassCoherEnable
    #[doc(alias = "GX2PerfSetPassCoherEnable")]
    #[link_name = "GX2PerfSetPassCoherEnable"]
    pub unsafe fn set_pass_coher_enable(data: *mut Data, enable: ffi::c_bool);

    /// GX2PerfGetPassCoherEnable
    #[doc(alias = "GX2PerfGetPassCoherEnable")]
    #[link_name = "GX2PerfGetPassCoherEnable"]
    pub unsafe fn get_pass_coher_enable(data: *mut Data) -> ffi::c_bool;

    /// GX2GetGPUSystemClock
    #[doc(alias = "GX2GetGPUSystemClock")]
    #[link_name = "GX2GetGPUSystemClock"]
    pub unsafe fn get_gpu_system_clock() -> u32;

    /// GX2ResetCounterInfo
    #[doc(alias = "GX2ResetCounterInfo")]
    #[link_name = "GX2ResetCounterInfo"]
    pub unsafe fn reset_counter_info(info: *mut CounterInfo);

    /// GX2InitCounterInfo
    ///
    /// Configures `counter` to count `stat`. Fails if the counter is already configured. Do not call between [start_counters] and [stop_counters].
    #[doc(alias = "GX2InitCounterInfo")]
    #[link_name = "GX2InitCounterInfo"]
    pub unsafe fn init_counter_info(
        info: *mut CounterInfo,
        counter: CounterId,
        stat: StatId,
    ) -> ffi::c_bool;

    /// GX2GetCounterSetting
    ///
    /// Returns false if `counter` is not configured.
    #[doc(alias = "GX2GetCounterSetting")]
    #[link_name = "GX2GetCounterSetting"]
    pub unsafe fn get_counter_setting(
        info: *mut CounterInfo,
        counter: CounterId,
        stat: *mut StatId,
    ) -> ffi::c_bool;

    /// GX2GetCounterResultSize
    ///
    /// Size of a separate result buffer for [sample_counters_ex], a multiple of 32. The buffer must be 32-byte aligned.
    #[doc(alias = "GX2GetCounterResultSize")]
    #[link_name = "GX2GetCounterResultSize"]
    pub unsafe fn get_counter_result_size(info: *mut CounterInfo) -> u32;

    /// GX2SetCounterInfo
    #[doc(alias = "GX2SetCounterInfo")]
    #[link_name = "GX2SetCounterInfo"]
    pub unsafe fn set_counter_info(info: *const CounterInfo);

    /// GX2SampleCounters
    #[doc(alias = "GX2SampleCounters")]
    #[link_name = "GX2SampleCounters"]
    pub unsafe fn sample_counters(info: *mut CounterInfo);

    /// GX2SampleCountersEx
    ///
    /// Same as [sample_counters], but into a separate 32-byte aligned buffer.
    #[doc(alias = "GX2SampleCountersEx")]
    #[link_name = "GX2SampleCountersEx"]
    pub unsafe fn sample_counters_ex(info: *mut CounterInfo, results: *mut u64);

    /// GX2GetCounterResult
    #[doc(alias = "GX2GetCounterResult")]
    #[link_name = "GX2GetCounterResult"]
    pub unsafe fn get_counter_result(info: *mut CounterInfo, stat: StatId) -> u64;

    /// GX2GetCounterResultEx
    ///
    /// Same as [get_counter_result], but from a buffer filled by [sample_counters_ex].
    #[doc(alias = "GX2GetCounterResultEx")]
    #[link_name = "GX2GetCounterResultEx"]
    pub unsafe fn get_counter_result_ex(
        info: *mut CounterInfo,
        results: *mut u64,
        stat: StatId,
    ) -> u64;

    /// GX2StartCounters
    #[doc(alias = "GX2StartCounters")]
    #[link_name = "GX2StartCounters"]
    pub unsafe fn start_counters(info: *const CounterInfo);

    /// GX2StopCounters
    #[doc(alias = "GX2StopCounters")]
    #[link_name = "GX2StopCounters"]
    pub unsafe fn stop_counters(info: *const CounterInfo);

    /// GX2ResetCounters
    #[doc(alias = "GX2ResetCounters")]
    #[link_name = "GX2ResetCounters"]
    pub unsafe fn reset_counters(info: *const CounterInfo);
}

/// Deprecated counter API, replaced by the functions in [perf][super].
pub mod legacy {
    use super::{CounterId, Metric, NUM_COUNTER_PIPELINE, NUM_COUNTER_SQ, SqType, StatId};
    use crate::ffi;

    const COUNTER_COUNT: usize = CounterId::PIPELINE.0 as usize + 1;
    const RESULT_SIZE: usize = CounterId::PIPELINE.0 as usize + 2 * NUM_COUNTER_PIPELINE;

    /// _GX2CounterInfo
    #[doc(alias = "_GX2CounterInfo")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CounterInfo {
        pub results: [u64; RESULT_SIZE],
        _padding: ffi::unknown<64>,
        pub enabled: [ffi::c_bool; COUNTER_COUNT],
        pub stats: [StatId; COUNTER_COUNT],
        pub sq_type: [SqType; NUM_COUNTER_SQ],
    }

    /// GX2PerfInfo
    #[doc(alias = "GX2PerfInfo")]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PerfInfo {
        pub counter_info: CounterInfo,
        /// Number of used counters per block: CP, GRBM, PA_SU, VGT, SQ, SPI, SX, TA, TCP, TCC, DB, CB.
        pub used_counts: [u8; 12],
        /// Counter indices of the individual stats used by the metrics.
        pub indices: [u8; 71],
    }

    const _: () = {
        assert!(size_of::<CounterInfo>() == 0x528);
        assert!(size_of::<PerfInfo>() == 0x580);
    };

    unsafe extern "C" {
        /// _GX2InitCounterInfo
        #[doc(alias = "_GX2InitCounterInfo")]
        #[link_name = "_GX2InitCounterInfo"]
        #[deprecated(note = "Use perf::init_counter_info instead")]
        pub unsafe fn init_counter_info(info: *mut CounterInfo, counter: CounterId, stat: StatId);

        /// _GX2SetCounterInfo
        #[doc(alias = "_GX2SetCounterInfo")]
        #[link_name = "_GX2SetCounterInfo"]
        #[deprecated(note = "Use perf::set_counter_info instead")]
        pub unsafe fn set_counter_info(info: *const CounterInfo);

        /// _GX2ResetCounters
        #[doc(alias = "_GX2ResetCounters")]
        #[link_name = "_GX2ResetCounters"]
        #[deprecated(note = "Use perf::reset_counters instead")]
        pub unsafe fn reset_counters();

        /// _GX2StartCounters
        #[doc(alias = "_GX2StartCounters")]
        #[link_name = "_GX2StartCounters"]
        #[deprecated(note = "Use perf::start_counters instead")]
        pub unsafe fn start_counters(info: *const CounterInfo);

        /// _GX2StopCounters
        #[doc(alias = "_GX2StopCounters")]
        #[link_name = "_GX2StopCounters"]
        #[deprecated(note = "Use perf::stop_counters instead")]
        pub unsafe fn stop_counters(info: *const CounterInfo);

        /// _GX2SampleCounters
        #[doc(alias = "_GX2SampleCounters")]
        #[link_name = "_GX2SampleCounters"]
        #[deprecated(note = "Use perf::sample_counters instead")]
        pub unsafe fn sample_counters(info: *mut CounterInfo);

        /// GX2InitPerfMetric
        #[doc(alias = "GX2InitPerfMetric")]
        #[link_name = "GX2InitPerfMetric"]
        #[deprecated(note = "Use perf::metric_enable instead")]
        pub unsafe fn init_perf_metric(info: *mut PerfInfo, metric: Metric) -> ffi::c_bool;

        /// GX2GetPerfMetricU64
        #[doc(alias = "GX2GetPerfMetricU64")]
        #[link_name = "GX2GetPerfMetricU64"]
        #[deprecated(note = "Use perf::get_result_by_frame instead")]
        pub unsafe fn get_perf_metric_u64(info: *const PerfInfo, metric: Metric, result: *mut u64);

        /// GX2GetPerfMetricF32
        #[doc(alias = "GX2GetPerfMetricF32")]
        #[link_name = "GX2GetPerfMetricF32"]
        #[deprecated(note = "Use perf::get_result_by_frame instead")]
        pub unsafe fn get_perf_metric_f32(info: *const PerfInfo, metric: Metric, result: *mut f32);
    }
}
