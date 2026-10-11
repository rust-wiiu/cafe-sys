//! nlibnss

use crate::ffi;

/// NSS error code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const OK: Self = Self(0);
    pub const NOT_INITIALIZED: Self = Self(0xFFD1FFFBu32 as i32);
    pub const INVALID_ARG: Self = Self(0xFFD1FFF7u32 as i32);
}

unsafe extern "C" {
    /// NSSInit
    #[doc(alias = "NSSInit")]
    #[link_name = "NSSInit"]
    pub unsafe fn init() -> Error;

    /// NSSFinish
    #[doc(alias = "NSSFinish")]
    #[link_name = "NSSFinish"]
    pub unsafe fn finish() -> Error;

    /// NSSLibReady
    ///
    /// Returns the internal device handle, or -1 if the library is not initialized.
    #[doc(alias = "NSSLibReady")]
    #[link_name = "NSSLibReady"]
    pub unsafe fn lib_ready() -> i32;

    /// NSSGetRandom
    ///
    /// Requires [init]. Returns [Error::INVALID_ARG] if `buf` is null or `size` is negative.
    #[doc(alias = "NSSGetRandom")]
    #[link_name = "NSSGetRandom"]
    pub unsafe fn fill_random(buf: *mut ffi::c_void, size: i32) -> Error;
}

imports_section!(
    "nlibnss",
    [
        "NSSCreateSignatureContext",
        "NSSDestroySignatureContext",
        "NSSExportDeviceCertChain",
        "NSSFinish",
        "NSSGetRandom",
        "NSSInit",
        "NSSLibReady",
        "NSSSecureStoreDeleteAllObjects",
        "NSSSecureStoreDeleteObject",
        "NSSSecureStoreDeleteTitleObjStore",
        "NSSSecureStoreExportObject",
        "NSSSecureStoreImportObject",
        "NSSSignatureGetSignatureLength",
        "NSSSignatureSetPrivateKey",
        "NSSSignatureSetPrivateKeyExternal",
        "NSSSignatureSignDigest"
    ],
    []
);

imports_section!(
    "nlibnss2",
    [
        "NSSCreateHMACContext",
        "NSSCreateSymkeyCryptoContext",
        "NSSDestroyHMACContext",
        "NSSDestroySymkeyCryptoContext",
        "NSSHMACFinish",
        "NSSHMACUpdate",
        "NSSSymkeyCryptoDecryptFinish",
        "NSSSymkeyCryptoDecryptInitInternalKey",
        "NSSSymkeyCryptoDecryptUpdate",
        "NSSSymkeyCryptoEncryptFinish",
        "NSSSymkeyCryptoEncryptInitInternalKey",
        "NSSSymkeyCryptoEncryptUpdate",
        "NSSSymkeyCryptoGetCipherBlockSize"
    ],
    []
);
