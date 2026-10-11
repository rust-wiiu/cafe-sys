//! Rendezvous
//!
//! Synchronization point between cores.

use crate::ffi;

/// OS_WAIT_CORE
///
/// Cores to wait for in [wait].
#[doc(alias = "OS_WAIT_CORE")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoreMask(pub u32);

impl CoreMask {
    pub const NONE: Self = Self(0);
    pub const CORE0: Self = Self(1 << 0);
    /// The main core.
    pub const CORE1: Self = Self(1 << 1);
    pub const CORE2: Self = Self(1 << 2);
    pub const ALL: Self = Self(0b111);
}

impl core::ops::BitOr for CoreMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSRendezvous
#[doc(alias = "OSRendezvous")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rendezvous {
    pub core: [u32; 3],
    pub id: u32,
}

const _: () = {
    assert!(size_of::<Rendezvous>() == 0x10);
};

unsafe extern "C" {
    /// OSInitRendezvous
    #[doc(alias = "OSInitRendezvous")]
    #[link_name = "OSInitRendezvous"]
    pub unsafe fn init(rendezvous: *mut Rendezvous);

    /// OSWaitRendezvous
    ///
    /// Marks the current core as arrived and waits for the cores in `mask`.
    #[doc(alias = "OSWaitRendezvous")]
    #[link_name = "OSWaitRendezvous"]
    pub unsafe fn wait(rendezvous: *mut Rendezvous, mask: CoreMask) -> ffi::c_bool;

    /// OSWaitRendezvousWithTimeout
    ///
    /// Returns false on timeout.
    #[doc(alias = "OSWaitRendezvousWithTimeout")]
    #[link_name = "OSWaitRendezvousWithTimeout"]
    pub unsafe fn wait_with_timeout(
        rendezvous: *mut Rendezvous,
        mask: CoreMask,
        timeout_ns: i64,
    ) -> ffi::c_bool;

    /// OSCoherencyBarrier
    #[doc(alias = "OSCoherencyBarrier")]
    #[link_name = "OSCoherencyBarrier"]
    pub unsafe fn coherency_barrier();
}
