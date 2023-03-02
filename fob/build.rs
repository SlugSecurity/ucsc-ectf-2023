use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use ecdsa::signature::Signer;
use hex::decode;
use k256::ecdsa::{Signature, SigningKey};
use k256::pkcs8::EncodePublicKey;
use k256::SecretKey;
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
        let mut pairing_signing_key_file =
            File::open(format!("{secrets_dir}/PAIRING_SIGNING_KEY")).unwrap();
        let mut pairing_verifying_key_file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(false)
            .append(false)
            .open(format!("{secrets_dir}/PAIRING_VERIFYING_KEY"))
            .unwrap();
        let mut pairing_private_key_file =
            File::open(format!("{secrets_dir}/PAIRING_PRIVATE_KEY")).unwrap();
        let mut pairing_public_key_signature_file =
            File::create(format!("{secrets_dir}/PAIRING_PUBLIC_KEY_SIGNATURE")).unwrap();
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

        let mut private_key_bytes = [0u8; 32];

        pairing_signing_key_file
            .read_exact(&mut private_key_bytes)
            .unwrap();
        let pairing_signing_key = SigningKey::from_bytes(&private_key_bytes).unwrap();
        let pairing_verifying_key = pairing_signing_key.verifying_key();
        pairing_verifying_key_file
            .write_all(
                pairing_verifying_key
                    .to_public_key_der()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();

        pairing_private_key_file
            .read_exact(&mut private_key_bytes)
            .unwrap();
        let pairing_private_key = SecretKey::from_be_bytes(&private_key_bytes).unwrap();
        let pairing_public_key = pairing_private_key.public_key();
        let pairing_public_key_der = pairing_public_key.to_public_key_der().unwrap();

        let pairing_public_key_signature: Signature =
            pairing_signing_key.sign(pairing_public_key_der.as_bytes());
        pairing_public_key_signature_file
            .write_all(&pairing_public_key_signature.to_bytes())
            .unwrap();

        let mut feature_signing_key_bytes = [0u8; SECRET_SIZE];
        feature_signing_key_file
            .read_exact(&mut feature_signing_key_bytes)
            .unwrap();
        let feature_signing_key = SigningKey::from_bytes(&feature_signing_key_bytes).unwrap();
        let feature_verifying_key = feature_signing_key.verifying_key();
        feature_verifying_key_file
            .write_all(
                feature_verifying_key
                    .to_public_key_der()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadOnlyField::PairingPrivateKey,
            format!("{secrets_dir}/PAIRING_PRIVATE_KEY"),
        );

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadOnlyField::PairingPublicKeySignature,
            format!("{secrets_dir}/PAIRING_PUBLIC_KEY_SIGNATURE"),
        );

        eeprom_field_from_path(
            &eeprom_file,
            EepromReadOnlyField::PairingVerifyingKey,
            format!("{secrets_dir}/PAIRING_VERIFYING_KEY"),
        );

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

        // Is paired key fob.
        if let (Some(car_id), Some(pairing_pin)) = (option_env!("CAR_ID"), option_env!("PAIR_PIN"))
        {
            let buf: u32 = car_id.parse().unwrap();
            eeprom_field_from_buf(
                &eeprom_file,
                EepromReadWriteField::CarId,
                &buf.to_be_bytes(),
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

            let buf = decode(pairing_pin).unwrap();
            eeprom_field_from_buf(&eeprom_file, EepromReadWriteField::PairingPin, &buf);
            eeprom_field_from_buf(&eeprom_file, EepromReadWriteField::PairingByte, &[1u8; 1]);
        }

        println!("cargo:rerun-if-changed={secrets_dir}");
    }

    // Only re-run the build script when this file or memory.x is changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
