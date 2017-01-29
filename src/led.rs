use ::peripheral::gpio::{OutclrWPin8, OutsetWPin8, DirsetWPin8};

/// P8
pub struct Led;

impl Led {
    pub fn off(&self) {
        unsafe { ::peripheral::gpio_mut().outclr.write(|w| w.pin8(OutclrWPin8::Clear)) }
    }

    pub fn on(&self) {
        unsafe { ::peripheral::gpio_mut().outset.write(|w| w.pin8(OutsetWPin8::Set)) }
    }
}

pub fn init() {
    unsafe {
        ::peripheral::gpio_mut().dirset.write(|w| w.pin8(DirsetWPin8::Set));
    }
}
