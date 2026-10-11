//! Performance monitor
//!
//! Hardware performance counters (PMC1-4) controlled by MMCR0/1.

/// OSPerfMonArg
///
/// Registers to write in [set_performance_monitor].
#[doc(alias = "OSPerfMonArg")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PerfMonArg(pub u32);

impl PerfMonArg {
    pub const MMCR0: Self = Self(1 << 0);
    pub const MMCR1: Self = Self(1 << 1);
    pub const PMC1: Self = Self(1 << 2);
    pub const PMC2: Self = Self(1 << 3);
    pub const PMC3: Self = Self(1 << 4);
    pub const PMC4: Self = Self(1 << 5);
}

impl core::ops::BitOr for PerfMonArg {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSPerfMonMMCR0Flags
///
/// Events counted by PMC1 and PMC2.
#[doc(alias = "OSPerfMonMMCR0Flags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mmcr0Flags(pub u32);

impl Mmcr0Flags {
    pub const PMC1_CURRENT: Self = Self(0b0000000 << 6);
    pub const PMC1_CPU_CYCLES: Self = Self(0b0000001 << 6);
    pub const PMC1_INSTRUCTIONS_COMPLETED: Self = Self(0b0000010 << 6);
    pub const PMC1_TBL_RISING_TRANSITIONS: Self = Self(0b0000011 << 6);
    pub const PMC1_INSTRUCTIONS_DISPATCHED: Self = Self(0b0000100 << 6);
    pub const PMC1_EIEIO_INSTRUCTIONS_COMPLETED: Self = Self(0b0000101 << 6);
    pub const PMC1_ITLB_SEARCH_CYCLES: Self = Self(0b0000110 << 6);
    pub const PMC1_L2_HITS: Self = Self(0b0000111 << 6);
    pub const PMC1_INSTRUCTIONS_EA_DELIVERED: Self = Self(0b0001000 << 6);
    pub const PMC1_INSTRUCTIONS_COMPLETED_MATCHES_IABR: Self = Self(0b0001001 << 6);
    pub const PMC1_SLOW_L1_MISSES: Self = Self(0b0001010 << 6);
    pub const PMC1_UNRESOLVED_BRANCHES: Self = Self(0b0001011 << 6);
    pub const PMC1_UNRESOLVED_STALL_CYCLES: Self = Self(0b0001100 << 6);
    pub const PMC1_L1_SHARED_STORES: Self = Self(0b0001110 << 6);
    pub const PMC1_L2_SHARED_INTERVENTIONS: Self = Self(0b0001111 << 6);
    pub const PMC1_CACHE_PARADOXES: Self = Self(0b0010000 << 6);
    pub const PMC1_CIU_LOAD_REQUESTS: Self = Self(0b0010100 << 6);
    pub const PMC1_BIU_ADDRESS_ONLY_REQUESTS: Self = Self(0b0010101 << 6);
    pub const PMC1_CIU_PARADOXES: Self = Self(0b0010110 << 6);
    pub const PMC1_60XE_BUS_DATA_BEATS: Self = Self(0b0010111 << 6);
    pub const PMC2_CURRENT: Self = Self(0b000000);
    pub const PMC2_CPU_CYCLES: Self = Self(0b000001);
    pub const PMC2_INSTRUCTIONS_COMPLETED: Self = Self(0b000010);
    pub const PMC2_TBL_RISING_TRANSITIONS: Self = Self(0b000011);
    pub const PMC2_INSTRUCTIONS_DISPATCHED: Self = Self(0b000100);
    pub const PMC2_L1_ICACHE_MISSES: Self = Self(0b000101);
    pub const PMC2_ITLB_MISSES: Self = Self(0b000110);
    pub const PMC2_L2_INSTRUCTION_MISSES: Self = Self(0b000111);
    pub const PMC2_PRED_BRANCHES_NOT_TAKEN: Self = Self(0b001000);
    pub const PMC2_RESERVED_LOADS: Self = Self(0b001010);
    pub const PMC2_LOADS_AND_STORES: Self = Self(0b001011);
    pub const PMC2_CACHE_SNOOPS: Self = Self(0b001100);
    pub const PMC2_L1_TO_L2_CASTOUTS: Self = Self(0b001101);
    pub const PMC2_SYSTEM_UNIT_INSTRUCTIONS: Self = Self(0b001110);
    pub const PMC2_L1_INSTRUCTION_MISS_CYCLES: Self = Self(0b001111);
    pub const PMC2_FIRST_SPECULATIVE_BRANCH_RESOLVES: Self = Self(0b010000);
    pub const PMC2_L2_SHARED_STORES: Self = Self(0b010001);
    pub const PMC2_L1_SHARED_INTERVENTIONS: Self = Self(0b010010);
    pub const PMC2_CIU_STORE_REQUESTS: Self = Self(0b010100);
    pub const PMC2_SLOW_OUTSTANDING_BIU_TRANSACTIONS: Self = Self(0b010101);
    pub const PMC2_CIU_MODIFIED_INTERVENTIONS: Self = Self(0b010110);
}

impl core::ops::BitOr for Mmcr0Flags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// OSPerfMonMMCR1Flags
///
/// Events counted by PMC3 and PMC4.
#[doc(alias = "OSPerfMonMMCR1Flags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mmcr1Flags(pub u32);

impl Mmcr1Flags {
    pub const PMC3_CURRENT: Self = Self(0b00000 << 27);
    pub const PMC3_CPU_CYCLES: Self = Self(0b00001 << 27);
    pub const PMC3_INSTRUCTIONS_COMPLETED: Self = Self(0b00010 << 27);
    pub const PMC3_TBL_RISING_TRANSITIONS: Self = Self(0b00011 << 27);
    pub const PMC3_INSTRUCTIONS_DISPATCHED: Self = Self(0b00100 << 27);
    pub const PMC3_L1_DCACHE_MISSES: Self = Self(0b00101 << 27);
    pub const PMC3_DTLB_MISSES: Self = Self(0b00110 << 27);
    pub const PMC3_L2_DATA_MISSES: Self = Self(0b00111 << 27);
    pub const PMC3_PRED_BRANCHES_TAKEN: Self = Self(0b01000 << 27);
    pub const PMC3_COND_STORES_COMPLETED: Self = Self(0b01010 << 27);
    pub const PMC3_FPU_INSTRUCTIONS_COMPLETED: Self = Self(0b01011 << 27);
    pub const PMC3_L2_CASTOUTS_BY_SNOOPS: Self = Self(0b01100 << 27);
    pub const PMC3_L2_CACHE_OPERATIONS: Self = Self(0b01101 << 27);
    pub const PMC3_L1_LOAD_MISS_CYCLES: Self = Self(0b01111 << 27);
    pub const PMC3_SECOND_SPECULATIVE_BRANCH_RESOLVES: Self = Self(0b10000 << 27);
    pub const PMC3_BPU_STALL_LR_CR_CYCLES: Self = Self(0b10001 << 27);
    pub const PMC3_L1_MODIFIED_INTERVENTIONS: Self = Self(0b10010 << 27);
    pub const PMC3_ICBI_SNOOPS: Self = Self(0b10011 << 27);
    pub const PMC3_CIU_ADDRESS_ONLY_REQUESTS: Self = Self(0b10100 << 27);
    pub const PMC3_BIU_LOAD_REQUESTS: Self = Self(0b10101 << 27);
    pub const PMC3_CIU_SHARED_INTERVENTIONS: Self = Self(0b10110 << 27);
    pub const PMC4_CURRENT: Self = Self(0b00000 << 22);
    pub const PMC4_CPU_CYCLES: Self = Self(0b00001 << 22);
    pub const PMC4_INSTRUCTIONS_COMPLETED: Self = Self(0b00010 << 22);
    pub const PMC4_TBL_RISING_TRANSITIONS: Self = Self(0b00011 << 22);
    pub const PMC4_INSTRUCTIONS_DISPATCHED: Self = Self(0b00100 << 22);
    pub const PMC4_L2_CASTOUTS: Self = Self(0b00101 << 22);
    pub const PMC4_DTLB_SEARCH_CYCLES: Self = Self(0b00110 << 22);
    pub const PMC4_BRANCHES_MISPREDICTED: Self = Self(0b01000 << 22);
    pub const PMC4_INTACT_COND_STORES_COMPLETED: Self = Self(0b01010 << 22);
    pub const PMC4_SYNC_INSTRUCTIONS_COMPLETED: Self = Self(0b01011 << 22);
    pub const PMC4_SNOOP_RETRIES: Self = Self(0b01100 << 22);
    pub const PMC4_INTEGER_OPERATIONS: Self = Self(0b01101 << 22);
    pub const PMC4_BPU_STALL_TWO_BRANCHES_CYCLES: Self = Self(0b01110 << 22);
    pub const PMC4_L2_MODIFIED_INTERVENTIONS: Self = Self(0b10000 << 22);
    pub const PMC4_TLBIE_SNOOPS: Self = Self(0b10001 << 22);
    pub const PMC4_L2_BANK_REFRESH_OVERFLOWS: Self = Self(0b10010 << 22);
    pub const PMC4_CIU_ARTRY_COUNT: Self = Self(0b10100 << 22);
    pub const PMC4_BIU_STORE_REQUESTS: Self = Self(0b10101 << 22);
    pub const PMC4_CIU_TWO_CORE_SHARED_INTERVENTIONS: Self = Self(0b10110 << 22);
}

impl core::ops::BitOr for Mmcr1Flags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

unsafe extern "C" {
    /// OSSetPerformanceMonitor
    ///
    /// Writes the registers selected by `mask` on the current core.
    #[doc(alias = "OSSetPerformanceMonitor")]
    #[link_name = "OSSetPerformanceMonitor"]
    pub unsafe fn set_performance_monitor(
        mask: PerfMonArg,
        mmcr0: Mmcr0Flags,
        mmcr1: Mmcr1Flags,
        pmc1: u32,
        pmc2: u32,
        pmc3: u32,
        pmc4: u32,
    );
}
