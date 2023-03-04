#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate tm4c123x_hal;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use tm4c123x_hal::{CorePeripherals, Peripherals};
use ucsc_ectf_comm_tests_common::run_send_tests;
use ucsc_ectf_util_no_std::Runtime;

#[entry]
fn main_test() -> ! {
    let peripherals = (
        CorePeripherals::take().unwrap(),
        Peripherals::take().unwrap(),
    );
    let mut rt_peripherals = peripherals.into();
    let mut rt = Runtime::new(
        &mut rt_peripherals,
        &Default::default(),
        &Default::default(),
    );

    run_send_tests(&mut rt.uart0_controller, |d| {
        rt.hib_controller.create_timer(d)
    });

    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Finished sending!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
