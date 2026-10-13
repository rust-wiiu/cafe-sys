//! File System (FS)
//!
//! Asynchronous and synchronous file access through clients and command blocks.

use crate::coreinit::message_queue::MessageQueue;
use crate::coreinit::time::DateTime;
use crate::ffi;

/// FS_MAX_CLIENTS
#[doc(alias = "FS_MAX_CLIENTS")]
pub const MAX_CLIENTS: u32 = 63;
pub const MAX_MOUNTPOINTS: u32 = 48;
/// FS_IO_BUFFER_ALIGN
///
/// Required alignment of read and write buffers.
#[doc(alias = "FS_IO_BUFFER_ALIGN")]
pub const IO_BUFFER_ALIGN: usize = 64;
/// FS_MAX_PATH
///
/// Maximum length of a full path, without the terminator.
#[doc(alias = "FS_MAX_PATH", alias = "FS_MAX_FULLPATH_SIZE")]
pub const MAX_PATH: usize = 0x27F;
pub const MAX_ENTRY_NAME_SIZE: usize = 256;
/// FS_MODE_LENGTH
///
/// Maximum size of an open mode string.
#[doc(alias = "FS_MODE_LENGTH")]
pub const MODE_LENGTH: usize = 16;

/// FSTime
///
/// Microseconds, see [time_to_calendar_time].
#[doc(alias = "FSTime")]
pub type Time = u64;

/// FSClient
#[doc(alias = "FSClient")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client(ffi::unknown<0x1700>);

/// FSPriority
#[doc(alias = "FSPriority")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CmdPriority(pub u32);

impl CmdPriority {
    pub const HIGHEST: Self = Self(0);
    pub const DEFAULT: Self = Self(16);
    pub const LOWEST: Self = Self(31);
}

/// FSCmdBlock
///
/// Initialize with [init_cmd_block]. Only one command can use a block at a time.
#[doc(alias = "FSCmdBlock")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CmdBlock(ffi::unknown<0xA80>);

/// FSFileHandle
#[doc(alias = "FSFileHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileHandle(pub i32);

impl FileHandle {
    pub const INVALID: Self = Self(-1);
}

/// FSDirectoryHandle
#[doc(alias = "FSDirectoryHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirHandle(pub i32);

impl DirHandle {
    pub const INVALID: Self = Self(-1);
}

/// FSStatus
///
/// Non-negative values are success, some functions return a count.
#[doc(alias = "FSStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub i32);

impl Status {
    /// FS_STATUS_OK
    #[doc(alias = "FS_STATUS_OK")]
    pub const SUCCESS: Self = Self(0);
    pub const CANCELLED: Self = Self(-1);
    /// End of the file or directory.
    pub const END: Self = Self(-2);
    pub const MAX: Self = Self(-3);
    pub const ALREADY_OPEN: Self = Self(-4);
    pub const EXISTS: Self = Self(-5);
    pub const NOT_FOUND: Self = Self(-6);
    pub const NOT_FILE: Self = Self(-7);
    pub const NOT_DIR: Self = Self(-8);
    pub const ACCESS_ERROR: Self = Self(-9);
    pub const PERMISSION_ERROR: Self = Self(-10);
    pub const FILE_TOO_BIG: Self = Self(-11);
    pub const STORAGE_FULL: Self = Self(-12);
    pub const JOURNAL_FULL: Self = Self(-13);
    pub const UNSUPPORTED_CMD: Self = Self(-14);
    pub const MEDIA_NOT_READY: Self = Self(-15);
    pub const INVALID_MEDIA: Self = Self(-16);
    pub const MEDIA_ERROR: Self = Self(-17);
    pub const DATA_CORRUPTED: Self = Self(-18);
    pub const WRITE_PROTECTED: Self = Self(-19);
    pub const FATAL_ERROR: Self = Self(-1024);

    #[inline]
    pub const fn is_success(self) -> bool {
        self.0 >= 0
    }
}

/// FSError
#[doc(alias = "FSError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    /// FS_ERROR_OK
    #[doc(alias = "FS_ERROR_OK")]
    pub const SUCCESS: Self = Self(0);
    pub const NOT_INIT: Self = Self(-0x30001);
    pub const BUSY: Self = Self(-0x30002);
    pub const CANCELLED: Self = Self(-0x30003);
    pub const END_OF_DIR: Self = Self(-0x30004);
    pub const END_OF_FILE: Self = Self(-0x30005);
    pub const MAX_MOUNT_POINTS: Self = Self(-0x30010);
    pub const MAX_VOLUMES: Self = Self(-0x30011);
    pub const MAX_CLIENTS: Self = Self(-0x30012);
    pub const MAX_FILES: Self = Self(-0x30013);
    pub const MAX_DIRS: Self = Self(-0x30014);
    pub const ALREADY_OPEN: Self = Self(-0x30015);
    pub const ALREADY_EXISTS: Self = Self(-0x30016);
    pub const NOT_FOUND: Self = Self(-0x30017);
    pub const NOT_EMPTY: Self = Self(-0x30018);
    pub const ACCESS_ERROR: Self = Self(-0x30019);
    pub const PERMISSION_ERROR: Self = Self(-0x3001A);
    pub const DATA_CORRUPTED: Self = Self(-0x3001B);
    pub const STORAGE_FULL: Self = Self(-0x3001C);
    pub const JOURNAL_FULL: Self = Self(-0x3001D);
    pub const UNAVAILABLE_COMMAND: Self = Self(-0x3001F);
    pub const UNSUPPORTED_COMMAND: Self = Self(-0x30020);
    pub const INVALID_PARAM: Self = Self(-0x30021);
    pub const INVALID_PATH: Self = Self(-0x30022);
    pub const INVALID_BUFFER: Self = Self(-0x30023);
    pub const INVALID_ALIGNMENT: Self = Self(-0x30024);
    pub const INVALID_CLIENT_HANDLE: Self = Self(-0x30025);
    pub const INVALID_FILE_HANDLE: Self = Self(-0x30026);
    pub const INVALID_DIR_HANDLE: Self = Self(-0x30027);
    pub const NOT_FILE: Self = Self(-0x30028);
    pub const NOT_DIR: Self = Self(-0x30029);
    pub const FILE_TOO_BIG: Self = Self(-0x3002A);
    pub const OUT_OF_RANGE: Self = Self(-0x3002B);
    pub const OUT_OF_RESOURCES: Self = Self(-0x3002C);
    pub const MEDIA_NOT_READY: Self = Self(-0x30040);
    pub const MEDIA_ERROR: Self = Self(-0x30041);
    pub const WRITE_PROTECTED: Self = Self(-0x30042);
    pub const INVALID_MEDIA: Self = Self(-0x30043);
    pub const FATAL: Self = Self(-0x30400);
}

/// FSErrorFlag
#[doc(alias = "FSErrorFlag")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorFlag(pub u32);

impl ErrorFlag {
    pub const NONE: Self = Self(0);
    pub const MAX: Self = Self(0x1);
    pub const ALREADY_OPEN: Self = Self(0x2);
    pub const EXISTS: Self = Self(0x4);
    pub const NOT_FOUND: Self = Self(0x8);
    pub const NOT_FILE: Self = Self(0x10);
    pub const NOT_DIR: Self = Self(0x20);
    pub const ACCESS_ERROR: Self = Self(0x40);
    pub const PERMISSION_ERROR: Self = Self(0x80);
    pub const FILE_TOO_BIG: Self = Self(0x100);
    pub const STORAGE_FULL: Self = Self(0x200);
    pub const UNSUPPORTED_CMD: Self = Self(0x400);
    pub const JOURNAL_FULL: Self = Self(0x800);
    pub const ALL: Self = Self(0xFFFF_FFFF);
}

impl core::ops::BitOr for ErrorFlag {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// FSMode
///
/// Access permissions.
#[doc(alias = "FSMode")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mode(pub u32);

impl Mode {
    pub const READ_OWNER: Self = Self(0x400);
    pub const WRITE_OWNER: Self = Self(0x200);
    pub const EXEC_OWNER: Self = Self(0x100);
    pub const READ_GROUP: Self = Self(0x040);
    pub const WRITE_GROUP: Self = Self(0x020);
    pub const EXEC_GROUP: Self = Self(0x010);
    pub const READ_OTHER: Self = Self(0x004);
    pub const WRITE_OTHER: Self = Self(0x002);
    pub const EXEC_OTHER: Self = Self(0x001);
}

impl core::ops::BitOr for Mode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// FSStatFlags
#[doc(alias = "FSStatFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatFlags(pub u32);

impl StatFlags {
    /// The entry is a directory.
    pub const DIRECTORY: Self = Self(0x8000_0000);
    /// The entry has a quota.
    pub const QUOTA: Self = Self(0x6000_0000);
    /// The entry is a file.
    pub const FILE: Self = Self(0x0100_0000);
    /// The file is encrypted and cannot be opened.
    pub const ENCRYPTED_FILE: Self = Self(0x0080_0000);
    /// The entry is a link.
    pub const LINK: Self = Self(0x0001_0000);

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl core::ops::BitOr for StatFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// FSOpenFileFlags
#[doc(alias = "FSOpenFileFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpenFileFlags(pub u32);

impl OpenFileFlags {
    pub const NONE: Self = Self(0);
    pub const UNENCRYPTED: Self = Self(1 << 0);
    /// Preallocates `prealloc_size` bytes.
    pub const PREALLOC_SIZE: Self = Self(1 << 1);
}

impl core::ops::BitOr for OpenFileFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// FSVolumeState
#[doc(alias = "FSVolumeState")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VolumeState(pub u32);

impl VolumeState {
    pub const INITIAL: Self = Self(0);
    pub const READY: Self = Self(1);
    pub const NO_MEDIA: Self = Self(2);
    pub const INVALID_MEDIA: Self = Self(3);
    pub const DIRTY_MEDIA: Self = Self(4);
    pub const WRONG_MEDIA: Self = Self(5);
    pub const MEDIA_ERROR: Self = Self(6);
    pub const DATA_CORRUPTED: Self = Self(7);
    pub const WRITE_PROTECTED: Self = Self(8);
    pub const JOURNAL_FULL: Self = Self(9);
    pub const FATAL: Self = Self(10);
    pub const INVALID: Self = Self(11);
}

/// FSMediaState
#[doc(alias = "FSMediaState")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MediaState(pub u32);

impl MediaState {
    pub const READY: Self = Self(0);
    pub const NO_MEDIA: Self = Self(1);
    pub const INVALID_MEDIA: Self = Self(2);
    pub const DIRTY_MEDIA: Self = Self(3);
    pub const MEDIA_ERROR: Self = Self(4);
}

/// FSMountSourceType
#[doc(alias = "FSMountSourceType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MountSourceType(pub u32);

impl MountSourceType {
    /// The SD card.
    pub const SD: Self = Self(0);
    /// Host file I/O.
    pub const HFIO: Self = Self(1);
}

/// FSMountSource
#[doc(alias = "FSMountSource")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MountSource {
    pub kind: MountSourceType,
    pub path: [ffi::c_char; MAX_PATH],
}

impl Default for MountSource {
    fn default() -> Self {
        Self {
            kind: MountSourceType::SD,
            path: [0; MAX_PATH],
        }
    }
}

/// FSStat
#[doc(alias = "FSStat")]
#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stat {
    pub flags: StatFlags,
    pub mode: Mode,
    pub owner: u32,
    pub group: u32,
    /// File size in bytes.
    pub size: u32,
    pub alloc_size: u32,
    pub quota_size: u64,
    pub entry_id: u32,
    pub created: Time,
    pub modified: Time,
    pub attributes: [u8; 48],
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            flags: StatFlags(0),
            mode: Mode(0),
            owner: 0,
            group: 0,
            size: 0,
            alloc_size: 0,
            quota_size: 0,
            entry_id: 0,
            created: 0,
            modified: 0,
            attributes: [0; 48],
        }
    }
}

/// FSDirectoryEntry
#[doc(alias = "FSDirectoryEntry")]
#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirectoryEntry {
    pub info: Stat,
    pub name: [ffi::c_char; MAX_ENTRY_NAME_SIZE],
}

impl Default for DirectoryEntry {
    fn default() -> Self {
        Self {
            info: Stat::default(),
            name: [0; MAX_ENTRY_NAME_SIZE],
        }
    }
}

/// FSVolumeInfo
#[doc(alias = "FSVolumeInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VolumeInfo {
    pub flags: u32,
    pub media_state: MediaState,
    _unk0: ffi::unknown<0x4>,
    pub unk0x0c: u32,
    pub unk0x10: u32,
    pub unk0x14: i32,
    pub unk0x18: i32,
    _unk1: ffi::unknown<0x10>,
    pub volume_label: [ffi::c_char; 128],
    pub volume_id: [ffi::c_char; 128],
    pub device_path: [ffi::c_char; 16],
    pub mount_path: [ffi::c_char; 128],
}

impl Default for VolumeInfo {
    fn default() -> Self {
        Self {
            flags: 0,
            media_state: MediaState::READY,
            _unk0: ffi::unknown::zero(),
            unk0x0c: 0,
            unk0x10: 0,
            unk0x14: 0,
            unk0x18: 0,
            _unk1: ffi::unknown::zero(),
            volume_label: [0; 128],
            volume_id: [0; 128],
            device_path: [0; 16],
            mount_path: [0; 128],
        }
    }
}

/// FSMessage
///
/// Message sent to an [AsyncData::io_msg_queue], see [get_async_result] and [get_state_change_info].
#[doc(alias = "FSMessage")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Message {
    /// [AsyncResult] or [StateNoticeInfo].
    pub data: *mut ffi::c_void,
    _unk: ffi::unknown<0x8>,
    /// OSFunctionType.
    pub kind: u32,
}

/// FSAsyncCallback
#[doc(alias = "FSAsyncCallback")]
pub type AsyncCallback = Option<
    unsafe extern "C" fn(
        client: *mut Client,
        block: *mut CmdBlock,
        status: Status,
        context: *mut ffi::c_void,
    ),
>;

/// FSAsyncData
///
/// Either `callback` or `io_msg_queue` is used to report the result.
#[doc(alias = "FSAsyncData")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncData {
    pub callback: AsyncCallback,
    pub context: *mut ffi::c_void,
    pub io_msg_queue: *mut MessageQueue,
}

impl Default for AsyncData {
    fn default() -> Self {
        Self {
            callback: None,
            context: core::ptr::null_mut(),
            io_msg_queue: core::ptr::null_mut(),
        }
    }
}

/// FSAsyncResult
#[doc(alias = "FSAsyncResult")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncResult {
    pub async_data: AsyncData,
    pub io_msg: Message,
    pub client: *mut Client,
    pub block: *mut CmdBlock,
    pub status: Status,
}

/// FSStateChangeCallback
#[doc(alias = "FSStateChangeCallback")]
pub type StateChangeCallback = Option<
    unsafe extern "C" fn(client: *mut Client, state: VolumeState, context: *mut ffi::c_void),
>;

/// FSStateChangeParams
#[doc(alias = "FSStateChangeParams", alias = "FSStateChangeInfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StateChangeParams {
    pub callback: StateChangeCallback,
    pub context: *mut ffi::c_void,
    pub io_msg_queue: *mut MessageQueue,
}

impl Default for StateChangeParams {
    fn default() -> Self {
        Self {
            callback: None,
            context: core::ptr::null_mut(),
            io_msg_queue: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StateNoticeInfo {
    pub params: StateChangeParams,
    pub io_msg: Message,
    pub client: *mut Client,
    pub state: VolumeState,
}

const _: () = {
    assert!(size_of::<Client>() == 0x1700);
    assert!(size_of::<CmdBlock>() == 0xA80);
    assert!(size_of::<Stat>() == 0x64);
    assert!(core::mem::offset_of!(Stat, created) == 0x24);
    assert!(core::mem::offset_of!(Stat, attributes) == 0x34);
    assert!(size_of::<DirectoryEntry>() == 0x164);
    assert!(size_of::<MountSource>() == 0x284);
    assert!(size_of::<VolumeInfo>() == 0x1BC);
    assert!(core::mem::offset_of!(VolumeInfo, volume_label) == 0x2C);
    assert!(core::mem::offset_of!(VolumeInfo, mount_path) == 0x13C);
};

unsafe extern "C" {
    /// FSInit
    ///
    /// Initializes the FS library. Can be called multiple times.
    #[doc(alias = "FSInit")]
    #[link_name = "FSInit"]
    pub unsafe fn init();

    /// FSShutdown
    #[doc(alias = "FSShutdown")]
    #[link_name = "FSShutdown"]
    pub unsafe fn shutdown();

    /// FSAddClient
    ///
    /// Registers `client`, up to [MAX_CLIENTS].
    #[doc(alias = "FSAddClient")]
    #[link_name = "FSAddClient"]
    pub unsafe fn add_client(client: *mut Client, error_mask: ErrorFlag) -> Status;

    /// FSDelClient
    #[doc(alias = "FSDelClient")]
    #[link_name = "FSDelClient"]
    pub unsafe fn del_client(client: *mut Client, error_mask: ErrorFlag) -> Status;

    /// FSGetClientNum
    #[doc(alias = "FSGetClientNum")]
    #[link_name = "FSGetClientNum"]
    pub unsafe fn get_client_num() -> u32;

    /// FSInitCmdBlock
    ///
    /// Also resets the command priority to [CmdPriority::DEFAULT].
    #[doc(alias = "FSInitCmdBlock")]
    #[link_name = "FSInitCmdBlock"]
    pub unsafe fn init_cmd_block(block: *mut CmdBlock);

    /// FSCancelCommand
    #[doc(alias = "FSCancelCommand")]
    #[link_name = "FSCancelCommand"]
    pub unsafe fn cancel_command(client: *mut Client, block: *mut CmdBlock);

    /// FSCancelAllCommands
    #[doc(alias = "FSCancelAllCommands")]
    #[link_name = "FSCancelAllCommands"]
    pub unsafe fn cancel_all_commands(client: *mut Client);

    /// FSSetUserData
    #[doc(alias = "FSSetUserData")]
    #[link_name = "FSSetUserData"]
    pub unsafe fn set_user_data(block: *mut CmdBlock, user_data: *mut ffi::c_void);

    /// FSGetUserData
    #[doc(alias = "FSGetUserData")]
    #[link_name = "FSGetUserData"]
    pub unsafe fn get_user_data(block: *const CmdBlock) -> *mut ffi::c_void;

    /// FSGetCurrentCmdBlock
    ///
    /// Null if there is no current command.
    #[doc(alias = "FSGetCurrentCmdBlock")]
    #[link_name = "FSGetCurrentCmdBlock"]
    pub unsafe fn get_current_cmd_block(client: *mut Client) -> *mut CmdBlock;

    /// FSGetVolumeState
    #[doc(alias = "FSGetVolumeState")]
    #[link_name = "FSGetVolumeState"]
    pub unsafe fn get_volume_state(client: *mut Client) -> VolumeState;

    /// FSSetStateChangeNotification
    ///
    /// `params` is copied. Null `params` disables the notification.
    #[doc(alias = "FSSetStateChangeNotification")]
    #[link_name = "FSSetStateChangeNotification"]
    pub unsafe fn set_state_change_notification(
        client: *mut Client,
        params: *const StateChangeParams,
    );

    /// FSGetAsyncResult
    ///
    /// Result of an async command reported through a message queue.
    #[doc(alias = "FSGetAsyncResult")]
    #[link_name = "FSGetAsyncResult"]
    pub unsafe fn get_async_result(message: *const Message) -> *mut AsyncResult;

    /// FSGetStateChangeInfo
    ///
    /// State change reported through a message queue.
    #[doc(alias = "FSGetStateChangeInfo")]
    #[link_name = "FSGetStateChangeInfo"]
    pub unsafe fn get_state_change_info(message: *const Message) -> *mut StateNoticeInfo;

    /// FSGetMountSource
    #[doc(alias = "FSGetMountSource")]
    #[link_name = "FSGetMountSource"]
    pub unsafe fn get_mount_source(
        client: *mut Client,
        block: *mut CmdBlock,
        kind: MountSourceType,
        source: *mut MountSource,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetMountSourceAsync
    ///
    /// Async version of [get_mount_source].
    #[doc(alias = "FSGetMountSourceAsync")]
    #[link_name = "FSGetMountSourceAsync"]
    pub unsafe fn get_mount_source_async(
        client: *mut Client,
        block: *mut CmdBlock,
        kind: MountSourceType,
        source: *mut MountSource,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetMountSourceNext
    #[doc(alias = "FSGetMountSourceNext")]
    #[link_name = "FSGetMountSourceNext"]
    pub unsafe fn get_mount_source_next(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *mut MountSource,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetMountSourceNextAsync
    ///
    /// Async version of [get_mount_source_next].
    #[doc(alias = "FSGetMountSourceNextAsync")]
    #[link_name = "FSGetMountSourceNextAsync"]
    pub unsafe fn get_mount_source_next_async(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *mut MountSource,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSMount
    ///
    /// Writes the mount path, up to `bytes`, into `target`.
    #[doc(alias = "FSMount")]
    #[link_name = "FSMount"]
    pub unsafe fn mount(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const MountSource,
        target: *mut ffi::c_char,
        bytes: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSMountAsync
    ///
    /// Async version of [mount].
    #[doc(alias = "FSMountAsync")]
    #[link_name = "FSMountAsync"]
    pub unsafe fn mount_async(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const MountSource,
        target: *mut ffi::c_char,
        bytes: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSUnmount
    #[doc(alias = "FSUnmount")]
    #[link_name = "FSUnmount"]
    pub unsafe fn unmount(
        client: *mut Client,
        block: *mut CmdBlock,
        target: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSUnmountAsync
    ///
    /// Async version of [unmount].
    #[doc(alias = "FSUnmountAsync")]
    #[link_name = "FSUnmountAsync"]
    pub unsafe fn unmount_async(
        client: *mut Client,
        block: *mut CmdBlock,
        target: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSBindMount
    #[doc(alias = "FSBindMount")]
    #[link_name = "FSBindMount"]
    pub unsafe fn bind_mount(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const ffi::c_char,
        target: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSBindUnmount
    #[doc(alias = "FSBindUnmount")]
    #[link_name = "FSBindUnmount"]
    pub unsafe fn bind_unmount(
        client: *mut Client,
        block: *mut CmdBlock,
        target: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSChangeDir
    #[doc(alias = "FSChangeDir")]
    #[link_name = "FSChangeDir"]
    pub unsafe fn change_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSChangeDirAsync
    ///
    /// Async version of [change_dir].
    #[doc(alias = "FSChangeDirAsync")]
    #[link_name = "FSChangeDirAsync"]
    pub unsafe fn change_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetCwd
    #[doc(alias = "FSGetCwd")]
    #[link_name = "FSGetCwd"]
    pub unsafe fn get_cwd(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *mut ffi::c_char,
        bytes: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetCwdAsync
    ///
    /// Async version of [get_cwd].
    #[doc(alias = "FSGetCwdAsync")]
    #[link_name = "FSGetCwdAsync"]
    pub unsafe fn get_cwd_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *mut ffi::c_char,
        bytes: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSOpenFile
    ///
    /// `mode` is a C `fopen` mode string.
    #[doc(alias = "FSOpenFile")]
    #[link_name = "FSOpenFile"]
    pub unsafe fn open_file(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSOpenFileAsync
    ///
    /// Async version of [open_file].
    #[doc(alias = "FSOpenFileAsync")]
    #[link_name = "FSOpenFileAsync"]
    pub unsafe fn open_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        handle: *mut FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSOpenFileEx
    #[doc(alias = "FSOpenFileEx")]
    #[link_name = "FSOpenFileEx"]
    pub unsafe fn open_file_ex(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        create_mode: Mode,
        flags: OpenFileFlags,
        prealloc_size: u32,
        handle: *mut FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSOpenFileExAsync
    ///
    /// Async version of [open_file_ex].
    #[doc(alias = "FSOpenFileExAsync")]
    #[link_name = "FSOpenFileExAsync"]
    pub unsafe fn open_file_ex_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        create_mode: Mode,
        flags: OpenFileFlags,
        prealloc_size: u32,
        handle: *mut FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSCloseFile
    #[doc(alias = "FSCloseFile")]
    #[link_name = "FSCloseFile"]
    pub unsafe fn close_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSCloseFileAsync
    ///
    /// Async version of [close_file].
    #[doc(alias = "FSCloseFileAsync")]
    #[link_name = "FSCloseFileAsync"]
    pub unsafe fn close_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSReadFile
    ///
    /// Reads `count` elements of `size` bytes. `dest` must be [IO_BUFFER_ALIGN]-aligned. Returns the number of elements read on success. `flag` must be 0.
    #[doc(alias = "FSReadFile")]
    #[link_name = "FSReadFile"]
    pub unsafe fn read_file(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut ffi::c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSReadFileAsync
    ///
    /// Async version of [read_file].
    #[doc(alias = "FSReadFileAsync")]
    #[link_name = "FSReadFileAsync"]
    pub unsafe fn read_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut ffi::c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSWriteFile
    ///
    /// Writes `count` elements of `size` bytes. `source` must be [IO_BUFFER_ALIGN]-aligned. Returns the number of elements written on success. `flag` must be 0.
    #[doc(alias = "FSWriteFile")]
    #[link_name = "FSWriteFile"]
    pub unsafe fn write_file(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const ffi::c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSWriteFileAsync
    ///
    /// Async version of [write_file].
    #[doc(alias = "FSWriteFileAsync")]
    #[link_name = "FSWriteFileAsync"]
    pub unsafe fn write_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const ffi::c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSReadFileWithPos
    ///
    /// Same as [read_file], starting at `position`.
    #[doc(alias = "FSReadFileWithPos")]
    #[link_name = "FSReadFileWithPos"]
    pub unsafe fn read_file_with_pos(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut ffi::c_void,
        size: u32,
        count: u32,
        position: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSReadFileWithPosAsync
    ///
    /// Async version of [read_file_with_pos].
    #[doc(alias = "FSReadFileWithPosAsync")]
    #[link_name = "FSReadFileWithPosAsync"]
    pub unsafe fn read_file_with_pos_async(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut ffi::c_void,
        size: u32,
        count: u32,
        position: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSWriteFileWithPos
    ///
    /// Same as [write_file], starting at `position`.
    #[doc(alias = "FSWriteFileWithPos")]
    #[link_name = "FSWriteFileWithPos"]
    pub unsafe fn write_file_with_pos(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const ffi::c_void,
        size: u32,
        count: u32,
        position: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSWriteFileWithPosAsync
    ///
    /// Async version of [write_file_with_pos].
    #[doc(alias = "FSWriteFileWithPosAsync")]
    #[link_name = "FSWriteFileWithPosAsync"]
    pub unsafe fn write_file_with_pos_async(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const ffi::c_void,
        size: u32,
        count: u32,
        position: u32,
        handle: FileHandle,
        flag: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSAppendFile
    ///
    /// Extends the file by `count` elements of `size` bytes.
    #[doc(alias = "FSAppendFile")]
    #[link_name = "FSAppendFile"]
    pub unsafe fn append_file(
        client: *mut Client,
        block: *mut CmdBlock,
        size: u32,
        count: u32,
        handle: FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSAppendFileAsync
    ///
    /// Async version of [append_file].
    #[doc(alias = "FSAppendFileAsync")]
    #[link_name = "FSAppendFileAsync"]
    pub unsafe fn append_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        size: u32,
        count: u32,
        handle: FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSTruncateFile
    ///
    /// Truncates the file at the current position.
    #[doc(alias = "FSTruncateFile")]
    #[link_name = "FSTruncateFile"]
    pub unsafe fn truncate_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSTruncateFileAsync
    ///
    /// Async version of [truncate_file].
    #[doc(alias = "FSTruncateFileAsync")]
    #[link_name = "FSTruncateFileAsync"]
    pub unsafe fn truncate_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSFlushFile
    #[doc(alias = "FSFlushFile")]
    #[link_name = "FSFlushFile"]
    pub unsafe fn flush_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSFlushFileAsync
    ///
    /// Async version of [flush_file].
    #[doc(alias = "FSFlushFileAsync")]
    #[link_name = "FSFlushFileAsync"]
    pub unsafe fn flush_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetPosFile
    #[doc(alias = "FSGetPosFile")]
    #[link_name = "FSGetPosFile"]
    pub unsafe fn get_pos_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        position: *mut u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetPosFileAsync
    ///
    /// Async version of [get_pos_file].
    #[doc(alias = "FSGetPosFileAsync")]
    #[link_name = "FSGetPosFileAsync"]
    pub unsafe fn get_pos_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        position: *mut u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSSetPosFile
    #[doc(alias = "FSSetPosFile")]
    #[link_name = "FSSetPosFile"]
    pub unsafe fn set_pos_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        position: u32,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSSetPosFileAsync
    ///
    /// Async version of [set_pos_file].
    #[doc(alias = "FSSetPosFileAsync")]
    #[link_name = "FSSetPosFileAsync"]
    pub unsafe fn set_pos_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        position: u32,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetStatFile
    #[doc(alias = "FSGetStatFile")]
    #[link_name = "FSGetStatFile"]
    pub unsafe fn get_stat_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        stat: *mut Stat,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetStatFileAsync
    ///
    /// Async version of [get_stat_file].
    #[doc(alias = "FSGetStatFileAsync")]
    #[link_name = "FSGetStatFileAsync"]
    pub unsafe fn get_stat_file_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        stat: *mut Stat,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSIsEof
    ///
    /// Returns [Status::END] at the end of the file.
    #[doc(alias = "FSIsEof")]
    #[link_name = "FSIsEof"]
    pub unsafe fn is_eof(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSIsEofAsync
    ///
    /// Async version of [is_eof].
    #[doc(alias = "FSIsEofAsync")]
    #[link_name = "FSIsEofAsync"]
    pub unsafe fn is_eof_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSOpenDir
    #[doc(alias = "FSOpenDir")]
    #[link_name = "FSOpenDir"]
    pub unsafe fn open_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        handle: *mut DirHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSOpenDirAsync
    ///
    /// Async version of [open_dir].
    #[doc(alias = "FSOpenDirAsync")]
    #[link_name = "FSOpenDirAsync"]
    pub unsafe fn open_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        handle: *mut DirHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSCloseDir
    #[doc(alias = "FSCloseDir")]
    #[link_name = "FSCloseDir"]
    pub unsafe fn close_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSCloseDirAsync
    ///
    /// Async version of [close_dir].
    #[doc(alias = "FSCloseDirAsync")]
    #[link_name = "FSCloseDirAsync"]
    pub unsafe fn close_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSReadDir
    ///
    /// Returns [Status::END] after the last entry.
    #[doc(alias = "FSReadDir")]
    #[link_name = "FSReadDir"]
    pub unsafe fn read_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        entry: *mut DirectoryEntry,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSReadDirAsync
    ///
    /// Async version of [read_dir].
    #[doc(alias = "FSReadDirAsync")]
    #[link_name = "FSReadDirAsync"]
    pub unsafe fn read_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        entry: *mut DirectoryEntry,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSRewindDir
    #[doc(alias = "FSRewindDir")]
    #[link_name = "FSRewindDir"]
    pub unsafe fn rewind_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSRewindDirAsync
    ///
    /// Async version of [rewind_dir].
    #[doc(alias = "FSRewindDirAsync")]
    #[link_name = "FSRewindDirAsync"]
    pub unsafe fn rewind_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSMakeDir
    #[doc(alias = "FSMakeDir")]
    #[link_name = "FSMakeDir"]
    pub unsafe fn make_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSMakeDirAsync
    ///
    /// Async version of [make_dir].
    #[doc(alias = "FSMakeDirAsync")]
    #[link_name = "FSMakeDirAsync"]
    pub unsafe fn make_dir_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSRemove
    ///
    /// Removes a file or an empty directory.
    #[doc(alias = "FSRemove")]
    #[link_name = "FSRemove"]
    pub unsafe fn remove(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSRemoveAsync
    ///
    /// Async version of [remove].
    #[doc(alias = "FSRemoveAsync")]
    #[link_name = "FSRemoveAsync"]
    pub unsafe fn remove_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSRename
    #[doc(alias = "FSRename")]
    #[link_name = "FSRename"]
    pub unsafe fn rename(
        client: *mut Client,
        block: *mut CmdBlock,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSRenameAsync
    ///
    /// Async version of [rename].
    #[doc(alias = "FSRenameAsync")]
    #[link_name = "FSRenameAsync"]
    pub unsafe fn rename_async(
        client: *mut Client,
        block: *mut CmdBlock,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSChangeMode
    #[doc(alias = "FSChangeMode")]
    #[link_name = "FSChangeMode"]
    pub unsafe fn change_mode(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: Mode,
        mode_mask: Mode,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSChangeModeAsync
    ///
    /// Async version of [change_mode].
    #[doc(alias = "FSChangeModeAsync")]
    #[link_name = "FSChangeModeAsync"]
    pub unsafe fn change_mode_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: Mode,
        mode_mask: Mode,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSMakeQuota
    #[doc(alias = "FSMakeQuota")]
    #[link_name = "FSMakeQuota"]
    pub unsafe fn make_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: Mode,
        size: u64,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSMakeQuotaAsync
    ///
    /// Async version of [make_quota].
    #[doc(alias = "FSMakeQuotaAsync")]
    #[link_name = "FSMakeQuotaAsync"]
    pub unsafe fn make_quota_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        mode: Mode,
        size: u64,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSRemoveQuota
    #[doc(alias = "FSRemoveQuota")]
    #[link_name = "FSRemoveQuota"]
    pub unsafe fn remove_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSRemoveQuotaAsync
    ///
    /// Async version of [remove_quota].
    #[doc(alias = "FSRemoveQuotaAsync")]
    #[link_name = "FSRemoveQuotaAsync"]
    pub unsafe fn remove_quota_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSFlushQuota
    #[doc(alias = "FSFlushQuota")]
    #[link_name = "FSFlushQuota"]
    pub unsafe fn flush_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSFlushQuotaAsync
    ///
    /// Async version of [flush_quota].
    #[doc(alias = "FSFlushQuotaAsync")]
    #[link_name = "FSFlushQuotaAsync"]
    pub unsafe fn flush_quota_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSRollbackQuota
    #[doc(alias = "FSRollbackQuota")]
    #[link_name = "FSRollbackQuota"]
    pub unsafe fn rollback_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSRollbackQuotaAsync
    ///
    /// Async version of [rollback_quota].
    #[doc(alias = "FSRollbackQuotaAsync")]
    #[link_name = "FSRollbackQuotaAsync"]
    pub unsafe fn rollback_quota_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetStat
    #[doc(alias = "FSGetStat")]
    #[link_name = "FSGetStat"]
    pub unsafe fn get_stat(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        stat: *mut Stat,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetStatAsync
    ///
    /// Async version of [get_stat].
    #[doc(alias = "FSGetStatAsync")]
    #[link_name = "FSGetStatAsync"]
    pub unsafe fn get_stat_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        stat: *mut Stat,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetFreeSpaceSize
    #[doc(alias = "FSGetFreeSpaceSize")]
    #[link_name = "FSGetFreeSpaceSize"]
    pub unsafe fn get_free_space_size(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        size: *mut u64,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetFreeSpaceSizeAsync
    ///
    /// Async version of [get_free_space_size].
    #[doc(alias = "FSGetFreeSpaceSizeAsync")]
    #[link_name = "FSGetFreeSpaceSizeAsync"]
    pub unsafe fn get_free_space_size_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        size: *mut u64,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetVolumeInfo
    #[doc(alias = "FSGetVolumeInfo")]
    #[link_name = "FSGetVolumeInfo"]
    pub unsafe fn get_volume_info(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        info: *mut VolumeInfo,
        error_mask: ErrorFlag,
    ) -> Status;

    /// FSGetVolumeInfoAsync
    ///
    /// Async version of [get_volume_info].
    #[doc(alias = "FSGetVolumeInfoAsync")]
    #[link_name = "FSGetVolumeInfoAsync"]
    pub unsafe fn get_volume_info_async(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const ffi::c_char,
        info: *mut VolumeInfo,
        error_mask: ErrorFlag,
        async_data: *const AsyncData,
    ) -> Status;

    /// FSGetLastErrorCodeForViewer
    ///
    /// Error code of the last error, for the error viewer.
    #[doc(alias = "FSGetLastErrorCodeForViewer")]
    #[link_name = "FSGetLastErrorCodeForViewer"]
    pub unsafe fn get_last_error_code_for_viewer(client: *mut Client) -> i32;

    /// FSGetErrorCodeForViewer
    ///
    /// Error code of a command for the error viewer.
    #[doc(alias = "FSGetErrorCodeForViewer")]
    #[link_name = "FSGetErrorCodeForViewer"]
    pub unsafe fn get_error_code_for_viewer(client: *mut Client, block: *mut CmdBlock) -> i32;

    /// FSTimeToCalendarTime
    #[doc(alias = "FSTimeToCalendarTime")]
    #[link_name = "FSTimeToCalendarTime"]
    pub unsafe fn time_to_calendar_time(time: Time, date_time: *mut DateTime);

    /// FSSetCmdPriority
    ///
    /// From [CmdPriority::HIGHEST] to [CmdPriority::LOWEST].
    #[doc(alias = "FSSetCmdPriority")]
    #[link_name = "FSSetCmdPriority"]
    pub unsafe fn set_cmd_priority(block: *mut CmdBlock, priority: CmdPriority) -> Status;

    /// FSGetCmdPriority
    #[doc(alias = "FSGetCmdPriority")]
    #[link_name = "FSGetCmdPriority"]
    pub unsafe fn get_cmd_priority(block: *mut CmdBlock) -> CmdPriority;

    /// FSGetLastError
    #[doc(alias = "FSGetLastError")]
    #[link_name = "FSGetLastError"]
    pub unsafe fn get_last_error(client: *mut Client) -> Error;

    /// FSSetEmulatedError
    #[doc(alias = "FSSetEmulatedError")]
    #[link_name = "FSSetEmulatedError"]
    pub unsafe fn set_emulated_error(client: *mut Client, error: Error) -> Status;

    /// FSGetEmulatedError
    #[doc(alias = "FSGetEmulatedError")]
    #[link_name = "FSGetEmulatedError"]
    pub unsafe fn get_emulated_error(client: *mut Client) -> Error;

}
