use sha3::Sha3_256;

use super::EntropySource;

pub struct Secret<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Secret<T> {
    fn init() -> Self {
        todo!("Finish host secret entropy source implementation.");

        Secret { next: T::init() }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        todo!("Finish host secret entropy source implementation.");

        self.next.add_to_hasher(hasher);
    }
}
