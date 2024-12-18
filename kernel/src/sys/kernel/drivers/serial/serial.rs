use core::{fmt, sync::atomic::{AtomicUsize, Ordering}};
use spin::Mutex;
use lazy_static::lazy_static;

use crate::sys::kernel::cpu::{inb, outb};
use crate::sys::kernel::cpu::x86_64::interrupts;

static PORT: u16 = 0x3f8;
static mut BUFFER: [u8; 256] = [0; 256];
static BUFFER_LEN: AtomicUsize = AtomicUsize::new(0);

lazy_static!{
    static ref SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter::new());
}

struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write(c as u8);
        }
        Ok(())
    }
}

impl SerialWriter {
    pub fn new() -> SerialWriter {
        // first we make sure that the serial port is setup and working.
        unsafe {
            outb(PORT + 1, 0x00);    // Disable all interrupts
            outb(PORT + 3, 0x80);    // Enable DLAB (set baud rate divisor)
            outb(PORT + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
            outb(PORT + 1, 0x00);    //                  (hi byte)
            outb(PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
            outb(PORT + 2, 0xC7);    // Enable FIFO, clear them, with 14-bytethreshold
            outb(PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set
            outb(PORT + 4, 0x1E);    // Set in loopback mode, test the serial chip
            outb(PORT + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
    
            if inb(PORT + 0)  != 0xAE {
                panic!("serial port is not working!");
            }
    
            outb(PORT + 4, 0x0F);
        }

        SerialWriter
    }

    // returnstrue if there is new data on the serial port
    unsafe fn serial_recieved(&self) -> bool {
        inb(PORT + 5) & 1 != 0
    }

    // returns true if the transmit buffer is empty
    unsafe fn serial_sent(&self) -> bool {
        inb(PORT + 5) & 0x20 != 0
    }

    pub fn read(&self) -> u8 { unsafe {
        while !self.serial_recieved() {};
        return inb(PORT + 0);
    }}

    pub fn read_str_to_buffer(&mut self) { unsafe {
        while !self.serial_recieved() {};

        BUFFER_LEN.store(0, Ordering::SeqCst);

        while self.serial_recieved() && BUFFER_LEN.load(Ordering::SeqCst) < 256 {
            let c = inb(PORT + 0);
            BUFFER[BUFFER_LEN.load(Ordering::SeqCst)] = c;
            if c == b'\n' {
                break;
            }
            BUFFER_LEN.fetch_add(1, Ordering::SeqCst);
        }
    }}

    pub fn write(&self, data: u8) { unsafe {
        while !self.serial_sent() {};
        outb(PORT + 0, data);
    }}
}

pub fn _serial_write(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without(|| {
        SERIAL_WRITER.lock().write_fmt(args).unwrap();
    })    
}

#[macro_use]
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

pub fn serial_read() -> &'static str {
    serial_println!("getting value!");

    interrupts::without(|| {
        SERIAL_WRITER.lock().read_str_to_buffer();
    });

    let i = BUFFER_LEN.load(Ordering::SeqCst);

    return unsafe {
        if i != 0 {
            core::str::from_utf8(&BUFFER[..i - 1]).unwrap()
        } else { "" }
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::_serial_write(format_args!($($arg)*)));
}

