//! proc_ui

use crate::ffi::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Called just before releasing the foreground. The callback must call [ready_to_release][crate::coreinit::foreground::ready_to_release].
///
/// ProcUISaveCallbackFunction
pub type SaveCallbackFn = unsafe extern "C" fn();

/// Called just before releasing the foreground. The callback must call [ready_to_release][crate::coreinit::foreground::ready_to_release].
///
/// Return `0` to indicate success. Return any non-zero value to indicate a failure.
///
/// ProcUISaveCallbackFunctionEx
pub type SaveCallbackFnEx = unsafe extern "C" fn(data: *mut c_void) -> u32;

/// Called periodically while in background.
///
/// ProcUICallbackFunction
pub type BackgroundCallbackFn = unsafe extern "C" fn(data: *mut c_void) -> u32;

/// Called when a specific message is received
///
/// Return `0` to indicate success. Return any non-zero value to indicate a failure.
///
/// ProcUICallbackFunction
pub type MessageCallbackFn = Option<unsafe extern "C" fn(data: *mut c_void) -> u32>;

/// ProcUIStatus
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Status {
    /// Process is currently in the foreground.
    Foreground = 0,
    /// Process is currently in the background. This is only possible if ... is set to non-blocking.
    Background = 1,
    /// Process needs to release the foreground. Upon receiving this, [ready_to_release] should be called to release the foreground.
    Releasing = 2,
    /// Process needs to exit.
    Exit = 3,
}

/// ProcUIMessage
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Message {
    /// Process moved to foreground.
    Acquire = 0,
    /// Process should move to background.
    Release = 1,
    /// Process should exit.
    Exit = 2,
    /// Another process is trying to use the network.
    NetIoStart = 3,
    /// Another process has stopped using the network.
    NetIoStop = 4,
    /// Home Button has been pressed and ignored.
    HomeButtonDenied = 5,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// Returns the amount of memory required by ProcUI to handle the specified number of callbacks.
    ///
    /// This function should only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUICalcMemorySize
    #[link_name = "ProcUICalcMemorySize"]
    pub unsafe fn required_memory_size(num_callbacks: u32) -> u32;

    /// Clears all callbacks the process has registered.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIClearCallbacks
    #[link_name = "ProcUIClearCallbacks"]
    pub unsafe fn clear_callbacks();

    /// Indicates that the system can proceed to release the foreground.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIDrawDoneRelease
    #[link_name = "ProcUIDrawDoneRelease"]
    pub unsafe fn drawing_done();

    /// Checks if the application is currently in the foreground.
    ///
    /// # Symbol
    ///
    /// ProcUIInForeground
    #[link_name = "ProcUIInForeground"]
    pub unsafe fn in_foreground() -> c_bool;

    /// Checks if the system is currently shutting down.
    ///
    /// During shutdown, the application may first receive a message to release the foreground.
    ///
    /// # Symbol
    ///
    /// ProcUIInShutdown
    #[link_name = "ProcUIInShutdown"]
    pub unsafe fn in_shutdown() -> c_bool;

    /// Initializes the resoureces needed by the ProcUI library. See [init_ex] for more information.
    ///
    /// The thread which calls [init] / [init_ex] becomes the main thread of the application. This function is equivalent to calling [init_ex] with no `data` and returning `0` from the callback.
    ///
    /// # Symbol
    ///
    /// ProcUIInit
    #[link_name = "ProcUIInit"]
    pub unsafe fn init(callback: SaveCallbackFn);

    /// Initializes the resoureces needed by the ProcUI library.
    ///
    /// The thread which calls [init] / [init_ex] becomes the main thread of the application. On initialization, this creates a callback thread for each core. The `data` pointer is passed to the callback and must therefore be valid for the entire programs lifetime.
    ///
    /// # Symbol
    ///
    /// ProcUIInitEx
    #[link_name = "ProcUIInitEx"]
    pub unsafe fn init_ex(callback: SaveCallbackFnEx, data: *mut c_void);

    /// Checks if ProcUI has been initialized and running.
    ///
    /// # Symbol
    ///
    /// ProcUIIsRunning
    #[link_name = "ProcUIIsRunning"]
    pub unsafe fn is_running() -> c_bool;

    /// Handle system messages.
    ///
    /// If in blocking mode and moved into background, the system will automatically manage a thread on core 2 which will handle system messages. If in non-blocking mode, the [main thread][init] must be on core 2 and system messages must be manually handled. Blocking mode is recommended.
    ///
    /// This function should be called periodically, at best every frame. This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIProcessMessages
    #[link_name = "ProcUIProcessMessages"]
    pub unsafe fn process_messages(blocking: c_bool) -> Status;

    /// Registers a function to be called periodically while in the background.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIRegisterBackgroundCallback
    #[link_name = "ProcUIRegisterBackgroundCallback"]
    pub unsafe fn register_background_callback(
        func: BackgroundCallbackFn,
        data: *mut c_void,
        ticks_to_delay: u64,
    );

    /// Registers a callback for when a specific system message is received.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIRegisterCallback
    #[link_name = "ProcUIRegisterCallback"]
    pub unsafe fn register_callback(
        message: Message,
        func: MessageCallbackFn,
        data: *mut c_void,
        priority: i32,
    );

    /// Registers a callback to run on a specified core for when a specific system message is received.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIRegisterCallbackCore
    #[link_name = "ProcUIRegisterCallbackCore"]
    pub unsafe fn register_callback_core(
        message: Message,
        func: MessageCallbackFn,
        data: *mut c_void,
        priority: i32,
        core: u32,
    );

    /// Sets the area to use when saving and restoring the foreground bucket.
    ///
    /// Don't use this function.
    ///
    /// # Symbol
    ///
    /// ProcUISetBucketStorage
    #[link_name = "ProcUISetBucketStorage"]
    #[deprecated]
    pub unsafe fn set_foreground_storage(data: c_void, size: u32);

    /// Set the stack size for the callback threads.
    ///
    /// The default stack size is `8 * 1024`.
    ///
    /// [set_callback_stack_size] must never be called after [calc_memory_size]. If [calc_memory_size] and [set_memory_pool] are never called, [set_callback_stack_size] may be called before or after [init] / [init_ex].
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUISetCallbackStackSize
    #[link_name = "ProcUISetCallbackStackSize"]
    pub unsafe fn set_callback_stack_size();

    /// Sets the area to use when saving and restoring the MEM1 bucket.
    ///
    /// Don't use this function.
    ///
    /// # Symbol
    ///
    /// ProcUISetMEM1Storage
    #[link_name = "ProcUISetMEM1Storage"]
    #[deprecated]
    pub unsafe fn set_mem1_storage(data: c_void, size: u32);

    /// Modifies ProcUI to use predefined memory as opposed to allocate from the default heap.
    ///
    /// [set_callback_stack_size] must be called before [set_memory_pool]. [set_memory_pool] may only be called before [init].
    ///
    /// # Symbol
    ///
    /// ProcUISetMemoryPool
    #[link_name = "ProcUISetMemoryPool"]
    pub unsafe fn set_memory_pool() -> i32;

    /// Registers a function to be called immediately before the foreground is released. Can only be used with [init_ex].
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUISetSaveCallback
    #[link_name = "ProcUISetSaveCallback"]
    pub unsafe fn set_save_callback(func: SaveCallbackFnEx, data: *mut c_void);

    /// Calls all exit callbacks and cleans up the threads.
    ///
    /// This function must only be called from the [main thread][init].
    ///
    /// # Symbol
    ///
    /// ProcUIShutdown
    #[link_name = "ProcUIShutdown"]
    pub unsafe fn shutdown();

    /// Allows multiple threads to handle system messages. This function may be used by sub-loops that handle graphics commands.
    ///
    /// This function must be valid to run in the background. If in blocking mode, blocks thread when in background.
    ///
    /// # Symbol
    ///
    /// ProcUISubProcessMessages
    #[link_name = "ProcUISubProcessMessages"]
    pub unsafe fn sub_process_messages(blocking: c_bool) -> Status;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "proc_ui",
    [
        "ProcUICalcMemorySize",
        "ProcUIClearCallbacks",
        "ProcUIDrawDoneRelease",
        "ProcUIInForeground",
        "ProcUIInShutdown",
        "ProcUIInit",
        "ProcUIInitEx",
        "ProcUIIsRunning",
        "ProcUIProcessMessages",
        "ProcUIRegisterBackgroundCallback",
        "ProcUIRegisterCallback",
        "ProcUIRegisterCallbackCore",
        "ProcUISetBucketStorage",
        "ProcUISetCallbackStackSize",
        "ProcUISetMEM1Storage",
        "ProcUISetMemoryPool",
        "ProcUISetSaveCallback",
        "ProcUIShutdown",
        "ProcUISubProcessMessages"
    ],
    []
);
