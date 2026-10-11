//! Spin lock
//!
//! Busy-waiting recursive lock.

/// OSSpinLock
#[doc(alias = "OSSpinLock")]
#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpinLock {
    /// Owning thread.
    pub owner: u32,
    pub id: u32,
    pub recursion: u32,
    /// Interrupt state before an uninterruptible acquire.
    pub interrupts_enabled: u32,
}

const _: () = {
    assert!(size_of::<SpinLock>() == 0x10);
};

unsafe extern "C" {
    /// OSInitSpinLock
    #[doc(alias = "OSInitSpinLock")]
    #[link_name = "OSInitSpinLock"]
    pub unsafe fn init(lock: *mut SpinLock);

    /// OSAcquireSpinLock
    #[doc(alias = "OSAcquireSpinLock")]
    #[link_name = "OSAcquireSpinLock"]
    pub unsafe fn acquire(lock: *mut SpinLock) -> crate::ffi::c_bool;

    /// OSTryAcquireSpinLock
    #[doc(alias = "OSTryAcquireSpinLock")]
    #[link_name = "OSTryAcquireSpinLock"]
    pub unsafe fn try_acquire(lock: *mut SpinLock) -> crate::ffi::c_bool;

    /// OSTryAcquireSpinLockWithTimeout
    #[doc(alias = "OSTryAcquireSpinLockWithTimeout")]
    #[link_name = "OSTryAcquireSpinLockWithTimeout"]
    pub unsafe fn try_acquire_with_timeout(
        lock: *mut SpinLock,
        timeout_ns: i64,
    ) -> crate::ffi::c_bool;

    /// OSReleaseSpinLock
    #[doc(alias = "OSReleaseSpinLock")]
    #[link_name = "OSReleaseSpinLock"]
    pub unsafe fn release(lock: *mut SpinLock) -> crate::ffi::c_bool;

    /// OSUninterruptibleSpinLock_Acquire
    ///
    /// Same as [acquire], also disables interrupts until released.
    #[doc(alias = "OSUninterruptibleSpinLock_Acquire")]
    #[link_name = "OSUninterruptibleSpinLock_Acquire"]
    pub unsafe fn uninterruptible_acquire(lock: *mut SpinLock) -> crate::ffi::c_bool;

    /// OSUninterruptibleSpinLock_TryAcquire
    #[doc(alias = "OSUninterruptibleSpinLock_TryAcquire")]
    #[link_name = "OSUninterruptibleSpinLock_TryAcquire"]
    pub unsafe fn uninterruptible_try_acquire(lock: *mut SpinLock) -> crate::ffi::c_bool;

    /// OSUninterruptibleSpinLock_TryAcquireWithTimeout
    #[doc(alias = "OSUninterruptibleSpinLock_TryAcquireWithTimeout")]
    #[link_name = "OSUninterruptibleSpinLock_TryAcquireWithTimeout"]
    pub unsafe fn uninterruptible_try_acquire_with_timeout(
        lock: *mut SpinLock,
        timeout_ns: i64,
    ) -> crate::ffi::c_bool;

    /// OSUninterruptibleSpinLock_Release
    #[doc(alias = "OSUninterruptibleSpinLock_Release")]
    #[link_name = "OSUninterruptibleSpinLock_Release"]
    pub unsafe fn uninterruptible_release(lock: *mut SpinLock) -> crate::ffi::c_bool;
}
