//! LZMA2 decoder

use crate::{
    ffi,
    lzma920::{Alloc, Error, FinishMode, Status, dec},
};

/// CLzma2Dec
///
/// Free with [dec::free] or [dec::free_probs] on [Decoder::decoder].
#[doc(alias = "CLzma2Dec")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoder {
    pub decoder: dec::Decoder,
    pub pack_size: u32,
    pub unpack_size: u32,
    pub control: u8,
    pub need_init_dic: ffi::c_bool,
    pub need_init_state: ffi::c_bool,
    pub need_init_prop: ffi::c_bool,
}

unsafe extern "C" {
    /// Lzma2Dec_AllocateProbs
    #[doc(alias = "Lzma2Dec_AllocateProbs")]
    #[link_name = "Lzma2Dec_AllocateProbs"]
    pub unsafe fn allocate_probs(decoder: *mut Decoder, prop: u8, alloc: *mut Alloc) -> Error;

    /// Lzma2Dec_Allocate
    #[doc(alias = "Lzma2Dec_Allocate")]
    #[link_name = "Lzma2Dec_Allocate"]
    pub unsafe fn allocate(decoder: *mut Decoder, prop: u8, alloc: *mut Alloc) -> Error;

    /// Lzma2Dec_Init
    #[doc(alias = "Lzma2Dec_Init")]
    #[link_name = "Lzma2Dec_Init"]
    pub unsafe fn init(decoder: *mut Decoder);

    /// Lzma2Dec_DecodeToDic
    #[doc(alias = "Lzma2Dec_DecodeToDic")]
    #[link_name = "Lzma2Dec_DecodeToDic"]
    pub unsafe fn decode_to_dic(
        decoder: *mut Decoder,
        dic_limit: usize,
        src: *const u8,
        src_len: *mut usize,
        finish_mode: FinishMode,
        status: *mut Status,
    ) -> Error;

    /// Lzma2Dec_DecodeToBuf
    #[doc(alias = "Lzma2Dec_DecodeToBuf")]
    #[link_name = "Lzma2Dec_DecodeToBuf"]
    pub unsafe fn decode_to_buf(
        decoder: *mut Decoder,
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: *mut usize,
        finish_mode: FinishMode,
        status: *mut Status,
    ) -> Error;

    /// Lzma2Decode
    #[doc(alias = "Lzma2Decode")]
    #[link_name = "Lzma2Decode"]
    pub unsafe fn decode(
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: *mut usize,
        prop: u8,
        finish_mode: FinishMode,
        status: *mut Status,
        alloc: *mut Alloc,
    ) -> Error;
}
