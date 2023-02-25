use ucsc_ectf_util_no_std::{messages::Uart0Message, Runtime};

pub(crate) fn unpaired_listen_and_pair(rt: &mut Runtime) {
    loop {
        // TODO: Exchange keys and signatures.

        //rt.uart1_controller.change_rx_key(&session_key.into());
        //rt.uart1_controller.change_tx_key(&session_key.into());
        //session_key.zeroize();

        // TODO: Pair self and set EEPROM data. Don't forget to set the pairing byte. Break if done.
    }
}

pub(crate) fn paired_process_msg(rt: &mut Runtime, msg: &Uart0Message) {
    // TODO: Check message type and pair the unpaired fob.
}
