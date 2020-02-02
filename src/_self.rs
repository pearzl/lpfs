//! Contains functions about /proc/self.
//!
//! There is no item named self in this crate, because self is a key word in rust, so we use _self instead.

use std::path::PathBuf;

type Result<T> = std::result::Result<T, crate::ProcErr>;

/// Return an PathBuf which /proc/self point to.
///
/// The returnd value should be same as `ls -l /proc/self`.
///
/// There is not self() function in this crate, because self is a key word in rust.
pub fn _self() -> Result<PathBuf> {
    Ok(std::fs::read_link("/proc/self")?)
}

/// Return the process ID (pid) of calling process.
///
/// This should have the same output of [`getpid()`](http://man7.org/linux/man-pages/man2/getpid.2.html]),
/// but it is a safe method.
///
/// *Note: std::process::id() have same behavior.*
pub fn self_pid() -> Result<u32> {
    let path = std::fs::read_link("/proc/self")?;
    let pid_str = path.display().to_string();
    let pid = pid_str.parse::<u32>()?;
    Ok(pid)
}
