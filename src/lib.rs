//! An OS for RISC-V processor.
#![no_std]

extern crate bare_metal;

//mod asm;
mod io;

use io::uart::Uart;

static HELLO: &[u8] = b"Hello from Rust!";

#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
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