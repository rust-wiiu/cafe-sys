//! nsyshid

#[cfg(target_arch = "powerpc")]
imports_section!(
    "nsyshid",
    [
        "HIDAddClient",
        "HIDAddPassiveClient",
        "HIDDecodeError",
        "HIDDelClient",
        "HIDDelPassiveClient",
        "HIDGetDescriptor",
        "HIDGetHierarchy",
        "HIDGetIdle",
        "HIDGetProtocol",
        "HIDGetReport",
        "HIDRead",
        "HIDResetDevice",
        "HIDSetDescriptor",
        "HIDSetIdle",
        "HIDSetProtocol",
        "HIDSetReport",
        "HIDSetup",
        "HIDTeardown",
        "HIDWrite"
    ],
    []
);
