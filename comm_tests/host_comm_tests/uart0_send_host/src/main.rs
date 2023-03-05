use std::{env, time::Duration};

use ucsc_ectf_comm_tests_common::run_send_tests;
use ucsc_ectf_util_std::{
    communication::{RxChannel, VerifiedFramedTcpSocket},
    timer::StdTimer,
};

const SUCCESS_MSG: &'static [u8] = b"success";

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

    run_send_tests(&mut socket, |d| StdTimer::new(d));

    println!("Finished sending!");

    let mut success = [0; 128];

    match socket.recv_with_data_timeout(
        &mut success,
        &mut StdTimer::new(Duration::from_millis(1000)),
    ) {
        Ok(_) if success.starts_with(SUCCESS_MSG) => println!("Successfully ran all receive tests."),
        _ => println!("Receive tests failed or failed to get right success message"),
    };
}
