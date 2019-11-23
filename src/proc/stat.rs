//! ### `/proc/stat`
//! 
//! > 
//! > <a id="idm139746007987584" class="indexterm" href=""></a><a id="idm139745976880960" class="indexterm" href=""></a><a id="idm139746026472640" class="indexterm" href=""></a>
//! > 
//! > This file keeps track of a variety of different statistics about the system since it was last restarted. The contents of `/proc/stat`, which can be quite long, usually begins like the following example:
//! > 
//! > <pre class="screen">cpu  259246 7001 60190 34250993 137517 772 0
//! > cpu0 259246 7001 60190 34250993 137517 772 0
//! > intr 354133732 347209999 2272 0 4 4 0 0 3 1 1249247 0 0 80143 0 422626 5169433
//! > ctxt 12547729
//! > btime 1093631447
//! > processes 130523
//! > procs_running 1
//! > procs_blocked 0
//! > preempt 5651840
//! > cpu  209841 1554 21720 118519346 72939 154 27168
//! > cpu0 42536 798 4841 14790880 14778 124 3117
//! > cpu1 24184 569 3875 14794524 30209 29 3130
//! > cpu2 28616 11 2182 14818198 4020 1 3493
//! > cpu3 35350 6 2942 14811519 3045 0 3659
//! > cpu4 18209 135 2263 14820076 12465 0 3373
//! > cpu5 20795 35 1866 14825701 4508 0 3615
//! > cpu6 21607 0 2201 14827053 2325 0 3334
//! > cpu7 18544 0 1550 14831395 1589 0 3447
//! > intr 15239682 14857833 6 0 6 6 0 5 0 1 0 0 0 29 0 2 0 0 0 0 0 0 0 94982 0 286812
//! > ctxt 4209609
//! > btime 1078711415
//! > processes 21905
//! > procs_running 1
//! > procs_blocked 0</pre>
//! > 
//! > Some of the more commonly used statistics include:
//! > 
//! > *   `cpu` — Measures the number of _jiffies_ (1/100 of a second for x86 systems) that the system has been in user mode, user mode with low priority (nice), system mode, idle task, I/O wait, IRQ (hardirq), and softirq respectively. The IRQ (hardirq) is the direct response to a hardware event. The IRQ takes minimal work for queuing the "heavy" work up for the softirq to execute. The softirq runs at a lower priority than the IRQ and therefore may be interrupted more frequently. The total for all CPUs is given at the top, while each individual CPU is listed below with its own statistics. The following example is a 4-way Intel Pentium Xeon configuration with multi-threading enabled, therefore showing four physical processors and four virtual processors totaling eight processors.
//! > 
//! > *   `page` — The number of memory pages the system has written in and out to disk.
//! > 
//! > *   `swap` — The number of swap pages the system has brought in and out.
//! > 
//! > *   `intr` — The number of interrupts the system has experienced.
//! > 
//! > *   `btime` — The boot time, measured in the number of seconds since January 1, 1970, otherwise known as the _epoch_.
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-stat
//! 
//! 
//!
//! > kernel/system statistics. Varies with architecture. Common entries include:
//! > cpu 10132153 290696 3084719 46828483 16683 0 25195 0 175628 0
//! > cpu0 1393280 32966 572056 13343292 6130 0 17875 0 23933 0
//! > The amount of time, measured in units of USER_HZ (1/100ths of a second on most architectures, use sysconf(_SC_CLK_TCK) to obtain the right value), that the system ("cpu" line) or the specific CPU ("cpuN" line) spent in various states:
//! > user
//! > (1) Time spent in user mode.
//! > nice
//! > (2) Time spent in user mode with low priority (nice).
//! > system
//! > (3) Time spent in system mode.
//! > idle
//! > (4) Time spent in the idle task. This value should be USER_HZ times the second entry in the /proc/uptime pseudo-file.
//! > iowait (since Linux 2.5.41)
//! > (5) Time waiting for I/O to complete. This value is not reliable, for the following reasons:
//! > 1.
//! > The CPU will not wait for I/O to complete; iowait is the time that a task is waiting for I/O to complete. When a CPU goes into idle state for outstanding task I/O, another task will be scheduled on this CPU.
//! > 2.
//! > On a multi-core CPU, the task waiting for I/O to complete is not running on any CPU, so the iowait of each CPU is difficult to calculate.
//! > 3.
//! > The value in this field may decrease in certain conditions.
//! > irq (since Linux 2.6.0)
//! > (6) Time servicing interrupts.
//! > softirq (since Linux 2.6.0
//! > (7) Time servicing softirqs.
//! > steal (since Linux 2.6.11)
//! > (8) Stolen time, which is the time spent in other operating systems when running in a virtualized environment
//! > guest (since Linux 2.6.24)
//! > (9) Time spent running a virtual CPU for guest operating systems under the control of the Linux kernel.
//! > guest_nice (since Linux 2.6.33)
//! > (10) Time spent running a niced guest (virtual CPU for guest operating systems under the control of the Linux kernel).
//! > page 5741 1808
//! > The number of pages the system paged in and the number that were paged out (from disk).
//! > swap 1 0
//! > The number of swap pages that have been brought in and out.
//! > intr 1462898
//! > This line shows counts of interrupts serviced since boot time, for each of the possible system interrupts. The first column is the total of all interrupts serviced including unnumbered architecture specific interrupts; each subsequent column is the total for that particular numbered interrupt. Unnumbered interrupts are not shown, only summed into the total.
//! > disk_io: (2,0):(31,30,5764,1,2) (3,0):...
//! > (major,disk_idx):(noinfo, read_io_ops, blks_read, write_io_ops, blks_written)
//! > (Linux 2.4 only)
//! > ctxt 115315
//! > The number of context switches that the system underwent.
//! > btime 769041601
//! > boot time, in seconds since the Epoch, 1970-01-01 00:00:00 +0000 (UTC).
//! > processes 86031
//! > Number of forks since boot.
//! > procs_running 6
//! > Number of processes in runnable state. (Linux 2.5.45 onward.)
//! > procs_blocked 2
//! > Number of processes blocked waiting for I/O to complete. (Linux 2.5.45 onward.)
//! > softirq 229245889 94 60001584 13619 5175704 2471304 28 51212741 59130143 0 51240672
//! > This line shows the number of softirq for all CPUs. The first column is the total of all softirqs and each subsequent column is the total for particular softirq. (Linux 2.6.31 onward.)
//! >
//! > -- https://manpages.debian.org/testing/manpages/procfs.5.en.html
//! 

define_struct! {
    /// Each instance represent an cpu entry in /proc/stat
    /// 
    /// The fields of this struct reference to (stat.c)[https://github.com/torvalds/linux/blob/master/fs/proc/stat.c].
    /// 
    /// This struct implement Index trait.
    pub struct Cpu {
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    }
}

impl std::convert::From<[u64;10]> for Cpu {
    fn from(x: [u64;10]) -> Cpu {
        Cpu {
            user: x[0],
            nice: x[1],
            system: x[2],
            idle: x[3],
            iowait: x[4],
            irq: x[5],
            softirq: x[6],
            steal: x[7],
            guest: x[8],
            guest_nice: x[9],
        }
    }
}

use std::ops::Index;
impl Index<usize> for Cpu {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.user,
            1 => &self.nice,
            2 => &self.system,
            3 => &self.idle,
            4 => &self.iowait,
            5 => &self.irq,
            6 => &self.softirq,
            7 => &self.steal,
            8 => &self.guest,
            9 => &self.guest_nice,
            x => panic!(format!("Cpu has 10 item, but index {}", x))
        }
    }
}

impl Cpu {
    /// Return the total jiffies of cpu time.
    pub fn cpu_time(&self) -> u64 {
        self.user +
        self.nice +
        self.system +
        self.idle +
        self.iowait +
        self.irq +
        self.softirq +
        self.steal +
        self.guest +
        self.guest_nice
    }

    /// Return the number of jiffies on user mode.
    /// 
    /// user + nice
    pub fn user_time(&self) -> u64 {
        self.user + self.nice
    }

    /// Return the number of jiffies on kernel mode.
    /// 
    /// system + irq + softirq
    pub fn system_time(&self) -> u64 {
        self.system + self.irq + self.softirq
    }

    pub fn to_array(&self) -> [u64;10] {
        [
            self.user,
            self.nice,
            self.system,
            self.idle,
            self.iowait,
            self.irq,
            self.softirq,
            self.steal,
            self.guest,
            self.guest_nice,
        ]
    }
}

use std::str::FromStr;
impl FromStr for Cpu {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Cpu, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 11 {
            return Err(bfe!(format!("need 10 number to parse to Cpu")))
        }

        let mut cpu = [0;10];
        for (cpuv, strv) in cpu.iter_mut().zip(columns[1..].into_iter()) {
            *cpuv = strv.parse::<u64>()?;
        }

        Ok(cpu.into())
    }
}



define_struct! {
    /// Represent the content of /proc/stat, returned by (stat())[fn.stat.html]
    /// 
    /// The fields of this struct reference to (stat.c)[https://github.com/torvalds/linux/blob/master/fs/proc/stat.c].
    pub struct Stat {
        cpu: Cpu,
        cpu_n: Vec<Cpu>,
        intr: Vec<u64>,
        ctxt: u64,
        btime: u64,
        procs_running: u64,
        processes: u64,
        procs_blocked: u64,
        softirq: Vec<u64>,
    }
}

impl FromStr for Stat {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Stat, crate::ProcErr> {
        let lines: Vec<&str> = s.trim().lines().collect();
        let line_num = lines.len();
        if line_num < 8 {
            return Err(bfe!(String::from("It takes at least 8 lines to parse to Stat")));
        }

        let cpu = lines[0].parse::<Cpu>()?;

        let mut last_n = line_num - 1;

        let softirq_columns: Vec<&str> = lines[last_n].split_ascii_whitespace().collect();
        if softirq_columns[0] != "softirq" {
            return Err(bfe!(String::from("softirq not found")));
        }
        let mut softirq: Vec<u64> = vec![];
        for s in &softirq_columns[1..] {
            let n = s.parse::<u64>()?;
            softirq.push(n);
        }
        last_n -= 1;

        macro_rules! single_value{
            ($name: ident) => {
                let $name = lines[last_n]
                .trim_start_matches(stringify!($name))
                .trim()
                .parse::<u64>()
                .map_err(|_|bfe!(format!("failed to parse {}", stringify!($name))))?;
            last_n -= 1;
            }
        }

        single_value!(procs_blocked);
        single_value!(procs_running);
        single_value!(processes);
        single_value!(btime);
        single_value!(ctxt);

        let mut intr: Vec<u64> = vec![];
        for s in lines[last_n]
            .trim_start_matches("intr")
            .split_ascii_whitespace()
        {
            let n = s.parse::<u64>()?;
            intr.push(n);
        }
        last_n -= 1;

        let mut cpu_n = Vec::with_capacity(last_n);

        for i in 0..last_n {
            let t = lines[i + 1].parse::<Cpu>()?;
            cpu_n.push(t);
        }

        Ok(Stat {
            cpu, cpu_n, intr, ctxt, btime, processes,
            procs_running, procs_blocked, softirq
        })
    }
}

instance_impl! {
    stat, "/proc/stat", Stat
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_stat() {
        let content = {"\
cpu  3321955 860 1356594 496669212 37722 0 19000 0 0 0
cpu0 1663503 446 679198 248162881 18470 0 11461 0 0 0
cpu1 1658451 413 677395 248506330 19252 0 7539 0 0 0
intr 265018021 51 4 0 0 0 0 0 0 0 0 0 0 6 0 0 0 0 38498825 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 3776959 1833394 218053 542 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
ctxt 331738534
btime 1572024946
processes 3312700
procs_running 1
procs_blocked 0
softirq 298297245 3 133941424 620453 5325395 1833481 0 38653917 73984142 0 43938430
"
        };
        let correct = Stat {
            cpu: [3321955, 860, 1356594, 496669212, 37722, 0, 19000, 0, 0, 0].into(),
            cpu_n: vec![
                [1663503, 446, 679198, 248162881, 18470, 0, 11461, 0, 0, 0].into(),
                [1658451, 413, 677395, 248506330, 19252, 0, 7539, 0, 0, 0].into()
            ],
            intr: vec![265018021, 51, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 38498825, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3776959, 1833394, 218053, 542, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ctxt: 331738534,
            btime: 1572024946,
            processes: 3312700,
            procs_running: 1,
            procs_blocked: 0,
            softirq: vec![298297245, 3, 133941424, 620453, 5325395, 1833481, 0, 38653917, 73984142, 0, 43938430]
        };
        let s = content.parse::<Stat>().unwrap();
        assert_eq!(s, correct);
    }
}