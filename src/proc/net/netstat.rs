use crate::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct NetStat {
    tcp_ext: HashMap<String, usize>,
    ip_ext: HashMap<String, usize>,
}

impl FromStr for NetStat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 4 {
            return Err(Error::BadFormat);
        }

        let mut tcp_ext = HashMap::new();
        let keys = lines[0].split_ascii_whitespace().skip(1);
        let values = lines[1].split_ascii_whitespace().skip(1);
        for (k, v) in keys.zip(values) {
            let v = v.parse::<usize>()?;
            tcp_ext.insert(k.to_string(), v);
        }

        let mut ip_ext = HashMap::new();
        let keys = lines[2].split_ascii_whitespace().skip(1);
        let values = lines[3].split_ascii_whitespace().skip(1);
        for (k, v) in keys.zip(values) {
            let v = v.parse::<usize>()?;
            ip_ext.insert(k.to_string(), v);
        }

        Ok(NetStat { tcp_ext, ip_ext })
    }
}

pub fn netstat() -> Result<NetStat> {
    let content = std::fs::read_to_string("/proc/net/netstat")?;
    content.trim().parse()
}
