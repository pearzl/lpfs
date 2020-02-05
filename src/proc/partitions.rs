// 5.2.24.  /proc/partitions
// This file contains partition block allocation information. A sampling of this file from a basic system looks similar to the following:
// major minor  #blocks  name
//   3     0   19531250 hda
//   3     1     104391 hda1
//   3     2   19422585 hda2
// 253     0   22708224 dm-0
// 253     1     524288 dm-1
// Most of the information here is of little importance to the user, except for the following columns:
// major — The major number of the device with this partition. The major number in the /proc/partitions, (3), corresponds with the block device ide0, in /proc/devices.
// minor — The minor number of the device with this partition. This serves to separate the partitions into different physical devices and relates to the number at the end of the name of the partition.
// #blocks — Lists the number of physical disk blocks contained in a particular partition.
// name — The name of the partition.
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-partitions
//

define_struct! {
    pub struct Partition {
        major: usize,
        minor: usize,
        blocks: usize,
        name: String,
    }
}

use std::str::FromStr;
impl FromStr for Partition {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 4 {
            return Err("partition should have 4 fields".into());
        }
        let major = columns[0].parse::<usize>()?;
        let minor = columns[1].parse::<usize>()?;
        let blocks = columns[2].parse::<usize>()?;
        let name = columns[3].to_string();
        Ok(Partition {
            major,
            minor,
            blocks,
            name,
        })
    }
}

list_impl! {
    partitions, "/proc/partitions", Partition, '\n', 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_partition() {
        let source = "252        0   41943040 vda";
        let correct = Partition {
            major: 252,
            minor: 0,
            blocks: 41943040,
            name: "vda".to_string(),
        };
        assert_eq!(source.parse::<Partition>().unwrap(), correct)
    }
}
