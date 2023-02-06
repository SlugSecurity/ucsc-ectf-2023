//! A uDMA controller can be used to transfer memory to other memory, memory to peripherals, peripherals to memory, and peripherals to peripherals independently from the processor. The processor always gets priority when the controller and processor need to write to the same memory. To schedule a transfer, you must use a channel.
//!
//! Each DMA channel (a configurable portion which emits requests) can be assigned one of five peripherals or assigned for software use. See table on page 587 of the [specs](https://www.ti.com/lit/ds/spms376e/spms376e.pdf?ts=1675265156139) for what each channel can be assigned to. Channel 30 is solely for software use while anything else labeled as software can be used but may change to be for peripherals. There are 32 channels total.
//!
//! Channels have priorities based on the channel number and priority level bit for the channel. The priority level bit can be used to indicate default priority or high priority. This takes precedence over channel number. Priority for channel numbers goes in descending order where channel 0 has the highest priority in its priority level. (See page 588).
//!
//! These channels also have a configurable arbitration size, which is the number of items that are transferred in a burst before rearbitration can occur. The uDMA controller arbitrates among all channels to service the request with the highest priority first. Try to set lower priority channels to have lower arbitration sizes. (See page 588).
//!
//! Single requests can be disabled if necessary or both can be kept enabled. In a case where both a burst request and single request are queued from the same channel, the burst request takes priority. (See page 589 of the [specs](https://www.ti.com/lit/ds/spms376e/spms376e.pdf?ts=1675265156139) for request type support for each peripheral).
//!
//! There is a control table to store data about each uDMA channel. The control table must contain the primary control structure and optionally an alternate control structure which is used for certain transfer modes (see next paragraph). Any memory for unused table entries can be used. The table must be aligned to 1024 bytes. Each entry in the table is 16-byte aligned. Each entry contains 4 words: the source end pointer (note the pointer is inclusive), the destination end pointer (inclusive pointer), the control word, and an unused entry.
//!
//! The control word contains the size of the transfer data being sent to source and destination (not total size of source and destination), the address increment size of the source and destination, the arbitration size, the number of items to transfer, and the transfer mode. See page 611 of the [specs](https://www.ti.com/lit/ds/spms376e/spms376e.pdf?ts=1675265156139) for more details and page 599 for more details on transfer size and increment.
//!
//! There are various transfer modes. See page 591 for more details. We will probably use ping-pong mode for continuous data flow to receive. To make a transfer to a peripheral, configure and enable a channel and use basic mode, which will disable the channel once the transfer is done and only transfer if the peripheral is ready for data transfer.
//!
//! When using the uDMA to transfer data to and from a peripheral, disable interrupts for that peripheral.
//!
//! If we ever want to transfer data from memory to memory, use channel 30 which is solely for software transfers. Software transfers can generally be done with auto mode.
//!
// See 14.3.11 for how to use UART with the DMA.
// for UART DMA control, enable it thorugh UARTDMACTL
// can configure burst request threshold at UARTIFLS

mod channel;

use core::{cell::RefCell, sync::atomic::AtomicU32};

pub use channel::*;
use cortex_m::interrupt::Mutex;
use tm4c123x_hal::{
    sysctl::{self, Domain, PowerControl, PowerState, RunMode},
    tm4c123x::UDMA,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Selector {
    Zero,
    One,
    Two,
    Three,
    Four,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Channel {
    Zero(Selector),
    One(Selector),
    Two(Selector),
    Three(Selector),
    Four(Selector),
    Five(Selector),
    Six(Selector),
    Seven(Selector),
    Eight(Selector),
    Nine(Selector),
    Ten(Selector),
    Eleven(Selector),
    Twelve(Selector),
    Thirteen(Selector),
    Fourteen(Selector),
    Fifteen(Selector),
    Sixteen(Selector),
    Seventeen(Selector),
    Eighteen(Selector),
    Nineteen(Selector),
    Twenty(Selector),
    TwentyOne(Selector),
    TwentyTwo(Selector),
    TwentyThree(Selector),
    TwentyFour(Selector),
    TwentyFive(Selector),
    TwentySix(Selector),
    TwentySeven(Selector),
    TwentyEight(Selector),
    TwentyNine(Selector),
    Thirty(Selector),
    ThirtyOne(Selector),
}

/// This is an entry in the uDMA control table, which can be used to configure
/// uDMA channels. See the module level documentation for more information on
/// this. This data is packed so it will ensure there is absolutely no padding
/// in this struct, even within the fields because u32's must be 4 bytes. This
/// is done so that it has the correct memory layout when used in the control
/// table.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UdmaTableEntry {
    src_end_ptr: u32,
    dest_end_ptr: u32,
    control_word: u32,
}

const UDMA_CHANNEL_CT: usize = 32;

/// The control table length is two times the channel count
/// because of the primary and alternate control structures.
const CONTROL_TABLE_LEN: usize = UDMA_CHANNEL_CT * 2;

static CTL_TABLE: Mutex<RefCell<ControlTable>> = Mutex::new(RefCell::new(ControlTable::new()));

/// This is the control table to configure the uDMA channels. The C standard
/// guarantees there's never padding at the beginning of a struct, meaning our
/// table will be correctly aligned to 1024 bytes as required. The array will
/// be as if it were a C array as well, so the layout is correct. We use one field
/// for both the primary and alternate control structure so there's no potential
/// padding in between.
#[repr(C, align(1024))]
struct ControlTable {
    table: [UdmaTableEntry; CONTROL_TABLE_LEN],
}

impl ControlTable {
    const fn new() -> Self {
        ControlTable {
            table: [UdmaTableEntry {
                src_end_ptr: 0,
                dest_end_ptr: 0,
                control_word: 0,
            }; CONTROL_TABLE_LEN],
        }
    }
}

/// A handle to the uDMA controller. This provides an abstraction over the direct registers.
/// You can configure channels with this handle and read from peripherals using this. When this
/// controller is dropped, the uDMA module is reset and disabled. During the lifetime of this
/// struct, registers affecting the uDMA controller shouldn't be touched. Otherwise, the behavior
/// is unspecified and could cause undefined behavior with certain unsafe register writes. There
/// should also be never be two instances of this struct in existence at any given time, which is
/// why UDMA is mutably borrowed.
pub struct FramedUdmaController<'a, 'b> {
    udma: &'a mut UDMA,
    power_control: &'b PowerControl,
}

impl<'a, 'b> FramedUdmaController<'a, 'b> {
    /// Enables and resets the uDMA controller to work in both run and sleep mode
    /// and initializes the control table.
    pub fn enable_and_reset(udma: &'a mut UDMA, power_control: &'b PowerControl) -> Self {
        // Resets the uDMA module.
        sysctl::reset(power_control, Domain::MicroDma);

        // Enable the clock and uDMA module when the processor is
        // not asleep.
        sysctl::control_power(
            power_control,
            Domain::MicroDma,
            RunMode::Run,
            PowerState::On,
        );

        // Enable the clock and uDMA module when the processor is
        // asleep, so it can run even when using WFI.
        sysctl::control_power(
            power_control,
            Domain::MicroDma,
            RunMode::Sleep,
            PowerState::On,
        );

        // Enable the controller by setting the MASTEREN bit of the DMACFG register
        udma.cfg.write(|w| w.masten().set_bit());

        // Write the base address of our control table to DMACTLBASE.
        // SAFETY: This table the pointer points to is 1024 byte-aligned with the proper layout.
        // The UDMA peripheral is mutably borrowed to ensure no other writes during this read-modify-
        // write operation. This isn't just a write because there are reserved bits to preserve.
        // See section 9.2.5 of the specs at https://www.ti.com/lit/ds/spms376e/spms376e.pdf?ts=1675265156139
        // for the required layout.
        udma.ctlbase
            .write(|w| unsafe { w.addr().bits(core::ptr::addr_of!(CTL_TABLE) as u32 / 1024) });

        FramedUdmaController {
            udma,
            power_control,
        }
    }

    pub(crate) fn disable_channel(&self, ch: Channel) {
        todo!()
    }

    /// Creates a [`UdmaChannelBuilder`] that creates  for a channel and channel assignment
    /// provided by ch that has a source address of the source parameter and a
    /// destination buffer specified by the provided mutable dest slice. The source
    /// should be an address that contains a readable byte from a peripheral's data
    /// where the peripheral can assert a uDMA request. If it isn't, it could result
    /// in a fault if the address isn't readable and a request is made. This is still
    /// safe because this behavior is still completely defined. The builder can be
    /// used to optionally configure the channel priority and the arbitration size
    /// (how many bytes are received in a burst transfer). After each uDMA request,
    /// the controller reabitrates priorities among the active requesting channels
    /// to determine which request to service next. The builder can also be used
    /// to configure whether to allow burst transfers or not. The provided destination
    /// buffer should be at least [`MIN_CHANNEL_SRC_BUFF_LEN`] bytes long and the source
    /// address should point to an address in peripheral memory.
    ///
    /// # ERRORS:
    ///
    /// - [`ChannelCreationError::DestTooSmall`] - The provided destination buffer
    /// isn't at least [`MIN_CHANNEL_SRC_BUFF_LEN`] bytes long.
    /// - [`ChannelCreationError::AddrOutOfBounds`] - The provided source address isn't within
    /// peripheral memory given by the range [`PERIPHERAL_MEMORY_BEGIN`] to [`PERIPHERAL_MEMORY_END`] (inclusive).
    pub fn new_rx_channel<'c, 'd>(
        &'d self,
        ch: Channel,
        src: usize,
        dest: &'c mut [u8],
    ) -> Result<
        UdmaChannelBuilder<'c, 'd, 'a, 'b, FramedUdmaRxChannel<'c, 'd, 'a, 'b>>,
        ChannelCreationError,
    > {
        if dest.len() >= MIN_CHANNEL_SRC_BUFF_LEN {
            UdmaChannelBuilder::new(src, dest, self, ch)
        } else {
            Err(ChannelCreationError::DestTooSmall {
                len_provided: dest.len(),
            })
        }
    }

    /// Creates a [`UdmaChannelBuilder`] for a channel and channel assignment
    /// provided by ch that has a source address of the source parameter and a
    /// destination buffer specified by the provided mutable dest slice. The source
    /// should be an address that contains a readable byte from a peripheral's data
    /// where the peripheral can assert a uDMA request. If it isn't, it could result
    /// in a fault if the address isn't readable and a request is made. This is still
    /// safe because this behavior is still completely defined. The builder can be
    /// used to optionally configure the channel priority and the arbitration size
    /// (how many bytes are received in a burst transfer). After each uDMA request,
    /// the controller reabitrates priorities among the active requesting channels
    /// to determine which request to service next. The builder can also be used
    /// to configure whether to allow burst transfers or not. The provided destination
    /// buffer should be at least [`MIN_CHANNEL_SRC_BUFF_LEN`] bytes long and the source
    /// address should point to an address in peripheral memory.
    ///
    /// # ERRORS:
    ///
    /// - [`ChannelCreationError::DestTooSmall`] - The provided destination buffer
    /// isn't at least [`MIN_CHANNEL_SRC_BUFF_LEN`] bytes long.
    /// - [`ChannelCreationError::AddrOutOfBounds`] - The provided source address isn't within
    /// peripheral memory given by the range [`PERIPHERAL_MEMORY_BEGIN`] to [`PERIPHERAL_MEMORY_END`] (inclusive).
    pub fn new_tx_channel<'c, 'd>(
        &'d self,
        ch: Channel,
        src: &'c mut [u8],
        dest: usize,
    ) -> Result<
        UdmaChannelBuilder<'c, 'd, 'a, 'b, FramedUdmaTxChannel<'c, 'd, 'a, 'b>>,
        ChannelCreationError,
    > {
        // write can also be made safe as long as we restrict the dest
        // to be within peripheral memory (not main memory)
        todo!()
    }

    // we need operations to configure channels
    // maybe have an option to configure a channel with a UdmaTableEntry

    // See 9.3.4 to see instructions on how to configure for ping-pong, above is how to configure for basic

    // we need operations to subscribe to interrupts
}

impl<'a, 'b> Drop for FramedUdmaController<'a, 'b> {
    fn drop(&mut self) {
        sysctl::reset(self.power_control, Domain::MicroDma);

        // Disable the clock for the uDMA module
        sysctl::control_power(
            self.power_control,
            Domain::MicroDma,
            RunMode::Run,
            PowerState::Off,
        );

        // Disable the clock for the uDMA module
        sysctl::control_power(
            self.power_control,
            Domain::MicroDma,
            RunMode::Sleep,
            PowerState::Off,
        );
    }
}
