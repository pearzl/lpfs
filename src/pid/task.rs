 
type Result<T> = std::result::Result<T, crate::ProcErr>;

/// Return a Vector contains thread id whose contained in current process.
pub fn task_self() -> Result<Vec<u32>> {
    let dir_entries = std::fs::read_dir("/proc/self/task/")?;
    let mut ret = vec![];

    for task_dir in dir_entries {
        let thread_id_str = task_dir?.file_name();
        let thread_id = thread_id_str
            .to_str()
            .ok_or(bfe!("contains non-unicode chatacter".to_string()))?
            .parse::<u32>()?;
        ret.push(thread_id);
    }

    Ok(ret)
}

/// Return a Vector contains thread id whose contained in specified process.
pub fn task_of(pid: u32) -> Result<Vec<u32>> {
    let dir_entries = std::fs::read_dir(format!("/proc/{}/task/", pid))?;
    let mut ret = vec![];

    for task_dir in dir_entries {
        let thread_id_str = task_dir?.file_name();
        let thread_id = thread_id_str
            .to_str()
            .ok_or(bfe!("contains non-unicode chatacter".to_string()))?
            .parse::<u32>()?;
        ret.push(thread_id);
    }

    Ok(ret)
}