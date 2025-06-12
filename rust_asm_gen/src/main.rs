#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unsafe_code, unsafe_op_in_unsafe_fn, unsafe_attr_outside_unsafe)]
#![allow(clippy::empty_loop)]

use core::arch::asm;
use panic_halt as _;

#[unsafe(no_mangle)]
pub extern "C" fn my_code() -> ! {
    unsafe {
        asm!("add x1, x2, x3", options(noreturn),);
    }
}

pub const MAGIC: u32 = 0x1234;

#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    let x = "abcdABCD";
    let _r = core::hint::black_box(x);
    my_code();
    loop {}
}
