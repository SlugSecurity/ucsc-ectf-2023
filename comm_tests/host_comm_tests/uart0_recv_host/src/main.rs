use std::env;

use ucsc_ectf_comm_tests_common::run_recv_tests;
use ucsc_ectf_util_std::{communication::VerifiedFramedTcpSocket, timer::StdTimer};

fn main() {
    let mut args = env::args();

    args.next();

    let port = args
        .next()
        .expect("Expected bridge port.")
        .parse::<u16>()
        .expect("Bridge port not valid.");
    let mut socket = VerifiedFramedTcpSocket::keyless_connect(("ectf-net", port))
        .expect("Couldn't connect with specified port.");

    run_recv_tests(&mut socket, |d| StdTimer::new(d));

    println!("Finished receiving!");
}
