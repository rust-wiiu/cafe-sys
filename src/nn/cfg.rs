//! System configuration (CFG) types
//!
//! Only the types used by other libraries are here.

/// Nintendo 3DS types.
pub mod ctr {
    /// CFG_USER_NAME_LENGTH
    ///
    /// Including the terminator.
    #[doc(alias = "CFG_USER_NAME_LENGTH")]
    pub const USER_NAME_LENGTH: usize = 11;

    /// nn::cfg::CTR::UserName
    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserName {
        /// Null-terminated UTF-16.
        pub user_name: [u16; USER_NAME_LENGTH],
        /// Whether the name contains profanity.
        pub is_ng_user_name: bool,
        _pad: u8,
    }

    const _: () = {
        assert!(size_of::<UserName>() == 0x18);
    };
}
