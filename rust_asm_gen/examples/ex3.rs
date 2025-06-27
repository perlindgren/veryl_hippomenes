#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![feature(naked_functions_rustic_abi)]
#![feature(abi_riscv_interrupt)]

use core::arch::asm;
use core::arch::naked_asm;
use core::hint::black_box;
use panic_halt as _;

// emulate interrupt vector table
#[unsafe(no_mangle)]
pub static mut INT_HANDLERS: [u32; 2] = [0; 2];

// emulates the application specific generated entry point
#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
pub extern "C" fn Reset() -> ! {
    // emulate interrupt table configuration
    unsafe {
        INT_HANDLERS[0] = handler as *const u32 as u32;
        INT_HANDLERS[1] = handler_call as *const u32 as u32;
    };

    loop {}
}

// our user provided handlers

#[unsafe(no_mangle)]
extern "riscv-interrupt-m" fn handler() {
    let r = "bar text";
    black_box(r);
}

// user provided handler
#[unsafe(no_mangle)]
extern "riscv-interrupt-m" fn handler_call() {
    bar();
}

// this will not be
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
