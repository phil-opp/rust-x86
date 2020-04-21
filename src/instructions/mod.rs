#![cfg(target_arch = "x86_64")]

//! Special x86_64 instructions.

pub mod interrupts;
pub mod port;
pub mod random;
pub mod segmentation;
pub mod tables;
pub mod tlb;

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    #[cfg(feature = "inline_asm")]
    unsafe {
        llvm_asm!("hlt" :::: "volatile");
    }

    #[cfg(not(feature = "inline_asm"))]
    unsafe {
        crate::asm::x86_64_asm_hlt();
    }
}

/// Emits a '[magic breakpoint](https://wiki.osdev.org/Bochs#Magic_Breakpoint)' instruction for the [Bochs](http://bochs.sourceforge.net/) CPU
/// emulator. Make sure to set `magic_break: enabled=1` in your `.bochsrc` file.
#[cfg(feature = "inline_asm")]
#[inline]
pub fn bochs_breakpoint() {
    unsafe {
        llvm_asm!("xchgw %bx, %bx" :::: "volatile");
    }
}
