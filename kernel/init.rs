use crate::drivers::serial; // porque proxy é drivers.rs que faz mod drivers
use crate::drivers::video::framebuffer::Framebuffer;
use crate::kernel::logger;  // proxy kernel.rs contém mod kernel
use crate::memory;          // proxy memory.rs contém mod memory
use crate::arch::x86_64::memory::{frame_alloc, paging}; // proxy x86_64.rs
use crate::drivers::video::VideoDevice;
use crate::idle_loop;
use crate::arch::x86_64::init;

pub fn init(mb_addr: usize) -> ! {
    // serial::init();
    logger::init();

    logger::info("Route Kernel starting...");

    crate::arch::x86_64::init();
    memory::init(mb_addr);

    let fb = Framebuffer::new(mb_addr);
    fb.clear(0x00000000);

    logger::info("Kernel initialized");
    idle_loop();
}
