#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![feature(naked_functions_rustic_abi)]

use core::arch::naked_asm;
use core::hint::black_box;
use panic_halt as _;

// emulate interrupt vector table
#[unsafe(no_mangle)]
pub static mut INT_VEC: [u32; 2] = [0; 2];

// application specific generated trampoline
#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn trampoline_push_pop() {
    naked_asm!(
        "
        # push return address on shared stack
        addi    sp, sp, -4
        sw      ra, 0(sp) 
        jal     bar
        # pop return address from shared stack
        lw      ra, 0(sp)
        addi    sp, sp, 4
        mret
        "
    );
}

// application specific generated trampoline
#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn trampoline() {
    naked_asm!(
        "
        jal     bar
        mret
        "
    );
}

// emulates the application specific generated entry point
#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
pub extern "C" fn reset() -> ! {
    // emulate interrupt table configuration
    unsafe {
        INT_VEC[0] = trampoline_push_pop as *const u32 as u32;
        INT_VEC[1] = trampoline as *const u32 as u32;
    };

    loop {}
}

// our user provided handler

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
