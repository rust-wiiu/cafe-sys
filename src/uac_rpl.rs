//! uac_rpl

#[cfg(target_arch = "powerpc")]
imports_section!(
    "uac_rpl",
    [
        "UACClose",
        "UACGetState",
        "UACINGetStatus",
        "UACINSetDataConsumed",
        "UACINStart",
        "UACINStartExt",
        "UACINStop",
        "UACInit",
        "UACOUTGetStatus",
        "UACOUTSetDataConsumed",
        "UACOUTStart",
        "UACOUTStartExt",
        "UACOUTStop",
        "UACOpen",
        "UACOpenQuery",
        "UACSetState",
        "UACUninit"
    ],
    []
);
