//! /proc/apm
//! 
//! > 
//! > This file provides information about the state of the _Advanced Power Management (APM)_ system and is used by the `apm` command. If a system with no battery is connected to an AC power source, this virtual file would look similar to the following:
//! > 
//! > <pre class="screen">1.16 1.2 0x07 0x01 0xff 0x80 -1% -1 ?</pre>
//! > 
//! > Running the `apm -v` command on such a system results in output similar to the following:
//! > 
//! > <pre class="screen">APM BIOS 1.2 (kernel driver 1.16ac) AC on-line, no system battery</pre>
//! > 
//! > For systems which do not use a battery as a power source, `apm` is able do little more than put the machine in standby mode. The `apm` command is much more useful on laptops. For example, the following output is from the command `cat /proc/apm` on a laptop while plugged into a power outlet:
//! > 
//! > <pre class="screen">1.16 1.2 0x03 0x01 0x03 0x09 100% -1 ?</pre>
//! > 
//! > When the same laptop is unplugged from its power source for a few minutes, the content of the `apm` file changes to something like the following:
//! > 
//! > <pre class="screen">1.16 1.2 0x03 0x00 0x00 0x01 99% 1792 min</pre>
//! > 
//! > The `apm -v` command now yields more useful data, such as the following:
//! > 
//! > <pre class="screen">APM BIOS 1.2 (kernel driver 1.16) AC off-line, battery status high: 99% (1 day, 5:52)</pre>
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-apm
//! >
//! 
//! 
//!
//! > 0) Linux driver version (this will change if format changes)
//! > 1) APM BIOS Version.  Usually 1.0, 1.1 or 1.2.
//! > 2) APM flags from APM Installation Check (0x00):
//! >    bit 0: APM_16_BIT_SUPPORT
//! >    bit 1: APM_32_BIT_SUPPORT
//! >    bit 2: APM_IDLE_SLOWS_CLOCK
//! >    bit 3: APM_BIOS_DISABLED
//! >    bit 4: APM_BIOS_DISENGAGED
//! > 3) AC line status
//! >    0x00: Off-line
//! >    0x01: On-line
//! >    0x02: On backup power (BIOS >= 1.1 only)
//! >    0xff: Unknown
//! > 4) Battery status
//! >    0x00: High
//! >    0x01: Low
//! >    0x02: Critical
//! >    0x03: Charging
//! >    0x04: Selected battery not present (BIOS >= 1.2 only)
//! >    0xff: Unknown
//! > 5) Battery flag
//! >    bit 0: High
//! >    bit 1: Low
//! >    bit 2: Critical
//! >    bit 3: Charging
//! >    bit 7: No system battery
//! >    0xff: Unknown
//! > 6) Remaining battery life (percentage of charge):
//! >    0-100: valid
//! >    -1: Unknown
//! > 7) Remaining battery life (time units):
//! >    Number of remaining minutes or seconds
//! >    -1: Unknown
//! > 8) min = minutes; sec = seconds */
//! >
//! > -- https://github.com/torvalds/linux/blob/86c2f5d653058798703549e1be39a819fcac0d5d/arch/x86/kernel/apm_32.c

define_struct! {
    /// Represent the content of /proc/apm, returnd by (apm())[fn.apm.html].
    ///
    /// fields of this struct reference to 
    /// [apm_32.c](https://github.com/torvalds/linux/blob/86c2f5d653058798703549e1be39a819fcac0d5d/arch/x86/kernel/apm_32.c#L1663) 
    pub struct Apm {
        driver_version: String,
        bios_version: (u8, u8),
        bios_flag: u8,
        ac_line_status: u8,
        battery_status: u8,
        battery_flag: u8,
        /// return None if remaining percentage is unknown.
        remain_percent: Option<u8>,
        /// return None if remaining time is unknown, or time units is seconds.
        remain_time: Option<u64>,
        unit: String,
    }
}

instance_impl! {
    apm, "/proc/apm", Apm
}

use std::str::FromStr;
impl FromStr for Apm {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Apm, Self::Err> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 9 {
            return Err(bfe!("unknow format".to_string()))
        }

        let driver_version = columns[0].to_string();

        let bios_version = {
            let vv: Vec<&str> = columns[1].split('.').collect();
            if vv.len() != 2 {
                return Err(bfe!("wrong bios version".to_string()))
            }
            (
                vv[0].parse::<u8>()?, 
                vv[1].parse::<u8>()?
            )
        };
        
        let bios_flag = u8::from_str_radix(&columns[2][2..], 16)?;
        
        let ac_line_status = u8::from_str_radix(&columns[3][2..], 16)?;
        
        let battery_status = u8::from_str_radix(&columns[4][2..], 16)?;
        
        let battery_flag = u8::from_str_radix(&columns[5][2..], 16)?;
        
        let remain_percent = {
            match columns[6].trim_end_matches('%').parse::<i8>()? {
                x if x <= 100 && x >= 0 => Some(x as u8),
                _ => None
            }
        };
        
        let remain_time = {
            match columns[7].parse::<i64>()? {
                x if x >= 0 => Some(x as u64),
                _ => None
            }
        };

        let unit = columns[8].to_string();

        Ok(Apm{
            driver_version, 
            bios_version,
            bios_flag,
            ac_line_status,
            battery_status,
            battery_flag,
            remain_percent,
            remain_time,
            unit
        })
    }
}

impl Apm {

    /// return remaining time in seconds.
    pub fn remain_time_sec(&self) -> Option <u64> {
        let unit = &self.unit;
        let unit_sec = if unit == "min" {
            Some(false)
        }else if unit == "sec" {
            Some(true)
        }else {
            None
        };

        match (self.remain_time, unit_sec) {
            (Some(v), Some(is_sec)) => {
                if is_sec {
                    Some(v)
                }else {
                    Some(v * 60)
                }
            },
            _ => None
        }
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_apm1() {
        let source = "1.16 1.2 0x07 0x01 0xff 0x80 -1% -1 ?";

        let apm_ = source.parse::<Apm>().unwrap();

        assert_eq!(apm_, Apm{
            driver_version: String::from("1.16"),
            bios_version: (1, 2),
            bios_flag: 0x07,
            ac_line_status: 0x01,
            battery_status: 0xff,
            battery_flag: 0x80,
            remain_percent: None,
            remain_time: None,
            unit: String::from("?")
        });

        assert_eq!(apm_.remain_time_sec(), None);
    }

    #[test]
    fn test_parse_apm2() {
        let source = "1.16 1.2 0x03 0x01 0x03 0x09 100% -1 ?";

        let apm_ = source.parse::<Apm>().unwrap();

        assert_eq!(apm_, Apm{
            driver_version: String::from("1.16"),
            bios_version: (1, 2),
            bios_flag: 0x03,
            ac_line_status: 0x01,
            battery_status: 0x03,
            battery_flag: 0x09,
            remain_percent: Some(100),
            remain_time: None,
            unit: String::from("?")
        });

        assert_eq!(apm_.remain_time_sec(), None);
    }

    #[test]
    fn test_parse_apm3() {
        let source = "1.16 1.2 0x03 0x00 0x00 0x01 99% 1792 min";

        let apm_ = source.parse::<Apm>().unwrap();

        assert_eq!(apm_, Apm{
            driver_version: String::from("1.16"),
            bios_version: (1, 2),
            bios_flag: 0x03,
            ac_line_status: 0x00,
            battery_status: 0x00,
            battery_flag: 0x01,
            remain_percent: Some(99),
            remain_time: Some(1792),
            unit: String::from("min")
        });

        assert_eq!(apm_.remain_time_sec(), Some(1792*60));
        
    }
}