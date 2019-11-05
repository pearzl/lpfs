use crate::proc::{Error, Result};
use std::str::FromStr;

/// returned by [`loadavg()`](fn.loadavg.html)
#[derive(Debug)]
pub struct LoadAvg {
    one: f32,
    five: f32,
    fifteen: f32,
    cur_num: usize,
    total_num: usize,
    last_pid: usize,
}

impl LoadAvg {
    getter_gen! {
        one: f32,
        five: f32,
        fifteen: f32,
        cur_num: usize,
        total_num: usize,
        last_pid: usize
    }
}

impl FromStr for LoadAvg {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut column_iter = value.trim().split_ascii_whitespace();
        let one = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let five = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let fifteen = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let pnum: Vec<&str> = column_iter
            .next()
            .ok_or(Error::BadFormat)?
            .split('/')
            .collect();
        let cur_num = pnum.get(0).ok_or(Error::BadFormat)?.parse::<usize>()?;
        let total_num = pnum.get(1).ok_or(Error::BadFormat)?.parse::<usize>()?;
        let last_pid = column_iter
            .next()
            .ok_or(Error::BadFormat)?
            .parse::<usize>()?;
        Ok(LoadAvg {
            one,
            five,
            fifteen,
            cur_num,
            total_num,
            last_pid,
        })
    }
}

/// The content is parsed to a LoadAvg.
/// ```
/// use linux_proc::*;
/// fn main() {
///     let la = loadavg().unwrap();
///     println!("one minute load: {}", la.one());
///     println!("five minutes load: {}", la.five());
///     println!("fifteen minutes laod: {}", la.fifteen());
///     println!("current process number: {}", la.cur_num());
///     println!("total process number: {}", la.total_num());
///     println!("last process id: {}", la.last_pid());
/// }
/// ```
pub fn loadavg() -> Result<LoadAvg> {
    let content = std::fs::read_to_string("/proc/loadavg")?;
    LoadAvg::from_str(&content)
}
