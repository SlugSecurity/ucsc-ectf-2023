//! This crate contains utility functions for use by both the car and key fob.

#![warn(missing_docs)]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate rand_chacha;
extern crate sha3;
extern crate tm4c123x_hal;

pub mod communication;
pub(crate) mod random;
// TODO: pub mod runtime;
// TODO: pub mod button;
