//! socket
//!
//! BSD style sockets (IPv4 only) and name resolution.
//!
//! Functions return `-1` on failure, use [last_error] to get the [Error].

use crate::ffi;

pub type FileDescriptor = i32;

/// socklen_t
#[doc(alias = "socklen_t")]
pub type SockLen = i32;

/// SO_* error codes
#[doc(alias = "SO_ERROR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const ENOBUFS: Self = Self(1);
    pub const ETIMEDOUT: Self = Self(2);
    pub const EISCONN: Self = Self(3);
    pub const EOPNOTSUPP: Self = Self(4);
    pub const ECONNABORTED: Self = Self(5);
    pub const EWOULDBLOCK: Self = Self(6);
    pub const ECONNREFUSED: Self = Self(7);
    pub const ECONNRESET: Self = Self(8);
    pub const ENOTCONN: Self = Self(9);
    pub const EALREADY: Self = Self(10);
    pub const EINVAL: Self = Self(11);
    pub const EMSGSIZE: Self = Self(12);
    pub const EPIPE: Self = Self(13);
    pub const EDESTADDRREQ: Self = Self(14);
    pub const ESHUTDOWN: Self = Self(15);
    pub const ENOPROTOOPT: Self = Self(16);
    pub const EHAVEOOB: Self = Self(17);
    pub const ENOMEM: Self = Self(18);
    pub const EADDRNOTAVAIL: Self = Self(19);
    pub const EADDRINUSE: Self = Self(20);
    pub const EAFNOSUPPORT: Self = Self(21);
    pub const EINPROGRESS: Self = Self(22);
    pub const ELOWER: Self = Self(23);
    pub const ENOTSOCK: Self = Self(24);
    pub const EIEIO: Self = Self(27);
    pub const ETOOMANYREFS: Self = Self(28);
    pub const EFAULT: Self = Self(29);
    pub const ENETUNREACH: Self = Self(30);
    pub const EPROTONOSUPPORT: Self = Self(31);
    pub const EPROTOTYPE: Self = Self(32);
    /// Generic error.
    #[doc(alias = "SO_ERROR")]
    pub const GENERIC: Self = Self(41);
    pub const ENOLIBRM: Self = Self(42);
    /// The library is not initialized.
    pub const ELIBNOTREADY: Self = Self(43);
    pub const EBUSY: Self = Self(44);
    pub const EUNKNOWN: Self = Self(45);
    pub const EAPIERROR: Self = Self(46);
    pub const ERANGEINVALID: Self = Self(47);
    pub const ENORESOURCES: Self = Self(48);
    pub const EBADFD: Self = Self(49);
    pub const EABORTED: Self = Self(50);
    /// Too many sockets.
    pub const EMFILE: Self = Self(51);
    pub const ICMP_DESTINATION_UNREACHABLE: Self = Self(100);
    pub const ICMP_SOURCE_QUENCH: Self = Self(101);
    pub const ICMP_REDIRECT: Self = Self(102);
    pub const ICMP_TIME_EXCEEDED: Self = Self(103);
    pub const ICMP_PARAMETER_PROBLEM: Self = Self(104);
}

/// AF_*
#[doc(alias = "AF")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Family(pub i32);

impl Family {
    pub const UNSPEC: Self = Self(0);
    pub const INET: Self = Self(2);
}

/// SOCK_*
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Type(pub i32);

impl Type {
    pub const STREAM: Self = Self(1);
    #[doc(alias = "SOCK_DGRAM")]
    pub const DATAGRAM: Self = Self(2);
}

/// IPPROTO_*
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Protocol(pub i32);

impl Protocol {
    pub const IP: Self = Self(0);
    pub const TCP: Self = Self(6);
    pub const UDP: Self = Self(17);
}

/// Option level for [get_sockopt] and [set_sockopt].
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    #[doc(alias = "SOL_SOCKET")]
    Socket = -1,
    Ip = 0,
    Tcp = 6,
    Udp = 17,
}

/// Option for [get_sockopt] and [set_sockopt].
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SockOpt {
    // Level::SOCKET
    Reuseaddr = 0x00004,
    Keepalive = 0x00008,
    Dontroute = 0x00010,
    Broadcast = 0x00020,
    /// Takes a [Linger].
    Linger = 0x00080,
    Oobinline = 0x00100,
    Tcpsack = 0x00200,
    Winscale = 0x00400,
    Hdrincl = 0x02000,
    Rusrbuf = 0x10000,
    Sndbuf = 0x1001,
    Rcvbuf = 0x1002,
    Lasterror = 0x1007,
    Type = 0x1008,
    Hopcnt = 0x1009,
    Maxmsg = 0x1010,
    Rxdata = 0x1011,
    Txdata = 0x1012,
    Myaddr = 0x1013,
    /// Non-blocking IO.
    Nbio = 0x1014,
    /// Blocking IO.
    Bio = 0x1015,
    Nonblock = 0x1016,
    Keepidle = 0x1019,
    Keepintvl = 0x101A,
    Keepcnt = 0x101B,
    // Level::IP
    IpTos = 3,
    IpMulticastIf = 9,
    IpMulticastTtl = 10,
    IpMulticastLoop = 11,
    IpAddMembership = 12,
    IpDropMembership = 13,
    // Level::TCP
    TcpAckdelaytime = 0x2001,
    TcpNoackdelay = 0x2002,
    TcpMaxseg = 0x2003,
    TcpNodelay = 0x2004,
    TcpAckfrequency = 0x2005,
}

impl SockOpt {
    pub const IP_TTL: Self = Self::Reuseaddr;
}

/// MSG_*
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MsgFlags(pub i32);

impl MsgFlags {
    pub const NONE: Self = Self(0);
    /// Out-of-band data.
    pub const OOB: Self = Self(0x01);
    /// Don't remove the data from the receive buffer.
    pub const PEEK: Self = Self(0x02);
    /// Don't route via a gateway.
    pub const DONTROUTE: Self = Self(0x04);
    pub const NEWPIPE: Self = Self(0x08);
    /// The data completes a record.
    pub const EOR: Self = Self(0x10);
    /// Don't block.
    pub const DONTWAIT: Self = Self(0x20);
    pub const IP_RECVTTL: Self = Self(0x40);
}

impl core::ops::BitOr for MsgFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// SHUT_*
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shutdown {
    /// Disallow reads.
    Read = 0,
    /// Disallow writes.
    Write = 1,
    ReadWrite = 2,
}

/// SOLIB_OPT_*
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LibOpt {
    AvoidCloseAll = 0x3000,
    SockfdTblInitCount = 0x3001,
    /// Takes one of the `SOCKBUF_CONSUMPTION_*` values.
    SockbufConsumption = 0x3002,
}

pub const SOCKBUF_CONSUMPTION_LOW: i32 = 0x01;
pub const SOCKBUF_CONSUMPTION_MED: i32 = 0x02;
pub const SOCKBUF_CONSUMPTION_HIGH: i32 = 0x04;
pub const SOCKBUF_CONSUMPTION_MAX: i32 = 0x08;

pub const INADDR_ANY: u32 = 0;

/// Number of sockets an [FdSet] can hold.
pub const FD_SETSIZE: usize = 32;

/// in_addr
#[doc(alias = "in_addr")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InAddr {
    /// Network byte order.
    pub addr: u32,
}

/// sockaddr
#[doc(alias = "sockaddr")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SockAddr {
    pub family: u16,
    pub data: [u8; 14],
}

/// sockaddr_in
///
/// Same size as [SockAddr], cast the pointer to pass it.
#[doc(alias = "sockaddr_in")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SockAddrIn {
    pub family: u16,
    /// Network byte order.
    pub port: u16,
    pub addr: InAddr,
    pub zero: [u8; 8],
}

/// linger
#[doc(alias = "linger")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Linger {
    pub onoff: ffi::c_int,
    /// Seconds.
    pub linger: ffi::c_int,
}

/// timeval
#[doc(alias = "timeval")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeVal {
    pub sec: ffi::c_long,
    pub usec: ffi::c_long,
}

/// fd_set
///
/// One bit per socket, see [FD_SETSIZE].
#[doc(alias = "fd_set")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FdSet {
    pub bits: ffi::c_ulong,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecvFromMultiBuffers {
    pub buf: *mut ffi::c_void,
    pub buf_len: ffi::c_uint,
    pub froms: *mut SockAddr,
    pub froms_len: ffi::c_uint,
    pub results: *mut ffi::c_int,
    pub results_len: ffi::c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SendToMultiExBuffers {
    pub buf: *mut ffi::c_void,
    pub buf_len: ffi::c_uint,
    pub datagram_lens: *mut ffi::c_int,
    pub datagram_lens_len: ffi::c_uint,
    pub dests: *mut SockAddr,
    pub dests_len: ffi::c_uint,
    pub results: *mut ffi::c_int,
    pub results_len: ffi::c_uint,
}

/// hostent
#[doc(alias = "hostent")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostEnt {
    pub name: *mut ffi::c_char,
    pub aliases: *mut *mut ffi::c_char,
    pub addr_type: ffi::c_int,
    pub length: ffi::c_int,
    pub addr_list: *mut *mut ffi::c_char,
}

/// AI_*
#[doc(alias = "AI")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiFlags(pub i32);

impl AiFlags {
    pub const NONE: Self = Self(0);
    pub const PASSIVE: Self = Self(0x01);
    pub const CANONNAME: Self = Self(0x02);
    pub const NUMERICHOST: Self = Self(0x04);
    pub const NUMERICSERV: Self = Self(0x08);
}

impl core::ops::BitOr for AiFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// addrinfo
#[doc(alias = "addrinfo")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddrInfo {
    pub flags: AiFlags,
    pub family: Family,
    pub sock_type: Type,
    pub protocol: Protocol,
    pub addr_len: SockLen,
    pub canon_name: *mut ffi::c_char,
    pub addr: *mut SockAddr,
    pub next: *mut AddrInfo,
}

/// EAI_*
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiError(pub i32);

impl AiError {
    pub const SUCCESS: Self = Self(0);
    pub const ADDRFAMILY: Self = Self(1);
    pub const AGAIN: Self = Self(2);
    pub const BADFLAGS: Self = Self(3);
    pub const FAIL: Self = Self(4);
    pub const FAMILY: Self = Self(5);
    pub const MEMORY: Self = Self(6);
    pub const NODATA: Self = Self(7);
    pub const NONAME: Self = Self(8);
    pub const SERVICE: Self = Self(9);
    pub const SOCKTYPE: Self = Self(10);
    pub const SYSTEM: Self = Self(11);
    pub const BADHINTS: Self = Self(12);
    pub const PROTOCOL: Self = Self(13);
    pub const OVERFLOW: Self = Self(14);
    pub const INPROGRESS: Self = Self(15);
}

/// NI_*
#[doc(alias = "NI")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NiFlags(pub i32);

impl NiFlags {
    pub const NONE: Self = Self(0);
    pub const NOFQDN: Self = Self(0x01);
    pub const NUMERICHOST: Self = Self(0x02);
    pub const NAMEREQD: Self = Self(0x04);
    pub const NUMERICSERV: Self = Self(0x08);
    pub const DGRAM: Self = Self(0x10);
}

impl core::ops::BitOr for NiFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const NI_MAXHOST: usize = 1025;
pub const NI_MAXSERV: usize = 32;

/// h_errno values
#[doc(alias = "h_errno")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostError(pub i32);

impl HostError {
    #[doc(alias = "NETDB_INTERNAL")]
    pub const INTERNAL: Self = Self(-1);
    #[doc(alias = "NETDB_SUCCESS")]
    pub const SUCCESS: Self = Self(0);
    pub const HOST_NOT_FOUND: Self = Self(1);
    pub const TRY_AGAIN: Self = Self(2);
    pub const NO_RECOVERY: Self = Self(3);
    pub const NO_DATA: Self = Self(4);
}

/// Allocator for the resolver, see [set_resolver_allocator].
pub type ResolverAlloc = Option<unsafe extern "C" fn(size: u32) -> *mut ffi::c_void>;

/// Deallocator for the resolver, see [set_resolver_allocator].
pub type ResolverFree = Option<unsafe extern "C" fn(ptr: *mut ffi::c_void)>;

unsafe extern "C" {
    /// socket_lib_init
    #[doc(alias("socket_lib_init", "SOInit"))]
    #[link_name = "socket_lib_init"]
    pub unsafe fn init() -> ffi::c_int;

    /// socket_lib_finish
    #[doc(alias("socket_lib_finish", "SOFinish"))]
    #[link_name = "socket_lib_finish"]
    pub unsafe fn deinit() -> ffi::c_int;

    /// socketlasterr
    ///
    /// Error set by the last socket call on this thread.
    #[doc(alias("socketlasterr", "SOLastError"))]
    #[link_name = "socketlasterr"]
    pub unsafe fn last_error() -> Error;

    /// socket
    #[link_name = "socket"]
    pub unsafe fn socket(family: Family, sock_type: Type, protocol: Protocol) -> FileDescriptor;

    /// socketclose
    #[doc(alias("socketclose", "SOClose"))]
    #[link_name = "socketclose"]
    pub unsafe fn close(fd: FileDescriptor) -> ffi::c_int;

    /// accept
    #[link_name = "accept"]
    pub unsafe fn accept(
        fd: FileDescriptor,
        addr: *mut SockAddr,
        addr_len: *mut SockLen,
    ) -> FileDescriptor;

    /// bind
    #[link_name = "bind"]
    pub unsafe fn bind(fd: FileDescriptor, addr: *const SockAddr, addr_len: SockLen) -> ffi::c_int;

    /// connect
    #[link_name = "connect"]
    pub unsafe fn connect(
        fd: FileDescriptor,
        addr: *const SockAddr,
        addr_len: SockLen,
    ) -> ffi::c_int;

    /// listen
    #[link_name = "listen"]
    pub unsafe fn listen(fd: FileDescriptor, backlog: ffi::c_int) -> ffi::c_int;

    /// shutdown
    #[link_name = "shutdown"]
    pub unsafe fn shutdown(fd: FileDescriptor, how: Shutdown) -> ffi::c_int;

    /// recv
    ///
    /// Returns the number of bytes received, 0 if the peer closed the connection.
    #[link_name = "recv"]
    pub unsafe fn recv(
        fd: FileDescriptor,
        buf: *mut ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
    ) -> ffi::c_int;

    /// recvfrom
    ///
    /// Returns the number of bytes received, 0 if the peer closed the connection.
    #[link_name = "recvfrom"]
    pub unsafe fn recvfrom(
        fd: FileDescriptor,
        buf: *mut ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
        from: *mut SockAddr,
        from_len: *mut SockLen,
    ) -> ffi::c_int;

    /// recvfrom_ex
    #[link_name = "recvfrom_ex"]
    pub unsafe fn recvfrom_ex(
        fd: FileDescriptor,
        buf: *mut ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
        from: *mut SockAddr,
        from_len: *mut SockLen,
        msg: *mut u8,
        msg_len: ffi::c_int,
    ) -> ffi::c_int;

    /// recvfrom_multi
    #[link_name = "recvfrom_multi"]
    pub unsafe fn recvfrom_multi(
        fd: FileDescriptor,
        flags: MsgFlags,
        bufs: *mut RecvFromMultiBuffers,
        datagram_len: ffi::c_int,
        datagram_count: ffi::c_int,
        timeout: *mut TimeVal,
    ) -> ffi::c_int;

    /// send
    ///
    /// Returns the number of bytes sent.
    #[link_name = "send"]
    pub unsafe fn send(
        fd: FileDescriptor,
        buf: *const ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
    ) -> ffi::c_int;

    /// sendto
    ///
    /// Returns the number of bytes sent.
    #[link_name = "sendto"]
    pub unsafe fn sendto(
        fd: FileDescriptor,
        buf: *const ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
        dest: *const SockAddr,
        dest_len: SockLen,
    ) -> ffi::c_int;

    /// sendto_multi
    ///
    /// Sends the same datagram to `dest_count` addresses, 1 to 32.
    #[link_name = "sendto_multi"]
    pub unsafe fn sendto_multi(
        fd: FileDescriptor,
        buf: *const ffi::c_void,
        len: ffi::c_int,
        flags: MsgFlags,
        dests: *const SockAddr,
        dest_count: ffi::c_int,
    ) -> ffi::c_int;

    /// sendto_multi_ex
    #[link_name = "sendto_multi_ex"]
    pub unsafe fn sendto_multi_ex(
        fd: FileDescriptor,
        flags: MsgFlags,
        bufs: *mut SendToMultiExBuffers,
        datagram_count: ffi::c_int,
    ) -> ffi::c_int;

    /// select
    ///
    /// Returns the number of ready sockets.
    #[link_name = "select"]
    pub unsafe fn select(
        nfds: ffi::c_int,
        read: *mut FdSet,
        write: *mut FdSet,
        except: *mut FdSet,
        timeout: *mut TimeVal,
    ) -> ffi::c_int;

    /// getpeername
    #[doc(alias("getpeername", "SOGetPeerName"))]
    #[link_name = "getpeername"]
    pub unsafe fn get_peername(
        fd: FileDescriptor,
        addr: *mut SockAddr,
        addr_len: *mut SockLen,
    ) -> ffi::c_int;

    /// getsockname
    #[doc(alias("getsockname", "SOGetSockName"))]
    #[link_name = "getsockname"]
    pub unsafe fn get_sockname(
        fd: FileDescriptor,
        addr: *mut SockAddr,
        addr_len: *mut SockLen,
    ) -> ffi::c_int;

    /// getsockopt
    #[doc(alias("getsockopt", "SOGetSockOpt"))]
    #[link_name = "getsockopt"]
    pub unsafe fn get_sockopt(
        fd: FileDescriptor,
        level: Level,
        option: SockOpt,
        value: *mut ffi::c_void,
        value_len: *mut SockLen,
    ) -> ffi::c_int;

    /// setsockopt
    #[doc(alias("setsockopt", "SOSetSockOpt"))]
    #[link_name = "setsockopt"]
    pub unsafe fn set_sockopt(
        fd: FileDescriptor,
        level: Level,
        option: SockOpt,
        value: *const ffi::c_void,
        value_len: SockLen,
    ) -> ffi::c_int;

    /// getsocklibopt
    #[doc(alias("getsocklibopt", "SOGetLibOpt"))]
    #[link_name = "getsocklibopt"]
    pub unsafe fn get_lib_opt(option: LibOpt, value: *mut ffi::c_int) -> ffi::c_int;

    /// setsocklibopt
    #[doc(alias("setsocklibopt", "SOSetLibOpt"))]
    #[link_name = "setsocklibopt"]
    pub unsafe fn set_lib_opt(option: LibOpt, value: ffi::c_int) -> ffi::c_int;

    /// set_multicast_state
    #[link_name = "set_multicast_state"]
    pub unsafe fn set_multicast_state(enabled: ffi::c_bool) -> ffi::c_int;

    /// htonl
    #[link_name = "htonl"]
    pub unsafe fn htonl(value: u32) -> u32;

    /// htons
    #[link_name = "htons"]
    pub unsafe fn htons(value: u16) -> u16;

    /// ntohl
    #[link_name = "ntohl"]
    pub unsafe fn ntohl(value: u32) -> u32;

    /// ntohs
    #[link_name = "ntohs"]
    pub unsafe fn ntohs(value: u16) -> u16;

    /// inet_aton
    ///
    /// Returns nonzero on success.
    #[link_name = "inet_aton"]
    pub unsafe fn inet_aton(text: *const ffi::c_char, addr: *mut InAddr) -> ffi::c_int;

    /// inet_ntoa
    ///
    /// The C function takes the [InAddr] by value, which the ABI passes as a pointer.
    /// The result is in a static buffer.
    #[link_name = "inet_ntoa"]
    pub unsafe fn inet_ntoa(addr: *const InAddr) -> *mut ffi::c_char;

    /// inet_ntoa_r
    ///
    /// The C function takes the [InAddr] by value, which the ABI passes as a pointer.
    #[link_name = "inet_ntoa_r"]
    pub unsafe fn inet_ntoa_r(addr: *const InAddr, buf: *mut ffi::c_char) -> *mut ffi::c_char;

    /// inet_ntop
    #[link_name = "inet_ntop"]
    pub unsafe fn inet_ntop(
        family: Family,
        src: *const ffi::c_void,
        dst: *mut ffi::c_char,
        size: SockLen,
    ) -> *const ffi::c_char;

    /// inet_pton
    #[link_name = "inet_pton"]
    pub unsafe fn inet_pton(
        family: Family,
        src: *const ffi::c_char,
        dst: *mut ffi::c_void,
    ) -> ffi::c_int;

    /// gethostbyname
    #[doc(alias("gethostbyname", "SOGetHostByName"))]
    #[link_name = "gethostbyname"]
    pub unsafe fn get_host_by_name(name: *const ffi::c_char) -> *mut HostEnt;

    /// gethostbyaddr
    #[doc(alias("gethostbyaddr", "SOGetHostByAddr"))]
    #[link_name = "gethostbyaddr"]
    pub unsafe fn get_host_by_addr(
        addr: *const ffi::c_void,
        len: ffi::c_int,
        family: Family,
    ) -> *mut HostEnt;

    /// getaddrinfo
    #[doc(alias("getaddrinfo", "SOGetAddrInfo"))]
    #[link_name = "getaddrinfo"]
    pub unsafe fn get_addr_info(
        node: *const ffi::c_char,
        service: *const ffi::c_char,
        hints: *const AddrInfo,
        result: *mut *mut AddrInfo,
    ) -> AiError;

    /// getaddrinfo_rs
    #[doc(alias("getaddrinfo_rs", "SOGetAddrInfoRSU"))]
    #[link_name = "getaddrinfo_rs"]
    pub unsafe fn get_addr_info_rs(
        node: *const ffi::c_char,
        service: *const ffi::c_char,
        hints: *const AddrInfo,
        result: *mut *mut AddrInfo,
    ) -> AiError;

    /// freeaddrinfo
    #[doc(alias("freeaddrinfo", "SOFreeAddrInfo"))]
    #[link_name = "freeaddrinfo"]
    pub unsafe fn free_addr_info(info: *mut AddrInfo);

    /// getnameinfo
    #[doc(alias("getnameinfo", "SOGetNameInfo"))]
    #[link_name = "getnameinfo"]
    pub unsafe fn get_name_info(
        addr: *const SockAddr,
        addr_len: SockLen,
        node: *mut ffi::c_char,
        node_len: SockLen,
        service: *mut ffi::c_char,
        service_len: SockLen,
        flags: NiFlags,
    ) -> AiError;

    /// gai_strerror
    #[link_name = "gai_strerror"]
    pub unsafe fn gai_strerror(error: AiError) -> *const ffi::c_char;

    /// get_h_errno
    ///
    /// Error of the last `get_host_by_*` call.
    #[link_name = "get_h_errno"]
    pub unsafe fn get_h_errno() -> *mut HostError;

    /// set_resolver_allocator
    #[link_name = "set_resolver_allocator"]
    pub unsafe fn set_resolver_allocator(alloc: ResolverAlloc, free: ResolverFree) -> ffi::c_int;

    /// clear_resolver_cache
    #[link_name = "clear_resolver_cache"]
    pub unsafe fn clear_resolver_cache();
}
