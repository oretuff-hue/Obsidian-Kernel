use crate::drivers::serial;
use crate::include::obsidian::{types::*, errno::*};
use crate::kernel::state;

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
            let code = regs.rdi as u64;

            serial::write_str("Program exited with code ");
            serial::write_num(code);
            serial::write_str("\n");

            state::stop_program();

            SUCCESS as i64
        }

        _ => -EINVAL as i64,
    }
}