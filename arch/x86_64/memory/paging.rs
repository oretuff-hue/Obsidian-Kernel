use crate::arch::x86_64::memory::frame_alloc;
use core::arch::asm;

const PAGE_SIZE: u64 = 4096;
const ENTRY_COUNT: usize = 512;

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER: u64 = 1 << 2;

static mut PML4: *mut u64 = 0 as *mut u64;

pub fn init() {
    unsafe {
        PML4 = create_pml4();

        // Identity map do kernel (0x100000 .. 0x2000000 por exemplo)
        map_range(0x100000, 0x100000, 16 * 1024 * 1024, PRESENT | WRITABLE);

        // Higher half kernel
        map_range(0xffffffff80000000, 0x100000, 16 * 1024 * 1024, PRESENT | WRITABLE);

        // Ativa paging
        enable_paging(PML4);
    }
}

// Cria PML4 vazio
unsafe fn create_pml4() -> *mut u64 {
    let frame = frame_alloc::alloc_frame().expect("No free frame for PML4") as *mut u64;
    for i in 0..ENTRY_COUNT {
        *frame.add(i) = 0;
    }
    frame
}

// Mapear range de memória (4KiB páginas)
unsafe fn map_range(virt_start: u64, phys_start: u64, size: u64, flags: u64) {
    let mut offset = 0;
    while offset < size {
        let va = virt_start + offset;
        let pa = phys_start + offset;

        map_page(va, pa, flags);
        offset += PAGE_SIZE;
    }
}

// Mapear página única (4KiB)
unsafe fn map_page(virt_addr: u64, phys_addr: u64, flags: u64) {
    // Simplificação: PML4, PDPT, PD, PT alocados na hora
    // Para cada nível, criar se necessário
    // Código completo de paging x86_64 é longo, mas aqui é early boot safe
    unimplemented!("map_page deve criar entradas PT e setar flags")
}

// Ativar paging (CR3)
unsafe fn enable_paging(pml4_addr: *mut u64) {
    asm!(
        "mov cr3, {0}",
        in(reg) pml4_addr,
        options(nostack, preserves_flags)
    );
}

// Mapeia heap do kernel e retorna endereço virtual inicial
pub fn map_kernel_heap(size: usize) -> usize {
    const HEAP_START: u64 = 0xffff_ffff_9000_0000;

    unsafe {
        map_range(HEAP_START, frame_alloc::alloc_frame().unwrap(), size as u64, PRESENT | WRITABLE);
    }

    HEAP_START as usize
}
