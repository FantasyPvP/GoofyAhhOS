
use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::sys::kernel::cpu::x86_64::interrupts;

use super::{font::FONT, render::FRAMEBUFFER_WRITER};

static FONT_WIDTH: u32 = 8;
static FONT_HEIGHT: u32 = 16;

lazy_static!{
    static ref TEXT_WRITER: Mutex<TextWriter> = Mutex::new(TextWriter::new());
}


pub struct TextWriter {
    // these are measured in chars NOT pixels
    screen_width: u32,
    screen_height: u32,

    text_line: u32, // 16 pixels tall
    text_col: u32,  // 8 pixels wide

    fg_color: u32,
    bg_color: u32
}

impl TextWriter {

    pub fn new() -> Self {
        if let Some(writer) = FRAMEBUFFER_WRITER.lock().as_mut() {
            Self {
                screen_width: writer.width() as u32 / 8,
                screen_height: writer.height() as u32 / 16,
                text_line: 0,
                text_col: 0,
                fg_color: 0xFFFFFF,
                bg_color: 0x000000
            }
        } else {
            panic!("Framebuffer writer not initialized");
        }
    }

    pub fn write_char(&mut self, mut c: u8) {
        if c == b'\n' {
            self.newline();
            return;
        }

        if c < 32 || c > 126 {
            c = '?' as u8;
        }

        // get the character data from the font array. -- each byte is a row of pixels
        let data: &[u8] = &FONT[c as usize * 16..(c as usize + 1) * 16];

        if let Some(writer) = FRAMEBUFFER_WRITER.lock().as_mut() {
            for row in 0..16 {
                let line: u8 = data[row];
                for col in 0..8 {
                    let pixel_x: u32 = self.text_col * FONT_WIDTH + col;
                    let pixel_y: u32 = self.text_line * FONT_HEIGHT + row as u32;
    
                    if line & (0x80 >> col) != 0 {
                        // write the foreground color
                        writer.write_pixel(pixel_x as usize, pixel_y as usize, self.fg_color);
                    } else {
                        // write the background color
                        writer.write_pixel(pixel_x as usize, pixel_y as usize, self.bg_color);
                    }
                }
            }
        }   

        // go to next position
        if self.text_col + 1 >= self.screen_width {
            self.newline();
        } else {
            self.text_col += 1;
        }
    }

    pub fn next_char(&mut self) {
        self.text_col += 1;
    }

    pub fn newline(&mut self) {
        self.text_col = 0;

        if self.text_line + 1 >= self.screen_height {
            self.text_line = 0;
        } else {
            self.text_line += 1;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c as u8);
        }
    }

    pub fn set_colour(&mut self, col: (u32, u32)) {
        self.fg_color = col.0;
        self.bg_color = col.1;
    }

    pub fn reset_colour(&mut self) {
        self.fg_color = 0xFFFFFF;
        self.bg_color = 0x000000;
    }
}

impl core::fmt::Write for TextWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

fn write(args: fmt::Arguments, fg_color: u32, bg_color: u32) {
    use core::fmt::Write;

    interrupts::without(|| {
        let mut writer = TEXT_WRITER.lock();
        writer.set_colour((fg_color, bg_color));
        writer.write_fmt(args).unwrap();
        writer.reset_colour();
    });
}

pub fn _print(args: fmt::Arguments) {
    write(args, 0xFFFFFF, 0x000000);
}

pub fn _printerr(args: fmt::Arguments) {
    write(args, 0xFF8080, 0x000000);
}

pub fn _log(args: fmt::Arguments) {
    write(args, 0xFFFF00, 0x000000);
}

pub fn clear_screen() {
    interrupts::without(|| {
        let mut writer = TEXT_WRITER.lock();
        writer.text_line = 0;
        writer.text_col = 0;
    
        if let Some(writer) = FRAMEBUFFER_WRITER.lock().as_mut() {
            writer.clear();
        }
    });    
}

#[macro_export]
macro_rules! println_log {
	() => ($crate::print_log!("\n"));
	($($arg:tt)*) => ($crate::print_log!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_log {
	($($arg:tt)*) => ($crate::_log(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printlnerr {
    () => ($crate::printerr!("\n"));
    ($($arg:tt)*) => ($crate::printerr!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printerr {
    ($($arg:tt)*) => ($crate::_printerr(format_args!($($arg)*)));
}