#[repr(C)]
struct MultibootTag {
    typ: u32,
    size: u32,
}

const TAG_FRAMEBUFFER: u32 = 8;

#[repr(C)]
pub struct FramebufferTag {
    pub typ: u32,
    pub size: u32,
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub fb_type: u8,
    _reserved: u16,
}

pub struct Framebuffer {
    tag: &'static FramebufferTag,
}

impl Framebuffer {
    pub fn new(mb_addr: usize) -> Self {
        Self {
            tag: unsafe { find_framebuffer(mb_addr) },
        }
    }

    pub fn put_pixel(&self, x: u32, y: u32, color: u32) {
        let ptr = self.tag.addr as *mut u8;
        let offset = (y * self.tag.pitch + x * 4) as isize;
        unsafe {
            *(ptr.offset(offset) as *mut u32) = color;
        }
    }

    pub fn clear(&self, color: u32) {
        for y in 0..self.tag.height {
            for x in 0..self.tag.width {
                self.put_pixel(x, y, color);
            }
        }
    }
}

unsafe fn find_framebuffer(mb_addr: usize) -> &'static FramebufferTag {
    let mut tag = (mb_addr + 8) as *const MultibootTag;

    loop {
        match (*tag).typ {
            TAG_FRAMEBUFFER => {
                return &*(tag as *const FramebufferTag);
            }
            0 => break, // END tag
            _ => {}
        }

        tag = ((tag as usize + (*tag).size as usize + 7) & !7)
            as *const MultibootTag;
    }

    panic!("Framebuffer tag not found");
}
