//! nlibnss

#[cfg(target_arch = "powerpc")]
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
