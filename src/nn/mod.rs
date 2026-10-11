pub mod ac;
pub mod acp;
pub mod act;
pub mod aoc;
pub mod boss;
pub mod ccr;
pub mod cmpt;
pub mod dlp;
pub mod ec;
pub mod fp;
pub mod hai;
pub mod hpad;
pub mod idbe;
pub mod ndm;
pub mod nets2;
pub mod nfp;
pub mod nim;
pub mod olv;
pub mod pdm;
pub mod save;
pub mod sl;
pub mod spm;
pub mod temp;
pub mod uds;
pub mod vctl;

/// NNResult
///
/// Negative values are failures.
///
/// The C type is a struct: functions taking it by value receive a pointer to a copy, so bind such parameters as `*const Result`. Returning it by value works as is.
#[doc(alias = "NNResult")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Result(pub i32);

impl Result {
    const SIGNATURE_IS_LEGACY: i32 = 3;

    #[inline]
    pub const fn is_success(self) -> bool {
        self.0 >= 0
    }

    #[inline]
    pub const fn is_failure(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub const fn is_legacy(self) -> bool {
        ((self.0 >> 27) & 0x3) == Self::SIGNATURE_IS_LEGACY
    }

    #[inline]
    pub const fn level(self) -> i32 {
        if self.is_legacy() {
            (self.0 << 14) >> 28
        } else {
            self.0 >> 29
        }
    }

    #[inline]
    pub const fn module(self) -> i32 {
        (self.0 >> 20) & if self.is_legacy() { 0x7F } else { 0x1FF }
    }

    #[inline]
    pub const fn description(self) -> i32 {
        self.0 & if self.is_legacy() { 0x3FF } else { 0xFFFFF }
    }

    /// Only meaningful for legacy results.
    #[inline]
    pub const fn summary(self) -> i32 {
        if self.is_legacy() {
            (self.0 >> 10) & 0xF
        } else {
            0
        }
    }
}
