#![no_main]
#![no_std]

extern crate nrf51822;

#[no_mangle]
pub fn main() -> ! {
    let _y: u32;
    let _x: u32 = 0xDEADBEEF;
    _y = 0xBAAAAAAD;

    loop {}
}
