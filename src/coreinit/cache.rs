//! Cache
//!
//! Data and instruction cache maintenance, and the locked cache (scratchpad) with its DMA engine.

use crate::ffi;

pub const LC_MAX_BYTES: u32 = 0x4000;
pub const LC_MIN_ALLOC: u32 = 512;
/// Maximum blocks of 32 bytes per DMA transfer.
pub const LC_MAX_DMA_BLOCKS: u32 = 128;

unsafe extern "C" {
    /// DCInvalidateRange
    ///
    /// Discards cached data without writing it back.
    #[doc(alias = "DCInvalidateRange")]
    #[link_name = "DCInvalidateRange"]
    pub unsafe fn dc_invalidate_range(addr: *mut ffi::c_void, size: u32);

    /// DCFlushRange
    ///
    /// Writes back and invalidates.
    #[doc(alias = "DCFlushRange")]
    #[link_name = "DCFlushRange"]
    pub unsafe fn dc_flush_range(addr: *const ffi::c_void, size: u32);

    /// DCStoreRange
    ///
    /// Writes back, keeping the data cached.
    #[doc(alias = "DCStoreRange")]
    #[link_name = "DCStoreRange"]
    pub unsafe fn dc_store_range(addr: *const ffi::c_void, size: u32);

    /// DCFlushRangeNoSync
    ///
    /// Same as [dc_flush_range] without waiting for completion.
    #[doc(alias = "DCFlushRangeNoSync")]
    #[link_name = "DCFlushRangeNoSync"]
    pub unsafe fn dc_flush_range_no_sync(addr: *const ffi::c_void, size: u32);

    /// DCStoreRangeNoSync
    ///
    /// Same as [dc_store_range] without waiting for completion.
    #[doc(alias = "DCStoreRangeNoSync")]
    #[link_name = "DCStoreRangeNoSync"]
    pub unsafe fn dc_store_range_no_sync(addr: *const ffi::c_void, size: u32);

    /// DCZeroRange
    ///
    /// Zeroes cache lines without reading memory first.
    #[doc(alias = "DCZeroRange")]
    #[link_name = "DCZeroRange"]
    pub unsafe fn dc_zero_range(addr: *mut ffi::c_void, size: u32);

    /// DCTouchRange
    ///
    /// Prefetches into the cache.
    #[doc(alias = "DCTouchRange")]
    #[link_name = "DCTouchRange"]
    pub unsafe fn dc_touch_range(addr: *const ffi::c_void, size: u32);

    /// DCCoreFlushAll
    ///
    /// Flushes the whole data cache of the current core, using `buffer` to evict lines.
    #[doc(alias = "DCCoreFlushAll")]
    #[link_name = "DCCoreFlushAll"]
    pub unsafe fn dc_core_flush_all(buffer: *mut ffi::c_void, size: u32) -> ffi::c_bool;

    /// ICInvalidateRange
    #[doc(alias = "ICInvalidateRange")]
    #[link_name = "ICInvalidateRange"]
    pub unsafe fn ic_invalidate_range(addr: *mut ffi::c_void, size: u32);

    /// OSMemoryBarrier
    #[doc(alias = "OSMemoryBarrier")]
    #[link_name = "OSMemoryBarrier"]
    pub unsafe fn memory_barrier();

    /// OSIsAddressRangeDCValid
    ///
    /// Whether the range is cacheable.
    #[doc(alias = "OSIsAddressRangeDCValid")]
    #[link_name = "OSIsAddressRangeDCValid"]
    pub unsafe fn is_address_range_dc_valid(addr: *const ffi::c_void, size: u32) -> ffi::c_bool;

    /// LCGetMaxSize
    #[doc(alias = "LCGetMaxSize")]
    #[link_name = "LCGetMaxSize"]
    pub unsafe fn lc_get_max_size() -> u32;

    /// LCGetUnallocated
    #[doc(alias = "LCGetUnallocated")]
    #[link_name = "LCGetUnallocated"]
    pub unsafe fn lc_get_unallocated() -> u32;

    /// LCGetAllocatableSize
    #[doc(alias = "LCGetAllocatableSize")]
    #[link_name = "LCGetAllocatableSize"]
    pub unsafe fn lc_get_allocatable_size() -> u32;

    /// LCAlloc
    ///
    /// At least [LC_MIN_ALLOC] bytes.
    #[doc(alias = "LCAlloc")]
    #[link_name = "LCAlloc"]
    pub unsafe fn lc_alloc(size: u32) -> *mut ffi::c_void;

    /// LCDealloc
    #[doc(alias = "LCDealloc")]
    #[link_name = "LCDealloc"]
    pub unsafe fn lc_dealloc(addr: *mut ffi::c_void);

    /// LCEnableDMA
    #[doc(alias = "LCEnableDMA")]
    #[link_name = "LCEnableDMA"]
    pub unsafe fn lc_enable_dma() -> ffi::c_bool;

    /// LCDisableDMA
    #[doc(alias = "LCDisableDMA")]
    #[link_name = "LCDisableDMA"]
    pub unsafe fn lc_disable_dma();

    /// LCIsDMAEnabled
    #[doc(alias = "LCIsDMAEnabled")]
    #[link_name = "LCIsDMAEnabled"]
    pub unsafe fn lc_is_dma_enabled() -> ffi::c_bool;

    /// LCLoadDMABlocks
    ///
    /// Queues a copy of `num_blocks` 32-byte blocks from memory into the locked cache.
    #[doc(alias = "LCLoadDMABlocks")]
    #[link_name = "LCLoadDMABlocks"]
    pub unsafe fn lc_load_dma_blocks(
        lc_dst: *mut ffi::c_void,
        mem_src: *const ffi::c_void,
        num_blocks: u32,
    );

    /// LCStoreDMABlocks
    ///
    /// Queues a copy of `num_blocks` 32-byte blocks from the locked cache into memory.
    #[doc(alias = "LCStoreDMABlocks")]
    #[link_name = "LCStoreDMABlocks"]
    pub unsafe fn lc_store_dma_blocks(
        mem_dst: *mut ffi::c_void,
        lc_src: *const ffi::c_void,
        num_blocks: u32,
    );

    /// LCGetDMAQueueLength
    #[doc(alias = "LCGetDMAQueueLength")]
    #[link_name = "LCGetDMAQueueLength"]
    pub unsafe fn lc_get_dma_queue_length() -> u32;

    /// LCWaitDMAQueue
    ///
    /// Waits until at most `length` transfers are queued.
    #[doc(alias = "LCWaitDMAQueue")]
    #[link_name = "LCWaitDMAQueue"]
    pub unsafe fn lc_wait_dma_queue(length: u32);
}
