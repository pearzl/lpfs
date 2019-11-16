use crate::{Error, Result};
use std::convert::From;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct MacAddr([u8; 6]);

impl From<[u8; 6]> for MacAddr {
    fn from(s: [u8; 6]) -> Self {
        MacAddr(s)
    }
}

impl FromStr for MacAddr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let columns: Vec<&str> = s.split(':').collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }

        let mut et = [0; 6];
        for (e, v) in et.iter_mut().zip(columns.into_iter()) {
            *e = u8::from_str_radix(v, 16)?;
        }

        Ok(et.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arp {
    ip_addr: Ipv4Addr,
    hw_type: u16,
    flags: i32,
    hw_addr: MacAddr,
    mask: String,
    device: String,
}

impl FromStr for Arp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }

        let ip_addr = columns[0].parse::<Ipv4Addr>()?;
        let hw_type = u16::from_str_radix(columns[1].trim_start_matches("0x"), 16)?;
        let flags = i32::from_str_radix(columns[2].trim_start_matches("0x"), 16)?;
        let hw_addr = columns[3].parse::<MacAddr>()?;
        let mask = columns[4].to_string();
        let device = columns[5].to_string();

        Ok(Arp {
            ip_addr,
            hw_type,
            flags,
            hw_addr,
            mask,
            device,
        })
    }
}

#[inline(always)]
fn to_arp(line: &str) -> Result<Arp> {
    Arp::from_str(line)
}

default_list! {
    arp, "/proc/net/arp", Arp, to_arp, '\n', 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_arp() {
        let content = "192.168.0.50   0x1       0x2       00:50:BF:25:68:F3   *      eth0";
        let arp = content.parse::<Arp>().unwrap();
        assert_eq!(arp.ip_addr, Ipv4Addr::new(192, 168, 0, 50));
        assert_eq!(arp.hw_type, 1);
        assert_eq!(arp.flags, 2);
        assert_eq!(arp.hw_addr, MacAddr([0, 80, 191, 37, 104, 243]));
        assert_eq!(arp.mask, "*".to_string());
        assert_eq!(arp.device, "eth0".to_string());
    }
}
