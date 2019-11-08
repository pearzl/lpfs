use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Swap {
    filename: String,
    r#type: String,
    size: usize,
    used: usize,
    priority: isize,
}

impl FromStr for Swap {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 5 {
            return Err(Error::BadFormat);
        }
        let filename = columns[0].to_string();
        let r#type = columns[1].to_string();
        let size = columns[2].parse::<usize>()?;
        let used = columns[3].parse::<usize>()?;
        let priority = columns[4].parse::<isize>()?;
        Ok(Swap {
            filename,
            r#type,
            size,
            used,
            priority,
        })
    }
}

#[inline(always)]
fn to_swaps(line: &str) -> Result<Swap> {
    Swap::from_str(line)
}

default_list! {
    swaps, "/proc/swaps", Swap, to_swaps, '\n', 1
}
