use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_args: Arguments,
                               _file: &'static str,
                               _line: u32)
                               -> ! {
    hprint!("panicked at '");
    match () {
        #[cfg(feature = "semihosting")]
        () => {
            ::cortex_m_semihosting::io::_write_fmt(args);
        }
        #[cfg(not(feature = "semihosting"))]
        () => {}
    }
    hprintln!("', {}:{}", file, line);

    bkpt!();

    loop {}
}
