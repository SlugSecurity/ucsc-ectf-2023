//! The entry point for the tests.

#![warn(missing_docs)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate tm4c123x_hal;
extern crate ucsc_ectf_util;

mod eeprom_tests;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Beginning tests...").unwrap();

    // Insert test module runs below. Use asserts to panic if tests fail.

    eeprom_tests::run();

    // Insert test module runs above. Use asserts to panic if tests fail.

    write!(stdout, "Tests passed!").unwrap();
    loop {}
}
