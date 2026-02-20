// use crate::ffi::*;

unsafe extern "C" {
    /// Allows application to exit while in foreground. By default apps can only exit from the background.
    #[doc(alias = "OSEnableForegroundExit")]
    #[link_name = "OSEnableForegroundExit"]
    pub unsafe fn enable_foreground_exit();

    /// Must be called by all processor cores when a process is releasing the foreground.
    #[deprecated]
    #[doc(alias = "OSReleaseForeground")]
    #[link_name = "OSReleaseForeground"]
    pub unsafe fn release();

    /// Signifies to the OS that the application has completed any in-progress data operations.
    #[doc(alias = "OSSavesDone_ReadyToRelease")]
    #[link_name = "OSSavesDone_ReadyToRelease"]
    pub unsafe fn ready_to_release();
}
