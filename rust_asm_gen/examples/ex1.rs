#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::arch::asm;
use panic_halt as _;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
fn reset() -> ! {
    unsafe {
        asm!(
            "lui a0, 0x12345", // 20 bit
            "add a0, a0, a0",
            "addi a0, a0, 0xff",
        );
    }

    loop {}
}
