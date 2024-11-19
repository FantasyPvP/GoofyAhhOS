pub mod interrupts;
pub mod gdt;

mod pics;

pub fn init() {
    gdt::init();
    interrupts::init();
}