#![no_std]
#![no_main]

mod methods;
mod vga_buffer;

use core::panic::PanicInfo;

// use methods::print_something;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // print_something();

    use core::fmt::Write;

    vga_buffer::WRITER
        .lock()
        .write_str("Welcome to RustyOS")
        .unwrap();

    write!(vga_buffer::WRITER.lock(), ", and this is its {}!", "BIOS").unwrap();

    loop {}
}
