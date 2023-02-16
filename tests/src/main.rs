//! The entry point for the tests.

#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate tm4c123x_hal;
extern crate ucsc_ectf_util;

mod eeprom_tests;

use core::fmt::Write;
use cortex_m_rt::entry;
use tm4c123x_hal::{
    gpio::{GpioExt, AF1},
    serial::Serial,
    sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, SysctlExt, SystemClock},
    time::Bps,
    Peripherals,
};

#[cfg(debug_assertions)]
#[entry]
fn main() -> ! {
    // Get and initialize peripherals.
    let mut peripherals = Peripherals::take().unwrap();
    let mut sysctl = peripherals.SYSCTL.constrain();

    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_80_00mhz),
    );

    let clocks = sysctl.clock_setup.freeze();

    // Set up the UART pins.
    let mut porta = peripherals.GPIO_PORTA.split(&sysctl.power_control);

    // Initialize the serial ports.
    let mut serial_usb = Serial::uart0(
        peripherals.UART0,
        porta.pa1.into_af_push_pull::<AF1>(&mut porta.control),
        porta.pa0.into_af_pull_down::<AF1>(&mut porta.control),
        (),
        (),
        Bps(115_200),
        tm4c123x_hal::serial::NewlineMode::Binary,
        &clocks,
        &sysctl.power_control,
    );

    writeln!(serial_usb, "Beginning tests...").unwrap();

    // Insert test module runs below. Use asserts to panic if tests fail.

    eeprom_tests::run(&mut peripherals.EEPROM, &sysctl.power_control);

    // Insert test module runs above. Use asserts to panic if tests fail.

    writeln!(serial_usb, "Tests passed!").unwrap();
    loop {}
}

#[cfg(not(debug_assertions))]
#[entry]
fn main() -> ! {
    // Get and initialize peripherals.
    let peripherals = Peripherals::take().unwrap();
    let mut sysctl = peripherals.SYSCTL.constrain();

    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_80_00mhz),
    );

    let clocks = sysctl.clock_setup.freeze();

    // Set up the UART pins.
    let mut porta = peripherals.GPIO_PORTA.split(&sysctl.power_control);

    // Initialize the serial ports.
    let mut serial_usb = Serial::uart0(
        peripherals.UART0,
        porta.pa1.into_af_push_pull::<AF1>(&mut porta.control),
        porta.pa0.into_af_pull_down::<AF1>(&mut porta.control),
        (),
        (),
        Bps(115_200),
        tm4c123x_hal::serial::NewlineMode::Binary,
        &clocks,
        &sysctl.power_control,
    );

    writeln!(serial_usb, "Tests are disabled in release mode!").unwrap();

    loop {}
}
