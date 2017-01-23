/// P8
pub struct Led;

impl Led {
    pub fn off(&self) {
        unsafe { ::peripheral::gpio_mut().outclr.write(|w| w.pin8(true)) }
    }

    pub fn on(&self) {
        unsafe { ::peripheral::gpio_mut().outset.write(|w| w.pin8(true)) }
    }
}

pub fn init() {
    unsafe {
        // configure P8 as output
        ::peripheral::gpio_mut().dirset.write(|w| w.pin8(true));
    }
}
