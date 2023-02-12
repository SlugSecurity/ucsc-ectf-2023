#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate tm4c123x;
extern crate tm4c123x_hal;

use core::fmt::Write;
use cortex_m_rt::entry;
use tm4c123x_hal::{
    gpio::{GpioExt, AF1},
    serial::Serial,
    sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, SysctlExt, SystemClock},
    time::Bps,
    Peripherals,
};

#[entry]
fn main() -> ! {
    const BUS_PORT: &str = match option_env!("BUS_PORT") {
        Some(s) => s,
        None => "Something wrong",
    };

    // Take the peripherals, note that there can only be one of these.
    let peripherals = Peripherals::take().unwrap();

    // Set up the clocks
    let mut sysctl = peripherals.SYSCTL.constrain();
    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_80_00mhz),
    );
    let clocks = sysctl.clock_setup.freeze();

    // Set up the UART pins.
    let mut portb = peripherals.GPIO_PORTB.split(&sysctl.power_control);

    // Initialize the serial ports.
    let mut serial_gpio = Serial::uart1(
        peripherals.UART1,
        portb.pb1.into_af_push_pull::<AF1>(&mut portb.control),
        portb.pb0.into_af_pull_up::<AF1>(&mut portb.control),
        (),
        (),
        Bps(115_200),
        tm4c123x_hal::serial::NewlineMode::Binary,
        &clocks,
        &sysctl.power_control,
    );

    loop {
        Write::write_str(&mut serial_gpio, BUS_PORT).unwrap();
        Write::write_char(&mut serial_gpio, '\n').unwrap();
    }
}
