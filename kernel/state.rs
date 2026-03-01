use core::sync::atomic::{AtomicBool, Ordering};

pub static PROGRAM_RUNNING: AtomicBool = AtomicBool::new(true);

pub fn stop_program() {
    PROGRAM_RUNNING.store(false, Ordering::SeqCst);
}