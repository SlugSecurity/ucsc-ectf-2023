#![cfg(debug_assertions)]

use core::iter;
use ucsc_ectf_util_no_std::eeprom::{
    EepromController, EepromReadField, EepromReadOnlyField, EepromReadWriteField, PUBLIC_KEY_SIZE,
};

const READ_ONLY_FIELDS: [EepromReadOnlyField; 9] = [
    EepromReadOnlyField::PairingPrivateKey,
    EepromReadOnlyField::PairingPublicKeySignature,
    EepromReadOnlyField::PairingVerifyingKey,
    EepromReadOnlyField::FeatureVerifyingKey,
    EepromReadOnlyField::SecretSeed,
    EepromReadOnlyField::FeatureThreeMessage,
    EepromReadOnlyField::FeatureTwoMessage,
    EepromReadOnlyField::FeatureOneMessage,
    EepromReadOnlyField::UnlockMessage,
];

const READ_WRITE_FIELDS: [EepromReadWriteField; 9] = [
    EepromReadWriteField::KeyFobEncryptionKey,
    EepromReadWriteField::CarEncryptionKey,
    EepromReadWriteField::CarId,
    EepromReadWriteField::PairingByte,
    EepromReadWriteField::PairingPin,
    EepromReadWriteField::PairingLongerCooldownByte,
    EepromReadWriteField::FeatureOneSignedPackaged,
    EepromReadWriteField::FeatureTwoSignedPackaged,
    EepromReadWriteField::FeatureThreeSignedPackaged,
];

const DEFAULT_EEPROM_DATA: u8 = 0xFF; // All 1s.

pub fn run(eeprom: &mut EepromController) {
    // Erase EEPROM before running tests.
    eeprom.erase_mem();

    // Run tests.
    read_default(eeprom);
    basic_write_read_test(eeprom);
    write_read_bleed_test(eeprom);
}

/// Tests reads of default EEPROM values (0xFF for all bytes).
fn read_default(eeprom: &mut EepromController) {
    let mut data = [0; PUBLIC_KEY_SIZE];

    for field in READ_ONLY_FIELDS.into_iter() {
        eeprom.read_slice(field, &mut data).unwrap();

        assert!(&data[..field.get_field_bounds().size]
            .iter()
            .all(|&n| n == DEFAULT_EEPROM_DATA));

        data.fill(0);
    }

    for field in READ_WRITE_FIELDS.into_iter() {
        eeprom.read_slice(field, &mut data).unwrap();

        assert!(&data[..field.get_field_bounds().size]
            .iter()
            .all(|&n| n == DEFAULT_EEPROM_DATA));

        data.fill(0);
    }
}

/// Tests writing and reading of EEPROM for read-write fields.
fn basic_write_read_test(eeprom: &mut EepromController) {
    const TEST_DATA_1: u8 = 0x55; // Alternate 0 and 1.
    const TEST_DATA_2: u8 = 0xAA; // Alternate 1 and 0.
    let mut data = [0; PUBLIC_KEY_SIZE];
    let mut read_data = [0; PUBLIC_KEY_SIZE];
    let mut test_data_iter = iter::once(TEST_DATA_1)
        .chain(iter::once(TEST_DATA_2))
        .cycle();

    for field in READ_WRITE_FIELDS.into_iter() {
        data.fill(test_data_iter.next().unwrap());
        eeprom
            .write_slice(field, &data[..field.get_field_bounds().size])
            .unwrap();
        read_data.fill(0); // Reset read data prior to reading into buffer.
        eeprom.read_slice(field, &mut read_data).unwrap();

        assert!(
            read_data[..field.get_field_bounds().size] == data[..field.get_field_bounds().size]
        );
    }
}

/// Tests that writing to one field does not affect another field.
fn write_read_bleed_test(eeprom: &mut EepromController) {
    const TEST_DATA_1: u8 = 0x55; // Alternate 0 and 1.
    const TEST_DATA_2: u8 = 0xAA; // Alternate 1 and 0.
    let mut data = [0; PUBLIC_KEY_SIZE];

    // Set all writable fields to the default values before starting test.
    for field in READ_WRITE_FIELDS.into_iter() {
        data.fill(DEFAULT_EEPROM_DATA);
        eeprom
            .write_slice(field, &data[..field.get_field_bounds().size])
            .unwrap();
    }

    // Test that writing to one field does not affect another field.
    let mut read_data = [0; PUBLIC_KEY_SIZE];
    let mut test_data_iter = iter::once(TEST_DATA_1)
        .chain(iter::once(TEST_DATA_2))
        .cycle();

    for fields in READ_WRITE_FIELDS.windows(2) {
        data.fill(test_data_iter.next().unwrap());
        eeprom
            .write_slice(fields[0], &data[..fields[0].get_field_bounds().size])
            .unwrap();
        read_data.fill(0); // Reset read data prior to reading into buffer.
        eeprom.read_slice(fields[1], &mut read_data).unwrap();

        assert!(&read_data[..fields[1].get_field_bounds().size]
            .iter()
            .all(|&n| n == DEFAULT_EEPROM_DATA));
    }
}
