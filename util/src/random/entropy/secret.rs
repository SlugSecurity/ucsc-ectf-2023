use super::EntropySource;
use crate::RuntimePeripherals;
use sha3::Sha3_256;

/// This entropy source is a constant secret value.
pub(crate) struct Secret<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Secret<T> {
    fn init(peripherals: &mut RuntimePeripherals) -> Self {
        todo!("Finish host secret entropy source implementation.");

        Secret {
            next: T::init(peripherals),
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        todo!("Finish host secret entropy source implementation.");

        self.next.add_to_hasher(hasher);
    }
}
