#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Hello, world!").unwrap();
    loop {}
}
