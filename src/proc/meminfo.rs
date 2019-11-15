use crate::{Error, Result};
use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Deref;

/// represent the content of /proc/meminfo
/// 
/// ```
/// let info = meminfo().unwrap();
/// println!("total usable RAM is {} KiB.", info.get("MemTotal").unwrap());
/// ```
#[derive(Debug)]
pub struct MemInfo (HashMap<String, usize>);

impl Deref for MemInfo {
    type Target = HashMap<String, usize>;

    fn deref(&self) -> &HashMap<String, usize> {
        &self.0
    }
}

impl FromStr for MemInfo {
    type Err = Error;

    fn from_str(mi: &str) -> Result<Self> {
        let mut map = HashMap::new();
        for line in mi.lines() {
            let columns: Vec<&str> = line.split_ascii_whitespace().collect();
            if columns.len() != 3 {
                return Err(Error::BadFormat);
            }
            let key = columns[0].trim_end_matches(':').to_string();
            let value = columns[1].parse::<usize>()?;
            map.insert(key, value);
        }
        Ok(MemInfo ( map ))
    }
}

pub fn meminfo() -> Result<MemInfo> {
    let content = std::fs::read_to_string("/proc/meminfo")?;
    MemInfo::from_str(&content)
}
