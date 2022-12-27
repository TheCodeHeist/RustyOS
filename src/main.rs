#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod console;
mod qemu;
mod serial;
mod vga_buffer;

use crate::qemu::{
    exit_qemu,
    QemuExitCode::{Failed, Success},
};
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(Failed);

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi {}, welcome to {} BIOS!!", "TheCodeHeist", "RustyOS");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
