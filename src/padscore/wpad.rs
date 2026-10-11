//! WPAD
//!
//! Low level Wii Remote and Wii U Pro Controller driver.

use crate::{coreinit::time::Time, ffi};

pub const MAX_CONTROLLERS: usize = 4;
pub const DPD_MAX_OBJECTS: usize = 4;
pub const DPD_IMG_RESO_WX: u32 = 1024;
pub const DPD_IMG_RESO_WY: u32 = 768;
pub const MAX_CUSTOM_BUFS: usize = 20;
pub const MAX_BULK_BUFS: usize = 21;
pub const PRESS_UNITS: usize = 4;
pub const ADDRESS_LEN: usize = 6;
pub const MEM_GAMEDATA_LEN: u32 = 3888;
pub const ACC_RESO: u32 = 1024;
pub const FS_STICK_RESO: u32 = 256;
pub const FS_ACC_RESO: u32 = 1024;
pub const CL_STICK_RESO: u32 = 1024;
pub const CL_TRIGGER_RESO: u32 = 256;
pub const UC_STICK_RESO: u32 = 4096;
/// Not needed, [register_blc_workarea] ignores its work area.
pub const BLCINT_WORK_LEN: usize = 307200;

pub const BATTERY_LEVEL_CRITICAL: u8 = 0;
pub const BATTERY_LEVEL_LOW: u8 = 1;
pub const BATTERY_LEVEL_MEDIUM: u8 = 2;
pub const BATTERY_LEVEL_HIGH: u8 = 3;
pub const BATTERY_LEVEL_MAX: u8 = 4;

/// Balance Board calibration blocks, see [get_bl_calibration].
pub const BLCLB_BLK1_ADDR: u16 = 0x0024;
pub const BLCLB_BLK1_LEN: u16 = 16;
pub const BLCLB_BLK2_ADDR: u16 = 0x0034;
pub const BLCLB_BLK2_LEN: u16 = 8;
pub const BLCLB_BLK3_ADDR: u16 = 0x003c;
pub const BLCLB_BLK3_LEN: u16 = 4;
pub const BLCLB_BLK4_ADDR: u16 = 0x0050;
pub const BLCLB_BLK4_LEN: u16 = 16;
pub const BLCLB_BLK5_ADDR: u16 = 0x0060;
pub const BLCLB_BLK5_LEN: u16 = 16;

/// WPADChan
///
/// Controller channel. See [kpad::set_max_controllers](super::kpad::set_max_controllers) for more than 4 controllers.
#[doc(alias = "WPADChan")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel(pub i32);

impl Channel {
    pub const C0: Self = Self(0);
    pub const C1: Self = Self(1);
    pub const C2: Self = Self(2);
    pub const C3: Self = Self(3);
    pub const C4: Self = Self(4);
    pub const C5: Self = Self(5);
    pub const C6: Self = Self(6);
    pub const INVALID: Self = Self(-1);
}

/// WPADError
#[doc(alias = "WPADError")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const NONE: Self = Self(0);
    pub const NO_CONTROLLER: Self = Self(-1);
    pub const BUSY: Self = Self(-2);
    pub const TRANSFER: Self = Self(-3);
    pub const INVALID: Self = Self(-4);
    pub const NOPERM: Self = Self(-5);
    pub const BROKEN: Self = Self(-6);
    pub const CORRUPTED: Self = Self(-7);
}

/// WPADButton
///
/// Wii Remote buttons. `Z` and `C` are on the Nunchuk.
#[doc(alias = "WPADButton")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Button(pub u16);

impl Button {
    pub const LEFT: Self = Self(1);
    pub const RIGHT: Self = Self(2);
    pub const DOWN: Self = Self(4);
    pub const UP: Self = Self(8);
    pub const PLUS: Self = Self(16);
    #[doc(alias = "WPAD_BUTTON_2")]
    pub const TWO: Self = Self(256);
    #[doc(alias = "WPAD_BUTTON_1")]
    pub const ONE: Self = Self(512);
    pub const B: Self = Self(1024);
    pub const A: Self = Self(2048);
    pub const MINUS: Self = Self(4096);
    pub const Z: Self = Self(8192);
    pub const C: Self = Self(16384);
    pub const HOME: Self = Self(32768);
}

impl core::ops::BitOr for Button {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// WPADClassicButton
///
/// Classic Controller buttons.
#[doc(alias = "WPADClassicButton")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClButton(pub u16);

impl ClButton {
    pub const UP: Self = Self(1);
    pub const LEFT: Self = Self(2);
    pub const ZR: Self = Self(4);
    pub const X: Self = Self(8);
    pub const A: Self = Self(16);
    pub const Y: Self = Self(32);
    pub const B: Self = Self(64);
    pub const ZL: Self = Self(128);
    pub const R: Self = Self(512);
    pub const PLUS: Self = Self(1024);
    pub const HOME: Self = Self(2048);
    pub const MINUS: Self = Self(4096);
    pub const L: Self = Self(8192);
    pub const DOWN: Self = Self(16384);
    pub const RIGHT: Self = Self(32768);
}

impl core::ops::BitOr for ClButton {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// WPADProButton
///
/// Wii U Pro Controller buttons.
#[doc(alias = "WPADProButton")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UcButton(pub u32);

impl UcButton {
    pub const UP: Self = Self(1);
    pub const LEFT: Self = Self(2);
    pub const ZR: Self = Self(4);
    pub const X: Self = Self(8);
    pub const A: Self = Self(16);
    pub const Y: Self = Self(32);
    pub const B: Self = Self(64);
    pub const ZL: Self = Self(128);
    pub const R: Self = Self(512);
    pub const PLUS: Self = Self(1024);
    pub const HOME: Self = Self(2048);
    pub const MINUS: Self = Self(4096);
    pub const L: Self = Self(8192);
    pub const DOWN: Self = Self(16384);
    pub const RIGHT: Self = Self(32768);
    pub const STICK_R: Self = Self(65536);
    pub const STICK_L: Self = Self(131072);
}

impl core::ops::BitOr for UcButton {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// WPADExtensionType
///
/// Attached extension. `FREESTYLE` is the Nunchuk, `URCC` the Wii U Pro Controller.
#[doc(alias = "WPADExtensionType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceType(pub u32);

impl DeviceType {
    pub const CORE: Self = Self(0);
    pub const FREESTYLE: Self = Self(1);
    pub const CLASSIC: Self = Self(2);
    pub const BALANCE_CHECKER: Self = Self(3);
    pub const MPLS: Self = Self(5);
    pub const MPLS_FREESTYLE: Self = Self(6);
    pub const MPLS_CLASSIC: Self = Self(7);
    pub const TRAIN: Self = Self(16);
    pub const GUITAR: Self = Self(17);
    pub const DRUM: Self = Self(18);
    pub const TAIKO: Self = Self(19);
    pub const BULK_01: Self = Self(21);
    pub const BULK_02: Self = Self(22);
    pub const BULK_03: Self = Self(23);
    pub const BULK_04: Self = Self(24);
    pub const BULK_05: Self = Self(25);
    pub const BULK_06: Self = Self(26);
    pub const BULK_07: Self = Self(27);
    pub const BULK_08: Self = Self(28);
    pub const URCC: Self = Self(31);
    pub const MPLS_FUTURE: Self = Self(250);
    pub const FUTURE: Self = Self(251);
    pub const NOT_SUPPORTED: Self = Self(252);
    pub const NOT_FOUND: Self = Self(253);
    pub const UNKNOWN: Self = Self(255);
}

/// WPADDataFormat
///
/// Selects which status struct [read] fills.
#[doc(alias = "WPADDataFormat")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataFormat(pub u32);

impl DataFormat {
    pub const CORE: Self = Self(0);
    pub const CORE_ACC: Self = Self(1);
    pub const CORE_ACC_DPD: Self = Self(2);
    pub const FREESTYLE: Self = Self(3);
    pub const FREESTYLE_ACC: Self = Self(4);
    pub const FREESTYLE_ACC_DPD: Self = Self(5);
    pub const CLASSIC: Self = Self(6);
    pub const CLASSIC_ACC: Self = Self(7);
    pub const CLASSIC_ACC_DPD: Self = Self(8);
    pub const CORE_ACC_DPD_FULL: Self = Self(9);
    pub const TRAIN: Self = Self(10);
    pub const GUITAR: Self = Self(11);
    pub const BALANCE_CHECKER: Self = Self(12);
    pub const DRUM: Self = Self(15);
    pub const MPLS: Self = Self(16);
    pub const TAIKO: Self = Self(17);
    pub const BULK: Self = Self(19);
    pub const URCC: Self = Self(22);
}

/// WPADDpdFormat
///
/// Pointer (IR camera) mode.
#[doc(alias = "WPADDpdFormat")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpdFormat(pub u32);

impl DpdFormat {
    pub const OFF: Self = Self(0);
    pub const STD: Self = Self(1);
    pub const EXP: Self = Self(3);
    pub const FULL: Self = Self(5);
}

/// WPADSpeakerCmd
#[doc(alias = "WPADSpeakerCmd")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeakerCommand(pub u32);

impl SpeakerCommand {
    pub const OFF: Self = Self(0);
    pub const ON: Self = Self(1);
    pub const MUTE: Self = Self(2);
    pub const MUTE_OFF: Self = Self(3);
    pub const PLAY: Self = Self(4);
    pub const RESET: Self = Self(5);
}

/// WPADMplsMode
///
/// MotionPlus mode. `EXP` passes through a Nunchuk, `CL` a Classic Controller.
#[doc(alias = "WPADMplsMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MplsMode(pub u8);

impl MplsMode {
    pub const OFF: Self = Self(0);
    pub const STD: Self = Self(4);
    pub const EXP: Self = Self(5);
    pub const CL: Self = Self(7);
    pub const ZRST: Self = Self(128);
}

/// WPADMplsStatus
#[doc(alias = "WPADMplsStatus")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MplsStatus(pub u8);

impl MplsStatus {
    pub const ATTACH: Self = Self(1);
    pub const ROLSEL: Self = Self(2);
    pub const PITSEL: Self = Self(4);
    pub const YAWSEL: Self = Self(8);
    pub const EXTVLD: Self = Self(64);
    pub const PLSVLD: Self = Self(128);
}

/// Whether MotionPlus is built into the Wii Remote.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MplsType(pub i8);

impl MplsType {
    pub const READING: Self = Self(-1);
    pub const STANDARD: Self = Self(0);
    pub const INTEGRATED: Self = Self(1);
}

/// WPADLed
#[doc(alias = "WPADLed")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Led(pub u32);

impl Led {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(4);
    pub const FOUR: Self = Self(8);
}

impl core::ops::BitOr for Led {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// WPADSensorBarPos
#[doc(alias = "WPADSensorBarPos")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SensorBarPos(pub u8);

impl SensorBarPos {
    pub const BOTTOM: Self = Self(0);
    pub const TOP: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyncType(pub u8);

impl SyncType {
    pub const STD: Self = Self(0);
    pub const SMP: Self = Self(1);
}

/// WPADSyncDeviceEvent
#[doc(alias = "WPADSyncDeviceEvent")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyncEvent(pub i32);

impl SyncEvent {
    pub const START: Self = Self(0);
    pub const DONE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClearEvent(pub i32);

impl ClearEvent {
    pub const BUSY: Self = Self(-1);
    pub const START: Self = Self(0);
    pub const DONE: Self = Self(1);
}

/// WPADBalanceBoardCmd
#[doc(alias = "WPADBalanceBoardCmd")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlCommand(pub u8);

impl BlCommand {
    pub const UPDATE_TEMP: Self = Self(0);
    pub const OFF: Self = Self(85);
    pub const ON: Self = Self(170);
}

/// WPADClampType
#[doc(alias = "WPADClampType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StickClamp(pub u32);

impl StickClamp {
    pub const OCTA_WITH_PLAY: Self = Self(0);
    pub const OCTA_WITHOUT_PLAY: Self = Self(1);
    pub const CIRCLE_WITH_PLAY: Self = Self(2);
    pub const CIRCLE_WITHOUT_PLAY: Self = Self(3);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TriggerClamp(pub u32);

impl TriggerClamp {
    pub const FIXED_BASE: Self = Self(0);
    pub const INDIVIDUAL_BASE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccClamp(pub u32);

impl AccClamp {
    pub const CUBE: Self = Self(0);
    pub const SPHERE: Self = Self(1);
}

/// WPAD_MOTOR_*
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotorCommand {
    #[default]
    Stop = 0,
    Rumble = 1,
}

/// WPADPeripheralSpace
#[doc(alias = "WPADPeripheralSpace")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeripheralSpace {
    Speaker = 0xA2,
    Extension = 0xA4,
    MotionPlus = 0xA6,
    Dpd = 0xB0,
}

/// WPADAddress
#[doc(alias = "WPADAddress")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub [u8; ADDRESS_LEN]);

/// IR point seen by the pointer camera.
#[doc(alias("DPDObject", "WPADIRDot"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpdObject {
    pub x: i16,
    pub y: i16,
    pub size: u16,
    pub trace_id: u8,
}

#[doc(alias("DPDObjEx", "WPADIRDotEx"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpdObjectEx {
    pub range_x1: i16,
    pub range_y1: i16,
    pub range_x2: i16,
    pub range_y2: i16,
    pub pixel: u16,
    pub radius: i8,
}

#[doc(alias("WPADAcc", "WPADVec3D"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Acc {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// WPADStatus
#[doc(alias = "WPADStatus")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
}

/// WPADStatusEx
#[doc(alias = "WPADStatusEx")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusEx {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub exp: [DpdObjectEx; DPD_MAX_OBJECTS],
}

/// Wii Remote with Nunchuk.
#[doc(alias("WPADFSStatus", "WPADStatusNunchuk"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FsStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub fs_acc_x: i16,
    pub fs_acc_y: i16,
    pub fs_acc_z: i16,
    pub fs_stick_x: i8,
    pub fs_stick_y: i8,
}

/// Wii Remote with Classic Controller.
#[doc(alias("WPADCLStatus", "WPADStatusClassic"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub cl_button: ClButton,
    pub cl_l_stick_x: i16,
    pub cl_l_stick_y: i16,
    pub cl_r_stick_x: i16,
    pub cl_r_stick_y: i16,
    pub cl_trigger_l: u8,
    pub cl_trigger_r: u8,
}

/// Wii U Pro Controller.
#[doc(alias("WPADUCStatus", "WPADStatusProController"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UcStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub uc_button: UcButton,
    pub uc_l_stick_x: i16,
    pub uc_l_stick_y: i16,
    pub uc_r_stick_x: i16,
    pub uc_r_stick_y: i16,
    pub charge: ffi::c_bool,
    pub cable: ffi::c_bool,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CmStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub cm_button: u16,
    pub custom: [u8; MAX_CUSTOM_BUFS],
}

/// Balance Board.
#[doc(alias("WPADBLStatus", "WPADStatusBalanceBoard"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub press: [u16; PRESS_UNITS],
    pub temp: i8,
    pub battery: u8,
}

#[doc(alias("WPADTRStatus", "WPADStatusTrain"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub tr_button: u16,
    pub brake: u8,
    pub mascon: u8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BkStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub bulk: [u8; MAX_BULK_BUFS],
    pub padding: u8,
}

/// Nunchuk data of [MpStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MpFsExt {
    pub fs_acc_x: i16,
    pub fs_acc_y: i16,
    pub fs_acc_z: i16,
    pub fs_stick_x: i8,
    pub fs_stick_y: i8,
}

/// Classic Controller data of [MpStatus].
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MpClExt {
    pub cl_button: ClButton,
    pub cl_l_stick_x: i16,
    pub cl_l_stick_y: i16,
    pub cl_r_stick_x: i16,
    pub cl_r_stick_y: i16,
    pub cl_trigger_l: u8,
    pub cl_trigger_r: u8,
}

/// Extension data of [MpStatus], depending on [MpStatus::dev].
#[repr(C)]
#[derive(Clone, Copy)]
pub union MpExt {
    pub fs: MpFsExt,
    pub cl: MpClExt,
}

/// Wii Remote with MotionPlus.
#[doc(alias("WPADMPStatus", "WPADStatusMotionPlus"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MpStatus {
    pub button: Button,
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub obj: [DpdObject; DPD_MAX_OBJECTS],
    /// [DeviceType].
    pub dev: u8,
    /// [Error].
    pub err: i8,
    pub ext: MpExt,
    pub stat: MplsStatus,
    pub reserved: u8,
    pub pitch: i16,
    pub yaw: i16,
    pub roll: i16,
}

#[doc(alias("WPADMpls", "WPADiMplsCalibration"))]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mpls {
    pub pitch_zero: f32,
    pub pitch_scale: f32,
    pub yaw_zero: f32,
    pub yaw_scale: f32,
    pub roll_zero: f32,
    pub roll_scale: f32,
    pub degrees: i32,
}

/// WPADInfo
#[doc(alias = "WPADInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Info {
    pub dpd: ffi::c_bool,
    pub speaker: ffi::c_bool,
    pub attach: ffi::c_bool,
    pub low_battery: ffi::c_bool,
    pub near_empty: ffi::c_bool,
    pub battery: u8,
    pub led: u8,
    pub protocol: u8,
    pub firmware: u8,
}

/// WENCParams
///
/// Speaker encoder state.
#[doc(alias = "WENCParams")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncParams(pub ffi::unknown<32>);

const _: () = {
    assert!(size_of::<Status>() == 0x2A);
    assert!(size_of::<StatusEx>() == 0x5A);
    assert!(size_of::<FsStatus>() == 0x32);
    assert!(size_of::<ClStatus>() == 0x36);
    assert!(size_of::<UcStatus>() == 0x40);
    assert!(size_of::<CmStatus>() == 0x40);
    assert!(size_of::<MpStatus>() == 0x3E);
    assert!(size_of::<BlStatus>() == 0x34);
    assert!(size_of::<TrStatus>() == 0x2E);
    assert!(size_of::<BkStatus>() == 0x40);
    assert!(size_of::<Info>() == 0x18);
};

/// WPADCallback
#[doc(alias = "WPADCallback")]
pub type Callback = Option<unsafe extern "C" fn(chan: Channel, result: Error)>;

/// WPADSamplingCallback
#[doc(alias = "WPADSamplingCallback")]
pub type SamplingCallback = Option<unsafe extern "C" fn(chan: Channel)>;

/// WPADExtensionCallback
#[doc(alias = "WPADExtensionCallback")]
pub type ExtensionCallback = Option<unsafe extern "C" fn(chan: Channel, device_type: DeviceType)>;

/// WPADConnectCallback
///
/// `reason` is [Error::NONE] on connect and [Error::NO_CONTROLLER] on disconnect.
#[doc(alias = "WPADConnectCallback")]
pub type ConnectCallback = Option<unsafe extern "C" fn(chan: Channel, reason: Error)>;

/// WPADSyncDeviceCallback
#[doc(alias = "WPADSyncDeviceCallback")]
pub type SyncDeviceCallback = Option<unsafe extern "C" fn(event: SyncEvent, num: i32)>;

/// WPADClearDeviceCallback
#[doc(alias = "WPADClearDeviceCallback")]
pub type ClearDeviceCallback = Option<unsafe extern "C" fn(event: ClearEvent)>;

pub type AllocFn = Option<unsafe extern "C" fn(size: u32) -> *mut ffi::c_void>;

pub type FreeFn = Option<unsafe extern "C" fn(ptr: *mut ffi::c_void) -> u8>;

unsafe extern "C" {
    /// WPADInit
    #[doc(alias = "WPADInit")]
    #[link_name = "WPADInit"]
    pub unsafe fn init();

    /// WPADShutdown
    #[doc(alias = "WPADShutdown")]
    #[link_name = "WPADShutdown"]
    pub unsafe fn deinit();

    /// WPADRegisterAllocator
    #[doc(alias = "WPADRegisterAllocator")]
    #[link_name = "WPADRegisterAllocator"]
    pub unsafe fn register_allocator(alloc: AllocFn, free: FreeFn);

    /// WPADGetWorkMemorySize
    #[doc(alias = "WPADGetWorkMemorySize")]
    #[link_name = "WPADGetWorkMemorySize"]
    pub unsafe fn get_work_memory_size() -> u32;

    /// WPADGetStatus
    #[doc(alias = "WPADGetStatus")]
    #[link_name = "WPADGetStatus"]
    pub unsafe fn get_status() -> i32;

    /// WPADProbe
    ///
    /// Returns [Error::NO_CONTROLLER] if nothing is connected.
    #[doc(alias = "WPADProbe")]
    #[link_name = "WPADProbe"]
    pub unsafe fn probe(chan: Channel, device_type: *mut DeviceType) -> Error;

    /// WPADDisconnect
    #[doc(alias = "WPADDisconnect")]
    #[link_name = "WPADDisconnect"]
    pub unsafe fn disconnect(chan: Channel);

    /// WPADGetInfo
    #[doc(alias = "WPADGetInfo")]
    #[link_name = "WPADGetInfo"]
    pub unsafe fn get_info(chan: Channel, info: *mut Info) -> Error;

    /// WPADGetInfoAsync
    #[doc(alias = "WPADGetInfoAsync")]
    #[link_name = "WPADGetInfoAsync"]
    pub unsafe fn get_info_async(chan: Channel, info: *mut Info, callback: Callback) -> Error;

    /// WPADGetAddress
    #[doc(alias = "WPADGetAddress")]
    #[link_name = "WPADGetAddress"]
    pub unsafe fn get_address(chan: Channel, addr: *mut Address);

    /// WPADGetRegisteredDevNum
    #[doc(alias = "WPADGetRegisteredDevNum")]
    #[link_name = "WPADGetRegisteredDevNum"]
    pub unsafe fn get_registered_dev_num() -> u8;

    /// WPADGetRadioSensitivity
    #[doc(alias = "WPADGetRadioSensitivity")]
    #[link_name = "WPADGetRadioSensitivity"]
    pub unsafe fn get_radio_sensitivity(chan: Channel) -> u8;

    /// WPADGetBatteryLevel
    ///
    /// See the `BATTERY_LEVEL_*` constants.
    #[doc(alias = "WPADGetBatteryLevel")]
    #[link_name = "WPADGetBatteryLevel"]
    pub unsafe fn get_battery_level(chan: Channel) -> u8;

    /// WPADGetDataFormat
    #[doc(alias = "WPADGetDataFormat")]
    #[link_name = "WPADGetDataFormat"]
    pub unsafe fn get_data_format(chan: Channel) -> DataFormat;

    /// WPADSetDataFormat
    #[doc(alias = "WPADSetDataFormat")]
    #[link_name = "WPADSetDataFormat"]
    pub unsafe fn set_data_format(chan: Channel, format: DataFormat) -> Error;

    /// WPADRead
    ///
    /// `status` must point to the status struct of the current [DataFormat], e.g. [Status], [FsStatus] or [UcStatus].
    #[doc(alias = "WPADRead")]
    #[link_name = "WPADRead"]
    pub unsafe fn read(chan: Channel, status: *mut ffi::c_void);

    /// WPADSetAutoSamplingBuf
    #[doc(alias = "WPADSetAutoSamplingBuf")]
    #[link_name = "WPADSetAutoSamplingBuf"]
    pub unsafe fn set_auto_sampling_buf(chan: Channel, buf: *mut ffi::c_void, len: u32);

    /// WPADGetLatestIndexInBuf
    #[doc(alias = "WPADGetLatestIndexInBuf")]
    #[link_name = "WPADGetLatestIndexInBuf"]
    pub unsafe fn get_latest_index_in_buf(chan: Channel) -> u32;

    /// WPADGetAccGravityUnit
    #[doc(alias = "WPADGetAccGravityUnit")]
    #[link_name = "WPADGetAccGravityUnit"]
    pub unsafe fn get_acc_gravity_unit(chan: Channel, device_type: DeviceType, acc: *mut Acc);

    /// WPADGetCLTriggerThreshold
    #[doc(alias = "WPADGetCLTriggerThreshold")]
    #[link_name = "WPADGetCLTriggerThreshold"]
    pub unsafe fn get_cl_trigger_threshold(chan: Channel, left: *mut u8, right: *mut u8);

    /// WPADClampStick
    #[doc(alias = "WPADClampStick")]
    #[link_name = "WPADClampStick"]
    pub unsafe fn clamp_stick(chan: Channel, status: *mut ffi::c_void, clamp: StickClamp);

    /// WPADClampTrigger
    #[doc(alias = "WPADClampTrigger")]
    #[link_name = "WPADClampTrigger"]
    pub unsafe fn clamp_trigger(chan: Channel, status: *mut ffi::c_void, clamp: TriggerClamp);

    /// WPADClampAcc
    #[doc(alias = "WPADClampAcc")]
    #[link_name = "WPADClampAcc"]
    pub unsafe fn clamp_acc(chan: Channel, status: *mut ffi::c_void, clamp: AccClamp);

    /// WPADSetSamplingCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "WPADSetSamplingCallback")]
    #[link_name = "WPADSetSamplingCallback"]
    pub unsafe fn set_sampling_callback(
        chan: Channel,
        callback: SamplingCallback,
    ) -> SamplingCallback;

    /// WPADSetExtensionCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "WPADSetExtensionCallback")]
    #[link_name = "WPADSetExtensionCallback"]
    pub unsafe fn set_extension_callback(
        chan: Channel,
        callback: ExtensionCallback,
    ) -> ExtensionCallback;

    /// WPADSetConnectCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "WPADSetConnectCallback")]
    #[link_name = "WPADSetConnectCallback"]
    pub unsafe fn set_connect_callback(chan: Channel, callback: ConnectCallback)
    -> ConnectCallback;

    /// WPADAttachDummyExtension
    #[doc(alias = "WPADAttachDummyExtension")]
    #[link_name = "WPADAttachDummyExtension"]
    pub unsafe fn attach_dummy_extension(chan: Channel, device_type: DeviceType) -> ffi::c_bool;

    /// WPADDetachDummyExtension
    #[doc(alias = "WPADDetachDummyExtension")]
    #[link_name = "WPADDetachDummyExtension"]
    pub unsafe fn detach_dummy_extension(chan: Channel) -> ffi::c_bool;

    /// WPADIsDpdEnabled
    #[doc(alias = "WPADIsDpdEnabled")]
    #[link_name = "WPADIsDpdEnabled"]
    pub unsafe fn is_dpd_enabled(chan: Channel) -> ffi::c_bool;

    /// WPADGetDpdFormat
    #[doc(alias = "WPADGetDpdFormat")]
    #[link_name = "WPADGetDpdFormat"]
    pub unsafe fn get_dpd_format(chan: Channel) -> u8;

    /// WPADControlDpd
    #[doc(alias = "WPADControlDpd")]
    #[link_name = "WPADControlDpd"]
    pub unsafe fn control_dpd(chan: Channel, format: DpdFormat, callback: Callback) -> Error;

    /// WPADGetDpdSensitivity
    #[doc(alias = "WPADGetDpdSensitivity")]
    #[link_name = "WPADGetDpdSensitivity"]
    pub unsafe fn get_dpd_sensitivity() -> u8;

    /// WPADSetDpdSensitivity
    #[doc(alias = "WPADSetDpdSensitivity")]
    #[link_name = "WPADSetDpdSensitivity"]
    pub unsafe fn set_dpd_sensitivity(sensitivity: u8);

    /// WPADGetSensorBarPosition
    #[doc(alias = "WPADGetSensorBarPosition")]
    #[link_name = "WPADGetSensorBarPosition"]
    pub unsafe fn get_sensor_bar_position() -> SensorBarPos;

    /// WPADSetSensorBarPosition
    #[doc(alias = "WPADSetSensorBarPosition")]
    #[link_name = "WPADSetSensorBarPosition"]
    pub unsafe fn set_sensor_bar_position(pos: SensorBarPos);

    /// WPADEnableSensorBar
    #[doc(alias = "WPADEnableSensorBar")]
    #[link_name = "WPADEnableSensorBar"]
    pub unsafe fn enable_sensor_bar(on: u8);

    /// WPADSetSensorBar
    #[doc(alias = "WPADSetSensorBar")]
    #[link_name = "WPADSetSensorBar"]
    pub unsafe fn set_sensor_bar(on: ffi::c_bool);

    /// WPADSetSensorBarPower
    #[doc(alias = "WPADSetSensorBarPower")]
    #[link_name = "WPADSetSensorBarPower"]
    pub unsafe fn set_sensor_bar_power(on: ffi::c_bool);

    /// WPADGetCalibratedDPDObject
    #[doc(alias = "WPADGetCalibratedDPDObject")]
    #[link_name = "WPADGetCalibratedDPDObject"]
    pub unsafe fn get_calibrated_dpd_object(dst: *mut DpdObject, src: *const DpdObject);

    /// WPADGetDpdCornerPoints
    #[doc(alias = "WPADGetDpdCornerPoints")]
    #[link_name = "WPADGetDpdCornerPoints"]
    pub unsafe fn get_dpd_corner_points(chan: Channel, dst: *mut ffi::c_void);

    /// WPADRestoreDpdData
    #[doc(alias = "WPADRestoreDpdData")]
    #[link_name = "WPADRestoreDpdData"]
    pub unsafe fn restore_dpd_data(unk1: u32, unk2: u32, dpd_enabled: ffi::c_bool, chan: Channel);

    /// WPADControlMotor
    #[doc(alias = "WPADControlMotor")]
    #[link_name = "WPADControlMotor"]
    pub unsafe fn control_motor(chan: Channel, command: MotorCommand);

    /// WPADIsMotorEnabled
    #[doc(alias = "WPADIsMotorEnabled")]
    #[link_name = "WPADIsMotorEnabled"]
    pub unsafe fn is_motor_enabled() -> ffi::c_bool;

    /// WPADEnableMotor
    #[doc(alias = "WPADEnableMotor")]
    #[link_name = "WPADEnableMotor"]
    pub unsafe fn enable_motor(enabled: ffi::c_bool);

    /// WPADControlLed
    #[doc(alias = "WPADControlLed")]
    #[link_name = "WPADControlLed"]
    pub unsafe fn control_led(chan: Channel, led: Led, callback: Callback) -> Error;

    /// WPADIsSpeakerEnabled
    #[doc(alias = "WPADIsSpeakerEnabled")]
    #[link_name = "WPADIsSpeakerEnabled"]
    pub unsafe fn is_speaker_enabled(chan: Channel) -> ffi::c_bool;

    /// WPADControlSpeaker
    #[doc(alias = "WPADControlSpeaker")]
    #[link_name = "WPADControlSpeaker"]
    pub unsafe fn control_speaker(
        chan: Channel,
        command: SpeakerCommand,
        callback: Callback,
    ) -> Error;

    /// WPADGetSpeakerVolume
    #[doc(alias = "WPADGetSpeakerVolume")]
    #[link_name = "WPADGetSpeakerVolume"]
    pub unsafe fn get_speaker_volume() -> u8;

    /// WPADSetSpeakerVolume
    #[doc(alias = "WPADSetSpeakerVolume")]
    #[link_name = "WPADSetSpeakerVolume"]
    pub unsafe fn set_speaker_volume(volume: u8);

    /// WPADSendStreamData
    #[doc(alias = "WPADSendStreamData")]
    #[link_name = "WPADSendStreamData"]
    pub unsafe fn send_stream_data(chan: Channel, buf: *mut ffi::c_void, len: u16) -> Error;

    /// WPADCanSendStreamData
    #[doc(alias = "WPADCanSendStreamData")]
    #[link_name = "WPADCanSendStreamData"]
    pub unsafe fn can_send_stream_data(chan: Channel) -> ffi::c_bool;

    /// WENCGetEncodeData
    ///
    /// Encodes PCM samples for the speaker.
    #[doc(alias = "WENCGetEncodeData")]
    #[link_name = "WENCGetEncodeData"]
    pub unsafe fn wenc_get_encode_data(
        params: *mut EncParams,
        continuing: ffi::c_bool,
        samples: *const i16,
        sample_count: u32,
        encoded: *mut u8,
    ) -> u32;

    /// WPADControlExtGimmick
    #[doc(alias = "WPADControlExtGimmick")]
    #[link_name = "WPADControlExtGimmick"]
    pub unsafe fn control_ext_gimmick(chan: Channel, command: u32, callback: Callback) -> Error;

    /// WPADSetAutoSleepTime
    #[doc(alias = "WPADSetAutoSleepTime")]
    #[link_name = "WPADSetAutoSleepTime"]
    pub unsafe fn set_auto_sleep_time(minutes: u8);

    /// WPADGetAutoSleepTimeCount
    #[doc(alias = "WPADGetAutoSleepTimeCount")]
    #[link_name = "WPADGetAutoSleepTimeCount"]
    pub unsafe fn get_auto_sleep_time_count(chan: Channel) -> u32;

    /// WPADResetAutoSleepTimeCount
    #[doc(alias = "WPADResetAutoSleepTimeCount")]
    #[link_name = "WPADResetAutoSleepTimeCount"]
    pub unsafe fn reset_auto_sleep_time_count(chan: Channel);

    /// WPADGetPowerSaveMode
    #[doc(alias = "WPADGetPowerSaveMode")]
    #[link_name = "WPADGetPowerSaveMode"]
    pub unsafe fn get_power_save_mode(chan: Channel) -> ffi::c_bool;

    /// WPADSetPowerSaveMode
    #[doc(alias = "WPADSetPowerSaveMode")]
    #[link_name = "WPADSetPowerSaveMode"]
    pub unsafe fn set_power_save_mode(chan: Channel, power_save: ffi::c_bool);

    /// WPADSetInactivePeriod
    #[doc(alias = "WPADSetInactivePeriod")]
    #[link_name = "WPADSetInactivePeriod"]
    pub unsafe fn set_inactive_period(period: u32);

    /// WPADDisableBluetooth
    #[doc(alias = "WPADDisableBluetooth")]
    #[link_name = "WPADDisableBluetooth"]
    pub unsafe fn disable_bluetooth();

    /// WPADSetGameTitleUtf16
    #[doc(alias = "WPADSetGameTitleUtf16")]
    #[link_name = "WPADSetGameTitleUtf16"]
    pub unsafe fn set_game_title_utf16(title: *const u16);

    /// WPADGetGameTitleUtf16
    #[doc(alias = "WPADGetGameTitleUtf16")]
    #[link_name = "WPADGetGameTitleUtf16"]
    pub unsafe fn get_game_title_utf16(chan: Channel, title: *mut *const u16) -> Error;

    /// WPADGetGameDataTimeStamp
    #[doc(alias = "WPADGetGameDataTimeStamp")]
    #[link_name = "WPADGetGameDataTimeStamp"]
    pub unsafe fn get_game_data_time_stamp(chan: Channel, time: *mut Time) -> Error;

    /// WPADReadMemoryAsync
    #[doc(alias = "WPADReadMemoryAsync")]
    #[link_name = "WPADReadMemoryAsync"]
    pub unsafe fn read_memory_async(
        chan: Channel,
        dst: *mut ffi::c_void,
        size: u16,
        addr: u32,
        callback: Callback,
    ) -> Error;

    /// WPADWriteMemoryAsync
    #[doc(alias = "WPADWriteMemoryAsync")]
    #[link_name = "WPADWriteMemoryAsync"]
    pub unsafe fn write_memory_async(
        chan: Channel,
        src: *mut ffi::c_void,
        size: u32,
        addr: u32,
        callback: Callback,
    ) -> Error;

    /// WPADReadExtReg
    #[doc(alias = "WPADReadExtReg")]
    #[link_name = "WPADReadExtReg"]
    pub unsafe fn read_ext_reg(
        chan: Channel,
        dst: *mut ffi::c_void,
        size: u16,
        space: PeripheralSpace,
        addr: u32,
        callback: Callback,
    ) -> Error;

    /// WPADWriteExtReg
    #[doc(alias = "WPADWriteExtReg")]
    #[link_name = "WPADWriteExtReg"]
    pub unsafe fn write_ext_reg(
        chan: Channel,
        src: *const ffi::c_void,
        size: u32,
        space: PeripheralSpace,
        addr: u32,
        callback: Callback,
    ) -> Error;

    /// WPADSetAcceptConnection
    #[doc(alias = "WPADSetAcceptConnection")]
    #[link_name = "WPADSetAcceptConnection"]
    pub unsafe fn set_accept_connection(accept: ffi::c_bool) -> ffi::c_bool;

    /// WPADGetAcceptConnection
    #[doc(alias = "WPADGetAcceptConnection")]
    #[link_name = "WPADGetAcceptConnection"]
    pub unsafe fn get_accept_connection() -> ffi::c_bool;

    /// WPADStartSyncDevice
    #[doc(alias = "WPADStartSyncDevice")]
    #[link_name = "WPADStartSyncDevice"]
    pub unsafe fn start_sync_device() -> ffi::c_bool;

    /// WPADStartSyncDeviceEx
    #[doc(alias = "WPADStartSyncDeviceEx")]
    #[link_name = "WPADStartSyncDeviceEx"]
    pub unsafe fn start_sync_device_ex(addr: *mut Address, name: *const ffi::c_char)
    -> ffi::c_bool;

    /// WPADStartFastSyncDevice
    #[doc(alias = "WPADStartFastSyncDevice")]
    #[link_name = "WPADStartFastSyncDevice"]
    pub unsafe fn start_fast_sync_device() -> ffi::c_bool;

    /// WPADCancelSyncDevice
    #[doc(alias = "WPADCancelSyncDevice")]
    #[link_name = "WPADCancelSyncDevice"]
    pub unsafe fn cancel_sync_device() -> ffi::c_bool;

    /// WPADIsBusyForSync
    #[doc(alias = "WPADIsBusyForSync")]
    #[link_name = "WPADIsBusyForSync"]
    pub unsafe fn is_busy_for_sync() -> ffi::c_bool;

    /// WPADGetSyncType
    #[doc(alias = "WPADGetSyncType")]
    #[link_name = "WPADGetSyncType"]
    pub unsafe fn get_sync_type(chan: Channel, sync_type: *mut u8) -> Error;

    /// WPADPurgeBtDb
    #[doc(alias = "WPADPurgeBtDb")]
    #[link_name = "WPADPurgeBtDb"]
    pub unsafe fn purge_bt_db() -> ffi::c_bool;

    /// WPADStartClearDevice
    #[doc(alias = "WPADStartClearDevice")]
    #[link_name = "WPADStartClearDevice"]
    pub unsafe fn start_clear_device();

    /// WPADDeleteControllerOrder
    #[doc(alias = "WPADDeleteControllerOrder")]
    #[link_name = "WPADDeleteControllerOrder"]
    pub unsafe fn delete_controller_order() -> ffi::c_bool;

    /// WPADSaveConfig
    #[doc(alias = "WPADSaveConfig")]
    #[link_name = "WPADSaveConfig"]
    pub unsafe fn save_config() -> ffi::c_bool;

    /// WPADRetrieveChannel
    #[doc(alias = "WPADRetrieveChannel")]
    #[link_name = "WPADRetrieveChannel"]
    pub unsafe fn retrieve_channel(unk: u32) -> Channel;

    /// WPADSetSyncDeviceCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "WPADSetSyncDeviceCallback")]
    #[link_name = "WPADSetSyncDeviceCallback"]
    pub unsafe fn set_sync_device_callback(callback: SyncDeviceCallback) -> SyncDeviceCallback;

    /// WPADSetClearDeviceCallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "WPADSetClearDeviceCallback")]
    #[link_name = "WPADSetClearDeviceCallback"]
    pub unsafe fn set_clear_device_callback(callback: ClearDeviceCallback) -> ClearDeviceCallback;

    /// WPADSetDisableChannelImm
    #[doc(alias = "WPADSetDisableChannelImm")]
    #[link_name = "WPADSetDisableChannelImm"]
    pub unsafe fn set_disable_channel_imm(afh_channel: u8);

    /// WPADIsMplsAttached
    #[doc(alias = "WPADIsMplsAttached")]
    #[link_name = "WPADIsMplsAttached"]
    pub unsafe fn is_mpls_attached(
        chan: Channel,
        attached: *mut ffi::c_bool,
        callback: Callback,
    ) -> i8;

    /// WPADIsMplsIntegrated
    #[doc(alias = "WPADIsMplsIntegrated")]
    #[link_name = "WPADIsMplsIntegrated"]
    pub unsafe fn is_mpls_integrated(chan: Channel) -> MplsType;

    /// WPADGetCalibrationStatus
    #[doc(alias = "WPADGetCalibrationStatus")]
    #[link_name = "WPADGetCalibrationStatus"]
    pub unsafe fn get_calibration_status(chan: Channel) -> ffi::c_bool;

    /// WPADRecalibrate
    #[doc(alias = "WPADRecalibrate")]
    #[link_name = "WPADRecalibrate"]
    pub unsafe fn recalibrate(chan: Channel);

    /// WPADEnableURCC
    ///
    /// Enables or disables Wii U Pro Controller support.
    #[doc(alias = "WPADEnableURCC")]
    #[link_name = "WPADEnableURCC"]
    pub unsafe fn enable_urcc(enable: ffi::c_bool);

    /// WPADIsEnabledURC
    #[doc(alias = "WPADIsEnabledURC")]
    #[link_name = "WPADIsEnabledURC"]
    pub unsafe fn is_enabled_urc() -> ffi::c_bool;

    /// WPADEnableWiiRemote
    #[doc(alias = "WPADEnableWiiRemote")]
    #[link_name = "WPADEnableWiiRemote"]
    pub unsafe fn enable_wii_remote(enable: ffi::c_bool);

    /// WPADEnableWBC
    ///
    /// Enables the Balance Board.
    #[doc(alias = "WPADEnableWBC")]
    #[link_name = "WPADEnableWBC"]
    pub unsafe fn enable_wbc();

    /// WPADIsEnabledWBC
    #[doc(alias = "WPADIsEnabledWBC")]
    #[link_name = "WPADIsEnabledWBC"]
    pub unsafe fn is_enabled_wbc() -> ffi::c_bool;

    /// WPADControlCustomDev
    #[doc(alias = "WPADControlCustomDev")]
    #[link_name = "WPADControlCustomDev"]
    pub unsafe fn control_custom_dev(device_type: DeviceType, enable: ffi::c_bool) -> Error;

    /// WPADIsEnabledCustomDev
    #[doc(alias = "WPADIsEnabledCustomDev")]
    #[link_name = "WPADIsEnabledCustomDev"]
    pub unsafe fn is_enabled_custom_dev(device_type: DeviceType) -> ffi::c_bool;

    /// WPADSetCallbackByKPAD
    #[doc(alias = "WPADSetCallbackByKPAD")]
    #[link_name = "WPADSetCallbackByKPAD"]
    pub unsafe fn set_callback_by_kpad(value: ffi::c_bool);

    /// WPADIsUsedCallbackByKPAD
    #[doc(alias = "WPADIsUsedCallbackByKPAD")]
    #[link_name = "WPADIsUsedCallbackByKPAD"]
    pub unsafe fn is_used_callback_by_kpad() -> ffi::c_bool;

    /// WPADRestoreReportType
    #[doc(alias = "WPADRestoreReportType")]
    #[link_name = "WPADRestoreReportType"]
    pub unsafe fn restore_report_type(chan: Channel, format: DataFormat, power_save: ffi::c_bool);

    /// WPADControlBLC
    #[doc(alias = "WPADControlBLC")]
    #[link_name = "WPADControlBLC"]
    pub unsafe fn control_blc(chan: Channel, command: BlCommand, callback: Callback) -> Error;

    /// WPADGetBLCalibration
    ///
    /// See the `BLCLB_*` constants for `addr` and `len`.
    #[doc(alias = "WPADGetBLCalibration")]
    #[link_name = "WPADGetBLCalibration"]
    pub unsafe fn get_bl_calibration(
        chan: Channel,
        data: *mut u8,
        addr: u16,
        len: u16,
        callback: Callback,
    ) -> Error;

    /// WPADIsRegisteredBLC
    #[doc(alias = "WPADIsRegisteredBLC")]
    #[link_name = "WPADIsRegisteredBLC"]
    pub unsafe fn is_registered_blc() -> ffi::c_bool;

    /// WPADRegisterBLCWorkarea
    ///
    /// `workarea` is ignored, this only calls [enable_wbc].
    #[doc(alias = "WPADRegisterBLCWorkarea")]
    #[link_name = "WPADRegisterBLCWorkarea"]
    pub unsafe fn register_blc_workarea(workarea: *mut ffi::c_void);

    /// WPADGetBLReg
    #[doc(alias = "WPADGetBLReg")]
    #[link_name = "WPADGetBLReg"]
    pub unsafe fn get_bl_reg(
        chan: Channel,
        dst: *mut ffi::c_void,
        addr: u32,
        callback: Callback,
    ) -> Error;

    /// WPADSetBLReg
    #[doc(alias = "WPADSetBLReg")]
    #[link_name = "WPADSetBLReg"]
    pub unsafe fn set_bl_reg(chan: Channel, value: u8, addr: u32, callback: Callback) -> Error;

    /// WPADGetVSMCalibration
    #[doc(alias = "WPADGetVSMCalibration")]
    #[link_name = "WPADGetVSMCalibration"]
    pub unsafe fn get_vsm_calibration(
        chan: Channel,
        dst: *mut ffi::c_void,
        addr: u32,
        len: u32,
        callback: Callback,
    ) -> Error;

    /// WPADGetVSMInputSource
    #[doc(alias = "WPADGetVSMInputSource")]
    #[link_name = "WPADGetVSMInputSource"]
    pub unsafe fn get_vsm_input_source(chan: Channel, value: *mut u8, callback: Callback) -> Error;

    /// WPADSetVSMInputSource
    #[doc(alias = "WPADSetVSMInputSource")]
    #[link_name = "WPADSetVSMInputSource"]
    pub unsafe fn set_vsm_input_source(chan: Channel, value: u8, callback: Callback);

    /// WPADGetVSMLEDDrivePWMDuty
    #[doc(alias = "WPADGetVSMLEDDrivePWMDuty")]
    #[link_name = "WPADGetVSMLEDDrivePWMDuty"]
    pub unsafe fn get_vsm_led_drive_pwm_duty(
        chan: Channel,
        value: *mut u8,
        callback: Callback,
    ) -> Error;

    /// WPADSetVSMLEDDrivePWMDuty
    #[doc(alias = "WPADSetVSMLEDDrivePWMDuty")]
    #[link_name = "WPADSetVSMLEDDrivePWMDuty"]
    pub unsafe fn set_vsm_led_drive_pwm_duty(chan: Channel, value: u8, callback: Callback);

    /// WPADGetVSMPOT1State
    #[doc(alias = "WPADGetVSMPOT1State")]
    #[link_name = "WPADGetVSMPOT1State"]
    pub unsafe fn get_vsm_pot1_state(chan: Channel, state: *mut u8, callback: Callback) -> Error;

    /// WPADSetVSMPOT1State
    #[doc(alias = "WPADSetVSMPOT1State")]
    #[link_name = "WPADSetVSMPOT1State"]
    pub unsafe fn set_vsm_pot1_state(chan: Channel, state: u8, callback: Callback);

    /// WPADGetVSMPOT2State
    #[doc(alias = "WPADGetVSMPOT2State")]
    #[link_name = "WPADGetVSMPOT2State"]
    pub unsafe fn get_vsm_pot2_state(chan: Channel, state: *mut u8, callback: Callback) -> Error;

    /// WPADSetVSMPOT2State
    #[doc(alias = "WPADSetVSMPOT2State")]
    #[link_name = "WPADSetVSMPOT2State"]
    pub unsafe fn set_vsm_pot2_state(chan: Channel, state: u8, callback: Callback);

}
