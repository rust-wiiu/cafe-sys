//! TV Encoder (TVE)
//!
//! TV output status and HDMI-CEC.

use crate::ffi;

/// TVECECLogicalAddress
#[doc(alias = "TVECECLogicalAddress")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CecLogicalAddress(pub u32);

impl CecLogicalAddress {
    pub const TV: Self = Self(0x0);
    pub const RECORDING_DEVICE_1: Self = Self(0x1);
    pub const RECORDING_DEVICE_2: Self = Self(0x2);
    pub const TUNER_1: Self = Self(0x3);
    pub const PLAYBACK_DEVICE_1: Self = Self(0x4);
    pub const AUDIO_SYSTEM: Self = Self(0x5);
    pub const TUNER_2: Self = Self(0x6);
    pub const TUNER_3: Self = Self(0x7);
    pub const PLAYBACK_DEVICE_2: Self = Self(0x8);
    pub const RECORDING_DEVICE_3: Self = Self(0x9);
    pub const TUNER_4: Self = Self(0xA);
    pub const PLAYBACK_DEVICE_3: Self = Self(0xB);
    pub const RESERVED_1: Self = Self(0xC);
    pub const RESERVED_2: Self = Self(0xD);
    pub const FREE_USE: Self = Self(0xE);
    pub const UNREGISTERED: Self = Self(0xF);
    pub const BROADCAST: Self = Self(0xF);
}

/// TVECECOpCode
#[doc(alias = "TVECECOpCode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CecOpCode(pub u32);

impl CecOpCode {
    pub const FEATURE_ABORT: Self = Self(0x00);
    pub const IMAGE_VIEW_ON: Self = Self(0x04);
    pub const TUNER_STEP_INCREMENT: Self = Self(0x05);
    pub const TUNER_STEP_DECREMENT: Self = Self(0x06);
    pub const TUNER_DEVICE_STATUS: Self = Self(0x07);
    pub const GIVE_TUNER_DEVICE_STATUS: Self = Self(0x08);
    pub const RECORD_ON: Self = Self(0x09);
    pub const RECORD_STATUS: Self = Self(0x0A);
    pub const RECORD_OFF: Self = Self(0x0B);
    pub const TEXT_VIEW_ON: Self = Self(0x0D);
    pub const RECORD_TV_SCREEN: Self = Self(0x0F);
    pub const GIVE_DECK_STATUS: Self = Self(0x1A);
    pub const DECK_STATUS: Self = Self(0x1B);
    pub const SET_MENU_LANGUAGE: Self = Self(0x32);
    pub const CLEAR_ANALOGUE_TIMER: Self = Self(0x33);
    pub const SET_ANALOGUE_TIMER: Self = Self(0x34);
    pub const TIMER_STATUS: Self = Self(0x35);
    pub const STANDBY: Self = Self(0x36);
    pub const PLAY: Self = Self(0x41);
    pub const DECK_CONTROL: Self = Self(0x42);
    pub const TIMER_CLEARED_STATUS: Self = Self(0x43);
    pub const USER_CONTROL_PRESSED: Self = Self(0x44);
    pub const USER_CONTROL_RELEASE: Self = Self(0x45);
    pub const GIVE_OSD_NAME: Self = Self(0x46);
    pub const SET_OSD_NAME: Self = Self(0x47);
    pub const SET_OSD_STRING: Self = Self(0x64);
    pub const SET_TIMER_PROGRAM_TITLE: Self = Self(0x67);
    pub const SYSTEM_AUDIO_MODE_REQUEST: Self = Self(0x70);
    pub const GIVE_AUDIO_STATUS: Self = Self(0x71);
    pub const SET_SYSTEM_AUDIO_MODE: Self = Self(0x72);
    pub const REPORT_AUDIO_STATUS: Self = Self(0x7A);
    pub const GIVE_SYSTEM_AUDIO_MODE_STATUS: Self = Self(0x7D);
    pub const SYSTEM_AUDIO_MODE_STATUS: Self = Self(0x7E);
    pub const ROUTING_CHANGE: Self = Self(0x80);
    pub const ROUTING_INFORMATION: Self = Self(0x81);
    pub const ACTIVE_SOURCE: Self = Self(0x82);
    pub const GIVE_PHYSICAL_ADDRESS: Self = Self(0x83);
    pub const REPORT_PHYSICAL_ADDRESS: Self = Self(0x84);
    pub const REQUEST_ACTIVE_SOURCE: Self = Self(0x85);
    pub const SET_STREAM_PATH: Self = Self(0x86);
    pub const DEVICE_VENDOR_ID: Self = Self(0x87);
    pub const VENDOR_COMMAND: Self = Self(0x89);
    pub const VENDOR_REMOTE_BUTTON_DOWN: Self = Self(0x8A);
    pub const VENDOR_REMOTE_BUTTON_UP: Self = Self(0x8B);
    pub const GIVE_DEVICE_VENDOR_ID: Self = Self(0x8C);
    pub const MENU_REQUEST: Self = Self(0x8D);
    pub const MENU_STATUS: Self = Self(0x8E);
    pub const GIVE_DEVICE_POWER_STATUS: Self = Self(0x8F);
    pub const REPORT_POWER_STATUS: Self = Self(0x90);
    pub const GET_MENU_LANGUAGE: Self = Self(0x91);
    pub const SELECT_ANALOGUE_SERVICE: Self = Self(0x92);
    pub const SELECT_DIGITAL_SERVICE: Self = Self(0x93);
    pub const SET_DIGITAL_TIMER: Self = Self(0x97);
    pub const CLEAR_DIGITAL_TIMER: Self = Self(0x99);
    pub const SET_AUDIO_RATE: Self = Self(0x9A);
    pub const INACTIVE_SOURCE: Self = Self(0x9D);
    pub const CEC_VERSION: Self = Self(0x9E);
    pub const GET_CEC_VERSION: Self = Self(0x9F);
    pub const VENDOR_COMMAND_WITH_ID: Self = Self(0xA0);
    pub const CLEAR_EXTERNAL_TIMER: Self = Self(0xA1);
    pub const SET_EXTERNAL_TIMER: Self = Self(0xA2);
    pub const ABORT: Self = Self(0xFF);
}

/// TVEPort
#[doc(alias = "TVEPort")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(pub u32);

impl Port {
    pub const HDMI: Self = Self(0);
    pub const COMPONENT: Self = Self(1);
    pub const COMPOSITE: Self = Self(2);
    pub const SCART: Self = Self(3);
}

/// TVEHdmiState
#[doc(alias = "TVEHdmiState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HdmiState(pub u32);

impl HdmiState {
    pub const NONE: Self = Self(0);
    pub const HTPG_OFF: Self = Self(1);
    pub const RXWAIT: Self = Self(2);
    pub const EDID_COMP: Self = Self(3);
    pub const DUMMY_TMDS: Self = Self(4);
    pub const W4WK: Self = Self(5);
    #[doc(alias = "TVE_HDMI_STATE_1STA")]
    pub const FIRST_A: Self = Self(6);
    #[doc(alias = "TVE_HDMI_STATE_2NDA")]
    pub const SECOND_A: Self = Self(7);
    #[doc(alias = "TVE_HDMI_STATE_3RDA")]
    pub const THIRD_A: Self = Self(8);
    pub const STAV_OFF: Self = Self(9);
    pub const DONE: Self = Self(10);
    pub const OTHER: Self = Self(11);
    pub const TMDSDOWN: Self = Self(12);
    pub const SHUTDOWN: Self = Self(13);
    pub const WII: Self = Self(14);
    pub const ERR_NRESET: Self = Self(15);
    pub const ERR_INT_LINE: Self = Self(16);
    pub const ERR_KEY_LOAD: Self = Self(17);
    pub const ERR_EDID_READ: Self = Self(18);
    pub const ERR_HOTPLUG: Self = Self(19);
    pub const ERR_VSYNC: Self = Self(20);
    pub const ERR_HDCP: Self = Self(21);
    pub const ERR_SYS: Self = Self(22);
    pub const ERR_CEC: Self = Self(23);
    pub const ERR_W41A: Self = Self(24);
    pub const ERR_W4ED: Self = Self(25);
    pub const UNKNOWN_1A: Self = Self(26);
}

unsafe extern "C" {
    /// TVECECInit
    #[doc(alias = "TVECECInit")]
    #[link_name = "TVECECInit"]
    pub unsafe fn cec_init() -> ffi::c_bool;

    /// TVESetCECEnable
    #[doc(alias = "TVESetCECEnable")]
    #[link_name = "TVESetCECEnable"]
    pub unsafe fn set_cec_enable(enable: ffi::c_bool) -> ffi::c_bool;

    /// TVEIsCECEnable
    #[doc(alias = "TVEIsCECEnable")]
    #[link_name = "TVEIsCECEnable"]
    pub unsafe fn is_cec_enable() -> ffi::c_bool;

    /// TVECECSendCommand
    #[doc(alias = "TVECECSendCommand")]
    #[link_name = "TVECECSendCommand"]
    pub unsafe fn cec_send_command(
        destination: CecLogicalAddress,
        op_code: CecOpCode,
        parameters: *const u8,
        num_parameters: u8,
    ) -> ffi::c_bool;

    /// TVECECReceiveCommand
    #[doc(alias = "TVECECReceiveCommand")]
    #[link_name = "TVECECReceiveCommand"]
    pub unsafe fn cec_receive_command(
        initiator: *mut CecLogicalAddress,
        op_code: *mut CecOpCode,
        parameters: *mut u8,
        num_parameters: *mut u8,
    ) -> ffi::c_bool;

    /// TVEGetAnalogStat
    #[doc(alias = "TVEGetAnalogStat")]
    #[link_name = "TVEGetAnalogStat"]
    pub unsafe fn get_analog_stat(state: *mut u32);

    /// TVEGetCurrentPort
    #[doc(alias = "TVEGetCurrentPort")]
    #[link_name = "TVEGetCurrentPort"]
    pub unsafe fn get_current_port() -> Port;

    /// TVEGetHDMIErrorStat
    #[doc(alias = "TVEGetHDMIErrorStat")]
    #[link_name = "TVEGetHDMIErrorStat"]
    pub unsafe fn get_hdmi_error_stat(state: *mut HdmiState);
}

imports_section!(
    "tve",
    [
        "I2CInit",
        "I2CMEnable",
        "I2CWriteDRCData",
        "I2CWriteDRCInfo",
        "TVECECInit",
        "TVECECReceiveCommand",
        "TVECECSendCommand",
        "TVECreateSysConfig",
        "TVEDisableStereoMode",
        "TVEDisableVideoOut",
        "TVEEnableVideoOut",
        "TVEExit",
        "TVEGet3DFormatEDID",
        "TVEGet3DInfo",
        "TVEGetAnalogAudioDACEnable",
        "TVEGetAnalogDTVStatus",
        "TVEGetAnalogStat",
        "TVEGetAudioConfig",
        "TVEGetAudioMute",
        "TVEGetCGMS",
        "TVEGetConfigVersion",
        "TVEGetContentType",
        "TVEGetCurrentConfig",
        "TVEGetCurrentPort",
        "TVEGetEDIDBaseCheckSum",
        "TVEGetEDIDInfo",
        "TVEGetEDIDRaw",
        "TVEGetHDCP",
        "TVEGetHDMIErrorStat",
        "TVEGetHotPlugState",
        "TVEGetMaxAudioChannel",
        "TVEGetSinkHDCPInfo",
        "TVEGetWSS",
        "TVEInit",
        "TVEIsCECEnable",
        "TVEIsForceRGBEnable",
        "TVEReset4in1",
        "TVESet3DInfo",
        "TVESetAll",
        "TVESetAllAsync",
        "TVESetAllEx",
        "TVESetAllExAsync",
        "TVESetAnalogAudioDACEnable",
        "TVESetAnalogAudioDACFlag",
        "TVESetAudioConfig",
        "TVESetAudioMute",
        "TVESetCECEnable",
        "TVESetCGMS",
        "TVESetContentType",
        "TVESetForceRGBMode",
        "TVESetForceShutDown",
        "TVESetHDCP",
        "TVESetHDMIHotplug5V",
        "TVESetRCA",
        "TVESetVideoMute",
        "TVESetWSS",
        "TVESetWiiCompat",
        "TVESetWiiCompatEx",
        "TVEShutDown",
        "TVEVsyncSignal",
        "TVEWait4in1Done",
        "TVEWait4in1DoneWithTimeout",
        "__TVESetContentTypeParam",
        "__TVESetRCAFlag"
    ],
    []
);
