//! Interrupts

use super::context::Context;
use crate::ffi;

/// OSInterruptType
#[doc(alias = "OSInterruptType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterruptType(pub u32);

impl InterruptType {
    pub const ERROR: Self = Self(0);
    pub const DSP: Self = Self(1);
    pub const GPU7: Self = Self(2);
    pub const GPIPPC: Self = Self(3);
    pub const PRIMARY_I2C: Self = Self(4);
    pub const DSPAI: Self = Self(5);
    pub const DSPAI2: Self = Self(6);
    pub const DSPACC: Self = Self(7);
    pub const DSPDSP: Self = Self(8);
    pub const IPCPPC0: Self = Self(9);
    pub const IPCPPC1: Self = Self(10);
    pub const IPCPPC2: Self = Self(11);
    pub const AHB: Self = Self(12);
}

/// OSUserInterruptHandler
#[doc(alias = "OSUserInterruptHandler")]
pub type UserInterruptHandler =
    Option<unsafe extern "C" fn(kind: InterruptType, interrupted_context: *mut Context)>;

unsafe extern "C" {
    /// OSEnableInterrupts
    ///
    /// Returns whether interrupts were enabled before.
    #[doc(alias = "OSEnableInterrupts")]
    #[link_name = "OSEnableInterrupts"]
    pub unsafe fn enable() -> ffi::c_bool;

    /// OSDisableInterrupts
    ///
    /// Returns whether interrupts were enabled before, pass it to [restore].
    #[doc(alias = "OSDisableInterrupts")]
    #[link_name = "OSDisableInterrupts"]
    pub unsafe fn disable() -> ffi::c_bool;

    /// OSRestoreInterrupts
    #[doc(alias = "OSRestoreInterrupts")]
    #[link_name = "OSRestoreInterrupts"]
    pub unsafe fn restore(enabled: ffi::c_bool) -> ffi::c_bool;

    /// OSIsInterruptEnabled
    #[doc(alias = "OSIsInterruptEnabled")]
    #[link_name = "OSIsInterruptEnabled"]
    pub unsafe fn is_enabled() -> ffi::c_bool;

    /// __OSSetInterruptHandler
    ///
    /// Returns the previous handler.
    #[doc(alias = "__OSSetInterruptHandler")]
    #[link_name = "__OSSetInterruptHandler"]
    pub unsafe fn set_handler(
        kind: InterruptType,
        handler: UserInterruptHandler,
    ) -> UserInterruptHandler;

    /// __OSClearAndEnableInterrupt
    #[doc(alias = "__OSClearAndEnableInterrupt")]
    #[link_name = "__OSClearAndEnableInterrupt"]
    pub unsafe fn clear_and_enable_interrupt(kind: InterruptType);

    /// __OSDisableInterrupt
    #[doc(alias = "__OSDisableInterrupt")]
    #[link_name = "__OSDisableInterrupt"]
    pub unsafe fn disable_interrupt(kind: InterruptType);
}
