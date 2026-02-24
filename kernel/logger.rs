use core::fmt::{self, Write};
use spin::Mutex;

use crate::drivers::serial;
use crate::drivers::video::framebuffer::Framebuffer;

static LOGGER: Mutex<Logger> = Mutex::new(Logger {
    framebuffer: None,
});

struct Logger {
    framebuffer: Option<Framebuffer>,
}

pub fn init() {
    // for now, nothing more than ensuring that the mutex exists
}

pub fn set_framebuffer(fb: Framebuffer) {
    let mut logger = LOGGER.lock();
    logger.framebuffer = Some(fb);
}

pub fn info(msg: &str) {
    log("[INFO] ", msg);
}

pub fn warn(msg: &str) {
    log("[WARN] ", msg);
}

pub fn error(msg: &str) {
    log("[ERROR] ", msg);
}

fn log(prefix: &str, msg: &str) {
    // Serial FOREVER
    serial::write_str(prefix);
    serial::write_str(msg);
    serial::write_str("\n");

    // optional Framebuffer
    let mut logger = LOGGER.lock();
    if let Some(_fb) = logger.framebuffer.as_mut() {
        // FUTURE:
        // draw text on framebuffer
        // for now, keeps only serial
    }
}

/* ---------- support for formatting ---------- */

pub fn info_fmt(args: fmt::Arguments) {
    log_fmt("[INFO] ", args);
}

pub fn error_fmt(args: fmt::Arguments) {
    log_fmt("[ERROR] ", args);
}

fn log_fmt(prefix: &str, args: fmt::Arguments) {
    serial::write_str(prefix);
    let _ = SerialWriter.write_fmt(args);
    serial::write_str("\n");
}

struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial::write_str(s);
        Ok(())
    }
}
