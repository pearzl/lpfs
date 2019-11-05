use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct IoPort {
    start: usize,
    end: usize,
    device: String
}

impl IoPort {
    getter_gen!{
        start: usize,
        end: usize,
        device: String
    }
}

impl FromStr for IoPort {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = line.split(|c| c=='-' || c == ':').collect();
        if items.len() != 3 {
            return Err(Error::BadFormat)
        }

        let start = usize::from_str_radic(items[0], 16)?;
        let end = usize::from_str_radic(items[1], 16)?;
        let device = items[2].to_string();

        Ok(IoPort{
            start, end, device
        })
    }
}

#[inline(always)]
fn to_iomem(line: &str) -> Result<IoPort> {
    IoPort::from_str(line)
}

default_list!{
    iomem, "/proc/IoPort", IoPort, to_iomem
}