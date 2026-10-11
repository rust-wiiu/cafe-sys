//! LZMA encoder

use super::{Alloc, CompressProgress, Error, SeqInStream, SeqOutStream};
use crate::ffi;

/// Initialize with [props_init].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Props {
    /// 0..=9, default 5.
    pub level: ffi::c_int,
    /// (1 << 12)..=(1 << 27), default 1 << 24.
    pub dict_size: u32,
    /// 0..=8, default 3.
    pub lc: ffi::c_int,
    /// 0..=4, default 0.
    pub lp: ffi::c_int,
    /// 0..=4, default 2.
    pub pb: ffi::c_int,
    /// 0: fast, 1: normal (default).
    pub algo: ffi::c_int,
    /// 5..=273, default 32.
    pub fb: ffi::c_int,
    /// 0: hash chain, 1: binary tree (default).
    pub bt_mode: ffi::c_int,
    /// 2, 3 or 4, default 4.
    pub num_hash_bytes: ffi::c_int,
    /// 1..=(1 << 30), default 32.
    pub mc: u32,
    /// Write an end marker, default 0.
    pub write_end_mark: ffi::c_uint,
    /// Default 1, this build is single-threaded.
    pub num_threads: ffi::c_int,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub *mut ffi::c_void);

unsafe extern "C" {
    /// LzmaEncProps_Init
    #[doc(alias = "LzmaEncProps_Init")]
    #[link_name = "LzmaEncProps_Init"]
    pub unsafe fn props_init(props: *mut Props);

    /// LzmaEncProps_Normalize
    #[doc(alias = "LzmaEncProps_Normalize")]
    #[link_name = "LzmaEncProps_Normalize"]
    pub unsafe fn props_normalize(props: *mut Props);

    /// LzmaEncProps_GetDictSize
    #[doc(alias = "LzmaEncProps_GetDictSize")]
    #[link_name = "LzmaEncProps_GetDictSize"]
    pub unsafe fn props_get_dict_size(props: *const Props) -> u32;

    /// LzmaEnc_Create
    ///
    /// Returns a null handle on failure.
    #[doc(alias = "LzmaEnc_Create")]
    #[link_name = "LzmaEnc_Create"]
    pub unsafe fn create(alloc: *mut Alloc) -> Handle;

    /// LzmaEnc_Destroy
    #[doc(alias = "LzmaEnc_Destroy")]
    #[link_name = "LzmaEnc_Destroy"]
    pub unsafe fn destroy(handle: Handle, alloc: *mut Alloc, alloc_big: *mut Alloc);

    /// LzmaEnc_SetProps
    #[doc(alias = "LzmaEnc_SetProps")]
    #[link_name = "LzmaEnc_SetProps"]
    pub unsafe fn set_props(handle: Handle, props: *const Props) -> Error;

    /// LzmaEnc_WriteProperties
    ///
    /// Writes [PROPS_SIZE](super::PROPS_SIZE) bytes.
    #[doc(alias = "LzmaEnc_WriteProperties")]
    #[link_name = "LzmaEnc_WriteProperties"]
    pub unsafe fn write_properties(handle: Handle, props: *mut u8, size: *mut usize) -> Error;

    /// LzmaEnc_Encode
    #[doc(alias = "LzmaEnc_Encode")]
    #[link_name = "LzmaEnc_Encode"]
    pub unsafe fn encode_stream(
        handle: Handle,
        out_stream: *mut SeqOutStream,
        in_stream: *mut SeqInStream,
        progress: *mut CompressProgress,
        alloc: *mut Alloc,
        alloc_big: *mut Alloc,
    ) -> Error;

    /// LzmaEnc_MemEncode
    #[doc(alias = "LzmaEnc_MemEncode")]
    #[link_name = "LzmaEnc_MemEncode"]
    pub unsafe fn encode_mem(
        handle: Handle,
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: usize,
        write_end_mark: ffi::c_int,
        progress: *mut CompressProgress,
        alloc: *mut Alloc,
        alloc_big: *mut Alloc,
    ) -> Error;

    /// LzmaEncode
    #[doc(alias = "LzmaEncode")]
    #[link_name = "LzmaEncode"]
    pub unsafe fn encode(
        dst: *mut u8,
        dst_len: *mut usize,
        src: *const u8,
        src_len: usize,
        props: *const Props,
        props_encoded: *mut u8,
        props_size: *mut usize,
        write_end_mark: ffi::c_int,
        progress: *mut CompressProgress,
        alloc: *mut Alloc,
        alloc_big: *mut Alloc,
    ) -> Error;
}
