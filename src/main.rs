#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}, welcome to RustyOS Kernel!", "TheCodeHeist");

    rusty_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rusty_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    rusty_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}
