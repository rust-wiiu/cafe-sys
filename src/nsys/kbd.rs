//! USB Keyboards (KBD)

use crate::ffi;

pub const MEM_SIZE: usize = 3264;

pub const MAX_CHANNELS: usize = 4;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const OTHER: Self = Self(1);
    pub const NOT_INITIALIZED: Self = Self(2);
    pub const ALREADY_INITIALIZED: Self = Self(3);
    pub const BAD_VALUE: Self = Self(4);
    pub const BAD_STATUS: Self = Self(5);
    pub const BUSY: Self = Self(7);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel(pub u8);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HidCode(pub u8);

impl HidCode {
    pub const NONE: Self = Self(0);
    pub const OVERFLOW: Self = Self(255);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelStatus(pub u32);

impl ChannelStatus {
    pub const OK: Self = Self(0x00);
    pub const DISCONNECTED: Self = Self(0x01);
    pub const ROLL_OVER: Self = Self(0x02);
    pub const ERROR: Self = Self(0x04);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyMode(pub u32);

impl KeyMode {
    pub const NULL: Self = Self(0x00);
    pub const DOWN: Self = Self(0x01);
    pub const REPEAT: Self = Self(0x02);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountryCode(pub u32);

impl CountryCode {
    pub const INTERNATIONAL: Self = Self(0);
    pub const CANADIAN_BI: Self = Self(1);
    pub const CANADIAN_FR: Self = Self(2);
    pub const DANISH: Self = Self(3);
    pub const FINNISH: Self = Self(4);
    pub const FRENCH: Self = Self(5);
    pub const GERMAN: Self = Self(6);
    pub const GREEK: Self = Self(7);
    pub const ITALIAN: Self = Self(8);
    pub const JAPANESE: Self = Self(9);
    pub const LATIN_AMERICAN: Self = Self(10);
    pub const NETHERLANDS_DUTCH: Self = Self(11);
    pub const NORWEGIAN: Self = Self(12);
    pub const PORTUGUESE: Self = Self(13);
    pub const SPANISH: Self = Self(14);
    pub const SWEDISH: Self = Self(15);
    pub const UNITED_KINGDOM: Self = Self(16);
    pub const UNITED_STATES: Self = Self(17);
    pub const RUSSIA: Self = Self(18);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModState(pub u32);

impl ModState {
    pub const NULL: Self = Self(0x0000);
    pub const CTRL: Self = Self(0x0001);
    pub const SHIFT: Self = Self(0x0002);
    pub const ALT: Self = Self(0x0004);
    pub const GUI: Self = Self(0x0008);
    pub const EXTRA: Self = Self(0x0010);
    pub const ALTGR: Self = Self(0x0020);
    pub const LANG1: Self = Self(0x0040);
    pub const LANG2: Self = Self(0x0080);
    pub const NUM_LOCK: Self = Self(0x0100);
    pub const CAPS_LOCK: Self = Self(0x0200);
    pub const SCROLL_LOCK: Self = Self(0x0400);
    pub const SHIFTED_KEY: Self = Self(0x0800);
    pub const SET_ALL: Self = Self(0x1000);
}

impl core::ops::BitOr for ModState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LedState(pub u32);

impl LedState {
    pub const NULL: Self = Self(0x00);
    pub const NUM_LOCK: Self = Self(0x01);
    pub const CAPS_LOCK: Self = Self(0x02);
    pub const SCROLL_LOCK: Self = Self(0x04);
}

impl core::ops::BitOr for LedState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DevEvent {
    pub channel: Channel,
}

/// KBDKeyEvent
#[doc(alias = "KBDKeyEvent")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub channel: Channel,
    pub hid: HidCode,
    pub mode: KeyMode,
    pub mod_state: ModState,
    /// UTF-16 character.
    pub unicode: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LedEvent {
    pub channel: Channel,
    pub leds: LedState,
    pub rc: Error,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Report {
    pub modifier: u8,
    pub reserved: u8,
    pub keycode: [u8; 6],
}

pub type DevCallback = Option<unsafe extern "C" fn(event: *mut DevEvent)>;

pub type KeyCallback = Option<unsafe extern "C" fn(event: *mut KeyEvent)>;

pub type LedCallback =
    Option<unsafe extern "C" fn(event: *mut LedEvent, context: *mut ffi::c_void)>;

unsafe extern "C" {
    /// KBDInit
    ///
    /// Ignores `mem` and calls [setup].
    #[doc(alias = "KBDInit")]
    #[link_name = "KBDInit"]
    pub unsafe fn init(
        mem: *mut ffi::c_void,
        attach: DevCallback,
        detach: DevCallback,
        key: KeyCallback,
    ) -> Error;

    /// KBDExit
    #[doc(alias = "KBDExit")]
    #[link_name = "KBDExit"]
    pub unsafe fn exit() -> Error;

    /// KBDSetup
    #[doc(alias = "KBDSetup")]
    #[link_name = "KBDSetup"]
    pub unsafe fn setup(attach: DevCallback, detach: DevCallback, key: KeyCallback) -> Error;

    /// KBDTeardown
    #[doc(alias = "KBDTeardown")]
    #[link_name = "KBDTeardown"]
    pub unsafe fn teardown() -> Error;

    /// KBDGetChannelStatus
    #[doc(alias = "KBDGetChannelStatus")]
    #[link_name = "KBDGetChannelStatus"]
    pub unsafe fn get_channel_status(channel: Channel, status: *mut ChannelStatus) -> Error;

    /// KBDResetChannel
    #[doc(alias = "KBDResetChannel")]
    #[link_name = "KBDResetChannel"]
    pub unsafe fn reset_channel(channel: Channel) -> Error;

    /// KBDSetCountry
    #[doc(alias = "KBDSetCountry")]
    #[link_name = "KBDSetCountry"]
    pub unsafe fn set_country(channel: Channel, country: CountryCode) -> Error;

    /// KBDGetCountry
    #[doc(alias = "KBDGetCountry")]
    #[link_name = "KBDGetCountry"]
    pub unsafe fn get_country(channel: Channel, country: *mut CountryCode) -> Error;

    /// KBDGetKey
    #[doc(alias = "KBDGetKey")]
    #[link_name = "KBDGetKey"]
    pub unsafe fn get_key(channel: Channel, event: *mut KeyEvent) -> Error;

    /// KBDEmptyQueue
    #[doc(alias = "KBDEmptyQueue")]
    #[link_name = "KBDEmptyQueue"]
    pub unsafe fn empty_queue(channel: Channel) -> Error;

    /// KBDSetLockProcessing
    #[doc(alias = "KBDSetLockProcessing")]
    #[link_name = "KBDSetLockProcessing"]
    pub unsafe fn set_lock_processing(channel: Channel, enabled: ffi::c_bool) -> Error;

    /// KBDGetLockProcessing
    #[doc(alias = "KBDGetLockProcessing")]
    #[link_name = "KBDGetLockProcessing"]
    pub unsafe fn get_lock_processing(channel: Channel, enabled: *mut ffi::c_bool) -> Error;

    /// KBDSetModState
    #[doc(alias = "KBDSetModState")]
    #[link_name = "KBDSetModState"]
    pub unsafe fn set_mod_state(channel: Channel, state: ModState) -> Error;

    /// KBDGetModState
    #[doc(alias = "KBDGetModState")]
    #[link_name = "KBDGetModState"]
    pub unsafe fn get_mod_state(channel: Channel, state: *mut ModState) -> Error;

    /// KBDSetRepeat
    #[doc(alias = "KBDSetRepeat")]
    #[link_name = "KBDSetRepeat"]
    pub unsafe fn set_repeat(channel: Channel, delay: u16, interval: u16) -> Error;

    /// KBDGetRepeat
    #[doc(alias = "KBDGetRepeat")]
    #[link_name = "KBDGetRepeat"]
    pub unsafe fn get_repeat(channel: Channel, delay: *mut u16, interval: *mut u16) -> Error;

    /// KBDSetAccessSticky
    #[doc(alias = "KBDSetAccessSticky")]
    #[link_name = "KBDSetAccessSticky"]
    pub unsafe fn set_access_sticky(channel: Channel, enabled: ffi::c_bool) -> Error;

    /// KBDGetAccessSticky
    #[doc(alias = "KBDGetAccessSticky")]
    #[link_name = "KBDGetAccessSticky"]
    pub unsafe fn get_access_sticky(channel: Channel, enabled: *mut ffi::c_bool) -> Error;

    /// KBDSetLeds
    #[doc(alias = "KBDSetLeds")]
    #[link_name = "KBDSetLeds"]
    pub unsafe fn set_leds(channel: Channel, leds: LedState) -> Error;

    /// KBDSetLedsAsync
    #[doc(alias = "KBDSetLedsAsync")]
    #[link_name = "KBDSetLedsAsync"]
    pub unsafe fn set_leds_async(
        channel: Channel,
        leds: LedState,
        callback: LedCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// KBDSetLedsRetry
    #[doc(alias = "KBDSetLedsRetry")]
    #[link_name = "KBDSetLedsRetry"]
    pub unsafe fn set_leds_retry(channel: Channel, leds: LedState) -> Error;

    /// KBDSetLedsEx
    #[doc(alias = "KBDSetLedsEx")]
    #[link_name = "KBDSetLedsEx"]
    pub unsafe fn set_leds_ex(channel: Channel, leds: LedState) -> Error;

    /// KBDSetLedsAsyncEx
    #[doc(alias = "KBDSetLedsAsyncEx")]
    #[link_name = "KBDSetLedsAsyncEx"]
    pub unsafe fn set_leds_async_ex(
        channel: Channel,
        leds: LedState,
        callback: LedCallback,
        context: *mut ffi::c_void,
    ) -> Error;

    /// KBDSetLedsRetryEx
    #[doc(alias = "KBDSetLedsRetryEx")]
    #[link_name = "KBDSetLedsRetryEx"]
    pub unsafe fn set_leds_retry_ex(channel: Channel, leds: LedState) -> Error;

    /// KBDTranslateHidCode
    ///
    /// Translates a HID usage code to a UTF-16 character.
    #[doc(alias = "KBDTranslateHidCode")]
    #[link_name = "KBDTranslateHidCode"]
    pub unsafe fn translate_hid_code(
        hid: HidCode,
        mod_state: ModState,
        country: CountryCode,
    ) -> u16;
}

/// Key processing: alt-keypad codes, dead keys and romaji to kana conversion.
pub mod kpr {
    pub const FLUSH_ALT_KEYPAD_CHAR: u16 = 0x0000;

    pub const FLUSH_ALL_CHAR: u16 = 0xFFFF;

    pub const MAX_QUEUE_LEN: usize = 5;

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Mode(pub u32);

    impl Mode {
        pub const NONE: Self = Self(0x00);
        pub const ALT_KEYPAD: Self = Self(0x01);
        pub const DEADKEY: Self = Self(0x02);
        pub const JP_ROMAJI_HIRAGANA: Self = Self(0x04);
        pub const JP_ROMAJI_KATAKANA: Self = Self(0x08);
    }

    impl core::ops::BitOr for Mode {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self {
            Self(self.0 | rhs.0)
        }
    }

    /// Initialize with [init_queue].
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Queue {
        pub text: [u16; MAX_QUEUE_LEN],
        pub mode: Mode,
        pub out_count: u8,
        pub in_count: u8,
        pub alt_value: u32,
    }

    unsafe extern "C" {
        /// KPRInitRegionUS
        #[doc(alias = "KPRInitRegionUS")]
        #[link_name = "KPRInitRegionUS"]
        pub unsafe fn init_region_us();

        /// KPRInitRegionJP
        #[doc(alias = "KPRInitRegionJP")]
        #[link_name = "KPRInitRegionJP"]
        pub unsafe fn init_region_jp();

        /// KPRInitRegionEU
        #[doc(alias = "KPRInitRegionEU")]
        #[link_name = "KPRInitRegionEU"]
        pub unsafe fn init_region_eu();

        /// KPRInitQueue
        #[doc(alias = "KPRInitQueue")]
        #[link_name = "KPRInitQueue"]
        pub unsafe fn init_queue(queue: *mut Queue);

        /// KPRClearQueue
        #[doc(alias = "KPRClearQueue")]
        #[link_name = "KPRClearQueue"]
        pub unsafe fn clear_queue(queue: *mut Queue);

        /// KPRSetMode
        #[doc(alias = "KPRSetMode")]
        #[link_name = "KPRSetMode"]
        pub unsafe fn set_mode(queue: *mut Queue, mode: Mode);

        /// KPRGetMode
        #[doc(alias = "KPRGetMode")]
        #[link_name = "KPRGetMode"]
        pub unsafe fn get_mode(queue: *mut Queue) -> Mode;

        /// KPRPutChar
        #[doc(alias = "KPRPutChar")]
        #[link_name = "KPRPutChar"]
        pub unsafe fn put_char(queue: *mut Queue, c: u16) -> u8;

        /// KPRGetChar
        #[doc(alias = "KPRGetChar")]
        #[link_name = "KPRGetChar"]
        pub unsafe fn get_char(queue: *mut Queue) -> u16;

        /// KPRRemoveChar
        #[doc(alias = "KPRRemoveChar")]
        #[link_name = "KPRRemoveChar"]
        pub unsafe fn remove_char(queue: *mut Queue) -> u16;

        /// KPRLookAhead
        #[doc(alias = "KPRLookAhead")]
        #[link_name = "KPRLookAhead"]
        pub unsafe fn look_ahead(queue: *mut Queue, buf: *mut u16, max_size: u32) -> u8;
    }
}

imports_section!(
    "nsyskbd",
    [
        "KBDEmptyQueue",
        "KBDExit",
        "KBDGetAccessSticky",
        "KBDGetChannelStatus",
        "KBDGetCountry",
        "KBDGetKey",
        "KBDGetLockProcessing",
        "KBDGetModState",
        "KBDGetRepeat",
        "KBDInit",
        "KBDInitRegionEU",
        "KBDInitRegionJP",
        "KBDInitRegionUS",
        "KBDResetChannel",
        "KBDSetAccessSticky",
        "KBDSetCountry",
        "KBDSetLeds",
        "KBDSetLedsAsync",
        "KBDSetLedsAsyncEx",
        "KBDSetLedsEx",
        "KBDSetLedsRetry",
        "KBDSetLedsRetryEx",
        "KBDSetLockProcessing",
        "KBDSetModState",
        "KBDSetRepeat",
        "KBDSetup",
        "KBDTeardown",
        "KBDTranslateHidCode",
        "KPRClearQueue",
        "KPRGetChar",
        "KPRGetMode",
        "KPRInitQueue",
        "KPRInitRegionEU",
        "KPRInitRegionJP",
        "KPRInitRegionUS",
        "KPRLookAhead",
        "KPRPutChar",
        "KPRRemoveChar",
        "KPRSetMode",
        "SKBDEmptyQueue",
        "SKBDGetAccessSticky",
        "SKBDGetChannelStatus",
        "SKBDGetCountry",
        "SKBDGetKey",
        "SKBDGetLockProcessing",
        "SKBDGetModState",
        "SKBDGetRepeat",
        "SKBDKeyEvent",
        "SKBDResetChannel",
        "SKBDSetAccessSticky",
        "SKBDSetCountry",
        "SKBDSetLockProcessing",
        "SKBDSetModState",
        "SKBDSetMode",
        "SKBDSetRepeat",
        "SKBDSetup",
        "SKBDTeardown"
    ],
    []
);
