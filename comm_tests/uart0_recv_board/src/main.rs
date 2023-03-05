#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal;

use cortex_m_rt::entry;
use tm4c123x_hal::{CorePeripherals, Peripherals};
use ucsc_ectf_comm_tests_common::run_recv_tests;
use ucsc_ectf_util_no_std::{communication::TxChannel, Runtime};

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

    run_recv_tests(&mut rt.uart0_controller, |d| {
        rt.hib_controller.create_timer(d)
    });

    // Indicate finish receiving here
    rt.uart0_controller
        .send(&mut [b's', b'u', b'c', b'c', b'e', b's', b's'])
        .expect("Couldn't send success");

    #[allow(clippy::empty_loop)]
    loop {}
}
