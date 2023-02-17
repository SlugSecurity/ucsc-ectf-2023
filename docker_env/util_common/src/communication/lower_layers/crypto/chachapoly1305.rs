use super::{KeyedChannel, RandomSource};
use crate::communication::{
    self,
    lower_layers::framing::{Frame, FramedTxChannel},
    CommunicationError, RxChannel, TxChannel,
};
use chacha20poly1305::{AeadCore, AeadInPlace, KeyInit, XChaCha20Poly1305};
use core::time::Duration;
use generic_array::GenericArray;
use typenum::Unsigned;

pub use chacha20poly1305::Key;

/// This typedef can be used to change what algorithm the channel in this module uses.
type ChannelAlgorithm = XChaCha20Poly1305;

type TagSize = <ChannelAlgorithm as AeadCore>::TagSize;
type NonceSize = <ChannelAlgorithm as AeadCore>::NonceSize;

const TAG_SIZE: usize = <TagSize as Unsigned>::USIZE;
const NONCE_SIZE: usize = <NonceSize as Unsigned>::USIZE;

/// This [`RxChannel`] wraps around another [`RxChannel`] to decrypt communications encrypted
/// by a [`XChacha20Poly1305TxChannel`], providing message authenticity and confidentiality.
/// When reading from an [`XChacha20Poly1305RxChannel`], care must be taken to ensure that
/// there is sufficient space to store the 16-byte tag and 24-byte nonce as well.
/// If a received message doesn't contain a nonce or authentication tag or has an invalid
/// authentication tag, a [`CommunicationError::RecvError`] is given. If the underlying
/// channel gives this error, it will be propagated up.
///
/// # ERRORS:
///
/// - [`CommunicationError::RecvError`] - The message didn't contain a nonce of the right size,
/// didn't match the authentication tag provided, didn't contain an authentication tag, couldn't
/// be read into the buffer because it was too small, or an error occurred while receiving the
/// message from the wrapped channel.
///
/// See the [`module`](super) documentation for more information on the cipher used.
pub struct XChacha20Poly1305RxChannel<T: RxChannel> {
    channel: T,
    decryptor: ChannelAlgorithm,
}

impl<T: RxChannel> XChacha20Poly1305RxChannel<T> {
    /// Creates a new [`XChacha20Poly1305RxChannel`] given an inner [`RxChannel`] and a
    /// decryption [`Key`].
    pub fn new(channel: T, rx_key: &Key) -> Self {
        Self {
            channel,
            decryptor: ChannelAlgorithm::new(rx_key),
        }
    }
}

impl<T: RxChannel> KeyedChannel for XChacha20Poly1305RxChannel<T> {
    type KeyType = Key;

    fn change_key(&mut self, new_key: &Self::KeyType) {
        self.decryptor = ChannelAlgorithm::new(new_key);
    }
}

impl<T: RxChannel> RxChannel for XChacha20Poly1305RxChannel<T> {
    fn recv(&mut self, dest: &mut [u8], timeout: Duration) -> communication::Result<usize> {
        const METADATA_SIZE: usize = TAG_SIZE + NONCE_SIZE;

        // Read message from inner channel.
        let bytes_read = self.channel.recv(dest, timeout)?;
        let dest = &mut dest[..bytes_read];

        // Check we have at least one byte of ciphertext.
        if dest.len() <= METADATA_SIZE {
            return Err(CommunicationError::RecvError);
        }

        // Split message from metadata.
        let (msg_body, metadata) = dest.split_at_mut(dest.len() - METADATA_SIZE);

        // Take nonce and tag
        let (&mut ref nonce, &mut ref tag) = metadata.split_at_mut(NONCE_SIZE);

        // Decrypt in place using the ciphertext, nonce, and tag
        self.decryptor
            .decrypt_in_place_detached(nonce.into(), b"", msg_body, tag.into())
            .map_err(|_| CommunicationError::RecvError)?;

        // Our decrypted buffer is at the beginning of our slice and we return the length of it.
        Ok(msg_body.len())
    }
}

/// This [`TxChannel`] wraps around a [`FramedTxChannel`] to encrypt communications encrypted by a [`XChacha20Poly1305TxChannel`],
/// providing message authenticity and confidentiality. This channel requires a [`RandomSource`] to generate a random nonce.
///
/// See the module-level documentation for more information on the cipher used.
pub struct XChacha20Poly1305TxChannel<T: FramedTxChannel, U: RandomSource> {
    channel: T,
    random_source: U,
    encryptor: ChannelAlgorithm,
}

impl<T: FramedTxChannel, U: RandomSource> XChacha20Poly1305TxChannel<T, U> {
    /// Creates a new [`XChacha20Poly1305TxChannel`] given an inner [`FramedTxChannel`] and an
    /// encryption [`Key`].
    pub fn new(channel: T, random_source: U, tx_key: &Key) -> Self {
        Self {
            channel,
            random_source,
            encryptor: ChannelAlgorithm::new(tx_key),
        }
    }
}

impl<T: FramedTxChannel, U: RandomSource> KeyedChannel for XChacha20Poly1305TxChannel<T, U> {
    type KeyType = Key;

    fn change_key(&mut self, new_key: &Self::KeyType) {
        self.encryptor = ChannelAlgorithm::new(new_key);
    }
}

impl<T: FramedTxChannel, U: RandomSource> TxChannel for XChacha20Poly1305TxChannel<T, U> {
    fn send(&mut self, buff: &mut [u8]) -> communication::Result<()> {
        let mut nonce: GenericArray<u8, NonceSize> = Default::default();

        // Fill nonce with random bytes.
        self.random_source.fill_rand_slice(&mut nonce);

        // Encrypt buff completely in place with no associated data, returning the auth tag.
        let tag = self
            .encryptor
            .encrypt_in_place_detached(&nonce, b"", buff)
            .map_err(|_| CommunicationError::SendError)?;

        // Write message in following order: Ciphertext + Nonce + Tag
        self.channel
            .frame::<3>(|| Frame::new().append(buff)?.append(&nonce)?.append(&tag))
    }
}
