use super::{
    Cpu, Link, Queue,
    alarm::Alarm,
    context::Context,
    exception::ExceptionCallbackFn,
    fast_mutex::{FastMutex, FastMutexQueue},
    mutex::{Mutex, MutexQueue},
    time::Time,
};
use crate::ffi;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ThreadState : u8 {
        const None = 0;
        const Ready = 1 << 0;
        const Running = 1 << 1;
        const Waiting = 1 << 2;
        const Moribund = 1 << 3;
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ThreadAttributes : u8 {
        const AffinityCpu0 = 1 << 0;
        const AffinityCpu1 = 1 << 1;
        const AffinityCpu2 = 1 << 2;
        const AffinityAny = Self::AffinityCpu0.bits() | Self::AffinityCpu1.bits() | Self::AffinityCpu2.bits();
        const Detached = 1 << 3;
        const StackUsage = 1 << 5;
        const Unknown = 1 << 7;
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ThreadType : u32 {
        const Driver = 0;
        const IO = 1;
        const App = 2;
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ThreadRequest : u32 {
        const None = 0;
        const Suspend = 1;
        const Cancel = 2;
    }
}

/// OSThreadQueue
pub type ThreadQueue = Queue<Thread>;

/// OSThreadLink
pub type ThreadLink = Link<Thread>;

/// OSThreadEntryPointFn
pub type ThreadEntryPointFn =
    Option<unsafe extern "C" fn(argc: i32, argv: *mut *const ffi::c_char) -> i32>;

/// OSThreadCleanupCallbackFn
pub type ThreadCleanupCallbackFn =
    Option<unsafe extern "C" fn(thread: *mut Thread, stack: *mut ffi::c_void)>;

/// OSThreadDeallocatorFn
pub type ThreadDeallocatorFn =
    Option<unsafe extern "C" fn(thread: *mut Thread, stack: *mut ffi::c_void)>;

/// OSThreadGHSExceptionHandling
#[repr(C, packed)]
#[derive(Clone)]
pub struct ThreadGHSExceptionHandling {
    _unk: ffi::unknown<0x68>,
    pub eh_globals: *mut ffi::c_void,
    pub eh_mem_manage: [*mut ffi::c_void; 9],
    pub eh_store_globals: [*mut ffi::c_void; 6],
    pub eh_store_globals_tdeh: [*mut ffi::c_void; 76],
}

/// OSTLSSection
#[repr(C)]
#[derive(Clone)]
pub struct TlsSection {
    pub data: *mut ffi::c_void,
    _unk: ffi::unknown<0x4>,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum SpecificID {
    Specific0 = 0,
    Specific1 = 1,
    Specific2 = 2,
    Specific3 = 3,
    Specific4 = 4,
    Specific5 = 5,
    Specific6 = 6,
    Specific7 = 7,
    Specific8 = 8,
    Specific9 = 9,
    Specific10 = 10,
    Specific11 = 11,
    Specific12 = 12,
    Specific13 = 13,
    Reserved0 = 14,
    Reserved1 = 15,
}

/// OSThread
#[repr(C, align(8))]
#[derive(Clone)]
pub struct Thread {
    pub context: Context,
    pub tag: u32,
    pub state: ThreadState,
    pub attr: ThreadAttributes,
    pub id: u16,
    pub suspend_counter: i32,
    pub priority: i32,
    pub base_priority: i32,
    pub exit_value: i32,
    pub core_run_queue: [*mut ThreadQueue; 3],
    pub core_run_queue_link: [*mut ThreadLink; 3],
    pub queue: *mut ThreadQueue,
    pub link: ThreadLink,
    pub join_queue: ThreadQueue,
    pub mutex: *mut Mutex,
    pub mutex_queue: MutexQueue,
    pub active_link: ThreadLink,
    pub stack_start: *mut ffi::c_void,
    pub stack_end: *mut ffi::c_void,
    pub entry_point: ThreadEntryPointFn,
    pub ghs_exception_handling: ThreadGHSExceptionHandling,
    pub alarm_canceled: ffi::c_bool,
    pub specific: [*const ffi::c_void; 0x10],
    pub r#type: ThreadType,
    pub name: *const ffi::c_char,
    pub wait_event_timeout_alaram: *mut Alarm,
    pub user_stack_pointer: *mut ffi::c_void,
    pub cleanup_callback: ThreadCleanupCallbackFn,
    pub deallocator: ThreadDeallocatorFn,
    pub cancel_state: ffi::c_bool,
    pub request_flag: ThreadRequest,
    pub need_suspend: i32,
    pub suspend_result: i32,
    pub suspend_queue: ThreadQueue,
    _unk0: ffi::unknown<0x4>,
    pub run_quantum_ticks: i64,
    pub core_time_consumed_ns: u64,
    pub wake_count: u64,
    _unk1: ffi::unknown<0x8>, // OSTime
    _unk2: ffi::unknown<0x8>, // OSTime
    _unk3: ffi::unknown<0x8>, // OSTime
    pub dsi_callback: [ExceptionCallbackFn; 3],
    pub isi_callback: [ExceptionCallbackFn; 3],
    pub program_callback: [ExceptionCallbackFn; 3],
    pub perf_mon_callback: [ExceptionCallbackFn; 3],
    pub stack_sync_obj_allowed: ffi::c_bool,
    pub tls_section_count: u16,
    _unk4: ffi::unknown<0x2>,
    pub tls_sections: *mut TlsSection,
    pub fast_mutex: *mut FastMutex,
    pub contended_fast_mutexes: FastMutexQueue,
    pub fast_mutex_queue: FastMutexQueue,
    pub align_callback: [ExceptionCallbackFn; 3],
    pub reserved: [u32; 5],
}

/// OSThreadSimpleQueue
#[repr(C)]
#[derive(Clone)]
pub struct ThreadSimpleQueue {
    pub head: *mut Thread,
    pub tail: *mut Thread,
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {

    /// OSCancelThread
    #[link_name = "OSCancelThread"]
    pub unsafe fn cancel(thread: *mut Thread);

    /// OSCheckActiveThreads
    #[link_name = "OSCheckActiveThreads"]
    pub unsafe fn active_threads() -> i32;

    /// OSCheckThreadStackUsage
    #[link_name = "OSCheckThreadStackUsage"]
    pub unsafe fn stack_usage(thread: *mut Thread) -> i32;

    /// OSClearThreadStackUsage
    #[link_name = "OSClearThreadStackUsage"]
    pub unsafe fn disable_stack_tracking(thread: *mut Thread);

    /// OSContinueThread
    #[link_name = "OSContinueThread"]
    pub unsafe fn resume(thread: *mut Thread);

    /// OSCreateThread
    #[link_name = "OSCreateThread"]
    pub unsafe fn create(
        thread: *mut Thread,
        entry: ThreadEntryPointFn,
        argc: i32,
        argv: *mut *const ffi::c_char,
        stack: *mut ffi::c_void,
        stack_size: u32,
        priority: i32,
        attributes: ThreadAttributes,
    ) -> ffi::c_bool;

    /// OSDetachThread
    #[link_name = "OSDetachThread"]
    pub unsafe fn detach(thread: *mut Thread);

    /// OSExitThread
    #[link_name = "OSExitThread"]
    pub unsafe fn exit(result: i32);

    /// OSGetActiveThreadLink
    #[link_name = "OSGetActiveThreadLink"]
    pub unsafe fn active_link(thread: *mut Thread, link: *mut ThreadLink);

    /// OSGetCurrentThread
    #[link_name = "OSGetCurrentThread"]
    pub unsafe fn current() -> *mut Thread;

    /// OSGetDefaultThread
    #[link_name = "OSGetDefaultThread"]
    pub unsafe fn default(core: Cpu) -> *mut Thread;

    /// OSGetStackPointer
    #[link_name = "OSGetStackPointer"]
    pub unsafe fn stack_pointer() -> u32;

    /// OSGetThreadAffinity
    #[link_name = "OSGetThreadAffinity"]
    pub unsafe fn get_affinity(thread: *mut Thread) -> u32;

    /// OSGetThreadName
    #[link_name = "OSGetThreadName"]
    pub unsafe fn get_name(thread: *mut Thread) -> *const ffi::c_char;

    /// OSGetThreadPriority
    #[link_name = "OSGetThreadPriority"]
    pub unsafe fn get_priority(thread: *mut Thread) -> i32;

    /// OSGetThreadSpecific
    #[link_name = "OSGetThreadSpecific"]
    pub unsafe fn get_specific(id: SpecificID) -> *mut ffi::c_void;

    /// OSIsThreadSuspended
    #[link_name = "OSIsThreadSuspended"]
    pub unsafe fn is_suspended(thread: *mut Thread) -> ffi::c_bool;

    /// OSIsThreadTerminated
    #[link_name = "OSIsThreadTerminated"]
    pub unsafe fn is_terminated(thread: *mut Thread) -> ffi::c_bool;

    /// OSJoinThread
    #[link_name = "OSJoinThread"]
    pub unsafe fn join(thread: *mut Thread, result: *mut i32) -> ffi::c_bool;

    /// OSResumeThread
    #[link_name = "OSResumeThread"]
    pub unsafe fn try_resume(thread: *mut Thread) -> i32;

    /// OSRunThread
    #[link_name = "OSRunThread"]
    pub unsafe fn reuse(
        thread: *mut Thread,
        entry: ThreadEntryPointFn,
        argc: i32,
        argv: *mut *const ffi::c_char,
    ) -> ffi::c_bool;

    /// OSSetThreadAffinity
    #[link_name = "OSSetThreadAffinity"]
    pub unsafe fn set_affinity(thread: *mut Thread, affinity: u32) -> ffi::c_bool;

    /// OSSetThreadCancelState
    #[link_name = "OSSetThreadCancelState"]
    pub unsafe fn set_cancel_state(state: ffi::c_bool) -> ffi::c_bool;

    /// OSSetThreadCleanupCallback
    #[link_name = "OSSetThreadCleanupCallback"]
    pub unsafe fn set_cleanup_callback(
        thread: *mut Thread,
        callback: ThreadCleanupCallbackFn,
    ) -> ThreadCleanupCallbackFn;

    /// OSSetThreadDeallocator
    #[link_name = "OSSetThreadDeallocator"]
    pub unsafe fn set_deallocator(
        thread: *mut Thread,
        deallocator: ThreadDeallocatorFn,
    ) -> ThreadDeallocatorFn;

    /// OSSetThreadName
    #[link_name = "OSSetThreadName"]
    pub unsafe fn set_name(thread: *mut Thread, name: *const ffi::c_char);

    /// OSSetThreadPriority
    #[link_name = "OSSetThreadPriority"]
    pub unsafe fn set_priority(thread: *mut Thread, priority: i32) -> ffi::c_bool;

    /// OSSetThreadRunQuantum
    #[link_name = "OSSetThreadRunQuantum"]
    pub unsafe fn set_run_quantum(thread: *mut Thread, quantum: u32) -> ffi::c_bool;

    /// OSSetThreadSpecific
    #[link_name = "OSSetThreadSpecific"]
    pub unsafe fn set_specific(id: SpecificID, value: *mut ffi::c_void);

    /// OSSetThreadStackUsage
    #[link_name = "OSSetThreadStackUsage"]
    pub unsafe fn enable_stack_tracking(thread: *mut Thread) -> ffi::c_bool;

    /// OSSleepThread
    #[link_name = "OSSleepThread"]
    pub unsafe fn sleep_until_wakeup(queue: *mut ThreadQueue);

    /// OSSleepTicks
    #[link_name = "OSSleepTicks"]
    pub unsafe fn sleep(ticks: Time);

    /// OSSuspendThread
    #[link_name = "OSSuspendThread"]
    pub unsafe fn suspend(thread: *mut Thread) -> u32;

    /// __OSSuspendThreadNolock
    #[link_name = "__OSSuspendThreadNolock"]
    pub unsafe fn suspend_no_lock(thread: *mut Thread);

    /// OSTestThreadCancel
    #[link_name = "OSTestThreadCancel"]
    pub unsafe fn test_cancel();

    /// OSWakeupThread
    #[link_name = "OSWakeupThread"]
    pub unsafe fn wakeup(queue: *mut ThreadQueue);

    /// OSYieldThread
    #[link_name = "OSYieldThread"]
    pub unsafe fn yield_now();
}
