use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct DevMcast {
    index: usize,
    name: String,
    dmi_user: usize,
    dmi_gusers: usize,
    dmi_addr: usize,
}

impl FromStr for DevMcast {
    type Err = Error;

    fn from_str(s: &str) -> Result<DevMcast> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 5 {
            return Err(Error::BadFormat);
        }

        let index = columns[0].parse::<usize>()?;
        let name = columns[1].to_string();
        let dmi_user = columns[2].parse::<usize>()?;
        let dmi_gusers = columns[3].parse::<usize>()?;
        let dmi_addr = usize::from_str_radix(columns[4], 16)?;

        Ok(DevMcast {
            index,
            name,
            dmi_user,
            dmi_gusers,
            dmi_addr,
        })
    }
}

#[inline(always)]
fn to_dev_mcast(line: &str) -> Result<DevMcast> {
    DevMcast::from_str(line)
}

default_list! {
    dev_mcast, "/proc/net/dev_mcast", DevMcast, to_dev_mcast
}
