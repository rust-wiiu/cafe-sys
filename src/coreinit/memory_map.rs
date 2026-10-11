//! Memory map
//!
//! Address translation and mapping of physical memory into the virtual address space.

use crate::ffi;

/// OS_PAGE_SIZE
#[doc(alias = "OS_PAGE_SIZE")]
pub const PAGE_SIZE: u32 = 128 * 1024;

/// OSMemoryMapMode
#[doc(alias = "OSMemoryMapMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapMode(pub u32);

impl MapMode {
    pub const INVALID: Self = Self(0);
    pub const READ_ONLY: Self = Self(1);
    pub const READ_WRITE: Self = Self(2);
    pub const FREE: Self = Self(3);
    pub const ALLOCATED: Self = Self(4);
}

unsafe extern "C" {
    /// OSEffectiveToPhysical
    #[doc(alias = "OSEffectiveToPhysical")]
    #[link_name = "OSEffectiveToPhysical"]
    pub unsafe fn effective_to_physical(virtual_address: u32) -> u32;

    /// __OSPhysicalToEffectiveCached
    #[doc(alias = "__OSPhysicalToEffectiveCached")]
    #[link_name = "__OSPhysicalToEffectiveCached"]
    pub unsafe fn physical_to_effective_cached(physical_address: u32) -> u32;

    /// __OSPhysicalToEffectiveUncached
    #[doc(alias = "__OSPhysicalToEffectiveUncached")]
    #[link_name = "__OSPhysicalToEffectiveUncached"]
    pub unsafe fn physical_to_effective_uncached(physical_address: u32) -> u32;

    /// OSIsAddressValid
    #[doc(alias = "OSIsAddressValid")]
    #[link_name = "OSIsAddressValid"]
    pub unsafe fn is_address_valid(address: *const ffi::c_void) -> ffi::c_bool;

    /// __OSValidateAddressSpaceRange
    #[doc(alias = "__OSValidateAddressSpaceRange")]
    #[link_name = "__OSValidateAddressSpaceRange"]
    pub unsafe fn validate_address_space_range(
        unused: i32,
        virtual_address: u32,
        size: u32,
    ) -> ffi::c_bool;

    /// OSAllocVirtAddr
    ///
    /// Reserves virtual address space for [map_memory]. `virtual_address` 0 picks any address. Returns 0 on failure.
    #[doc(alias = "OSAllocVirtAddr")]
    #[link_name = "OSAllocVirtAddr"]
    pub unsafe fn alloc_virt_addr(virtual_address: u32, size: u32, align: u32) -> u32;

    /// OSFreeVirtAddr
    #[doc(alias = "OSFreeVirtAddr")]
    #[link_name = "OSFreeVirtAddr"]
    pub unsafe fn free_virt_addr(virtual_address: u32, size: u32) -> ffi::c_bool;

    /// OSQueryVirtAddr
    #[doc(alias = "OSQueryVirtAddr")]
    #[link_name = "OSQueryVirtAddr"]
    pub unsafe fn query_virt_addr(virtual_address: u32) -> MapMode;

    /// OSMapMemory
    #[doc(alias = "OSMapMemory")]
    #[link_name = "OSMapMemory"]
    pub unsafe fn map_memory(
        virtual_address: u32,
        physical_address: u32,
        size: u32,
        mode: MapMode,
    ) -> ffi::c_bool;

    /// OSUnmapMemory
    #[doc(alias = "OSUnmapMemory")]
    #[link_name = "OSUnmapMemory"]
    pub unsafe fn unmap_memory(virtual_address: u32, size: u32) -> ffi::c_bool;

    /// OSGetMapVirtAddrRange
    ///
    /// Virtual address range available to [alloc_virt_addr].
    #[doc(alias = "OSGetMapVirtAddrRange")]
    #[link_name = "OSGetMapVirtAddrRange"]
    pub unsafe fn get_map_virt_addr_range(virtual_address: *mut u32, size: *mut u32);

    /// OSGetAvailPhysAddrRange
    #[doc(alias = "OSGetAvailPhysAddrRange")]
    #[link_name = "OSGetAvailPhysAddrRange"]
    pub unsafe fn get_avail_phys_addr_range(physical_address: *mut u32, size: *mut u32);

    /// OSGetDataPhysAddrRange
    #[doc(alias = "OSGetDataPhysAddrRange")]
    #[link_name = "OSGetDataPhysAddrRange"]
    pub unsafe fn get_data_phys_addr_range(physical_address: *mut u32, size: *mut u32);
}
