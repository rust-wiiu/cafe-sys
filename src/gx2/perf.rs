//! GX2 performance counters

use crate::coreinit::mem::allocator::Allocator;
use crate::ffi;

pub const INVALID_COUNTER_VALUE_U64: u64 = u64::MAX;
pub const NUM_COUNTER_SQ: usize = 4;
pub const NUM_COUNTER_PIPELINE: usize = 8;

pub type Tag = u32;

/// Returns the name of a tag for printing.
pub type Tag2StringFunc = Option<unsafe extern "C" fn(tag: Tag) -> *const ffi::c_char>;

/// Low-level hardware counter.
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CounterId {
    #[default]
    Cp0 = 0x00,
    Grbm0 = 0x01,
    Grbm1 = 0x02,
    PaSu0 = 0x03,
    PaSu1 = 0x04,
    PaSu2 = 0x05,
    PaSu3 = 0x06,
    PaSc0 = 0x07,
    PaSc1 = 0x08,
    PaSc2 = 0x09,
    PaSc3 = 0x0a,
    Vgt0 = 0x0b,
    Vgt1 = 0x0c,
    Vgt2 = 0x0d,
    Vgt3 = 0x0e,
    Sq0 = 0x0f,
    Sq1 = 0x10,
    Sq2 = 0x11,
    Sq3 = 0x12,
    Spi0 = 0x13,
    Spi1 = 0x14,
    Spi2 = 0x15,
    Spi3 = 0x16,
    Sx0 = 0x17,
    Sx1 = 0x18,
    Sx2 = 0x19,
    Sx3 = 0x1a,
    Smx0 = 0x1b,
    Smx1 = 0x1c,
    Smx2 = 0x1d,
    Smx3 = 0x1e,
    Td00 = 0x1f,
    Td01 = 0x20,
    Ta00 = 0x21,
    Ta01 = 0x22,
    Ta10 = 0x23,
    Ta11 = 0x24,
    Tcp00 = 0x25,
    Tcp01 = 0x26,
    Tcp10 = 0x27,
    Tcp11 = 0x28,
    Tcp20 = 0x29,
    Tcp21 = 0x2a,
    Tcp30 = 0x2b,
    Tcp31 = 0x2c,
    Tcc00 = 0x2d,
    Tcc01 = 0x2e,
    Tcc10 = 0x2f,
    Tcc11 = 0x30,
    Tcc20 = 0x31,
    Tcc21 = 0x32,
    Tcc30 = 0x33,
    Tcc31 = 0x34,
    Cb00 = 0x3d,
    Cb01 = 0x3e,
    Cb10 = 0x3f,
    Cb11 = 0x40,
    Cb20 = 0x41,
    Cb21 = 0x42,
    Cb30 = 0x43,
    Cb31 = 0x44,
    Db00 = 0x35,
    Db01 = 0x36,
    Db10 = 0x37,
    Db11 = 0x38,
    Db20 = 0x39,
    Db21 = 0x3a,
    Db30 = 0x3b,
    Db31 = 0x3c,
    Pipeline = 0x45,
}

/// Stat counted by a low-level counter, specific to the counter block.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatId(pub u32);

/// Shader type of an SQ counter.
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
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metric {
    #[default]
    U64Time = 0,
    U64GpuTime = 1,
    F32GpuBusy = 2,
    F32ShaderBusy = 3,
    U64ReusedIndicesVs = 4,
    F32ShaderBusyVs = 5,
    F32ShaderBusyGs = 6,
    F32ShaderBusyPs = 7,
    F32AluBusy = 8,
    F32TexBusy = 9,
    U64VsVerticesIn = 10,
    F32VsTexInstCount = 11,
    F32VsTexBusy = 12,
    F32VsAluInstCount = 13,
    F32VsAluBusy = 14,
    F32VsAluEfficiency = 15,
    F32VsAluTexRatio = 16,
    F32GsTexInstCount = 17,
    F32GsTexBusy = 18,
    F32GsAluInstCount = 19,
    F32GsAluBusy = 20,
    F32GsAluEfficiency = 21,
    F32GsAluTexRatio = 22,
    F32PrimitiveAssemblyBusy = 23,
    U64PrimitivesIn = 24,
    F32PaStalledOnRasterizer = 25,
    F32InterpBusy = 26,
    U64PsPixelsIn = 27,
    F32PsTexInstCount = 28,
    F32PsTexBusy = 29,
    F32PsAluInstCount = 30,
    F32PsAluBusy = 31,
    F32PsAluEfficiency = 32,
    F32PsAluTexRatio = 33,
    U64PsPixelsOut = 34,
    F32PsExportStalls = 35,
    F32TexUnitBusy = 36,
    U64TexelFetchCount = 37,
    F32TexCacheStalled = 38,
    F32TexMissRate = 39,
    U64TexMemBytesRead = 40,
    F32DepthStencilTestBusy = 41,
    F32HizTrivialAccept = 42,
    F32HizReject = 43,
    U64PreZSamplesPassing = 44,
    U64PreZSamplesFailingS = 45,
    U64PreZSamplesFailingZ = 46,
    U64PostZSamplesPassing = 47,
    U64PostZSamplesFailingS = 48,
    U64PostZSamplesFailingZ = 49,
    F32ZUnitStalled = 50,
    U64PixelsAtCb = 51,
    U64PixelsCbMemWritten = 52,
    U64IaVertices = 53,
    U64IaPrimitives = 54,
    U64VsInvocations = 55,
    U64GsInvocations = 56,
    U64GsPrimitives = 57,
    U64CInvocations = 58,
    U64CPrimitives = 59,
    U64PsInvocations = 60,
    U64PaInputPrim = 61,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetricType(pub u32);

impl MetricType {
    pub const U64: Self = Self(0);
    pub const F32: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectionMethod(pub u32);

impl CollectionMethod {
    /// Collection off.
    pub const NONE: Self = Self(0);
    /// Results of all enabled tags are accumulated.
    pub const TAGS_ACCUMULATE: Self = Self(1);
    /// Results are collected per tag.
    pub const TAGS_INDIVIDUAL: Self = Self(2);
}

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

/// Selected by the [MetricType] of the metric.
#[repr(C)]
#[derive(Clone, Copy)]
pub union MetricResult {
    pub f32: f32,
    pub u64: u64,
}

/// Perf collection state, see [init].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Data(ffi::unknown<0x8A0>);

/// Low-level counter configuration followed by the results.
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
    #[doc(alias = "GX2GetPerfMetricType")]
    #[link_name = "GX2GetPerfMetricType"]
    pub unsafe fn get_metric_type(metric: Metric) -> MetricType;

    /// GX2PerfInit
    ///
    /// `max_tags` is the size of the tag list, allocated with `allocator`.
    #[doc(alias = "GX2PerfInit")]
    #[link_name = "GX2PerfInit"]
    pub unsafe fn init(data: *mut Data, max_tags: u32, allocator: *mut Allocator);

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
    /// `number` is the occurrence of `tag`. Returns false unless the collection method is [CollectionMethod::TAGS_INDIVIDUAL].
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
    /// Configures `counter` to count `stat`.
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
    /// Size of a separate result buffer for [sample_counters_ex], a multiple of 32.
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
    /// Same as [sample_counters], but into a separate buffer.
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

/// Older counter API.
pub mod legacy {
    use super::{CounterId, Metric, NUM_COUNTER_PIPELINE, NUM_COUNTER_SQ, SqType, StatId};
    use crate::ffi;

    const COUNTER_COUNT: usize = CounterId::Pipeline as usize + 1;
    const RESULT_SIZE: usize = CounterId::Pipeline as usize + 2 * NUM_COUNTER_PIPELINE;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CounterInfo {
        pub results: [u64; RESULT_SIZE],
        _padding: ffi::unknown<64>,
        pub enabled: [ffi::c_bool; COUNTER_COUNT],
        pub stats: [StatId; COUNTER_COUNT],
        pub sq_type: [SqType; NUM_COUNTER_SQ],
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PerfInfo {
        pub counter_info: CounterInfo,
        /// Number of used counters per block.
        pub used_counts: [u8; 12],
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
