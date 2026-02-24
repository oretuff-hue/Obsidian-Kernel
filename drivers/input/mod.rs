pub trait KeyboardDevice {
    fn read_scancode(&mut self) -> Option<u8>;
}

static mut KEYBOARD: Option<&'static dyn KeyboardDevice> = None;

pub fn register(device: &'static dyn KeyboardDevice) {
    unsafe {
        KEYBOARD = Some(device);
    }
}

pub fn read_scancode() -> Option<u8> {
    unsafe {
        KEYBOARD.and_then(|k| k.read_scancode())
    }
}

pub mod keyboard;
