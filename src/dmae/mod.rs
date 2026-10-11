//! Direct Memory Access Engine

use crate::ffi;

pub const DEFAULT_TIMEOUT_MS: u32 = 30 * 1000;

/// DMAETimeStamp
#[doc(alias = "DMAETimeStamp")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeStamp(pub u64);

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    #[default]
    None = 0,
    /// Swap bytes within each 16-bit value.
    Swap8In16 = 1,
    /// Swap bytes within each 32-bit value.
    Swap8In32 = 2,
    /// Swap bytes within each 64-bit value.
    Swap8In64 = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemaphoreAction {
    Wait = 0,
    Signal = 1,
}

unsafe extern "C" {
    /// DMAEWaitDone
    ///
    /// Returns [ffi::FALSE] if the wait timed out.
    #[doc(alias = "DMAEWaitDone")]
    #[link_name = "DMAEWaitDone"]
    pub unsafe fn wait_done(ts: TimeStamp) -> ffi::c_bool;

    /// DMAEGetLastSubmittedTimeStamp
    #[doc(alias = "DMAEGetLastSubmittedTimeStamp")]
    #[link_name = "DMAEGetLastSubmittedTimeStamp"]
    pub unsafe fn get_last_submitted_timestamp() -> TimeStamp;

    /// DMAEGetRetiredTimeStamp
    ///
    /// Time stamp of the most recently completed command.
    #[doc(alias = "DMAEGetRetiredTimeStamp")]
    #[link_name = "DMAEGetRetiredTimeStamp"]
    pub unsafe fn get_retired_timestamp() -> TimeStamp;

    /// DMAESemaphore
    ///
    /// Synchronizes the DMA engine with the GPU.
    #[doc(alias = "DMAESemaphore")]
    #[link_name = "DMAESemaphore"]
    pub unsafe fn semaphore(semaphore: *mut u64, action: SemaphoreAction) -> TimeStamp;

    /// DMAECopyMem
    ///
    /// `count` is in units of `u32`.
    #[doc(alias = "DMAECopyMem")]
    #[link_name = "DMAECopyMem"]
    pub unsafe fn copy_memory(
        dst: *mut ffi::c_void,
        src: *const ffi::c_void,
        count: u32,
        endian: Endian,
    ) -> TimeStamp;

    /// DMAEFillMem
    ///
    /// `count` is in units of `u32`.
    #[doc(alias = "DMAEFillMem")]
    #[link_name = "DMAEFillMem"]
    pub unsafe fn fill_memory(dst: *mut ffi::c_void, value: u32, count: u32) -> TimeStamp;

    /// DMAEFillMemPhys
    ///
    /// Like [fill_memory], but `dst` is a physical address.
    #[doc(alias = "DMAEFillMemPhys")]
    #[link_name = "DMAEFillMemPhys"]
    pub unsafe fn fill_memory_phys(dst: u32, value: u32, count: u32) -> TimeStamp;

    /// DMAESetTimeout
    #[doc(alias = "DMAESetTimeout")]
    #[link_name = "DMAESetTimeout"]
    pub unsafe fn set_timeout(ms: u32);

    /// DMAEGetTimeout
    #[doc(alias = "DMAEGetTimeout")]
    #[link_name = "DMAEGetTimeout"]
    pub unsafe fn get_timeout() -> u32;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "dmae",
    [
        "DMAECopyMem",
        "DMAEFillMem",
        "DMAEFillMemPhys",
        "DMAEGetLastSubmittedTimeStamp",
        "DMAEGetRetiredTimeStamp",
        "DMAEGetTimeout",
        "DMAESemaphore",
        "DMAESetTimeout",
        "DMAEWaitDone",
        "_DMAESubmit",
        "_DMAESubmitToRing"
    ],
    []
);
