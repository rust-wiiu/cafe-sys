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
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct unknown<const N: usize>([u8; N]);

    impl<const N: usize> unknown<N> {
        pub const fn zero() -> Self {
            Self([0; N])
        }
    }

    impl<const N: usize> Default for unknown<N> {
        fn default() -> Self {
            Self::zero()
        }
    }
}

pub mod avm;
pub mod camera;
pub mod coreinit;
pub mod dc;
pub mod dmae;
pub mod drmapp;
pub mod erreula;
pub mod gx2;
pub mod h264;
pub mod lzma920;
pub mod mic;
pub mod nfc;
pub mod nio_prof;
pub mod nlibnss;
pub mod nlibnss2;
// pub mod nn_ac;
// pub mod nn_acp;
// pub mod nn_act;
// pub mod nn_aoc;
// pub mod nn_boss;
// pub mod nn_ccr;
// pub mod nn_cmpt;
// pub mod nn_dlp;
// pub mod nn_ec;
// pub mod nn_fp;
// pub mod nn_hai;
// pub mod nn_hpad;
// pub mod nn_idbe;
// pub mod nn_ndm;
// pub mod nn_nets2;
// pub mod nn_nfp;
// pub mod nn_nim;
// pub mod nn_olv;
// pub mod nn_pdm;
// pub mod nn_save;
// pub mod nn_sl;
// pub mod nn_spm;
// pub mod nn_temp;
// pub mod nn_uds;
// pub mod nn_vctl;
pub mod nn;
// pub mod nsysccr;
// pub mod nsyshid;
// pub mod nsyskbd;
// pub mod nsysnet;
// pub mod nsysuhs;
// pub mod nsysuvd;
pub mod nsys;
pub mod ntag;
pub mod padscore;
pub mod proc_ui;
pub mod sndcore2;
pub mod snduser2;
pub mod swkbd;
pub mod sysapp;
pub mod tcl;
pub mod tve;
pub mod uac;
// pub mod uac_rpl;
pub mod usb_mic;
pub mod uvc;
pub mod uvd;
pub mod vpad;
pub mod vpadbase;

// #[cfg(all(feature = "uac", feature = "uac_rpl"))]
// compile_error!("Cannot enable both 'uac' and 'uac_rpl' features simultaneously.");
