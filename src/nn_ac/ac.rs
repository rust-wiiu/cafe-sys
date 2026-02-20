#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum AcStatus {
    Failed = -1,
    Ok = 0,
    Processing = 1,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// Initializes the AC library.
    ///
    /// If this function has already been called, it doesn nothing. This function **must** be called before calling any other AC functions.
    ///
    /// # Symbol
    ///
    /// ACInitialize
    #[link_name = "ACInitialize"]
    pub unsafe fn init() -> AcStatus;

    /// Deinitializes the AC library.
    ///
    /// Close any active connections with [close] before calling this function. This function asserts(?) if called more times than the [init] function.
    ///
    /// # Symbol
    ///
    /// ACFinalize
    #[link_name = "ACFinalize"]
    pub unsafe fn deinit();

    /// Creates a network connection.
    ///
    /// Uses the default network profile configures in the profile settings. Any network related functions (like [socket]) must be used after this function and before [close].
    ///
    /// # Symbol
    ///
    /// ACConnect
    #[link_name = "ACConnect"]
    pub unsafe fn connect() -> AcStatus;

    /// Closes a network connection.
    ///
    /// Only closes connections created by this process. Any network related functions (like [socket]) must be used after [connect] function and before this function.
    ///
    /// # Symbol
    ///
    /// ACClose
    #[link_name = "ACClose"]
    pub unsafe fn close() -> AcStatus;

}
