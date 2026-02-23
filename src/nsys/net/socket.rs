//! socket

use crate::ffi;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(i32)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
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

pub const FAILURE: i32 = -1;

/// Raw file descriptor
///
/// A file descriptor is an identifier to access resources managed by the OS. See more: [File descriptor](wikipedia.org/wiki/File_descriptor).
///
/// # Safety
///
/// Can be `-1` to indicate the error invariant. Use [last_error] to get more information.
pub type RawFd = i32;

pub type IPv4Address = u32;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SocketFamily {
    /// AF_INET
    IPv4 = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    /// SOCK_STREAM
    Stream = 1,
    /// SOCK_DGRAM
    Datagram = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SocketProtocol {
    /// IPPROTO_IP
    Ip = 0,
    /// IPPROTO_TCP
    Tcp = 6,
    /// IPPROTO_UDP
    Udp = 17,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SocketAddress {
    pub family: u16,
    pub port: u16,
    pub address: IPv4Address,
    zero: [u8; 8],
}

impl SocketAddress {
    pub const fn new(family: SocketFamily, port: u16, address: IPv4Address) -> Self {
        Self {
            family: family as u16,
            port: port,
            address: address,
            zero: [0; 8],
        }
    }
}

impl From<core::net::SocketAddrV4> for SocketAddress {
    fn from(value: core::net::SocketAddrV4) -> Self {
        Self {
            family: SocketFamily::IPv4 as u16,
            port: value.port(),
            address: value.ip().to_bits(),
            zero: [0; 8],
        }
    }
}

impl TryFrom<core::net::SocketAddr> for SocketAddress {
    type Error = ();

    fn try_from(value: core::net::SocketAddr) -> Result<Self, Self::Error> {
        match value {
            core::net::SocketAddr::V4(addr) => Ok(Self::from(addr)),
            core::net::SocketAddr::V6(_) => Err(()),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RecvMultiBuffers {
    buffer: *mut ffi::c_void,
    bufferlen: u32,
    froms: *mut SocketAddress,
    fromslen: u32,
    results: *mut i32,
    resultslen: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SendToMultiBuffers {
    buffer: *mut ffi::c_void,
    bufferlen: u32,
    datagram_lens: *mut i32,
    datagram_lens_len: u32,
    dests: *mut SocketAddress,
    destslen: u32,
    results: *mut i32,
    resultslen: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Timeval {
    tv_sec: u32,
    tv_usec: u32,
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FdSet {
    bits: u32,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    #[repr(transparent)]
    pub struct SocketFlags : i32 {
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
    ///
    /// This function **must** be called before any socket related functions can be used. (Can be called multiple times?)
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// socket_lib_init
    #[link_name = "socket_lib_init"]
    pub unsafe fn init() -> i32;

    /// Deinitializes the socket library.
    ///
    /// All calls to socket related functions are invalid after this function; they might lead to erros or undefined behavior.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// socket_lib_finish
    #[link_name = "socket_lib_finish"]
    pub unsafe fn deinit() -> i32;

    /// socketlasterr
    #[link_name = "socketlasterr"]
    pub unsafe fn last_error() -> i32;

    /// Accept a pending connection on a socket.
    ///
    /// The socket must be setup first using [socket], [bind], and [listen]. The `addr` parameter will be filled with the accepted connection on success. If the socket is marked as non-blocking, the fuction will return with an error if no connections are pending, otherwise it will block until one comes in. Used with [SocketType::Stream] sockets.
    ///
    /// Returns [RawFd] on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// accept
    #[link_name = "accept"]
    pub unsafe fn accept(fd: RawFd, addr: *mut SocketAddress, addrlen: *mut i32) -> i32;

    /// Assign an address to a socket.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// bind
    #[link_name = "bind"]
    pub unsafe fn bind(fd: RawFd, addr: *mut SocketAddress, addrlen: *mut i32) -> i32;

    /// connect
    #[link_name = "connect"]
    pub unsafe fn connect(fd: RawFd, addr: *mut SocketAddress, addrlen: *mut i32) -> i32;

    /// Listen for conenctions on a socket. Only applies to [SocketType::Stream] sockets.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// listen
    #[link_name = "listen"]
    pub unsafe fn listen(fd: RawFd, backlog: i32) -> i32;

    /// Receive data on socket connection
    ///
    /// Identical to [recvfrom] with a null pointer as `from`.
    ///
    /// Returns number of bytes received on success. Returns `-1` on failure and sets [last_error] appropriately. Returns `0` when the peer has performaned an orderly shutdown.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// recv
    #[link_name = "recv"]
    pub unsafe fn recv(fd: RawFd, buffer: *mut ffi::c_void, len: i32, flags: i32) -> i32;

    /// Receive data on socket connection
    ///
    /// For [SocketType::Datagram], it returns the same address as [get_peername]. For [SocketType::Stream], it does not return the address of the sender. Identical to [recv] with a null pointer as `from`.
    ///
    /// Returns number of bytes received on success. Returns `-1` on failure and sets [last_error] appropriately. Returns `0` when the peer has performaned an orderly shutdown.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// recvfrom
    #[link_name = "recvfrom"]
    pub unsafe fn recvfrom(
        fd: RawFd,
        buffer: *mut ffi::c_void,
        len: i32,
        flags: i32,
        from: *mut SocketAddress,
        fromlen: *mut i32,
    ) -> i32;

    // /// recvfrom_ex
    // #[link_name = "recvfrom_ex"]
    // pub unsafe fn recvfrom_ex(
    //     fd: RawFd,
    //     buffer: *mut ffi::c_void,
    //     len: i32,
    //     flags: i32,
    //     from: *mut SocketAddress,
    //     fromlen: *mut i32,
    //     msg: *mut ffi::c_char,
    //     msglen: i32,
    // ) -> i32;

    // /// recvfrom_multi
    // #[link_name = "recvfrom_multi"]
    // pub unsafe fn recvfrom_multi(
    //     fd: RawFd,
    //     flags: i32,
    //     buffs: *mut RecvMultiBuffers,
    //     recv_datagram_len: i32,
    //     recv_datagram_count: i32,
    //     timeout: *mut Timeval,
    // ) -> i32;

    /// Send data on a socket connection.
    ///
    /// This fuction will block if no message space is available on the socket. Identical to [sendto] with a null pointer as `dest_addr`.
    ///
    /// Returns number of bytes sent on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// send
    #[link_name = "send"]
    pub unsafe fn send(fd: RawFd, buffer: *const ffi::c_void, len: i32, flags: SocketFlags) -> i32;

    /// Send data on a socket connection to a specified address.
    ///
    /// This fuction will block if no message space is available on the socket. Identical to [send] with a null pointer as `dest_addr`.
    ///
    /// Returns number of bytes sent on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// sendto
    #[link_name = "sendto"]
    pub unsafe fn sendto(
        fd: RawFd,
        buffer: *const ffi::c_void,
        len: i32,
        flags: i32,
        dest_addr: *const SocketAddress,
        dest_len: i32,
    ) -> i32;

    // /// sendto_multi
    // #[link_name = "sendto_multi"]
    // pub unsafe fn sendto_multi(
    //     fd: RawFd,
    //     buffer: *const ffi::c_void,
    //     len: i32,
    //     flags: i32,
    //     dest_addr: *const SocketAddress,
    //     dest_count: i32,
    // ) -> i32;

    // /// sendto_multi_ex
    // #[link_name = "sendto_multi_ex"]
    // pub unsafe fn sendto_multi_ex(
    //     fd: RawFd,
    //     flags: i32,
    //     buffs: *mut SendToMultiBuffers,
    //     send_datagram_count: i32,
    // ) -> i32;

    /// Shut down all or part of a full-duplex connection on the specified socket.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// shutdown
    #[link_name = "shutdown"]
    pub unsafe fn shutdown(fd: RawFd, how: Shutdown) -> i32;

    /// Creates a communication endpoint and returns a descriptor for that endpoint.
    ///
    /// Returns [RawFd] on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// socket
    #[link_name = "socket"]
    pub unsafe fn socket(family: SocketFamily, r#type: SocketType, proto: SocketProtocol) -> RawFd;

    /// Closes the specified socket.
    ///
    /// The a thread is currently blocking on the socket in another thread, the blocked call will return with an error code.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// socketclose
    #[link_name = "socketclose"]
    pub unsafe fn close(fd: RawFd) -> i32;

    // /// inet_aton
    // #[link_name = "inet_aton"]
    // pub unsafe fn inet_aton(cp: *const ffi::c_char, addr: *mut IPv4Address) -> i32;

    // /// inet_ntoa_r
    // #[link_name = "inet_ntoa_r"]
    // pub unsafe fn inet_ntoa_r(r#in: IPv4Address, buf: *mut ffi::c_char) -> *mut ffi::c_char;

    // /// inet_ntop
    // #[link_name = "inet_ntop"]
    // pub unsafe fn inet_ntop(
    //     af: i32,
    //     src: *const ffi::c_void,
    //     dst: *mut ffi::c_char,
    //     size: u32,
    // ) -> *const ffi::c_char;

    // /// inet_pton
    // #[link_name = "inet_pton"]
    // pub unsafe fn inet_pton(af: i32, src: *const ffi::c_char, dst: *mut ffi::c_void) -> i32;

    /// Retrieve the address of the peer connected to the specified socket.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// getpeername
    #[link_name = "getpeername"]
    pub unsafe fn get_peername(fd: RawFd, name: *mut SocketAddress, addrlen: *mut i32) -> i32;

    /// Retrieves the address for the specified socket.
    ///
    /// Returns `0` on success. Returns `-1` on failure and sets [last_error] appropriately.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// # Symbol
    ///
    /// getsockname
    #[link_name = "getsockname"]
    pub unsafe fn get_sockname(fd: RawFd, name: *mut SocketAddress, addrlen: *mut i32) -> i32;

    // /// getsockopt
    // #[link_name = "getsockopt"]
    // pub unsafe fn getsockopt(
    //     fd: RawFd,
    //     level: i32,
    //     optname: i32,
    //     optval: *mut ffi::c_void,
    //     optlen: *mut u32,
    // ) -> i32;

    // /// setsockopt
    // #[link_name = "setsockopt"]
    // pub unsafe fn setsockopt(
    //     fd: RawFd,
    //     level: i32,
    //     optname: i32,
    //     optval: *mut ffi::c_void,
    //     optlen: u32,
    // ) -> i32;

    // /// setsocketlasterr
    // #[link_name = "setsocketlasterr"]
    // pub unsafe fn set_last_error(err: i32);

    // /// select
    // #[link_name = "select"]
    // pub unsafe fn select(
    //     nfds: i32,
    //     readfds: *mut FdSet,
    //     writefds: *mut FdSet,
    //     exceptfds: *mut FdSet,
    //     timeout: *mut Timeval,
    // ) -> i32;

    // /// setsocklibopt
    // #[link_name = "setsocklibopt"]
    // pub unsafe fn setsocklibopt(liboptname: i32, optval: i32) -> i32;

    // /// getsocklibopt
    // #[link_name = "getsocklibopt"]
    // pub unsafe fn getsocklibopt(liboptname: i32, optval: *mut i32) -> i32;

    // /// somemopt
    // #[link_name = "somemopt"]
    // pub unsafe fn somemopt(req_type: i32, mem: *mut ffi::c_char, memlen: u32, flags: i32) -> i32;

}
