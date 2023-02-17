use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use k256::SecretKey;
use postcard::to_allocvec;
use ucsc_ectf_eeprom_layout::{
    EepromReadField, EepromReadOnlyField, EepromReadWriteField, SECRET_SIZE,
};

fn eeprom_field_from_path<P, F>(eeprom_file: &File, field: F, path: P)
where
    P: AsRef<Path>,
    F: EepromReadField,
{
    let mut f = File::open(path).unwrap();
    let bounds = EepromReadField::get_field_bounds(&field);
    let offset = bounds.address as u64;
    let mut buf = vec![0u8; bounds.size];
    f.read_exact(&mut buf).unwrap();
    eeprom_file.write_all_at(&buf, offset).unwrap();
}

fn eeprom_field_from_buf<F>(eeprom_file: &File, field: F, buf: &[u8])
where
    F: EepromReadField,
{
    let bounds = EepromReadField::get_field_bounds(&field);
    let offset = bounds.address as u64;
    eeprom_file.write_all_at(buf, offset).unwrap();
}

fn main() {
    // Get the out directory.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Add out directory to the linker search path.
    println!("cargo:rustc-link-search={}", out.display());

    // Put the memory.x linker script somewhere the linker can find it.
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    if let Some(secrets_dir) = option_env!("SECRETS_DIR") {
        let mut feature_signing_key_file =
            File::open(format!("{secrets_dir}/FEATURE_SIGNING_KEY")).unwrap();
        let mut feature_verifying_key_file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(false)
            .append(false)
            .open(format!("{secrets_dir}/FEATURE_VERIFYING_KEY"))
            .unwrap();
        let eeprom_file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(false)
            .append(false)
            .open(option_env!("EEPROM_PATH").unwrap())
            .unwrap();

        let mut feature_signing_key_bytes = [0u8; SECRET_SIZE];
        feature_signing_key_file
            .read_exact(&mut feature_signing_key_bytes)
            .unwrap();
        let feature_signing_key = SecretKey::from_be_bytes(&feature_signing_key_bytes).unwrap();
        let feature_verifying_key = feature_signing_key.public_key();
        feature_verifying_key_file
            .write_all(&to_allocvec(&feature_verifying_key).unwrap())
            .unwrap();

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadOnlyField::FeatureVerifyingKey,
            format!("{secrets_dir}/FEATURE_VERIFYING_KEY"),
        );

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadOnlyField::SecretSeed,
            format!("{secrets_dir}/SECRET_SEED"),
        );

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadWriteField::UnlockKeyOne,
            format!("{secrets_dir}/UNLOCK_KEY_ONE"),
        );

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadWriteField::UnlockKeyTwo,
            format!("{secrets_dir}/UNLOCK_KEY_TWO"),
        );

        let car_id = option_env!("CAR_ID").unwrap();
        let buf: u32 = car_id.parse::<u32>().unwrap();
        eeprom_field_from_buf(
            &eeprom_file,
            EepromReadWriteField::CarId,
            &buf.to_be_bytes(),
        );

        println!("cargo:rerun-if-changed={secrets_dir}");
    }

    // Only re-run the build script when this file or memory.x is changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
