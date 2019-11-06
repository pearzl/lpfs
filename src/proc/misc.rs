use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Misc {
    device: usize,
    driver: String,
}

impl FromStr for Misc {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.trim().split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Misc {
            device: columns[0].parse::<usize>()?,
            driver: columns[1].to_string(),
        })
    }
}

#[inline(always)]
fn to_misc(line: &str) -> Result<Misc> {
    Misc::from_str(line)
}

default_list! { misc, "/proc/misc", Misc, to_misc }
