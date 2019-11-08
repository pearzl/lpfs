use super::{Error, Result};

#[derive(Debug)]
pub struct Device {
    device: String,
    s_state: String,
    status: String,
    sysfs_node: Option<String>,
}

pub fn wakeup() -> Result<Vec<Device>> {
    let content = std::fs::read_to_string("/proc/acpi/wakeup")?;
    let mut ret = vec![];

    let mut line_iter = content.lines();
    let _ = line_iter.next();

    for line in line_iter {
        let items: Vec<&str> = line.split_ascii_whitespace().collect();
        if items.len() < 3 {
            return Err(Error::BadFormat);
        }

        ret.push(Device {
            device: items[0].to_string(),
            s_state: items[1].to_string(),
            status: items[2].to_string(),
            sysfs_node: items.get(3).map(|s| s.to_string()),
        })
    }

    Ok(ret)
}
