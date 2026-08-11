use core::marker::PhantomData;

use crate::ffi::*;
use bitflags::bitflags;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

pub const MAX_LOCALPATH_LEN: usize = 511;
pub const MAX_MOUNTPATH_LEN: usize = 128;

#[doc(alias = "FSClient")]
#[repr(C)]
pub struct Client([u8; 5888]);

#[doc(alias = "FSCmdBlock")]
#[repr(C)]
pub struct CmdBlock([u8; 2688]);

bitflags! {
    #[doc(alias = "FSRetFlag")]
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ErrorMask : u32 {
        const None = 0;
        const Max = 1 << 0;
        const AlreadyOpen = 1 << 1;
        const Exists = 1 << 2;
        const NotFound = 1 << 3;
        const NotFile = 1 << 4;
        const NotDir = 1 << 5;
        const AccessError = 1 << 6;
        const PermissionError = 1 << 7;
        const FileTooBig = 1 << 8;
        const StorageFull = 1 << 9;
        const UnsupportedCmd = 1 << 10;
        const JournalFull = 1 << 11;
        const All = u32::MAX;
    }
}

#[doc(alias = "FSStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Status {
    Ok = 0,
    /// Command cancelled
    Cancelled = -1,
    /// End of file / directory entry
    End = -2,
    /// Max number of file / directory / client handles / mount points
    Max = -3,
    /// Already opened or locked by another transaction
    AlreadyOpen = -4,
    /// Path already exists or not empty
    Exists = -5,
    /// Path not found
    NotFound = -6,
    /// Path is directory or quota instead of file
    NotFile = -7,
    /// Path is file instead of directory or quota
    NotDir = -8,
    /// Attempted to open file with bad file mode
    AccessError = -9,
    /// Missing permissions to complete operation
    PermissionError = -10,
    /// Request would push file over the size limit (not quota limit)
    FileTooBig = -11,
    /// Request would cause one of the parent directories to exceed quota or not free space left in storage
    StorageFull = -12,
    /// Transaction journal full; need to flush
    JournalFull = -13,
    /// Operation is not supported by file system
    UnsupportedCmd = -14,
    /// Mounted media not ready
    MediaNotReady = -15,
    /// Mounted media invalid
    MediaInvalid = -16,
    /// Mounted media is in bad condition
    MediaError = -17,
    /// Mounted data corrupted
    DataCorrupted = -18,
    /// Mounted media is write protected
    WriteProtected = -19,
    /// Internal resources ran short
    FatalError = -0x400,
}

#[repr(C)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Result<T: From<i32>> {
    value: i32,
    _marker: PhantomData<T>,
}

impl<T: From<i32>> Result<T> {
    pub fn check(self) -> core::result::Result<T, Status> {
        if self.value >= 0 {
            Ok(T::from(self.value))
        } else {
            Err(Status::try_from(self.value).unwrap())
        }
    }
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
pub enum Priority {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
    #[default]
    P16,
    P17,
    P18,
    P19,
    P20,
    P21,
    P22,
    P23,
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
    P31,
}

impl Priority {
    pub const HIGHEST: Self = Self::P0;
    pub const LOWEST: Self = Self::P31;
}

pub const MAX_CLIENTS: u32 = 63;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum SourceType {
    /// External storage device (FAT)
    External = 0,
    /// Host file io device
    HFIO = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MountSource {
    pub ty: SourceType,
    pub path: [c_char; 0x27f],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHandle(i32);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirHandle(i32);

bitflags! {
    #[doc(alias = "FSFlag")]
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct StatFlag: u32 {
        /// Entry is directory
        const Directory = 0x80000000;
        /// Entry is quota
        const Quota = 0x40000000;
        /// FS supports `.quota_size` field
        const QuotaSize = 0x20000000;
        /// FS supports `.ent_id` field
        const EntId = 0x10000000;
        /// FS supports `.ctime` field
        const Ctime = 0x08000000;
        /// FS supports `.mtime` field
        const Mtime = 0x04000000;
        /// FS supports `.attributes` field
        const Attributes = 0x02000000;
        /// FS supports `.alloc_size` field
        const FileAllocSize = 0x01000000;
        /// File is not encrypted
        const NotEncrypted = 0x00800000;
        /// FS supports `.size` field for directory (no meaning for file)
        const DirSize = 0x00100000;
        /// Entry name contains unsupported character
        const UnsupportedChar = 0x00080000;
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permission(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time(pub u64);

#[doc(alias = "FSStat")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stat {
    pub flag: StatFlag,
    pub permission: Permission,
    pub owner_id: UserId,
    pub group_id: GroupId,
    pub size: u32,
    pub alloc_size: u32,
    pub quota_size: u64,
    pub ent_id: u32,
    pub ctime: Time,
    pub mtime: Time,
    pub attributes: [u8; 48],
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode(*const c_char);

impl Mode {
    /// Open a file for reading. The file must exist.
    pub const READ: Self = Self(c"r".as_ptr());
    /// Create an empty file for writing. If a file with the same name already exists, its contents will be erased and the file size will become 0 byte.
    pub const WRITE_TRUNCATE: Self = Self(c"w".as_ptr());
    /// Append to a file. Writing operations append data at the end of the file. The file is created if it does not exist.
    pub const APPEND: Self = Self(c"a".as_ptr());
    /// Open a file for update both reading and writing. The file must exist.
    pub const READ_WRITE: Self = Self(c"r+".as_ptr());
    /// Create an empty file for both reading and writing. If a file with the same name already exists, its content will be erased and the file size will become 0 byte.
    pub const READ_WRITE_TRUNCATE: Self = Self(c"r+".as_ptr());
    /// Open a file for reading and appending. All writing operations are performed at the end of the file, protecting the previous contents from being overwritten. You can reposition the internal pointer to anywhere in the file for reading, but writing operations will move it back to the end of file. The file is created if it does not exist.
    pub const READ_APPEND: Self = Self(c"r+".as_ptr());
}

#[doc(alias = "FSDirEntry")]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirEntry {
    pub stat: Stat,
    pub name: [c_char; 256],
}

// #[doc(alias = "FSMessage")]
// #[repr(C, packed)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Message {
//     pub data: *mut c_void,
//     pub reserved1: u32,
//     pub reserved2: u32,
//     pub ty: (),
// }

// #[doc(alias = "FSStateNoticeInfo")]
// #[repr(C, packed)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct StateNoticeInfo {
//     pub user_params: StateChangeParams,
//     pub msg: Message,
//     pub client: *mut Client,
//     pub state: VolumeState,
// }

// #[doc(alias = "FSStateChangeParams")]
// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct StateChangeParams {
//     pub callback: (),
//     pub context: *mut c_void,
//     pub msg_queue: todo!(),
// }

// #[repr(u32)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
// pub enum VolumeState {
//     Initial = 0,
//     Ready = 1,
//     NoMedia = 2,
//     InvalidMedia = 3,
//     DirtyMedia = 4,
//     WrongMedia = 5,
//     MediaError = 6,
//     DataCorrupted = 7,
//     WriteProtected = 8,
//     JournalFull = 9,
//     Fatal = 10,
//     Invalid = 11,
// }

unsafe extern "C" {
    #[doc(alias = "FSInit")]
    #[link_name = "FSInit"]
    pub unsafe fn init();

    #[doc(alias = "FSShutdown")]
    #[link_name = "FSShutdown"]
    pub unsafe fn deinit();

    #[doc(alias = "FSAddClient")]
    #[link_name = "FSAddClient"]
    pub unsafe fn add_client(client: *mut Client, emask: ErrorMask) -> Status;

    #[doc(alias = "FSDelClient")]
    #[link_name = "FSDelClient"]
    pub unsafe fn delete_client(client: *mut Client, emask: ErrorMask) -> Status;

    #[doc(alias = "FSGetClientNum")]
    #[link_name = "FSGetClientNum"]
    pub unsafe fn num_clients() -> u32;

    #[doc(alias = "FSInitCmdBlock")]
    #[link_name = "FSInitCmdBlock"]
    pub unsafe fn init_cmd_block(block: *mut CmdBlock);

    #[doc(alias = "FSCancelAllCommands")]
    #[link_name = "FSCancelAllCommands"]
    pub unsafe fn cancel_queued_commands(client: *mut Client);

    #[doc(alias = "FSCancelCommand")]
    #[link_name = "FSCancelCommand"]
    pub unsafe fn cancel_command(client: *mut Client, block: *mut CmdBlock);

    #[doc(alias = "FSSetUserData")]
    #[link_name = "FSSetUserData"]
    pub unsafe fn set_command_user_data(block: *mut CmdBlock, data: *mut c_void);

    #[doc(alias = "FSGetUserData")]
    #[link_name = "FSGetUserData"]
    pub unsafe fn get_command_user_data(block: *const CmdBlock) -> *mut c_void;

    #[doc(alias = "FSSetCmdPriority")]
    #[link_name = "FSSetCmdPriority"]
    pub unsafe fn set_command_priority(block: *mut CmdBlock, priority: Priority) -> Status;

    #[doc(alias = "FSGetCmdPriority")]
    #[link_name = "FSGetCmdPriority"]
    pub unsafe fn get_command_priority(block: *const CmdBlock) -> Priority;

    #[doc(alias = "FSGetCurrentCmdBlock")]
    #[link_name = "FSGetCurrentCmdBlock"]
    pub unsafe fn current_command(client: *mut Client) -> *mut CmdBlock;

    #[doc(alias = "FSGetMountSource")]
    #[link_name = "FSGetMountSource"]
    pub unsafe fn get_mount_source(
        client: *mut Client,
        block: *mut CmdBlock,
        ty: SourceType,
        source: *mut MountSource,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetMountSourceNext")]
    #[link_name = "FSGetMountSourceNext"]
    pub unsafe fn next_mount_source(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *mut MountSource,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSMount")]
    #[link_name = "FSMount"]
    pub unsafe fn mount(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const MountSource,
        target: *const c_char,
        len: u32,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSUnmount")]
    #[link_name = "FSUnmount"]
    pub unsafe fn unmount(
        client: *mut Client,
        block: *mut CmdBlock,
        target: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSAppendFile")]
    #[link_name = "FSAppendFile"]
    pub unsafe fn append_file(
        client: *mut Client,
        block: *mut CmdBlock,
        size: u32,
        count: u32,
        emask: ErrorMask,
    ) -> Result<i32>;

    #[doc(alias = "FSCloseDir")]
    #[link_name = "FSCloseDir"]
    pub unsafe fn close_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSCloseFile")]
    #[link_name = "FSCloseFile"]
    pub unsafe fn close_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSFlushFile")]
    #[link_name = "FSFlushFile"]
    pub unsafe fn flush_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetFreeSpaceSize")]
    #[link_name = "FSGetFreeSpaceSize"]
    pub unsafe fn free_space_size(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        free_size: *mut u64,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetPosFile")]
    #[link_name = "FSGetPosFile"]
    pub unsafe fn pos_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        file_pos: *mut u32,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetStat")]
    #[link_name = "FSGetStat"]
    pub unsafe fn stat(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        stat: *mut Stat,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetStatFile")]
    #[link_name = "FSGetStatFile"]
    pub unsafe fn stat_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        stat: *mut Stat,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSIsEof")]
    #[link_name = "FSIsEof"]
    pub unsafe fn is_eof(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSMakeDir")]
    #[link_name = "FSMakeDir"]
    pub unsafe fn make_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSOpenDir")]
    #[link_name = "FSOpenDir"]
    pub unsafe fn open_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        handle: *mut DirHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSOpenFile")]
    #[link_name = "FSOpenFile"]
    pub unsafe fn open_file(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        mode: Mode,
        handle: *mut FileHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSReadDir")]
    #[link_name = "FSReadDir"]
    pub unsafe fn read_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        entry: *mut DirEntry,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSReadFile")]
    #[link_name = "FSReadFile"]
    pub unsafe fn read_file(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        unused: u32,
        emask: ErrorMask,
    ) -> Result<i32>;

    #[doc(alias = "FSReadFileWithPos")]
    #[link_name = "FSReadFileWithPos"]
    pub unsafe fn read_file_with_pos(
        client: *mut Client,
        block: *mut CmdBlock,
        dest: *mut c_void,
        size: u32,
        count: u32,
        pos: u32,
        handle: FileHandle,
        unused: u32,
        emask: ErrorMask,
    ) -> Result<i32>;

    #[doc(alias = "FSRemove")]
    #[link_name = "FSRemove"]
    pub unsafe fn remove(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSRename")]
    #[link_name = "FSRename"]
    pub unsafe fn rename(
        client: *mut Client,
        block: *mut CmdBlock,
        old: *const c_char,
        new: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSRewindDir")]
    #[link_name = "FSRewindDir"]
    pub unsafe fn rewind_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: DirHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSSetPosFile")]
    #[link_name = "FSSetPosFile"]
    pub unsafe fn set_pos_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        pos: u32,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSTruncateFile")]
    #[link_name = "FSTruncateFile"]
    pub unsafe fn truncate_file(
        client: *mut Client,
        block: *mut CmdBlock,
        handle: FileHandle,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSWriteFile")]
    #[link_name = "FSWriteFile"]
    pub unsafe fn write_file(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const c_void,
        size: u32,
        count: u32,
        handle: FileHandle,
        unused: u32,
        emask: ErrorMask,
    ) -> Result<i32>;

    #[doc(alias = "FSWriteFileWithPos")]
    #[link_name = "FSWriteFileWithPos"]
    pub unsafe fn write_file_with_pos(
        client: *mut Client,
        block: *mut CmdBlock,
        source: *const c_void,
        size: u32,
        count: u32,
        pos: u32,
        handle: FileHandle,
        unused: u32,
        emask: ErrorMask,
    ) -> Result<i32>;

    #[doc(alias = "FSChangeDir")]
    #[link_name = "FSChangeDir"]
    pub unsafe fn change_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetCwd")]
    #[link_name = "FSGetCwd"]
    pub unsafe fn current_dir(
        client: *mut Client,
        block: *mut CmdBlock,
        cwd: *mut c_char,
        len: u32,
        emask: ErrorMask,
    ) -> Status;

    // #[doc(alias = "FSGetStateChangeInfo")]
    // #[link_name = "FSGetStateChangeInfo"]
    // pub unsafe fn volume_state_change_info(msg: *mut Message) -> *mut StateNoticeInfo;

    // #[doc(alias = "FSGetVolumeState")]
    // #[link_name = "FSGetVolumeState"]
    // pub unsafe fn volume_state(client: *mut Client) -> VolumeState;

    // #[doc(alias = "FSSetStateChangeNotification")]
    // #[link_name = "FSSetStateChangeNotification"]
    // pub unsafe fn volume_set_change_callback(
    //     client: *mut Client,
    //     params: *mut StateChangeParams,
    // ) -> VolumeState;

    #[doc(alias = "FSFlushQuota")]
    #[link_name = "FSFlushQuota"]
    pub unsafe fn flush_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSRollbackQuota")]
    #[link_name = "FSRollbackQuota"]
    pub unsafe fn rollback_quota(
        client: *mut Client,
        block: *mut CmdBlock,
        path: *const c_char,
        emask: ErrorMask,
    ) -> Status;

    #[doc(alias = "FSGetErrorCodeForViewer")]
    #[link_name = "FSGetErrorCodeForViewer"]
    pub unsafe fn error_code_for_viewer(client: *mut Client, block: *mut CmdBlock) -> i32;

    #[doc(alias = "FSGetLastErrorCodeForViewer")]
    #[link_name = "FSGetLastErrorCodeForViewer"]
    pub unsafe fn last_error_code_for_viewer(client: *mut Client) -> i32;

    #[doc(alias = "FSTimeToCalendarTime")]
    #[link_name = "FSTimeToCalendarTime"]
    pub unsafe fn time_to_calendar(fs_time: Time, os_time: *mut crate::coreinit::time::DateTime);
}
