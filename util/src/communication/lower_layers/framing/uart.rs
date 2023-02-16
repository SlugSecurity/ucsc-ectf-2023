use core::{ops::Deref, time::Duration};

use cortex_m::prelude::_embedded_hal_serial_Read;
use tm4c123x_hal::{
    serial::{Rx, RxPin, Tx, TxPin},
    tm4c123x::{uart0, UART0, UART1},
};

use crate::communication::{self, CommunicationError, RxChannel};

use super::{Frame, FramedTxChannel};

const UART_FIFO_LEN: usize = 16;

/// The minimum size a framed UART message can be.
pub const MIN_FRAMED_UART_MESSAGE: usize = UART_FIFO_LEN;

/// A [`FramedTxChannel`] for transmitting UART data. This channel is unreliable and can lose bytes
/// in transmission. It is also insecure and should be wrapped around one of the channels in the
/// [`crypto`](crate::communication::lower_layers::crypto) layer for confidentiality and/or integrity.
/// A message sent by this channel must be at least [`MIN_FRAMED_UART_MESSAGE`] bytes long.
/// See the module-level documentation for details on how framing works for this channel.
pub struct FramedUartTxChannel<'a, UART, TX>
where
    UART: Deref<Target = uart0::RegisterBlock>,
    TX: TxPin<UART>,
{
    tx: &'a mut Tx<UART, TX, ()>,
}

impl<'a, TX> FramedUartTxChannel<'a, UART0, TX>
where
    TX: TxPin<UART0>,
{
    /// Creates a new [`FramedUartTxChannel`] for UART0 tranmission given the [`Tx`] end
    /// of a split [`Serial`](tm4c123x_hal::serial::Serial).
    pub fn new_uart0_tx_channel(tx: &'a mut Tx<UART0, TX, ()>) -> Self {
        Self { tx }
    }
}

impl<'a, TX> FramedUartTxChannel<'a, UART1, TX>
where
    TX: TxPin<UART1>,
{
    /// Creates a new [`FramedUartTxChannel`] for UART1 tranmission given the [`Tx`] end
    /// of a split [`Serial`](tm4c123x_hal::serial::Serial).
    pub fn new_uart1_tx_channel(tx: &'a mut Tx<UART1, TX, ()>) -> Self {
        Self { tx }
    }
}

/// An [`RxChannel`] for receiving UART data. This channel is unreliable and might not receive transmitted bytes.
/// It can also receive data that was never sent, receive merged frames, or split a message. It is also insecure
/// and should be wrapped around one of the channels in the [`crypto`](crate::communication::lower_layers::crypto)
/// layer for confidentiality and/or integrity. See the module-level documentation for details on how framing works
/// for this channel.
pub struct FramedUartRxChannel<'a, UART, RX>
where
    UART: Deref<Target = uart0::RegisterBlock>,
    RX: RxPin<UART>,
{
    rx: &'a mut Rx<UART, RX, ()>,
}

impl<'a, RX> FramedUartRxChannel<'a, UART0, RX>
where
    RX: RxPin<UART0>,
{
    /// Creates a new [`FramedUartRxChannel`] for UART0 tranmission given the [`Rx`] end
    /// of a split [`Serial`](tm4c123x_hal::serial::Serial).
    pub fn new_uart0_rx_channel(rx: &'a mut Rx<UART0, RX, ()>) -> Self {
        Self { rx }
    }
}

impl<'a, RX> FramedUartRxChannel<'a, UART1, RX>
where
    RX: RxPin<UART1>,
{
    /// Creates a new [`FramedUartRxChannel`] for UART1 tranmission given the [`Tx`] end
    /// of a split [`Serial`](tm4c123x_hal::serial::Serial).
    pub fn new_uart1_rx_channel(rx: &'a mut Rx<UART1, RX, ()>) -> Self {
        Self { rx }
    }
}

impl<'a, UART, TX> FramedUartTxChannel<'a, UART, TX>
where
    UART: Deref<Target = uart0::RegisterBlock>,
    TX: TxPin<UART>,
{
    fn frame_with<'b, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> communication::Result<Frame<'b, FRAME_CT>>,
        mut write_fn: impl FnMut(&mut Self, &[u8]),
    ) -> communication::Result<()> {
        const HEX_ARRAY_LEN: usize = 32;

        let frame = frame()?;

        if frame.total_len < MIN_FRAMED_UART_MESSAGE {
            return Err(CommunicationError::SendError);
        }

        let mut hex_array = [0; HEX_ARRAY_LEN];

        write_fn(self, b"\0");

        for frame_piece in frame.frame_components {
            for chunk in frame_piece.chunks(HEX_ARRAY_LEN / 2) {
                // This should never panic because the chunks should always fit in our hex array.
                hex::encode_to_slice(chunk, &mut hex_array).unwrap();

                write_fn(self, &hex_array[..chunk.len() * 2]);
            }
        }

        write_fn(self, b"\0");

        Ok(())
    }
}

impl<'a, UART, RX> FramedUartRxChannel<'a, UART, RX>
where
    UART: Deref<Target = uart0::RegisterBlock>,
    RX: RxPin<UART>,
{
    fn read_with<T>(
        &mut self,
        dest: &mut [u8],
        timeout: Duration,
        mut read_fn: impl FnMut(&mut Self) -> Result<u8, T>,
    ) -> communication::Result<usize> {
        /// Reads a hex nibble, returning ``Ok(None)`` if NULL character is encountered,
        /// or the hex digit as a number if successful, or an error upon receiving
        /// an invalid character or timeout
        fn read_hex_nibble<T, U>(
            read_fn_arg: &mut T,
            mut read_fn: impl FnMut(&mut T) -> Result<u8, U>,
            timeout: Duration,
        ) -> communication::Result<Option<u8>> {
            loop {
                match read_fn(read_fn_arg) {
                    Ok(b'\0') => return Ok(None),
                    Ok(read @ b'0'..=b'9') => return Ok(Some(read - b'0')),
                    Ok(read @ b'a'..=b'f') => return Ok(Some(read - b'a' + 10)),
                    Ok(_) => return Err(CommunicationError::RecvError),
                    Err(_) => (),
                }

                // TODO: check timeout
            }
        }

        if dest.len() < MIN_FRAMED_UART_MESSAGE {
            return Err(CommunicationError::RecvError);
        }

        // First, read and discard data until a NULL character is found because any data that's not NULL
        // is garbage, keeping the timeout in mind.
        while let Ok(1..) | Err(_) = read_fn(self) {
            // TODO: check timeout
        }

        // Once a NULL is found, keep reading until we find a non-NULL character to indicate the start of
        // a message, keeping the timeout in mind. We can do this because a frame must contain at least
        // 1 character.
        let mut first_nibble = loop {
            // If we find an non-hex and non-NULL character, we return an error.
            // If it's a NULL character, we continue.
            // If it's a hex character, we return the number it represents.
            if let Some(n) = read_hex_nibble(self, &mut read_fn, timeout)? {
                break Some(n);
            }
        };

        // Start reading bytes into dest until a NULL is found, the buffer is full before a NULL is reached,
        // a non-hex character is read, or the timeout occurs.
        for (idx, byte) in dest.iter_mut().enumerate() {
            let second_nibble = read_hex_nibble(self, &mut read_fn, timeout)?;

            let read = if let (Some(first), Some(second)) = (first_nibble, second_nibble) {
                (first << 4) | second
            } else {
                // We've received a NULL character in the second nibble, which means we have an odd
                // number of hex digits.
                return Err(CommunicationError::RecvError);
            };

            *byte = read;

            first_nibble = read_hex_nibble(self, &mut read_fn, timeout)?;

            // We've received a NULL character in the right place.
            if first_nibble.is_none() {
                let ct = idx + 1;

                if ct < MIN_FRAMED_UART_MESSAGE {
                    return Err(CommunicationError::RecvError);
                } else {
                    return Ok(ct);
                }
            }
        }

        // If we've reached this point, it means we've reached the end of the buffer and haven't read NULL
        Err(CommunicationError::RecvError)
    }
}

impl<'a, TX> FramedTxChannel for FramedUartTxChannel<'a, UART0, TX>
where
    TX: TxPin<UART0>,
{
    fn frame<'b, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> communication::Result<Frame<'b, FRAME_CT>>,
    ) -> communication::Result<()> {
        self.frame_with(frame, |ch, s| ch.tx.write_all(s))
    }
}

impl<'a, TX> FramedTxChannel for FramedUartTxChannel<'a, UART1, TX>
where
    TX: TxPin<UART1>,
{
    fn frame<'b, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> communication::Result<Frame<'b, FRAME_CT>>,
    ) -> communication::Result<()> {
        self.frame_with(frame, |ch, s| ch.tx.write_all(s))
    }
}

impl<'a, RX> RxChannel for FramedUartRxChannel<'a, UART0, RX>
where
    RX: RxPin<UART0>,
{
    fn recv(&mut self, dest: &mut [u8], timeout: Duration) -> communication::Result<usize> {
        self.read_with(dest, timeout, |ch| ch.rx.read())
    }
}

impl<'a, RX> RxChannel for FramedUartRxChannel<'a, UART1, RX>
where
    RX: RxPin<UART1>,
{
    fn recv(&mut self, dest: &mut [u8], timeout: Duration) -> communication::Result<usize> {
        self.read_with(dest, timeout, |ch| ch.rx.read())
    }
}
