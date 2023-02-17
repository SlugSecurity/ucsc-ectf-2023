//! This crate contains definitions for the EEPROM layout.

#![warn(missing_docs)]
#![no_std]

/// The start address of the EEPROM.
const EEPROM_START_ADDRESS: usize = 0x000;

/// The start address of the EEPROM reserved message space.
const EEPROM_MESSAGES_START_ADDRESS: usize = 0x700;

/// The size of encryption secrets. 256 bits = 32 bytes.
pub const SECRET_SIZE: usize = 32;

/// The size of Postcard-encoded signatures.
pub const SIGNATURE_SIZE: usize = 64;

/// The max size of Postcard-encoded public keys.
pub const PUBLIC_KEY_SIZE: usize = 128;

/// The size of unlock/feature messages.
pub const MESSAGE_SIZE: usize = 64;

/// The size of the car ID. 32 bits = 4 bytes.
pub const CAR_ID_SIZE: usize = 4;

/// The size of the pairing byte.
pub const PAIRING_BYTE_SIZE: usize = 1;

/// The size of the pairing PIN.
pub const PAIRING_PIN_SIZE: usize = 4;

/// The bounds of the pairing secret EEPROM field.
const PAIRING_PRIVATE_KEY_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: EEPROM_START_ADDRESS,
    size: SECRET_SIZE,
};

/// The bounds of the pairing public key signature EEPROM field.
const PAIRING_PUBLIC_KEY_SIGNATURE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: PAIRING_PRIVATE_KEY_BOUNDS.address + PAIRING_PRIVATE_KEY_BOUNDS.size,
    size: SIGNATURE_SIZE,
};

/// The bounds of the pairing verifying key EEPROM field.
const PAIRING_VERIFYING_KEY_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: PAIRING_PUBLIC_KEY_SIGNATURE_BOUNDS.address + PAIRING_PUBLIC_KEY_SIGNATURE_BOUNDS.size,
    size: PUBLIC_KEY_SIZE,
};

/// The bounds of the feature verifying key EEPROM field.
const FEATURE_VERIFYING_KEY_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: PAIRING_VERIFYING_KEY_BOUNDS.address + PAIRING_VERIFYING_KEY_BOUNDS.size,
    size: PUBLIC_KEY_SIZE,
};

/// The bounds of the secret seed EEPROM field.
const SECRET_SEED_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: FEATURE_VERIFYING_KEY_BOUNDS.address + FEATURE_VERIFYING_KEY_BOUNDS.size,
    size: SECRET_SIZE,
};

/// The bounds of the unlock key 1 EEPROM field.
const UNLOCK_KEY_ONE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: SECRET_SEED_BOUNDS.address + SECRET_SEED_BOUNDS.size,
    size: SECRET_SIZE,
};

/// The bounds of the unlock key 2 EEPROM field.
const UNLOCK_KEY_TWO_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: UNLOCK_KEY_ONE_BOUNDS.address + UNLOCK_KEY_ONE_BOUNDS.size,
    size: SECRET_SIZE,
};

/// The bounds of the car ID EEPROM field.
const CAR_ID_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: UNLOCK_KEY_TWO_BOUNDS.address + UNLOCK_KEY_TWO_BOUNDS.size,
    size: CAR_ID_SIZE,
};

/// The bounds of the pairing byte EEPROM field.
const PAIRING_BYTE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: CAR_ID_BOUNDS.address + CAR_ID_BOUNDS.size,
    size: PAIRING_BYTE_SIZE,
};

/// The bounds of the pairing PIN EEPROM field.
const PAIRING_PIN_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: PAIRING_BYTE_BOUNDS.address + PAIRING_BYTE_BOUNDS.size + 3, // 3 bytes of padding for word alignment.
    size: PAIRING_PIN_SIZE,
};

/// The bounds of the feature three message EEPROM field.
const FEATURE_THREE_MESSAGE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: EEPROM_MESSAGES_START_ADDRESS,
    size: MESSAGE_SIZE,
};

/// The bounds of the feature two message EEPROM field.
const FEATURE_TWO_MESSAGE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: FEATURE_THREE_MESSAGE_BOUNDS.address + FEATURE_THREE_MESSAGE_BOUNDS.size,
    size: MESSAGE_SIZE,
};

/// The bounds of the feature one message EEPROM field.
const FEATURE_ONE_MESSAGE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: FEATURE_TWO_MESSAGE_BOUNDS.address + FEATURE_TWO_MESSAGE_BOUNDS.size,
    size: MESSAGE_SIZE,
};

/// The bounds of the unlock message EEPROM field.
const UNLOCK_MESSAGE_BOUNDS: EepromFieldBounds = EepromFieldBounds {
    address: FEATURE_ONE_MESSAGE_BOUNDS.address + FEATURE_ONE_MESSAGE_BOUNDS.size,
    size: MESSAGE_SIZE,
};

/// This enum specifies the fields of the EEPROM that can be read from, but not written to.
#[derive(Copy, Clone)]
pub enum EepromReadOnlyField {
    /// The secret of the key used for the Diffie-Hellman key exchange during pairing.
    PairingPrivateKey,
    /// The signature of the public key used for the Diffie-Hellman key exchange during pairing.
    PairingPublicKeySignature,
    /// The verifying key used for the Diffie-Hellman key exchange during pairing.
    PairingVerifyingKey,
    /// The verifying key used to verify packaged features.
    FeatureVerifyingKey,
    /// The key used as a starting point for the RNG seed hash.
    SecretSeed,
    /// The message to be printed when feature three is enabled.
    FeatureThreeMessage,
    /// The message to be printed when feature two is enabled.
    FeatureTwoMessage,
    /// The message to be printed when feature one is enabled.
    FeatureOneMessage,
    /// The message to be printed when the car is successfully unlocked.
    UnlockMessage,
}

/// This enum specifies the fields of the EEPROM that can be read from and written to.
#[derive(Copy, Clone)]
pub enum EepromReadWriteField {
    /// The key used to facilitate encrypted communications from a paired key fob to a car during the
    /// unlock sequence.
    UnlockKeyOne,
    /// The key used to facilitate encrypted communications from a car to a paired key fob during the
    /// unlock sequence.
    UnlockKeyTwo,
    /// The car ID.
    CarId,
    /// Whether or not a key fob is paired with a car.
    PairingByte,
    /// The pairing PIN used to authenticate the pairing of an unpaired key fob to a car, given a
    /// paired key fob.
    PairingPin,
}

/// A struct for EEPROM field bounds.
pub struct EepromFieldBounds {
    /// The address of the EEPROM field.
    pub address: usize,
    /// The size of the EEPROM field.
    pub size: usize,
}

/// A trait for all readable EEPROM fields.
pub trait EepromReadField: Copy {
    /// Returns the bounds of the EEPROM field.
    fn get_field_bounds(&self) -> EepromFieldBounds;
}

impl EepromReadField for EepromReadOnlyField {
    fn get_field_bounds(&self) -> EepromFieldBounds {
        match self {
            Self::PairingPrivateKey => PAIRING_PRIVATE_KEY_BOUNDS,
            Self::PairingPublicKeySignature => PAIRING_PUBLIC_KEY_SIGNATURE_BOUNDS,
            Self::PairingVerifyingKey => PAIRING_VERIFYING_KEY_BOUNDS,
            Self::FeatureVerifyingKey => FEATURE_VERIFYING_KEY_BOUNDS,
            Self::SecretSeed => SECRET_SEED_BOUNDS,
            Self::FeatureThreeMessage => FEATURE_THREE_MESSAGE_BOUNDS,
            Self::FeatureTwoMessage => FEATURE_TWO_MESSAGE_BOUNDS,
            Self::FeatureOneMessage => FEATURE_ONE_MESSAGE_BOUNDS,
            Self::UnlockMessage => UNLOCK_MESSAGE_BOUNDS,
        }
    }
}

impl EepromReadField for EepromReadWriteField {
    fn get_field_bounds(&self) -> EepromFieldBounds {
        match self {
            Self::UnlockKeyOne => UNLOCK_KEY_ONE_BOUNDS,
            Self::UnlockKeyTwo => UNLOCK_KEY_TWO_BOUNDS,
            Self::CarId => CAR_ID_BOUNDS,
            Self::PairingByte => PAIRING_BYTE_BOUNDS,
            Self::PairingPin => PAIRING_PIN_BOUNDS,
        }
    }
}
