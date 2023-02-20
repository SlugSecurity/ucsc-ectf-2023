//! The entry point for the tests.

#![no_main]
#![no_std]

extern crate panic_semihosting;

mod eeprom_tests;
mod timer_tests;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use tm4c123x_hal::{CorePeripherals, Peripherals};
use ucsc_ectf_util_no_std::{Runtime, RuntimePeripherals};

#[cfg(debug_assertions)]
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting tests...").unwrap();

    // Get and initialize peripherals.
    let core_peripherals = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let mut rt_peripherals = RuntimePeripherals::from((core_peripherals, peripherals));

    {
        let mut rt = Runtime::new(
            &mut rt_peripherals,
            &Default::default(),
            &Default::default(),
        );

        // Insert tests relying on runtime below. Use asserts to panic if tests fail.
        eeprom_tests::run(&mut rt.eeprom_controller);
    }

    // Insert non-runtime tests below. Use asserts to panic if tests fail.

    timer_tests::run(&rt_peripherals.hib, &mut rt_peripherals.delay);

    // Insert non-runtime tests above. Use asserts to panic if tests fail.

    writeln!(stdout, "Tests passed!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(not(debug_assertions))]
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Tests are disabled in release mode!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
