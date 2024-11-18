//! CPU I/O port operations

use core::arch::asm;

/// Read a byte from the specified I/O port
/// 
/// # Safety
/// This function is unsafe because it performs direct I/O port operations
/// which could have unpredictable effects on hardware
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// Write a byte to the specified I/O port
/// 
/// # Safety
/// This function is unsafe because it performs direct I/O port operations
/// which could have unpredictable effects on hardware
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

/// Read a word (16 bits) from the specified I/O port
/// 
/// # Safety
/// This function is unsafe because it performs direct I/O port operations
/// which could have unpredictable effects on hardware
#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let value: u16;
    asm!(
        "in ax, dx",
        out("ax") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// Write a word (16 bits) to the specified I/O port
/// 
/// # Safety
/// This function is unsafe because it performs direct I/O port operations
/// which could have unpredictable effects on hardware
#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    asm!(
        "out dx, ax",
        in("dx") port,
        in("ax") value,
        options(nomem, nostack, preserves_flags)
    );
}
