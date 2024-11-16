use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::Mutex;
use limine::request::FramebufferRequest;

use crate::font::FONT;

pub struct FramebufferWriter<'a> {
    framebuffer: Framebuffer<'a>,
    x_pos: AtomicUsize,
    y_pos: AtomicUsize,
}

unsafe impl<'a> Send for FramebufferWriter<'a> {}
unsafe impl<'a> Sync for FramebufferWriter<'a> {}

impl<'a> FramebufferWriter<'a> {
    pub fn new(framebuffer: Framebuffer<'a>) -> Self {
        Self {
            framebuffer,
            x_pos: AtomicUsize::new(0),
            y_pos: AtomicUsize::new(0),
        }
    }

    pub fn write_pixel(&self, x: usize, y: usize, color: u32) {
        let pitch = self.framebuffer.pitch() as usize;
        let bpp = (self.framebuffer.bpp() / 8) as usize;
        let pixel_offset = y * pitch + x * bpp;

        unsafe {
            *(self.framebuffer.addr().add(pixel_offset) as *mut u32) = color;
        }
    }

    pub fn write_char(&self, x: u32, y: u32, fg_color: u32, bg_color: u32, mut c: u8) {
        if c < 32 || c > 126 {
            c = '?' as u8;
        }
        
        let data: &[u8] = &FONT[c as usize * 16..(c as usize + 1) * 16];
        
        for row in 0..16 {
            let line: u8 = data[row];
            for col in 0..8 {
                let pixel_x: u32 = x + col;
                let pixel_y: u32 = y + row as u32;
                if line & (0x80 >> col) != 0 {
                    self.write_pixel(pixel_x as usize, pixel_y as usize, fg_color);
                } else {
                    self.write_pixel(pixel_x as usize, pixel_y as usize, bg_color);
                }
            }
        }
    }

    pub fn write_string(&self, x: u32, y: u32, fg_color: u32, bg_color: u32, s: &str) {
        let mut curr_x: u32 = x;
        let mut curr_y: u32 = y;

        for c in s.chars() {
            if c == '\n' {
                curr_x = x;
                curr_y += 16;
                continue;
            } 

            if curr_x + 8 > self.framebuffer.width() as u32 {
                curr_x = x;
                curr_y += 16;
            }

            if curr_y + 16 > self.framebuffer.height() as u32 {
                break;
            }

            self.write_char(curr_x, curr_y, fg_color, bg_color, c as u8);
            curr_x += 8;
        }
    }

    pub fn clear(&self, color: u32) {
        let width = self.framebuffer.width() as usize;
        let height = self.framebuffer.height() as usize;

        for y in 0..height {
            for x in 0..width {
                self.write_pixel(x, y, color);
            }
        }
    }
}

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

lazy_static! {
    pub static ref FRAMEBUFFER_WRITER: Mutex<Option<FramebufferWriter<'static>>> = Mutex::new(None);
}

pub fn init() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        // The framebuffer from the response has a 'static lifetime
        let framebuffer = framebuffer_response.framebuffers().next().unwrap();
        *FRAMEBUFFER_WRITER.lock() = Some(FramebufferWriter::new(framebuffer));
        clear_screen(0x00000000);
    }
}

pub fn clear_screen(color: u32) {
    if let Some(writer) = FRAMEBUFFER_WRITER.lock().as_ref() {
        writer.clear(color);
    }
}

pub fn write_string(s: &str, fg_color: u32, bg_color: u32) {
    FRAMEBUFFER_WRITER.lock().as_ref().unwrap().write_string(0, 0, fg_color, bg_color, s);
}
