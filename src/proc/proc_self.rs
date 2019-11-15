use crate::Result;
use std::path::PathBuf;

/// return an PathBuf which /proc/self point to.
/// 
/// there is not self() function in this crate, because self is a key word in rust.
pub fn proc_self() -> Result<PathBuf> {
    Ok(std::fs::read_link("/proc/self")?)
}