#![no_std]
#![no_main]

mod console;
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // print_something();

    // use core::fmt::Write;

    // vga_buffer::WRITER
    //     .lock()
    //     .write_str("Welcome to RustyOS")
    //     .unwrap();

    // write!(vga_buffer::WRITER.lock(), ", and this is its {}!", "BIOS").unwrap();

    println!("Hi {}, welcome to {} BIOS!!", "TheCodeHeist", "RustyOS");

    panic!("You are the reason for my panicking!!!");

    // loop {}
}
