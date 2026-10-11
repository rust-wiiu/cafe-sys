//! Intrusive object lists (MEMList)

use crate::ffi;

/// MEMMemoryLink
///
/// Embedded in each object of a [List].
#[doc(alias = "MEMMemoryLink", alias = "MEMLink")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Link {
    pub prev: *mut ffi::c_void,
    pub next: *mut ffi::c_void,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        }
    }
}

/// MEMMemoryList
#[doc(alias = "MEMMemoryList", alias = "MEMList")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct List {
    pub head: *mut ffi::c_void,
    pub tail: *mut ffi::c_void,
    pub count: u16,
    /// Offset of the [Link] within the objects.
    pub offset_to_memory_link: u16,
}

impl Default for List {
    fn default() -> Self {
        Self {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            count: 0,
            offset_to_memory_link: 0,
        }
    }
}

unsafe extern "C" {
    /// MEMInitList
    ///
    /// `offset_to_memory_link` is the offset of the [Link] field within the objects.
    #[doc(alias = "MEMInitList")]
    #[link_name = "MEMInitList"]
    pub unsafe fn init(list: *mut List, offset_to_memory_link: u16);

    /// MEMAppendListObject
    #[doc(alias = "MEMAppendListObject")]
    #[link_name = "MEMAppendListObject"]
    pub unsafe fn append(list: *mut List, object: *mut ffi::c_void);

    /// MEMPrependListObject
    #[doc(alias = "MEMPrependListObject")]
    #[link_name = "MEMPrependListObject"]
    pub unsafe fn prepend(list: *mut List, object: *mut ffi::c_void);

    /// MEMInsertListObject
    ///
    /// Inserts `object` before `before`, or at the end if `before` is null.
    #[doc(alias = "MEMInsertListObject")]
    #[link_name = "MEMInsertListObject"]
    pub unsafe fn insert(list: *mut List, before: *mut ffi::c_void, object: *mut ffi::c_void);

    /// MEMRemoveListObject
    #[doc(alias = "MEMRemoveListObject")]
    #[link_name = "MEMRemoveListObject"]
    pub unsafe fn remove(list: *mut List, object: *mut ffi::c_void);

    /// MEMGetNextListObject
    ///
    /// Returns the head if `object` is null.
    #[doc(alias = "MEMGetNextListObject")]
    #[link_name = "MEMGetNextListObject"]
    pub unsafe fn get_next(list: *mut List, object: *mut ffi::c_void) -> *mut ffi::c_void;

    /// MEMGetPrevListObject
    ///
    /// Returns the tail if `object` is null.
    #[doc(alias = "MEMGetPrevListObject")]
    #[link_name = "MEMGetPrevListObject"]
    pub unsafe fn get_prev(list: *mut List, object: *mut ffi::c_void) -> *mut ffi::c_void;

    /// MEMGetNthListObject
    #[doc(alias = "MEMGetNthListObject")]
    #[link_name = "MEMGetNthListObject"]
    pub unsafe fn get_nth(list: *mut List, index: u16) -> *mut ffi::c_void;
}
