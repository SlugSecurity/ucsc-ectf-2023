//! This module contains an interface to generate random numbers from two CSPRNGS: the main CSPRNG
//! and the secondary CSPRNG.
//!
//! The main CSPRNG is to be used by the runtime to generate random numbers and internally by this
//! crate when the runtime is known to be initialized. Initialization of this CSPRNG will block to
//! gather entropy.
//!
//! The secondary CSPRNG is to be used internally by this crate where the main CSPRNG has not been
//! initialized yet. Initialization of this CSPRNG does not need to block to gather entropy.

mod entropy;

use core::cell::RefCell;

use cortex_m::interrupt::{self, Mutex};

use once_cell::sync::OnceCell;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use self::entropy::{Adc, ClockDrift, EntropyHasher, Secret, UninitMemory};

static MAIN_CSPRNG: OnceCell<Mutex<RefCell<ChaCha20Rng>>> = OnceCell::new();
static SECONDARY_CSPRNG: OnceCell<Mutex<RefCell<ChaCha20Rng>>> = OnceCell::new();

/// Initializes the main CSPRNG. This initialization will block while gathering entropy.
///
/// Will do nothing if called more than once.
pub fn init_rng() {
    MAIN_CSPRNG.get_or_init(|| {
        Mutex::new(RefCell::new(ChaCha20Rng::from_seed(
            EntropyHasher::<UninitMemory<Secret<Adc<ClockDrift<()>>>>>::new().hash(),
        )))
    });
}

/// Fills a slice with random bytes from the main CSPRNG.
///
/// Panics if the main CSPRNG has not been initialized yet.
pub fn fill_rand_slice(dest: &mut [u8]) {
    interrupt::free(|c| {
        MAIN_CSPRNG
            .get()
            .expect("The main CSPRNG has not been initialized yet. Initialize it first with init_rng().")
            .borrow(c)
            .borrow_mut()
            .fill_bytes(dest);
    });
}

/// Initializes the secondary CSPRNG. This initialization does not need to block to gather entropy.
///
/// Will do nothing if called more than once.
pub fn init_secondary_rng() {
    SECONDARY_CSPRNG.get_or_init(|| {
        Mutex::new(RefCell::new(ChaCha20Rng::from_seed(
            // We use the secret twice here to make this entropy hash different from the one used to
            // seed the next piece of "uninitialized" memory.
            EntropyHasher::<UninitMemory<Secret<Secret<()>>>>::new().hash(),
        )))
    });
}

/// Fills a slice with random bytes from the secondary CSPRNG.
///
/// Panics if the secondary CSPRNG has not been initialized yet.
pub fn fill_rand_slice_secondary(dest: &mut [u8]) {
    interrupt::free(|c| {
        SECONDARY_CSPRNG
            .get()
            .expect("The secondary CSPRNG has not been initialized yet. Initialize it first with init_secondary_rng().")
            .borrow(c)
            .borrow_mut()
            .fill_bytes(dest);
    });
}
