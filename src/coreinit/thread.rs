//! Thread
//!
//! Threads are scheduled per core by priority, 0 is the highest and 31 the lowest.

use super::alarm::Alarm;
use super::context::Context;
use super::exception::ExceptionCallback;
use super::fast_mutex::{FastMutex, FastMutexQueue};
use super::mutex::{Mutex, MutexQueue};
use super::thread_queue::{ThreadLink, ThreadQueue};
use super::time::Time;
use crate::ffi;

/// OS_THREAD_TAG
///
/// Value of [Thread::tag].
pub const THREAD_TAG: u32 = 0x74487244;
/// OS_THREAD_SPECIFIC_MAX
///
/// Number of thread specific values, see [get_specific].
pub const SPECIFIC_MAX: usize = 16;

/// Thread priority, 0 is the highest and 31 the lowest.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Priority(pub i32);

impl Priority {
    /// Highest priority.
    pub const MIN: Self = Self(0);
    pub const DEFAULT: Self = Self(16);
    /// Lowest priority.
    pub const MAX: Self = Self(31);
}

/// Time slice of a thread in microseconds.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quantum(pub u32);

impl Quantum {
    /// OS_THREAD_QUANTUM_INFINITE
    pub const INFINITE: Self = Self(0);
    /// OS_THREAD_QUANTUM_MIN_MICROSECONDS
    pub const MIN: Self = Self(10);
    /// OS_THREAD_QUANTUM_MAX_MICROSECONDS
    pub const MAX: Self = Self(0xFFFFF);
}

/// OSThreadState
#[doc(alias = "OSThreadState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadState(pub u8);

impl ThreadState {
    pub const NONE: Self = Self(0);
    pub const READY: Self = Self(1 << 0);
    pub const RUNNING: Self = Self(1 << 1);
    /// Waiting, e.g. on a mutex.
    pub const WAITING: Self = Self(1 << 2);
    /// About to terminate.
    pub const MORIBUND: Self = Self(1 << 3);
}

/// OSThreadAttributes
///
/// Core affinity and creation flags.
#[doc(alias = "OSThreadAttributes", alias = "OS_THREAD_ATTRIB")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadAttributes(pub u8);

impl ThreadAttributes {
    pub const NONE: Self = Self(0);
    pub const AFFINITY_CPU0: Self = Self(1 << 0);
    pub const AFFINITY_CPU1: Self = Self(1 << 1);
    pub const AFFINITY_CPU2: Self = Self(1 << 2);
    pub const AFFINITY_ANY: Self = Self(0b111);
    /// Starts the thread detached, see [detach].
    pub const DETACHED: Self = Self(1 << 3);
    pub const PINNED_AFFINITY: Self = Self(1 << 4);
    /// Tracks stack usage, see [check_stack_usage].
    pub const STACK_USAGE: Self = Self(1 << 6);
    pub const NAME_SENT: Self = Self(1 << 7);
}

impl core::ops::BitOr for ThreadAttributes {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSThreadRequest
#[doc(alias = "OSThreadRequest")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadRequest(pub u32);

impl ThreadRequest {
    pub const NONE: Self = Self(0);
    pub const SUSPEND: Self = Self(1);
    pub const CANCEL: Self = Self(2);
}

/// OSThreadType
#[doc(alias = "OSThreadType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadType(pub u32);

impl ThreadType {
    pub const DRIVER: Self = Self(0);
    pub const IO: Self = Self(1);
    pub const APP: Self = Self(2);
}

/// Reasons the thread cannot be cancelled, see [Thread::cancel_state].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancelState(pub u32);

impl CancelState {
    pub const ENABLED: Self = Self(0);
    pub const DISABLED_USER: Self = Self(1 << 0);
    pub const DISABLED_MUTEX: Self = Self(1 << 16);
    pub const DISABLED_SPINLOCK: Self = Self(1 << 17);
    pub const DISABLED_STACK: Self = Self(1 << 18);
    pub const DISABLED_FAST_MUTEX: Self = Self(1 << 19);
}

/// OSThreadEntryPointFn
///
/// The return value is the exit value of the thread.
#[doc(alias = "OSThreadEntryPointFn")]
pub type ThreadEntry =
    Option<unsafe extern "C" fn(int_arg: ffi::c_int, ptr_arg: *mut ffi::c_void) -> ffi::c_int>;

/// OSThreadCleanupCallbackFn
///
/// Called on the terminating thread before it ends.
#[doc(alias = "OSThreadCleanupCallbackFn")]
pub type CleanupCallback =
    Option<unsafe extern "C" fn(thread: *mut Thread, stack_end: *mut ffi::c_void)>;

/// OSThreadDeallocatorFn
///
/// Called after the thread ended, to free the thread and its stack.
#[doc(alias = "OSThreadDeallocatorFn")]
pub type Deallocator =
    Option<unsafe extern "C" fn(thread: *mut Thread, stack_end: *mut ffi::c_void)>;

/// OSThreadGHSExceptionHandling
///
/// Per-thread C runtime state.
#[doc(alias = "OSThreadGHSExceptionHandling")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GhsExceptionHandling {
    _crt: ffi::unknown<0x68>,
    pub eh_globals: *mut ffi::c_void,
    pub eh_mem_manage: [*mut ffi::c_void; 9],
    pub eh_store_globals: [*mut ffi::c_void; 6],
    pub eh_store_globals_tdeh: [*mut ffi::c_void; 76],
}

/// OSTLSSection
#[doc(alias = "OSTLSSection")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlsBlock {
    pub address: *mut ffi::c_void,
    pub status: i32,
}

/// OSThread
#[doc(alias = "OSThread")]
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct Thread {
    pub context: Context,
    /// [THREAD_TAG].
    pub tag: u32,
    pub state: ThreadState,
    pub attr: ThreadAttributes,
    pub id: u16,
    /// Increased by [suspend].
    pub suspend_counter: i32,
    /// Current priority, see also [Thread::base_priority].
    pub priority: Priority,
    pub base_priority: Priority,
    pub exit_value: i32,
    pub core_run_queue: [*mut ThreadQueue; 3],
    pub core_run_queue_link: [ThreadLink; 3],
    /// Queue the thread is waiting on.
    pub queue: *mut ThreadQueue,
    pub link: ThreadLink,
    /// Threads waiting to join this thread.
    pub join_queue: ThreadQueue,
    /// Mutex the thread is waiting to lock.
    pub mutex: *mut Mutex,
    /// Mutexes owned by the thread.
    pub mutex_queue: MutexQueue,
    pub active_link: ThreadLink,
    /// Highest address of the stack.
    pub stack_start: *mut ffi::c_void,
    /// Lowest address of the stack.
    pub stack_end: *mut ffi::c_void,
    pub entry_point: ThreadEntry,
    pub ghs_exception_handling: GhsExceptionHandling,
    pub alarm_cancelled: ffi::c_bool,
    pub specific: [*mut ffi::c_void; SPECIFIC_MAX],
    pub kind: ThreadType,
    pub name: *const ffi::c_char,
    pub wait_event_timeout_alarm: *mut Alarm,
    pub user_stack_pointer: *mut ffi::c_void,
    pub cleanup_callback: CleanupCallback,
    pub deallocator: Deallocator,
    pub cancel_state: CancelState,
    pub request_flag: ThreadRequest,
    pub need_suspend: i32,
    pub suspend_result: i32,
    /// Threads waiting for this thread to suspend.
    pub suspend_queue: ThreadQueue,
    pub run_quantum_ticks: i64,
    pub core_time_consumed_ns: u64,
    pub wake_count: u64,
    pub sum_wake_delta: Time,
    pub awakened_time: Time,
    pub min_wake_delta: Time,
    pub max_wake_delta: Time,
    pub dsi_callback: [ExceptionCallback; 3],
    pub isi_callback: [ExceptionCallback; 3],
    pub program_callback: [ExceptionCallback; 3],
    pub perf_mon_callback: [ExceptionCallback; 3],
    /// Allows synchronization objects on the stack in debug mode.
    pub stack_sync_obj_allowed: ffi::c_bool,
    pub tls_section_count: i16,
    pub tls_status: i16,
    pub tls_sections: *mut TlsBlock,
    /// Fast mutex the thread is waiting to lock.
    pub fast_mutex: *mut FastMutex,
    pub contended_fast_mutexes: FastMutexQueue,
    pub owned_fast_mutexes: FastMutexQueue,
    pub align_callback: [ExceptionCallback; 3],
    _reserved: ffi::unknown<0x14>,
}

unsafe extern "C" {
    /// OSCreateThread
    ///
    /// Creates a suspended thread, start it with [resume]. `stack` is the highest address of the stack. Returns false on failure.
    #[doc(alias = "OSCreateThread")]
    #[link_name = "OSCreateThread"]
    pub unsafe fn create(
        thread: *mut Thread,
        entry: ThreadEntry,
        int_arg: ffi::c_int,
        ptr_arg: *mut ffi::c_void,
        stack: *mut ffi::c_void,
        stack_size: u32,
        priority: Priority,
        attributes: ThreadAttributes,
    ) -> ffi::c_bool;

    /// OSCreateThreadType
    ///
    /// Same as [create] with a [ThreadType].
    #[doc(alias = "OSCreateThreadType")]
    #[link_name = "OSCreateThreadType"]
    pub unsafe fn create_type(
        thread: *mut Thread,
        entry: ThreadEntry,
        int_arg: ffi::c_int,
        ptr_arg: *mut ffi::c_void,
        stack: *mut ffi::c_void,
        stack_size: u32,
        priority: Priority,
        attributes: ThreadAttributes,
        kind: ThreadType,
    ) -> ffi::c_bool;

    /// OSRunThread
    ///
    /// Runs `entry` on an existing idle thread.
    #[doc(alias = "OSRunThread")]
    #[link_name = "OSRunThread"]
    pub unsafe fn run(
        thread: *mut Thread,
        entry: ThreadEntry,
        int_arg: ffi::c_int,
        ptr_arg: *mut ffi::c_void,
    ) -> ffi::c_bool;

    /// OSExitThread
    #[doc(alias = "OSExitThread")]
    #[link_name = "OSExitThread"]
    pub unsafe fn exit(value: ffi::c_int) -> !;

    /// OSJoinThread
    ///
    /// Waits for `thread` to terminate. `value` (nullable) receives its exit value.
    #[doc(alias = "OSJoinThread")]
    #[link_name = "OSJoinThread"]
    pub unsafe fn join(thread: *mut Thread, value: *mut ffi::c_int) -> ffi::c_bool;

    /// OSDetachThread
    ///
    /// The thread is released on termination and cannot be joined.
    #[doc(alias = "OSDetachThread")]
    #[link_name = "OSDetachThread"]
    pub unsafe fn detach(thread: *mut Thread);

    /// OSCancelThread
    #[doc(alias = "OSCancelThread")]
    #[link_name = "OSCancelThread"]
    pub unsafe fn cancel(thread: *mut Thread);

    /// OSTestThreadCancel
    ///
    /// Terminates the current thread if a cancel is pending.
    #[doc(alias = "OSTestThreadCancel")]
    #[link_name = "OSTestThreadCancel"]
    pub unsafe fn test_cancel();

    /// OSSetThreadCancelState
    ///
    /// Returns the previous state.
    #[doc(alias = "OSSetThreadCancelState")]
    #[link_name = "OSSetThreadCancelState"]
    pub unsafe fn set_cancel_state(enabled: ffi::c_bool) -> ffi::c_bool;

    /// OSResumeThread
    ///
    /// Decrements the suspend count and returns the previous count.
    #[doc(alias = "OSResumeThread")]
    #[link_name = "OSResumeThread"]
    pub unsafe fn resume(thread: *mut Thread) -> i32;

    /// OSSuspendThread
    ///
    /// Increments the suspend count and returns the previous count.
    #[doc(alias = "OSSuspendThread")]
    #[link_name = "OSSuspendThread"]
    pub unsafe fn suspend(thread: *mut Thread) -> i32;

    /// __OSSuspendThreadNolock
    #[doc(alias = "__OSSuspendThreadNolock")]
    #[link_name = "__OSSuspendThreadNolock"]
    pub unsafe fn suspend_no_lock(thread: *mut Thread);

    /// OSContinueThread
    ///
    /// Resumes a suspended thread regardless of its suspend count.
    #[doc(alias = "OSContinueThread")]
    #[link_name = "OSContinueThread"]
    pub unsafe fn continue_(thread: *mut Thread);

    /// OSIsThreadSuspended
    #[doc(alias = "OSIsThreadSuspended")]
    #[link_name = "OSIsThreadSuspended"]
    pub unsafe fn is_suspended(thread: *mut Thread) -> ffi::c_bool;

    /// OSIsThreadTerminated
    #[doc(alias = "OSIsThreadTerminated")]
    #[link_name = "OSIsThreadTerminated"]
    pub unsafe fn is_terminated(thread: *mut Thread) -> ffi::c_bool;

    /// OSYieldThread
    #[doc(alias = "OSYieldThread")]
    #[link_name = "OSYieldThread"]
    pub unsafe fn yield_();

    /// OSSleepThread
    ///
    /// Sleeps the current thread on `queue` until [wakeup].
    #[doc(alias = "OSSleepThread")]
    #[link_name = "OSSleepThread"]
    pub unsafe fn sleep(queue: *mut ThreadQueue);

    /// OSWakeupThread
    ///
    /// Wakes all threads sleeping on `queue`.
    #[doc(alias = "OSWakeupThread")]
    #[link_name = "OSWakeupThread"]
    pub unsafe fn wakeup(queue: *mut ThreadQueue);

    /// OSSleepTicks
    #[doc(alias = "OSSleepTicks")]
    #[link_name = "OSSleepTicks"]
    pub unsafe fn sleep_ticks(ticks: Time);

    /// OSGetCurrentThread
    #[doc(alias = "OSGetCurrentThread")]
    #[link_name = "OSGetCurrentThread"]
    pub unsafe fn get_current() -> *mut Thread;

    /// OSGetDefaultThread
    ///
    /// The thread created for core `core_id` before the application started.
    #[doc(alias = "OSGetDefaultThread")]
    #[link_name = "OSGetDefaultThread"]
    pub unsafe fn get_default(core_id: u32) -> *mut Thread;

    /// OSGetThreadPriority
    #[doc(alias = "OSGetThreadPriority")]
    #[link_name = "OSGetThreadPriority"]
    pub unsafe fn get_priority(thread: *mut Thread) -> Priority;

    /// OSSetThreadPriority
    #[doc(alias = "OSSetThreadPriority")]
    #[link_name = "OSSetThreadPriority"]
    pub unsafe fn set_priority(thread: *mut Thread, priority: Priority) -> ffi::c_bool;

    /// OSGetThreadAffinity
    #[doc(alias = "OSGetThreadAffinity")]
    #[link_name = "OSGetThreadAffinity"]
    pub unsafe fn get_affinity(thread: *mut Thread) -> ThreadAttributes;

    /// OSSetThreadAffinity
    ///
    /// Replaces the affinity bits of the attributes. Other bits set in `affinity` are set as well.
    #[doc(alias = "OSSetThreadAffinity")]
    #[link_name = "OSSetThreadAffinity"]
    pub unsafe fn set_affinity(thread: *mut Thread, affinity: ThreadAttributes) -> ffi::c_bool;

    /// OSGetThreadName
    #[doc(alias = "OSGetThreadName")]
    #[link_name = "OSGetThreadName"]
    pub unsafe fn get_name(thread: *const Thread) -> *const ffi::c_char;

    /// OSSetThreadName
    ///
    /// `name` is not copied.
    #[doc(alias = "OSSetThreadName")]
    #[link_name = "OSSetThreadName"]
    pub unsafe fn set_name(thread: *mut Thread, name: *const ffi::c_char);

    /// OSSetThreadRunQuantum
    ///
    /// Time slice in microseconds, [Quantum::INFINITE] or between [Quantum::MIN] and [Quantum::MAX].
    #[doc(alias = "OSSetThreadRunQuantum")]
    #[link_name = "OSSetThreadRunQuantum"]
    pub unsafe fn set_run_quantum(thread: *mut Thread, quantum: Quantum) -> ffi::c_bool;

    /// OSGetThreadSpecific
    ///
    /// `index` is below [SPECIFIC_MAX].
    #[doc(alias = "OSGetThreadSpecific")]
    #[link_name = "OSGetThreadSpecific"]
    pub unsafe fn get_specific(index: i32) -> *mut ffi::c_void;

    /// OSSetThreadSpecific
    #[doc(alias = "OSSetThreadSpecific")]
    #[link_name = "OSSetThreadSpecific"]
    pub unsafe fn set_specific(index: i32, value: *mut ffi::c_void);

    /// OSSetThreadCleanupCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "OSSetThreadCleanupCallback")]
    #[link_name = "OSSetThreadCleanupCallback"]
    pub unsafe fn set_cleanup_callback(
        thread: *mut Thread,
        callback: CleanupCallback,
    ) -> CleanupCallback;

    /// OSSetThreadDeallocator
    ///
    /// Returns the previous deallocator.
    #[doc(alias = "OSSetThreadDeallocator")]
    #[link_name = "OSSetThreadDeallocator"]
    pub unsafe fn set_deallocator(thread: *mut Thread, deallocator: Deallocator) -> Deallocator;

    /// OSGetThreadCoreTime
    ///
    /// Time `thread` ran on the cores in `affinity`.
    #[doc(alias = "OSGetThreadCoreTime")]
    #[link_name = "OSGetThreadCoreTime"]
    pub unsafe fn get_core_time(thread: *const Thread, affinity: ThreadAttributes) -> Time;

    /// OSGetActiveThreadLink
    #[doc(alias = "OSGetActiveThreadLink")]
    #[link_name = "OSGetActiveThreadLink"]
    pub unsafe fn get_active_thread_link(thread: *mut Thread, link: *mut ThreadLink)
    -> ffi::c_bool;

    /// OSCheckActiveThreads
    ///
    /// Returns the number of active threads.
    #[doc(alias = "OSCheckActiveThreads")]
    #[link_name = "OSCheckActiveThreads"]
    pub unsafe fn check_active_threads() -> i32;

    /// OSGetStackPointer
    #[doc(alias = "OSGetStackPointer")]
    #[link_name = "OSGetStackPointer"]
    pub unsafe fn get_stack_pointer() -> u32;

    /// OSGetUserStackPointer
    #[doc(alias = "OSGetUserStackPointer")]
    #[link_name = "OSGetUserStackPointer"]
    pub unsafe fn get_user_stack_pointer(thread: *mut Thread) -> u32;

    /// OSClearStack
    ///
    /// Fills the unused part of the current stack with `value`.
    #[doc(alias = "OSClearStack")]
    #[link_name = "OSClearStack"]
    pub unsafe fn clear_stack(value: u8);

    /// OSSetThreadStackUsage
    ///
    /// Starts tracking the stack usage of `thread`, or the current thread if null.
    #[doc(alias = "OSSetThreadStackUsage")]
    #[link_name = "OSSetThreadStackUsage"]
    pub unsafe fn set_stack_usage(thread: *mut Thread) -> ffi::c_bool;

    /// OSClearThreadStackUsage
    #[doc(alias = "OSClearThreadStackUsage")]
    #[link_name = "OSClearThreadStackUsage"]
    pub unsafe fn clear_stack_usage(thread: *mut Thread);

    /// OSCheckThreadStackUsage
    ///
    /// Returns the maximum stack usage in bytes.
    #[doc(alias = "OSCheckThreadStackUsage")]
    #[link_name = "OSCheckThreadStackUsage"]
    pub unsafe fn check_stack_usage(thread: *mut Thread) -> i32;

    /// __OSBoostThreadPriority
    #[doc(alias = "__OSBoostThreadPriority")]
    #[link_name = "__OSBoostThreadPriority"]
    pub unsafe fn boost_priority(thread: *mut Thread);

    /// __OSDeboostThreadPrioriy
    #[doc(alias = "__OSDeboostThreadPrioriy")]
    #[link_name = "__OSDeboostThreadPrioriy"]
    pub unsafe fn deboost_priority(thread: *mut Thread);
}
