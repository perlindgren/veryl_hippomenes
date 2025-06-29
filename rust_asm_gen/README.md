# Rust Asm Gen

## Prerequisites

- Latest Rust toolchain.
- The `riscv32i-unknown-none-elf` target for RV32I:

  ```shell
  rustup target add riscv32i-unknown-none-elf
  ```

- The `llvm-tools-preview` (for `cargo nm`/`objdump` etc.)

  ```shell
  rustup component add llvm-tools-preview
  ```

## Rust code generation

Rust allows for highly optimized code based on aggressive optimizations performed by LLVM and linker backend passes. The aggressive optimization removes all unreachable code and data, thus we have to make sure that LLVM is correctly able to identify the entry point, and from there span reachable code and data.

For sake of completeness we provide a self contained build and linking environment as follows:

## Linker

The file `link.x` provides the configuration for (LLVM) `lld`. The target architecture is a Harvard architecture with two memory regions `CODE` (for executable code) and `RAM` (for mutable data and constants). We do not have non-volatile memory, and as the `CODE` region is inaccessible to the executing code, we place constants in `RAM`. To ensure correct spanning of reachable code we set `Entry(Reset)`.

``` text
MEMORY
{
  CODE : ORIGIN = 0x00000000, LENGTH = 1K
  RAM : ORIGIN = 0x00010000, LENGTH = 1K
} ENTRY(Reset) 

SECTIONS
{
  .reset :
  {
    *(.reset);
  } > CODE

  .text :
  {
    *(.text .text.*);
  } > CODE

  .rodata :
  {
    *(.rodata .rodata.*);
  } > RAM

  .data :
  {
    *(.data .data.*);
  } > RAM
}

```

The `.cargo/config.toml` defines the compilation environment:

``` toml
[build]
target = "riscv32i-unknown-none-elf"

rustflags = ["-C", "link-arg=-Tlink.x"]
```

## Example ex1

A minimal Rust example is provided in `examples/ex1.rs`:

``` rust
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::arch::asm;
use panic_halt as _;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
pub extern "C" fn Reset() -> ! {
    unsafe {
        asm!(
            "lui a0, 0x12345", // 20 bit
            "add a0, a0, a0",
            "addi a0, a0, 0xff",
        );
    }

    loop {}
}
```

Inspecting the generated code:

``` shell
cargo nm --example ex1 --release
    Finished `release` profile [optimized] target(s) in 0.00s
00000000 T Reset
```

As expected we have a single symbol `Reset` placed at address 0. Further inspection of the binary:

``` shell
cargo objdump --example ex1 --release -- -d
    Finished `release` profile [optimized] target(s) in 0.00s

ex1:    file format elf32-littleriscv

Disassembly of section .reset:

00000000 <Reset>:
       0: 12345537      lui     a0, 0x12345
       4: 00a50533      add     a0, a0, a0
       8: 0ff50513      addi    a0, a0, 0xff
       c: 0000006f      j       0xc <Reset+0xc>
```

## Example ex2

The Rust RTIC framework provides a means to model a system in terms of concurrent tasks with shared resources. RTIC generates the bindings as shown in `example/ex2.rs`. In the example `black_box` is used to prevent out-optimization.

```rust
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![feature(naked_functions_rustic_abi)]

use core::arch::asm;
use core::arch::naked_asm;
use core::hint::black_box;
use panic_halt as _;

// emulate interrupt vector table
#[unsafe(no_mangle)]
pub static mut INT_VEC: [u32; 2] = [0; 2];

// application specific generated trampoline
#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn trampoline() {
    naked_asm!("jal bar", "mret",);
}

// emulates the application specific generated entry point
#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset")]
pub extern "C" fn Reset() -> ! {
    // emulate interrupt table configuration
    unsafe {
        INT_VEC[0] = trampoline as *const u32 as u32;
    };

    loop {}
}

// our user provided handler

// this will be
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
```
