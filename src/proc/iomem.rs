use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct IoMem {
    start: usize,
    end: usize,
    kind: String,
}

impl IoMem {
    getter_gen! {
        start: usize,
        end: usize,
        kind: String
    }
}

impl FromStr for IoMem {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = value.split(|c| c == '-' || c == ':').collect();
        if items.len() != 3 {
            return Err(Error::BadFormat);
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let kind = items[2].to_string();

        Ok(IoMem { start, end, kind })
    }
}

#[inline(always)]
fn to_iomem(line: &str) -> Result<IoMem> {
    IoMem::from_str(line)
}

default_list! {
    iomem, "/proc/iomem", IoMem, to_iomem
}
