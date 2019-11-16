use crate::{Error, Result};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

/// represent an entry in /proc/cpuinfo
///
/// There are some common fileds can be retrived by the same name method,
/// other fileds should retrvie from inner HashMap.
///
/// ```
/// use linux_proc::cpuinfo::*;
///
/// let info = cpuinfo().unwrap();
/// assert_eq!(info[0].cpu_family(), info[0].get("cpu family").unwrap());
/// ```
#[derive(Debug)]
pub struct CpuInfo(HashMap<String, String>);

impl CpuInfo {
    pub fn processor(&self) -> usize {
        self.get("processor").unwrap().parse::<usize>().unwrap()
    }

    pub fn cpu_family(&self) -> &str {
        self.get("cpu family").unwrap()
    }

    pub fn model_name(&self) -> &str {
        self.get("model name").unwrap()
    }

    pub fn cpu_mhz(&self) -> f64 {
        self.get("cpu MHz").unwrap().parse::<f64>().unwrap()
    }

    pub fn cache_size(&self) -> &str {
        self.get("cache size").unwrap()
    }

    pub fn siblings(&self) -> usize {
        self.get("siblings").unwrap().parse::<usize>().unwrap()
    }

    pub fn flags(&self) -> Vec<&str> {
        self.get("flags")
            .unwrap()
            .split_ascii_whitespace()
            .collect()
    }
}

impl Deref for CpuInfo {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &HashMap<String, String> {
        &self.0
    }
}

impl FromStr for CpuInfo {
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
        Ok(CpuInfo(ret))
    }
}

#[inline(always)]
fn to_cpuinfo(block: &str) -> Result<CpuInfo> {
    CpuInfo::from_str(block)
}

default_list! {
    cpuinfo, "/proc/cpuinfo", CpuInfo, to_cpuinfo, "\n\n"
}
