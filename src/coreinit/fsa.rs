//! FSA
//!
//! Low-level file system client that talks to the FSA device directly, without the command blocks of `fs`.

use super::fs::{DirectoryEntry, Error, Message, Mode, OpenFileFlags, Stat, VolumeInfo};
use super::ios;
use super::message_queue::{Message as OsMessage, MessageQueue};
use crate::ffi;

/// FSAClientHandle
///
/// Negative values are [Error] codes.
#[doc(alias = "FSAClientHandle")]
pub type ClientHandle = ios::Handle;

/// FSAFileHandle
#[doc(alias = "FSAFileHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileHandle(pub u32);

/// FSADirectoryHandle
#[doc(alias = "FSADirectoryHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirectoryHandle(pub u32);

/// FSACommand
#[doc(alias = "FSACommand")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Command(pub u32);

impl Command {
    pub const INVALID: Self = Self(0x0);
    pub const MOUNT: Self = Self(0x1);
    pub const UNMOUNT: Self = Self(0x2);
    pub const GET_VOLUME_INFO: Self = Self(0x3);
    pub const GET_ATTACH: Self = Self(0x4);
    pub const CHANGE_DIR: Self = Self(0x5);
    pub const GET_CWD: Self = Self(0x6);
    pub const MAKE_DIR: Self = Self(0x7);
    pub const REMOVE: Self = Self(0x8);
    pub const RENAME: Self = Self(0x9);
    pub const OPEN_DIR: Self = Self(0xA);
    pub const READ_DIR: Self = Self(0xB);
    pub const REWIND_DIR: Self = Self(0xC);
    pub const CLOSE_DIR: Self = Self(0xD);
    pub const OPEN_FILE: Self = Self(0xE);
    pub const READ_FILE: Self = Self(0xF);
    pub const WRITE_FILE: Self = Self(0x10);
    pub const GET_POS_FILE: Self = Self(0x11);
    pub const SET_POS_FILE: Self = Self(0x12);
    pub const IS_EOF: Self = Self(0x13);
    pub const STAT_FILE: Self = Self(0x14);
    pub const CLOSE_FILE: Self = Self(0x15);
    pub const GET_ERROR: Self = Self(0x16);
    pub const FLUSH_FILE: Self = Self(0x17);
    pub const GET_INFO_BY_QUERY: Self = Self(0x18);
    pub const APPEND_FILE: Self = Self(0x19);
    pub const TRUNCATE_FILE: Self = Self(0x1A);
    pub const FLUSH_VOLUME: Self = Self(0x1B);
    pub const ROLLBACK_VOLUME: Self = Self(0x1C);
    pub const MAKE_QUOTA: Self = Self(0x1D);
    pub const FLUSH_QUOTA: Self = Self(0x1E);
    pub const ROLLBACK_QUOTA: Self = Self(0x1F);
    pub const CHANGE_MODE: Self = Self(0x20);
    pub const OPEN_FILE_BY_STAT: Self = Self(0x21);
    pub const REGISTER_FLUSH_QUOTA: Self = Self(0x22);
    pub const FLUSH_MULTI_QUOTA: Self = Self(0x23);
    pub const GET_FILE_BLOCK_ADDRESS: Self = Self(0x25);
    pub const ADD_USER_PROCESS: Self = Self(0x65);
    pub const DEL_USER_PROCESS: Self = Self(0x66);
    pub const MOUNT_WITH_PROCESS: Self = Self(0x67);
    pub const UNMOUNT_WITH_PROCESS: Self = Self(0x68);
    pub const FORMAT: Self = Self(0x69);
    pub const RAW_OPEN: Self = Self(0x6A);
    pub const RAW_READ: Self = Self(0x6B);
    pub const RAW_WRITE: Self = Self(0x6C);
    pub const RAW_CLOSE: Self = Self(0x6D);
    pub const GET_LAST_FAILED_VOLUME: Self = Self(0x6E);
    pub const GET_VOLUME_EXISTENCE: Self = Self(0x6F);
    pub const CHANGE_OWNER: Self = Self(0x70);
    pub const CANCEL_GET_ATTACH: Self = Self(0x71);
    pub const REMOVE_QUOTA: Self = Self(0x72);
    pub const SET_CLIENT_PRIORITY: Self = Self(0x73);
    pub const APPLY_MEMORY_CACHE: Self = Self(0x74);
    pub const MAKE_LINK: Self = Self(0x75);
    pub const XFER_PARAMS: Self = Self(0x76);
    pub const EXEC_DEBUG_PROC: Self = Self(0x78);
    pub const DEBUG_SET_TITLE_ID: Self = Self(0x79);
    pub const DEBUG_SET_CAPABILITY: Self = Self(0x7A);
    pub const SET_PROCESS_CONFIG: Self = Self(0x82);
    pub const CONFIG_SET_MEMORY_CACHE: Self = Self(0x83);
    pub const CONFIG_UNSET_MEMORY_CACHE: Self = Self(0x84);
    pub const CONFIG_SET_PRF2_CHAR_CODE: Self = Self(0x85);
    pub const GET_PROC_RESOURCE_USAGE: Self = Self(0x8C);
    pub const GET_ALL_RESOURCE_USAGE: Self = Self(0x8D);
    pub const SEND_PROFILE_CMD: Self = Self(0x8E);
}

/// FSAReadFlag
#[doc(alias = "FSAReadFlag")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadFlag(pub u32);

impl ReadFlag {
    pub const NONE: Self = Self(0);
    pub const READ_WITH_POS: Self = Self(1);
}

/// FSAWriteFlag
#[doc(alias = "FSAWriteFlag")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WriteFlag(pub u32);

impl WriteFlag {
    pub const NONE: Self = Self(0);
    pub const WRITE_WITH_POS: Self = Self(1);
}

/// FSAMountFlags
#[doc(alias = "FSAMountFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MountFlags(pub u32);

impl MountFlags {
    pub const LOCAL_MOUNT: Self = Self(0);
    pub const BIND_MOUNT: Self = Self(1);
    pub const GLOBAL_MOUNT: Self = Self(2);
}

/// FSAUnmountFlags
#[doc(alias = "FSAUnmountFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnmountFlags(pub u32);

impl UnmountFlags {
    pub const NONE: Self = Self(0);
    pub const FORCE: Self = Self(0x2);
    pub const BIND_MOUNT: Self = Self(0x8000_0000);
}

impl core::ops::BitOr for UnmountFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// FSABlockInfo
#[doc(alias = "FSABlockInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockInfo(pub ffi::unknown<0x14>);

/// FSADeviceInfo
#[doc(alias = "FSADeviceInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceInfo {
    _unk0: ffi::unknown<0x08>,
    pub device_size_in_sectors: u64,
    pub device_sector_size: u32,
    _unk1: ffi::unknown<0x14>,
}

/// FSAFileSystemInfo
#[doc(alias = "FSAFileSystemInfo")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileSystemInfo(pub ffi::unknown<0x1E>);

/// FSARequest
///
/// IPC request buffer of a command.
#[doc(alias = "FSARequest")]
#[repr(C)]
pub struct Request {
    _private: [u8; 0],
}

/// FSAResponse
///
/// IPC response buffer of a command.
#[doc(alias = "FSAResponse")]
#[repr(C)]
pub struct Response {
    _private: [u8; 0],
}

/// FSAAsyncCallbackFn
#[doc(alias = "FSAAsyncCallbackFn", alias = "FSAClientAttachAsyncCallbackFn")]
pub type AsyncCallback = Option<
    unsafe extern "C" fn(
        result: Error,
        command: Command,
        request: *mut Request,
        response: *mut Response,
        user_context: *mut ffi::c_void,
    ),
>;

/// FSAAsyncResult
#[doc(alias = "FSAAsyncResult")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncResult {
    /// Receives [AsyncResult::msg] when the command completes.
    pub io_msg_queue: *mut MessageQueue,
    pub msg: Message,
    pub user_callback: AsyncCallback,
    pub error: Error,
    pub command: Command,
    pub request: *mut Request,
    pub response: *mut Response,
    pub user_context: *mut ffi::c_void,
}

/// FSAClientAttachAsyncData
#[doc(alias = "FSAClientAttachAsyncData")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientAttachAsyncData {
    pub user_callback: AsyncCallback,
    pub user_context: *mut ffi::c_void,
    pub io_msg_queue: *mut MessageQueue,
}

const _: () = {
    assert!(size_of::<BlockInfo>() == 0x14);
    assert!(size_of::<DeviceInfo>() == 0x28);
    assert!(core::mem::offset_of!(DeviceInfo, device_sector_size) == 0x10);
    assert!(size_of::<FileSystemInfo>() == 0x1E);
};

unsafe extern "C" {
    /// FSAInit
    #[doc(alias = "FSAInit")]
    #[link_name = "FSAInit"]
    pub unsafe fn init() -> Error;

    /// FSAShutdown
    #[doc(alias = "FSAShutdown")]
    #[link_name = "FSAShutdown"]
    pub unsafe fn shutdown();

    /// FSAGetClientNum
    #[doc(alias = "FSAGetClientNum")]
    #[link_name = "FSAGetClientNum"]
    pub unsafe fn get_client_num() -> u32;

    /// FSAAddClient
    ///
    /// `attach_async_data` may be null. Returns the client, or a negative [Error].
    #[doc(alias = "FSAAddClient")]
    #[link_name = "FSAAddClient"]
    pub unsafe fn add_client(attach_async_data: *mut ClientAttachAsyncData) -> ClientHandle;

    /// FSADelClient
    #[doc(alias = "FSADelClient")]
    #[link_name = "FSADelClient"]
    pub unsafe fn del_client(client: ClientHandle) -> Error;

    /// FSAGetStatusStr
    #[doc(alias = "FSAGetStatusStr")]
    #[link_name = "FSAGetStatusStr"]
    pub unsafe fn get_status_str(error: Error) -> *const ffi::c_char;

    /// __FSAShimDecodeIosErrorToFsaStatus
    #[doc(alias = "__FSAShimDecodeIosErrorToFsaStatus")]
    #[link_name = "__FSAShimDecodeIosErrorToFsaStatus"]
    pub unsafe fn shim_decode_ios_error_to_fsa_status(
        handle: ios::Handle,
        error: ios::Error,
    ) -> Error;

    /// FSAGetAsyncResult
    ///
    /// Returns the result carried by a completion message.
    #[doc(alias = "FSAGetAsyncResult")]
    #[link_name = "FSAGetAsyncResult"]
    pub unsafe fn get_async_result(message: *mut OsMessage) -> *mut AsyncResult;

    /// FSAFreeAsyncResult
    #[doc(alias = "FSAFreeAsyncResult")]
    #[link_name = "FSAFreeAsyncResult"]
    pub unsafe fn free_async_result(result: *mut AsyncResult);

    /// FSAGetTransactionBlockPoolAttributes
    #[doc(alias = "FSAGetTransactionBlockPoolAttributes")]
    #[link_name = "FSAGetTransactionBlockPoolAttributes"]
    pub unsafe fn get_transaction_block_pool_attributes(
        message_size: *mut u32,
        pool_size: *mut u32,
        num_messages: *mut u32,
    ) -> Error;

    /// FSAMount
    #[doc(alias = "FSAMount")]
    #[link_name = "FSAMount"]
    pub unsafe fn mount(
        client: ClientHandle,
        source: *const ffi::c_char,
        target: *const ffi::c_char,
        flags: MountFlags,
        arg_buf: *mut ffi::c_void,
        arg_len: u32,
    ) -> Error;

    /// FSAUnmount
    #[doc(alias = "FSAUnmount")]
    #[link_name = "FSAUnmount"]
    pub unsafe fn unmount(
        client: ClientHandle,
        mounted_target: *const ffi::c_char,
        flags: UnmountFlags,
    ) -> Error;

    /// FSAFlushVolume
    #[doc(alias = "FSAFlushVolume")]
    #[link_name = "FSAFlushVolume"]
    pub unsafe fn flush_volume(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSARollbackVolume
    #[doc(alias = "FSARollbackVolume")]
    #[link_name = "FSARollbackVolume"]
    pub unsafe fn rollback_volume(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSAGetVolumeInfo
    #[doc(alias = "FSAGetVolumeInfo")]
    #[link_name = "FSAGetVolumeInfo"]
    pub unsafe fn get_volume_info(
        client: ClientHandle,
        path: *const ffi::c_char,
        info: *mut VolumeInfo,
    ) -> Error;

    /// FSAMakeQuota
    #[doc(alias = "FSAMakeQuota")]
    #[link_name = "FSAMakeQuota"]
    pub unsafe fn make_quota(
        client: ClientHandle,
        name: *const ffi::c_char,
        mode: u32,
        quota: u64,
    ) -> Error;

    /// FSAFlushQuota
    #[doc(alias = "FSAFlushQuota")]
    #[link_name = "FSAFlushQuota"]
    pub unsafe fn flush_quota(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSAFlushMultiQuota
    #[doc(alias = "FSAFlushMultiQuota")]
    #[link_name = "FSAFlushMultiQuota"]
    pub unsafe fn flush_multi_quota(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSARegisterFlushQuota
    #[doc(alias = "FSARegisterFlushQuota")]
    #[link_name = "FSARegisterFlushQuota"]
    pub unsafe fn register_flush_quota(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSARollbackQuota
    #[doc(alias = "FSARollbackQuota")]
    #[link_name = "FSARollbackQuota"]
    pub unsafe fn rollback_quota(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSARollbackQuotaForce
    #[doc(alias = "FSARollbackQuotaForce")]
    #[link_name = "FSARollbackQuotaForce"]
    pub unsafe fn rollback_quota_force(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSAGetFreeSpaceSize
    #[doc(alias = "FSAGetFreeSpaceSize")]
    #[link_name = "FSAGetFreeSpaceSize"]
    pub unsafe fn get_free_space_size(
        client: ClientHandle,
        path: *const ffi::c_char,
        size: *mut u64,
    ) -> Error;

    /// FSAGetJournalFreeSpaceSize
    #[doc(alias = "FSAGetJournalFreeSpaceSize")]
    #[link_name = "FSAGetJournalFreeSpaceSize"]
    pub unsafe fn get_journal_free_space_size(
        client: ClientHandle,
        path: *const ffi::c_char,
        size: *mut u64,
    ) -> Error;

    /// FSAGetDirSize
    #[doc(alias = "FSAGetDirSize")]
    #[link_name = "FSAGetDirSize"]
    pub unsafe fn get_dir_size(
        client: ClientHandle,
        path: *const ffi::c_char,
        size: *mut u64,
    ) -> Error;

    /// FSAGetEntryNum
    #[doc(alias = "FSAGetEntryNum")]
    #[link_name = "FSAGetEntryNum"]
    pub unsafe fn get_entry_num(
        client: ClientHandle,
        path: *const ffi::c_char,
        entry_num: *mut u32,
    ) -> Error;

    /// FSAGetFileSystemInfo
    #[doc(alias = "FSAGetFileSystemInfo")]
    #[link_name = "FSAGetFileSystemInfo"]
    pub unsafe fn get_file_system_info(
        client: ClientHandle,
        path: *const ffi::c_char,
        info: *mut FileSystemInfo,
    ) -> Error;

    /// FSAGetDeviceInfo
    #[doc(alias = "FSAGetDeviceInfo")]
    #[link_name = "FSAGetDeviceInfo"]
    pub unsafe fn get_device_info(
        client: ClientHandle,
        path: *const ffi::c_char,
        info: *mut DeviceInfo,
    ) -> Error;

    /// FSAGetBadBlockInfo
    #[doc(alias = "FSAGetBadBlockInfo")]
    #[link_name = "FSAGetBadBlockInfo"]
    pub unsafe fn get_bad_block_info(
        client: ClientHandle,
        path: *const ffi::c_char,
        info: *mut BlockInfo,
    ) -> Error;

    /// FSAGetFragmentBlockInfo
    #[doc(alias = "FSAGetFragmentBlockInfo")]
    #[link_name = "FSAGetFragmentBlockInfo"]
    pub unsafe fn get_fragment_block_info(
        client: ClientHandle,
        path: *const ffi::c_char,
        info: *mut BlockInfo,
    ) -> Error;

    /// FSAChangeDir
    #[doc(alias = "FSAChangeDir")]
    #[link_name = "FSAChangeDir"]
    pub unsafe fn change_dir(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSAGetCwd
    #[doc(alias = "FSAGetCwd")]
    #[link_name = "FSAGetCwd"]
    pub unsafe fn get_cwd(client: ClientHandle, path: *mut ffi::c_char, path_len: u32) -> Error;

    /// FSAChangeMode
    #[doc(alias = "FSAChangeMode")]
    #[link_name = "FSAChangeMode"]
    pub unsafe fn change_mode(
        client: ClientHandle,
        path: *const ffi::c_char,
        permission: Mode,
    ) -> Error;

    /// FSAGetStat
    #[doc(alias = "FSAGetStat")]
    #[link_name = "FSAGetStat"]
    pub unsafe fn get_stat(
        client: ClientHandle,
        path: *const ffi::c_char,
        stat: *mut Stat,
    ) -> Error;

    /// FSARemove
    #[doc(alias = "FSARemove")]
    #[link_name = "FSARemove"]
    pub unsafe fn remove(client: ClientHandle, path: *const ffi::c_char) -> Error;

    /// FSARename
    #[doc(alias = "FSARename")]
    #[link_name = "FSARename"]
    pub unsafe fn rename(
        client: ClientHandle,
        old_path: *const ffi::c_char,
        new_path: *const ffi::c_char,
    ) -> Error;

    /// FSAMakeDir
    #[doc(alias = "FSAMakeDir")]
    #[link_name = "FSAMakeDir"]
    pub unsafe fn make_dir(client: ClientHandle, path: *const ffi::c_char, mode: Mode) -> Error;

    /// FSAOpenDir
    #[doc(alias = "FSAOpenDir")]
    #[link_name = "FSAOpenDir"]
    pub unsafe fn open_dir(
        client: ClientHandle,
        path: *const ffi::c_char,
        dir: *mut DirectoryHandle,
    ) -> Error;

    /// FSAReadDir
    #[doc(alias = "FSAReadDir")]
    #[link_name = "FSAReadDir"]
    pub unsafe fn read_dir(
        client: ClientHandle,
        dir: DirectoryHandle,
        entry: *mut DirectoryEntry,
    ) -> Error;

    /// FSARewindDir
    #[doc(alias = "FSARewindDir")]
    #[link_name = "FSARewindDir"]
    pub unsafe fn rewind_dir(client: ClientHandle, dir: DirectoryHandle) -> Error;

    /// FSACloseDir
    #[doc(alias = "FSACloseDir")]
    #[link_name = "FSACloseDir"]
    pub unsafe fn close_dir(client: ClientHandle, dir: DirectoryHandle) -> Error;

    /// FSAOpenFileEx
    ///
    /// `mode` is an fopen-style mode string, `prealloc_size` reserves space for new files.
    #[doc(alias = "FSAOpenFileEx")]
    #[link_name = "FSAOpenFileEx"]
    pub unsafe fn open_file_ex(
        client: ClientHandle,
        path: *const ffi::c_char,
        mode: *const ffi::c_char,
        create_mode: Mode,
        flags: OpenFileFlags,
        prealloc_size: u32,
        file: *mut FileHandle,
    ) -> Error;

    /// FSAOpenFileByStat
    #[doc(alias = "FSAOpenFileByStat")]
    #[link_name = "FSAOpenFileByStat"]
    pub unsafe fn open_file_by_stat(
        client: ClientHandle,
        stat: *mut Stat,
        mode: *const ffi::c_char,
        path: *const ffi::c_char,
        file: *mut FileHandle,
    ) -> Error;

    /// FSACloseFile
    #[doc(alias = "FSACloseFile")]
    #[link_name = "FSACloseFile"]
    pub unsafe fn close_file(client: ClientHandle, file: FileHandle) -> Error;

    /// FSAGetStatFile
    #[doc(alias = "FSAGetStatFile")]
    #[link_name = "FSAGetStatFile"]
    pub unsafe fn get_stat_file(client: ClientHandle, file: FileHandle, stat: *mut Stat) -> Error;

    /// FSAReadFile
    #[doc(alias = "FSAReadFile")]
    #[link_name = "FSAReadFile"]
    pub unsafe fn read_file(
        client: ClientHandle,
        buffer: *mut ffi::c_void,
        size: u32,
        count: u32,
        file: FileHandle,
        flags: ReadFlag,
    ) -> Error;

    /// FSAReadFileWithPos
    #[doc(alias = "FSAReadFileWithPos")]
    #[link_name = "FSAReadFileWithPos"]
    pub unsafe fn read_file_with_pos(
        client: ClientHandle,
        buffer: *mut ffi::c_void,
        size: u32,
        count: u32,
        pos: u32,
        file: FileHandle,
        flags: ReadFlag,
    ) -> Error;

    /// FSAWriteFile
    #[doc(alias = "FSAWriteFile")]
    #[link_name = "FSAWriteFile"]
    pub unsafe fn write_file(
        client: ClientHandle,
        buffer: *const ffi::c_void,
        size: u32,
        count: u32,
        file: FileHandle,
        flags: WriteFlag,
    ) -> Error;

    /// FSAWriteFileWithPos
    #[doc(alias = "FSAWriteFileWithPos")]
    #[link_name = "FSAWriteFileWithPos"]
    pub unsafe fn write_file_with_pos(
        client: ClientHandle,
        buffer: *const ffi::c_void,
        size: u32,
        count: u32,
        pos: u32,
        file: FileHandle,
        flags: WriteFlag,
    ) -> Error;

    /// FSAAppendFile
    #[doc(alias = "FSAAppendFile")]
    #[link_name = "FSAAppendFile"]
    pub unsafe fn append_file(
        client: ClientHandle,
        file: FileHandle,
        size: u32,
        count: u32,
    ) -> Error;

    /// FSAAppendFileEx
    #[doc(alias = "FSAAppendFileEx")]
    #[link_name = "FSAAppendFileEx"]
    pub unsafe fn append_file_ex(
        client: ClientHandle,
        file: FileHandle,
        size: u32,
        count: u32,
        flags: u32,
    ) -> Error;

    /// FSAGetPosFile
    #[doc(alias = "FSAGetPosFile")]
    #[link_name = "FSAGetPosFile"]
    pub unsafe fn get_pos_file(client: ClientHandle, file: FileHandle, pos: *mut u32) -> Error;

    /// FSASetPosFile
    #[doc(alias = "FSASetPosFile")]
    #[link_name = "FSASetPosFile"]
    pub unsafe fn set_pos_file(client: ClientHandle, file: FileHandle, pos: u32) -> Error;

    /// FSAIsEof
    #[doc(alias = "FSAIsEof")]
    #[link_name = "FSAIsEof"]
    pub unsafe fn is_eof(client: ClientHandle, file: FileHandle) -> Error;

    /// FSAFlushFile
    #[doc(alias = "FSAFlushFile")]
    #[link_name = "FSAFlushFile"]
    pub unsafe fn flush_file(client: ClientHandle, file: FileHandle) -> Error;

    /// FSATruncateFile
    #[doc(alias = "FSATruncateFile")]
    #[link_name = "FSATruncateFile"]
    pub unsafe fn truncate_file(client: ClientHandle, file: FileHandle) -> Error;
}
