//! An OS for RISC-V processor.
#![feature(global_asm)]
#![feature(asm)]
#![no_std]
#![no_main]

mod asm;
mod io;

use io::uart::Uart;

static HELLO: &[u8] = b"Hello from Rust!";

#[no_mangle]
pub fn __start_rust() -> ! {
    let uart1 = Uart::new(0x10013000 as *mut u8);
    
    for c in HELLO {
        uart1.write(*c);
    }
    
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// `abort` is needed when linking.
#[cfg(target_arch = "riscv32")]
global_asm!(r#"
.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    addi    x1, zero, 0

    /* Set up stack pointer. */
    addi    sp, zero, 0
    lui     sp, %hi(stacks + 1024)
    ori     sp, sp, %lo(stacks + 1024)

    /* Now jump to the rust world; __start_rust.  */
    auipc   t0, %hi(__start_rust)
    addi    t0, t0, %lo(__start_rust)
    j       __start_rust

abort:
    jal    zero, abort
.bss

    .global stacks
stacks:
    .skip 1024
"#);