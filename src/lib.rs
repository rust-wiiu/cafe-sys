//! cafe-sys
//!
//! This crates provides bindings to the system libraries of the Cafe operating system for the Nintendo Wii U™.
//!
//! Not every function is mapped to its original name in the ABI, but it's link name should be noted in the documentation of the symbol.
//!
//! This crate is not affiliated with Nintendo.

#![no_std]
#![cfg_attr(target_arch = "powerpc", feature(asm_experimental_arch))]

#[allow(unused_macros)]
macro_rules! imports_section {
    // Main entry point
    ($name:literal, [$($func:literal),*], [$($data:literal),*]) => {
        imports_section!(@fimport $name, $($func),*);
        imports_section!(@dimport $name, $($data),*);
    };

    // Function imports - non-empty
    (@fimport $name:literal, $($func:literal),+) => {
        ::core::arch::global_asm!(
            concat!(".section .fimport_", $name, ", \"ax\""),
            ".align 4",
            ".long 1",
            ".long 0x00000000",
            concat!(".string \"", $name, "\""),
            ".balign 8, 0",
        );
        $(
            ::core::arch::global_asm!(
                concat!(".section .fimport_", $name, ".", $func, ", \"ax\""),
                concat!(".global ", $func),
                concat!(".type ", $func, ", @function"),
                concat!($func, ":"),
                ".long 0x0",
                ".long 0x0",
            );
        )*
    };
    (@fimport $name:literal,) => {};

    // Data imports - non-empty
    (@dimport $name:literal, $($data:literal),+) => {
        ::core::arch::global_asm!(
            concat!(".section .dimport_", $name, ", \"a\""),
            ".align 4",
            ".long 1",
            ".long 0x00000000",
            concat!(".string \"", $name, "\""),
            ".balign 8, 0",
        );
        $(
            ::core::arch::global_asm!(
                concat!(".section .dimport_", $name, ".", $data, ", \"a\""),
                concat!(".global ", $data),
                concat!(".type ", $data, ", @object"),
                concat!($data, ":"),
                ".long 0x0",
                ".long 0x0",
            );
        )*
    };
    (@dimport $name:literal,) => {};
}

/// Should only be used on "#[[repr(C)]]" structs.
pub trait UnsafeInit: Sized {
    /// Zero-initializes the struct, allows the user to set specific fields via a mutable reference, then returns the fully initialized type.
    ///
    /// Prefer initialiting the struct as a literal when setting all fields!
    ///
    /// # Safety
    ///
    /// Has the same safety implications as [MaybeUninit]. Should only be used on "#[[repr(C)]]" structs i.e. structs with valid zero-invariant.
    fn init<F>(f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        let mut s = core::mem::MaybeUninit::<Self>::zeroed();

        unsafe {
            // Get a mutable reference to the zeroed memory.
            // SAFETY: For C-compatible structs, zero-initialization
            // is generally a valid (but unsafe) state.
            f(s.assume_init_mut());

            s.assume_init()
        }
    }
}

#[allow(non_camel_case_types)]
pub mod ffi {
    pub use core::ffi::*;

    pub type c_bool = c_int;
    pub const TRUE: c_bool = 1;
    pub const FALSE: c_bool = 0;

    /// Bytes inside a struct with unknown use
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct unknown<const N: usize>([u8; N]);

    impl<const N: usize> unknown<N> {
        pub const fn zero() -> Self {
            Self([0; N])
        }
    }
}

#[cfg(feature = "avm")]
pub mod avm;
#[cfg(feature = "camera")]
pub mod camera;
#[cfg(feature = "coreinit")]
pub mod coreinit;
#[cfg(feature = "dc")]
pub mod dc;
#[cfg(feature = "dmae")]
pub mod dmae;
#[cfg(feature = "drmapp")]
pub mod drmapp;
#[cfg(feature = "erreula")]
pub mod erreula;
#[cfg(feature = "gx2")]
pub mod gx2;
#[cfg(feature = "h264")]
pub mod h264;
#[cfg(feature = "lzma920")]
pub mod lzma920;
#[cfg(feature = "mic")]
pub mod mic;
#[cfg(feature = "nfc")]
pub mod nfc;
#[cfg(feature = "nio_prof")]
pub mod nio_prof;
#[cfg(feature = "nlibnss")]
pub mod nlibnss;
#[cfg(feature = "nlibnss2")]
pub mod nlibnss2;
#[cfg(feature = "nn_ac")]
pub mod nn_ac;
#[cfg(feature = "nn_acp")]
pub mod nn_acp;
#[cfg(feature = "nn_act")]
pub mod nn_act;
#[cfg(feature = "nn_aoc")]
pub mod nn_aoc;
#[cfg(feature = "nn_boss")]
pub mod nn_boss;
#[cfg(feature = "nn_ccr")]
pub mod nn_ccr;
#[cfg(feature = "nn_cmpt")]
pub mod nn_cmpt;
#[cfg(feature = "nn_dlp")]
pub mod nn_dlp;
#[cfg(feature = "nn_ec")]
pub mod nn_ec;
#[cfg(feature = "nn_fp")]
pub mod nn_fp;
#[cfg(feature = "nn_hai")]
pub mod nn_hai;
#[cfg(feature = "nn_hpad")]
pub mod nn_hpad;
#[cfg(feature = "nn_idbe")]
pub mod nn_idbe;
#[cfg(feature = "nn_ndm")]
pub mod nn_ndm;
#[cfg(feature = "nn_nets2")]
pub mod nn_nets2;
#[cfg(feature = "nn_nfp")]
pub mod nn_nfp;
#[cfg(feature = "nn_nim")]
pub mod nn_nim;
#[cfg(feature = "nn_olv")]
pub mod nn_olv;
#[cfg(feature = "nn_pdm")]
pub mod nn_pdm;
#[cfg(feature = "nn_save")]
pub mod nn_save;
#[cfg(feature = "nn_sl")]
pub mod nn_sl;
#[cfg(feature = "nn_spm")]
pub mod nn_spm;
#[cfg(feature = "nn_temp")]
pub mod nn_temp;
#[cfg(feature = "nn_uds")]
pub mod nn_uds;
#[cfg(feature = "nn_vctl")]
pub mod nn_vctl;
#[cfg(feature = "nsysccr")]
pub mod nsysccr;
#[cfg(feature = "nsyshid")]
pub mod nsyshid;
#[cfg(feature = "nsyskbd")]
pub mod nsyskbd;
#[cfg(feature = "nsysnet")]
pub mod nsysnet;
#[cfg(feature = "nsysuhs")]
pub mod nsysuhs;
#[cfg(feature = "nsysuvd")]
pub mod nsysuvd;
#[cfg(feature = "ntag")]
pub mod ntag;
#[cfg(feature = "padscore")]
pub mod padscore;
#[cfg(feature = "proc_ui")]
pub mod proc_ui;
#[cfg(feature = "sndcore2")]
pub mod sndcore2;
#[cfg(feature = "snduser2")]
pub mod snduser2;
#[cfg(feature = "swkbd")]
pub mod swkbd;
#[cfg(feature = "sysapp")]
pub mod sysapp;
#[cfg(feature = "tcl")]
pub mod tcl;
#[cfg(feature = "tve")]
pub mod tve;
#[cfg(feature = "uac")]
pub mod uac;
#[cfg(feature = "uac_rpl")]
pub mod uac_rpl;
#[cfg(feature = "usb_mic")]
pub mod usb_mic;
#[cfg(feature = "uvc")]
pub mod uvc;
#[cfg(feature = "uvd")]
pub mod uvd;
#[cfg(feature = "vpad")]
pub mod vpad;
#[cfg(feature = "vpadbase")]
pub mod vpadbase;

#[cfg(all(feature = "uac", feature = "uac_rpl"))]
compile_error!("Cannot enable both 'uac' and 'uac_rpl' features simultaneously.");
