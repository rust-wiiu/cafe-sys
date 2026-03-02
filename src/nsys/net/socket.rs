//! socket

use crate::ffi;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(i32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[non_exhaustive]
pub enum Error {
    // Success = 0,
    InsufficientResources = 1,
    TimedOut = 2,
    AlreadyConnected = 3,
    OperationNotSupported = 4,
    ConnectionAborted = 5,
    WouldBlock = 6,
    ConnectionRefused = 7,
    ConnectionReset = 8,
    NotConnected = 9,
    AlreadyInProgress = 10,
    InvalidOperation = 11,
    MessageTooLarge = 12,
    BrokenPipe = 13,
    DestinationAddressRequired = 14,
    Shutdown = 15,
    ProtocolOptionNotSupported = 16,
    OutOfBandDataPending = 17,
    InsufficientMemory = 18,
    AddressNotAvailable = 19,
    AddressInUse = 20,
    AddressFamilyNotSupported = 21,
    InProgress = 22,
    IpLayerError = 23,
    NotASocket = 24,
    IoError = 27,
    TooManyReferences = 28,
    BadAddress = 29,
    NetworkUnreachable = 30,
    ProtocolNotSupported = 31,
    ProtocolTypeMismatch = 32,

    GenericError = 41,
    FailedToOpenResourceManager = 42,
    LibraryNotInitialized = 43,
    Busy = 44,
    Unknown = 45,
    InternalApiError = 46,
    InvalidErrorCode = 47,
    NoResources = 48,
    BadFileDescriptor = 49,
    Aborted = 50,
    TooManySockets = 51,

    IcmpDestinationUnreachable = 100,
    IcmpSourceQuench = 101,
    IcmpRedirect = 102,
    IcmpTimeExceeded = 103,
    IcmpParameterProblem = 104,
}

pub type FileDescriptor = i32;
pub type IPv4Address = u32;

#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    #[default]
    /// AF_INET
    IPv4 = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// SOCK_STREAM
    Stream = 1,
    /// SOCK_DGRAM
    Datagram = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    #[doc(alias = "IPPROTO_IP")]
    Ip = 0,
    #[doc(alias = "IPPROTO_TCP")]
    Tcp = 6,
    #[doc(alias = "IPPROTO_UDP")]
    Udp = 17,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Address {
    pub family: Family,
    pub port: u16,
    pub address: IPv4Address,
    pub zero: [u8; 8],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Shutdown {
    /// Reads disallowed; write-only.
    Read = 0,
    /// Writes disallowed; read-only.
    Write = 1,
    /// Reads and writes disallowed.
    ReadWrite = 2,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    #[repr(transparent)]
    pub struct Flags : i32 {
        /// Process out-of-band data (supported by [SocketType::Stream]).
        const OutOfBand = 0x01;
        /// Peek at incoming message i.e. do not remove it from buffer.
        const Peek = 0x02;
        /// Do not use a gateway / routing table to send the packet.
        const DontRoute = 0x04;
        /// Create a new pipe for [recvfrom] call.
        const NewPipe = 0x08;
        /// Data completes a record (if supported by socket).
        const CompletesRecord = 0x10;
        /// Do not block thread if no data is available.
        const NonBlocking = 0x20;
        /// Recieve packet TTL.
        const RecieveTtl = 0x40;

    }
}

// #[cfg(target_arch = "powerpc")]
unsafe extern "C" {
    /// Initializes the socket library.
    #[doc(alias = "socket_lib_init")]
    #[link_name = "socket_lib_init"]
    pub unsafe fn init() -> i32;

    /// Deinitializes the socket library.
    #[doc(alias = "socket_lib_finish")]
    #[link_name = "socket_lib_finish"]
    pub unsafe fn deinit() -> i32;

    #[doc(alias = "socketlasterr")]
    #[link_name = "socketlasterr"]
    pub unsafe fn last_error() -> Error;

    /// Accept a pending connection on a socket.
    #[link_name = "accept"]
    pub unsafe fn accept(
        fd: FileDescriptor,
        addr: *mut Address,
        addrlen: *mut i32,
    ) -> FileDescriptor;

    /// Assign an address to a socket.
    #[link_name = "bind"]
    pub unsafe fn bind(fd: FileDescriptor, addr: *const Address, addrlen: usize) -> i32;

    /// Initiaite a connection on a socket.
    #[link_name = "connect"]
    pub unsafe fn connect(fd: FileDescriptor, addr: *const Address, addrlen: usize) -> i32;

    /// Listen for connections on a socket. Only applies to [SocketType::Stream] sockets.
    #[link_name = "listen"]
    pub unsafe fn listen(fd: FileDescriptor, backlog: i32) -> i32;

    /// Receive data on socket connection
    ///
    /// Identical to [recvfrom] with a null pointer as `from`.
    ///
    /// Returns number of bytes received on success. Returns `-1` on failure and sets [last_error] appropriately. Returns `0` when the peer has performaned an orderly shutdown.
    #[link_name = "recv"]
    pub unsafe fn recv(fd: FileDescriptor, buffer: *mut ffi::c_void, len: u32, flags: Flags)
    -> i32;

    /// Receive data on socket connection
    ///
    /// For [SocketType::Datagram], it returns the same address as [get_peername]. For [SocketType::Stream], it does not return the address of the sender. Identical to [recv] with a null pointer as `from`.
    ///
    /// Returns number of bytes received on success. Returns `-1` on failure and sets [last_error] appropriately. Returns `0` when the peer has performaned an orderly shutdown.
    #[link_name = "recvfrom"]
    pub unsafe fn recvfrom(
        fd: FileDescriptor,
        buffer: *mut ffi::c_void,
        len: u32,
        flags: Flags,
        from: *const Address,
        fromlen: usize,
    ) -> i32;

    /// Send data on a socket connection.
    ///
    /// This fuction will block if no message space is available on the socket. Identical to [sendto] with a null pointer as `dest_addr`.
    ///
    /// Returns number of bytes sent on success. Returns `-1` on failure and sets [last_error] appropriately.
    #[link_name = "send"]
    pub unsafe fn send(
        fd: FileDescriptor,
        buffer: *const ffi::c_void,
        len: u32,
        flags: Flags,
    ) -> i32;

    /// Send data on a socket connection to a specified address.
    ///
    /// This fuction will block if no message space is available on the socket. Identical to [send] with a null pointer as `dest_addr`.
    ///
    /// Returns number of bytes sent on success. Returns `-1` on failure and sets [last_error] appropriately.
    #[link_name = "sendto"]
    pub unsafe fn sendto(
        fd: FileDescriptor,
        buffer: *const ffi::c_void,
        len: u32,
        flags: Flags,
        dest: *const Address,
        destlen: usize,
    ) -> i32;

    /// Shut down all or part of a full-duplex connection on the specified socket.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    #[link_name = "shutdown"]
    pub unsafe fn shutdown(fd: FileDescriptor, how: Shutdown) -> i32;

    /// Creates a communication endpoint and returns a descriptor for that endpoint.
    #[link_name = "socket"]
    pub unsafe fn socket(family: Family, r#type: Type, proto: Protocol) -> FileDescriptor;

    /// Closes the specified socket.
    ///
    /// The a thread is currently blocking on the socket in another thread, the blocked call will return with an error code.
    #[link_name = "socketclose"]
    pub unsafe fn close(fd: FileDescriptor) -> i32;

    /// Retrieve the address of the peer connected to the specified socket.
    #[link_name = "getpeername"]
    pub unsafe fn get_peername(fd: FileDescriptor, addr: *mut Address, addrlen: usize) -> i32;

    /// Retrieves the address for the specified socket.
    #[link_name = "getsockname"]
    pub unsafe fn get_sockname(fd: FileDescriptor, addr: *mut Address, addrlen: usize) -> i32;
}
