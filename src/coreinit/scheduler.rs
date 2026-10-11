//! Scheduler
//!
//! Internal scheduler lock.

use crate::ffi;

unsafe extern "C" {
    /// OSIsSchedulerLocked
    #[doc(alias = "OSIsSchedulerLocked")]
    #[link_name = "OSIsSchedulerLocked"]
    pub unsafe fn is_locked(lock_id: *mut ffi::c_void) -> ffi::c_bool;

    /// __OSEnableScheduler
    #[doc(alias = "__OSEnableScheduler")]
    #[link_name = "__OSEnableScheduler"]
    pub unsafe fn enable();

    /// __OSDisableScheduler
    #[doc(alias = "__OSDisableScheduler")]
    #[link_name = "__OSDisableScheduler"]
    pub unsafe fn disable();

    /// __OSLockScheduler
    #[doc(alias = "__OSLockScheduler")]
    #[link_name = "__OSLockScheduler"]
    pub unsafe fn lock(lock_id: *mut ffi::c_void);

    /// __OSUnlockScheduler
    #[doc(alias = "__OSUnlockScheduler")]
    #[link_name = "__OSUnlockScheduler"]
    pub unsafe fn unlock(lock_id: *mut ffi::c_void);

    /// __OSTryLockScheduler
    #[doc(alias = "__OSTryLockScheduler")]
    #[link_name = "__OSTryLockScheduler"]
    pub unsafe fn try_lock(lock_id: *mut ffi::c_void);

    /// __OSTouchSchedulerLock
    #[doc(alias = "__OSTouchSchedulerLock")]
    #[link_name = "__OSTouchSchedulerLock"]
    pub unsafe fn touch_lock();
}
