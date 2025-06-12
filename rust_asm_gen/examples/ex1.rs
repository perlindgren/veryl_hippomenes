#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::arch::asm;
use panic_halt as _;

#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    unsafe {
        asm!(
            "lui x1, 0x12345", // 20 bit
            "add x1, x2, x3",
            "addi x1, x2, 0xff",
            "jal 0x1234"
        );
    }

    loop {}
}
