//! <pre>
//! > 5.2.12.  /proc/iomem
//! > This file shows you the current map of the system's memory for each physical device:
//! > 00000000-0009fbff : System RAM
//! > 0009fc00-0009ffff : reserved
//! > 000a0000-000bffff : Video RAM area
//! > 000c0000-000c7fff : Video ROM
//! > 000f0000-000fffff : System ROM
//! > 00100000-07ffffff : System RAM
//! > 00100000-00291ba8 : Kernel code
//! > 00291ba9-002e09cb : Kernel data
//! > e0000000-e3ffffff : VIA Technologies, Inc. VT82C597 [Apollo VP3] 
//! > e4000000-e7ffffff : PCI Bus #01
//! > e4000000-e4003fff : Matrox Graphics, Inc. MGA G200 AGP
//! > e5000000-e57fffff : Matrox Graphics, Inc. MGA G200 AGP
//! > e8000000-e8ffffff : PCI Bus #01
//! > e8000000-e8ffffff : Matrox Graphics, Inc. MGA G200 AGP
//! > ea000000-ea00007f : Digital Equipment Corporation DECchip 21140 [FasterNet]
//! > ea000000-ea00007f : tulip ffff0000-ffffffff : reserved
//! > The first column displays the memory registers used by each of the different types of memory. The second column lists the kind of memory located within those registers and displays which memory registers are used by the kernel within the system RAM or, if the network interface card has multiple Ethernet ports, the memory registers assigned for each port.
//! </pre>
//! 
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-iomem

define_struct! {
    /// represent an entry in /proc/iomem
    pub struct IoMem {
        start: usize,
        end: usize,
        kind: String,
    }
}

use std::str::FromStr;
impl FromStr for IoMem {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<IoMem, Self::Err> {
        let items: Vec<&str> = s.splitn(3 ,|c| c == '-' || c == ':').map(|s| s.trim()).collect();
        if items.len() != 3 {
            return Err("require three items at least to parse iomem".into());
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let kind = items[2].to_string();

        Ok(IoMem { start, end, kind })
    }
}

list_impl! {
    iomem, "/proc/iomem", IoMem, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_iomem() {
        let source = "009fc00-0009ffff : reserved";
        let correct = IoMem {
            start: 0x09fc00,
            end: 0x0009ffff,
            kind: "reserved".to_string()
        };
        assert_eq!(correct, source.parse::<IoMem>().unwrap());
    }
}