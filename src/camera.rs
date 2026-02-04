//! Camera

#[cfg(target_arch = "powerpc")]
imports_section!(
    "camera",
    [
        "CAMCheckMemSegmentation",
        "CAMClose",
        "CAMExit",
        "CAMGetMemReq",
        "CAMGetState",
        "CAMGetStateInfo",
        "CAMInit",
        "CAMOpen",
        "CAMSetState",
        "CAMSubmitTargetSurface"
    ],
    []
);
