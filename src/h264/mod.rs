//! H264

use crate::ffi;

pub const BUFFER_ALIGNMENT: u32 = 256;

/// H264Error
#[doc(alias = "H264Error")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const STREAM: Self = Self(0x01000000);
    pub const PROFILE: Self = Self(0x01080000);
    pub const NODATA: Self = Self(0x01100000);
    pub const NOTSUPPORTED: Self = Self(0x01180000);
    pub const PARAM: Self = Self(0x01010000);
    pub const OUTOFMEMORY: Self = Self(0x01020000);
    pub const INTERNAL: Self = Self(0x01030000);
    pub const SEQUENCE: Self = Self(0x01050000);
    pub const MEMSEGMENT: Self = Self(0x01060000);
    pub const MAX_SIZE: Self = Self(0x01070000);
    pub const BUFFEREMPTY: Self = Self(0x00080000);
    pub const FRAME_STORED: Self = Self(0x00000080);
    /// Lower 8 bits of the result of [execute]: [Error::FRAME_STORED] and the percentage (0-100) of decoded macroblocks.
    pub const STATUS_MASK: Self = Self(0x000000FF);
}

/// H264Parameter
#[doc(alias = "H264Parameter")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamId {
    /// `param` is an [OutputCallback].
    FptrOutput = 0x00000001,
    /// `param` points to a `u8` holding the [OutputPerFrame] value.
    OutputPerFrame = 0x20000002,
    /// `param` itself is passed back in [Output::user_memory].
    UserMemory = 0x70000001,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputPerFrame {
    /// Frames are buffered internally.
    #[default]
    Buffering = 0,
    /// The output callback is called as soon as a frame is emitted.
    Unbuffering = 1,
}

/// H264_*_PROFILE
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Profile {
    Baseline = 66,
    Main = 77,
    High = 100,
}

/// H264DecodedVuiParameters
#[doc(alias = "H264DecodedVuiParameters")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VuiParameters {
    pub aspect_ratio_info_present_flag: u8,
    pub aspect_ratio_idc: u8,
    pub sar_width: i16,
    pub sar_height: i16,
    pub overscan_info_present_flag: u8,
    pub overscan_appropriate_flag: u8,
    pub video_signal_type_present_flag: u8,
    pub video_format: u8,
    pub video_full_range_flag: u8,
    pub colour_description_present_flag: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub chroma_loc_info_present_flag: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub timing_info_present_flag: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub fixed_frame_rate_flag: u8,
    pub nal_hrd_parameters_present_flag: u8,
    pub vcl_hrd_parameters_present_flag: u8,
    pub low_delay_hrd_flag: u8,
    pub pic_struct_present_flag: u8,
    pub bitstream_restriction_flag: u8,
    pub motion_vectors_over_pic_boundaries_flag: u8,
    pub max_bytes_per_pic_denom: i16,
    pub max_bits_per_mb_denom: i16,
    pub log2_max_mv_length_horizontal: i16,
    pub log2_max_mv_length_vertical: i16,
    pub num_reorder_frames: i16,
    pub max_dec_frame_buffering: i16,
}

/// H264DecodeResult
#[doc(alias = "H264DecodeResult")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodeResult {
    pub status: Error,
    pub timestamp: f64,
    pub width: i32,
    pub height: i32,
    pub next_line: i32,
    pub crop_enable_flag: u8,
    pub top_crop: i32,
    pub bottom_crop: i32,
    pub left_crop: i32,
    pub right_crop: i32,
    pub pan_scan_enable_flag: u8,
    pub top_pan_scan: i32,
    pub bottom_pan_scan: i32,
    pub left_pan_scan: i32,
    pub right_pan_scan: i32,
    pub result: *mut ffi::c_void,
    pub vui_parameters_present_flag: u8,
    pub vui_parameters: *mut VuiParameters,
    pub reserved: ffi::unknown<0x28>,
}

/// H264DecodeOutput
#[doc(alias = "H264DecodeOutput")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Output {
    /// Number of entries in `results`.
    pub frame_count: i32,
    pub results: *mut *mut DecodeResult,
    pub user_memory: *mut ffi::c_void,
}

/// H264DECFptrOutputFn
///
/// Set with [ParamId::FptrOutput].
#[doc(alias = "H264DECFptrOutputFn")]
pub type OutputCallback = Option<unsafe extern "C" fn(output: *mut Output) -> ffi::c_int>;

unsafe extern "C" {
    /// H264DECInitParam
    #[doc(alias = "H264DECInitParam")]
    #[link_name = "H264DECInitParam"]
    pub unsafe fn init_param(size: i32, mem: *mut ffi::c_void) -> Error;

    /// H264DECSetParam
    #[doc(alias = "H264DECSetParam")]
    #[link_name = "H264DECSetParam"]
    pub unsafe fn set_param(mem: *mut ffi::c_void, id: ParamId, param: *const ffi::c_void) -> Error;

    /// H264DECOpen
    #[doc(alias = "H264DECOpen")]
    #[link_name = "H264DECOpen"]
    pub unsafe fn open(mem: *mut ffi::c_void) -> Error;

    /// H264DECBegin
    #[doc(alias = "H264DECBegin")]
    #[link_name = "H264DECBegin"]
    pub unsafe fn begin(mem: *mut ffi::c_void) -> Error;

    /// H264DECSetBitstream
    #[doc(alias = "H264DECSetBitstream")]
    #[link_name = "H264DECSetBitstream"]
    pub unsafe fn set_bitstream(
        mem: *mut ffi::c_void,
        bitstream: *const u8,
        length: i32,
        timestamp: f64,
    ) -> Error;

    /// H264DECExecute
    ///
    /// The upper bits of the result are an error code, the lower 8 bits the decode status ([Error::STATUS_MASK]). A fully decoded frame returns `0xE4`.
    #[doc(alias = "H264DECExecute")]
    #[link_name = "H264DECExecute"]
    pub unsafe fn execute(mem: *mut ffi::c_void, frame: *mut ffi::c_void) -> Error;

    /// H264DECFlush
    #[doc(alias = "H264DECFlush")]
    #[link_name = "H264DECFlush"]
    pub unsafe fn flush(mem: *mut ffi::c_void) -> Error;

    /// H264DECEnd
    #[doc(alias = "H264DECEnd")]
    #[link_name = "H264DECEnd"]
    pub unsafe fn end(mem: *mut ffi::c_void) -> Error;

    /// H264DECClose
    #[doc(alias = "H264DECClose")]
    #[link_name = "H264DECClose"]
    pub unsafe fn close(mem: *mut ffi::c_void) -> Error;

    /// H264DECMemoryRequirement
    ///
    /// `level` is the H.264 level multiplied by 10 (e.g. 41 for level 4.1). `max_width` must be in 64..=2800 and `max_height` in 64..=1408.
    #[doc(alias = "H264DECMemoryRequirement")]
    #[link_name = "H264DECMemoryRequirement"]
    pub unsafe fn memory_requirement(
        profile: Profile,
        level: i32,
        max_width: i32,
        max_height: i32,
        size: *mut i32,
    ) -> Error;

    /// H264DECGetImageSize
    #[doc(alias = "H264DECGetImageSize")]
    #[link_name = "H264DECGetImageSize"]
    pub unsafe fn get_image_size(
        buf: *const u8,
        total_bytes: i32,
        offset: i32,
        width: *mut i32,
        height: *mut i32,
    ) -> Error;

    /// H264DECFindDecstartpoint
    #[doc(alias = "H264DECFindDecstartpoint")]
    #[link_name = "H264DECFindDecstartpoint"]
    pub unsafe fn find_decode_start_point(
        buf: *const u8,
        total_bytes: i32,
        offset: *mut i32,
    ) -> Error;

    /// H264DECFindIdrpoint
    #[doc(alias = "H264DECFindIdrpoint")]
    #[link_name = "H264DECFindIdrpoint"]
    pub unsafe fn find_idr_point(buf: *const u8, total_bytes: i32, offset: *mut i32) -> Error;

    /// H264DECCheckDecunitLength
    #[doc(alias = "H264DECCheckDecunitLength")]
    #[link_name = "H264DECCheckDecunitLength"]
    pub unsafe fn check_decode_unit_length(
        mem: *mut ffi::c_void,
        buf: *const u8,
        total_bytes: i32,
        offset: i32,
        length: *mut i32,
    ) -> Error;

    /// H264DECCheckMemSegmentation
    ///
    /// Returns -1 if the memory crosses a 256 MB boundary, [Error::PARAM] for a null pointer or zero size.
    #[doc(alias = "H264DECCheckMemSegmentation")]
    #[link_name = "H264DECCheckMemSegmentation"]
    pub unsafe fn check_memory_segmentation(mem: *const ffi::c_void, size: u32) -> Error;

    /// H264DECCheckSkipableFrame
    #[doc(alias = "H264DECCheckSkipableFrame")]
    #[link_name = "H264DECCheckSkipableFrame"]
    pub unsafe fn check_skippable_frame(buf: *const u8, length: i32, skip: *mut i32) -> Error;

    /// H264DECSetParam_FPTR_OUTPUT
    ///
    /// Same as [set_param] with [ParamId::FptrOutput].
    #[doc(alias = "H264DECSetParam_FPTR_OUTPUT")]
    #[link_name = "H264DECSetParam_FPTR_OUTPUT"]
    pub unsafe fn set_param_output_callback(
        mem: *mut ffi::c_void,
        callback: OutputCallback,
    ) -> Error;

    /// H264DECSetParam_OUTPUT_PER_FRAME
    ///
    /// Same as [set_param] with [ParamId::OutputPerFrame].
    #[doc(alias = "H264DECSetParam_OUTPUT_PER_FRAME")]
    #[link_name = "H264DECSetParam_OUTPUT_PER_FRAME"]
    pub unsafe fn set_param_output_per_frame(mem: *mut ffi::c_void, mode: OutputPerFrame) -> Error;

    /// H264DECSetParam_USER_MEMORY
    ///
    /// Like [set_param] with [ParamId::UserMemory], but passes `*user_memory`.
    #[doc(alias = "H264DECSetParam_USER_MEMORY")]
    #[link_name = "H264DECSetParam_USER_MEMORY"]
    pub unsafe fn set_param_user_memory(
        mem: *mut ffi::c_void,
        user_memory: *mut *mut ffi::c_void,
    ) -> Error;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "h264",
    [
        "H264DECBegin",
        "H264DECCheckDecunitLength",
        "H264DECCheckMemSegmentation",
        "H264DECCheckSkipableFrame",
        "H264DECClose",
        "H264DECEnd",
        "H264DECExecute",
        "H264DECFindDecstartpoint",
        "H264DECFindIdrpoint",
        "H264DECFlush",
        "H264DECGetImageSize",
        "H264DECInitParam",
        "H264DECMemoryRequirement",
        "H264DECOpen",
        "H264DECSetBitstream",
        "H264DECSetParam",
        "H264DECSetParam_FPTR_OUTPUT",
        "H264DECSetParam_OUTPUT_PER_FRAME",
        "H264DECSetParam_USER_MEMORY"
    ],
    []
);
