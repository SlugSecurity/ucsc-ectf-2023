//! The entry point for the car firmware.

#![warn(missing_docs)]
#![no_main]
#![no_std]

#[cfg(feature = "panic-semihosting")]
extern crate panic_semihosting;

#[cfg(not(feature = "panic-semihosting"))]
extern crate panic_halt;

use core::{fmt::Write, time::Duration};
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use tm4c123x_hal::{CorePeripherals, Peripherals};
use ucsc_ectf_util_no_std::{
    communication::RxChannel,
    eeprom::{EepromReadWriteField, SECRET_SIZE},
    Runtime, RuntimePeripherals,
};
use zeroize::Zeroize;

mod features;
mod unlock;

const MAX_MESSAGE_SIZE: usize = 1024;

#[entry]
fn main() -> ! {
    // Start message.
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Starting firmware!").unwrap();

    // Grab peripherals.
    let core_peripherals = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let mut rt_peripherals = RuntimePeripherals::from((core_peripherals, peripherals));

    // Initialize runtime.
    let mut rt = Runtime::new(
        &mut rt_peripherals,
        &Default::default(),
        &Default::default(),
    );

    // Transmit and receive using unlock keys.
    let mut unlock_key_one = [0; SECRET_SIZE];
    let mut unlock_key_two = [0; SECRET_SIZE];
    rt.eeprom_controller
        .read_slice(EepromReadWriteField::UnlockKeyOne, &mut unlock_key_one)
        .unwrap();
    rt.eeprom_controller
        .read_slice(EepromReadWriteField::UnlockKeyTwo, &mut unlock_key_two)
        .unwrap();
    rt.uart1_controller.change_rx_key(&unlock_key_one.into());
    rt.uart1_controller.change_tx_key(&unlock_key_two.into());
    unlock_key_one.zeroize();
    unlock_key_two.zeroize();

    // Listen for unlock requests.
    loop {
        let mut receive_buffer = [0; MAX_MESSAGE_SIZE];

        // Process message if one is received on UART1.
        if let Ok(size_read) = rt.uart1_controller.recv_with_data_timeout(
            &mut receive_buffer,
            &mut rt.hib_controller.create_timer(Duration::from_secs(1000)),
        ) {
            unlock::process_msg(&mut rt, &receive_buffer[..size_read]);
        }
    }
}
