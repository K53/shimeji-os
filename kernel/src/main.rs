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

// #[no_mangle]
// fn show_white(i: u32) {
//     // 白色なので15
//     let a: u8 = 15;
//     // 生ポインタを使って、15を代入
//     let ptr = unsafe { &mut *(i as *mut u8) };
//     *ptr = a 
// }

// https://skoji.jp/blog/2021/04/mikan-laranja-os.html
// https://os.phil-opp.com/freestanding-rust-binary/
#[no_mangle]
pub extern "sysv64" fn kernel_main() -> ! {
    // for i in 0xa0000..0xaffff {
    //     show_white(i);
    // }
    unsafe {
        loop {asm!("hlt")}
    }
}
