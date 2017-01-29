#![no_main]
#![no_std]

extern crate nrf51822;

use nrf51822::led::Led;

#[no_mangle]
pub fn main() -> ! {
    Led.on();
    Led.off();

    loop {}
}
