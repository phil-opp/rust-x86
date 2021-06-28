//! Access to various extended system registers
use bitflags::bitflags;

/// Extended feature enable mask register
#[derive(Debug)]
pub struct XCr0;

bitflags! {
    /// Configuration flags of the XCr0 register.
    pub struct XCr0Flags: u64 {
        /// Enables x87 FPU
        const X87 = 1;
        /// Enables 128-bit (legacy) SSE
        /// Must be set to enable AVX and YMM
        const SSE = 1<<1;
        /// Enables 256-bit SSE
        /// Must be set to enable AVX
        const YMM = 1<<2;
        /// When set, MPX instructions are enabled and the bound registers BND0-BND3 can be managed by XSAVE.
        const BNDREG = 1 << 3;
        /// When set, MPX instructions can be executed and XSAVE can manage the BNDCFGU and BNDSTATUS registers.
        const BNDCSR = 1 << 4;
        /// If set, AVX-512 instructions can be executed and XSAVE can manage the K0-K7 mask registers.
        const OPMASK = 1 << 5;
        /// If set, AVX-512 instructions can be executed and XSAVE can be used to manage the upper halves of the lower ZMM registers.
        const ZMM_HI256 = 1 << 6;
        /// If set, AVX-512 instructions can be executed and XSAVE can manage the upper ZMM registers.
        const HI16_ZMM = 1 << 7;
        /// When set, PKRU state management is supported by
        /// XSAVE/XRSTOR
        const MPK = 1<<9;
        /// When set the Lightweight Profiling extensions are enabled
        const LWP = 1<<62;
    }
}

#[cfg(feature = "instructions")]
mod x86_64 {
    use super::*;
    impl XCr0 {
        /// Read the current set of XCR0 flags.
        #[inline]
        pub fn read() -> XCr0Flags {
            XCr0Flags::from_bits_truncate(Self::read_raw())
        }

        /// Read the current raw XCR0 value.
        #[inline]
        pub fn read_raw() -> u64 {
            #[cfg(feature = "inline_asm")]
            unsafe {
                let (low, high): (u32, u32);
                asm!(
                    "xgetbv",
                    in("ecx") 0,
                    out("rax") low, out("rdx") high,
                    options(nomem, nostack, preserves_flags),
                );
                (high as u64) << 32 | (low as u64)
            }

            #[cfg(not(feature = "inline_asm"))]
            unsafe {
                crate::asm::x86_64_asm_xgetbv(0)
            }
        }

        /// Write XCR0 flags.
        ///
        /// Preserves the value of reserved fields.
        ///
        /// ## Safety
        ///
        /// This function is unsafe because it's possible to
        /// enable features that are not supported by the architecture
        #[inline]
        pub unsafe fn write(flags: XCr0Flags) {
            let old_value = Self::read_raw();
            let reserved = old_value & !(XCr0Flags::all().bits());
            let new_value = reserved | flags.bits();
            assert!(flags.contains(XCr0Flags::X87), "The X87 flag must be set");
            assert!((flags.contains(XCr0Flags::AVX) && flags.contains(XCr0Flags::OPMASK) && flags.contains(XCr0Flags::ZMM_HI256) && flags.contains(XCr0Flags::HI16_ZMM)) || !(flags.contains(XCr0Flags::AVX) && flags.contains(XCr0Flags::OPMASK) && flags.contains(XCr0Flags::ZMM_HI256) && flags.contains(XCr0Flags::HI16_ZMM)), "You must enable AVX to set or unset any of XCR0.opmask, XCR0.ZMM_Hi256, and XCR0.Hi16_ZMM");
            if !flags.contains(XCr0Flags::AVX) && (flags.contains(XCr0Flags::OPMASK) || flags.contains(XCr0Flags::ZMM_HI256) || flags.contains(XCr0Flags::HI16_ZMM)) {
            panic!("You must have AVX enabled to set XCR0.opmask, XCR0.ZMM_Hi256, or XCR0.Hi16_ZMM");
            }
            assert!((flags.contains(XCr0Flags::BNDREG) && flags.contains(XCr0Flags::BNDCSR)) || !(flags.contains(XCr0Flags::BNDREG) && flags.contains(XCr0Flags::BNDCSR)), "BNDREG and BNDCSR must be set and unset together");
            assert!((flags.contains(XCr0Flags::OPMASK) && flags.contains(XCr0Flags::ZMM_HI256) && flags.contains(XCr0Flags::HI16_ZMM)) || !(flags.contains(XCr0Flags::OPMASK) && flags.contains(XCr0Flags::ZMM_HI256) && flags.contains(XCr0Flags::HI16_ZMM)), "You must set or unset all of XCR0.opmask, XCR0.ZMM_Hi256, and XCR0.Hi16_ZMM");

            Self::write_raw(new_value);
        }

        /// Write raw XCR0 flags.
        ///
        /// Does _not_ preserve any values, including reserved fields.
        ///
        /// ## Safety
        ///
        /// This function is unsafe because it's possible to
        /// enable features that are not supported by the architecture
        #[inline]
        pub unsafe fn write_raw(value: u64) {
            let low = value as u32;
            let high = (value >> 32) as u32;

            #[cfg(feature = "inline_asm")]
            asm!(
                "xsetbv",
                in("ecx") 0,
                in("rax") low, in("rdx") high,
                options(nomem, nostack, preserves_flags),
            );

            #[cfg(not(feature = "inline_asm"))]
            crate::asm::x86_64_asm_xsetbv(0, low, high);
        }
    }
}
