//! NSSL (TLS)
//!
//! TLS on top of a connected [socket](super::socket). See also [crate::nn::nets2] for CRLs and context flags.

use super::socket::FileDescriptor;
use crate::ffi;

/// NSSL_RVAL
#[doc(alias = "NSSL_RVAL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(pub i32);

impl Error {
    pub const OK: Self = Self(0);
    pub const INVALID_NSSL_CTX: Self = Self(-2621441);
    pub const INVALID_CERTID: Self = Self(-2621442);
    pub const CERT_LIMIT: Self = Self(-2621443);
    pub const INVALID_NSSL_CONNECTION: Self = Self(-2621444);
    pub const INVALID_CERT: Self = Self(-2621445);
    pub const ZERO_RETURN: Self = Self(-2621446);
    pub const WANT_READ: Self = Self(-2621447);
    pub const WANT_WRITE: Self = Self(-2621448);
    pub const IO_ERROR: Self = Self(-2621449);
    pub const NSSLLIB_ERROR: Self = Self(-2621450);
    pub const UNKNOWN: Self = Self(-2621451);
    pub const OUT_OF_MEMORY: Self = Self(-2621452);
    pub const INVALID_STATE: Self = Self(-2621453);
    pub const HANDSHAKE_ERROR: Self = Self(-2621454);
    pub const NO_CERT: Self = Self(-2621455);
    pub const INVALID_FD: Self = Self(-2621456);
    pub const LIB_NOT_READY: Self = Self(-2621457);
    pub const IPC_ERROR: Self = Self(-2621458);
    pub const RESOURCE_LIMIT: Self = Self(-2621459);
    pub const INVALID_HANDLE: Self = Self(-2621460);
    pub const INVALID_CERT_TYPE: Self = Self(-2621461);
    pub const INVALID_KEY_TYPE: Self = Self(-2621462);
    pub const INVALID_SIZE: Self = Self(-2621463);
    pub const NO_PEER_CERT: Self = Self(-2621464);
    pub const INSUFFICIENT_SIZE: Self = Self(-2621465);
    pub const NO_CIPHER: Self = Self(-2621466);
    pub const INVALID_ARG: Self = Self(-2621467);
    pub const INVALID_NSSL_SESSION: Self = Self(-2621468);
    pub const NO_SESSION: Self = Self(-2621469);
    pub const SSL_SHUTDOWN_ERROR: Self = Self(-2621470);
    pub const CERT_SIZE_LIMIT: Self = Self(-2621471);
    pub const CERT_NO_ACCESS: Self = Self(-2621472);
    pub const INVALID_CERT_ID: Self = Self(-2621473);
    pub const CERT_READ_ERROR: Self = Self(-2621474);
    pub const CERTSTORE_INIT_FAILURE: Self = Self(-2621475);
    pub const INVALID_CERT_ENCODING: Self = Self(-2621476);
    pub const CERTSTORE_ERR: Self = Self(-2621477);
    pub const PVTKEY_READ_ERROR: Self = Self(-2621478);
    pub const INVALID_PVTKEY: Self = Self(-2621479);
    pub const NOT_READY: Self = Self(-2621480);
    pub const ENCRYPTION_ERROR: Self = Self(-2621481);
    pub const NO_CERTSTORE: Self = Self(-2621482);
    pub const PVTKEY_SIZE_LIMIT: Self = Self(-2621483);
    pub const PROC_MAX_EXT_CERTS: Self = Self(-2621484);
    pub const PROC_MAX_CONTEXTS: Self = Self(-2621485);
    pub const PROC_MAX_CONNECTIONS: Self = Self(-2621486);
    #[doc(alias = "NSSL_RVAL_CERT_NOT_EXPORTALE")]
    pub const CERT_NOT_EXPORTABLE: Self = Self(-2621487);
    pub const INVALID_CERT_SIZE: Self = Self(-2621488);
    pub const INVALID_KEY_SIZE: Self = Self(-2621489);
    pub const CRL_SIZE_LIMIT: Self = Self(-2621490);
    pub const INVALID_CRL: Self = Self(-2621491);
    pub const INVALID_CRL_TYPE: Self = Self(-2621492);
}

/// NSSLContextHandle
#[doc(alias = "NSSLContextHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextHandle(pub i32);

/// NSSLConnectionHandle
#[doc(alias = "NSSLConnectionHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionHandle(pub i32);

/// NSSLSessionHandle
#[doc(alias = "NSSLSessionHandle")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionHandle(pub i32);

/// NSSLSSLVersion
#[doc(alias = "NSSLSSLVersion")]
#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SslVersion {
    #[default]
    Auto = 0,
    Tlsv1 = 1,
    Sslv3 = 2,
}

/// NSSLCertType
#[doc(alias = "NSSLCertType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CertType(pub u32);

impl CertType {
    pub const DER: Self = Self(0);
}

/// NSSLPrivKeyType
#[doc(alias = "NSSLPrivKeyType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrivKeyType(pub u32);

impl PrivKeyType {
    pub const RSA: Self = Self(0);
}

/// NSSLCrlType
#[doc(alias = "NSSLCrlType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrlType {
    #[default]
    Der = 0,
}

/// NSSLHandshakeError
#[doc(alias = "NSSLHandshakeError")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandshakeError(pub u32);

impl HandshakeError {
    pub const OK: Self = Self(0);
    pub const INCOMPLETE: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
    pub const INVALID_CLIENT_CERT: Self = Self(3);
    pub const CLIENT_CERT_NEEDED: Self = Self(4);
    pub const CIPHER_MISMATCH: Self = Self(5);
    pub const UNSUPPORTED_PROTOCOL: Self = Self(6);
    pub const PEER_CERT_HOSTNAME_MISMATCH: Self = Self(7);
    pub const SERVER_CERT_VERIFY_FAILED: Self = Self(8);
}

/// NSSLCertValidationStatus
#[doc(alias = "NSSLCertValidationStatus")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CertValidationStatus(pub i32);

impl CertValidationStatus {
    pub const VALID: Self = Self(0);
    pub const UNKNOWN: Self = Self(1000);
    pub const HOSTNAME_MISMATCH: Self = Self(1001);
    pub const UNABLE_TO_GET_ISSUER_CERT: Self = Self(2);
    pub const UNABLE_TO_GET_CRL: Self = Self(3);
    pub const UNABLE_TO_DECRYPT_CERT_SIGNATURE: Self = Self(4);
    pub const UNABLE_TO_DECRYPT_CRL_SIGNATURE: Self = Self(5);
    pub const UNABLE_TO_DECODE_ISSUER_PUBLIC_KEY: Self = Self(6);
    pub const CERT_SIGNATURE_FAILURE: Self = Self(7);
    pub const CRL_SIGNATURE_FAILURE: Self = Self(8);
    pub const CERT_NOT_YET_VALID: Self = Self(9);
    pub const CERT_HAS_EXPIRED: Self = Self(10);
    pub const CRL_NOT_YET_VALID: Self = Self(11);
    pub const CRL_HAS_EXPIRED: Self = Self(12);
    pub const ERROR_IN_CERT_NOT_BEFORE_FIELD: Self = Self(13);
    pub const ERROR_IN_CERT_NOT_AFTER_FIELD: Self = Self(14);
    pub const ERROR_IN_CRL_LAST_UPDATE_FIELD: Self = Self(15);
    pub const ERROR_IN_CRL_NEXT_UPDATE_FIELD: Self = Self(16);
    pub const OUT_OF_MEM: Self = Self(17);
    pub const DEPTH_ZERO_SELF_SIGNED_CERT: Self = Self(18);
    pub const SELF_SIGNED_CERT_IN_CHAIN: Self = Self(19);
    pub const UNABLE_TO_GET_ISSUER_CERT_LOCALLY: Self = Self(20);
    pub const UNABLE_TO_VERIFY_LEAF_SIGNATURE: Self = Self(21);
    pub const CERT_CHAIN_TOO_LONG: Self = Self(22);
    pub const CERT_REVOKED: Self = Self(23);
    pub const INVALID_CA: Self = Self(24);
    pub const PATH_LENGTH_EXCEEDED: Self = Self(25);
    pub const INVALID_PURPOSE: Self = Self(26);
    pub const CERT_UNTRUSTED: Self = Self(27);
    pub const CERT_REJECTED: Self = Self(28);
    pub const SUBJECT_ISSUER_MISMATCH: Self = Self(29);
    pub const AKID_SKID_MISMATCH: Self = Self(30);
    pub const AKID_ISSUER_SERIAL_MISMATCH: Self = Self(31);
    pub const KEYUSAGE_NO_CERTSIGN: Self = Self(32);
    pub const UNABLE_TO_GET_CRL_ISSUER: Self = Self(33);
    pub const UNHANDLED_CRITICAL_EXTENSION: Self = Self(34);
    pub const KEYUSAGE_NO_CRL_SIGN: Self = Self(35);
    pub const UNHANDLED_CRITICAL_CRL_EXTENSION: Self = Self(36);
    pub const INVALID_NON_CA: Self = Self(37);
    pub const PROXY_PATH_LENGTH_EXCEEDED: Self = Self(38);
    pub const KEYUSAGE_NO_DIGITAL_SIGNATURE: Self = Self(39);
    pub const PROXY_CERTIFICATES_NOT_ALLOWED: Self = Self(40);
    pub const INVALID_EXTENSION: Self = Self(41);
    pub const INVALID_POLICY_EXTENSION: Self = Self(42);
    pub const NO_EXPLICIT_POLICY: Self = Self(43);
    pub const DIFFERENT_CRL_SCOPE: Self = Self(44);
    pub const UNSUPPORTED_EXTENSION_FEATURE: Self = Self(45);
    pub const UNNESTED_RESOURCE: Self = Self(46);
    pub const PERMITTED_VIOLATION: Self = Self(47);
    pub const EXCLUDED_VIOLATION: Self = Self(48);
    pub const SUBTREE_MINMAX: Self = Self(49);
    pub const UNSUPPORTED_CONSTRAINT_TYPE: Self = Self(51);
    pub const APPLICATION_VERIFICATION: Self = Self(50);
    pub const UNSUPPORTED_CONSTRAINT_SYNTAX: Self = Self(52);
    pub const UNSUPPORTED_NAME_SYNTAX: Self = Self(53);
    pub const CRL_PATH_VALIDATION_ERROR: Self = Self(54);
}

/// NSSL_VERIFY_*
#[doc(alias = "NSSL_VERIFY")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerifyOptions(pub i32);

impl VerifyOptions {
    pub const NONE: Self = Self(0x0);
    pub const PEER_CA: Self = Self(0x1);
    pub const HOSTNAME: Self = Self(0x2);
    pub const DATE_VALIDITY: Self = Self(0x4);
    pub const ALL: Self = Self(0x7);
    pub const ALL_EXCEPT_DATE: Self = Self(0x3);
}

impl core::ops::BitOr for VerifyOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// NSSL_CTX_FLAG_*
#[doc(alias = "NSSL_CTX_FLAG")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextFlags(pub u32);

impl ContextFlags {
    pub const CRL_CHECK_LEAF: Self = Self(0x1);
    pub const CRL_CHECK_CHAIN: Self = Self(0x2);
}

impl core::ops::BitOr for ContextFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// NSSLClientCertID
#[doc(alias = "NSSLClientCertID")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientCertId(pub i32);

impl ClientCertId {
    pub const NONE: Self = Self(0);
    pub const DEFAULT: Self = Self(1);
}

/// NSSLServerCertID
///
/// Built-in root certificates.
#[doc(alias = "NSSLServerCertID")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerCertId(pub i32);

impl ServerCertId {
    pub const NONE: Self = Self(0);
    pub const NINTENDO_CA: Self = Self(100);
    pub const NINTENDO_CA_G2: Self = Self(101);
    pub const NINTENDO_CA_G3: Self = Self(102);
    pub const NINTENDO_CLASS2_CA: Self = Self(103);
    pub const NINTENDO_CLASS2_CA_G2: Self = Self(104);
    pub const NINTENDO_CLASS2_CA_G3: Self = Self(105);
    pub const BALTIMORE_CYBERTRUST_ROOT_CA: Self = Self(1001);
    pub const CYBERTRUST_GLOBAL_ROOT_CA: Self = Self(1002);
    pub const VERIZON_GLOBAL_ROOT_CA: Self = Self(1003);
    pub const GLOBALSIGN_ROOT_CA: Self = Self(1004);
    pub const GLOBALSIGN_ROOT_CA_R2: Self = Self(1005);
    pub const GLOBALSIGN_ROOT_CA_R3: Self = Self(1006);
    pub const VERISIGN_CLASS3_PUBLIC_PRIMARY_CA_G3: Self = Self(1007);
    pub const VERISIGN_UNIVERSAL_ROOT_CA: Self = Self(1008);
    pub const VERISIGN_CLASS3_PUBLIC_PRIMARY_CA_G5: Self = Self(1009);
    pub const THAWTE_PRIMARY_ROOT_CA_G3: Self = Self(1010);
    pub const THAWTE_PRIMARY_ROOT_CA: Self = Self(1011);
    pub const GEOTRUST_GLOBAL_CA: Self = Self(1012);
    pub const GEOTRUST_GLOBAL_CA2: Self = Self(1013);
    pub const GEOTRUST_PRIMARY_CA: Self = Self(1014);
    pub const GEOTRUST_PRIMARY_CA_G3: Self = Self(1015);
    pub const ADDTRUST_EXT_CA_ROOT: Self = Self(1016);
    pub const COMODO_CA: Self = Self(1017);
    pub const UTN_DATACORP_SGC_CA: Self = Self(1018);
    pub const UTN_USERFIRST_HARDWARE_CA: Self = Self(1019);
    pub const DIGICERT_HIGH_ASSURANCE_EV_ROOT_CA: Self = Self(1020);
    pub const DIGICERT_ASSURED_ID_ROOT_CA: Self = Self(1021);
    pub const DIGICERT_GLOBAL_ROOT_CA: Self = Self(1022);
    pub const GTE_CYBERTRUST_GLOBAL_ROOT: Self = Self(1023);
    pub const VERISIGN_CLASS3_PUBLIC_PRIMARY_CA: Self = Self(1024);
    pub const THAWTE_PREMIUM_SERVER_CA: Self = Self(1025);
    pub const EQUIFAX_SECURE_CA: Self = Self(1026);
    pub const ENTRUST_SECURE_SERVER_CA: Self = Self(1027);
    pub const VERISIGN_CLASS3_PUBLIC_PRIMARY_CA_G2: Self = Self(1028);
    pub const ENTRUST_CA_2048: Self = Self(1029);
    pub const ENTRUST_ROOT_CA: Self = Self(1030);
    pub const ENTRUST_ROOT_CA_G2: Self = Self(1031);
    pub const DIGICERT_ASSURED_ID_ROOT_CA_G2: Self = Self(1032);
    pub const DIGICERT_GLOBAL_ROOT_CA_G2: Self = Self(1033);
}

/// CERTSTORE_ENTITY_GROUP_*
#[doc(alias("NSSLServerCertGroupMask", "CertstoreEntityGroupMask"))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CertGroupMask(pub u32);

impl CertGroupMask {
    pub const NINTENDO_CA_CERTS: Self = Self(0x1);
    pub const COMMERCIAL_CA_CERTS: Self = Self(0x2);
    pub const ANY: Self = Self(0x3);
}

impl core::ops::BitOr for CertGroupMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

unsafe extern "C" {
    /// NSSLInit
    #[doc(alias = "NSSLInit")]
    #[link_name = "NSSLInit"]
    pub unsafe fn init() -> Error;

    /// NSSLFinish
    #[doc(alias = "NSSLFinish")]
    #[link_name = "NSSLFinish"]
    pub unsafe fn deinit() -> Error;

    /// NSSLCreateContext
    ///
    /// Returns a negative [Error] on failure.
    #[doc(alias = "NSSLCreateContext")]
    #[link_name = "NSSLCreateContext"]
    pub unsafe fn create_context(version: SslVersion) -> ContextHandle;

    /// NSSLDestroyContext
    #[doc(alias = "NSSLDestroyContext")]
    #[link_name = "NSSLDestroyContext"]
    pub unsafe fn destroy_context(context: ContextHandle) -> Error;

    /// NSSLSetClientPKI
    #[doc(alias = "NSSLSetClientPKI")]
    #[link_name = "NSSLSetClientPKI"]
    pub unsafe fn set_client_pki(context: ContextHandle, cert: ClientCertId) -> Error;

    /// NSSLAddServerPKI
    ///
    /// Trusts one of the built-in root certificates.
    #[doc(alias = "NSSLAddServerPKI")]
    #[link_name = "NSSLAddServerPKI"]
    pub unsafe fn add_server_pki(context: ContextHandle, cert: ServerCertId) -> Error;

    /// NSSLAddServerPKIGroups
    ///
    /// Trusts groups of built-in root certificates.
    #[doc(alias = "NSSLAddServerPKIGroups")]
    #[link_name = "NSSLAddServerPKIGroups"]
    pub unsafe fn add_server_pki_groups(
        context: ContextHandle,
        groups: CertGroupMask,
        added: *mut ffi::c_int,
        errors: *mut ffi::c_int,
    ) -> Error;

    /// NSSLSetClientPKIExternal
    #[doc(alias = "NSSLSetClientPKIExternal")]
    #[link_name = "NSSLSetClientPKIExternal"]
    pub unsafe fn set_client_pki_external(
        context: ContextHandle,
        cert: *const u8,
        cert_len: ffi::c_int,
        cert_type: CertType,
        key: *const u8,
        key_len: ffi::c_int,
        key_type: PrivKeyType,
    ) -> Error;

    /// NSSLAddServerPKIExternal
    #[doc(alias = "NSSLAddServerPKIExternal")]
    #[link_name = "NSSLAddServerPKIExternal"]
    pub unsafe fn add_server_pki_external(
        context: ContextHandle,
        cert: *const u8,
        cert_len: ffi::c_int,
        cert_type: CertType,
    ) -> Error;

    /// NSSLCreateConnection
    ///
    /// Wraps the connected socket `fd`. Returns a negative [Error] on failure.
    #[doc(alias = "NSSLCreateConnection")]
    #[link_name = "NSSLCreateConnection"]
    pub unsafe fn create_connection(
        context: ContextHandle,
        hostname: *const ffi::c_char,
        hostname_len: ffi::c_int,
        verify: VerifyOptions,
        fd: FileDescriptor,
        blocking: ffi::c_bool,
    ) -> ConnectionHandle;

    /// NSSLDestroyConnection
    #[doc(alias = "NSSLDestroyConnection")]
    #[link_name = "NSSLDestroyConnection"]
    pub unsafe fn destroy_connection(connection: ConnectionHandle) -> Error;

    /// NSSLDoHandshake
    #[doc(alias = "NSSLDoHandshake")]
    #[link_name = "NSSLDoHandshake"]
    pub unsafe fn do_handshake(
        connection: ConnectionHandle,
        handshake_error: *mut HandshakeError,
        validation_status: *mut CertValidationStatus,
    ) -> Error;

    /// NSSLRead
    #[doc(alias = "NSSLRead")]
    #[link_name = "NSSLRead"]
    pub unsafe fn read(
        connection: ConnectionHandle,
        buf: *mut ffi::c_void,
        len: ffi::c_int,
        bytes_read: *mut ffi::c_int,
    ) -> Error;

    /// NSSLWrite
    #[doc(alias = "NSSLWrite")]
    #[link_name = "NSSLWrite"]
    pub unsafe fn write(
        connection: ConnectionHandle,
        buf: *const ffi::c_void,
        len: ffi::c_int,
        bytes_written: *mut ffi::c_int,
    ) -> Error;

    /// NSSLGetSession
    #[doc(alias = "NSSLGetSession")]
    #[link_name = "NSSLGetSession"]
    pub unsafe fn get_session(connection: ConnectionHandle) -> SessionHandle;

    /// NSSLSetSession
    ///
    /// Resumes a previous session.
    #[doc(alias = "NSSLSetSession")]
    #[link_name = "NSSLSetSession"]
    pub unsafe fn set_session(connection: ConnectionHandle, session: SessionHandle) -> Error;

    /// NSSLFreeSession
    #[doc(alias = "NSSLFreeSession")]
    #[link_name = "NSSLFreeSession"]
    pub unsafe fn free_session(session: SessionHandle) -> Error;

    /// NSSLRemoveSession
    #[doc(alias = "NSSLRemoveSession")]
    #[link_name = "NSSLRemoveSession"]
    pub unsafe fn remove_session(session: SessionHandle) -> Error;

    /// NSSLGetPending
    ///
    /// Number of bytes that can be read without blocking.
    #[doc(alias = "NSSLGetPending")]
    #[link_name = "NSSLGetPending"]
    pub unsafe fn get_pending(connection: ConnectionHandle, pending: *mut ffi::c_int) -> Error;

    /// NSSLGetPeerCertSize
    #[doc(alias = "NSSLGetPeerCertSize")]
    #[link_name = "NSSLGetPeerCertSize"]
    pub unsafe fn get_peer_cert_size(connection: ConnectionHandle) -> ffi::c_int;

    /// NSSLGetPeerCert
    #[doc(alias = "NSSLGetPeerCert")]
    #[link_name = "NSSLGetPeerCert"]
    pub unsafe fn get_peer_cert(
        connection: ConnectionHandle,
        buf: *mut u8,
        buf_len: *mut ffi::c_int,
    ) -> Error;

    /// NSSLGetCipherInfo
    #[doc(alias = "NSSLGetCipherInfo")]
    #[link_name = "NSSLGetCipherInfo"]
    pub unsafe fn get_cipher_info(
        connection: ConnectionHandle,
        version: *mut ffi::c_char,
        version_len: ffi::c_int,
        name: *mut ffi::c_char,
        name_len: ffi::c_int,
        bits: *mut ffi::c_int,
    ) -> Error;

    /// NSSLExportInternalServerCertificate
    #[doc(alias = "NSSLExportInternalServerCertificate")]
    #[link_name = "NSSLExportInternalServerCertificate"]
    pub unsafe fn export_internal_server_certificate(
        cert: ServerCertId,
        buf: *mut ffi::c_char,
        buf_size: *mut ffi::c_int,
        cert_type: *mut CertType,
    ) -> Error;

    /// NSSLExportInternalClientCertificate
    #[doc(alias = "NSSLExportInternalClientCertificate")]
    #[link_name = "NSSLExportInternalClientCertificate"]
    pub unsafe fn export_internal_client_certificate(
        cert: ClientCertId,
        cert_buf: *mut ffi::c_char,
        cert_buf_size: *mut ffi::c_int,
        cert_type: *mut CertType,
        key_buf: *mut ffi::c_char,
        key_buf_size: *mut ffi::c_int,
        key_type: *mut PrivKeyType,
    ) -> Error;
}
