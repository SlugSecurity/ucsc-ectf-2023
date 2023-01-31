use core::time::Duration;

use arrayvec::{ArrayVec, CapacityError};

mod error;
pub use error::*;
use tm4c123x_hal::interrupt;

pub trait Listener {
    fn add_action(&mut self) -> Result<()>;
}

pub struct Listeners<'a, const CAPACITY: usize> {
    listeners: ArrayVec<&'a mut dyn Listener, CAPACITY>,
}

impl<'a, const CAPACITY: usize> Listeners<'a, CAPACITY> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(
        &mut self,
        listener: &'a mut dyn Listener,
    ) -> core::result::Result<(), CapacityError<&'a dyn Listener>> {
        todo!()
    }
}

impl<'a, const CAPACITY: usize> Listener for Listeners<'a, CAPACITY> {
    fn tick(&mut self) -> Result<()> {
        todo!()
    }
}

#[interrupt]
unsafe fn UART0() {
    static mut BUFFER: [u8; 1000] = [0; 1000];

    for f in BUFFER.iter_mut() {
        *f = 5;
    }
}
