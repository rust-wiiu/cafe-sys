//! uvd

#[cfg(target_arch = "powerpc")]
imports_section!(
    "uvd",
    [
        "UVDCheckSegmentViolation",
        "UVDCreateContext",
        "UVDCreateDecodeSession",
        "UVDDecodePicture",
        "UVDDeinitHW",
        "UVDDestroyContext",
        "UVDDestroyDecodeSession",
        "UVDEndDecodePicture",
        "UVDGetDPBSize",
        "UVDInitHW",
        "UVDStartDecodePicture"
    ],
    []
);
