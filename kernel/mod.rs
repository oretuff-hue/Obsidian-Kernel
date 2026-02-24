pub mod init;
pub mod logger;
pub mod panic;

pub fn init() {
    logger::init();
}