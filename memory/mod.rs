pub mod map;
pub mod heap;

use crate::arch::x86_64::memory::frame_alloc;
use crate::kernel::logger;

pub fn init(mb_addr: usize) {
    logger::info("Loading memory map");

    let memory_map = unsafe { map::from_multiboot(mb_addr) };

    frame_alloc::init(&memory_map);

    logger::info("Frame allocator ready");

    const HEAP_SIZE: usize = 1024 * 1024;
    let heap_start = 0x4000000;

    unsafe {
        heap::init_heap(heap_start, HEAP_SIZE);
    }

    logger::info("Heap ready");
}