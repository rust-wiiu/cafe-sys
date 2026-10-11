//! Mic

use crate::ffi;

/// MICHandle
#[doc(alias = "MICHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub ffi::c_int);

/// MICError
#[doc(alias = "MICError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub ffi::c_int);

impl Error {
    pub const NONE: Self = Self(0);
    pub const NOT_SUPPORTED: Self = Self(-1);
    pub const INVALID_ARG: Self = Self(-2);
    pub const INVALID_STATE: Self = Self(-3);
    pub const NO_MEMORY: Self = Self(-4);
    pub const ALREADY_OPEN: Self = Self(-5);
    pub const NOT_OPEN: Self = Self(-6);
    pub const NOT_INITIALIZED: Self = Self(-7);
    pub const NOT_CONNECTED: Self = Self(-8);
}

/// MICInstance
#[doc(alias = "MICInstance")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instance {
    #[default]
    I0 = 0,
    I1 = 1,
}

/// mic_state_t
#[doc(alias = "mic_state_t")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    SampleRate = 0,
    GainDb = 1,
    GainMin = 2,
    GainMax = 3,
    GainStep = 4,
    Mute = 5,
    EchoCancellation = 7,
    AutoSelection = 8,
    DigitalGainDb = 9,
}

/// MIC_STATUS_FLAG_*
#[doc(alias = "MIC_STATUS_FLAG")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusFlags(pub u32);

impl StatusFlags {
    pub const PCM16: Self = Self(1 << 0);
    pub const OPEN: Self = Self(1 << 1);
    pub const CONNECTED: Self = Self(1 << 2);
}

/// MICWorkMemory
#[doc(alias = "MICWorkMemory")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingBuffer {
    /// Number of samples in `base`.
    pub modulus: u32,
    pub base: *mut i16,
}

/// MICStatus
#[doc(alias = "MICStatus")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub flags: StatusFlags,
    /// Number of samples available to read.
    pub available: u32,
    /// Index of the first unread sample in [RingBuffer::base].
    pub read_idx: u32,
}

unsafe extern "C" {
    /// MICInit
    ///
    /// `res` is unused and may be null.
    #[doc(alias = "MICInit")]
    #[link_name = "MICInit"]
    pub unsafe fn init(
        instance: Instance,
        res: *mut ffi::c_void,
        ring_buffer: *mut RingBuffer,
        err: *mut Error,
    ) -> Handle;

    /// MICUninit
    #[doc(alias = "MICUninit")]
    #[link_name = "MICUninit"]
    pub unsafe fn uninit(handle: Handle) -> Error;

    /// MICOpen
    #[doc(alias = "MICOpen")]
    #[link_name = "MICOpen"]
    pub unsafe fn open(handle: Handle) -> Error;

    /// MICClose
    #[doc(alias = "MICClose")]
    #[link_name = "MICClose"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// MICGetStatus
    #[doc(alias = "MICGetStatus")]
    #[link_name = "MICGetStatus"]
    pub unsafe fn get_status(handle: Handle, status: *mut Status) -> Error;

    /// MICSetDataConsumed
    ///
    /// Marks `count` samples as read.
    #[doc(alias = "MICSetDataConsumed")]
    #[link_name = "MICSetDataConsumed"]
    pub unsafe fn set_data_consumed(handle: Handle, count: ffi::c_uint) -> Error;

    /// MICSetState
    #[doc(alias = "MICSetState")]
    #[link_name = "MICSetState"]
    pub unsafe fn set_state(handle: Handle, state: State, value: ffi::c_uint) -> Error;

    /// MICGetState
    #[doc(alias = "MICGetState")]
    #[link_name = "MICGetState"]
    pub unsafe fn get_state(handle: Handle, state: State, value: *mut ffi::c_uint) -> Error;
}

// #[cfg(target_arch = "powerpc")]
imports_section!(
    "mic",
    [
        "MICClose",
        "MICGetState",
        "MICGetStatus",
        "MICInit",
        "MICOpen",
        "MICSetDataConsumed",
        "MICSetState",
        "MICUninit"
    ],
    []
);
