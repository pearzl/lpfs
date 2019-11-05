use crate::proc::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct CpuInfo(HashMap<String, String>);

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
