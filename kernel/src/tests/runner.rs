use crate::{serial_print, serial_println};

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests\n", tests.len());
    for test in tests {
        test.run();
    }
}

pub trait Testable {
    fn run(&self) -> ();
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