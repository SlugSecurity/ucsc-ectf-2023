extern crate rand_chacha;

use core::{ffi::c_uchar, mem::MaybeUninit};

use sha2::{Digest, Sha256};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

const fn get_random_bytes_size() -> usize {
    const FILE: &str = include_str!("../../rand_uninit_memory/rand_uninit_memory.h");

    let mut file_idx = 0;
    let mut line_len;
    let mut line_buff = [0; FILE.len()];
    let eq_check = b"#define RANDOM_BYTES_SIZE ";

    'outer: while file_idx < FILE.len() {
        line_len = 0;

        // Get each line and put it in line_buff.
        while file_idx < FILE.len() && FILE.as_bytes()[file_idx] != b'\n' {
            line_buff[line_len] = FILE.as_bytes()[file_idx];

            line_len += 1;
            file_idx += 1;
        }

        file_idx += 1;

        // This means we don't have enough bytes for this line to be the one.
        // Or it had too many to be valid (more than 20 characters).
        if line_len <= eq_check.len() || line_len > eq_check.len() + 20 {
            continue;
        }

        let mut line_idx = 0;

        // Check if our line begins with eq_check.
        while line_idx < eq_check.len() {
            if line_buff[line_idx] != eq_check[line_idx] {
                continue 'outer;
            }

            line_idx += 1;
        }

        let mut num = 0;

        // Now, we just need to get the number if there is one.
        while line_idx < line_len {
            if !line_buff[line_idx].is_ascii_digit() {
                continue 'outer;
            }

            num = num * 10 + (line_buff[line_idx] - b'0') as usize;

            line_idx += 1;
        }

        assert!(
            num == 1024,
            "Our old size was 1024. Now it is not. Did you mean to do this?"
        );

        return num;
    }

    assert!(false, "Bad header file. No size present.");
    unreachable!()
}

const RANDOM_BYTES_SIZE: usize = get_random_bytes_size();

#[link(name = "rand_uninit_memory", kind = "static")]
extern "aapcs" {
    static mut random_bytes: [c_uchar; RANDOM_BYTES_SIZE];

    fn init_random_bytes(new_rand_callback: unsafe extern "aapcs" fn(*mut MaybeUninit<c_uchar>));
}

// This is the callback function passed into the init_random_bytes function. The callback function is
// used to set the uninitialized stack memory to a new set of random values so that on the next CPU
// reset without a power cycle, there will be a new set of random "uninitialized" memory.
//
// We generate an SHA-256 hash of the uninitialized memory and use it to seed a ChaCha20 CSPRNG, which
// will generate uniform random numbers used to set the uninitialized memory for the next CPU reset.
//
// Safety:
// uninit_memory must be a valid pointer that points to an array of unsigned chars of size
// RANDOM_BYTES_SIZE or higher.
//
// random_bytes may only be modified on one thread.
//
// random_bytes must be fully initialized and be of size RANDOM_BYTES_SIZE or higher.
//
// This function can only be run on the same thread that random_bytes is modified on.
#[no_mangle]
unsafe extern "aapcs" fn new_rand_callback(uninit_memory: *mut MaybeUninit<c_uchar>) {
    let mut seed_hasher = Sha256::new();
    // SAFETY: The use of random_bytes is data-race-free due to the guarantees provided by this
    // function. Since random_bytes is fully initialized and is data-race-free, this use of
    // random_bytes is safe.
    seed_hasher.update(&random_bytes);
    let seed_hash = seed_hasher.finalize();
    let mut uninit_memory_rng = ChaCha20Rng::from_seed(seed_hash.into());

    for i in 0..RANDOM_BYTES_SIZE {
        // SAFETY: The use of offset() is safe because the size of uninit_memory is RANDOM_BYTES_SIZE
        // or higher, and we are only offsetting by a maximum of RANDOM_BYTES_SIZE - 1. We are offsetting
        // a pointer to a C unsigned char, which is one byte. Therefore, incrementing the offset by 1 each
        // iteration is safe.
        // SAFETY: The use of write_volatile() is safe because the pointer is always valid assuming the
        // uninit_memory pointer + offset is valid.
        uninit_memory
            .offset(i as isize)
            .write_volatile(MaybeUninit::new(uninit_memory_rng.next_u32() as u8));
    }
}
