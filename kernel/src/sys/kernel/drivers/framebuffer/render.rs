use core::panic;
use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::Mutex;
use limine::request::FramebufferRequest;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

lazy_static! {
    pub static ref FRAMEBUFFER_WRITER: Mutex<Option<FramebufferWriter<'static>>> = Mutex::new({
        if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
            let framebuffer = framebuffer_response.framebuffers().next().unwrap();
            Some(FramebufferWriter::new(framebuffer))
        } else {
            panic!("Framebuffer request failed");
        }
    });
}

pub struct FramebufferWriter<'a> {
    framebuffer: Framebuffer<'a>,
}

unsafe impl<'a> Send for FramebufferWriter<'a> {}
unsafe impl<'a> Sync for FramebufferWriter<'a> {}

impl<'a> FramebufferWriter<'a> {
    pub fn new(framebuffer: Framebuffer<'a>) -> Self {
        Self {
            framebuffer,
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

    pub fn width(&self) -> u32 {
        self.framebuffer.width() as u32
    }

    pub fn height(&self) -> u32 {
        self.framebuffer.height() as u32
    }

    pub fn clear(&self) {
        let width = self.framebuffer.width() as usize;
        let height = self.framebuffer.height() as usize;

        for y in 0..height {
            for x in 0..width {
                self.write_pixel(x, y, 0x000000);
            }
        }
    }
}

