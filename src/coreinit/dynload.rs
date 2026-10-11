//! Dynamic loading
//!
//! Loading RPLs at runtime and looking up their exports.

use crate::ffi;

/// OSDynLoad_Module
#[doc(alias = "OSDynLoad_Module", alias = "OSDynLoad_ModuleHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Module(pub *mut ffi::c_void);

impl Module {
    /// The main RPX.
    pub const MAIN: Self = Self(core::ptr::without_provenance_mut(usize::MAX));
}

/// OSDynLoad_Error
#[doc(alias = "OSDynLoad_Error")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const UNKNOWN: Self = Self(0xBAD1_0001u32 as i32);
    pub const OUT_OF_MEMORY: Self = Self(0xBAD1_0002u32 as i32);
    pub const BAD_MODULE_HANDLE: Self = Self(0xBAD1_0003u32 as i32);
    pub const FILE_TOO_SMALL: Self = Self(0xBAD1_0004u32 as i32);
    pub const BAD_ELF_HEADER: Self = Self(0xBAD1_0005u32 as i32);
    pub const NOT_32BIT_ELF: Self = Self(0xBAD1_0006u32 as i32);
    pub const WRONG_ENDIANNESS: Self = Self(0xBAD1_0007u32 as i32);
    pub const UNSUPPORTED_HEADER_VERSION: Self = Self(0xBAD1_0008u32 as i32);
    pub const WRONG_TARGET_MACHINE: Self = Self(0xBAD1_0009u32 as i32);
    pub const UNSUPPORTED_FILE_VERSION: Self = Self(0xBAD1_000Au32 as i32);
    pub const NO_SECTION_HEADERS: Self = Self(0xBAD1_000Bu32 as i32);
    pub const MALFORMED_RPL: Self = Self(0xBAD1_000Cu32 as i32);
    pub const NOT_CAFE_RPL: Self = Self(0xBAD1_000Du32 as i32);
    pub const INVALID_NOTIFY_PTR: Self = Self(0xBAD1_000Eu32 as i32);
    pub const INVALID_MODULE_NAME_PTR: Self = Self(0xBAD1_000Fu32 as i32);
    pub const INVALID_MODULE_NAME: Self = Self(0xBAD1_0010u32 as i32);
    pub const INVALID_ACQUIRE_PTR: Self = Self(0xBAD1_0011u32 as i32);
    pub const EMPTY_MODULE_NAME: Self = Self(0xBAD1_0012u32 as i32);
    pub const LOADER_RETURNED_NO_DATA: Self = Self(0xBAD1_0013u32 as i32);
    pub const LOADER_PTR_BAD: Self = Self(0xBAD1_0014u32 as i32);
    pub const NULL_LOADER: Self = Self(0xBAD1_0015u32 as i32);
    pub const CANT_LOAD_INSIDE_MODULE_ENTRYPOINT: Self = Self(0xBAD1_0016u32 as i32);
    pub const INVALID_ALLOCATOR_PTR: Self = Self(0xBAD1_0017u32 as i32);
    pub const MODULE_HAS_NONRELOCATABLE_CODE: Self = Self(0xBAD1_0018u32 as i32);
    pub const CANT_LINK_BLIND_SYMBOL: Self = Self(0xBAD1_0019u32 as i32);
    pub const IMPORT_NOT_FOUND_IN_EXPORTING_MODULE: Self = Self(0xBAD1_001Au32 as i32);
    pub const MODULE_HAS_NO_DATA_EXPORTS: Self = Self(0xBAD1_001Bu32 as i32);
    pub const MODULE_HAS_NO_CODE_EXPORTS: Self = Self(0xBAD1_001Cu32 as i32);
    pub const EXPORT_NOT_FOUND: Self = Self(0xBAD1_001Du32 as i32);
    pub const BAD_ARGUMENT: Self = Self(0xBAD1_001Eu32 as i32);
    pub const BUFFER_TOO_SMALL: Self = Self(0xBAD1_001Fu32 as i32);
    pub const LOADER_FAILURE: Self = Self(0xBAD1_0021u32 as i32);
    pub const TAG_NOT_FOUND: Self = Self(0xBAD1_0022u32 as i32);
    pub const MODULE_NOT_FOUND: Self = Self(0xBAD1_0023u32 as i32);
    pub const ALLOC_FAILED_LD_SECT_INFO: Self = Self(0xBAD1_0024u32 as i32);
    pub const ALLOC_FAILED_MAIN_NOTIF_ENTRY: Self = Self(0xBAD1_0025u32 as i32);
    pub const FAILED_DEBUG_SET_CALLBACK: Self = Self(0xBAD1_0026u32 as i32);
    pub const NULL_OBJECT: Self = Self(0xBAD1_0027u32 as i32);
    pub const CONTAINING_MODULE_NOT_FOUND: Self = Self(0xBAD1_0028u32 as i32);
    pub const LOAD_ENTRYPOINT_FAILED: Self = Self(0xBAD1_0029u32 as i32);
    pub const UNKNOWN_MOD_FOR_HANDLE: Self = Self(0xBAD1_002Au32 as i32);
    pub const LOADER_BUSY: Self = Self(0xBAD1_002Bu32 as i32);
    pub const LOADER_MEM_INFO_BAD_PTR: Self = Self(0xBAD1_002Cu32 as i32);
    pub const NULL_LOADER_MEM_INFO: Self = Self(0xBAD1_002Du32 as i32);
    pub const PREVIOUS_LOADER_FAILURE: Self = Self(0xBAD1_002Eu32 as i32);
    pub const OUT_OF_SYSTEM_MEMORY: Self = Self(0xBAD1_002Fu32 as i32);
    pub const TLS_INCONSISTENCY: Self = Self(0xBAD1_0030u32 as i32);
    pub const TLS_ALLOCATOR_LOCKED: Self = Self(0xBAD1_0031u32 as i32);
    pub const TLS_TOO_MANY_MODULES: Self = Self(0xBAD1_0032u32 as i32);
    pub const TLS_FINDEXPORT_UNSUPPORTED: Self = Self(0xBAD1_0033u32 as i32);
    pub const TAG_ADDRSPACE_WRONG: Self = Self(0xBAD1_0034u32 as i32);
    pub const INVAL_TAG_VALUE_RET_BUF: Self = Self(0xBAD1_0035u32 as i32);
    pub const INVAL_TAG_VALUE_SIZE_RET_BUF: Self = Self(0xBAD1_0036u32 as i32);
    pub const INVAL_TAG_INPUT_BUF: Self = Self(0xBAD1_0037u32 as i32);
    pub const TAG_SNPRINTF_INTERNAL: Self = Self(0xBAD1_0038u32 as i32);
    pub const TAG_VALUE_SIZE_IS_ZERO: Self = Self(0xBAD1_0039u32 as i32);
    pub const MISSING_SHSTRTAB_INDEX: Self = Self(0xBAD1_003Au32 as i32);
    pub const MISSING_SHSTRTAB: Self = Self(0xBAD1_003Bu32 as i32);
    pub const MISSING_IMPORT_NAME: Self = Self(0xBAD1_003Cu32 as i32);
    pub const MISSING_TLS_DATA: Self = Self(0xBAD1_003Du32 as i32);
    pub const MISSING_RPX_ENTRYPOINT: Self = Self(0xBAD1_003Eu32 as i32);
    pub const DISPLACEMENT_TOO_FAR: Self = Self(0xBAD1_003Fu32 as i32);
    /// The module does not exist.
    pub const NOT_FOUND: Self = Self(0xFFFF_FFFAu32 as i32);
}

/// OSDynLoad_ExportType
#[doc(alias = "OSDynLoad_ExportType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportType {
    #[default]
    Func = 0,
    Data = 1,
}

/// OSDynLoad_SegmentRestrictionFlags
#[doc(alias = "OSDynLoad_SegmentRestrictionFlags")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentRestriction {
    #[default]
    None = 0,
    /// The object must be in the code segment.
    Code = 1,
    /// The object must be in the data segment.
    Data = 2,
}

/// OSDynLoad_NotifyReason
#[doc(alias = "OSDynLoad_NotifyReason")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotifyReason(pub ffi::c_int);

impl NotifyReason {
    pub const UNLOADED: Self = Self(0);
    pub const LOADED: Self = Self(1);
}

/// OSDynLoad_NotifyData
#[doc(alias = "OSDynLoad_NotifyData", alias = "OSDynLoad_NotifyHdr")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotifyData {
    /// Full path of the module.
    pub name: *mut ffi::c_char,
    pub text_addr: u32,
    pub text_offset: u32,
    pub text_size: u32,
    pub data_addr: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub read_addr: u32,
    pub read_offset: u32,
    pub read_size: u32,
}

/// OSDynLoad_LoaderHeapStatistics
#[doc(alias = "OSDynLoad_LoaderHeapStatistics")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoaderHeapStatistics {
    pub code_heap_used: u32,
    pub code_heap_num_allocations: u32,
    pub code_heap_free: u32,
    pub code_heap_largest_free: u32,
    pub data_heap_used: u32,
    pub data_heap_num_allocations: u32,
}

/// OSDynLoadAllocFn
#[doc(alias = "OSDynLoadAllocFn")]
pub type AllocFn =
    Option<unsafe extern "C" fn(size: i32, align: i32, address: *mut *mut ffi::c_void) -> Error>;

/// OSDynLoadFreeFn
#[doc(alias = "OSDynLoadFreeFn")]
pub type FreeFn = Option<unsafe extern "C" fn(address: *mut ffi::c_void)>;

/// OSDynLoadNotifyFunc
///
/// Called when an RPL is loaded or unloaded.
#[doc(alias = "OSDynLoadNotifyFunc")]
pub type NotifyCallback = Option<
    unsafe extern "C" fn(
        module: Module,
        user_context: *mut ffi::c_void,
        reason: NotifyReason,
        data: *const NotifyData,
    ),
>;

const _: () = {
    assert!(size_of::<LoaderHeapStatistics>() == 0x18);
};

unsafe extern "C" {
    /// OSDynLoad_Acquire
    ///
    /// Loads a module, or increases its reference count.
    #[doc(alias = "OSDynLoad_Acquire")]
    #[link_name = "OSDynLoad_Acquire"]
    pub unsafe fn acquire(name: *const ffi::c_char, module: *mut Module) -> Error;

    /// OSDynLoad_AcquireContainingModule
    ///
    /// Increases the reference count of the module containing `object`.
    #[doc(alias = "OSDynLoad_AcquireContainingModule")]
    #[link_name = "OSDynLoad_AcquireContainingModule"]
    pub unsafe fn acquire_containing_module(
        object: *mut ffi::c_void,
        restriction: SegmentRestriction,
        module: *mut Module,
    ) -> Error;

    /// OSDynLoad_IsModuleLoaded
    ///
    /// Same as [acquire] without loading the module.
    #[doc(alias = "OSDynLoad_IsModuleLoaded")]
    #[link_name = "OSDynLoad_IsModuleLoaded"]
    pub unsafe fn is_module_loaded(name: *const ffi::c_char, module: *mut Module) -> Error;

    /// OSDynLoad_Release
    #[doc(alias = "OSDynLoad_Release")]
    #[link_name = "OSDynLoad_Release"]
    pub unsafe fn release(module: Module);

    /// OSDynLoad_FindExport
    #[doc(alias = "OSDynLoad_FindExport")]
    #[link_name = "OSDynLoad_FindExport"]
    pub unsafe fn find_export(
        module: Module,
        kind: ExportType,
        name: *const ffi::c_char,
        address: *mut *mut ffi::c_void,
    ) -> Error;

    /// OSDynLoad_FindTag
    ///
    /// `size` is the buffer size on input and the value size on output.
    #[doc(alias = "OSDynLoad_FindTag")]
    #[link_name = "OSDynLoad_FindTag"]
    pub unsafe fn find_tag(
        module: Module,
        tag: *const ffi::c_char,
        value: *mut ffi::c_char,
        size: *mut i32,
    ) -> Error;

    /// OSDynLoad_GetModuleName
    ///
    /// `size` is the buffer size on input and the name size on output.
    #[doc(alias = "OSDynLoad_GetModuleName")]
    #[link_name = "OSDynLoad_GetModuleName"]
    pub unsafe fn get_module_name(module: Module, name: *mut ffi::c_char, size: *mut i32) -> Error;

    /// OSDynLoad_GetNumberOfRPLs
    ///
    /// Always 0 on retail consoles.
    #[doc(alias = "OSDynLoad_GetNumberOfRPLs")]
    #[link_name = "OSDynLoad_GetNumberOfRPLs"]
    pub unsafe fn get_number_of_rpls() -> i32;

    /// OSDynLoad_GetRPLInfo
    ///
    /// Returns false if an RPL was loaded since [get_number_of_rpls].
    #[doc(alias = "OSDynLoad_GetRPLInfo")]
    #[link_name = "OSDynLoad_GetRPLInfo"]
    pub unsafe fn get_rpl_info(first: u32, count: u32, infos: *mut NotifyData) -> ffi::c_bool;

    /// OSDynLoad_GetLoaderHeapStatistics
    #[doc(alias = "OSDynLoad_GetLoaderHeapStatistics")]
    #[link_name = "OSDynLoad_GetLoaderHeapStatistics"]
    pub unsafe fn get_loader_heap_statistics(statistics: *mut LoaderHeapStatistics) -> Error;

    /// OSDynLoad_SetAllocator
    #[doc(alias = "OSDynLoad_SetAllocator")]
    #[link_name = "OSDynLoad_SetAllocator"]
    pub unsafe fn set_allocator(alloc: AllocFn, free: FreeFn) -> Error;

    /// OSDynLoad_GetAllocator
    #[doc(alias = "OSDynLoad_GetAllocator")]
    #[link_name = "OSDynLoad_GetAllocator"]
    pub unsafe fn get_allocator(alloc: *mut AllocFn, free: *mut FreeFn) -> Error;

    /// OSDynLoad_SetTLSAllocator
    #[doc(alias = "OSDynLoad_SetTLSAllocator")]
    #[link_name = "OSDynLoad_SetTLSAllocator"]
    pub unsafe fn set_tls_allocator(alloc: AllocFn, free: FreeFn) -> Error;

    /// OSDynLoad_GetTLSAllocator
    #[doc(alias = "OSDynLoad_GetTLSAllocator")]
    #[link_name = "OSDynLoad_GetTLSAllocator"]
    pub unsafe fn get_tls_allocator(alloc: *mut AllocFn, free: *mut FreeFn) -> Error;

    /// OSDynLoad_AddNotifyCallback
    #[doc(alias = "OSDynLoad_AddNotifyCallback")]
    #[link_name = "OSDynLoad_AddNotifyCallback"]
    pub unsafe fn add_notify_callback(
        callback: NotifyCallback,
        user_context: *mut ffi::c_void,
    ) -> Error;

    /// OSDynLoad_AddNofifyCallback
    #[doc(alias = "OSDynLoad_AddNofifyCallback")]
    #[link_name = "OSDynLoad_AddNofifyCallback"]
    #[deprecated(note = "Use add_notify_callback instead")]
    pub unsafe fn add_nofify_callback(
        callback: NotifyCallback,
        user_context: *mut ffi::c_void,
    ) -> Error;

    /// OSDynLoad_DelNotifyCallback
    #[doc(alias = "OSDynLoad_DelNotifyCallback")]
    #[link_name = "OSDynLoad_DelNotifyCallback"]
    pub unsafe fn del_notify_callback(callback: NotifyCallback, user_context: *mut ffi::c_void);

    /// OSGetSecurityLevel
    #[doc(alias = "OSGetSecurityLevel")]
    #[link_name = "OSGetSecurityLevel"]
    pub unsafe fn get_security_level() -> u32;
}
