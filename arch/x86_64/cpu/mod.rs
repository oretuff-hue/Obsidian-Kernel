pub mod gdt;
pub mod idt;
pub mod interrupts;

pub fn init() {
    gdt::init();
    idt::init();
    interrupts::enable();
}
