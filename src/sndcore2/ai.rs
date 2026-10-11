//! Audio interface
//!
//! Low-level audio DMA driver, used by AX.

use crate::ffi;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleRate(pub u32);

impl SampleRate {
    pub const KHZ_32: Self = Self(0);
    pub const KHZ_48: Self = Self(1);
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    #[default]
    Stereo = 0,
    MultiPcm = 1,
    Bitstream = 2,
}

pub type DmaCallback = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    /// AIInit
    #[doc(alias = "AIInit")]
    #[link_name = "AIInit"]
    pub unsafe fn init(stack: *mut u8);

    /// AICheckInit
    #[doc(alias = "AICheckInit")]
    #[link_name = "AICheckInit"]
    pub unsafe fn check_init() -> ffi::c_bool;

    /// AIReset
    #[doc(alias = "AIReset")]
    #[link_name = "AIReset"]
    pub unsafe fn reset();

    /// AIQuit
    #[doc(alias = "AIQuit")]
    #[link_name = "AIQuit"]
    pub unsafe fn quit();

    /// AISetChannel
    #[doc(alias = "AISetChannel")]
    #[link_name = "AISetChannel"]
    pub unsafe fn set_channel(channel: Channel, stuffing: ffi::c_bool);

    /// AIGetAudioFrameCount
    #[doc(alias = "AIGetAudioFrameCount")]
    #[link_name = "AIGetAudioFrameCount"]
    pub unsafe fn get_audio_frame_count() -> u32;

    /// AIGetDSPSampleRate
    #[doc(alias = "AIGetDSPSampleRate")]
    #[link_name = "AIGetDSPSampleRate"]
    pub unsafe fn get_dsp_sample_rate() -> SampleRate;

    /// AISetDSPSampleRate
    #[doc(alias = "AISetDSPSampleRate")]
    #[link_name = "AISetDSPSampleRate"]
    pub unsafe fn set_dsp_sample_rate(rate: SampleRate);

    /// AIRegisterDMACallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "AIRegisterDMACallback")]
    #[link_name = "AIRegisterDMACallback"]
    pub unsafe fn register_dma_callback(callback: DmaCallback) -> DmaCallback;

    /// AIDRCRegisterDMACallback
    ///
    /// Returns the previous callback.
    #[doc(alias = "AIDRCRegisterDMACallback")]
    #[link_name = "AIDRCRegisterDMACallback"]
    pub unsafe fn drc_register_dma_callback(callback: DmaCallback) -> DmaCallback;

    /// AIInitDMA
    #[doc(alias = "AIInitDMA")]
    #[link_name = "AIInitDMA"]
    pub unsafe fn init_dma(start_addr: u32, length: u32);

    /// AIGetDMAEnableFlag
    #[doc(alias = "AIGetDMAEnableFlag")]
    #[link_name = "AIGetDMAEnableFlag"]
    pub unsafe fn get_dma_enable_flag() -> ffi::c_bool;

    /// AIStartDMA
    #[doc(alias = "AIStartDMA")]
    #[link_name = "AIStartDMA"]
    pub unsafe fn start_dma();

    /// AIStopDMA
    #[doc(alias = "AIStopDMA")]
    #[link_name = "AIStopDMA"]
    pub unsafe fn stop_dma();

    /// AIGetDMABytesLeft
    #[doc(alias = "AIGetDMABytesLeft")]
    #[link_name = "AIGetDMABytesLeft"]
    pub unsafe fn get_dma_bytes_left() -> u32;

    /// AIGetDMAStartAddr
    #[doc(alias = "AIGetDMAStartAddr")]
    #[link_name = "AIGetDMAStartAddr"]
    pub unsafe fn get_dma_start_addr() -> u32;

    /// AIDRCGetDMAStartAddr
    #[doc(alias = "AIDRCGetDMAStartAddr")]
    #[link_name = "AIDRCGetDMAStartAddr"]
    pub unsafe fn drc_get_dma_start_addr() -> u32;

    /// AIGetDMALength
    #[doc(alias = "AIGetDMALength")]
    #[link_name = "AIGetDMALength"]
    pub unsafe fn get_dma_length() -> u32;

    /// AIDRCGetDMALength
    #[doc(alias = "AIDRCGetDMALength")]
    #[link_name = "AIDRCGetDMALength"]
    pub unsafe fn drc_get_dma_length() -> u32;

    /// AIGetDMARange
    #[doc(alias = "AIGetDMARange")]
    #[link_name = "AIGetDMARange"]
    pub unsafe fn get_dma_range(start_addr: u32, addr_hi: *mut u32, addr_lo: *mut u32);
}
