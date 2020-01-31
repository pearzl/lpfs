//! <pre>
//! > 5.2.21.  /proc/modules
//! > This file displays a list of all modules loaded into the kernel. Its contents vary based on the configuration and use of your system, but it should be organized in a similar manner to this sample /proc/modules file output:
//! > Note
//! > 
//! > This example has been reformatted into a readable format. Most of this information can also be viewed via the /sbin/lsmod command.
//! > nfs      170109  0 -          Live 0x129b0000
//! > lockd    51593   1 nfs,       Live 0x128b0000
//! > nls_utf8 1729    0 -          Live 0x12830000
//! > vfat     12097   0 -          Live 0x12823000
//! > fat      38881   1 vfat,      Live 0x1287b000
//! > autofs4  20293   2 -          Live 0x1284f000
//! > sunrpc   140453  3 nfs,lockd, Live 0x12954000
//! > 3c59x    33257   0 -          Live 0x12871000
//! > uhci_hcd 28377   0 -          Live 0x12869000
//! > md5      3777    1 -          Live 0x1282c000
//! > ipv6     211845 16 -          Live 0x128de000
//! > ext3     92585   2 -          Live 0x12886000
//! > jbd      65625   1 ext3,      Live 0x12857000
//! > dm_mod   46677   3 -          Live 0x12833000
//! > The first column contains the name of the module.
//! > The second column refers to the memory size of the module, in bytes.
//! > The third column lists how many instances of the module are currently loaded. A value of zero represents an unloaded module.
//! > The fourth column states if the module depends upon another module to be present in order to function, and lists those other modules.
//! > The fifth column lists what load state the module is in: Live, Loading, or Unloading are the only possible values.
//! > The sixth column lists the current kernel memory offset for the loaded module. This information can be useful for debugging purposes, or for profiling tools such as oprofile.
//! </pre>
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-modules
//! 

define_struct! {
    pub struct Module {
        name: String,
        mem_size: usize,
        instance_nums: usize,
        deps: Vec<String>,
        state: State,
        offset: usize,
    }
}

use std::str::FromStr;
impl FromStr for Module {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err("require 6 items to parse module".into());
        }
        let name = columns[0].to_string();
        let mem_size = columns[1].parse::<usize>()?;
        let instance_nums = columns[2].parse::<usize>()?;
        let deps: Vec<String> = if columns[3] == "-" {
            Vec::new()
        } else {
            let mut v: Vec<String> = columns[3].split(',').map(|s| s.to_string()).collect();
            v.pop();
            v
        };
        let state = State::from_str(columns[4])?;
        let offset = usize::from_str_radix(columns[5].trim_start_matches("0x"), 16)?;
        Ok(Module {
            name,
            mem_size,
            instance_nums,
            deps,
            state,
            offset,
        })
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Live,
    Loading,
    Unloading,
}

impl FromStr for State {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Live" {
            Ok(State::Live)
        } else if s == "Loading" {
            Ok(State::Loading)
        } else if s == "Unloading" {
            Ok(State::Unloading)
        } else {
            Err("unknow stat".into())
        }
    }
}

list_impl! {
    modules, "/proc/modules", Module, '\n', 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_state() {
        assert_eq!(State::Live, "Live".parse().unwrap());
        assert_eq!(State::Loading, "Loading".parse().unwrap());
        assert_eq!(State::Unloading, "Unloading".parse().unwrap());
        assert!("XYZ".parse::<State>().is_err())
    }

    #[test]
    fn test_parse_module1() {
        let source = "sbhid 53248 0 - Live 0x0000000000000000";
        let correct = Module{
            name: "sbhid".to_string(),
            mem_size: 53248,
            instance_nums: 0,
            deps: vec![],
            state: State::Live,
            offset: 0,
        };
        assert_eq!(correct, source.parse().unwrap());
    }

    #[test]
    fn test_parse_module2() {
        let source = "hid 110592 2 hid_generic,usbhid, Live 0x0000000000000000";
        let correct = Module {
            name: "hid".to_string(),
            mem_size: 110592,
            instance_nums: 2,
            deps: vec!["hid_generic".to_string(), "usbhid".to_string()],
            state: State::Live,
            offset: 0
        };
        assert_eq!(correct, source.parse().unwrap());
    }
}
