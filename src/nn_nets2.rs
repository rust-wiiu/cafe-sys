//! NN NETS2

#[cfg(target_arch = "powerpc")]
imports_section!(
    "nn_nets2",
    [
        "NSSLAddCRLExternal",
        "NSSLContextClearFlags",
        "NSSLContextGetFlags",
        "NSSLContextSetFlags",
        "NSSLContextSetMode",
        "icmp_cancel",
        "icmp_close_handle",
        "icmp_create_handle",
        "icmp_last_code_type",
        "icmp_ping",
        "__rplwrap_someopt"
    ],
    []
);
