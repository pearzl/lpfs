use crate::proc::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Crypto(HashMap<String, String>);

impl FromStr for Crypto {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut ret = HashMap::new();
        for line in value.trim().lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err(Error::BadFormat);
            }
            ret.insert(columns[0].to_string(), columns[1].to_string());
        }
        Ok(Crypto(ret))
    }
}

#[inline(always)]
fn to_crypto(block: &str) -> Result<Crypto> {
    Crypto::from_str(block)
}

default_list! {
    crypto, "/proc/crypto", Crypto, to_crypto, "\n\n"
}
