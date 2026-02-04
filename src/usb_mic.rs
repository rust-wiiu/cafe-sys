//! usb_mic

#[cfg(target_arch = "powerpc")]
imports_section!(
    "usb_mic",
    [
        "USBMICClose",
        "USBMICGetState",
        "USBMICGetStatus",
        "USBMICInit",
        "USBMICOpen",
        "USBMICOpenQuery",
        "USBMICSetDataConsumed",
        "USBMICSetState",
        "USBMICStart",
        "USBMICStartExt",
        "USBMICStop",
        "USBMICUninit"
    ],
    []
);
