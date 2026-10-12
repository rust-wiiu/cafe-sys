//! Debug
//!
//! Console output, fatal errors, crash handling, the system log and the disassembler.

use super::dynload::{Module, NotifyData};
use super::system::Upid;
use crate::ffi;

/// OSFatalErrorMessageType
#[doc(alias = "OSFatalErrorMessageType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatalErrorMessageType(pub u32);

impl FatalErrorMessageType {
    pub const UNKNOWN: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const CORRUPTION: Self = Self(2);
    pub const FATAL_SYSTEM_OR_USB: Self = Self(3);
    pub const CORRUPTION_SLC: Self = Self(4);
    pub const CORRUPTION_USB: Self = Self(5);
    pub const STORAGE_REMOVED: Self = Self(6);
    pub const DISC_REMOVED: Self = Self(7);
    pub const CORRUPTION_DISC: Self = Self(8);
    pub const WRITE_PROTECT: Self = Self(9);
}

/// OSFatalError
#[doc(alias = "OSFatalError")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatalError {
    pub message_type: FatalErrorMessageType,
    pub error_code: u32,
    pub process_id: u32,
    pub internal_error_code: u32,
    pub line: u32,
    pub function_name: [ffi::c_char; 64],
    _unk: ffi::unknown<0x80>,
}

/// OSError
///
/// Last exception, see [get_last_error].
#[doc(alias = "OSError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LastError(pub u32);

impl LastError {
    pub const SYSTEM_RESET: Self = Self(0);
    pub const MACHINE_CHECK: Self = Self(1);
    pub const DSI: Self = Self(2);
    pub const ISI: Self = Self(3);
    pub const EXTERNAL_INTERRUPT: Self = Self(4);
    pub const ALIGNMENT: Self = Self(5);
    pub const PROGRAM: Self = Self(6);
    /// Floating-point unavailable.
    pub const FLOATING_POINT: Self = Self(7);
    pub const DECREMENTER: Self = Self(8);
    pub const SYSTEM_CALL: Self = Self(9);
    pub const TRACE: Self = Self(10);
    pub const PERFORMANCE_MONITOR: Self = Self(11);
    pub const BREAKPOINT: Self = Self(12);
    pub const SYSTEM_INTERRUPT: Self = Self(13);
    pub const THERMAL_INTERRUPT: Self = Self(14);
    pub const PROTECTION: Self = Self(15);
    /// Floating-point exception.
    pub const FPE: Self = Self(16);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrashControlMode(pub u32);

impl CrashControlMode {
    pub const NONE: Self = Self(0);
    pub const UNHANDLED: Self = Self(1);
    pub const RESTART: Self = Self(2);
    pub const REBOOT: Self = Self(3);
    pub const KILL: Self = Self(4);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrashControl {
    pub mode: CrashControlMode,
    pub message: *mut ffi::c_void,
    pub data0: u32,
    pub data1: u32,
    pub data2: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrashInfo {
    pub error: u32,
    pub upid: Upid,
    pub title_id: u64,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrashDumpType(pub u32);

impl CrashDumpType {
    pub const MINI: Self = Self(0);
    pub const FULL: Self = Self(1);
}

pub type PanicCallback = Option<unsafe extern "C" fn(arg: *mut ffi::c_void)>;

/// DisassemblePPCFlags
#[doc(alias = "DisassemblePPCFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisassemblyFlags(pub u32);

impl DisassemblyFlags {
    pub const NONE: Self = Self(0);
}

/// DisassemblyPrintFn
pub type DisassemblyPrintFn = Option<unsafe extern "C" fn(format: *const ffi::c_char, ...)>;

/// DisassemblyFindSymbolFn
///
/// Writes the symbol name to `name` and returns its address.
pub type DisassemblyFindSymbolFn =
    Option<unsafe extern "C" fn(addr: u32, name: *mut ffi::c_char, name_size: u32) -> u32>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogCategory(pub u32);

impl LogCategory {
    pub const OS: Self = Self(0);
    pub const OS_REPORT: Self = Self(1);
    pub const LOADER: Self = Self(2);
    pub const APP: Self = Self(3);
    pub const GFX: Self = Self(4);
    pub const DRC: Self = Self(5);
    pub const SOUND: Self = Self(6);
    pub const FS: Self = Self(7);
    pub const CUSTOM_RANGE_START: Self = Self(128);
    pub const ALL: Self = Self(256);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Noise = 0,
    Notice = 1,
    Warning = 2,
    Error = 3,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogEntryHeader {
    pub size: u32,
    pub time_stamp: u64,
    pub tag: u32,
    pub info0: u32,
    pub info1: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogRetrieval {
    pub total_size: u32,
    pub number_of_entries: u32,
    pub current_session: u32,
    /// First of `number_of_entries` entries.
    pub first_entry: LogEntryHeader,
}

const _: () = {
    assert!(size_of::<FatalError>() == 0xD4);
    assert!(size_of::<LogEntryHeader>() == 0x18);
    assert!(size_of::<LogRetrieval>() == 0x24);
};

unsafe extern "C" {
    /// OSReport
    #[doc(alias = "OSReport")]
    #[link_name = "OSReport"]
    pub unsafe fn report(format: *const ffi::c_char, ...);

    /// OSVReport
    ///
    /// `args` is a `va_list`.
    #[doc(alias = "OSVReport")]
    #[link_name = "OSVReport"]
    pub unsafe fn vreport(format: *const ffi::c_char, args: *mut ffi::c_void);

    /// OSReportVerbose
    #[doc(alias = "OSReportVerbose")]
    #[link_name = "OSReportVerbose"]
    pub unsafe fn report_verbose(format: *const ffi::c_char, ...);

    /// OSReportInfo
    #[doc(alias = "OSReportInfo")]
    #[link_name = "OSReportInfo"]
    pub unsafe fn report_info(format: *const ffi::c_char, ...);

    /// OSReportWarn
    #[doc(alias = "OSReportWarn")]
    #[link_name = "OSReportWarn"]
    pub unsafe fn report_warn(format: *const ffi::c_char, ...);

    /// OSConsoleWrite
    #[doc(alias = "OSConsoleWrite")]
    #[link_name = "OSConsoleWrite"]
    pub unsafe fn console_write(message: *const ffi::c_char, size: u32);

    /// __OSConsoleWrite
    #[doc(alias = "__OSConsoleWrite")]
    #[link_name = "__OSConsoleWrite"]
    pub unsafe fn console_write_raw(message: *const ffi::c_char, size: u32);

    /// OSSupressConsoleOutput
    #[doc(alias = "OSSupressConsoleOutput")]
    #[link_name = "OSSupressConsoleOutput"]
    pub unsafe fn suppress_console_output(user: ffi::c_bool, app: ffi::c_bool, kernel: ffi::c_bool);

    /// OSPanic
    ///
    /// Prints the message and halts.
    #[doc(alias = "OSPanic")]
    #[link_name = "OSPanic"]
    pub unsafe fn panic(
        file: *const ffi::c_char,
        line: ffi::c_int,
        format: *const ffi::c_char,
        ...
    ) -> !;

    /// OSFatal
    ///
    /// Shows the message on screen and halts.
    #[doc(alias = "OSFatal")]
    #[link_name = "OSFatal"]
    pub unsafe fn fatal(message: *const ffi::c_char) -> !;

    /// OSSendFatalError
    #[doc(alias = "OSSendFatalError")]
    #[link_name = "OSSendFatalError"]
    pub unsafe fn send_fatal_error(
        error: *mut FatalError,
        function_name: *const ffi::c_char,
        line: u32,
    );

    /// OSSetPanicCallback
    #[doc(alias = "OSSetPanicCallback")]
    #[link_name = "OSSetPanicCallback"]
    pub unsafe fn set_panic_callback(callback: PanicCallback, arg: *mut ffi::c_void);

    /// OSGetSymbolName
    ///
    /// Writes the name of the symbol containing `addr` to `name`. Returns the symbol address.
    #[doc(alias = "OSGetSymbolName")]
    #[link_name = "OSGetSymbolName"]
    pub unsafe fn get_symbol_name(addr: u32, name: *mut ffi::c_char, name_size: u32) -> u32;

    /// OSGetUPID
    #[doc(alias = "OSGetUPID")]
    #[link_name = "OSGetUPID"]
    pub unsafe fn get_upid() -> Upid;

    /// OSIsDebuggerInitialized
    #[doc(alias = "OSIsDebuggerInitialized")]
    #[link_name = "OSIsDebuggerInitialized"]
    pub unsafe fn is_debugger_initialized() -> ffi::c_bool;

    /// OSIsDebuggerPresent
    #[doc(alias = "OSIsDebuggerPresent")]
    #[link_name = "OSIsDebuggerPresent"]
    pub unsafe fn is_debugger_present() -> ffi::c_bool;

    /// OSIsECOBoot
    #[doc(alias = "OSIsECOBoot")]
    #[link_name = "OSIsECOBoot"]
    pub unsafe fn is_eco_boot() -> ffi::c_bool;

    /// OSIsECOMode
    #[doc(alias = "OSIsECOMode")]
    #[link_name = "OSIsECOMode"]
    pub unsafe fn is_eco_mode() -> ffi::c_bool;

    /// OSGetLastError
    #[doc(alias = "OSGetLastError")]
    #[link_name = "OSGetLastError"]
    pub unsafe fn get_last_error() -> LastError;

    /// __OSSetCrashRecovery
    #[doc(alias = "__OSSetCrashRecovery")]
    #[link_name = "__OSSetCrashRecovery"]
    pub unsafe fn set_crash_recovery(crash_recovery: u32);

    /// __OSGetCrashRecovery
    #[doc(alias = "__OSGetCrashRecovery")]
    #[link_name = "__OSGetCrashRecovery"]
    pub unsafe fn get_crash_recovery() -> u32;

    /// __OSGetAppCrashControl
    #[doc(alias = "__OSGetAppCrashControl")]
    #[link_name = "__OSGetAppCrashControl"]
    pub unsafe fn get_app_crash_control(control: *mut CrashControl);

    /// __OSSetAppCrashControl
    #[doc(alias = "__OSSetAppCrashControl")]
    #[link_name = "__OSSetAppCrashControl"]
    pub unsafe fn set_app_crash_control(control: *mut CrashControl);

    /// __OSClearAppCrash
    #[doc(alias = "__OSClearAppCrash")]
    #[link_name = "__OSClearAppCrash"]
    pub unsafe fn clear_app_crash();

    /// OSGetCrashInfo
    #[doc(alias = "OSGetCrashInfo")]
    #[link_name = "OSGetCrashInfo"]
    pub unsafe fn get_crash_info(info: *mut CrashInfo);

    /// OSRestartCrashedApp
    #[doc(alias = "OSRestartCrashedApp")]
    #[link_name = "OSRestartCrashedApp"]
    pub unsafe fn restart_crashed_app(args: *mut ffi::c_void, args_size: u32) -> ffi::c_bool;

    /// OSRebootCrash
    #[doc(alias = "OSRebootCrash")]
    #[link_name = "OSRebootCrash"]
    pub unsafe fn reboot_crash() -> ffi::c_bool;

    /// OSGetCrashDumpType
    #[doc(alias = "OSGetCrashDumpType")]
    #[link_name = "OSGetCrashDumpType"]
    pub unsafe fn get_crash_dump_type() -> CrashDumpType;

    /// OSSetCrashDumpType
    #[doc(alias = "OSSetCrashDumpType")]
    #[link_name = "OSSetCrashDumpType"]
    pub unsafe fn set_crash_dump_type(kind: CrashDumpType);

    /// __OSSetTestSetting
    #[doc(alias = "__OSSetTestSetting")]
    #[link_name = "__OSSetTestSetting"]
    pub unsafe fn set_test_setting(name: *const ffi::c_char, value: u32);

    /// __OSGetTestSetting
    #[doc(alias = "__OSGetTestSetting")]
    #[link_name = "__OSGetTestSetting"]
    pub unsafe fn get_test_setting(name: *const ffi::c_char) -> u32;

    /// MasterAgent_LoadNotify
    #[doc(alias = "MasterAgent_LoadNotify")]
    #[link_name = "MasterAgent_LoadNotify"]
    pub unsafe fn master_agent_load_notify(
        module: Module,
        user_arg: *mut ffi::c_void,
        is_load: ffi::c_int,
        data: *const NotifyData,
    );

    /// DisassemblePPCOpcode
    #[doc(alias = "DisassemblePPCOpcode")]
    #[link_name = "DisassemblePPCOpcode"]
    pub unsafe fn disassemble_ppc_opcode(
        opcode: *mut u32,
        buffer: *mut ffi::c_char,
        buffer_size: u32,
        find_symbol: DisassemblyFindSymbolFn,
        flags: DisassemblyFlags,
    ) -> ffi::c_bool;

    /// DisassemblePPCRange
    #[doc(alias = "DisassemblePPCRange")]
    #[link_name = "DisassemblePPCRange"]
    pub unsafe fn disassemble_ppc_range(
        start: *mut ffi::c_void,
        end: *mut ffi::c_void,
        print: DisassemblyPrintFn,
        find_symbol: DisassemblyFindSymbolFn,
        flags: DisassemblyFlags,
    );

    /// OSLogBuffer
    #[doc(alias = "OSLogBuffer")]
    #[link_name = "OSLogBuffer"]
    pub unsafe fn log_buffer(
        category: LogCategory,
        level: LogLevel,
        options: u32,
        buffer: *mut ffi::c_void,
        size: usize,
    );

    /// OSLogArgs
    #[doc(alias = "OSLogArgs")]
    #[link_name = "OSLogArgs"]
    pub unsafe fn log_args(
        category: LogCategory,
        level: LogLevel,
        options: u32,
        seq_byte: u32,
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
    );

    /// OSLogFunc
    #[doc(alias = "OSLogFunc")]
    #[link_name = "OSLogFunc"]
    pub unsafe fn log_func(
        category: LogCategory,
        level: LogLevel,
        options: u32,
        func_name: *const ffi::c_char,
        func_name_size: u32,
        line: u32,
        argv: *mut u32,
        argc: u32,
    );

    /// OSLogPrintf
    ///
    /// Up to 128 characters.
    #[doc(alias = "OSLogPrintf")]
    #[link_name = "OSLogPrintf"]
    pub unsafe fn log_printf(
        category: LogCategory,
        level: LogLevel,
        options: u32,
        format: *const ffi::c_char,
        ...
    );

    /// OSLogReport
    ///
    /// Prints the log to the console.
    #[doc(alias = "OSLogReport")]
    #[link_name = "OSLogReport"]
    pub unsafe fn log_report(
        options: u32,
        workspace: *mut ffi::c_void,
        workspace_size: u32,
    ) -> ffi::c_bool;

    /// OSLogRetrieve
    ///
    /// Returns false if no entries were retrieved.
    #[doc(alias = "OSLogRetrieve")]
    #[link_name = "OSLogRetrieve"]
    pub unsafe fn log_retrieve(
        options: u32,
        retrieval: *mut LogRetrieval,
        max_size: u32,
    ) -> ffi::c_bool;

    /// OSLogSaveCrashDump
    #[doc(alias = "OSLogSaveCrashDump")]
    #[link_name = "OSLogSaveCrashDump"]
    pub unsafe fn log_save_crash_dump() -> ffi::c_bool;

    /// OSLogSetFilterPolicy
    #[doc(alias = "OSLogSetFilterPolicy")]
    #[link_name = "OSLogSetFilterPolicy"]
    pub unsafe fn log_set_filter_policy(category: LogCategory, minimum_level: LogLevel);
}
