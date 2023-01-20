#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

extern crate tm4c123x_hal;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Hello, world!").unwrap();
    loop {}
}