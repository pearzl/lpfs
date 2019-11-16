use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/ioports
#[derive(Debug)]
pub struct IoPort {
    start: usize,
    end: usize,
    device: String,
}

impl IoPort {
    getter_gen! {
        start: usize,
        end: usize,
        device: String
    }
}

impl FromStr for IoPort {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = value.split(|c| c == '-' || c == ':').collect();
        if items.len() != 3 {
            return Err(Error::BadFormat);
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let device = items[2].to_string();

        Ok(IoPort { start, end, device })
    }
}

#[inline(always)]
fn to_ioports(line: &str) -> Result<IoPort> {
    IoPort::from_str(line)
}

default_list! {
    ioports, "/proc/ioports", IoPort, to_ioports
}
