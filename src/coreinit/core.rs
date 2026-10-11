//! Core
//!
//! CPU cores and console type.

/// MAX_SYSTEM_CORES
#[doc(alias = "MAX_SYSTEM_CORES")]
pub const MAX_CORES: u32 = 3;

/// OS_CONSOLE_TYPE_MASK
#[doc(alias = "OS_CONSOLE_TYPE_MASK")]
pub const CONSOLE_TYPE_MASK: u32 = 0xF000_0000;

unsafe extern "C" {
    /// OSGetCoreCount
    #[doc(alias = "OSGetCoreCount")]
    #[link_name = "OSGetCoreCount"]
    pub unsafe fn get_core_count() -> u32;

    /// OSGetCoreId
    #[doc(alias = "OSGetCoreId")]
    #[link_name = "OSGetCoreId"]
    pub unsafe fn get_core_id() -> u32;

    /// OSGetMainCoreId
    #[doc(alias = "OSGetMainCoreId")]
    #[link_name = "OSGetMainCoreId"]
    pub unsafe fn get_main_core_id() -> u32;

    /// OSGetConsoleType
    #[doc(alias = "OSGetConsoleType")]
    #[link_name = "OSGetConsoleType"]
    pub unsafe fn get_console_type() -> u32;
}
