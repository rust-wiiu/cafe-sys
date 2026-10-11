//! Face Library (FFL) types
//!
//! FFL itself is a static library, only the types shared with system libraries are here.

pub const MII_DATA_PACKET_SIZE: usize = 96;

/// FFLStoreData
///
/// Mii data as stored, including the trailing checksum.
#[doc(alias = "FFLStoreData")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreData {
    pub data: [u8; MII_DATA_PACKET_SIZE],
}

impl Default for StoreData {
    fn default() -> Self {
        Self {
            data: [0; MII_DATA_PACKET_SIZE],
        }
    }
}

const _: () = {
    assert!(size_of::<StoreData>() == 0x60);
};
