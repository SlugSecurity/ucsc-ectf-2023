#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate tm4c123x;
extern crate tm4c123x_hal;

use cortex_m::prelude::{_embedded_hal_serial_Read, _embedded_hal_serial_Write};
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
    // Take the peripherals, note that there can only be one of these.
    let peripherals = Peripherals::take().unwrap();

    // Set up the clocks.
    let mut sysctl = peripherals.SYSCTL.constrain();
    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_80_00mhz),
    );
    let clocks = sysctl.clock_setup.freeze();

    // Set up the UART pins.
    let mut porta = peripherals.GPIO_PORTA.split(&sysctl.power_control);
    let mut portb = peripherals.GPIO_PORTB.split(&sysctl.power_control);

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
        if let Ok(i) = serial_gpio.read() {
            if serial_usb.write(i).is_ok() {}
        };
    }
}
