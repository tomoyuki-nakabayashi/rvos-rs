//! Inline assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => asm!($asm :::: "volatile"),
            }
        }
    )
}

/// Priviledged ISA Instructions
// Wait for interrupt.
instruction!(wfi, "wfi");