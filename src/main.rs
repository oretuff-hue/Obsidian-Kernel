#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// ==== módulos do kernel (fora de src) ====

#[path = "../arch/mod.rs"]
pub mod arch;

#[path = "../drivers/mod.rs"]
pub mod drivers;

#[path = "../kernel/mod.rs"]
pub mod kernel;

#[path = "../memory/mod.rs"]
pub mod memory;

#[path = "../include/mod.rs"]
pub mod include;

// ==== imports ====

use ::x86_64::instructions;
use core::panic::PanicInfo;

// ==== utilitários globais ====

pub fn idle_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

/// Entry point chamado pelo assembly (multiboot / long mode)
#[no_mangle]
pub extern "C" fn kernel_main(multiboot_addr: usize) -> ! {
    kernel::init();
    memory::init(multiboot_addr);

    idle_loop();
}
