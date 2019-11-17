use super::stat::Stat;
use crate::Result;
use std::str::FromStr;

macro_rules! tid_path {
    ($pid: expr, $tid: expr, $fname: expr) => {
        format!("/proc/{}/task/{}/{}", $pid, $tid, $fname)
    };
}

pub fn stat_oft(pid: i32, tid: i32) -> Result<Stat> {
    let content = std::fs::read_to_string(tid_path!(pid, tid, "stat"))?;
    Stat::from_str(content.trim())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stat_oft() {
        println!("{:#?}", stat_oft(1, 1).unwrap());
    }
}
