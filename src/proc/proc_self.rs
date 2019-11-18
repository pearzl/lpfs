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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_self() {
        println!("{:?}", _self().unwrap());
        println!("{:?}", self_pid().unwrap())
    }
}
