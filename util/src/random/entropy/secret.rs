use super::EntropySource;
use sha3::Sha3_256;
use tm4c123x_hal::Peripherals;

/// This entropy source is a constant secret value.
pub struct Secret<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Secret<T> {
    fn init(peripherals: &mut Peripherals) -> Self {
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
