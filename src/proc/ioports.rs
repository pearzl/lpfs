//! > <pre>
//! > 5.2.13.  /proc/ioports
//! > The output of /proc/ioports provides a list of currently registered port regions used for input or output communication with a device. This file can be quite long. The following is a partial listing:
//! > 0000-001f : dma1
//! > 0020-003f : pic1
//! > 0040-005f : timer
//! > 0060-006f : keyboard
//! > 0070-007f : rtc
//! > 0080-008f : dma page reg
//! > 00a0-00bf : pic2
//! > 00c0-00df : dma2
//! > 00f0-00ff : fpu
//! > 0170-0177 : ide1
//! > 01f0-01f7 : ide0
//! > 02f8-02ff : serial(auto)
//! > 0376-0376 : ide1
//! > 03c0-03df : vga+
//! > 03f6-03f6 : ide0
//! > 03f8-03ff : serial(auto)
//! > 0cf8-0cff : PCI conf1
//! > d000-dfff : PCI Bus #01
//! > e000-e00f : VIA Technologies, Inc. Bus Master IDE
//! > e000-e007 : ide0
//! > e008-e00f : ide1
//! > e800-e87f : Digital Equipment Corporation DECchip 21140 [FasterNet]
//! > e800-e87f : tulip
//! > The first column gives the I/O port address range reserved for the device listed in the second column.
//! </pre>
//! 
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-ioports
//! 
//! 

define_struct! {
    pub struct IoPort {
        range: (usize, usize),
        device: String,
    }
}

use std::str::FromStr;
impl FromStr for IoPort {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<IoPort, Self::Err> {
        let items: Vec<&str> = s.split(|c| c == '-' || c == ':').map(|s| s.trim()).collect();
        if items.len() != 3 {
            return Err("require three items at least to parse ioport".into());
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let device = items[2].to_string();

        Ok(IoPort { range: (start, end), device })
    }
}

list_impl! {
    ioports, "/proc/ioports", IoPort, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_ioport() {
        let source = "00f0-00ff : fpu";
        let correct = IoPort{
            range: (0xf0, 0xff),
            device: "fpu".to_string()
        };
        assert_eq!(correct, source.parse::<IoPort>().unwrap());
    }
}