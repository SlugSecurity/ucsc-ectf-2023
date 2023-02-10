//! The entry point for the car firmware.

#![warn(missing_docs)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[cfg(feature = "panic-semihosting")]
extern crate panic_semihosting;

#[cfg(not(feature = "panic-semihosting"))]
extern crate panic_halt;

extern crate tm4c123x_hal;
extern crate ucsc_ectf_util;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Hello, world!").unwrap();
    loop {}
}
