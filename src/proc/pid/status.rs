use crate::{Error, Result};
use std::collections::HashMap;

pub fn status_of(pid: u32) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(pid_path!(pid, "status"))?;
    let mut map = HashMap::new();
    for line in content.trim().lines() {
        let kv: Vec<&str> = line.split(':').collect();
        if kv.len() != 2 {
            return Err(Error::BadFormat);
        }
        let k = kv[0].to_string();
        let v = kv[1].to_string();
        map.insert(k, v);
    }
    Ok(map)
}
