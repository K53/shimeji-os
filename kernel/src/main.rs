#![no_std]
#![no_main]
// #![feature(start)]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        loop {asm!("hlt")}
    }
}

#[no_mangle]
pub extern "sysv64" fn kernel_main() -> ! {
    unsafe {
        loop {asm!("hlt")}
    }
}
