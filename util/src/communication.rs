pub mod layers;
mod uart;

pub use uart::*;

pub trait RxChannel {
    type RxError;

    fn read(&mut self, dest: &mut [u8]) -> Result<(), Self::RxError>;
}

pub trait TxChannel {
    type TxError;

    fn write(&mut self, src: &[u8]) -> Result<(), Self::TxError>;
}
