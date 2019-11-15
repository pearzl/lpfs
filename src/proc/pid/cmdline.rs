use crate::Result;

pub fn cmdline_of(pid: u32) -> Result<String> {
    Ok(std::fs::read_to_string(pid_path!(pid, "cmdline"))?)
}