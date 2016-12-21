#![no_main]
#![no_std]

extern crate nrf51822;

#[no_mangle]
pub fn main() -> ! {
    let y: u32;
    let x: u32 = 0xDEADBEEF;
    y = 0xBAAAAAAD;

    loop {}
}
