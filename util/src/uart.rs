use crate::listener::{self, Listener};

pub struct UartHandle {
    // Write this and its implementation
    // Make it generic and write typedefs for Uart0Handle and Uart1Handle
    // The way they implement is with the struct Serial which contains
    // constructors for each uart thingy
    // Serial implements Read<u8> and Write<u8> which is a custom trait
    // that allows reading and writing bytes
    // this can probs block, so read and write in a loop or until an erro thats not wouldblock appears
    // It also implements core::fmt::Write

    // can we use Serial?
}

impl UartHandle {}

pub struct UartListener {}

impl Listener for UartListener {
    fn tick(&mut self) -> listener::Result<()> {
        todo!()
    }
}
