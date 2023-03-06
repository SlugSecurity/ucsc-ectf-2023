use std::time::Duration;

use clap::Parser;
use ucsc_ectf_util_std::{
    communication::{self, CommunicationError, RxChannel, TxChannel, VerifiedFramedTcpSocket},
    messages::{PairingPin, Uart0Message},
    timer::{StdTimer, Timer},
};

const RECV_BUFF_LEN: usize = 128;
const FAILED_PAIRING: &str = "Failed to pair fob.";

#[derive(Parser)]
struct Args {
    /// Bridge for the paired fob
    #[arg(long)]
    paired_fob_bridge: u16,

    /// Bridge for the unpaired fob
    #[arg(long)]
    unpaired_fob_bridge: u16,

    /// Program PIN
    #[arg(long)]
    pair_pin: String,
}

fn pair(pin: PairingPin, unpaired_port: u16, paired_port: u16) -> communication::Result<()> {
    let mut unpaired_socket =
        VerifiedFramedTcpSocket::keyless_connect(("ectf-net", unpaired_port))?;
    let mut paired_socket = VerifiedFramedTcpSocket::keyless_connect(("ectf-net", paired_port))?;
    let pin_msg = Uart0Message::PairingPin(pin);
    let mut pin_msg_bytes =
        postcard::to_allocvec(&pin_msg).map_err(|_| CommunicationError::InternalError)?;

    paired_socket.send(&mut pin_msg_bytes)?;

    let mut buff = [0; RECV_BUFF_LEN];
    let mut timeout_timer = StdTimer::new(Duration::from_secs(5));
    let _ = unpaired_socket.recv_with_data_timeout(&mut buff, &mut timeout_timer);

    // The stupid bridge is cutting off our acknowledgement messages, so we just check for any
    // message.
    if timeout_timer.poll() {
        return Err(CommunicationError::RecvError);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    let pairing_pin = match u32::from_str_radix(&args.pair_pin, 16) {
        Ok(pin) => pin,
        Err(_) => {
            println!("{FAILED_PAIRING}");
            return;
        }
    };

    match pair(
        PairingPin(pairing_pin),
        args.unpaired_fob_bridge,
        args.paired_fob_bridge,
    ) {
        Ok(()) => println!("Paired."),
        Err(_) => println!("{FAILED_PAIRING}"),
    }
}
