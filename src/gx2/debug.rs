use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Level {
    #[default]
    None = 0,
    Severe = 1,
    Medium = 2,
    All = 3,
}

unsafe extern "C" {
    #[doc(alias = "GX2SetVerifyLevel")]
    #[link_name = "GX2SetVerifyLevel"]
    pub unsafe fn set_warning_level(level: Level);
}
