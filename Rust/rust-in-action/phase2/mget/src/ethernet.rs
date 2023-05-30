use rand::RngCore;
use std::fmt;
use std::fmt::Display;
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let octet = &self.0;
        write!(f, "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
               octet[0], octet[1], octet[2],
               octet[3], octet[4], octet[5])
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0010;
        octets[0] &= 0b_1111_1110;
        MacAddress(octets)
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress(self.0)
    }
}
