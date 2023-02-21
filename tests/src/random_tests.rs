#![cfg(debug_assertions)]

use core::fmt::Write;
use cortex_m_semihosting::hio::HostStream;
use ucsc_ectf_util_no_std::Runtime;

pub fn run(rt: &mut Runtime, stdout: &mut HostStream) {
    basic_slice_test(rt, stdout);
}

fn basic_slice_test(rt: &mut Runtime, stdout: &mut HostStream) {
    let mut data = [0; 16];
    rt.fill_rand_slice(&mut data);

    assert!(!data.iter().all(|&n| n == 0)); // Check that the slice is not all zeros.

    writeln!(stdout, "Verify the randomness of this slice: {:02X?}", data).unwrap();
}
