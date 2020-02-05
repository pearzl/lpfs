// /proc/[pid]/task
//
// /proc/[pid]/task (since Linux 2.6.0-test6)
// This  is	a directory that contains one subdirectory for each thread in the process.  The name of each subdirectory is the numerical
// thread ID ([tid]) of the thread (see gettid(2)).	Within each of these subdirectories, there is a set of files with the  same  names
// and  contents  as  under	the  /proc/[pid] directories.  For attributes that are shared by all threads, the contents for each of the
// files under the task/[tid] subdirectories will be the same as in the corresponding file in the parent /proc/[pid]  directory  (e.g.,
// in  a  multithreaded  process,  all  of  the task/[tid]/cwd files will have the same value as the /proc/[pid]/cwd file in the parent
// directory, since all of the threads in a process share a working directory).  For attributes that are distinct for each thread,  the
// corresponding  files  under task/[tid] may have different values (e.g., various fields in each of the task/[tid]/status files may be
// different for each thread).
//
// In a multithreaded process, the contents of the /proc/[pid]/task directory are not available if the main thread has  already  termi-
// nated (typically by calling pthread_exit(3)).
//
// -- https://www.unix.com/man-page/suse/5/proc/


/// Return a Vector contains thread id whose contained in current process.
pub fn task_self() -> Result<Vec<u32>, crate::ProcErr> {
    let dir_entries = std::fs::read_dir("/proc/self/task/")?;
    let mut ret = vec![];

    for task_dir in dir_entries {
        let thread_id_str = task_dir?.file_name();
        let thread_id = thread_id_str
            .to_str()
            .ok_or_else(|| "contains non-unicode chatacter")?
            .parse::<u32>()?;
        ret.push(thread_id);
    }

    Ok(ret)
}

/// Return a Vector contains thread id whose contained in specified process.
pub fn task_of(pid: u32) -> Result<Vec<u32>, crate::ProcErr> {
    let dir_entries = std::fs::read_dir(format!("/proc/{}/task/", pid))?;
    let mut ret = vec![];

    for task_dir in dir_entries {
        let thread_id_str = task_dir?.file_name();
        let thread_id = thread_id_str
            .to_str()
            .ok_or_else(|| "contains non-unicode chatacter")?
            .parse::<u32>()?;
        ret.push(thread_id);
    }

    Ok(ret)
}
