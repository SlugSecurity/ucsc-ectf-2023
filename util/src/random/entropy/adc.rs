use super::EntropySource;
use crate::RuntimePeripherals;
use sha3::Sha3_256;

/// This entropy source gathers entropy from the LSBs of the ADC inputs.
pub(crate) struct Adc<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Adc<T> {
    fn init(peripherals: &mut RuntimePeripherals) -> Self {
        todo!("Randomize ADC pins and take ADC peripherals.");
        todo!("Finish ADC entropy source implementation.");

        Adc {
            next: T::init(peripherals),
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        todo!("Finish ADC entropy source implementation.");

        self.next.add_to_hasher(hasher);
    }
}
