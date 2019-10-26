
use super::{Result, Error};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Device {
    device : String,
    s_state: String,
    status: String,
    sysfs_node: Option<String>
}

pub fn wakeup() -> Result<Vec<Device>> {
    let content = read_to_string("/proc/acpi/wakeup")?;
    let mut ret = vec![];

    let mut line_iter = content.lines();
    let _ = line_iter.next();

    while let Some(line) = line_iter.next() {
        let items: Vec<&str> = line.split_ascii_whitespace().collect();
        if items.len() < 3 {
            return Err(Error::BadFormat);
        }

        ret.push(Device{
            device: items[0].to_string(),
            s_state: items[1].to_string(),
            status: items[2].to_string(),
            sysfs_node: items.get(3).map(|s| s.to_string()),
        })
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_wakeup() {
        println!("{:#?}", wakeup());
    } 
}
