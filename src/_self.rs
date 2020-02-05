//! Contains functions about /proc/self and /proc/thread/self;
//!
//! There is no item named self in this crate, because self is a key word in rust, so we use _self instead.

use std::path::PathBuf;

type Result<T> = std::result::Result<T, crate::ProcErr>;

/// Return an PathBuf which /proc/self point to.
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
/// **Note: std::process::id() have same behavior.**
pub fn self_pid() -> Result<u32> {
    let path = std::fs::read_link("/proc/self")?;
    let pid_str = path.display().to_string();
    let pid = pid_str.parse::<u32>()?;
    Ok(pid)
}

/// Return an PathBuf which /proc/thread-self point to.
///
/// **Note: There is no thread-self directroy on many linux.**
pub fn thread_self() -> Result<PathBuf> {
    Ok(std::fs::read_link("/proc/thread-self")?)
}

/// Return the thread ID (tid) of calling thread.
///
/// ****
pub fn self_tid() -> Result<u32> {
    let path = std::fs::read_link("/proc/thread-self")?;
    let path_str = path.display().to_string();
    let tid_str_iter = path_str.split('/');
    let tid_str = tid_str_iter.last().unwrap_or_default();
    let tid = tid_str.parse::<u32>()?;
    Ok(tid)
}
