//let them configure arb size and priority, whether to allow burst transfers

use core::marker::PhantomData;

use self::private::Channel;

use super::{Channel as UdmaChannelNum, FramedUdmaController};

const WINDOW_SIZE: usize = 8;

/// The minimum size in bytes a provided source buffer for an [`FramedUdmaRxChannel`] can be.
pub const MIN_CHANNEL_SRC_BUFF_LEN: usize = WINDOW_SIZE * 2;

/// The lower bound of peripheral memory.
pub const PERIPHERAL_MEMORY_BEGIN: usize = 0x4000_0000;

/// The upper bound of peripheral memory.
pub const PERIPHERAL_MEMORY_END: usize = 0xDFFF_FFFF;

#[derive(Copy, Clone)]
pub enum ChannelCreationError {
    AddrOutOfBounds { addr_provided: usize },
    DestTooSmall { len_provided: usize },
}

#[derive(Copy, Clone, Default)]
pub enum ChannelPriority {
    Normal = 0,

    #[default]
    High = 1,
}

#[derive(Copy, Clone, Default)]
pub enum ArbitrationRate {
    EveryByte = 0,

    EveryTwoBytes = 1,

    EveryFourBytes = 2,

    #[default]
    EveryEightBytes = 3,
}

pub struct UdmaChannelBuilder<'a, 'b, 'c, 'd, T>
where
    T: Channel<'a, 'b, 'c, 'd>,
{
    addr: usize,
    buff: &'a mut [u8],
    channel: UdmaChannelNum,
    arb_rate: ArbitrationRate,
    allow_single_reqs: bool,
    controller: &'b FramedUdmaController<'c, 'd>,
    priority: ChannelPriority,
    _p: PhantomData<T>,
}

impl<'a, 'b, 'c, 'd, T> UdmaChannelBuilder<'a, 'b, 'c, 'd, T>
where
    T: Channel<'a, 'b, 'c, 'd>,
{
    pub(crate) fn new(
        addr: usize,
        buff: &'a mut [u8],
        controller: &'b FramedUdmaController<'c, 'd>,
        channel: UdmaChannelNum,
    ) -> Result<UdmaChannelBuilder<'a, 'b, 'c, 'd, T>, ChannelCreationError> {
        // Check addresses here

        Ok(UdmaChannelBuilder {
            addr,
            buff,
            channel,
            arb_rate: Default::default(),
            allow_single_reqs: true,
            controller,
            priority: Default::default(),
            _p: PhantomData,
        })
    }

    pub fn build(self) -> T {
        T::new(
            self.addr,
            self.buff,
            self.controller,
            self.channel,
            self.arb_rate,
            self.allow_single_reqs,
            self.priority,
        )
    }
}

mod private {
    use super::*;

    #[doc(hidden)]
    pub trait Channel<'a, 'b, 'c, 'd> {
        fn new(
            addr: usize,
            buff: &'a mut [u8],
            controller: &'b FramedUdmaController<'c, 'd>,
            channel: UdmaChannelNum,
            arb_rate: ArbitrationRate,
            allow_single_reqs: bool,
            priority: ChannelPriority,
        ) -> Self;
    }
}

struct UdmaChannel<'a, 'b, 'c> {
    ch: UdmaChannelNum,
    controller: &'a FramedUdmaController<'b, 'c>,
}

impl<'a, 'b, 'c> UdmaChannel<'a, 'b, 'c> {
    fn new(ch: UdmaChannelNum, controller: &'a FramedUdmaController<'b, 'c>) -> Self {
        todo!();

        // do the configurations for the options we must set but aren't specified by the user here

        // enable channel here

        UdmaChannel { ch, controller }
    }
}

impl<'a, 'b, 'c> Drop for UdmaChannel<'a, 'b, 'c> {
    fn drop(&mut self) {
        self.controller.disable_channel(self.ch)
    }
}

pub struct FramedUdmaRxChannel<'a, 'b, 'c, 'd> {
    dest: &'a mut [u8],
    channel: UdmaChannel<'b, 'c, 'd>,
}

impl<'a, 'b, 'c, 'd> FramedUdmaRxChannel<'a, 'b, 'c, 'd> {
    pub fn read(&mut self) {}

    pub fn read_exact() {}
}

impl<'a, 'b, 'c, 'd> Channel<'a, 'b, 'c, 'd> for FramedUdmaRxChannel<'a, 'b, 'c, 'd> {
    fn new(
        src: usize,
        dest: &'a mut [u8],
        controller: &'b FramedUdmaController<'c, 'd>,
        channel: UdmaChannelNum,
        arb_rate: ArbitrationRate,
        allow_single_reqs: bool,
        priority: ChannelPriority,
    ) -> Self {
        // configure user tunable stuff for channel

        // create UdmaChannel to enable it
        todo!()
    }
}

pub struct FramedUdmaTxChannel<'a, 'b, 'c, 'd> {
    src: &'a mut [u8],
    channel: UdmaChannel<'b, 'c, 'd>,
}

impl<'a, 'b, 'c, 'd> FramedUdmaTxChannel<'a, 'b, 'c, 'd> {
    pub fn write() {}

    pub fn write_all() {}
}

impl<'a, 'b, 'c, 'd> Channel<'a, 'b, 'c, 'd> for FramedUdmaTxChannel<'a, 'b, 'c, 'd> {
    fn new(
        dest: usize,
        src: &'a mut [u8],
        controller: &'b FramedUdmaController<'c, 'd>,
        channel: UdmaChannelNum,
        arb_rate: ArbitrationRate,
        allow_single_reqs: bool,
        priority: ChannelPriority,
    ) -> Self {
        // configure user tunable stuff for channel

        // create UdmaChannel to enable it

        todo!()
    }
}
