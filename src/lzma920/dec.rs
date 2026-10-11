//! LZMA decoder

use super::{Alloc, Error, FinishMode, Status};
use crate::ffi;

/// LZMA_REQUIRED_INPUT_MAX
#[doc(alias = "LZMA_REQUIRED_INPUT_MAX")]
pub const REQUIRED_INPUT_MAX: usize = 20;

/// CLzmaProps
#[doc(alias = "CLzmaProps")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Props {
    pub lc: ffi::c_uint,
    pub lp: ffi::c_uint,
    pub pb: ffi::c_uint,
    pub dic_size: u32,
}

/// CLzmaDec
///
/// `dic` and `probs` must be null before the first allocation.
#[doc(alias = "CLzmaDec")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoder {
    pub prop: Props,
    pub probs: *mut u16,
    pub dic: *mut u8,
    pub buf: *const u8,
    pub range: u32,
    pub code: u32,
    pub dic_pos: usize,
    pub dic_buf_size: usize,
    pub processed_pos: u32,
    pub check_dic_size: u32,
    pub state: ffi::c_uint,
    pub reps: [u32; 4],
    pub remain_len: ffi::c_uint,
    pub need_flush: ffi::c_int,
    pub need_init_state: ffi::c_int,
    pub num_probs: u32,
    pub temp_buf_size: ffi::c_uint,
    pub temp_buf: [u8; REQUIRED_INPUT_MAX],
}

unsafe extern "C" {
    /// LzmaProps_Decode
    #[doc(alias = "LzmaProps_Decode")]
    #[link_name = "LzmaProps_Decode"]
    pub unsafe fn props_decode(props: *mut Props, data: *const u8, size: ffi::c_uint) -> Error;

    /// LzmaDec_Init
    #[doc(alias = "LzmaDec_Init")]
    #[link_name = "LzmaDec_Init"]
    pub unsafe fn init(decoder: *mut Decoder);

    /// LzmaDec_AllocateProbs
    ///
    /// Allocates only the probabilities, the dictionary must be set manually.
    #[doc(alias = "LzmaDec_AllocateProbs")]
    #[link_name = "LzmaDec_AllocateProbs"]
    pub unsafe fn allocate_probs(
        decoder: *mut Decoder,
        props: *const u8,
        props_size: ffi::c_uint,
        alloc: *mut Alloc,
    ) -> Error;

    /// LzmaDec_FreeProbs
    #[doc(alias = "LzmaDec_FreeProbs")]
    #[link_name = "LzmaDec_FreeProbs"]
    pub unsafe fn free_probs(decoder: *mut Decoder, alloc: *mut Alloc);

    /// LzmaDec_Allocate
    #[doc(alias = "LzmaDec_Allocate")]
    #[link_name = "LzmaDec_Allocate"]
    pub unsafe fn allocate(
        decoder: *mut Decoder,
        props: *const u8,
        props_size: ffi::c_uint,
        alloc: *mut Alloc,
    ) -> Error;

    /// LzmaDec_Free
    #[doc(alias = "LzmaDec_Free")]
    #[link_name = "LzmaDec_Free"]
    pub unsafe fn free(decoder: *mut Decoder, alloc: *mut Alloc);

    /// LzmaDec_DecodeToDic
    ///
    /// Decodes into [Decoder::dic]. The caller must reset [Decoder::dic_pos] once it reaches [Decoder::dic_buf_size].
    #[doc(alias = "LzmaDec_DecodeToDic")]
    #[link_name = "LzmaDec_DecodeToDic"]
    pub unsafe fn decode_to_dic(
        decoder: *mut Decoder,
        dic_limit: usize,
        src: *const u8,
        src_len: *mut usize,
        finish_mode: FinishMode,
        status: *mut Status,
    ) -> Error;

    /// LzmaDec_DecodeToBuf
    #[doc(alias = "LzmaDec_DecodeToBuf")]
    #[link_name = "LzmaDec_DecodeToBuf"]
    pub unsafe fn decode_to_buf(
        decoder: *mut Decoder,
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: *mut usize,
        finish_mode: FinishMode,
        status: *mut Status,
    ) -> Error;

    /// LzmaDecode
    #[doc(alias = "LzmaDecode")]
    #[link_name = "LzmaDecode"]
    pub unsafe fn decode(
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: *mut usize,
        props: *const u8,
        props_size: ffi::c_uint,
        finish_mode: FinishMode,
        status: *mut Status,
        alloc: *mut Alloc,
    ) -> Error;
}
