use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[doc(alias = "KPADChan")]
#[doc(alias = "WPADChan")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum Channel {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

bitflags! {
    #[doc(alias = "WPADButton")]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Button: u32 {
        const Left = 0x0001;
        const Right = 0x0002;
        const Down = 0x0004;
        const Up = 0x0008;
        const Plus = 0x0010;
        const Two = 0x0100;
        const One = 0x0200;
        const B = 0x0400;
        const A = 0x0800;
        const Minus = 0x1000;
        const Z = 0x2000;
        const C = 0x4000;
        const Home = 0x8000;
    }

    #[doc(alias = "WPADNunchukButton")]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct NunchukButton : u32 {
        const LStickLeft = 0x0001;
        const LStickRight = 0x0002;
        const LStickDown = 0x0004;
        const LStickUp = 0x0008;
        const Z = 0x2000;
        const C = 0x4000;
    }

    #[doc(alias = "WPADClassicButton")]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ClassicButton : u32 {
        const Up = 0x00000001;
        const Left = 0x00000002;
        const ZR = 0x00000004;
        const X = 0x00000008;
        const A = 0x00000010;
        const Y = 0x00000020;
        const B = 0x00000040;
        const ZL = 0x00000080;
        const R = 0x00000200;
        const Plus = 0x00000400;
        const Home = 0x00000800;
        const Minus = 0x00001000;
        const L = 0x00002000;
        const Down = 0x00004000;
        const Right = 0x00008000;
        const LStickLeft = 0x00010000;
        const LStickRight = 0x00020000;
        const LStickDown = 0x00040000;
        const LStickUp = 0x00080000;
        const RStickLeft = 0x00100000;
        const RStickRight = 0x00200000;
        const RStickDown = 0x00400000;
        const RStickUp = 0x00800000;
    }

    #[doc(alias = "WPADClassicButton")]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct URCCButton : u32 {
        const Up = 0x00000001;
        const Left = 0x00000002;
        const ZR = 0x00000004;
        const X = 0x00000008;
        const A = 0x00000010;
        const Y = 0x00000020;
        const B = 0x00000040;
        const ZL = 0x00000080;
        const R = 0x00000200;
        const Plus = 0x00000400;
        const Home = 0x00000800;
        const Minus = 0x00001000;
        const L = 0x00002000;
        const Down = 0x00004000;
        const Right = 0x00008000;
        const R3 = 0x00010000;
        const L3 = 0x00020000;
        const LStickUp = 0x00200000;
        const LStickDown = 0x00100000;
        const LStickLeft = 0x00040000;
        const LStickRight = 0x00080000;
        const RStickUp = 0x02000000;
        const RStickDown = 0x01000000;
        const RStickLeft = 0x00400000;
        const RStickRight = 0x00800000;
    }
}

unsafe extern "C" {
    #[doc(alias = "WPADEnableURCC")]
    #[link_name = "WPADEnableURCC"]
    pub unsafe fn enable_urcc(enable: bool);

    #[doc(alias = "WPADEnableWBC")]
    #[link_name = "WPADEnableWBC"]
    pub unsafe fn enable_wbc(enable: bool);

    #[doc(alias = "WPADEnableWiiRemote")]
    #[link_name = "WPADEnableWiiRemote"]
    pub unsafe fn enable_wiimote(enable: bool);
}
