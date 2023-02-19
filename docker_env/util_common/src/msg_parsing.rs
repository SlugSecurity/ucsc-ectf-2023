//! This module is responsible for parsing the message types in messages.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// The UARTs that the car and key fob can send/receive messages on.
#[derive(Copy, Clone)]
pub enum UartType {
    /// UART0
    Uart0,
    /// UART1
    Uart1,
}

impl UartType {
    /// Checks if a message type is valid for a UART.
    pub fn is_message_valid(&self, msg_type: MessageType) -> bool {
        match self {
            UartType::Uart0 => match msg_type {
                // Computer
                MessageType::UnlockRequest => false,
                MessageType::UnlockChallengeOrResponse => false,
                MessageType::PairingPinOrResponse => true,
                MessageType::PairingRequest => false,
                MessageType::PairingChallengeOrResponse => false,
                MessageType::PackagedFeature => true,
                MessageType::Unknown => false,
            },
            UartType::Uart1 => match msg_type {
                // Another board
                MessageType::UnlockRequest => true,
                MessageType::UnlockChallengeOrResponse => true,
                MessageType::PairingPinOrResponse => false,
                MessageType::PairingRequest => true,
                MessageType::PairingChallengeOrResponse => true,
                MessageType::PackagedFeature => false,
                MessageType::Unknown => false,
            },
        }
    }
}

/// The type of message.
#[derive(Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    /// Unlock request from car to key fob.
    UnlockRequest,
    /// Unlock challenge/challenge response between car and key fob.
    UnlockChallengeOrResponse,
    /// Pairing PIN sent by host tool or pairing PIN response sent to host tool.
    PairingPinOrResponse,
    /// Pairing request from paired key fob to unpaired key fob.
    PairingRequest,
    /// Pairing challenge/challenge response between paired key fob and unpaired key fob.
    PairingChallengeOrResponse,
    /// Packaged feature sent by host tool to paired key fob.
    PackagedFeature,
    /// Unknown message type.
    Unknown,
}

impl From<u8> for MessageType {
    fn from(msg_type: u8) -> Self {
        match msg_type {
            0 => MessageType::UnlockRequest,
            1 => MessageType::UnlockChallengeOrResponse,
            2 => MessageType::PairingPinOrResponse,
            3 => MessageType::PairingRequest,
            4 => MessageType::PairingChallengeOrResponse,
            5 => MessageType::PackagedFeature,
            _ => MessageType::Unknown,
        }
    }
}

/// Gets the message type from a message slice. Checks if the message came from the correct UART.
///
/// # Panics
///
/// Panics if the message is empty.
pub fn get_msg_type(uart: UartType, msg: &[u8]) -> MessageType {
    let msg_type = msg[0].into();

    if uart.is_message_valid(msg_type) {
        msg_type
    } else {
        MessageType::Unknown
    }
}
