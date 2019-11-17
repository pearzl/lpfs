use crate::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

/// represent the content of /proc/[pid]/status
///
/// Currently, this struct has some common fileds as listed in
/// [here](https://linux.die.net/man/5/proc).
/// Other entries can be accessed by `entries` filed which is a HashMap<String, String>.
#[derive(Debug)]
pub struct Status {
    name: String,
    state: State,
    tgid: i32,
    pid: i32,
    ppid: i32,
    tracer_pid: i32,
    uid: String,
    gid: String,
    fd_size: usize,
    groups: String,
    vm_peak: usize,
    vm_size: usize,
    vm_lck: usize,
    vm_hwm: usize,
    vm_rss: usize,
    vm_data: usize,
    vm_stk: usize,
    vm_exe: usize,
    vm_lib: usize,
    vm_pte: usize,
    threads: usize,
    sig_q: String,
    sig_pnd: String,
    shd_pnd: String,
    sig_blk: String,
    sig_ign: String,
    sig_cgt: String,
    cap_inh: String,
    cap_prm: String,
    cap_eff: String,
    cap_bnd: String,
    cpus_allowed: String,
    cpus_allowed_list: String,
    mems_allowed: String,
    mems_allowed_list: String,
    voluntary_ctxt_switches: usize,
    nonvoluntary_context_switches: usize,
    entries: HashMap<String, String>,
}

impl Status {
    getter_gen! {
        name: String,
        state: State,
        tgid: i32,
        pid: i32,
        ppid: i32,
        tracer_pid: i32,
        uid: String,
        gid: String,
        fd_size: usize,
        groups: String,
        vm_peak: usize,
        vm_size: usize,
        vm_lck: usize,
        vm_hwm: usize,
        vm_rss: usize,
        vm_data: usize,
        vm_stk: usize,
        vm_exe: usize,
        vm_lib: usize,
        vm_pte: usize,
        threads: usize,
        sig_q: String,
        sig_pnd: String,
        shd_pnd: String,
        sig_blk: String,
        sig_ign: String,
        sig_cgt: String,
        cap_inh: String,
        cap_prm: String,
        cap_eff: String,
        cap_bnd: String,
        cpus_allowed: String,
        cpus_allowed_list: String,
        mems_allowed: String,
        mems_allowed_list: String,
        voluntary_ctxt_switches: usize,
        nonvoluntary_context_switches: usize,
        entries: HashMap<String, String>
    }
}

#[derive(Debug)]
pub enum State {
    R,
    S,
    D,
    TS,
    TT,
    Z,
    X,
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "R (running)" => Ok(State::R),
            "S (sleeping)" => Ok(State::S),
            "D (disk sleep)" => Ok(State::D),
            "T (stopped)" => Ok(State::TS),
            "T (tracing stop" => Ok(State::TT),
            "Z (zombie)" => Ok(State::Z),
            "X (dead)" => Ok(State::X),
            _ => Err(Error::BadFormat),
        }
    }
}

pub fn status_of(pid: u32) -> Result<Status> {
    let content = std::fs::read_to_string(pid_path!(pid, "status"))?;
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
    Ok(Status {
        name: map.remove("Name").ok_or(Error::BadFormat)?,
        state: map
            .remove("State")
            .ok_or(Error::BadFormat)
            .and_then(|s| State::from_str(&s))?,
        tgid: map
            .remove("Tgid")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<i32>()?))?,
        pid: map
            .remove("Pid")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<i32>()?))?,
        ppid: map
            .remove("PPid")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<i32>()?))?,
        tracer_pid: map
            .remove("TracerPid")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<i32>()?))?,
        uid: map.remove("Uid").ok_or(Error::BadFormat)?,
        gid: map.remove("Gid").ok_or(Error::BadFormat)?,
        fd_size: map
            .remove("FDSize")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        groups: map.remove("Groups").ok_or(Error::BadFormat)?,
        vm_peak: map
            .remove("VmPeak")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_size: map
            .remove("VmSize")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_lck: map
            .remove("VmLck")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_hwm: map
            .remove("VmHWM")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_rss: map
            .remove("VmRSS")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_data: map
            .remove("VmData")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_stk: map
            .remove("VmStk")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_exe: map
            .remove("VmExe")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_lib: map
            .remove("VmLib")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        vm_pte: map
            .remove("VmPTE")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        threads: map
            .remove("Threads")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        sig_q: map.remove("SigQ").ok_or(Error::BadFormat)?,
        sig_pnd: map.remove("SigPnd").ok_or(Error::BadFormat)?,
        shd_pnd: map.remove("ShdPnd").ok_or(Error::BadFormat)?,
        sig_blk: map.remove("SigBlk").ok_or(Error::BadFormat)?,
        sig_ign: map.remove("SigIng").ok_or(Error::BadFormat)?,
        sig_cgt: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cap_inh: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cap_prm: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cap_eff: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cap_bnd: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cpus_allowed: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        cpus_allowed_list: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        mems_allowed: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        mems_allowed_list: map.remove("SigCgt").ok_or(Error::BadFormat)?,
        voluntary_ctxt_switches: map
            .remove("SigCgt")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        nonvoluntary_context_switches: map
            .remove("SigCgt")
            .ok_or(Error::BadFormat)
            .and_then(|s| Ok(s.parse::<usize>()?))?,
        entries: map,
    })
}
