//! nIO prof

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "nio_prof",
    [
        "IO_ProfilerGetStatsAndEndCheckpoint",
        "IO_ProfilerGetStatsAndRestartCheckpoint",
        "IO_ProfilerLibFinish",
        "IO_ProfilerLibInit",
        "IO_ProfilerStartCheckpoint"
    ],
    []
);
