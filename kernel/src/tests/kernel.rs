#[cfg(test)]
use x86_64::instructions::interrupts::int3;
#[cfg(test)]
use crate::println;

#[test_case]
pub fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }   
}

#[test_case]
pub fn test_println_output() {
    println!("test_println output");
}

#[test_case]
pub fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    int3();
}
