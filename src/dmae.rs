//! DMAE

#[cfg(target_arch = "powerpc")]
imports_section!(
    "dmae",
    [
        "DMAECopyMem",
        "DMAEFillMem",
        "DMAEFillMemPhys",
        "DMAEGetLastSubmittedTimeStamp",
        "DMAEGetRetiredTimeStamp",
        "DMAEGetTimeout",
        "DMAESemaphore",
        "DMAESetTimeout",
        "DMAEWaitDone",
        "_DMAESubmit",
        "_DMAESubmitToRing"
    ],
    []
);
