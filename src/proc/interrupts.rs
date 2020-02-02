// https://github.com/torvalds/linux/blob/cef7298262e9af841fb70d8673af45caf55300a1/kernel/irq/proc.c
// 
// 5.2.11.  /proc/interrupts
// This file records the number of interrupts per IRQ on the x86 architecture. A standard /proc/interrupts looks similar to the following:
//   CPU0
//   0:   80448940          XT-PIC  timer
//   1:     174412          XT-PIC  keyboard
//   2:          0          XT-PIC  cascade
//   8:          1          XT-PIC  rtc
//  10:     410964          XT-PIC  eth0
//  12:      60330          XT-PIC  PS/2 Mouse
//  14:    1314121          XT-PIC  ide0
//  15:    5195422          XT-PIC  ide1
// NMI:          0
// ERR:          0
// For a multi-processor machine, this file may look slightly different:
// 	   CPU0       CPU1
//   0: 1366814704          0          XT-PIC  timer
//   1:        128        340    IO-APIC-edge  keyboard
//   2:          0          0          XT-PIC  cascade
//   8:          0          1    IO-APIC-edge  rtc
//  12:       5323       5793    IO-APIC-edge  PS/2 Mouse
//  13:          1          0          XT-PIC  fpu
//  16:   11184294   15940594   IO-APIC-level  Intel EtherExpress Pro 10/100 Ethernet
//  20:    8450043   11120093   IO-APIC-level  megaraid
//  30:      10432      10722   IO-APIC-level  aic7xxx
//  31:         23         22   IO-APIC-level  aic7xxx
// NMI:          0
// ERR:          0
// The first column refers to the IRQ number. Each CPU in the system has its own column and its own number of interrupts per IRQ. The next column reports the type of interrupt, and the last column contains the name of the device that is located at that IRQ.
// Each of the types of interrupts seen in this file, which are architecture-specific, mean something different. For x86 machines, the following values are common:
// XT-PIC — This is the old AT computer interrupts.
// IO-APIC-edge — The voltage signal on this interrupt transitions from low to high, creating an edge, where the interrupt occurs and is only signaled once. This kind of interrupt, as well as the IO-APIC-level interrupt, are only seen on systems with processors from the 586 family and higher.
// IO-APIC-level — Generates interrupts when its voltage signal is high until the signal is low again.
// 
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-interrupts
// 
// /proc/interrupts
// This is used to record the number of interrupts per CPU per IO
// device.  Since Linux 2.6.24, for the i386 and x86-64 architec‐
// tures, at least, this also includes interrupts internal to the
// system (that is, not associated with a device as such), such
// as NMI (nonmaskable interrupt), LOC (local timer interrupt),
// and for SMP systems, TLB (TLB flush interrupt), RES
// (rescheduling interrupt), CAL (remote function call inter‐
// rupt), and possibly others.  Very easy to read formatting,
// done in ASCII.
// 
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    pub struct InternalInterrupt {
        name: String,
        /// The length of Vector equals the CPU numbers.
        /// The first element is for CPU0, second for CPU1, and so on.
        counts: Vec<usize>,
        details: String,
    } 
}

define_struct! {
    pub struct DeviceInterrupt {
        irq_number: usize,
        /// The length of Vector equals the CPU numbers.
        /// The first element is for CPU0, second for CPU1, and so on.
        counts: Vec<usize>,
        type_device: String,
    }
}

define_struct! {
    /// returned by [`interrupts()`](fn.interrupts.html)
    pub struct Interrupts {
        cpu_num: usize,
        internals: Vec<InternalInterrupt>,
        devices: Vec<DeviceInterrupt>,
    }
}

use std::str::FromStr;

impl FromStr for InternalInterrupt {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<InternalInterrupt, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() < 2 {
            return Err("require at least two clolums to parse an InternalInterrupt".into())
        }
        let name = columns[0].trim_end_matches(':').to_string();
        let mut cpu_num = 0;
        let mut counts = vec![];
        for item in columns[1..].iter() {
            if let Ok(n) = item.parse::<usize>() {
                cpu_num += 1;
                counts.push(n);
            }else {
                break;
            }
        }
        if counts.is_empty() {
            return Err("interrupt count not found".into());
        }
        let details = columns[1+cpu_num..].join(" ");
        Ok(InternalInterrupt{
            name, counts, details
        })
    }
}

impl FromStr for DeviceInterrupt {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<DeviceInterrupt, Self::Err>  {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() < 4{
            return Err("require at least four clolums to parse an InternalInterrupt".into())
        }
        let irq_number = columns[0].trim_end_matches(':').parse::<usize>()?;
        let mut cpu_num = 0;
        let mut counts = vec![];
        for item in columns[1..].iter() {
            if let Ok(n) = item.parse::<usize>() {
                cpu_num += 1;
                counts.push(n);
            }else {
                break;
            }
        }
        if counts.is_empty() {
            return Err("interrupt count not found".into());
        }
        let type_device = columns[1+cpu_num..].join(" ");
        Ok(DeviceInterrupt{
            irq_number, counts, type_device
        })
    }
}

impl FromStr for Interrupts {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Interrupts, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.is_empty() {
            return Err("no enough lines to parse Interrupts".into());
        }
        let cpu_num = lines[0].split_ascii_whitespace().count();
        let mut internals = vec![];
        let mut devices = vec![];
        let mut skip_lines = 1;
        for line in lines.iter().skip(skip_lines) {
            if let Ok(itnl) = line.parse::<InternalInterrupt>() {
                internals.push(itnl);
                skip_lines += 1;
            }else {
                break;
            }
        }
        for line in lines.iter().skip(skip_lines) {
            let dvs = line.parse::<DeviceInterrupt>()?; 
            devices.push(dvs);
        }
        Ok(Interrupts{
            cpu_num, internals, devices
        })
    }
}

instance_impl! {
    interrupts, "/proc/interrupts", Interrupts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_internal1() {
        let source = "MCP:        250   Machine check polls";
        let correct = InternalInterrupt {
            name: "MCP".to_string(),
            counts: vec![250],
            details: "Machine check polls".to_string()
        };
        assert_eq!(correct, source.parse::<InternalInterrupt>().unwrap())
    }

    #[test]
    fn test_parse_internal2() {
        let source = "MIS:          0";
        let correct = InternalInterrupt {
            name: "MIS".to_string(),
            counts: vec![0],
            details: "".to_string()
        };
        assert_eq!(correct, source.parse::<InternalInterrupt>().unwrap())
    }

    #[test]
    fn test_parse_device() {
        let source = "1:          9   IO-APIC   1-edge      i8042";
        let correct = DeviceInterrupt {
            irq_number: 1,
            counts: vec![9],
            type_device: "IO-APIC 1-edge i8042".to_string()
        };
        assert_eq!(correct, source.parse::<DeviceInterrupt>().unwrap());
    }
}