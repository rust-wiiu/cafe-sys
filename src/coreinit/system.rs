// use crate::ffi::*;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct SystemInfo {
    pub bus_clock_speed: u32,
    pub core_clock_speed: u32,
    pub time_base: u64,
    pub l2_size: [u32; 3],
    pub cpu_freq_ratio: u32,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// Global information about the system.
    ///
    /// Kernel sets this information. Applications can only read it.
    ///
    /// # Symbol
    ///
    /// OSGetSystemInfo
    #[link_name = "OSGetSystemInfo"]
    pub unsafe fn system_info() -> *const SystemInfo;

    /// Gets the core executing the current thread.
    ///
    /// # Symbol
    ///
    /// OSGetCoreId
    #[link_name = "OSGetCoreId"]
    pub unsafe fn core_id() -> u32;
}
