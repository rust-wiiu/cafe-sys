use crate::ffi;

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {

    /// __OSSchedulerLock
    #[link_name = "__OSSchedulerLock"]
    pub unsafe static mut LOCK: *mut ffi::c_void;

    /// __OSEnableScheduler
    #[link_name = "__OSEnableScheduler"]
    pub unsafe fn enable();

    /// __OSDisableScheduler
    #[link_name = "__OSDisableScheduler"]
    pub unsafe fn disable();

    /// __OSLockScheduler
    #[link_name = "__OSLockScheduler"]
    pub unsafe fn lock(lockId: *mut ffi::c_void);

    /// __OSUnlockScheduler
    #[link_name = "__OSUnlockScheduler"]
    pub unsafe fn unlock(lockId: *mut ffi::c_void);

    /// OSIsSchedulerLocked
    #[link_name = "OSIsSchedulerLocked"]
    pub unsafe fn is_locked(lockId: *mut ffi::c_void) -> ffi::c_bool;

    /// __OSTryLockScheduler
    #[link_name = "__OSTryLockScheduler"]
    pub unsafe fn try_lock(lockId: *mut ffi::c_void);

    /// __OSTouchSchedulerLock
    #[link_name = "__OSTouchSchedulerLock"]
    pub unsafe fn touch_lock();
}
