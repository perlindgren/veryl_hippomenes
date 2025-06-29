#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![feature(abi_riscv_interrupt)]

use core::hint::black_box;
use panic_halt as _;

// emulate interrupt vector table
#[unsafe(no_mangle)]
pub static mut INT_VEC: [u32; 2] = [0; 2];

// emulates the application specific generated entry point
#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
fn reset() -> ! {
    // emulate interrupt table configuration
    unsafe {
        INT_VEC[0] = int0 as *const u32 as u32;
        INT_VEC[1] = int1 as *const u32 as u32;
    };

    loop {}
}

// user provided handlers
#[unsafe(no_mangle)]
extern "riscv-interrupt-m" fn int0() {
    let r = "bar text";
    black_box(r);
}

#[unsafe(no_mangle)]
extern "riscv-interrupt-m" fn int1() {
    bar();
}

// this will not be inlined
#[unsafe(no_mangle)]
#[inline(never)]
fn bar() {
    let r = "bar text";
    black_box(r);
    baz();
}

// this will be inlined
fn baz() {
    let r = "baz text";
    black_box(r);
}
