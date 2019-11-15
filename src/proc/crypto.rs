use crate::{Error, Result};
use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Deref;

/// represent an entry in /proc/crypto
///
/// ```
/// let info = crypto().unwrap();
/// assert_eq!(info[0].name(), info[0].get("name").unwrap());
/// ```
#[derive(Debug)]
pub struct Crypto(HashMap<String, String>);

impl Crypto {
    pub fn name(&self) -> &str {
        self.get("name").unwrap()
    }

    pub fn driver(&self) -> &str {
        self.get("driver").unwrap()
    }

    pub fn module(&self) -> &str {
        self.get("module").unwrap()
    }

    pub fn selftest(&self) -> bool {
        self.get("selftest").unwrap() == "passed"
    }

    pub fn refcnt(&self) -> usize {
        self.get("refcnt").unwrap().parse::<usize>().unwrap()
    }

    pub fn priority(&self) -> usize {
        self.get("priority").unwrap().parse::<usize>().unwrap()
    }

    pub fn r#type(&self) -> &str {
        self.get("type").unwrap()
    }
}

impl Deref for Crypto {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &HashMap<String, String> {
        &self.0
    }
}

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
