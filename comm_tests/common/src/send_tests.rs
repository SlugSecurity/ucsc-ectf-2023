use core::time::Duration;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use ucsc_ectf_util_common::timer::Timer;

use crate::{RxTxChannel, STARTING_SEED, METADATA_OVERHEAD, MAX_MESSAGE_SIZE};

/// Tests that trying to send data with length 0 returns an error.
pub fn empty_send_error_test<T: RxTxChannel>(uart: &mut T) {
    uart.send(&mut [])
        .expect_err("Failed empty send error test");
}

/// Sends a short message containing text data.
pub fn send_basic_text_test<T: RxTxChannel>(uart: &mut T) {
    const TO_SEND: &[u8] = b"a";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Sends a medium message containing text data.
pub fn send_medium_text_test<T: RxTxChannel>(uart: &mut T) {
    const TO_SEND: &[u8] = b"This is a medium sized test. Idk what else to send here.";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Sends a short message containing binary data.
pub fn send_basic_binary_test<T: RxTxChannel>(uart: &mut T) {
    const TO_SEND: &[u8] = b"\0";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Sends a medium message containing binary data.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_medium_binary_test<T: RxTxChannel>(uart: &mut T) {
    const TO_SEND: &[u8] = &[
        14, 229, 39, 7, 0, 147, 65, 14, 47, 235, 87, 198, 187, 226, 71, 175, 243, 244, 152, 111,
        138, 125, 85, 30, 60, 150, 173, 71, 72, 231, 194, 0, 3, 246, 135, 22, 137, 185, 149, 31,
        79, 107, 220, 80, 145, 65, 248, 92, 102, 75,
    ];
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Repeatedly sends large messages containing binary data.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_repeated_large_binary_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    // Sync with the receiver.
    let mut sync = [0; 1 + METADATA_OVERHEAD];
    let mut sync_timer = timer_fn(Duration::from_secs(10));
    let _ = uart.recv_with_data_timeout(&mut sync, &mut sync_timer);

    // Give the receiver time to start receiving.
    let mut wait_timer = timer_fn(Duration::from_millis(1));
    while !wait_timer.poll() {}

    const ITERATION_COUNT: usize = 30;
    const MSG_LEN: usize = MAX_MESSAGE_SIZE;

    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    let mut msg = [0; MSG_LEN];

    for _ in 0..ITERATION_COUNT {
        // Delay to allow for the previous message to be processed.
        let mut timer = timer_fn(Duration::from_millis(500));

        while !timer.poll() {}

        rng.fill_bytes(&mut msg);
        uart.send(&mut msg).unwrap();

        // Set stream number to keep each message unique
        rng.set_stream(rng.get_stream() + 1);
    }
}

/// Sends data after waiting less than 1 second so the receiver can test that no timeout occurs.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_should_not_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    let mut timer = timer_fn(Duration::from_millis(900));

    while !timer.poll() {}

    const TO_SEND: &[u8] = b"a";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Sends data after waiting more than 1 second so the receiver can test that a timeout occurs.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_should_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    let mut timer = timer_fn(Duration::from_millis(1100));

    while !timer.poll() {}

    const TO_SEND: &[u8] = b"a";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();

    let mut flush_timer = timer_fn(Duration::from_millis(100)); // Wait for other side to flush receive.

    while !flush_timer.poll() {}

    const TO_SEND_RESYNC: &[u8] = b"a";
    let mut resync = [0; TO_SEND_RESYNC.len()];
    resync.copy_from_slice(TO_SEND_RESYNC);
    uart.send(&mut resync).unwrap();

    let mut process_timer = timer_fn(Duration::from_millis(1)); // Wait for other side to process.

    while !process_timer.poll() {}
}

/// Sends a large amount of data so the receiver can test that no data timeout occurs.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_should_not_data_timeout_test<T: RxTxChannel>(uart: &mut T) {
    const TO_SEND: &[u8] = &[0; 1000];
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();
}

/// Sends data after waiting more than 1 second so the receiver can test that a data timeout occurs.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn send_should_data_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    // Sync with the receiver.
    let mut sync = [0; 1 + METADATA_OVERHEAD];
    let mut sync_timer = timer_fn(Duration::from_secs(10));
    let _ = uart.recv_with_data_timeout(&mut sync, &mut sync_timer);

    let mut timer = timer_fn(Duration::from_millis(1100));

    while !timer.poll() {}

    const TO_SEND: &[u8] = b"a";
    let mut test = [0; TO_SEND.len()];
    test.copy_from_slice(TO_SEND);
    uart.send(&mut test).unwrap();

    let mut flush_timer = timer_fn(Duration::from_millis(100)); // Wait for other side to flush receive.

    while !flush_timer.poll() {}

    const TO_SEND_RESYNC: &[u8] = b"a";
    let mut resync = [0; TO_SEND_RESYNC.len()];
    resync.copy_from_slice(TO_SEND_RESYNC);
    uart.send(&mut resync).unwrap();

    let mut process_timer = timer_fn(Duration::from_millis(1)); // Wait for other side to process.

    while !process_timer.poll() {}
}
