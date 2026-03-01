use crate::drivers::serial;
use crate::kernel::logger;
use crate::memory;
use core::sync::atomic::Ordering;
use crate::kernel::state::PROGRAM_RUNNING;

pub fn init(mb_addr: usize) -> ! {
    serial::init();
    logger::init();

    logger::info("Obsidian Kernel v0.0.3-pre-alpha");
    logger::info("Entering 64-bit long mode");

    memory::init(mb_addr);

    logger::info("Memory initialized");
    logger::info("Kernel ready");

    kernel_loop()
}

fn kernel_loop() -> ! {
    loop {
        if !PROGRAM_RUNNING.load(Ordering::SeqCst) {
            idle();
        }

        unsafe { core::arch::asm!("hlt"); }
    }
}

fn idle() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}