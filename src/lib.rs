//! Hack the nrf51822

#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

extern crate compiler_builtins;
#[cfg(feature = "semihosting")]
#[macro_reexport(hprint, hprintln)]
#[macro_use]
extern crate cortex_m_semihosting;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
extern crate r0;

pub extern crate nrf51_memory_map as peripheral;

#[macro_use]
mod macros;

mod lang_items;

pub mod led;
pub mod exception;
pub mod interrupt;

// "Pre `main`" initialization routine
fn init() {
    led::init()
}
