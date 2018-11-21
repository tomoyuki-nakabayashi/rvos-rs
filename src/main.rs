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

    unsafe { asm::wfi() };
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

// `abort` is needed when linking.
#[cfg(target_arch = "riscv32")]
#[link_section = ".boot"]
global_asm!(r#"
_start:
    /* Set up stack pointer. */
    lui     sp, %hi(stack_end)
    ori     sp, sp, %lo(stack_end)

    /* Now jump to the rust world; __start_rust.  */
    j       __start_rust

.bss

stack_start:
    .skip 1024
stack_end:
"#);