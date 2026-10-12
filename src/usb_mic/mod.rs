//! USB Microphone (usb_mic)
//!
//! Input streaming from USB microphones.

use crate::coreinit::event::Event;
use crate::ffi;

pub use crate::uac_rpl::{
    Error, EventData, EventHandler, EventType, Handle, MemRes, MemResElement, MemResQuery,
    MemResQueryElement, RingBuffer, Status, StatusFlags,
};

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

unsafe extern "C" {
    /// USBMICInit
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
    /// `mem_res` is unused.
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
