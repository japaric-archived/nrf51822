#![no_main]
#![no_std]

extern crate nrf51822;

use nrf51822::peripheral;
use nrf51822::led::Led;

#[no_mangle]
pub fn main() -> ! {
    let timer0 = unsafe { peripheral::timer0_mut() };

    // configure as timer
    timer0.mode.write(|w| w.mode(false));

    // 24-bit timer
    timer0.bitmode.write(|w| w.bitmode(2));

    // prescaler = 2 ^ 4 = 16
    timer0.prescaler.write(|w| w.prescaler(4));

    // clear the timer after the `compare` value is reached
    timer0.shorts.modify(|_, w| w.compare0_clear(true));

    // reset the timer after 1_000_000 ticks
    // NOTE clock frequency = 16 MHz
    timer0.cc0.write(1_000_000);

    // start the timer
    timer0.tasks_start.write(1);

    let mut on = true;
    loop {
        if on {
            Led.on();
        } else {
            Led.off();
        }

        on = !on;

        while timer0.events_compare0.read() == 0 {}
        timer0.events_compare0.write(0);
    }
}
