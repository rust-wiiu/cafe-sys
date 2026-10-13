//! Camera

use crate::ffi;

pub const STREAM_WIDTH: u32 = 640;
pub const STREAM_PITCH: u32 = 768;
pub const STREAM_HEIGHT: u32 = 480;

pub const STREAM_BUFFER_COUNT: u32 = 20;

pub const Y_BUFFER_SIZE: u32 = STREAM_PITCH * STREAM_HEIGHT;
pub const UV_BUFFER_SIZE: u32 = STREAM_PITCH * STREAM_HEIGHT / 2;
pub const YUV_BUFFER_SIZE: u32 = Y_BUFFER_SIZE + UV_BUFFER_SIZE;
pub const YUV_BUFFER_ALIGNMENT: u32 = 256;

/// CAMHandle
#[doc(alias = "CAMHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub ffi::c_int);

/// CAMError
#[doc(alias = "CAMError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub ffi::c_int);

impl Error {
    pub const NONE: Self = Self(0);
    pub const INVALID_ARG: Self = Self(-1);
    pub const INVALID_HANDLE: Self = Self(-2);
    pub const INVALID_STATE: Self = Self(-3);
    pub const QUEUE_FULL: Self = Self(-4);
    pub const INSUFFICIENT_MEMORY: Self = Self(-5);
    pub const NOT_READY: Self = Self(-6);
    pub const FSA_FAILURE: Self = Self(-7);
    pub const UNINITIALIZED: Self = Self(-8);
    pub const DEVICE_NOT_READY: Self = Self(-9);
    pub const UNKNOWN: Self = Self(-10);
    pub const NOT_IMPLEMENTED: Self = Self(-11);
    pub const DEVICE_IN_USE: Self = Self(-12);
    pub const UVD_FAILURE: Self = Self(-13);
    pub const NOT_SUPPORTED: Self = Self(-14);
    pub const SEGMENT_VIOLATION: Self = Self(-15);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    /// Not implemented.
    DevicePowerMode = 0,
    AutoExposureMode = 1,
    AutoExposureTimeAbsolute = 2,
    Brightness = 3,
    Contrast = 4,
    Gain = 5,
    /// See [PowerLineFrequency].
    PowerLineFrequency = 6,
    Hue = 7,
    Saturation = 8,
    Sharpness = 9,
    Gamma = 10,
    /// Not implemented.
    DigitalMultiplierStep = 11,
    /// Not implemented.
    DigitalMultiplierLimit = 12,
    WhiteBalanceComponent = 13,
    WhiteBalanceComponentAuto = 14,
    /// 1: DRC attached, 0: DRC detached. Only implemented in [get_state].
    DrcConnectionState = 15,
    /// Only implemented in [set_state].
    GenerateKeyFrame = 16,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instance {
    #[default]
    I0 = 0,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    #[default]
    Nv12 = 0,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileMode {
    #[default]
    Linear = 0,
}

/// CamEventType
#[doc(alias = "CamEventType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventType(pub u32);

impl EventType {
    pub const DECODE_DONE: Self = Self(0);
    pub const DRC_DETACH: Self = Self(1);
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayMode {
    #[default]
    Downstream = 0,
    LocalLoopback = 1,
}

/// CamFps
#[doc(alias = "CamFps")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fps {
    #[default]
    Fps15 = 0,
    Fps30 = 1,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerLineFrequency {
    #[default]
    Disabled = 0,
    Hz50 = 1,
    Hz60 = 2,
}

/// CamStreamType
#[doc(alias = "CamStreamType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamType {
    #[default]
    Video = 0,
}

/// CAMMode
#[doc(alias = "CAMMode")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    pub display: DisplayMode,
    pub fps: Fps,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StateInfo {
    pub max: ffi::c_int,
    pub min: ffi::c_int,
    /// Step size.
    pub res: ffi::c_int,
}

/// CAMEventData
#[doc(alias = "CAMEventData")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventHandlerInput {
    pub kind: EventType,
    pub data0: u32,
    pub data1: u32,
    pub data2: u32,
}

/// CAMEventHandler
#[doc(alias = "CAMEventHandler")]
pub type EventHandler = Option<unsafe extern "C" fn(input: *mut EventHandlerInput)>;

/// CAMWorkMem
#[doc(alias = "CAMWorkMem")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkMem {
    pub size: ffi::c_int,
    pub mem: *mut ffi::c_void,
}

/// CAMStreamInfo
#[doc(alias = "CAMStreamInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StreamInfo {
    pub kind: StreamType,
    pub height: ffi::c_int,
    pub width: ffi::c_int,
}

/// CAMSetupInfo
#[doc(alias = "CAMSetupInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetupInfo {
    pub stream_info: StreamInfo,
    pub work_mem: WorkMem,
    pub event_handler: EventHandler,
    pub mode: Mode,
    /// [ThreadAttributes][crate::coreinit::thread::ThreadAttributes] affinity bits.
    pub thread_affinity: u32,
    pub reserved: ffi::unknown<0x10>,
}

/// CAMSurface
#[doc(alias = "CAMSurface")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Surface {
    pub image_size: ffi::c_int,
    pub image: *mut ffi::c_void,
    pub height: ffi::c_int,
    pub width: ffi::c_int,
    pub pitch: ffi::c_int,
    pub alignment: ffi::c_int,
    pub tile_mode: TileMode,
    pub pixel_format: PixelFormat,
}

unsafe extern "C" {
    /// CAMInit
    #[doc(alias = "CAMInit")]
    #[link_name = "CAMInit"]
    pub unsafe fn init(instance: Instance, setup: *const SetupInfo, err: *mut Error) -> Handle;

    /// CAMExit
    #[doc(alias = "CAMExit")]
    #[link_name = "CAMExit"]
    pub unsafe fn exit(handle: Handle);

    /// CAMOpen
    #[doc(alias = "CAMOpen")]
    #[link_name = "CAMOpen"]
    pub unsafe fn open(handle: Handle) -> Error;

    /// CAMClose
    #[doc(alias = "CAMClose")]
    #[link_name = "CAMClose"]
    pub unsafe fn close(handle: Handle) -> Error;

    /// CAMGetMemReq
    ///
    /// Returns the size of the work memory in bytes.
    #[doc(alias = "CAMGetMemReq")]
    #[link_name = "CAMGetMemReq"]
    pub unsafe fn memory_requirement(stream_info: *const StreamInfo) -> ffi::c_int;

    /// CAMSubmitTargetSurface
    #[doc(alias = "CAMSubmitTargetSurface")]
    #[link_name = "CAMSubmitTargetSurface"]
    pub unsafe fn submit_target_surface(handle: Handle, surface: *mut Surface) -> Error;

    /// CAMGetState
    #[doc(alias = "CAMGetState")]
    #[link_name = "CAMGetState"]
    pub unsafe fn get_state(handle: Handle, state: State, value: *mut ffi::c_int) -> Error;

    /// CAMGetStateInfo
    #[doc(alias = "CAMGetStateInfo")]
    #[link_name = "CAMGetStateInfo"]
    pub unsafe fn get_state_info(handle: Handle, state: State, info: *mut StateInfo) -> Error;

    /// CAMSetState
    #[doc(alias = "CAMSetState")]
    #[link_name = "CAMSetState"]
    pub unsafe fn set_state(handle: Handle, state: State, value: *const ffi::c_int) -> Error;

    /// CAMCheckMemSegmentation
    ///
    /// Checks that the range does not cross a 256 MB (physical) boundary.
    /// Returns [Error::SEGMENT_VIOLATION] if they do, [Error::INVALID_ARG] for a null pointer or zero size.
    #[doc(alias = "CAMCheckMemSegmentation")]
    #[link_name = "CAMCheckMemSegmentation"]
    pub unsafe fn check_memory_segmentation(mem: *const ffi::c_void, size: ffi::c_uint) -> Error;
}

imports_section!(
    "camera",
    [
        "CAMCheckMemSegmentation",
        "CAMClose",
        "CAMExit",
        "CAMGetMemReq",
        "CAMGetState",
        "CAMGetStateInfo",
        "CAMInit",
        "CAMOpen",
        "CAMSetState",
        "CAMSubmitTargetSurface"
    ],
    []
);
