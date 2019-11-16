use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/fb
#[derive(Debug)]
pub struct Fb {
    device: usize,
    driver: String,
}

impl Fb {
    getter_gen! {
        device: usize,
        driver: String
    }
}

impl FromStr for Fb {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut columns = value.split_ascii_whitespace();
        let device = columns.next().ok_or(Error::BadFormat)?.parse::<usize>()?;
        let driver: String = columns.collect();
        Ok(Fb { device, driver })
    }
}

#[inline(always)]
fn to_fb(line: &str) -> Result<Fb> {
    Fb::from_str(line)
}

default_list! {
    fb, "/proc/fb", Fb, to_fb
}
