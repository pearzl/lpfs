use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Stat {
    pid: u32,
    comm: String,
    state: String,
    ppid: u32,
    pgid: u32,
    sid: u32,
    tty_nr: usize,
    tty_pgrp: isize,
    flags: usize,
    min_flt: usize,
    cmin_flt: usize,
    maj_flt: usize,
    cmaj_flt: usize,
    utime: usize,
    stime: usize,
    cutime: usize,
    cstime: usize,
    priority: usize,
    nice: usize,
    num_threads: usize,
    it_real_value: usize,
    start_time: usize,
    vsize: usize,
    rss: usize,
    rlim: usize,
    start_code: usize,
    end_code: usize,
    start_stack: usize,
    kstkesp: usize,
    kstkeip: usize,
    pendingsig: usize,
    block_sig: usize,
    sigign: usize,
    sigcatch: usize,
    wchan: usize,
    nswap: usize,
    cnswap: usize,
    exit_signal: usize,
    task_cpu: usize,
    task_rt_priority: usize,
    task_policy: usize,
    blio_ticks: usize,
    gtime: usize,
    cgtime: usize,
    start_data: usize,
    end_data: usize,
    start_brk: usize,
    arg_start: usize,
    arg_end: usize,
    env_start: usize,
    env_end: usize,
    exit_code: usize,
}

impl Stat {
    getter_gen! {
        pid: u32,
        comm: String,
        state: String,
        ppid: u32,
        pgid: u32,
        sid: u32,
        tty_nr: usize,
        tty_pgrp: isize,
        flags: usize,
        min_flt: usize,
        cmin_flt: usize,
        maj_flt: usize,
        cmaj_flt: usize,
        utime: usize,
        stime: usize,
        cutime: usize,
        cstime: usize,
        priority: usize,
        nice: usize,
        num_threads: usize,
        it_real_value: usize,
        start_time: usize,
        vsize: usize,
        rss: usize,
        rlim: usize,
        start_code: usize,
        end_code: usize,
        start_stack: usize,
        kstkesp: usize,
        kstkeip: usize,
        pendingsig: usize,
        block_sig: usize,
        sigign: usize,
        sigcatch: usize,
        wchan: usize,
        nswap: usize,
        cnswap: usize,
        exit_signal: usize,
        task_cpu: usize,
        task_rt_priority: usize,
        task_policy: usize,
        blio_ticks: usize,
        gtime: usize,
        cgtime: usize,
        start_data: usize,
        end_data: usize,
        start_brk: usize,
        arg_start: usize,
        arg_end: usize,
        env_start: usize,
        env_end: usize,
        exit_code: usize
    }
}

impl FromStr for Stat {
    type Err = Error;

    fn from_str(line: &str) -> Result<Stat> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 52 {
            return Err(Error::BadFormat);
        }
        Ok(Stat {
            pid: columns[0].parse::<u32>().unwrap(),
            comm: columns[1].to_string(),
            state: columns[2].to_string(),
            ppid: columns[3].parse::<u32>().unwrap(),
            pgid: columns[4].parse::<u32>().unwrap(),
            sid: columns[5].parse::<u32>().unwrap(),
            tty_nr: columns[6].parse::<usize>().unwrap(),
            tty_pgrp: columns[7].parse::<isize>().unwrap(),
            flags: columns[8].parse::<usize>().unwrap(),
            min_flt: columns[9].parse::<usize>().unwrap(),
            cmin_flt: columns[10].parse::<usize>().unwrap(),
            maj_flt: columns[11].parse::<usize>().unwrap(),
            cmaj_flt: columns[12].parse::<usize>().unwrap(),
            utime: columns[13].parse::<usize>().unwrap(),
            stime: columns[14].parse::<usize>().unwrap(),
            cutime: columns[15].parse::<usize>().unwrap(),
            cstime: columns[16].parse::<usize>().unwrap(),
            priority: columns[17].parse::<usize>().unwrap(),
            nice: columns[18].parse::<usize>().unwrap(),
            num_threads: columns[19].parse::<usize>().unwrap(),
            it_real_value: columns[20].parse::<usize>().unwrap(),
            start_time: columns[21].parse::<usize>().unwrap(),
            vsize: columns[22].parse::<usize>().unwrap(),
            rss: columns[23].parse::<usize>().unwrap(),
            rlim: columns[24].parse::<usize>().unwrap(),
            start_code: columns[25].parse::<usize>().unwrap(),
            end_code: columns[26].parse::<usize>().unwrap(),
            start_stack: columns[27].parse::<usize>().unwrap(),
            kstkesp: columns[28].parse::<usize>().unwrap(),
            kstkeip: columns[29].parse::<usize>().unwrap(),
            pendingsig: columns[30].parse::<usize>().unwrap(),
            block_sig: columns[31].parse::<usize>().unwrap(),
            sigign: columns[32].parse::<usize>().unwrap(),
            sigcatch: columns[33].parse::<usize>().unwrap(),
            wchan: columns[34].parse::<usize>().unwrap(),
            nswap: columns[35].parse::<usize>().unwrap(),
            cnswap: columns[36].parse::<usize>().unwrap(),
            exit_signal: columns[37].parse::<usize>().unwrap(),
            task_cpu: columns[38].parse::<usize>().unwrap(),
            task_rt_priority: columns[39].parse::<usize>().unwrap(),
            task_policy: columns[40].parse::<usize>().unwrap(),
            blio_ticks: columns[41].parse::<usize>().unwrap(),
            gtime: columns[42].parse::<usize>().unwrap(),
            cgtime: columns[43].parse::<usize>().unwrap(),
            start_data: columns[44].parse::<usize>().unwrap(),
            end_data: columns[45].parse::<usize>().unwrap(),
            start_brk: columns[46].parse::<usize>().unwrap(),
            arg_start: columns[47].parse::<usize>().unwrap(),
            arg_end: columns[48].parse::<usize>().unwrap(),
            env_start: columns[49].parse::<usize>().unwrap(),
            env_end: columns[50].parse::<usize>().unwrap(),
            exit_code: columns[51].parse::<usize>().unwrap(),
        })
    }
}

pub fn stat_of(pid: u32) -> Result<Stat> {
    let content = std::fs::read_to_string(pid_path!(pid, "stat"))?;
    Stat::from_str(content.trim())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stat_of() {
        println!("{:?}", stat_of(1).unwrap());
    }
}
