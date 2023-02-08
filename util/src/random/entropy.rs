mod adc;
mod clock_drift;
mod secret;
mod uninit_memory;

pub use adc::Adc;
pub use clock_drift::ClockDrift;
pub use secret::Secret;
pub use uninit_memory::UninitMemory;

use sha3::{Digest, Sha3_256};

const ENTROPY_HASH_SIZE: usize = 32; // 256 bits (32 bytes).

pub trait EntropySource {
    fn init() -> Self;
    fn add_to_hasher(&self, hasher: &mut Sha3_256);
}

impl EntropySource for () {
    fn init() {}
    fn add_to_hasher(&self, _hasher: &mut Sha3_256) {}
}

pub struct EntropyHasher<T: EntropySource> {
    entropy: T,
}

impl<T: EntropySource> EntropyHasher<T> {
    pub fn new() -> Self {
        EntropyHasher { entropy: T::init() }
    }

    pub fn hash(&self) -> [u8; ENTROPY_HASH_SIZE] {
        let mut hasher = Sha3_256::new();
        self.entropy.add_to_hasher(&mut hasher);

        hasher.finalize().into()
    }
}
