use super::context::Context;
use crate::ffi;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// OSInterruptType
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum InterruptType {
    Error = 0,
    Dsp = 1,
    Gpu7 = 2,
    Gpippc = 3,
    PrimaryI2C = 4,
    Dspai = 5,
    Dspai2 = 6,
    Dspacc = 7,
    Dspdsp = 8,
    Ipccpc0 = 9,
    Ipccp1 = 10,
    Ipccp2 = 11,
    Ahb = 12,
}

/// OSUserInterruptHandler
pub type UserInterruptHandler =
    Option<unsafe extern "C" fn(r#type: InterruptType, interrupted_context: *mut Context)>;

#[cfg(target_arch = "powerpc")]
unsafe extern "C" {

    /// OSEnableInterrupts
    #[link_name = "OSEnableInterrupts"]
    pub unsafe fn enable() -> ffi::c_bool;

    /// OSDisableInterrupts
    #[link_name = "OSDisableInterrupts"]
    pub unsafe fn disable() -> ffi::c_bool;

    /// OSRestoreInterrupts
    #[link_name = "OSRestoreInterrupts"]
    pub unsafe fn restore(enable: ffi::c_bool) -> ffi::c_bool;

    /// OSIsInterruptEnabled
    #[link_name = "OSIsInterruptEnabled"]
    pub unsafe fn is_enabeld() -> ffi::c_bool;

    /// __OSSetInterruptHandler
    #[link_name = "__OSSetInterruptHandler"]
    pub unsafe fn set_handler(
        r#type: InterruptType,
        handler: UserInterruptHandler,
    ) -> UserInterruptHandler;

    /// __OSClearAndEnableInterrupt
    #[link_name = "__OSClearAndEnableInterrupt"]
    pub unsafe fn enable_cleared(r#type: InterruptType);

    /// __OSDisableInterrupt
    #[link_name = "__OSDisableInterrupt"]
    pub unsafe fn disable_type(r#type: InterruptType);
}
