//! This module encompasses the framing layer of the BogoStack, which provides framing protocols
//! to use in [`RxChannels`](crate::communication::RxChannel) and [`FramedTxChannels`](FramedTxChannel).
//! Any framing implementation must have channels implementing the aforementioned traits.
//! [`FramedTxChannels`](FramedTxChannel) differ from [`TxChannels`](TxChannel) in that they require
//! framing while [`TxChannels`](TxChannel) do not necessarily require any concept of framing.
//!
//! # Current framing protocol implementations:
//! ## BogoFraming
//! Each message sent/received will be hex encoded and decoded, delimited by a NULL (\0) character
//! at the start and at the end. Messages must be at least 1 character long.  This framing protocol
//! implementation is used in  [`FramedUartRxChannels`](FramedUartRxChannel) and
//! [`FramedUartTxChannels`](FramedUartTxChannel).
//!
//! See the documentation for [`communication`](crate::communication) for a description of the BogoStack
//! and more info on the other layers of the BogoStack.

mod uart;

use chacha20poly1305::aead::heapless;
pub use uart::*;

use crate::communication::{CommunicationError, TxChannel};

/// A trait to be implemented by all transmission channels in framing protocol implementations.
/// This contains one function to specify the slices that go into the frame to be transmitted.
pub trait FramedTxChannel: TxChannel {
    /// Transmits a frame through the [`TxChannel`] given a closure returning a [`Frame`] or
    /// a [`CommunicationError`]. The const generic, FRAME_CT, must be the number of
    /// slices in the created frame.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::SendError`] - Occurs when there's no more space
    /// in the frame for the number of slices provided or some error occurs when
    /// sending the frame through the [`TxChannel`].
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError>;
}

impl<T: FramedTxChannel> TxChannel for T {
    fn send(&mut self, src: &mut [u8]) -> Result<(), CommunicationError> {
        self.frame::<1>(|| Frame::new().append(src))
    }
}

/// A struct that keeps track of slices of u8's to write as one frame
/// in a [`FramedTxChannel`]. This can be used to write discontiguous
/// pieces of memory into one frame. The const generic ``FRAME_SLICES``
/// indicates the number of slices in the [`Frame`].
#[derive(Default)]
pub struct Frame<'a, const FRAME_SLICES: usize> {
    frame_components: heapless::Vec<&'a [u8], FRAME_SLICES>,
    total_len: usize,
}

impl<'a, const FRAME_CT: usize> Frame<'a, FRAME_CT> {
    /// Instantiates a new [`Frame`]. See the struct documentation for
    /// more information.
    pub fn new() -> Self {
        Frame {
            frame_components: heapless::Vec::new(),
            total_len: 0,
        }
    }

    /// Adds a slice to the frame.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::InternalError`] - Occurs when there's no more space
    /// in the frame for another slice.
    pub fn append(mut self, buff: &'a [u8]) -> Result<Self, CommunicationError> {
        match self.frame_components.push(buff) {
            Ok(_) => {
                self.total_len += buff.len();

                Ok(self)
            }
            Err(_) => Err(CommunicationError::InternalError),
        }
    }
}
