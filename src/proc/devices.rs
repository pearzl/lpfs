use crate::{Error, Result};
use std::str::FromStr;

/// represented an entry in /proc/devices.
///
/// Both character devices and block devices.
#[derive(Debug)]
pub struct Device {
    major_number: usize,
    name: String,
}

impl Device {
    getter_gen! {
        major_number: usize,
        name: String
    }
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Device {
            major_number: columns[0].parse::<usize>()?,
            name: columns[1].to_string(),
        })
    }
}

/// return character devices and block devices.
///
/// ```
/// use linux_proc::devices::*;
///
/// let (cha, blk) = devices().unwrap();
///
/// println!("first character device is {}", cha[0].name());
/// println!("first block device is {}", blk[0].name());
/// ```
pub fn devices() -> Result<(Vec<Device>, Vec<Device>)> {
    let content = std::fs::read_to_string("/proc/devices")?;
    let areas: Vec<&str> = content.split("\n\n").collect();
    if areas.len() != 2 {
        return Err(Error::BadFormat);
    }

    let characters: Vec<&str> = areas[0].trim().lines().collect();
    let mut character_devices = vec![];
    for s in characters.iter().skip(1) {
        let t = Device::from_str(s)?;
        character_devices.push(t);
    }

    let blocks: Vec<&str> = areas[1].trim().lines().collect();
    let mut block_devices = vec![];
    for s in blocks.iter().skip(1) {
        let t = Device::from_str(s)?;
        block_devices.push(t);
    }

    Ok((character_devices, block_devices))
}
