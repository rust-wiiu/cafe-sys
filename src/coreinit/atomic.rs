//! Atomic
//!
//! Atomic operations on 32- and 64-bit values, coherent across cores.

use crate::ffi;

unsafe extern "C" {
    /// OSAddAtomic
    ///
    /// Returns the previous value.
    #[doc(alias = "OSAddAtomic")]
    #[link_name = "OSAddAtomic"]
    pub unsafe fn add(ptr: *mut i32, value: i32) -> i32;

    /// OSOrAtomic
    ///
    /// Returns the previous value.
    #[doc(alias = "OSOrAtomic")]
    #[link_name = "OSOrAtomic"]
    pub unsafe fn or(ptr: *mut u32, mask: u32) -> u32;

    /// OSAndAtomic
    ///
    /// Returns the previous value.
    #[doc(alias = "OSAndAtomic")]
    #[link_name = "OSAndAtomic"]
    pub unsafe fn and(ptr: *mut u32, mask: u32) -> u32;

    /// OSXorAtomic
    ///
    /// Returns the previous value.
    #[doc(alias = "OSXorAtomic")]
    #[link_name = "OSXorAtomic"]
    pub unsafe fn xor(ptr: *mut u32, mask: u32) -> u32;

    /// OSSwapAtomic
    ///
    /// Returns the previous value.
    #[doc(alias = "OSSwapAtomic")]
    #[link_name = "OSSwapAtomic"]
    pub unsafe fn swap(ptr: *mut u32, value: u32) -> u32;

    /// OSCompareAndSwapAtomic
    ///
    /// Stores `value` if the current value equals `compare`.
    #[doc(alias = "OSCompareAndSwapAtomic")]
    #[link_name = "OSCompareAndSwapAtomic"]
    pub unsafe fn compare_and_swap(ptr: *mut u32, compare: u32, value: u32) -> ffi::c_bool;

    /// OSCompareAndSwapAtomicEx
    ///
    /// Same as [compare_and_swap], also returning the previous value.
    #[doc(alias = "OSCompareAndSwapAtomicEx")]
    #[link_name = "OSCompareAndSwapAtomicEx"]
    pub unsafe fn compare_and_swap_ex(
        ptr: *mut u32,
        compare: u32,
        value: u32,
        old: *mut u32,
    ) -> ffi::c_bool;

    /// OSTestAndSetAtomic
    ///
    /// Sets bit `bit`, returning its previous state.
    #[doc(alias = "OSTestAndSetAtomic")]
    #[link_name = "OSTestAndSetAtomic"]
    pub unsafe fn test_and_set(ptr: *mut u32, bit: u32) -> ffi::c_bool;

    /// OSTestAndClearAtomic
    ///
    /// Clears bit `bit`, returning its previous state.
    #[doc(alias = "OSTestAndClearAtomic")]
    #[link_name = "OSTestAndClearAtomic"]
    pub unsafe fn test_and_clear(ptr: *mut u32, bit: u32) -> ffi::c_bool;

    /// OSGetAtomic64
    #[doc(alias = "OSGetAtomic64")]
    #[link_name = "OSGetAtomic64"]
    pub unsafe fn get_64(ptr: *mut u64) -> u64;

    /// OSSetAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSSetAtomic64")]
    #[link_name = "OSSetAtomic64"]
    pub unsafe fn set_64(ptr: *mut u64, value: u64) -> u64;

    /// OSAddAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSAddAtomic64")]
    #[link_name = "OSAddAtomic64"]
    pub unsafe fn add_64(ptr: *mut i64, value: i64) -> i64;

    /// OSOrAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSOrAtomic64")]
    #[link_name = "OSOrAtomic64"]
    pub unsafe fn or_64(ptr: *mut u64, mask: u64) -> u64;

    /// OSAndAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSAndAtomic64")]
    #[link_name = "OSAndAtomic64"]
    pub unsafe fn and_64(ptr: *mut u64, mask: u64) -> u64;

    /// OSXorAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSXorAtomic64")]
    #[link_name = "OSXorAtomic64"]
    pub unsafe fn xor_64(ptr: *mut u64, mask: u64) -> u64;

    /// OSSwapAtomic64
    ///
    /// Returns the previous value.
    #[doc(alias = "OSSwapAtomic64")]
    #[link_name = "OSSwapAtomic64"]
    pub unsafe fn swap_64(ptr: *mut u64, value: u64) -> u64;

    /// OSCompareAndSwapAtomic64
    ///
    /// Stores `value` if the current value equals `compare`.
    #[doc(alias = "OSCompareAndSwapAtomic64")]
    #[link_name = "OSCompareAndSwapAtomic64"]
    pub unsafe fn compare_and_swap_64(ptr: *mut u64, compare: u64, value: u64) -> ffi::c_bool;

    /// OSCompareAndSwapAtomicEx64
    ///
    /// Same as [compare_and_swap_64], also returning the previous value.
    #[doc(alias = "OSCompareAndSwapAtomicEx64")]
    #[link_name = "OSCompareAndSwapAtomicEx64"]
    pub unsafe fn compare_and_swap_ex_64(
        ptr: *mut u64,
        compare: u64,
        value: u64,
        old: *mut u64,
    ) -> ffi::c_bool;

    /// OSTestAndSetAtomic64
    ///
    /// Sets bit `bit`, returning its previous state.
    #[doc(alias = "OSTestAndSetAtomic64")]
    #[link_name = "OSTestAndSetAtomic64"]
    pub unsafe fn test_and_set_64(ptr: *mut u64, bit: u32) -> ffi::c_bool;

    /// OSTestAndClearAtomic64
    ///
    /// Clears bit `bit`, returning its previous state.
    #[doc(alias = "OSTestAndClearAtomic64")]
    #[link_name = "OSTestAndClearAtomic64"]
    pub unsafe fn test_and_clear_64(ptr: *mut u64, bit: u32) -> ffi::c_bool;
}
