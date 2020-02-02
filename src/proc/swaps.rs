//! > 5.2.28.  /proc/swaps
//! > This file measures swap space and its utilization. For a system with only one swap partition, the output of /proc/swaps may look similar to the following:
//! > Filename                          Type        Size     Used    Priority
//! > /dev/mapper/VolGroup00-LogVol01   partition   524280   0       -1
//! > While some of this information can be found in other files in the /proc/ directory, /proc/swaps provides a snapshot of every swap file name, the type of swap space, the total size, and the amount of space in use (in kilobytes). The priority column is useful when multiple swap files are in use. The lower the priority, the more likely the swap file is to be used.
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-swaps
//! 

define_struct!{
    pub struct Swap {
        filename: String,
        r#type: String,
        size: usize,
        used: usize,
        priority: isize,
    }
}

use std::str::FromStr;
impl FromStr for Swap {
    type Err = crate::ProcErr;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 5 {
            return Err("no enough fields".into());
        }
        println!("{:?}", columns);
        let filename = columns[0].to_string();
        let r#type = columns[1].to_string();
        let size = columns[2].parse::<usize>()?;
        let used = columns[3].parse::<usize>()?;
        let priority = columns[4].parse::<isize>()?;
        Ok(Swap {
            filename,
            r#type,
            size,
            used,
            priority,
        })
    }
}

list_impl!{
    swaps, "/proc/swaps", Swap, '\n', 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_swap() {
        let source = "/swapfile                               file            969964  0       -2";
        let correct = Swap{
            filename: "/swapfile".to_string(),
            r#type: "file".to_string(),
            size: 969964,
            used: 0,
            priority: -2
        };
        assert_eq!(correct, source.parse::<Swap>().unwrap());
    }

}