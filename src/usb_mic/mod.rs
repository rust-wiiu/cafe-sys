//! USB Microphone (usb_mic)
//!
//! Input streaming from USB microphones. Cannot be used together with `uac_rpl`.

use crate::coreinit::event::Event;
use crate::ffi;

/// usb_mic_handle_t
#[doc(alias = "usb_mic_handle_t")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub ffi::c_long);

/// USB_MIC_ERROR
#[doc(alias = "USB_MIC_ERROR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub ffi::c_int);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_SUPPORTED: Self = Self(-1);
    pub const INVALID_ARG: Self = Self(-2);
    pub const INVALID_STATE: Self = Self(-3);
    pub const NO_MEMORY: Self = Self(-4);
    pub const ALREADY_OPEN: Self = Self(-5);
    pub const NOT_OPEN: Self = Self(-6);
    pub const DEVICE: Self = Self(-7);
}

/// usb_mic_state_t
#[doc(alias = "usb_mic_state_t")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    SampleRate = 0,
    GainDb = 1,
    GainMin = 2,
    GainMax = 3,
    GainStep = 4,
    Mute = 5,
    DigitalGainDb = 6,
    QueueDepth = 7,
    XferPerEntry = 8,
    MaxQueueDepth = 9,
    MaxXferPerEntry = 10,
    Core = 11,
}

/// usb_mic_ringbuffer_t
///
/// 16-bit PCM, at least 4-byte aligned.
#[doc(alias = "usb_mic_ringbuffer_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RingBuffer {
    /// Size in samples of all channels.
    pub modulus: ffi::c_ulong,
    pub base: *mut i16,
}

/// MIC_STATUS_FLAG
#[doc(alias = "MIC_STATUS_FLAG")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusFlags(pub ffi::c_ulong);

impl StatusFlags {
    pub const PCM16: Self = Self(1 << 0);
}

/// usb_mic_status_t
#[doc(alias = "usb_mic_status_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status {
    pub flags: StatusFlags,
    /// Samples available to read.
    pub available: ffi::c_ulong,
    pub read_idx: ffi::c_ulong,
}

/// usb_event_type_t
#[doc(alias = "usb_event_type_t")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventType(pub u32);

impl EventType {
    pub const ATTACH: Self = Self(0);
    pub const DETACH: Self = Self(1);
    pub const ERROR: Self = Self(2);
}

/// usb_event_data_t
#[doc(alias = "usb_event_data_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventData {
    pub host: ffi::c_ulong,
    pub port: ffi::c_ulong,
    pub vid: ffi::c_ulong,
    pub pid: ffi::c_ulong,
    pub handle: Handle,
}

/// usb_event_handler_t
///
/// Called on a driver thread for every device event. The functions except [init] and [uninit] may be called from here.
#[doc(alias = "usb_event_handler_t")]
pub type EventHandler = Option<
    unsafe extern "C" fn(
        context: *mut ffi::c_void,
        kind: EventType,
        data: *mut EventData,
        error: ffi::c_int,
    ),
>;

/// mem_res_query_element_t
#[doc(alias = "mem_res_query_element_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemResQueryElement {
    pub size: ffi::c_ulong,
    pub alignment: ffi::c_ulong,
    pub quantum: ffi::c_ulong,
    pub num_quanta: ffi::c_ulong,
    pub max_quanta: ffi::c_ulong,
    pub flags: ffi::c_ulong,
    pub links: [ffi::c_ulong; 3],
}

/// mem_res_query_t
#[doc(alias = "mem_res_query_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemResQuery {
    pub num_mem_res: ffi::c_ulong,
    pub mem_res: *mut MemResQueryElement,
}

/// mem_res_element_t
#[doc(alias = "mem_res_element_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemResElement {
    pub size: ffi::c_ulong,
    pub alignment: ffi::c_ulong,
    pub quantum: ffi::c_ulong,
    pub num_quanta: ffi::c_ulong,
    pub flags: ffi::c_ulong,
    pub pv: *mut ffi::c_void,
}

/// mem_res_t
///
/// No longer used, pass zero resources.
#[doc(alias = "mem_res_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemRes {
    pub num_mem_res: ffi::c_ulong,
    pub mem_res: *mut MemResElement,
}

unsafe extern "C" {
    /// USBMICInit
    ///
    /// Must be called again after reacquiring the foreground.
    #[doc(alias = "USBMICInit")]
    #[link_name = "USBMICInit"]
    pub unsafe fn init(context: *mut ffi::c_void, handler: EventHandler) -> Error;

    /// USBMICUninit
    #[doc(alias = "USBMICUninit")]
    #[link_name = "USBMICUninit"]
    pub unsafe fn uninit() -> Error;

    /// USBMICOpenQuery
    #[doc(alias = "USBMICOpenQuery")]
    #[link_name = "USBMICOpenQuery"]
    #[deprecated(note = "Always returns zero resources")]
    pub unsafe fn open_query(handle: Handle, query: *mut MemResQuery) -> Error;

    /// USBMICOpen
    ///
    /// `ring_buffer` must stay valid until [close] or [uninit].
    #[doc(alias = "USBMICOpen")]
    #[link_name = "USBMICOpen"]
    pub unsafe fn open(handle: Handle, mem_res: *mut MemRes, ring_buffer: *mut RingBuffer)
    -> Error;

    /// USBMICClose
    #[doc(alias = "USBMICClose")]
    #[link_name = "USBMICClose"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// USBMICStart
    #[doc(alias = "USBMICStart")]
    #[link_name = "USBMICStart"]
    pub unsafe fn start(handle: Handle) -> Error;

    /// USBMICStartExt
    ///
    /// Signals `wait_event` when new input is available, at most every `delay_us`.
    #[doc(alias = "USBMICStartExt")]
    #[link_name = "USBMICStartExt"]
    pub unsafe fn start_ext(handle: Handle, wait_event: *mut Event, delay_us: ffi::c_uint)
    -> Error;

    /// USBMICStop
    #[doc(alias = "USBMICStop")]
    #[link_name = "USBMICStop"]
    pub unsafe fn stop(handle: Handle) -> Error;

    /// USBMICGetStatus
    #[doc(alias = "USBMICGetStatus")]
    #[link_name = "USBMICGetStatus"]
    pub unsafe fn get_status(handle: Handle, status: *mut Status) -> Error;

    /// USBMICSetDataConsumed
    #[doc(alias = "USBMICSetDataConsumed")]
    #[link_name = "USBMICSetDataConsumed"]
    pub unsafe fn set_data_consumed(handle: Handle, samples: ffi::c_uint) -> Error;

    /// USBMICSetState
    #[doc(alias = "USBMICSetState")]
    #[link_name = "USBMICSetState"]
    pub unsafe fn set_state(handle: Handle, state: State, value: ffi::c_uint) -> Error;

    /// USBMICGetState
    #[doc(alias = "USBMICGetState")]
    #[link_name = "USBMICGetState"]
    pub unsafe fn get_state(handle: Handle, state: State, value: *mut ffi::c_uint) -> Error;
}

imports_section!(
    "usb_mic",
    [
        "USBMICClose",
        "USBMICGetState",
        "USBMICGetStatus",
        "USBMICInit",
        "USBMICOpen",
        "USBMICOpenQuery",
        "USBMICSetDataConsumed",
        "USBMICSetState",
        "USBMICStart",
        "USBMICStartExt",
        "USBMICStop",
        "USBMICUninit"
    ],
    []
);
