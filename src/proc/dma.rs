use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Dma {
    channel: usize,
    driver: String,
}

impl FromStr for Dma {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split(':').collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Dma {
            channel: columns[0].trim().parse::<usize>()?,
            driver: columns[1].trim().to_string(),
        })
    }
}

#[inline(always)]
fn to_dma(line: &str) -> Result<Dma> {
    Dma::from_str(line)
}

default_list! {
    dma, "/proc/dma", Dma, to_dma
}
