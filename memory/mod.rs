pub mod map;
pub mod heap;

use crate::arch::x86_64::memory::{frame_alloc, paging};
use crate::kernel::logger;

pub fn init(mb_addr: usize) {
    logger::info("Inicializando subsistema de memória");

    // 1. Ler mapa de memória do Multiboot2
    let memory_map = map::from_multiboot(mb_addr);
    logger::info("Mapa de memória carregado");

    // 2. Inicializar frame allocator físico
    frame_alloc::init(&memory_map);
    logger::info("Frame allocator inicializado");

    // 3. Inicializar paging (MMU)
    paging::init();
    logger::info("Paging inicializado");

    // 4. Mapear heap do kernel
    const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB
    let heap_start = paging::map_kernel_heap(HEAP_SIZE);

    unsafe {
        heap::init_heap(heap_start, HEAP_SIZE);
    }

    logger::info("Heap do kernel inicializado");
}
