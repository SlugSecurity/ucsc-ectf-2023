use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Instant,
};

use ucsc_ectf_util_common::{
    communication::{
        self,
        lower_layers::framing::{bogoframing, Frame, FramedTxChannel},
        CommunicationError, RxChannel,
    },
    timer::Timer,
};

const UART_FIFO_LEN: usize = 16;

/// The minimum size a framed UART message can be.
pub(crate) const MIN_FRAMED_UART_MESSAGE: usize = UART_FIFO_LEN;

pub(crate) fn connect(
    addr: impl ToSocketAddrs,
) -> Result<(FramedTcpTxChannel, FramedTcpRxChannel), CommunicationError> {
    let stream = TcpStream::connect(addr).map_err(|_| CommunicationError::InternalError)?;
    let stream_2 = stream
        .try_clone()
        .map_err(|_| CommunicationError::InternalError)?;

    Ok((
        FramedTcpTxChannel(BufWriter::new(stream)),
        FramedTcpRxChannel(BufReader::new(stream_2)),
    ))
}

fn read_byte(stream: &mut BufReader<TcpStream>) -> Result<u8, CommunicationError> {
    let mut data = [0; 1];

    // Our socket should never close, so if it does (returns 0), it's an error
    match stream.read(&mut data) {
        Ok(1..) => Ok(data[0]),
        _ => Err(CommunicationError::RecvError),
    }
}

pub struct FramedTcpRxChannel(BufReader<TcpStream>);

impl RxChannel for FramedTcpRxChannel {
    fn recv_with_data_timeout<T: Timer>(
        &mut self,
        dest: &mut [u8],
        timer: &mut T,
    ) -> communication::Result<usize> {
        let timeout_duration = timer.duration();

        // We keep track of this to not kill our CPUs on host tools :)
        self.0
            .get_ref()
            .set_read_timeout(Some(timeout_duration))
            .map_err(|_| CommunicationError::InternalError)?;

        bogoframing::recv_frame_with_timeout(
            self,
            dest,
            timer,
            |ch| read_byte(&mut ch.0),
            MIN_FRAMED_UART_MESSAGE,
        )
    }

    fn recv_with_timeout<T: Timer>(
        &mut self,
        dest: &mut [u8],
        timer: &mut T,
    ) -> communication::Result<usize> {
        // This implementation is here for completeness. We won't be using this...

        let start_instant = Instant::now();
        let timeout_duration = timer.duration();

        bogoframing::recv_frame_with_timeout(
            self,
            dest,
            timer,
            |ch| {
                // We keep track of this to not kill our CPUs on host tools :)
                ch.0.get_ref()
                    .set_read_timeout(Some(timeout_duration - (Instant::now() - start_instant)))
                    .map_err(|_| CommunicationError::InternalError)?;

                read_byte(&mut ch.0)
            },
            MIN_FRAMED_UART_MESSAGE,
        )
    }
}

pub struct FramedTcpTxChannel(BufWriter<TcpStream>);

impl FramedTxChannel for FramedTcpTxChannel {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> communication::Result<()> {
        let frame = frame()?;

        for part in frame {
            self.0
                .write(part)
                .map_err(|_| CommunicationError::SendError)?;
        }

        self.0.flush().map_err(|_| CommunicationError::SendError)?;

        Ok(())
    }
}
