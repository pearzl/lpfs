use crate::proc::{Error, Result};
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
pub struct Stat {
    cpu: Cpu,
    cpu_n: Vec<Cpu>,
    intr: Vec<usize>,
    ctxt: usize,
    btime: usize,
    procs_running: usize,
    processes: usize,
    procs_blocked: usize,
    softirq: Vec<usize>,
}

#[derive(Debug)]
pub struct Cpu {
    user: usize,
    nice: usize,
    system: usize,
    idle: usize,
    iowait: usize,
    irq: usize,
    softirq: usize,
    steal: usize,
    guest: usize,
    guest_nice: usize,
}

impl Cpu {
    getter_gen! {
        user: usize,
        nice: usize,
        system: usize,
        idle: usize,
        iowait: usize,
        irq: usize,
        softirq: usize,
        steal: usize,
        guest: usize,
        guest_nice: usize
    }
}

impl From<[usize; 10]> for Cpu {
    fn from(x: [usize; 10]) -> Cpu {
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

impl FromStr for Stat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Stat> {
        let lines: Vec<&str> = s.trim().lines().collect();
        let line_num = lines.len();
        if line_num < 8 {
            return Err(Error::BadFormat);
        }

        let mut cpu_arr = [0; 10];
        let cpu_columns: Vec<&str> = lines[0].split_ascii_whitespace().collect();
        if cpu_columns.len() != 11 || cpu_columns[0] != "cpu" {
            return Err(Error::BadFormat);
        }
        for (i, v) in cpu_columns[1..].iter().enumerate() {
            let n = v.parse::<usize>()?;
            cpu_arr[i] = n;
        }
        let cpu = Cpu::from(cpu_arr);

        let mut last_n = line_num - 1;

        let softirq_columns: Vec<&str> = lines[last_n].split_ascii_whitespace().collect();
        if softirq_columns[0] != "softirq" {
            return Err(Error::BadFormat);
        }
        let mut softirq: Vec<usize> = vec![];
        for s in &softirq_columns[1..] {
            let n = s.parse::<usize>()?;
            softirq.push(n);
        }
        last_n -= 1;

        let procs_blocked = lines[last_n]
            .trim_start_matches("procs_blocked")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let procs_running = lines[last_n]
            .trim_start_matches("procs_running")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let processes = lines[last_n]
            .trim_start_matches("processes")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let btime = lines[last_n]
            .trim_start_matches("btime")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let ctxt = lines[last_n]
            .trim_start_matches("ctxt")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let mut intr: Vec<usize> = vec![];
        for s in lines[last_n]
            .trim_start_matches("intr")
            .split_ascii_whitespace()
        {
            let n = s.parse::<usize>()?;
            intr.push(n);
        }
        last_n -= 1;

        let mut cpu_n = Vec::with_capacity(last_n);

        for i in 0..last_n {
            let mut cpu_arr = [0; 10];
            let columns: Vec<&str> = lines[i + 1]
                .trim_start_matches("cpu")
                .split_ascii_whitespace()
                .collect();
            if columns.len() != 11 || columns[0] != i.to_string() {
                return Err(Error::BadFormat);
            }
            for (k, v) in columns[1..].iter().enumerate() {
                cpu_arr[k] = v.parse::<usize>()?;
            }
            cpu_n.push(Cpu::from(cpu_arr));
        }

        Ok(Stat {
            cpu,
            cpu_n,
            intr,
            ctxt,
            btime,
            processes,
            procs_running,
            procs_blocked,
            softirq,
        })
    }
}

pub fn stat() -> Result<Stat> {
    let content = std::fs::read_to_string("/proc/stat")?;
    Stat::from_str(&content)
}
