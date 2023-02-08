use sha3::Sha3_256;

use super::EntropySource;

/// This entropy source gathers entropy from drift between clocks on the board.
pub struct ClockDrift<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for ClockDrift<T> {
    fn init() -> Self {
        todo!("Finish clock drift entropy source implementation.");

        ClockDrift { next: T::init() }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        todo!("Finish clock drift entropy source implementation.");

        self.next.add_to_hasher(hasher);
    }
}
