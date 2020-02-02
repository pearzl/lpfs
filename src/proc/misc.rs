// 5.2.20.  /proc/misc
// This file lists miscellaneous drivers registered on the miscellaneous major device, which is device number 10:
// 63 device-mapper 175 agpgart 135 rtc 134 apm_bios
// The first column is the minor number of each device, while the second column shows the driver in use.
// 

define_struct! {
    pub struct Misc {
        minor_number: usize,
        driver: String,
    }
}

use std::str::FromStr;
impl FromStr for Misc {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err("not key-value pair".into())
        }
        let minor_number: usize = columns[0].parse()?;
        let driver = columns[1].to_string();
        Ok(Misc{
            minor_number, driver
        })
    }
}

list_impl! {
    misc, "/proc/misc", Misc, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_misc() {
        let source = " 56 memory_bandwidth";
        let correct = Misc{
            minor_number: 56, 
            driver: "memory_bandwidth".into()
        };
        assert_eq!(correct, source.parse().unwrap())
    }
}