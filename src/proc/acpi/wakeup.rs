use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/acpi/wakeup
#[derive(Debug)]
pub struct Device {
    device: String,
    s_state: String,
    status: String,
    sysfs_node: Option<String>,
}

impl Device {
    getter_gen! {
        device: String,
        s_state: String,
        status: String,
        sysfs_node: Option<String>
    }
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let items: Vec<&str> = s.split_ascii_whitespace().collect();
        if items.len() < 3 {
            return Err(Error::BadFormat);
        }

        Ok(Device {
            device: items[0].to_string(),
            s_state: items[1].to_string(),
            status: items[2].to_string(),
            sysfs_node: items.get(3).map(|s| (*s).to_string()),
        })
    }
}

#[inline(always)]
fn to_wakeup(line: &str) -> Result<Device> {
    Device::from_str(line)
}

default_list! {
    wakeup, "/proc/acpi/wakeup", Device, to_wakeup, '\n', 1
}
