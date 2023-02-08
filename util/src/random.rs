mod entropy;

use self::entropy::{Adc, ClockDrift, EntropyHasher, Secret, UninitMemory};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

pub struct RandomNumberGenerator(ChaCha20Rng);

impl RandomNumberGenerator {
    /// Initializes a CSPRNG.
    pub fn new() -> Self {
        RandomNumberGenerator(ChaCha20Rng::from_seed(
            EntropyHasher::<UninitMemory<Secret<Adc<ClockDrift<()>>>>>::new().hash(),
        ))
    }

    /// Fills the given buffer with random bytes.
    pub fn fill_with_random_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }
}
