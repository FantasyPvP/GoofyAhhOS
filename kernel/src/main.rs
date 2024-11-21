#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(GoofyAhhOS::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use limine::*;

use GoofyAhhOS::sys::kernel::drivers::serial::serial_read;
use GoofyAhhOS::{print, println, serial_println};
use GoofyAhhOS::sys::kernel::drivers::framebuffer::textwriter::clear_screen;

// Set the base revision
static BASE_REVISION: BaseRevision = BaseRevision::new();

// Kernel entry point
#[no_mangle]
pub extern "C" fn kmain() -> ! {

    GoofyAhhOS::init();

    println!("Hello from GoofyAhhOS!");
    // serial_println!("SERIAL OUT ACHIEVED :check:");

    loop {}

    loop {
        let input: &str = serial_read();

        clear_screen();

        if input.starts_with("print ") {
            let input = &input[6..];
            println!("{}", input);
        } else if input == "" {
            let x: i32 = 239423889;
            let y: i32 = 123456678;

            print!("num: {} {}", x, y);
        } else {
            println!("Unknown command: {}", input);
        }
    }

    GoofyAhhOS::hcf()
}

