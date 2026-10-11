//! Task queue (MP)
//!
//! Queue of tasks run by multiple cores.

use super::spinlock::SpinLock;
use super::time::Time;
use crate::ffi;

/// MPTaskState
#[doc(alias = "MPTaskState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskState(pub u32);

impl TaskState {
    pub const INITIALIZED: Self = Self(1 << 0);
    pub const READY: Self = Self(1 << 1);
    pub const RUNNING: Self = Self(1 << 2);
    pub const FINISHED: Self = Self(1 << 3);
}

/// MPTaskQueueState
#[doc(alias = "MPTaskQueueState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskQueueState(pub u32);

impl TaskQueueState {
    pub const INITIALIZED: Self = Self(1 << 0);
    pub const RUNNING: Self = Self(1 << 1);
    pub const STOPPING: Self = Self(1 << 2);
    pub const STOPPED: Self = Self(1 << 3);
    pub const FINISHED: Self = Self(1 << 4);
}

impl core::ops::BitOr for TaskQueueState {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// MPTaskFunc
#[doc(alias = "MPTaskFunc")]
pub type TaskFunc = Option<unsafe extern "C" fn(data: *mut ffi::c_void, data_len: u32) -> u32>;

/// MPTaskInfo
#[doc(alias = "MPTaskInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskInfo {
    pub state: TaskState,
    pub result: u32,
    pub core_id: u32,
    pub duration: Time,
}

/// MPTask
#[doc(alias = "MPTask")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: u32,
    pub queue: *mut TaskQueue,
    pub state: TaskState,
    pub func: TaskFunc,
    pub data: *mut ffi::c_void,
    pub data_len: u32,
    pub result: u32,
    pub core_id: u32,
    pub duration: Time,
    pub user_data: *mut ffi::c_void,
}

/// MPTaskQueueInfo
#[doc(alias = "MPTaskQueueInfo", alias = "MPTaskQInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskQueueInfo {
    pub state: TaskQueueState,
    pub tasks: u32,
    pub tasks_ready: u32,
    pub tasks_running: u32,
    pub tasks_finished: u32,
}

/// MPTaskQueue
#[doc(alias = "MPTaskQueue", alias = "MPTaskQ")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskQueue {
    pub id: u32,
    pub state: TaskQueueState,
    pub tasks: u32,
    pub tasks_ready: u32,
    pub tasks_running: u32,
    /// Finished tasks per core.
    pub tasks_finished: [u32; 3],
    pub next_index: [i32; 3],
    pub end_index: [i32; 3],
    pub queue: *mut *mut Task,
    pub queue_len: u32,
    pub lock: SpinLock,
}

const _: () = {
    assert!(size_of::<TaskInfo>() == 0x18);
    assert!(size_of::<TaskQueueInfo>() == 0x14);
};

unsafe extern "C" {
    /// MPInitTaskQ
    ///
    /// `queue_buffer` holds up to `queue_len` tasks.
    #[doc(alias = "MPInitTaskQ")]
    #[link_name = "MPInitTaskQ"]
    pub unsafe fn init_queue(queue: *mut TaskQueue, queue_buffer: *mut *mut Task, queue_len: u32);

    /// MPTermTaskQ
    #[doc(alias = "MPTermTaskQ")]
    #[link_name = "MPTermTaskQ"]
    pub unsafe fn term_queue(queue: *mut TaskQueue) -> ffi::c_bool;

    /// MPGetTaskQInfo
    #[doc(alias = "MPGetTaskQInfo")]
    #[link_name = "MPGetTaskQInfo"]
    pub unsafe fn get_queue_info(queue: *mut TaskQueue, info: *mut TaskQueueInfo) -> ffi::c_bool;

    /// MPStartTaskQ
    #[doc(alias = "MPStartTaskQ")]
    #[link_name = "MPStartTaskQ"]
    pub unsafe fn start_queue(queue: *mut TaskQueue) -> ffi::c_bool;

    /// MPStopTaskQ
    #[doc(alias = "MPStopTaskQ")]
    #[link_name = "MPStopTaskQ"]
    pub unsafe fn stop_queue(queue: *mut TaskQueue) -> ffi::c_bool;

    /// MPResetTaskQ
    #[doc(alias = "MPResetTaskQ")]
    #[link_name = "MPResetTaskQ"]
    pub unsafe fn reset_queue(queue: *mut TaskQueue) -> ffi::c_bool;

    /// MPRunTasksFromTaskQ
    ///
    /// Runs tasks on the current core, dequeuing `granularity` tasks at a time.
    #[doc(alias = "MPRunTasksFromTaskQ")]
    #[link_name = "MPRunTasksFromTaskQ"]
    pub unsafe fn run_tasks_from_queue(
        queue: *mut TaskQueue,
        granularity: ffi::c_int,
    ) -> ffi::c_bool;

    /// MPWaitTaskQ
    ///
    /// Waits until the queue reaches one of the states in `mask`.
    #[doc(alias = "MPWaitTaskQ")]
    #[link_name = "MPWaitTaskQ"]
    pub unsafe fn wait_queue(queue: *mut TaskQueue, mask: TaskQueueState) -> ffi::c_bool;

    /// MPWaitTaskQWithTimeout
    ///
    /// Returns false on timeout.
    #[doc(alias = "MPWaitTaskQWithTimeout")]
    #[link_name = "MPWaitTaskQWithTimeout"]
    pub unsafe fn wait_queue_with_timeout(
        queue: *mut TaskQueue,
        mask: TaskQueueState,
        timeout_ns: i64,
    ) -> ffi::c_bool;

    /// MPPrintTaskQStats
    #[doc(alias = "MPPrintTaskQStats")]
    #[link_name = "MPPrintTaskQStats"]
    pub unsafe fn print_queue_stats(queue: *mut TaskQueue, size: ffi::c_int) -> ffi::c_bool;

    /// MPEnqueTask
    #[doc(alias = "MPEnqueTask")]
    #[link_name = "MPEnqueTask"]
    pub unsafe fn enqueue(queue: *mut TaskQueue, task: *mut Task) -> ffi::c_bool;

    /// MPDequeTask
    #[doc(alias = "MPDequeTask")]
    #[link_name = "MPDequeTask"]
    pub unsafe fn dequeue(queue: *mut TaskQueue) -> *mut Task;

    /// MPDequeTasks
    ///
    /// Returns the number of tasks dequeued into `tasks`.
    #[doc(alias = "MPDequeTasks")]
    #[link_name = "MPDequeTasks"]
    pub unsafe fn dequeue_many(
        queue: *mut TaskQueue,
        tasks: *mut *mut Task,
        max_tasks: ffi::c_int,
    ) -> u32;

    /// MPInitTask
    #[doc(alias = "MPInitTask")]
    #[link_name = "MPInitTask"]
    pub unsafe fn init(task: *mut Task, func: TaskFunc, data: *mut ffi::c_void, data_len: u32);

    /// MPTermTask
    #[doc(alias = "MPTermTask")]
    #[link_name = "MPTermTask"]
    pub unsafe fn term(task: *mut Task) -> ffi::c_bool;

    /// MPGetTaskInfo
    #[doc(alias = "MPGetTaskInfo")]
    #[link_name = "MPGetTaskInfo"]
    pub unsafe fn get_info(task: *mut Task, info: *mut TaskInfo) -> ffi::c_bool;

    /// MPRunTask
    ///
    /// Runs a single task on the current core.
    #[doc(alias = "MPRunTask")]
    #[link_name = "MPRunTask"]
    pub unsafe fn run(task: *mut Task) -> ffi::c_bool;

    /// MPGetTaskUserData
    #[doc(alias = "MPGetTaskUserData")]
    #[link_name = "MPGetTaskUserData"]
    pub unsafe fn get_user_data(task: *mut Task) -> *mut ffi::c_void;

    /// MPSetTaskUserData
    #[doc(alias = "MPSetTaskUserData")]
    #[link_name = "MPSetTaskUserData"]
    pub unsafe fn set_user_data(task: *mut Task, user_data: *mut ffi::c_void);
}
