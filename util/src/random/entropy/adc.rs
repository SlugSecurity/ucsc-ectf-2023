use super::EntropySource;
use sha3::Sha3_256;
use tm4c123x_hal::Peripherals;

/// This entropy source gathers entropy from the LSBs of the ADC inputs.
pub struct Adc<T: EntropySource> {
    next: T,
}

impl<T: EntropySource> EntropySource for Adc<T> {
    fn init(peripherals: &mut Peripherals) -> Self {
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
