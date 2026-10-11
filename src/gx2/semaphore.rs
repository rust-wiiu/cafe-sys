//! GX2 semaphores

use super::enums::SemaphoreAction;

/// GX2Semaphore
///
/// Semaphore value in memory, shared with DMAE.
#[doc(alias = "GX2Semaphore")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Semaphore(pub u64);

unsafe extern "C" {
    /// GX2SetSemaphore
    ///
    /// Has the GPU wait on or signal `semaphore`.
    #[doc(alias = "GX2SetSemaphore")]
    #[link_name = "GX2SetSemaphore"]
    pub unsafe fn set_semaphore(semaphore: *mut Semaphore, action: SemaphoreAction);
}
