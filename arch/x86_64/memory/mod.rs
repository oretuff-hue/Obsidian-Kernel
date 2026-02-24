pub mod paging;
pub mod frame_alloc;

use crate::memory::map;
use crate::kernel::logger;
use crate::memory::map::MemoryRegionType;

pub fn init(mb_addr: usize) {
    logger::info("Inicializando subsistema de memória");

    // Parser memory map of Multiboot2
    let mut regions = [map::MemoryRegion {
        start: 0,
        end: 0,
        region_type: map::MemoryRegionType::Reserved,
    }; 64]; // adjust if needs more regions

    let mut count = 0;
    unsafe {
        map::parse_memory_map(mb_addr, |region| {
            if count < regions.len() {
                regions[count] = region;
                count += 1;
            }
        });
    }

    let memory_map = map::MemoryMap {
        regions: &regions[..count],
    };

    logger::info("Mapa de memória parseado");

    // Initalize physic frame allocator
    frame_alloc::init(&memory_map);
    logger::info("Frame allocator inicializado");

    // Initializes paging (needed to map kernel + heap)
    paging::init();
    logger::info("Paging inicializado");

    // Map heap (optional here, but needed before init_heap)
    // let heap_start = paging::map_kernel_heap(HEAP_SIZE);
    // unsafe { heap::init_heap(heap_start, HEAP_SIZE); }
}
