use core::fmt::{self, Write};

pub trait SerialDevice {
    fn write_byte(&mut self, byte: u8);
}

static mut SERIAL: Option<&'static dyn SerialDevice> = None;

pub fn register(device: &'static dyn SerialDevice) {
    unsafe {
        SERIAL = Some(device);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        if let Some(dev) = SERIAL {
            dev.write_byte(byte);
        }
    }
}

pub fn write_str(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}

/// Writer para usar com `core::fmt`
pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}

pub mod uart16550;