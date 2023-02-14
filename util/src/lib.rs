//! This crate contains utility functions for use by both the car and key fob.

#![warn(missing_docs)]
#![no_std]

extern crate bitvec;
extern crate chacha20poly1305;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate once_cell;
extern crate rand_chacha;
extern crate sha3;
extern crate tm4c123x_hal;

pub mod communication;
pub(crate) mod random;
pub mod runtime;
// TODO: pub mod button;

pub use runtime::Runtime;
pub use runtime::RuntimePeripherals;
