#![no_std]

use core::time::Duration;

use ucsc_ectf_util_common::{
    communication::{RxChannel, TxChannel},
    timer::Timer,
};

mod recv_tests;
mod send_tests;

use recv_tests::*;
use send_tests::*;

pub const METADATA_OVERHEAD: usize = 40;
pub const STARTING_SEED: [u8; 32] = [
    196, 2, 4, 181, 226, 4, 184, 242, 23, 194, 111, 197, 111, 33, 186, 168, 243, 171, 47, 145, 148,
    6, 198, 144, 100, 77, 160, 249, 128, 209, 165, 72,
];
pub const RECV_TX_KEY: [u8; 32] = [
    216, 51, 162, 200, 251, 210, 183, 149, 46, 208, 228, 128, 199, 189, 148, 254, 83, 54, 148, 249,
    242, 216, 110, 61, 58, 163, 152, 155, 68, 145, 14, 232,
];
pub const RECV_RX_KEY: [u8; 32] = [
    45, 249, 236, 184, 243, 34, 227, 240, 89, 1, 15, 213, 123, 109, 16, 235, 244, 202, 22, 193,
    124, 140, 26, 111, 12, 222, 90, 234, 64, 178, 67, 17,
];
pub const SEND_TX_KEY: [u8; 32] = RECV_RX_KEY;
pub const SEND_RX_KEY: [u8; 32] = RECV_TX_KEY;
pub const MAX_MESSAGE_SIZE: usize = 1024;

pub trait RxTxChannel: RxChannel + TxChannel {}
impl<T: RxChannel + TxChannel> RxTxChannel for T {}

pub fn run_recv_tests<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(ch: &mut T, timer_fn: V) {
    empty_recv_error_test(ch, &timer_fn);
    too_small_recv_error_test(ch, &timer_fn);

    // Run non-error tests below. Do not add them above because this could
    // throw off timing.

    recv_basic_text_test(ch, &timer_fn);
    recv_medium_text_test(ch, &timer_fn);
    recv_basic_binary_test(ch, &timer_fn);
    recv_medium_binary_test(ch, &timer_fn);
    recv_repeated_large_binary_test(ch, &timer_fn);

    recv_should_not_timeout_test(ch, &timer_fn);
    recv_should_timeout_test(ch, &timer_fn);
    recv_should_not_data_timeout_test(ch, &timer_fn);
    recv_should_data_timeout_test(ch, &timer_fn);
}

pub fn run_send_tests<T: RxTxChannel, U: Timer, V: Fn(Duration) -> U>(ch: &mut T, timer_fn: V) {
    empty_send_error_test(ch);

    // Run non-error tests below. This technically isn't necessary
    // but we're putting error tests at the beginning for consistency.

    send_basic_text_test(ch);
    send_medium_text_test(ch);
    send_basic_binary_test(ch);
    send_medium_binary_test(ch);
    send_repeated_large_binary_test(ch, &timer_fn);

    send_should_not_timeout_test(ch, &timer_fn);
    send_should_timeout_test(ch, &timer_fn);
    send_should_not_data_timeout_test(ch);
    send_should_data_timeout_test(ch, &timer_fn);
}
