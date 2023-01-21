use core::time::Duration;

use arrayvec::{ArrayVec, CapacityError};

mod error;
pub use error::*;

pub trait Listener {
    fn tick(&mut self) -> Result<(), ListenerError>;

    fn tick_with_delay(&mut self, delay: Duration) -> Result<(), ListenerError> {
        todo!() // make a default implementation using poll with it
    }
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
    ) -> Result<(), CapacityError<&'a dyn Listener>> {
        todo!()
    }
}

impl<'a, const CAPACITY: usize> Listener for Listeners<'a, CAPACITY> {
    fn tick(&mut self) -> Result<(), ListenerError> {
        todo!()
    }
}
