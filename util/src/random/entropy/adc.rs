use sha3::Sha3_256;

use super::EntropySource;

pub struct Adc<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Adc<T> {
    fn init() -> Self {
        todo!("Finish ADC entropy source implementation.");

        Adc { next: T::init() }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        todo!("Finish ADC entropy source implementation.");

        self.next.add_to_hasher(hasher);
    }
}
