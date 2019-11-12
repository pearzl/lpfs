use super::{Error, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

macro_rules! path {
    ($pid: expr, $fname: expr) => {
        format!("/proc/{}/{}", $pid, $fname)
    };
}

pub fn cmdline_of(pid: u32) -> Result<String> {
    Ok(std::fs::read_to_string(path!(pid, "cmdline"))?)
}

pub fn cwd_of(pid: u32) -> Result<PathBuf> {
    Ok(std::fs::read_link(path!(pid, "cwd"))?)
}

pub fn environ_of(pid: u32) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(path!(pid, "environ"))?;
    let environs: Vec<&str> = content.split('\0').collect();
    let mut map = HashMap::new();
    for envi in environs {
        if envi == "" {
            continue;
        }
        let kv: Vec<&str> = envi.splitn(2, '=').collect();
        if kv.len() != 2 {
            return Err(Error::BadFormat);
        }
        map.insert(kv[0].to_string(), kv[1].to_string());
    }
    Ok(map)
}

pub fn exe_of(pid: u32) -> Result<PathBuf> {
    Ok(std::fs::read_link(path!(pid, "exe"))?)
}

pub fn fd_of(pid: u32) -> Result<Vec<(PathBuf)>> {
    unimplemented! {}
}

pub struct Map {
    addr_begin: usize,
    addr_end: usize,
    permission: String,
    offset: usize,
    device_major: usize,
    device_minor: usize,
    inode: usize,
    path: PathBuf,
}

impl Map {
    getter_gen! {
        addr_begin: usize,
        addr_end:   usize,
        permission: String,
        offset:     usize,
        device_major: usize,
        device_minor: usize,
        inode: usize,
        path: PathBuf
    }

    pub fn addr(&self) -> (usize, usize) {
        (self.addr_begin, self.addr_end)
    }

    pub fn device(&self) -> String {
        format!("{}:{}", self.device_major, self.device_minor)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }

        let addr: Vec<&str> = columns[0].split('-').collect();
        if addr.len() != 2 {
            return Err(Error::BadFormat);
        }
        let addr_begin = usize::from_str_radix(addr[0], 16)?;
        let addr_end = usize::from_str_radix(addr[1], 16)?;

        let permission = columns[1].to_string();

        let offset = usize::from_str_radix(columns[2], 16)?;

        let device: Vec<&str> = columns[3].split(':').collect();
        if device.len() != 2 {
            return Err(Error::BadFormat);
        }
        let device_major = usize::from_str_radix(device[0], 16)?;
        let device_minor = usize::from_str_radix(device[1], 16)?;

        let inode = columns[4].parse::<usize>()?;

        let path = PathBuf::from(columns[5]);

        Ok(Map {
            addr_begin,
            addr_end,
            permission,
            offset,
            device_major,
            device_minor,
            inode,
            path,
        })
    }
}

pub fn maps_of(pid: u32) -> Result<Vec<Map>> {
    let content = std::fs::read_to_string(path!(pid, "maps"))?;
    let mut ret = vec![];
    for line in content.trim().lines() {
        ret.push(Map::from_str(line)?)
    }
    Ok(ret)
}

pub fn mem_of() {
    unimplemented! {}
}

pub fn root_of(pid: u32) -> Result<PathBuf> {
    Ok(std::fs::read_link(path!(pid, "root"))?)
}

pub struct Stat {
    pid: u32,
    comm: String,
    state: String,
    ppid: u32,
    pgid: u32,
    sid: u32,
    tty_nr: usize,
    tty_pgrp: usize,
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
        tty_pgrp: usize,
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
            pid: columns[0].parse::<u32>()?,
            comm: columns[1].to_string(),
            state: columns[2].to_string(),
            ppid: columns[3].parse::<u32>()?,
            pgid: columns[4].parse::<u32>()?,
            sid: columns[5].parse::<u32>()?,
            tty_nr: columns[6].parse::<usize>()?,
            tty_pgrp: columns[7].parse::<usize>()?,
            flags: columns[8].parse::<usize>()?,
            min_flt: columns[9].parse::<usize>()?,
            cmin_flt: columns[10].parse::<usize>()?,
            maj_flt: columns[11].parse::<usize>()?,
            cmaj_flt: columns[12].parse::<usize>()?,
            utime: columns[13].parse::<usize>()?,
            stime: columns[14].parse::<usize>()?,
            cutime: columns[15].parse::<usize>()?,
            cstime: columns[16].parse::<usize>()?,
            priority: columns[17].parse::<usize>()?,
            nice: columns[18].parse::<usize>()?,
            num_threads: columns[19].parse::<usize>()?,
            it_real_value: columns[20].parse::<usize>()?,
            start_time: columns[21].parse::<usize>()?,
            vsize: columns[22].parse::<usize>()?,
            rss: columns[23].parse::<usize>()?,
            rlim: columns[24].parse::<usize>()?,
            start_code: columns[25].parse::<usize>()?,
            end_code: columns[26].parse::<usize>()?,
            start_stack: columns[27].parse::<usize>()?,
            kstkesp: columns[28].parse::<usize>()?,
            kstkeip: columns[29].parse::<usize>()?,
            pendingsig: columns[30].parse::<usize>()?,
            block_sig: columns[31].parse::<usize>()?,
            sigign: columns[32].parse::<usize>()?,
            sigcatch: columns[33].parse::<usize>()?,
            wchan: columns[34].parse::<usize>()?,
            nswap: columns[35].parse::<usize>()?,
            cnswap: columns[36].parse::<usize>()?,
            exit_signal: columns[37].parse::<usize>()?,
            task_cpu: columns[38].parse::<usize>()?,
            task_rt_priority: columns[39].parse::<usize>()?,
            task_policy: columns[40].parse::<usize>()?,
            blio_ticks: columns[41].parse::<usize>()?,
            gtime: columns[42].parse::<usize>()?,
            cgtime: columns[43].parse::<usize>()?,
            start_data: columns[44].parse::<usize>()?,
            end_data: columns[45].parse::<usize>()?,
            start_brk: columns[46].parse::<usize>()?,
            arg_start: columns[47].parse::<usize>()?,
            arg_end: columns[48].parse::<usize>()?,
            env_start: columns[49].parse::<usize>()?,
            env_end: columns[50].parse::<usize>()?,
            exit_code: columns[51].parse::<usize>()?,
        })
    }
}

pub fn stat_of(pid: u32) -> Result<Stat> {
    let content = std::fs::read_to_string(path!(pid, "stat"))?;
    Stat::from_str(content.trim())
}

pub struct Statm {
    total: usize,
    portions: usize,
    shared: usize,
    code: usize,
    data: usize,
    library: usize,
    dirty: usize,
}

impl Statm {
    getter_gen! {
        total: usize,
        portions: usize,
        shared: usize,
        code: usize,
        data: usize,
        library: usize,
        dirty: usize
    }
}

impl FromStr for Statm {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 7 {
            return Err(Error::BadFormat);
        }

        let total = columns[0].parse::<usize>()?;
        let portions = columns[1].parse::<usize>()?;
        let shared = columns[2].parse::<usize>()?;
        let code = columns[3].parse::<usize>()?;
        let data = columns[4].parse::<usize>()?;
        let library = columns[5].parse::<usize>()?;
        let dirty = columns[6].parse::<usize>()?;

        Ok(Statm {
            total,
            portions,
            shared,
            code,
            data,
            library,
            dirty,
        })
    }
}

pub fn statm_of(pid: u32) -> Result<Statm> {
    let content = std::fs::read_to_string(path!(pid, "statm"))?;
    Statm::from_str(content.trim())
}

pub fn status_of(pid: u32) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(path!(pid, "status"))?;
    let mut map = HashMap::new();
    for line in content.trim().lines() {
        let kv: Vec<&str> = line.split(':').collect();
        if kv.len() != 2 {
            return Err(Error::BadFormat);
        }
        let k = kv[0].to_string();
        let v = kv[1].to_string();
        map.insert(k, v);
    }
    Ok(map)
}
