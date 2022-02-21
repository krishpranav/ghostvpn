/*
 * Copyright (c) 2022, Krisna Pranav
 *
 * SPDX-License-Identifier: MIT License
 */

pub struct Mactun {
    fd: Device,
}

impl Mactun {

}

struct MactunTx {
    tx: Writer,
}

struct MactunRx {
    rx: Reader,
}

impl Tx for MactunTx {
    fn send_packet(&mut self, packet: &[u8]) -> Result<()> {
        self.tx.write(packet)?;
        Ok(()) 
    }
}