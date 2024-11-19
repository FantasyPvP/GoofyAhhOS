#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

// #[cfg(test)]
// use limine::BaseRevision;

#[cfg(not(test))]
use core::panic::PanicInfo;

pub mod tests;
pub mod sys;
pub mod usr;


pub use sys::kernel::drivers::framebuffer::textwriter::{
    _print,
    _printerr,
    _log,
};

pub use sys::kernel::drivers::serial::{
    _serial_write,
    serial_read
};

pub fn init() {
    sys::kernel::cpu::init();
}

pub fn hcf() -> ! {
    loop {
        unsafe { core::arch::asm!("cli; hlt") }
    }
}

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    hcf()
}

// code for testing etc.

#[cfg(test)]
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Set up the base revision for Limine
    // static BASE_REVISION: BaseRevision = BaseRevision::new();
    
    init();
    test_main();

    serial_println!("All tests passed! exiting.");

    sys::qemu::exit_success();
}


