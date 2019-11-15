use crate::{Error, Result};
use std::collections::HashMap;

pub fn rtc() -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string("/proc/driver/rtc")?;
    let mut ret = HashMap::new();

    for line in content.lines() {
        let mut kv = line.split(':');
        let key = kv.next().ok_or(Error::BadFormat)?;
        let value = kv.next().ok_or(Error::BadFormat)?;

        ret.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(ret)
}
