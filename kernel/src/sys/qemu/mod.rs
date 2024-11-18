use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    // Writing 0x10 will make QEMU exit with code 33 ((0x10 << 1) | 1)
    Success = 0x10,
    // Writing 0x11 will make QEMU exit with code 35 ((0x11 << 1) | 1)
    Failed = 0x11,
}

/// Exit QEMU with the given exit code.
/// 
/// This function uses the special QEMU debug exit device with
/// port 0xf4 to exit QEMU with a specified exit code.
/// QEMU will exit with code: (value << 1) | 1
pub fn exit(exit_code: ExitCode) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    loop {
        unsafe { 
            core::arch::asm!("cli; hlt", options(nomem, nostack)) 
        }
    }
}

/// Exit QEMU with a success code
/// Writing 0x10 will make QEMU exit with code 33
#[cfg(test)]
pub fn exit_success() -> ! {
    exit(ExitCode::Success)
}

/// Exit QEMU with a failed code
/// Writing 0x11 will make QEMU exit with code 35
#[cfg(test)]
pub fn exit_failed() -> ! {
    use crate::serial_println;
    serial_println!("[failed]\n");
    exit(ExitCode::Failed)
}
