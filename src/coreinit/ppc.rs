//! PPC
//!
//! Processor register access and synchronization instructions, plus simple performance counting.

use crate::ffi;

unsafe extern "C" {
    /// PPCMfmsr
    ///
    /// Reads MSR.
    #[doc(alias = "PPCMfmsr")]
    #[link_name = "PPCMfmsr"]
    pub unsafe fn mfmsr() -> u32;

    /// PPCMtmsr
    ///
    /// Writes MSR.
    #[doc(alias = "PPCMtmsr")]
    #[link_name = "PPCMtmsr"]
    pub unsafe fn mtmsr(value: u32);

    /// PPCMfhid0
    ///
    /// Reads HID0.
    #[doc(alias = "PPCMfhid0")]
    #[link_name = "PPCMfhid0"]
    pub unsafe fn mfhid0() -> u32;

    /// PPCMthid0
    ///
    /// Writes HID0.
    #[doc(alias = "PPCMthid0")]
    #[link_name = "PPCMthid0"]
    pub unsafe fn mthid0(value: u32);

    /// PPCMfhid1
    ///
    /// Reads HID1.
    #[doc(alias = "PPCMfhid1")]
    #[link_name = "PPCMfhid1"]
    pub unsafe fn mfhid1() -> u32;

    /// PPCMfhid2
    ///
    /// Reads HID2.
    #[doc(alias = "PPCMfhid2")]
    #[link_name = "PPCMfhid2"]
    pub unsafe fn mfhid2() -> u32;

    /// PPCMthid2
    ///
    /// Writes HID2.
    #[doc(alias = "PPCMthid2")]
    #[link_name = "PPCMthid2"]
    pub unsafe fn mthid2(value: u32);

    /// PPCMfhid4
    ///
    /// Reads HID4.
    #[doc(alias = "PPCMfhid4")]
    #[link_name = "PPCMfhid4"]
    pub unsafe fn mfhid4() -> u32;

    /// PPCMthid4
    ///
    /// Writes HID4.
    #[doc(alias = "PPCMthid4")]
    #[link_name = "PPCMthid4"]
    pub unsafe fn mthid4(value: u32);

    /// PPCMfwpar
    ///
    /// Reads WPAR.
    #[doc(alias = "PPCMfwpar")]
    #[link_name = "PPCMfwpar"]
    pub unsafe fn mfwpar() -> u32;

    /// PPCMtwpar
    ///
    /// Writes WPAR.
    #[doc(alias = "PPCMtwpar")]
    #[link_name = "PPCMtwpar"]
    pub unsafe fn mtwpar(value: u32);

    /// PPCMfdmaU
    ///
    /// Reads DMAU.
    #[doc(alias = "PPCMfdmaU")]
    #[link_name = "PPCMfdmaU"]
    pub unsafe fn mfdmau() -> u32;

    /// PPCMtdmaU
    ///
    /// Writes DMAU.
    #[doc(alias = "PPCMtdmaU")]
    #[link_name = "PPCMtdmaU"]
    pub unsafe fn mtdmau(value: u32);

    /// PPCMfdmaL
    ///
    /// Reads DMAL.
    #[doc(alias = "PPCMfdmaL")]
    #[link_name = "PPCMfdmaL"]
    pub unsafe fn mfdmal() -> u32;

    /// PPCMtdmaL
    ///
    /// Writes DMAL.
    #[doc(alias = "PPCMtdmaL")]
    #[link_name = "PPCMtdmaL"]
    pub unsafe fn mtdmal(value: u32);

    /// PPCMfl2cr
    ///
    /// Reads L2CR.
    #[doc(alias = "PPCMfl2cr")]
    #[link_name = "PPCMfl2cr"]
    pub unsafe fn mfl2cr() -> u32;

    /// PPCMtl2cr
    ///
    /// Writes L2CR.
    #[doc(alias = "PPCMtl2cr")]
    #[link_name = "PPCMtl2cr"]
    pub unsafe fn mtl2cr(value: u32);

    /// PPCMfdec
    ///
    /// Reads DEC.
    #[doc(alias = "PPCMfdec")]
    #[link_name = "PPCMfdec"]
    pub unsafe fn mfdec() -> u32;

    /// PPCMtdec
    ///
    /// Writes DEC.
    #[doc(alias = "PPCMtdec")]
    #[link_name = "PPCMtdec"]
    pub unsafe fn mtdec(value: u32);

    /// PPCMfmmcr0
    ///
    /// Reads MMCR0.
    #[doc(alias = "PPCMfmmcr0")]
    #[link_name = "PPCMfmmcr0"]
    pub unsafe fn mfmmcr0() -> u32;

    /// PPCMtmmcr0
    ///
    /// Writes MMCR0.
    #[doc(alias = "PPCMtmmcr0")]
    #[link_name = "PPCMtmmcr0"]
    pub unsafe fn mtmmcr0(value: u32);

    /// PPCMfmmcr1
    ///
    /// Reads MMCR1.
    #[doc(alias = "PPCMfmmcr1")]
    #[link_name = "PPCMfmmcr1"]
    pub unsafe fn mfmmcr1() -> u32;

    /// PPCMtmmcr1
    ///
    /// Writes MMCR1.
    #[doc(alias = "PPCMtmmcr1")]
    #[link_name = "PPCMtmmcr1"]
    pub unsafe fn mtmmcr1(value: u32);

    /// PPCMfpmc1
    ///
    /// Reads PMC1.
    #[doc(alias = "PPCMfpmc1")]
    #[link_name = "PPCMfpmc1"]
    pub unsafe fn mfpmc1() -> u32;

    /// PPCMtpmc1
    ///
    /// Writes PMC1.
    #[doc(alias = "PPCMtpmc1")]
    #[link_name = "PPCMtpmc1"]
    pub unsafe fn mtpmc1(value: u32);

    /// PPCMfpmc2
    ///
    /// Reads PMC2.
    #[doc(alias = "PPCMfpmc2")]
    #[link_name = "PPCMfpmc2"]
    pub unsafe fn mfpmc2() -> u32;

    /// PPCMtpmc2
    ///
    /// Writes PMC2.
    #[doc(alias = "PPCMtpmc2")]
    #[link_name = "PPCMtpmc2"]
    pub unsafe fn mtpmc2(value: u32);

    /// PPCMfpmc3
    ///
    /// Reads PMC3.
    #[doc(alias = "PPCMfpmc3")]
    #[link_name = "PPCMfpmc3"]
    pub unsafe fn mfpmc3() -> u32;

    /// PPCMtpmc3
    ///
    /// Writes PMC3.
    #[doc(alias = "PPCMtpmc3")]
    #[link_name = "PPCMtpmc3"]
    pub unsafe fn mtpmc3(value: u32);

    /// PPCMfpmc4
    ///
    /// Reads PMC4.
    #[doc(alias = "PPCMfpmc4")]
    #[link_name = "PPCMfpmc4"]
    pub unsafe fn mfpmc4() -> u32;

    /// PPCMtpmc4
    ///
    /// Writes PMC4.
    #[doc(alias = "PPCMtpmc4")]
    #[link_name = "PPCMtpmc4"]
    pub unsafe fn mtpmc4(value: u32);

    /// PPCMfsia
    ///
    /// Reads SIA.
    #[doc(alias = "PPCMfsia")]
    #[link_name = "PPCMfsia"]
    pub unsafe fn mfsia() -> u32;

    /// PPCMtsia
    ///
    /// Writes SIA.
    #[doc(alias = "PPCMtsia")]
    #[link_name = "PPCMtsia"]
    pub unsafe fn mtsia(value: u32);

    /// PPCMfpvr
    ///
    /// Reads PVR.
    #[doc(alias = "PPCMfpvr")]
    #[link_name = "PPCMfpvr"]
    pub unsafe fn mfpvr() -> u32;

    /// PPCMffpscr
    ///
    /// Reads FPSCR.
    #[doc(alias = "PPCMffpscr")]
    #[link_name = "PPCMffpscr"]
    pub unsafe fn mffpscr() -> u32;

    /// PPCMtfpscr
    ///
    /// Writes FPSCR.
    #[doc(alias = "PPCMtfpscr")]
    #[link_name = "PPCMtfpscr"]
    pub unsafe fn mtfpscr(value: u32);

    /// PPCMfpir
    ///
    /// Reads PIR.
    #[doc(alias = "PPCMfpir")]
    #[link_name = "PPCMfpir"]
    pub unsafe fn mfpir() -> u32;

    /// PPCMfsprg0
    ///
    /// Reads SPRG0.
    #[doc(alias = "PPCMfsprg0")]
    #[link_name = "PPCMfsprg0"]
    pub unsafe fn mfsprg0() -> u32;

    /// PPCOrMsr
    ///
    /// Sets bits in MSR, returning the previous value.
    #[doc(alias = "PPCOrMsr")]
    #[link_name = "PPCOrMsr"]
    pub unsafe fn or_msr(value: u32) -> u32;

    /// PPCAndMsr
    ///
    /// Masks MSR, returning the previous value.
    #[doc(alias = "PPCAndMsr")]
    #[link_name = "PPCAndMsr"]
    pub unsafe fn and_msr(value: u32) -> u32;

    /// PPCAndCMsr
    ///
    /// Clears bits in MSR, returning the previous value.
    #[doc(alias = "PPCAndCMsr")]
    #[link_name = "PPCAndCMsr"]
    pub unsafe fn andc_msr(value: u32) -> u32;

    /// PPCSync
    #[doc(alias = "PPCSync")]
    #[link_name = "PPCSync"]
    pub unsafe fn sync();

    /// PPCEieio
    #[doc(alias = "PPCEieio")]
    #[link_name = "PPCEieio"]
    pub unsafe fn eieio();

    /// PPCIsync
    #[doc(alias = "PPCIsync")]
    #[link_name = "PPCIsync"]
    pub unsafe fn isync();

    /// PPCHalt
    ///
    /// Halts with a message.
    #[doc(alias = "PPCHalt")]
    #[link_name = "PPCHalt"]
    pub unsafe fn halt(message: *mut ffi::c_char, size: u32);

    /// __PPCHalt
    #[doc(alias = "__PPCHalt")]
    #[link_name = "__PPCHalt"]
    pub unsafe fn halt_raw();

    /// __PPCExit
    #[doc(alias = "__PPCExit")]
    #[link_name = "__PPCExit"]
    pub unsafe fn exit_raw(result: ffi::c_int);

    /// PPCEnableSpeculation
    #[doc(alias = "PPCEnableSpeculation")]
    #[link_name = "PPCEnableSpeculation"]
    pub unsafe fn enable_speculation();

    /// PPCDisableSpeculation
    #[doc(alias = "PPCDisableSpeculation")]
    #[link_name = "PPCDisableSpeculation"]
    pub unsafe fn disable_speculation();

    /// PPCSetFpIEEEMode
    #[doc(alias = "PPCSetFpIEEEMode")]
    #[link_name = "PPCSetFpIEEEMode"]
    pub unsafe fn set_fp_ieee_mode();

    /// PPCSetFpNonIEEEMode
    #[doc(alias = "PPCSetFpNonIEEEMode")]
    #[link_name = "PPCSetFpNonIEEEMode"]
    pub unsafe fn set_fp_non_ieee_mode();

    /// PMBegin
    ///
    /// Starts counting cycles, instructions and L1 misses.
    #[doc(alias = "PMBegin")]
    #[link_name = "PMBegin"]
    pub unsafe fn pm_begin();

    /// PMEnd
    #[doc(alias = "PMEnd")]
    #[link_name = "PMEnd"]
    pub unsafe fn pm_end();

    /// PMCycles
    #[doc(alias = "PMCycles")]
    #[link_name = "PMCycles"]
    pub unsafe fn pm_cycles() -> u32;

    /// PMInstructions
    #[doc(alias = "PMInstructions")]
    #[link_name = "PMInstructions"]
    pub unsafe fn pm_instructions() -> u32;

    /// PML1FetchMisses
    #[doc(alias = "PML1FetchMisses")]
    #[link_name = "PML1FetchMisses"]
    pub unsafe fn pm_l1_fetch_misses() -> u32;

    /// PML1MissCycles
    #[doc(alias = "PML1MissCycles")]
    #[link_name = "PML1MissCycles"]
    pub unsafe fn pm_l1_miss_cycles() -> u32;
}
