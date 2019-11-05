
use crate::proc::{Result, Error};
use std::str::FromStr;

#[derive(Debug)]
pub struct Device {
    major_number: usize,
    name: String
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat)
        }
        Ok(Device{
            major_number: columns[0].parse::<usize>(),
            name: columns[1].to_string()
        })
    }
}

pub fn devices() -> Result<(Vec<Device>, Vec<Device>)> {
    let content = std::fs::read_to_string("/proc/devices")?;
    let areas: Vec<&str> = content.split("\n\n").collect();
    if areas.len() != 2 {
        return Err(Error::BadFormat)
    }

    let mut character_iter = areas[0].trim().lines();
    let _ = character_iter.next();
    let character_devices: Vec<Device> = character_iter.map(|s| Device::from_str(s)?).collect();

    let mut block_iter = areas[1].trim().lines();
    let _ = block_iter.next();
    let block_devices: Vec<Device> = block_iter.map(|s| Device::from_str(s)?).collect();

    Ok((character_devices, block_devices))
}
