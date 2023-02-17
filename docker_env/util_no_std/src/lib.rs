//! This crate contains utility modules for use by the car and key fob.

#![warn(missing_docs)]
#![no_std]

pub mod communication;
pub mod eeprom;
pub mod msg_parsing;
pub mod runtime;
pub mod timer;

pub(crate) mod random;

pub use runtime::Runtime;
pub use runtime::RuntimePeripherals;
pub use timer::Timer;
