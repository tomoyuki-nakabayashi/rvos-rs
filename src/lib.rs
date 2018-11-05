//! An OS for RISC-V processor.
#![feature(global_asm)]
#![feature(asm)]
#![no_std]

extern crate bare_metal;

mod asm;
mod io;

use io::uart::Uart;
use bare_metal::Mutex;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn __start_rust() -> ! {
    let uart_16550 = Uart::new(0x1000_0000 as *mut u8);
    uart_16550.write('a' as u8);


    unsafe { asm::wfi(); }
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}