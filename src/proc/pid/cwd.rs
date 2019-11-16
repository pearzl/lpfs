use crate::Result;
use std::path::PathBuf;

pub fn cwd_of(pid: u32) -> Result<PathBuf> {
    Ok(std::fs::read_link(pid_path!(pid, "cwd"))?)
}
