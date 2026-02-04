//! uvc

#[cfg(target_arch = "powerpc")]
imports_section!(
    "uvc",
    [
        "UVCClose",
        "UVCGetFrame",
        "UVCInit",
        "UVCOpen",
        "UVCRequest"
    ],
    []
);
