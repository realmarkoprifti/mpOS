#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::VGA;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA.write("data");
    loop {}
}