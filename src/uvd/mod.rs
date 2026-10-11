//! UVD hardware video decoder driver

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
