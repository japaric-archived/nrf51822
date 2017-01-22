#![no_std]
#![no_main]

#[macro_use]
extern crate nrf51822;

#[no_mangle]
pub fn main() -> ! {
    hprintln!("Hello, world!");

    loop {}
}
