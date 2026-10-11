//! LZMA2 encoder

use crate::{
    ffi,
    lzma920::{Alloc, CompressProgress, Error, SeqInStream, SeqOutStream, enc},
};

/// CLzma2EncProps
///
/// Initialize with [props_init].
#[doc(alias = "CLzma2EncProps")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Props {
    pub lzma_props: enc::Props,
    pub block_size: usize,
    pub num_block_threads: ffi::c_int,
    pub num_total_threads: ffi::c_int,
}

/// CLzma2EncHandle
#[doc(alias = "CLzma2EncHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub *mut ffi::c_void);

unsafe extern "C" {
    /// Lzma2EncProps_Init
    #[doc(alias = "Lzma2EncProps_Init")]
    #[link_name = "Lzma2EncProps_Init"]
    pub unsafe fn props_init(props: *mut Props);

    /// Lzma2EncProps_Normalize
    #[doc(alias = "Lzma2EncProps_Normalize")]
    #[link_name = "Lzma2EncProps_Normalize"]
    pub unsafe fn props_normalize(props: *mut Props);

    /// Lzma2Enc_Create
    ///
    /// Returns a null handle on failure.
    #[doc(alias = "Lzma2Enc_Create")]
    #[link_name = "Lzma2Enc_Create"]
    pub unsafe fn create(alloc: *mut Alloc, alloc_big: *mut Alloc) -> Handle;

    /// Lzma2Enc_Destroy
    #[doc(alias = "Lzma2Enc_Destroy")]
    #[link_name = "Lzma2Enc_Destroy"]
    pub unsafe fn destroy(handle: Handle);

    /// Lzma2Enc_SetProps
    #[doc(alias = "Lzma2Enc_SetProps")]
    #[link_name = "Lzma2Enc_SetProps"]
    pub unsafe fn set_props(handle: Handle, props: *const Props) -> Error;

    /// Lzma2Enc_WriteProperties
    ///
    /// Returns the single LZMA2 property byte.
    #[doc(alias = "Lzma2Enc_WriteProperties")]
    #[link_name = "Lzma2Enc_WriteProperties"]
    pub unsafe fn write_properties(handle: Handle) -> u8;

    /// Lzma2Enc_Encode
    #[doc(alias = "Lzma2Enc_Encode")]
    #[link_name = "Lzma2Enc_Encode"]
    pub unsafe fn encode_stream(
        handle: Handle,
        out_stream: *mut SeqOutStream,
        in_stream: *mut SeqInStream,
        progress: *mut CompressProgress,
    ) -> Error;
}
