#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;
use limine::*;

use limine::request::{
    FramebufferRequest, RequestsEndMarker, RequestsStartMarker,
};

mod font;
mod render;

use crate::font::FONT;

// Set the base revision
static BASE_REVISION: BaseRevision = BaseRevision::new();


// Halt and catch fire function
fn hcf() -> ! {
    loop {
        unsafe { core::arch::asm!("cli; hlt") }
    }
}

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf()
}

// Kernel entry point
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    render::init();
    render::write_string("Welcome to GoofyAhhOS!\nthis is the superior os\nif you disagree you are a heretic in the name of steven.", 0xff0000, 0);

    hcf()
}
