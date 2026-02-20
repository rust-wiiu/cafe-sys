//! nlibnss2

// #[cfg(target_arch = "powerpc")]
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
