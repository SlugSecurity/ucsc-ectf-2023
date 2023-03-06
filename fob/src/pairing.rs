use crate::MAX_MESSAGE_SIZE;
use core::time::Duration;
use ucsc_ectf_util_no_std::{
    communication::{CommunicationError, TxChannel},
    eeprom::{EepromController, EepromReadWriteField, BYTE_FIELD_SIZE, PAIRING_PIN_SIZE},
    hib::HibController,
    messages::{HostToolAck, Uart0Message},
    timer::{HibTimer, Timer},
    Runtime,
};
use zeroize::Zeroize;

mod diffie_hellman;
mod pairing_sequence;

fn send_ack(rt: &mut Runtime) {
    let mut buf = [0; MAX_MESSAGE_SIZE];
    let res = postcard::to_slice(
        &Uart0Message::PairingPinResponse(HostToolAck(true)),
        &mut buf,
    )
    .expect("Failed to serialize pairing response.");

    if let Err(CommunicationError::InternalError) = rt.uart0_controller.send(res) {
        panic!("Failed to send pairing response (internal error).");
    }
}

/// Processes pairing messages while unpaired.
pub(crate) fn unpaired_listen_and_pair(rt: &mut Runtime) {
    loop {
        // Perform Diffie-Hellman key exchange and set UART1 channel key.
        if !diffie_hellman::run_unpaired(rt) {
            continue;
        }

        // Pair self. Break if pairing is successful.
        if pairing_sequence::run_unpaired(rt) {
            // Send acknowledgement to host.
            send_ack(rt);

            break;
        }
    }
}

/// Gets the pairing longer cooldown byte and a cooldown timer for a pairing PIN attempt.
fn get_pin_cooldown_timer<'a>(
    eeprom_controller: &mut EepromController,
    hib_controller: &'a mut HibController,
) -> (u8, HibTimer<'a>) {
    // Check pairing longer cooldown byte.
    let mut pairing_longer_cooldown_byte = [0; BYTE_FIELD_SIZE];
    eeprom_controller
        .read_slice(
            EepromReadWriteField::PairingLongerCooldownByte,
            &mut pairing_longer_cooldown_byte,
        )
        .expect("EEPROM read failed: pairing longer cooldown byte.");

    // Create cooldown timer.
    let pin_cooldown_timer = match pairing_longer_cooldown_byte[0] {
        0 => hib_controller.create_timer(Duration::from_millis(800)),
        1 => hib_controller.create_timer(Duration::from_millis(4800)),
        _ => panic!("Invalid pairing longer cooldown byte."),
    };

    (pairing_longer_cooldown_byte[0], pin_cooldown_timer)
}

/// Checks a pairing PIN with a cooldown if the PIN is incorrect.
fn check_pin_attempt(rt: &mut Runtime, pairing_pin_attempt: u32) -> bool {
    const PAIRING_PIN_REAL_SIZE: usize = 3;

    // Get pairing PIN from EEPROM and check against attempt.
    let mut pairing_pin_bytes = [0; PAIRING_PIN_SIZE];
    rt.eeprom_controller
        .read_slice(EepromReadWriteField::PairingPin, &mut pairing_pin_bytes)
        .expect("EEPROM read failed: pairing PIN.");
    pairing_pin_bytes.rotate_right(PAIRING_PIN_SIZE - PAIRING_PIN_REAL_SIZE); // Account for build script's encoding.
    let mut pairing_pin = u32::from_be_bytes(pairing_pin_bytes);
    pairing_pin_bytes.zeroize();
    let pairing_pin_correct = pairing_pin_attempt == pairing_pin;
    pairing_pin.zeroize();

    pairing_pin_correct
}

fn set_pairing_longer_cooldown_byte(rt: &mut Runtime, to_set: bool) {
    let pairing_longer_cooldown_byte = [to_set.into(); BYTE_FIELD_SIZE];
    rt.eeprom_controller
        .write_slice(
            EepromReadWriteField::PairingLongerCooldownByte,
            &pairing_longer_cooldown_byte,
        )
        .expect("EEPROM write failed: pairing longer cooldown byte.");
}

fn check_pin_and_diffie_hellman(
    rt: &mut Runtime,
    pairing_pin_attempt: u32,
    pairing_longer_cooldown_byte: u8,
) -> bool {
    // Check PIN attempt.
    if !check_pin_attempt(rt, pairing_pin_attempt) {
        if pairing_longer_cooldown_byte == 0 {
            set_pairing_longer_cooldown_byte(rt, true);
        }

        return false;
    }

    // PIN is correct. Reset longer cooldown timer.
    if pairing_longer_cooldown_byte == 1 {
        set_pairing_longer_cooldown_byte(rt, false);
    }

    // Perform Diffie-Hellman key exchange and set UART1 channel key.
    diffie_hellman::run_paired(rt)
}

/// Processes pairing messages while paired.
pub(crate) fn paired_process_msg(rt: &mut Runtime, msg: &Uart0Message) {
    // Get PIN attempt.
    let pairing_pin_attempt = match msg {
        Uart0Message::PairingPin(msg) => msg.0,
        _ => return,
    };

    // Create cooldown timer.
    let mut hib_controller_cooldown = rt.hib_controller.clone();
    let (pairing_longer_cooldown_byte, mut pin_cooldown_timer) =
        get_pin_cooldown_timer(&mut rt.eeprom_controller, &mut hib_controller_cooldown);

    // Process PIN and Diffie-Hellman key exchange.
    let success =
        check_pin_and_diffie_hellman(rt, pairing_pin_attempt, pairing_longer_cooldown_byte);

    // Wait for cooldown timer to expire. Do this after Diffie-Hellman because it takes a while.
    while !pin_cooldown_timer.poll() {}

    // Pair if Diffie-Hellman was successful.
    if success {
        pairing_sequence::run_paired(rt);
    }
}
