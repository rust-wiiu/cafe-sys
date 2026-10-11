//! LZMA 920
//!
//! LZMA SDK 9.20 by Igor Pavlov (public domain).

use crate::ffi;

pub mod dec;
pub mod enc;
pub mod lzma2;

/// LZMA_PROPS_SIZE
#[doc(alias = "LZMA_PROPS_SIZE")]
pub const PROPS_SIZE: usize = 5;

/// SRes
#[doc(alias = "SRes")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub ffi::c_int);

impl Error {
    pub const OK: Self = Self(0);
    pub const DATA: Self = Self(1);
    pub const MEM: Self = Self(2);
    pub const CRC: Self = Self(3);
    pub const UNSUPPORTED: Self = Self(4);
    pub const PARAM: Self = Self(5);
    pub const INPUT_EOF: Self = Self(6);
    pub const OUTPUT_EOF: Self = Self(7);
    pub const READ: Self = Self(8);
    pub const WRITE: Self = Self(9);
    pub const PROGRESS: Self = Self(10);
    pub const FAIL: Self = Self(11);
    pub const THREAD: Self = Self(12);
    pub const ARCHIVE: Self = Self(16);
    pub const NO_ARCHIVE: Self = Self(17);
}

/// ELzmaFinishMode
#[doc(alias = "ELzmaFinishMode")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FinishMode {
    /// Finish at any point.
    #[default]
    Any = 0,
    /// The block must be finished at the end.
    End = 1,
}

/// ELzmaStatus
#[doc(alias = "ELzmaStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub u32);

impl Status {
    /// Use the returned [Error] instead.
    pub const NOT_SPECIFIED: Self = Self(0);
    pub const FINISHED_WITH_MARK: Self = Self(1);
    pub const NOT_FINISHED: Self = Self(2);
    pub const NEEDS_MORE_INPUT: Self = Self(3);
    pub const MAYBE_FINISHED_WITHOUT_MARK: Self = Self(4);
}

/// ISzAlloc
///
/// `p` points to this struct.
#[doc(alias = "ISzAlloc")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alloc {
    pub alloc: Option<unsafe extern "C" fn(p: *mut ffi::c_void, size: usize) -> *mut ffi::c_void>,
    pub free: Option<unsafe extern "C" fn(p: *mut ffi::c_void, address: *mut ffi::c_void)>,
}

/// ISeqInStream
///
/// `p` points to this struct. `read` sets `size` to the number of bytes read, 0 means end of stream.
#[doc(alias = "ISeqInStream")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqInStream {
    pub read: Option<
        unsafe extern "C" fn(p: *mut ffi::c_void, buf: *mut ffi::c_void, size: *mut usize) -> Error,
    >,
}

/// ISeqOutStream
///
/// `p` points to this struct. `write` returns the number of bytes written, less than `size` means error.
#[doc(alias = "ISeqOutStream")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqOutStream {
    pub write:
        Option<unsafe extern "C" fn(p: *mut ffi::c_void, buf: *const ffi::c_void, size: usize) -> usize>,
}

/// ICompressProgress
///
/// `p` points to this struct. Returning anything but [Error::OK] aborts. A size of `u64::MAX` means unknown.
#[doc(alias = "ICompressProgress")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressProgress {
    pub progress: Option<unsafe extern "C" fn(p: *mut ffi::c_void, in_size: u64, out_size: u64) -> Error>,
}

unsafe extern "C" {
    /// LzmaCompress
    ///
    /// `props_size` must be [PROPS_SIZE]. Defaults: `level` 5, `dict_size` 1 << 24, `lc` 3, `lp` 0, `pb` 2, `fb` 32, `num_threads` 2.
    #[doc(alias = "LzmaCompress")]
    #[link_name = "LzmaCompress"]
    pub unsafe fn compress(
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: usize,
        props: *mut u8,
        props_size: *mut usize,
        level: ffi::c_int,
        dict_size: ffi::c_uint,
        lc: ffi::c_int,
        lp: ffi::c_int,
        pb: ffi::c_int,
        fb: ffi::c_int,
        num_threads: ffi::c_int,
    ) -> Error;

    /// LzmaUncompress
    #[doc(alias = "LzmaUncompress")]
    #[link_name = "LzmaUncompress"]
    pub unsafe fn uncompress(
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: *mut usize,
        props: *const u8,
        props_size: usize,
    ) -> Error;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "lzma920",
    [
        "Bt3Zip_MatchFinder_GetMatches",
        "Bt3Zip_MatchFinder_Skip",
        "GetMatchesSpec1",
        "Hc3Zip_MatchFinder_GetMatches",
        "Hc3Zip_MatchFinder_Skip",
        "Lzma2Dec_Allocate",
        "Lzma2Dec_AllocateProbs",
        "Lzma2Dec_DecodeToBuf",
        "Lzma2Dec_DecodeToDic",
        "Lzma2Dec_Init",
        "Lzma2Decode",
        "Lzma2EncProps_Init",
        "Lzma2EncProps_Normalize",
        "Lzma2Enc_Create",
        "Lzma2Enc_Destroy",
        "Lzma2Enc_Encode",
        "Lzma2Enc_SetProps",
        "Lzma2Enc_WriteProperties",
        "LzmaCompress",
        "LzmaDec_Allocate",
        "LzmaDec_AllocateProbs",
        "LzmaDec_DecodeToBuf",
        "LzmaDec_DecodeToDic",
        "LzmaDec_Free",
        "LzmaDec_FreeProbs",
        "LzmaDec_Init",
        "LzmaDecode",
        "LzmaEncProps_GetDictSize",
        "LzmaEncProps_Init",
        "LzmaEncProps_Normalize",
        "LzmaEnc_Create",
        "LzmaEnc_Destroy",
        "LzmaEnc_Encode",
        "LzmaEnc_MemEncode",
        "LzmaEnc_SetProps",
        "LzmaEnc_WriteProperties",
        "LzmaEncode",
        "LzmaProps_Decode",
        "LzmaUncompress",
        "MatchFinder_Construct",
        "MatchFinder_Create",
        "MatchFinder_CreateVTable",
        "MatchFinder_Free",
        "MatchFinder_GetPointerToCurrentPos",
        "MatchFinder_Init",
        "MatchFinder_MoveBlock",
        "MatchFinder_NeedMove",
        "MatchFinder_Normalize3",
        "MatchFinder_ReadIfRequired",
        "MatchFinder_ReduceOffsets"
    ],
    []
);
