use core::time::Duration;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use ucsc_ectf_util_common::{communication::CommunicationError, timer::Timer};

use crate::{RxTxChannel, METADATA_OVERHEAD, STARTING_SEED, MAX_MESSAGE_SIZE};

/// Tests that trying to receive data into a buffer of length 0 immediately returns an error.
pub fn empty_recv_error_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    let mut test = [];
    let mut timer = timer_fn(Duration::from_secs(1));
    let res = uart.recv_with_data_timeout(&mut test, &mut timer);

    assert!(
        !timer.poll(),
        "Timeout occurred instead of an immediate error for empty recv."
    );

    assert_eq!(
        res,
        Err(CommunicationError::RecvError),
        "Failed empty recv error test"
    );
}

/// Tests that trying to receive data into a buffer too small to hold any ciphertext immediately
/// returns an error.
pub fn too_small_recv_error_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    // This buffer is too small because it doesn't have enough space to hold the metadata and at
    // least one byte.
    let mut too_small = [0; METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let res = uart.recv_with_data_timeout(&mut too_small, &mut timer);

    assert!(
        !timer.poll(),
        "Timeout occurred instead of an immediate error for too small recv."
    );

    assert_eq!(
        res,
        Err(CommunicationError::RecvError),
        "Failed too small recv error test"
    );
}

/// Tests that a short message containing text data can be received successfully.
pub fn recv_basic_text_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    const EXPECTED: &[u8] = b"a";
    let mut test = [0; EXPECTED.len() + METADATA_OVERHEAD];
    // Since this is the first test, set the timeout to 1000 seconds to give the user time to start
    // the sending firmware.
    let mut timer = timer_fn(Duration::from_secs(1000));
    let read = uart.recv_with_data_timeout(&mut test, &mut timer).unwrap();

    assert_eq!(&test[..read], EXPECTED);
}

/// Tests that a medium message containing text data can be received successfully.
pub fn recv_medium_text_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    const EXPECTED: &[u8] = b"This is a medium sized test. Idk what else to send here.";
    let mut test = [0; EXPECTED.len() + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let read = uart.recv_with_data_timeout(&mut test, &mut timer).unwrap();

    assert_eq!(&test[..read], EXPECTED);
}

/// Tests that a short message containing binary data can be received successfully.
pub fn recv_basic_binary_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    const EXPECTED: &[u8] = b"\0";
    let mut test = [0; EXPECTED.len() + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let read = uart.recv_with_data_timeout(&mut test, &mut timer).unwrap();

    assert_eq!(&test[..read], EXPECTED);
}

/// Tests that a medium message containing binary data can be received successfully.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_medium_binary_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    const EXPECTED: &[u8] = &[
        14, 229, 39, 7, 0, 147, 65, 14, 47, 235, 87, 198, 187, 226, 71, 175, 243, 244, 152, 111,
        138, 125, 85, 30, 60, 150, 173, 71, 72, 231, 194, 0, 3, 246, 135, 22, 137, 185, 149, 31,
        79, 107, 220, 80, 145, 65, 248, 92, 102, 75,
    ];
    let mut test = [0; EXPECTED.len() + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let read = uart.recv_with_data_timeout(&mut test, &mut timer).unwrap();

    assert_eq!(&test[..read], EXPECTED);
}

/// Tests that large messages containing binary data can be repeatedly received successfully.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_repeated_large_binary_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    // Sync with the sender.
    const TO_SEND_SYNC: &[u8] = b"a";
    let mut sync = [0; TO_SEND_SYNC.len()];
    sync.copy_from_slice(TO_SEND_SYNC);
    uart.send(&mut sync).unwrap();

    const ITERATION_COUNT: usize = 30;
    const MSG_LEN: usize = MAX_MESSAGE_SIZE;

    let mut rng = ChaCha20Rng::from_seed(STARTING_SEED);
    let mut msg = [0; MSG_LEN + METADATA_OVERHEAD];

    for _ in 0..ITERATION_COUNT {
        let mut expected = [0; MSG_LEN];
        let mut timer = timer_fn(Duration::from_secs(2));
        let read = uart.recv_with_data_timeout(&mut msg, &mut timer).unwrap();

        rng.fill_bytes(&mut expected);

        assert_eq!(&msg[..read], expected);

        // Set stream number to stay in sync with the other side.
        rng.set_stream(rng.get_stream() + 1);
    }
}

/// Tests that receiving data with a timeout of 1 second doesn't time out when the sender waits for
/// less than 1 second before sending data.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_should_not_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    const EXPECTED: &[u8] = b"a";
    let mut test = [0; EXPECTED.len() + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let read = uart.recv_with_timeout(&mut test, &mut timer).unwrap();

    assert_eq!(&test[..read], EXPECTED);
}

/// Tests that receiving data with a timeout of 1 second times out when the sender waits for more
/// than 1 second before sending data.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_should_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    let mut test = [0; 1 + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let res = uart.recv_with_timeout(&mut test, &mut timer);

    res.expect_err("Failed recv timeout test");

    // Flush last message.
    let mut flush_timer = timer_fn(Duration::from_millis(50));

    while !flush_timer.poll() {} // Wait for message to come through. Message will be cut off, so don't verify it.
    flush_timer.reset();

    let _ = uart.recv_with_data_timeout(&mut test, &mut flush_timer);

    // Resync time.
    let mut resync = [0; 1 + METADATA_OVERHEAD];
    let mut resync_timer = timer_fn(Duration::from_secs(1));
    let _ = uart.recv_with_data_timeout(&mut resync, &mut resync_timer);
}

/// Tests that receiving data with a data timeout of 5 milliseconds resets the timer after receiving
/// each byte and doesn't time out.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_should_not_data_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    let mut test = [0; 1000 + METADATA_OVERHEAD];
    // 5 ms is enough to get some of the message, but not enough to get the entire message.
    let mut timer = timer_fn(Duration::from_millis(5));
    let read = uart.recv_with_data_timeout(&mut test, &mut timer).unwrap();

    assert!(&test[..read].iter().all(|&x| x == 0));
}

/// Tests that receiving data with a data timeout of 1 second times out when the sender waits for
/// more than 1 second before sending data.
///
/// This function is never inlined in order to avoid using too much stack space.
#[inline(never)]
pub fn recv_should_data_timeout_test<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(
    uart: &mut T,
    timer_fn: &V,
) {
    // Sync with the sender.
    const TO_SEND_SYNC: &[u8] = b"a";
    let mut sync = [0; TO_SEND_SYNC.len()];
    sync.copy_from_slice(TO_SEND_SYNC);
    uart.send(&mut sync).unwrap();

    let mut test = [0; 1 + METADATA_OVERHEAD];
    let mut timer = timer_fn(Duration::from_secs(1));
    let res = uart.recv_with_data_timeout(&mut test, &mut timer);

    res.expect_err("Failed recv data timeout test");

    // Flush last message.
    let mut flush_timer = timer_fn(Duration::from_millis(100));

    while !flush_timer.poll() {} // Wait for message to come through. Message will be cut off, so don't verify it.
    flush_timer.reset();

    let _ = uart.recv_with_data_timeout(&mut test, &mut flush_timer);

    // Resync time.
    let mut resync = [0; 1 + METADATA_OVERHEAD];
    let mut resync_timer = timer_fn(Duration::from_secs(1));
    let _ = uart.recv_with_data_timeout(&mut resync, &mut resync_timer);
}
