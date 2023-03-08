use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, Payload},
    AeadInPlace, XChaCha20Poly1305,
};
use clap::Parser;
use hex::decode;
use std::{fs::File, os::unix::prelude::FileExt};
use typenum::Unsigned;
use ucsc_ectf_eeprom_layout::{EepromReadField, EepromReadWriteField, SECRET_SIZE};

type TagSize = <XChaCha20Poly1305 as AeadCore>::TagSize;
type NonceSize = <XChaCha20Poly1305 as AeadCore>::NonceSize;
const TAG_SIZE: usize = <TagSize as Unsigned>::USIZE;
const NONCE_SIZE: usize = <NonceSize as Unsigned>::USIZE;
const METADATA_SIZE: usize = TAG_SIZE + NONCE_SIZE;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    eeprom_file: String,

    #[arg(long)]
    message: String,
}

fn main() {
    let args = Args::parse();

    let eeprom_file = File::open(args.eeprom_file).unwrap();

    let mut message = decode(args.message).unwrap();

    let mut car_key_bytes = [0u8; SECRET_SIZE];
    eeprom_file
        .read_exact_at(
            &mut car_key_bytes,
            EepromReadWriteField::CarEncryptionKey
                .get_field_bounds()
                .address as u64,
        )
        .unwrap();
    let car_cipher = XChaCha20Poly1305::new((&car_key_bytes).into());

    let mut fob_key_bytes = [0u8; SECRET_SIZE];
    eeprom_file
        .read_exact_at(
            &mut fob_key_bytes,
            EepromReadWriteField::KeyFobEncryptionKey
                .get_field_bounds()
                .address as u64,
        )
        .unwrap();
    let fob_cipher = XChaCha20Poly1305::new((&fob_key_bytes).into());

    // Split message from metadata.
    let len = message.len();
    let (msg_body, metadata) = message.split_at_mut(len - METADATA_SIZE);

    // Take nonce and tag
    let (&mut ref nonce, &mut ref tag) = metadata.split_at_mut(NONCE_SIZE);

    let mut car = Vec::new();
    car.extend_from_slice(msg_body);
    let mut fob = Vec::new();
    fob.extend_from_slice(msg_body);

    if car_cipher
        .decrypt_in_place_detached(nonce.into(), b"", &mut car, tag.into())
        .is_ok()
    {
        print!("{car:#x?}");
        return;
    }
    if fob_cipher
        .decrypt_in_place_detached(nonce.into(), b"", &mut fob, tag.into())
        .is_ok()
    {
        print!("{fob:#x?}");
        return;
    }
    println!("BAD");
}
