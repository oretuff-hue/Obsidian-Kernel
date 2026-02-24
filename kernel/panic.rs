use core::panic::PanicInfo;
use crate::kernel::logger;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logger::error("KERNEL PANIC");

    if let Some(msg) = info.message() {
        use core::fmt::Write;
        logger::error("Mensagem:");
        log_message(msg);
    }

    if let Some(loc) = info.location() {
        logger::error("Local:");
        log_location(loc);
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn log_message(msg: &core::fmt::Arguments) {
    use crate::drivers::serial;
    serial::write_str("  ");
    serial::write_byte(*msg);
    serial::write_str("\n");
}

fn log_location(loc: &core::panic::Location) {
    use crate::drivers::serial;
    serial::write_str("  ");
    serial::write_str(loc.file());
    serial::write_str(":");
    serial::write_str(loc.line() as u64);
    serial::write_str("\n");
}
