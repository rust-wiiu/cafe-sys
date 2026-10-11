//! Storage / USB Periferal Management (SPM)
//!
//! [StorageIndex] is a C++ class, so functions taking it by value receive a pointer to a copy.

use crate::ffi;

/// SPM error code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
}

/// StorageType
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageType(pub u32);

impl StorageType {
    pub const RAW: Self = Self(0);
    pub const WFS: Self = Self(1);
}

/// StorageIndex
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageIndex(pub u64);

/// VolumeId
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VolumeId {
    pub id: [ffi::c_char; 16],
}

/// StorageListItem
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageListItem {
    pub index: StorageIndex,
    _unk: ffi::unknown<0x4>,
    pub kind: StorageType,
}

/// StorageInfo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageInfo {
    /// Mount path, e.g. `/vol/storage_usb01`.
    pub path: [ffi::c_char; 0x280],
    /// Connection type, e.g. `usb`.
    pub connection_type: [ffi::c_char; 8],
    /// Format, e.g. `raw` or `wfs`.
    pub format: [ffi::c_char; 8],
    _unk: ffi::unknown<0x4>,
    pub kind: StorageType,
    pub volume_id: VolumeId,
}

unsafe extern "C" {
    /// Initialize
    #[doc(alias = "Initialize")]
    #[link_name = "Initialize__Q2_2nn3spmFv"]
    pub unsafe fn init();

    /// Finalize
    #[doc(alias = "Finalize")]
    #[link_name = "Finalize__Q2_2nn3spmFv"]
    pub unsafe fn deinit();

    /// SetAutoFatal
    #[doc(alias = "SetAutoFatal")]
    #[link_name = "SetAutoFatal__Q2_2nn3spmFb"]
    pub unsafe fn set_auto_fatal(enabled: bool);

    /// SetExtendedStorage
    ///
    /// Selects the drive used as extended storage.
    #[doc(alias = "SetExtendedStorage")]
    #[link_name = "SetExtendedStorage__Q2_2nn3spmFQ3_2nn3spm12StorageIndex"]
    pub unsafe fn set_extended_storage(index: *const StorageIndex) -> Error;

    /// SetDefaultExtendedStorageVolumeId
    #[doc(alias = "SetDefaultExtendedStorageVolumeId")]
    #[link_name = "SetDefaultExtendedStorageVolumeId__Q2_2nn3spmFRCQ3_2nn3spm8VolumeId"]
    pub unsafe fn set_default_extended_storage_volume_id(id: *const VolumeId);

    /// GetDefaultExtendedStorageVolumeId
    ///
    /// The C++ function returns the [VolumeId] by value, which the ABI passes as a hidden pointer.
    #[doc(alias = "GetDefaultExtendedStorageVolumeId")]
    #[link_name = "GetDefaultExtendedStorageVolumeId__Q2_2nn3spmFv"]
    pub unsafe fn get_default_extended_storage_volume_id(id: *mut VolumeId);

    /// GetPreviousExtendedStorageVolumeId
    #[doc(alias = "GetPreviousExtendedStorageVolumeId")]
    #[link_name = "GetPreviousExtendedStorageVolumeId__Q2_2nn3spmFPbPQ3_2nn3spm8VolumeId"]
    pub unsafe fn get_previous_extended_storage_volume_id(
        unk: *mut bool,
        id: *mut VolumeId,
    ) -> bool;

    /// FindStorageByVolumeId
    #[doc(alias = "FindStorageByVolumeId")]
    #[link_name = "FindStorageByVolumeId__Q2_2nn3spmFPQ3_2nn3spm12StorageIndexRCQ3_2nn3spm8VolumeId"]
    pub unsafe fn find_storage_by_volume_id(index: *mut StorageIndex, id: *const VolumeId) -> bool;

    /// GetExtendedStorageIndex
    #[doc(alias = "GetExtendedStorageIndex")]
    #[link_name = "GetExtendedStorageIndex__Q2_2nn3spmFPQ3_2nn3spm12StorageIndex"]
    pub unsafe fn get_extended_storage_index(index: *mut StorageIndex) -> Error;

    /// GetStorageList
    #[doc(alias = "GetStorageList")]
    #[link_name = "GetStorageList__Q2_2nn3spmFPQ3_2nn3spm15StorageListItemUi"]
    pub unsafe fn get_storage_list(items: *mut StorageListItem, max_items: u32) -> i32;

    /// GetStorageInfo
    #[doc(alias = "GetStorageInfo")]
    #[link_name = "GetStorageInfo__Q2_2nn3spmFPQ3_2nn3spm11StorageInfoQ3_2nn3spm12StorageIndex"]
    pub unsafe fn get_storage_info(info: *mut StorageInfo, index: *const StorageIndex) -> Error;

    /// ReadRawStorageHead512
    ///
    /// Reads the first 512 bytes of the drive.
    #[doc(alias = "ReadRawStorageHead512")]
    #[link_name = "ReadRawStorageHead512__Q2_2nn3spmFQ3_2nn3spm12StorageIndexPc"]
    pub unsafe fn read_raw_storage_head_512(
        index: *const StorageIndex,
        buf: *mut [u8; 512],
    ) -> Error;

    /// IsStorageMaybePcFormatted
    #[doc(alias = "IsStorageMaybePcFormatted")]
    #[link_name = "IsStorageMaybePcFormatted__Q2_2nn3spmFPbQ3_2nn3spm12StorageIndex"]
    pub unsafe fn is_storage_maybe_pc_formatted(
        pc_formatted: *mut bool,
        index: *const StorageIndex,
    ) -> Error;

    /// IsStorageMaybeWfsFormatted
    #[doc(alias = "IsStorageMaybeWfsFormatted")]
    #[link_name = "IsStorageMaybeWfsFormatted__Q2_2nn3spmFPbQ3_2nn3spm12StorageIndex"]
    pub unsafe fn is_storage_maybe_wfs_formatted(
        wfs_formatted: *mut bool,
        index: *const StorageIndex,
    ) -> Error;

    /// VolumeId::Compare
    ///
    /// Returns 0 if equal.
    #[doc(alias = "Compare")]
    #[link_name = "Compare__Q3_2nn3spm8VolumeIdCFRCQ3_2nn3spm8VolumeId"]
    pub unsafe fn volume_id_compare(this: *const VolumeId, other: *const VolumeId) -> i32;
}

imports_section!(
    "nn_spm",
    [
        "CancelWaitStateUpdated__Q2_2nn3spmFv",
        "ClearExtendedStorage__Q2_2nn3spmFv",
        "Compare__Q3_2nn3spm8VolumeIdCFRCQ3_2nn3spm8VolumeId",
        "FilterBriefTitleList__Q2_2nn3spmFP22MCP_TitleListBriefTypeUi",
        "FilterTitleList__Q2_2nn3spmFP17MCP_TitleListTypeUi",
        "Finalize__Q2_2nn3spmFv",
        "FindStorageByVolumeId__Q2_2nn3spmFPQ3_2nn3spm12StorageIndexRCQ3_2nn3spm8VolumeId",
        "GetDefaultExtendedStorageVolumeId__Q2_2nn3spmFv",
        "GetExtendedStorageIndex__Q2_2nn3spmFPQ3_2nn3spm12StorageIndex",
        "GetPreviousExtendedStorageVolumeId__Q2_2nn3spmFPbPQ3_2nn3spm8VolumeId",
        "GetStorageInfo__Q2_2nn3spmFPQ3_2nn3spm11StorageInfoQ3_2nn3spm12StorageIndex",
        "GetStorageList__Q2_2nn3spmFPQ3_2nn3spm15StorageListItemUi",
        "GetUsbDetectionMaxTimeMilliSeconds__Q2_2nn3spmFv",
        "GetUsbDetectionRestTimeMilliSeconds__Q2_2nn3spmFv",
        "Initialize__Q2_2nn3spmFv",
        "IsDefaultTimedOut__Q2_2nn3spmFv",
        "IsStorageMaybePcFormatted__Q2_2nn3spmFPbQ3_2nn3spm12StorageIndex",
        "IsStorageMaybeWfsFormatted__Q2_2nn3spmFPbQ3_2nn3spm12StorageIndex",
        "NeedsManualExtendedStorageResolutionImpl__Q2_2nn3spmFPb",
        "NeedsManualExtendedStorageResolution__Q2_2nn3spmFv",
        "ReadRawStorageHead512__Q2_2nn3spmFQ3_2nn3spm12StorageIndexPc",
        "SetAutoFatal__Q2_2nn3spmFb",
        "SetDefaultExtendedStorageVolumeId__Q2_2nn3spmFRCQ3_2nn3spm8VolumeId",
        "SetExtendedStorage__Q2_2nn3spmFQ3_2nn3spm12StorageIndex",
        "SetResolveManuallyOnWarmBoot__Q2_2nn3spmFb",
        "StopFatalDetection__Q2_2nn3spmFv",
        "WaitStateUpdated__Q2_2nn3spmFPUL",
        "__CPR86__FormatExternalStorage__Q2_2nn3spmFPQ3_2nn3spm12StorageIndexiQ3_2nn3spmJ45JPi",
        "__ct__Q3_2nn3spm8VolumeIdFPCc"
    ],
    []
);
