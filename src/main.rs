#![no_std]
#![no_main]

mod methods;
mod vga_buffer;

use core::panic::PanicInfo;

use methods::print_something;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"Welcome to RustyOS!!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();

    loop {}
}
