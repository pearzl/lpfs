use crate::{Error, Result};
use std::collections::HashMap;

pub fn environ_of(pid: u32) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(pid_path!(pid, "environ"))?;
    let environs: Vec<&str> = content.split('\0').collect();
    let mut map = HashMap::new();
    for envi in environs {
        if envi == "" {
            continue;
        }
        let kv: Vec<&str> = envi.splitn(2, '=').collect();
        if kv.len() != 2 {
            return Err(Error::BadFormat);
        }
        map.insert(kv[0].to_string(), kv[1].to_string());
    }
    Ok(map)
}
