use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Partition {
    major: usize,
    minor: usize,
    blocks: usize,
    name: String,
}

impl FromStr for Partition {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        if columns.len() != 4 {
            return Err(Error::BadFormat);
        }
        let major = columns[0].parse::<usize>()?;
        let minor = columns[1].parse::<usize>()?;
        let blocks = columns[2].parse::<usize>()?;
        let name = columns[4].to_string();
        Ok(Partition {
            major,
            minor,
            blocks,
            name,
        })
    }
}

#[inline(always)]
fn to_partition(line: &str) -> Result<Partition> {
    Partition::from_str(line)
}

default_list! {
    partitions, "/proc/partitions", Partition, to_partition, '\n', 1
}
