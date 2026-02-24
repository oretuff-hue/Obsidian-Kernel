// include/obsidian/syscall.rs

use crate::drivers::serial;
use crate::include::obsidian::{types::*, errno::*};

pub const SYS_WRITE: u64 = 1;
pub const SYS_EXIT: u64 = 2;

pub fn handle_syscall(number: u64, regs: &SyscallRegs) -> i64 {
    match number {
        SYS_WRITE => {
            let ptr = regs.rdi as *const u8;
            let len = regs.rsi as usize;

            unsafe {
                for i in 0..len {
                    serial::write_byte(*ptr.add(i));
                }
            }
            SUCCESS as i64
        }
        SYS_EXIT => {
            // TODO: end process / thread
            loop {
                unsafe { core::arch::asm!("hlt"); }
            }
        }
        _ => -EINVAL as i64,
    }
}
