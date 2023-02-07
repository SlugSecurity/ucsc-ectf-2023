use crate::communication::RxChannel;

pub struct Chacha20Poly1305RxChannel<T: RxChannel> {
    channel: T,
}

impl<T: RxChannel> Chacha20Poly1305RxChannel<T> {
    pub fn new(channel: T, tx_key: &Key, rx_key: &Key) -> Self {
        Self { channel }
    }
}

impl<T: RxChannel> RxChannel for Chacha20Poly1305RxChannel<T> {
    type RxError = chacha20poly1305::Error;

    fn read(&mut self, dest: &mut [u8]) -> Result<(), Self::RxError> {
        todo!()
    }
}
