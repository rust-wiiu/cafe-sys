//! proc_ui

use crate::coreinit::time::Time;
use crate::ffi;

/// ProcUISaveCallback
///
/// Called just before the foreground is released. Must call [ready_to_release][crate::coreinit::foreground::ready_to_release].
#[doc(alias = "ProcUISaveCallback")]
pub type SaveCallback = Option<unsafe extern "C" fn()>;

/// ProcUISaveCallbackEx
///
/// Called just before the foreground is released. Must call [ready_to_release][crate::coreinit::foreground::ready_to_release].
#[doc(alias = "ProcUISaveCallbackEx")]
pub type SaveCallbackEx = Option<unsafe extern "C" fn(data: *mut ffi::c_void) -> u32>;

/// ProcUICallback
///
/// Return 0 on success.
#[doc(alias = "ProcUICallback")]
pub type Callback = Option<unsafe extern "C" fn(data: *mut ffi::c_void) -> u32>;

/// ProcUIStatus
#[doc(alias = "ProcUIStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub u32);

impl Status {
    /// All resources may be used.
    pub const IN_FOREGROUND: Self = Self(0);
    /// Only limited resources may be used.
    pub const IN_BACKGROUND: Self = Self(1);
    /// The foreground must be released by calling [draw_done_release].
    pub const RELEASE_FOREGROUND: Self = Self(2);
    /// All resources, including ProcUI, must be released and the process must exit.
    pub const EXITING: Self = Self(3);
}

/// ProcUICallbackType
#[doc(alias = "ProcUICallbackType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallbackType {
    /// The foreground was acquired.
    Acquire = 0,
    /// The foreground must be released.
    Release = 1,
    /// The process must exit.
    Exit = 2,
    /// Networking may be used.
    NetIoStart = 3,
    /// Networking must stop being used.
    NetIoStop = 4,
    /// The HOME button was pressed but denied.
    HomeButtonDenied = 5,
}

/// ProcUI status code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_HEAP: Self = Self(-1);
}

unsafe extern "C" {
    /// ProcUICalcMemorySize
    ///
    /// Returns the memory needed by [set_memory_pool] for `num_callbacks` callbacks. Call [set_callback_stack_size] first.
    #[doc(alias = "ProcUICalcMemorySize")]
    #[link_name = "ProcUICalcMemorySize"]
    pub unsafe fn calc_memory_size(num_callbacks: u32) -> u32;

    /// ProcUIClearCallbacks
    #[doc(alias = "ProcUIClearCallbacks")]
    #[link_name = "ProcUIClearCallbacks"]
    pub unsafe fn clear_callbacks();

    /// ProcUIDrawDoneRelease
    ///
    /// Call after receiving [Status::RELEASE_FOREGROUND] to let the system release the foreground.
    #[doc(alias = "ProcUIDrawDoneRelease")]
    #[link_name = "ProcUIDrawDoneRelease"]
    pub unsafe fn draw_done_release();

    /// ProcUIInForeground
    #[doc(alias = "ProcUIInForeground")]
    #[link_name = "ProcUIInForeground"]
    pub unsafe fn in_foreground() -> ffi::c_bool;

    /// ProcUIInShutdown
    #[doc(alias = "ProcUIInShutdown")]
    #[link_name = "ProcUIInShutdown"]
    pub unsafe fn in_shutdown() -> ffi::c_bool;

    /// ProcUIInit
    #[doc(alias = "ProcUIInit")]
    #[link_name = "ProcUIInit"]
    pub unsafe fn init(callback: SaveCallback);

    /// ProcUIInitEx
    ///
    /// `data` is passed to `callback`.
    #[doc(alias = "ProcUIInitEx")]
    #[link_name = "ProcUIInitEx"]
    pub unsafe fn init_ex(callback: SaveCallbackEx, data: *mut ffi::c_void);

    /// ProcUIIsRunning
    #[doc(alias = "ProcUIIsRunning")]
    #[link_name = "ProcUIIsRunning"]
    pub unsafe fn is_running() -> ffi::c_bool;

    /// ProcUIProcessMessages
    ///
    /// In blocking mode the call blocks while in the background. Non-blocking mode must be called from core 2.
    #[doc(alias = "ProcUIProcessMessages")]
    #[link_name = "ProcUIProcessMessages"]
    pub unsafe fn process_messages(block: ffi::c_bool) -> Status;

    /// ProcUIRegisterBackgroundCallback
    ///
    /// Calls `callback` every `ticks_to_delay` while in the background.
    #[doc(alias = "ProcUIRegisterBackgroundCallback")]
    #[link_name = "ProcUIRegisterBackgroundCallback"]
    pub unsafe fn register_background_callback(
        callback: Callback,
        data: *mut ffi::c_void,
        ticks_to_delay: Time,
    );

    /// ProcUIRegisterCallback
    #[doc(alias = "ProcUIRegisterCallback")]
    #[link_name = "ProcUIRegisterCallback"]
    pub unsafe fn register_callback(
        kind: CallbackType,
        callback: Callback,
        data: *mut ffi::c_void,
        priority: i32,
    );

    /// ProcUIRegisterCallbackCore
    ///
    /// Same as [register_callback], but runs `callback` on `core`.
    #[doc(alias = "ProcUIRegisterCallbackCore")]
    #[link_name = "ProcUIRegisterCallbackCore"]
    pub unsafe fn register_callback_core(
        kind: CallbackType,
        callback: Callback,
        data: *mut ffi::c_void,
        priority: i32,
        core: ffi::c_int,
    );

    /// ProcUISetBucketStorage
    ///
    /// Area used to save and restore the foreground bucket.
    #[doc(alias = "ProcUISetBucketStorage")]
    #[link_name = "ProcUISetBucketStorage"]
    pub unsafe fn set_bucket_storage(data: *mut ffi::c_void, size: u32);

    /// ProcUISetCallbackStackSize
    ///
    /// Stack size of the callback threads, [init] sets it to 8 KiB. Must be called before [calc_memory_size].
    #[doc(alias = "ProcUISetCallbackStackSize")]
    #[link_name = "ProcUISetCallbackStackSize"]
    pub unsafe fn set_callback_stack_size(size: u32);

    /// ProcUISetMEM1Storage
    ///
    /// Area used to save and restore MEM1.
    #[doc(alias = "ProcUISetMEM1Storage")]
    #[link_name = "ProcUISetMEM1Storage"]
    pub unsafe fn set_mem1_storage(data: *mut ffi::c_void, size: u32);

    /// ProcUISetMemoryPool
    ///
    /// Uses `memory` instead of the default heap. Call before [init]. See [calc_memory_size].
    #[doc(alias = "ProcUISetMemoryPool")]
    #[link_name = "ProcUISetMemoryPool"]
    pub unsafe fn set_memory_pool(memory: *mut ffi::c_void, size: u32) -> Error;

    /// ProcUISetSaveCallback
    #[doc(alias = "ProcUISetSaveCallback")]
    #[link_name = "ProcUISetSaveCallback"]
    pub unsafe fn set_save_callback(callback: SaveCallbackEx, data: *mut ffi::c_void);

    /// ProcUIShutdown
    #[doc(alias = "ProcUIShutdown")]
    #[link_name = "ProcUIShutdown"]
    pub unsafe fn shutdown();

    /// ProcUISubProcessMessages
    ///
    /// Same as [process_messages], for use on secondary cores.
    #[doc(alias = "ProcUISubProcessMessages")]
    #[link_name = "ProcUISubProcessMessages"]
    pub unsafe fn sub_process_messages(block: ffi::c_bool) -> Status;
}

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
