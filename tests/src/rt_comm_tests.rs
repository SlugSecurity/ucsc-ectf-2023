#![cfg(debug_assertions)]

use ucsc_ectf_util_no_std::{
    communication::{self, TxChannel},
    Uart0RxPin, Uart0TxPin, Uart1RxPin, Uart1TxPin,
};

type Uart0Controller<'a> = communication::Uart0Controller<'a, Uart0TxPin, Uart0RxPin>;
type Uart1Controller<'a> = communication::Uart1Controller<'a, Uart1TxPin, Uart1RxPin>;

pub fn run(uart0: &mut Uart0Controller, _uart1: &mut Uart1Controller) {
    basic_uart0_send_test(uart0);
}

/// This test requires manual intervention. Verify that the "Basic test!!!" was truly sent over UART 0.
/// It will be encrypted and authenticated using ChaCha20Poly1305 with the ciphertext followed by the nonce
/// followed by the tag. The encryption key will be purely 0's.
fn basic_uart0_send_test(uart0: &mut Uart0Controller) {
    let mut basic_test = [0; 13];
    basic_test.copy_from_slice(b"Basic test!!!");
    uart0.send(&mut basic_test).expect("Failed to send");
}
