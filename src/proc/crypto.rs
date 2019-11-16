use crate::{Error, Result};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

/// represent an entry in /proc/crypto
///
/// ```
/// use linux_proc::crypto::*;
///
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
            ret.insert(columns[0].trim().to_string(), columns[1].trim().to_string());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getter() {
        let source = r#"
name         : sha1
driver       : sha1-ssse3
module       : kernel
priority     : 150
refcnt       : 5
selftest     : passed
type         : shash
blocksize    : 64
digestsize   : 20
"#;
        let c = Crypto::from_str(source.trim()).unwrap();
        assert_eq!(c.get("name").unwrap(), c.name());
        assert_eq!(c.get("driver").unwrap(), c.driver());
        assert_eq!(c.get("module").unwrap(), c.module());
        assert_eq!(c.get("priority").unwrap(), &c.priority().to_string());
        assert_eq!(c.get("refcnt").unwrap(), &c.refcnt().to_string());
        assert_eq!(c.get("selftest").unwrap() == "passed", c.selftest());
        assert_eq!(c.get("type").unwrap(), c.r#type());
    }
}
