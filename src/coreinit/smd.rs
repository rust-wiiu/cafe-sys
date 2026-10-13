//! SMD
//!
//! Shared memory message queues between the PPC and IOS.

use super::mutex::Mutex;
use crate::ffi;

/// SmdLockType
#[doc(alias = "SmdLockType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LockType(pub u32);

impl LockType {
    pub const MUTEX: Self = Self(0);
    pub const DISABLE_INTERRUPTS: Self = Self(1);
    pub const NONE: Self = Self(2);
}

/// SmdPpcState
#[doc(alias = "SmdPpcState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PpcState(pub u32);

impl PpcState {
    pub const INVALID: Self = Self(0);
    pub const INITIALIZED: Self = Self(1);
    pub const CLOSED: Self = Self(2);
    pub const OPENED: Self = Self(3);
}

/// SmdInterfaceState
#[doc(alias = "SmdInterfaceState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceState(pub u32);

impl InterfaceState {
    pub const OPENED: Self = Self(0x2222);
    pub const CLOSED: Self = Self(0x3333);
}

/// SmdElementType
#[doc(alias = "SmdElementType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementType(pub u32);

impl ElementType {
    pub const MESSAGE: Self = Self(0);
    pub const VECTOR_SPEC: Self = Self(1);
    pub const VECTOR: Self = Self(2);
}

/// SMD status code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// SmdVectorSpec
#[doc(alias = "SmdVectorSpec")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorSpec {
    pub ptr: *mut ffi::c_void,
    pub size: u32,
}

/// SmdVector
#[doc(alias = "SmdVector")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub command: u32,
    pub count: i32,
    pub vecs: [VectorSpec; 4],
}

/// Payload of an [Element].
#[repr(C)]
#[derive(Clone, Copy)]
pub union ElementData {
    pub data: [u8; 0xF8],
    pub spec: Vector,
    pub vector_paddr: u32,
}

/// SmdElement
#[doc(alias = "SmdElement")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Element {
    pub kind: ElementType,
    pub size: u32,
    pub data: ElementData,
}

/// Payload of a [ReceiveData].
#[repr(C)]
#[derive(Clone, Copy)]
pub union ReceiveDataPayload {
    pub message: [u8; 0x80],
    pub spec: Vector,
    pub vector: *mut Vector,
}

/// SmdReceiveData
#[doc(alias = "SmdReceiveData")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReceiveData {
    pub kind: ElementType,
    pub size: u32,
    pub payload: ReceiveDataPayload,
}

/// SmdInterface
#[doc(alias = "SmdInterface")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interface {
    pub state: InterfaceState,
    _pad0: ffi::unknown<0x7C>,
    pub element_count: u32,
    _pad1: ffi::unknown<0x7C>,
    pub read_offset: i32,
    _pad2: ffi::unknown<0x7C>,
    pub write_offset: i32,
    _pad3: ffi::unknown<0x7C>,
    pub buf_paddr: u32,
    _pad4: ffi::unknown<0x7C>,
}

/// SmdCtrlTable
#[doc(alias = "SmdCtrlTable")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtrlTable {
    pub name: [ffi::c_char; 0x10],
    pub reused_count: u32,
    _pad0: ffi::unknown<0x6C>,
    pub iop_interface: Interface,
    _pad1: ffi::unknown<0x40>,
    pub ppc_interface: Interface,
    _pad2: ffi::unknown<0x40>,
}

/// SmdPpcCtrlTableVectors
#[doc(alias = "SmdPpcCtrlTableVectors")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PpcCtrlTableVectors {
    pub ctrl_table: *mut CtrlTable,
    pub ctrl_table_size: u32,
    pub write_buf: *mut Element,
    pub write_buf_size: u32,
    pub read_buf: *mut Element,
    pub read_buf_size: u32,
}

/// SmdPpc
#[doc(alias = "SmdPpc")]
#[repr(C)]
pub struct Ppc {
    pub self_: *mut Ppc,
    pub ctrl_table: *mut CtrlTable,
    pub lock_type: LockType,
    pub mutex: Mutex,
    pub message_count: u32,
    pub write_buf: *mut Element,
    pub read_buf: *mut Element,
    pub state: PpcState,
    _pad: ffi::unknown<0x38>,
}

/// SmdSimpleBufPool
#[doc(alias = "SmdSimpleBufPool")]
#[repr(C)]
pub struct SimpleBufPool {
    pub self_: *mut SimpleBufPool,
    pub mutex: Mutex,
    pub pool_data: *mut ffi::c_void,
    pub pool_data_size: u32,
    pub lock_type: LockType,
    pub alloc_size: u32,
    pub real_alloc_size: u32,
    pub max_alloc_count: u32,
    pub alloc_pool_start: *mut ffi::c_void,
    pub alloc_pool_end: *mut ffi::c_void,
    pub elements_in: u32,
    pub free_error_count: u32,
    _pad: ffi::unknown<0x28>,
}

const _: () = {
    assert!(size_of::<Interface>() == 0x280);
    assert!(size_of::<CtrlTable>() == 0x600);
};

unsafe extern "C" {
    /// smdPpcInit
    ///
    /// Creates the queue in `buffer`.
    #[doc(alias = "smdPpcInit")]
    #[link_name = "smdPpcInit"]
    pub unsafe fn ppc_init(
        buffer: *mut ffi::c_void,
        buffer_size: u32,
        message_count: u32,
        name: *const ffi::c_char,
        lock_type: LockType,
    ) -> *mut Ppc;

    /// smdPpcGetCtrlTableVectors
    #[doc(alias = "smdPpcGetCtrlTableVectors")]
    #[link_name = "smdPpcGetCtrlTableVectors"]
    pub unsafe fn ppc_get_ctrl_table_vectors(
        smd: *mut Ppc,
        vectors: *mut PpcCtrlTableVectors,
    ) -> Error;

    /// smdPpcOpen
    #[doc(alias = "smdPpcOpen")]
    #[link_name = "smdPpcOpen"]
    pub unsafe fn ppc_open(smd: *mut Ppc) -> Error;

    /// smdPpcClose
    #[doc(alias = "smdPpcClose")]
    #[link_name = "smdPpcClose"]
    pub unsafe fn ppc_close(smd: *mut Ppc) -> Error;

    /// smdPpcGetInterfaceState
    #[doc(alias = "smdPpcGetInterfaceState")]
    #[link_name = "smdPpcGetInterfaceState"]
    pub unsafe fn ppc_get_interface_state(
        smd: *mut Ppc,
        ppc_state: *mut InterfaceState,
        iop_state: *mut InterfaceState,
    ) -> Error;

    /// smdPpcReceive
    #[doc(alias = "smdPpcReceive")]
    #[link_name = "smdPpcReceive"]
    pub unsafe fn ppc_receive(smd: *mut Ppc, data: *mut ReceiveData) -> Error;

    /// smdPpcSendMessage
    #[doc(alias = "smdPpcSendMessage")]
    #[link_name = "smdPpcSendMessage"]
    pub unsafe fn ppc_send_message(
        smd: *mut Ppc,
        message: *const ffi::c_void,
        message_size: u32,
    ) -> Error;

    /// smdPpcSendVectorSpec
    #[doc(alias = "smdPpcSendVectorSpec")]
    #[link_name = "smdPpcSendVectorSpec"]
    pub unsafe fn ppc_send_vector_spec(
        smd: *mut Ppc,
        command: u32,
        specs: *const VectorSpec,
        specs_count: i32,
    ) -> Error;

    /// smdPpcSendVector
    #[doc(alias = "smdPpcSendVector")]
    #[link_name = "smdPpcSendVector"]
    pub unsafe fn ppc_send_vector(smd: *mut Ppc, vector: *const Vector) -> Error;

    /// smdSimpleBufPoolCreate
    #[doc(alias = "smdSimpleBufPoolCreate")]
    #[link_name = "smdSimpleBufPoolCreate"]
    pub unsafe fn simple_buf_pool_create(
        pool_data: *mut ffi::c_void,
        pool_data_size: u32,
        alloc_size: u32,
        alloc_count: u32,
        lock_type: LockType,
    ) -> *mut SimpleBufPool;

    /// smdSimpleBufAlloc
    #[doc(alias = "smdSimpleBufAlloc")]
    #[link_name = "smdSimpleBufAlloc"]
    pub unsafe fn simple_buf_alloc(pool: *mut SimpleBufPool, alloc: *mut *mut ffi::c_void)
    -> Error;

    /// smdSimpleBufFree
    #[doc(alias = "smdSimpleBufFree")]
    #[link_name = "smdSimpleBufFree"]
    pub unsafe fn simple_buf_free(pool: *mut SimpleBufPool, alloc: *mut ffi::c_void) -> Error;

    /// smdSimpleBufGetStatistics
    #[doc(alias = "smdSimpleBufGetStatistics")]
    #[link_name = "smdSimpleBufGetStatistics"]
    pub unsafe fn simple_buf_get_statistics(
        pool: *mut SimpleBufPool,
        alloc_count: *mut u32,
        free_error_count: *mut u32,
    ) -> Error;
}
