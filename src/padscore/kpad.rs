use super::wpad::*;
use crate::ffi::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[doc(alias = "KPADVec2D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[doc(alias = "KPADVec3D")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[doc(alias = "KPADError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Error {
    Ok = 0,
    NoSamples = -1,
    InvalidController = -2,
    WpadUninitialized = -3,
    Busy = -4,
    Uninitialized = -5,
}

#[doc(alias = "WPADExtensionType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ExtensionType {
    Core = 0x00,
    Nunchuk = 0x01,
    Classic = 0x02,
    BalanceBoard = 0x03,
    MotionPlus = 0x05,
    MotionPlusNunchuk = 0x06,
    MotionPlusClassic = 0x07,
    Train = 0x10,
    Guitar = 0x11,
    Drum = 0x12,
    Taiko = 0x13,
    Urcc = 0x1f,
    DevNotFound = 0xfd,
    Unknown = 0xff,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NunchukExt {
    pub stick: Vec2,
    pub accelerometer: Vec3,
    pub magbitude: f32,
    pub variation: f32,
    pub hold: NunchukButton,
    pub trigger: NunchukButton,
    pub release: NunchukButton,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ClassicExt {
    pub hold: ClassicButton,
    pub trigger: ClassicButton,
    pub release: ClassicButton,
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct UrccExt {
    pub hold: URCCButton,
    pub trigger: URCCButton,
    pub release: URCCButton,
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub charging: i32,
    pub wired: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BalanceBoardExt {
    pub avg_tgc_weight: f64,
    pub weights: [f64; 4],
    pub avg_weights: [f64; 4],
    pub error: i32,
    pub calibration: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Extension {
    pub nunchuk: NunchukExt,
    pub classic: ClassicExt,
    pub urcc: UrccExt,
    pub balance: BalanceBoardExt,
}

impl core::fmt::Debug for Extension {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Extension {{ UNION }}")
    }
}

impl PartialEq for Extension {
    fn eq(&self, other: &Self) -> bool {
        let self_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                (self as *const Extension) as *const u8,
                core::mem::size_of::<Extension>(),
            )
        };

        let other_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                (other as *const Extension) as *const u8,
                core::mem::size_of::<Extension>(),
            )
        };

        self_bytes == other_bytes
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MotionPlus {
    pub acc: Vec3,
    pub angles: Vec3,
    pub dir_x: Vec3,
    pub dir_y: Vec3,
    pub dir_z: Vec3,
}

#[doc(alias = "KPADStatus")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    pub hold: Button,
    pub trigger: Button,
    pub release: Button,
    pub acc: Vec3,                 // TODO: renaming / packaging
    pub acc_magnitude: f32,        // TODO: renaming / packaging
    pub acc_variation: f32,        // TODO: renaming / packaging
    pub pos: Vec2,                 // TODO: renaming / packaging
    pub pos_diff: Vec2,            // TODO: renaming / packaging
    pub pos_diff_magnitude: f32,   // TODO: renaming / packaging
    pub angle: Vec2,               // TODO: renaming / packaging
    pub angle_diff: Vec2,          // TODO: renaming / packaging
    pub angle_diff_magnitude: f32, // TODO: renaming / packaging
    pub dist: f32,                 // TODO: renaming / packaging
    pub dist_diff: f32,            // TODO: renaming / packaging
    pub dist_diff_magnitude: f32,  // TODO: renaming / packaging
    pub down: Vec2,                // TODO: renaming / packaging
    pub extension_type: ExtensionType,
    pub error: i8,
    pub position_valid: bool,
    pub format: u8,
    pub extension: Extension,
    pub mplus: MotionPlus,
    pub _pad: unknown<0x4>,
}

unsafe extern "C" {
    #[doc(alias = "KPADInit")]
    #[link_name = "KPADInit"]
    pub unsafe fn init();

    #[doc(alias = "KPADShutdown")]
    #[link_name = "KPADShutdown"]
    pub unsafe fn deinit();

    #[doc(alias = "KPADReadEx")]
    #[link_name = "KPADReadEx"]
    pub unsafe fn poll(
        channel: Channel,
        buffers: *mut Status,
        samples: u32,
        error: *mut Error,
    ) -> i32;
}
