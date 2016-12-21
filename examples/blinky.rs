#![no_main]
#![no_std]

extern crate nrf51822;

use nrf51822::peripheral;

#[no_mangle]
pub fn main() -> ! {
    let gpio = unsafe { peripheral::gpio_mut() };
    let timer0 = unsafe { peripheral::timer0_mut() };

    // configure P8 as output
    gpio.dirset.write(|w| w.pin8(true));

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
            // set P8 high
            gpio.outset.write(|w| w.pin8(true));
        } else {
            // set P8 low
            gpio.outclr.write(|w| w.pin8(true));
        }

        on = !on;

        while timer0.events_compare0.read() == 0 {}
        timer0.events_compare0.write(0);
    }
}
