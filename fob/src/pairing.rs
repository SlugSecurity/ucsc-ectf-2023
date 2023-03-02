use core::time::Duration;
use ucsc_ectf_util_no_std::{
    eeprom::{EepromReadWriteField, BYTE_SIZE, PAIRING_PIN_SIZE},
    messages::Uart0Message,
    timer::Timer,
    Runtime,
};
use zeroize::Zeroize;

mod diffie_hellman;
mod pairing_sequence;

/// Processes pairing messages while unpaired.
pub(crate) fn unpaired_listen_and_pair(rt: &mut Runtime) {
    loop {
        // Perform Diffie-Hellman key exchange and set UART1 channel key.
        if !diffie_hellman::run_unpaired(rt) {
            continue;
        }

        // Pair self. Break if pairing is successful.
        if pairing_sequence::run_unpaired(rt) {
            break;
        }
    }
}

/// Checks a pairing PIN with a cooldown if the PIN is incorrect.
fn check_pin_attempt(rt: &mut Runtime, pairing_pin_attempt: u32) -> bool {
    // Check pairing longer cooldown byte and create cooldown timer.
    let mut pairing_longer_cooldown_byte = [0; BYTE_SIZE];
    rt.eeprom_controller
        .read_slice(
            EepromReadWriteField::PairingLongerCooldownByte,
            &mut pairing_longer_cooldown_byte,
        )
        .expect("EEPROM read failed: pairing longer cooldown byte.");

    let mut pin_cooldown_timer = match pairing_longer_cooldown_byte {
        [0] => rt.hib_controller.create_timer(Duration::from_millis(900)),
        [1] => rt.hib_controller.create_timer(Duration::from_millis(4900)),
        _ => panic!("Invalid pairing longer cooldown byte."),
    };

    // Get pairing PIN from EEPROM and check against attempt.
    let mut pairing_pin_bytes = [0; PAIRING_PIN_SIZE];
    rt.eeprom_controller
        .read_slice(EepromReadWriteField::PairingPin, &mut pairing_pin_bytes)
        .expect("EEPROM read failed: pairing PIN.");
    let mut pairing_pin = u32::from_be_bytes(pairing_pin_bytes);
    pairing_pin_bytes.zeroize();
    let pairing_pin_correct = pairing_pin_attempt == pairing_pin;
    pairing_pin.zeroize();

    // Spin before resuming if PIN is wrong. Spin after comparison in case the comparison leaks information.
    if !pairing_pin_correct {
        while !pin_cooldown_timer.poll() {}

        if pairing_longer_cooldown_byte == [0] {
            pairing_longer_cooldown_byte = [1];
            rt.eeprom_controller
                .write_slice(
                    EepromReadWriteField::PairingLongerCooldownByte,
                    &pairing_longer_cooldown_byte,
                )
                .expect("EEPROM write failed: pairing longer cooldown byte.");
        }

        return false;
    }

    // PIN is correct. Reset longer cooldown timer.
    if pairing_longer_cooldown_byte == [1] {
        pairing_longer_cooldown_byte = [0];
        rt.eeprom_controller
            .write_slice(
                EepromReadWriteField::PairingLongerCooldownByte,
                &pairing_longer_cooldown_byte,
            )
            .expect("EEPROM write failed: pairing longer cooldown byte.");
    }

    true
}

/// Processes pairing messages while paired.
pub(crate) fn paired_process_msg(rt: &mut Runtime, msg: &Uart0Message) {
    // Check PIN attempt.
    let pairing_pin_attempt = match msg {
        Uart0Message::PairingPin(msg) => msg.0,
        _ => return,
    };

    if !check_pin_attempt(rt, pairing_pin_attempt) {
        return;
    }

    // Perform Diffie-Hellman key exchange and set UART1 channel key.
    if !diffie_hellman::run_paired(rt) {
        return;
    }

    // Pair unpaired fob.
    pairing_sequence::run_paired(rt);
}
