extern crate rand;

use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let octet = &self.0;
        write!(f, "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
               octet[0], octet[1], octet[2],
               octet[3], octet[4], octet[5])
    }
}

impl MacAddress {
    fn new() -> MacAddress {
        let mut octets:[u8;6] = [0;6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0011; 
        MacAddress(octets)
    }

    fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0000
    }
}

fn main() {
    let mac = MacAddress::new();
    println!("MAC address: {}", mac);
    println!("Is unicast: {}", mac.is_unicast());
    println!("Is local: {}", mac.is_local());
}