use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(args: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    hprint!("panicked at '");
    ::cortex_m_semihosting::io::_write_fmt(args);
    hprintln!("', {}:{}", file, line);

    bkpt!();

    loop {}
}
