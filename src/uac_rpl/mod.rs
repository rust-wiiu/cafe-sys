//! USB Audio Class (uac_rpl)
//!
//! Input and output streaming for USB audio devices such as headsets. Cannot be used together with `usb_mic`.

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
    QueueDepthOut = 9,
    XferPerEntryOut = 10,
    MaxQueueDepth = 11,
    MaxXferPerEntry = 12,
    Core = 13,
}

/// UAC_SAMPLE_RATE
#[doc(alias = "UAC_SAMPLE_RATE")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleRates(pub ffi::c_ulong);

impl SampleRates {
    pub const KHZ_32: Self = Self(1 << 5);
    pub const KHZ_48: Self = Self(1 << 7);
}

/// UAC_CAPS
#[doc(alias = "UAC_CAPS")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Caps(pub ffi::c_ulong);

impl Caps {
    pub const INPUT: Self = Self(1 << 0);
    pub const OUTPUT: Self = Self(1 << 1);
}

/// usb_mic_ringbuffer_t
///
/// Interleaved 16-bit PCM, at least 4-byte aligned.
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
    /// Samples available to read (input) or free to write (output).
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

/// usb_dev_attr_t
#[doc(alias = "usb_dev_attr_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceAttr {
    pub num_channels: ffi::c_ulong,
    pub sample_rates: SampleRates,
}

/// usb_event_data_ex_t
///
/// The event data passed on attach.
#[doc(alias = "usb_event_data_ex_t")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventDataEx {
    pub data: EventData,
    pub caps: Caps,
    pub input: DeviceAttr,
    pub output: DeviceAttr,
}

/// usb_event_handler_t
///
/// Called on a UAC thread for every device event. `data` points to an [EventDataEx] on attach. The UAC functions except [init] and [uninit] may be called from here.
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
    /// UACInit
    ///
    /// Must be called again after reacquiring the foreground.
    #[doc(alias = "UACInit")]
    #[link_name = "UACInit"]
    pub unsafe fn init(context: *mut ffi::c_void, handler: EventHandler) -> Error;

    /// UACUninit
    #[doc(alias = "UACUninit")]
    #[link_name = "UACUninit"]
    pub unsafe fn uninit() -> Error;

    /// UACOpenQuery
    #[doc(alias = "UACOpenQuery")]
    #[link_name = "UACOpenQuery"]
    #[deprecated(note = "Always returns zero resources")]
    pub unsafe fn open_query(handle: Handle, query: *mut MemResQuery) -> Error;

    /// UACOpen
    ///
    /// `ring_buffer` is the input buffer and must stay valid until [close] or [uninit].
    #[doc(alias = "UACOpen")]
    #[link_name = "UACOpen"]
    pub unsafe fn open(handle: Handle, mem_res: *mut MemRes, ring_buffer: *mut RingBuffer)
    -> Error;

    /// UACClose
    #[doc(alias = "UACClose")]
    #[link_name = "UACClose"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// UACINStart
    #[doc(alias = "UACINStart")]
    #[link_name = "UACINStart"]
    pub unsafe fn in_start(handle: Handle) -> Error;

    /// UACINStartExt
    ///
    /// Signals `wait_event` when new input is available, at most every `delay_us`.
    #[doc(alias = "UACINStartExt")]
    #[link_name = "UACINStartExt"]
    pub unsafe fn in_start_ext(
        handle: Handle,
        wait_event: *mut Event,
        delay_us: ffi::c_uint,
    ) -> Error;

    /// UACINStop
    #[doc(alias = "UACINStop")]
    #[link_name = "UACINStop"]
    pub unsafe fn in_stop(handle: Handle) -> Error;

    /// UACINGetStatus
    #[doc(alias = "UACINGetStatus")]
    #[link_name = "UACINGetStatus"]
    pub unsafe fn in_get_status(handle: Handle, status: *mut Status) -> Error;

    /// UACINSetDataConsumed
    #[doc(alias = "UACINSetDataConsumed")]
    #[link_name = "UACINSetDataConsumed"]
    pub unsafe fn in_set_data_consumed(handle: Handle, samples: ffi::c_uint) -> Error;

    /// UACOUTStart
    ///
    /// Streams `ring_buffer` until [out_stop]. Silence is sent if the buffer runs empty.
    #[doc(alias = "UACOUTStart")]
    #[link_name = "UACOUTStart"]
    pub unsafe fn out_start(
        handle: Handle,
        ring_buffer: *mut RingBuffer,
        sample_rate: SampleRates,
    ) -> Error;

    /// UACOUTStartExt
    #[doc(alias = "UACOUTStartExt")]
    #[link_name = "UACOUTStartExt"]
    pub unsafe fn out_start_ext(
        handle: Handle,
        wait_event: *mut Event,
        delay_us: ffi::c_uint,
        ring_buffer: *mut RingBuffer,
        sample_rate: SampleRates,
    ) -> Error;

    /// UACOUTStop
    #[doc(alias = "UACOUTStop")]
    #[link_name = "UACOUTStop"]
    pub unsafe fn out_stop(handle: Handle) -> Error;

    /// UACOUTGetStatus
    #[doc(alias = "UACOUTGetStatus")]
    #[link_name = "UACOUTGetStatus"]
    pub unsafe fn out_get_status(handle: Handle, status: *mut Status) -> Error;

    /// UACOUTSetDataConsumed
    ///
    /// Advances the output ring buffer unconditionally.
    #[doc(alias = "UACOUTSetDataConsumed")]
    #[link_name = "UACOUTSetDataConsumed"]
    pub unsafe fn out_set_data_consumed(handle: Handle, samples: ffi::c_uint) -> Error;

    /// UACSetState
    #[doc(alias = "UACSetState")]
    #[link_name = "UACSetState"]
    pub unsafe fn set_state(handle: Handle, state: State, value: ffi::c_uint) -> Error;

    /// UACGetState
    #[doc(alias = "UACGetState")]
    #[link_name = "UACGetState"]
    pub unsafe fn get_state(handle: Handle, state: State, value: *mut ffi::c_uint) -> Error;
}

imports_section!(
    "uac_rpl",
    [
        "UACClose",
        "UACGetState",
        "UACINGetStatus",
        "UACINSetDataConsumed",
        "UACINStart",
        "UACINStartExt",
        "UACINStop",
        "UACInit",
        "UACOUTGetStatus",
        "UACOUTSetDataConsumed",
        "UACOUTStart",
        "UACOUTStartExt",
        "UACOUTStop",
        "UACOpen",
        "UACOpenQuery",
        "UACSetState",
        "UACUninit"
    ],
    []
);
