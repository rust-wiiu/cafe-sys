//! NN HPAD

#[cfg(target_arch = "powerpc")]
imports_section!(
    "nn_hpad",
    [
        "BETA_DEBUG_DUMP",
        "BETA_DEBUG_GET_QUEUE_SIZE",
        "BETA_DEBUG_GET_RAW_DATA",
        "BETA_DEBUG_SEND_REPT_ID",
        "BETA_DEBUG_SET_DUMP_MODE",
        "HPADControlMotor",
        "HPADGetGGGGStatus",
        "HPADInit",
        "HPADRecalibrate",
        "HPADResetDevice",
        "HPADSetConnectCallback",
        "HPADSetGgggConnectCallback",
        "HPADSetPowerSupplyCallback",
        "HPADSetSamplingCallback",
        "HPADShutdown",
        "__rplwrap_HPADRead"
    ],
    []
);
