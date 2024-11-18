mod runner;
pub use runner::test_runner;

pub mod kernel;

/// Called on panic
/// 
/// # Arguments
/// 
/// * `info` - The panic info
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::serial_println;
    
    // print a failed message saying the kernel panicked
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    crate::sys::qemu::exit_failed();
}
