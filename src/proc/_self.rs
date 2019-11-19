use crate::{Error, Result};
use std::path::PathBuf;

/// return an PathBuf which /proc/self point to.
///
/// the returnd value should be same as `ls -l /proc/self`
///
/// there is not self() function in this crate, because self is a key word in rust.
pub fn _self() -> Result<PathBuf> {
    Ok(std::fs::read_link("/proc/self")?)
}

/// return the process ID (pid) of calling process.
pub fn self_pid() -> Result<u32> {
    let path = std::fs::read_link("/proc/self")?;
    let pid_str = path.to_str().ok_or(Error::BadFormat)?;
    let pid = pid_str.parse::<u32>()?;
    Ok(pid)
}

pub fn threads_of(pid: u32) -> Result<Vec<u32>> {
    let path = format!("/proc/{}/task/", pid);
    let dir_entries = std::fs::read_dir(path)?;
    let mut ret = vec![];

    for task_dir in dir_entries {
        let thread_id_str = task_dir?.file_name();
        let thread_id = thread_id_str
            .to_str()
            .ok_or(Error::BadFormat)?
            .parse::<u32>()?;
        ret.push(thread_id);
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_self() {
        println!("{:?}", _self().unwrap());
        println!("{:?}", self_pid().unwrap());
        assert_eq!(vec![1u32], threads_of(1).unwrap());
    }
}
