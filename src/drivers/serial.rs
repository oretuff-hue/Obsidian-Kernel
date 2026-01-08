use x86_64::instructions::port::Port;

pub fn init() {
    unsafe {
        let mut port = Port::new(0x3F8);
        port.write(0x00u8); // desativar interrupções
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        let mut port = Port::new(0x3F8);
        port.write(byte);
    }
}

pub fn write_string(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}
