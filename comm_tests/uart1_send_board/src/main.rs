#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate tm4c123x_hal;

use core::{fmt::Write, time::Duration};
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use tm4c123x_hal::{CorePeripherals, Peripherals};
use ucsc_ectf_comm_tests_common::{
    run_send_tests, RxTxChannel, RECV_RX_KEY, RECV_TX_KEY, STARTING_SEED,
};
use ucsc_ectf_util_no_std::{hib::HibController, timer::Timer, Runtime};

#[entry]
fn main_test() -> ! {
    let peripherals = (
        CorePeripherals::take().unwrap(),
        Peripherals::take().unwrap(),
    );
    let mut rt_peripherals = peripherals.into();
    let mut rt = Runtime::new(
        &mut rt_peripherals,
        &RECV_RX_KEY.into(),
        &RECV_TX_KEY.into(),
    );

    run_send_tests(&mut rt.uart1_controller, |d| {
        rt.hib_controller.create_timer(d)
    });

    send_small_crypto_test(&mut rt.uart1_controller, &rt.hib_controller);
    send_medium_crypto_test(&mut rt.uart1_controller, &rt.hib_controller);
    send_nonce_uniqueness_test(&mut rt.uart1_controller, &rt.hib_controller);

    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Finished sending!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Sends a short encrypted message.
fn send_small_crypto_test<T: RxTxChannel>(uart: &mut T, hib: &HibController) {
    const MSG_LEN: usize = 10;

    // We delay for 50ms to give time for other board to process.
    let mut timer = hib.create_timer(Duration::from_millis(50));

    while !timer.poll() {}

    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    let mut msg = [0; MSG_LEN];

    rng.set_stream(10000);
    rng.fill_bytes(&mut msg);

    uart.send(&mut msg).unwrap();
}

/// Sends a medium encrypted message.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
fn send_medium_crypto_test<T: RxTxChannel>(uart: &mut T, hib: &HibController) {
    const MSG_LEN: usize = 1000;

    // We delay for 50ms to give time for other board to process.
    let mut timer = hib.create_timer(Duration::from_millis(50));

    while !timer.poll() {}

    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    let mut msg = [0; MSG_LEN];

    rng.set_stream(20000);
    rng.fill_bytes(&mut msg);

    uart.send(&mut msg).unwrap();
}

/// Sends a large amount of messages to allow the receiver to test that all nonces are unique.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
fn send_nonce_uniqueness_test<T: RxTxChannel>(uart: &mut T, hib: &HibController) {
    const NONCE_CT: usize = 1000;

    // We add a small delay to give the other board time to start receiving.
    let mut timer = hib.create_timer(Duration::from_millis(50));

    while !timer.poll() {}

    for _ in 0..NONCE_CT {
        uart.send(&mut [0]).unwrap();
        let mut timer = hib.create_timer(Duration::from_millis(1));

        while !timer.poll() {}
    }

    // We add a small delay to give the other board time to check for duplicates.
    let mut timer = hib.create_timer(Duration::from_millis(5));

    while !timer.poll() {}
}
