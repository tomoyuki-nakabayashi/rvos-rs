#![feature(global_asm)]
#![no_std]
#![no_main]

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn __start_rust(x: i32, y: i32) -> i32 {
    x + y
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}