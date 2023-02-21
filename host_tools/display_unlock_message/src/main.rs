use std::time::Duration;

use clap::Parser;
use ucsc_ectf_util_std::{
    communication::{self, CommunicationError, RxChannel, VerifiedFramedTcpSocket},
    messages::{Uart0Message, UnlockMessage},
    timer::StdTimer,
};

const UNLOCK_BUFF_LEN: usize = 1024;

#[derive(Parser)]
struct Args {
    /// Port number of the socket for the car
    #[arg(long)]
    car_bridge: u16,
}

fn get_unlock_message<'a>(
    car_bridge: u16,
    buff: &'a mut [u8],
) -> communication::Result<UnlockMessage<'a>> {
    let mut socket = VerifiedFramedTcpSocket::keyless_connect(("ectf-net", car_bridge))?;
    let mut timeout_timer = StdTimer::new(Duration::from_secs(5));
    let msg_len = socket.recv_with_data_timeout(buff, &mut timeout_timer)?;
    let msg_bytes = &buff[..msg_len];
    let msg: Uart0Message =
        postcard::from_bytes(msg_bytes).map_err(|_| CommunicationError::RecvError)?;

    match msg {
        Uart0Message::HostUnlock(msg) => Ok(msg),
        _ => Err(CommunicationError::RecvError),
    }
}

fn main() {
    let args = Args::parse();
    let mut unlock_buff = [0; UNLOCK_BUFF_LEN];

    let msg = match get_unlock_message(args.car_bridge, &mut unlock_buff) {
        Ok(msg) => msg,
        Err(_) => {
            println!("Failed to unlock car because unlock message never came or was malformed or the port specified was bad.");

            return;
        }
    };

    println!("Unlock message: {}", msg.unlock_msg);

    for (idx, msg) in msg.feature_msgs.into_iter().enumerate() {
        println!("Feature message #{}: {msg}", idx + 1);
    }
}
