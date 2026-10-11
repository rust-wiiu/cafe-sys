//! USB Audio Class driver
//!
//! GamePad USB audio shim, used internally by `mic`.

imports_section!(
    "uac",
    [
        "UACClose",
        "UACFreeISODesc",
        "UACGetAudio",
        "UACInit",
        "UACOpen",
        "UACRequest"
    ],
    []
);
