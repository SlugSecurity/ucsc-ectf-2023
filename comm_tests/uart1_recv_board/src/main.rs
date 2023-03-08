#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate tm4c123x_hal;

use chacha20poly1305::{AeadInPlace, KeyInit, XChaCha20Poly1305};
use core::{fmt::Write, time::Duration};
use cortex_m::prelude::_embedded_hal_serial_Read;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use tm4c123x_hal::{serial::Rx, tm4c123x::UART1, CorePeripherals, Peripherals};
use ucsc_ectf_comm_tests_common::{
    run_recv_tests, METADATA_OVERHEAD, SEND_RX_KEY, SEND_TX_KEY, STARTING_SEED,
};
use ucsc_ectf_util_no_std::{
    communication::{lower_layers::framing::bogoframing, CommunicationError},
    timer::HibTimer,
    Arc, HibPool, Runtime, Uart1RxPin,
};

type Uart1Rx = Rx<UART1, Uart1RxPin, ()>;

#[entry]
fn main_test() -> ! {
    let peripherals = (
        CorePeripherals::take().unwrap(),
        Peripherals::take().unwrap(),
    );
    let mut rt_peripherals = peripherals.into();

    {
        let mut rt = Runtime::new(
            &mut rt_peripherals,
            &SEND_RX_KEY.into(),
            &SEND_TX_KEY.into(),
        );

        run_recv_tests(&mut rt.uart1_controller, |d| {
            rt.hib_controller.create_timer(d)
        });
    }

    recv_small_crypto_test(&mut rt_peripherals.uart1_rx, &rt_peripherals.hib);
    recv_medium_crypto_test(&mut rt_peripherals.uart1_rx, &rt_peripherals.hib);
    recv_nonce_uniqueness_test(&mut rt_peripherals.uart1_rx, &rt_peripherals.hib);

    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Finished receiving!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Tests that a short encrypted message can be received successfully.
fn recv_small_crypto_test(uart: &mut Uart1Rx, hib: &Arc<HibPool>) {
    const MSG_LEN: usize = 10;

    let mut msg = [0; MSG_LEN + METADATA_OVERHEAD];

    let read = bogoframing::recv_frame_with_data_timeout(
        uart,
        &mut msg,
        &mut HibTimer::new(hib, Duration::from_secs(1)),
        |u| u.read().map_err(|_| CommunicationError::RecvError),
        1,
    )
    .unwrap();
    let msg = &mut msg[..read];
    let decryptor = XChaCha20Poly1305::new(&SEND_RX_KEY.into());
    let mut expected = [0; MSG_LEN];
    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    rng.set_stream(10000);
    rng.fill_bytes(&mut expected);

    // Structure is ciphertext + 24 byte nonce + 16 byte tag
    let (data, rest) = msg.split_at_mut(MSG_LEN);
    let (nonce, tag) = rest.split_at(24);

    decryptor
        .decrypt_in_place_detached(nonce.into(), b"", data, tag.into())
        .expect("Couldn't decrypt with provided nonce, tag, and ciphertext.");

    assert_eq!(data, expected);
}

/// Tests that a medium encrypted message can be received successfully.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
fn recv_medium_crypto_test(uart: &mut Uart1Rx, hib: &Arc<HibPool>) {
    const MSG_LEN: usize = 1000;

    let mut msg = [0; MSG_LEN + METADATA_OVERHEAD];
    let read = bogoframing::recv_frame_with_data_timeout(
        uart,
        &mut msg,
        &mut HibTimer::new(hib, Duration::from_secs(1)),
        |u| u.read().map_err(|_| CommunicationError::RecvError),
        1,
    )
    .unwrap();
    let msg = &mut msg[..read];
    let decryptor = XChaCha20Poly1305::new(&SEND_RX_KEY.into());
    let mut expected = [0; MSG_LEN];
    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    rng.set_stream(20000);
    rng.fill_bytes(&mut expected);

    // Structure is ciphertext + 24 byte nonce + 16 byte tag
    let (data, rest) = msg.split_at_mut(MSG_LEN);
    let (nonce, tag) = rest.split_at(24);

    decryptor
        .decrypt_in_place_detached(nonce.into(), b"", data, tag.into())
        .expect("Couldn't decrypt with provided nonce, tag, and ciphertext.");

    assert_eq!(data, expected);
}

/// Tests that all nonces the sender uses are unique.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
fn recv_nonce_uniqueness_test(uart: &mut Uart1Rx, hib: &Arc<HibPool>) {
    const NONCE_CT: usize = 1000;
    let mut nonces = [[0; 24]; NONCE_CT];
    let mut msg = [0; METADATA_OVERHEAD + 1];

    for nonce in nonces.iter_mut() {
        let read = bogoframing::recv_frame_with_data_timeout(
            uart,
            &mut msg,
            &mut HibTimer::new(hib, Duration::from_secs(1)),
            |u| u.read().map_err(|_| CommunicationError::RecvError),
            1,
        )
        .unwrap();
        let msg = &mut msg[..read];

        // Extract nonce from 1 byte encrypted message.
        nonce.copy_from_slice(&msg[1..25]);
    }

    nonces.sort_unstable();

    let duplicate_nonces = nonces.windows(2).find(|n| n[0] == n[1]);

    if let Some(n) = duplicate_nonces {
        panic!("Duplicate nonce found: {:?}", n[0]);
    }
}
