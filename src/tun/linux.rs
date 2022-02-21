use std::io;
use std::io::{Read, Write};
use std::io::Result;
use std::net::Ipv4Addr;

use tun::platform::Device;
use tun::platform::posix::{Reader, Writer};

use crate::common::net::proto::MTU;
use crate::tun::{Rx, TunDevice, Tx};

pub struct Linuxtun {
    fd: Device,
}