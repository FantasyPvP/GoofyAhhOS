pub mod interrupts;
pub mod gdt;
pub mod apic;

mod pics;

pub fn init() {
    gdt::init();
    interrupts::init();
}