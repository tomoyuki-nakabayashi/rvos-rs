#![feature(global_asm)]
#![no_std]

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn __start_rust(x: i32, y: i32) -> i32 {
    let uart_16550 = 0x1000_0000 as *mut u8;
    unsafe {
        *uart_16550 = 'a' as u8;
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}