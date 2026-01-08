#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
use drivers::framebuffer::Framebuffer;
use drivers::keyboard;
use drivers::serial;

#[no_mangle]
pub extern "C" fn kernel_main(mb_addr: usize) -> ! {
    // inicializa framebuffer
    let fb = Framebuffer::new(mb_addr);
    fb.clear(0x00000000); // limpa tela preta

    // desenha um retângulo branco
    for y in 200..400 {
        for x in 300..700 {
            fb.put_pixel(x, y, 0x00FFFFFF);
        }
    }

    // inicializa serial (debug)
    serial::init();
    serial::write_string("Kernel iniciou!\n");

    loop {
        // lê scancode do teclado
        let _sc = keyboard::read_scancode();
        // aqui você poderia processar e desenhar algo no framebuffer
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::write_string("Kernel panic!\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}
