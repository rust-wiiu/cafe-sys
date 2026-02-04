use crate::ffi;

/// OSSpinLock
#[repr(C, align(16))]
#[derive(Clone)]
pub struct SpinLock {
    pub owner: u32,
    _unk0: ffi::unknown<0x4>,
    pub recursion: u32,
    _unk1: ffi::unknown<0x04>,
}
