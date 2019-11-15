use crate::{Error, Result};

/// returned by [`interrupts`](fn.interrupts.html)
#[derive(Debug)]
pub enum Interrupt {
    Internal {
        name: String,
        counts: Vec<usize>,
        detail: String,
    },
    Device {
        irq_number: usize,
        counts: Vec<usize>,
        type_of: String,
        device_name: String,
    },
}

/// > Since Linux 2.6.24, for the i386 and x86-64 architec‐tures,
/// > at least, this also includes interrupts internal to the system.
/// >
/// > http://man7.org/linux/man-pages/man5/proc.5.html
///
/// For the reasons mentioned above, interrupt is represent by enum, Interrupt.
/// There are two variant, Internal and Device,
/// describe the internal interrupt and io device interrupt respectively.
///
/// Both variants contain counts filed, which is an Vector and represent the number of interrupts.
/// The length of Vector equals the CPU numbers.
/// The first element is for CPU0, second for CPU1, and so on.
pub fn interrupts() -> Result<Vec<Interrupt>> {
    let content = std::fs::read_to_string("/proc/interrupts")?;
    let mut ret = vec![];
    let mut line_iter = content.trim_end().lines();

    let cpu_line = line_iter.next().ok_or(Error::BadFormat)?;
    let mut cpu_num = 0;
    for _ in cpu_line.trim().split_ascii_whitespace() {
        cpu_num += 1;
    }

    for line in line_iter {
        let mut columns: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        let column1 = columns[0].trim_end_matches(':');
        let mut counts = Vec::with_capacity(cpu_num);

        if let Ok(irq) = column1.parse::<usize>() {
            for item in columns.iter().take(cpu_num + 1).skip(1) {
                let c = item.trim().parse::<usize>()?;
                counts.push(c);
            }

            let device_name = columns.pop().ok_or(Error::BadFormat)?.to_string();
            let type_of: String = columns[1 + cpu_num..].iter().copied().collect();

            ret.push(Interrupt::Device {
                irq_number: irq,
                counts,
                type_of,
                device_name,
            });
        } else if column1 == "ERR" {
            let c = columns[1].trim().parse::<usize>()?;
            counts.push(c);
            ret.push(Interrupt::Internal {
                name: "ERR".to_string(),
                counts,
                detail: "".to_string(),
            });
        } else if column1 == "MIS" {
            let c = columns[1].trim().parse::<usize>()?;
            counts.push(c);
            ret.push(Interrupt::Internal {
                name: "MIS".to_string(),
                counts,
                detail: "".to_string(),
            });
        } else {
            let name = column1.to_string();

            for item in columns.iter().take(cpu_num + 1).skip(1) {
                let c = item.trim().parse::<usize>()?;
                counts.push(c);
            }

            let detail: String = columns[1 + cpu_num..].iter().copied().collect();

            ret.push(Interrupt::Internal {
                name,
                counts,
                detail,
            })
        }
    }

    Ok(ret)
}
