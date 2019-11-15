use crate::{Error, Result};
use std::str::FromStr;

/// represent the content of /proc/uptime
#[derive(Debug)]
pub struct Uptime {
    total: f64,
    idle: f64,
}

impl Uptime {
    getter_gen! {
        total: f64,
        idle: f64
    }
}

impl FromStr for Uptime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let columns: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        let total = columns[0].parse::<f64>()?;
        let idle = columns[1].parse::<f64>()?;
        Ok(Uptime { total, idle })
    }
}

pub fn uptime() -> Result<Uptime> {
    let content = std::fs::read_to_string("/proc/uptime")?;
    Uptime::from_str(&content)
}
