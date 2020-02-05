// /proc/[pid]/cwd
// This is a symbolic link to the current working directory of
// the process.  To find out the current working directory of
// process 20, for instance, you can do this:
//
//     $ cd /proc/20/cwd; /bin/pwd
//
// Note that the pwd command is often a shell built-in, and might
// not work properly.  In bash(1), you may use pwd -P.
//
// In a multithreaded process, the contents of this symbolic link
// are not available if the main thread has already terminated
// (typically by calling pthread_exit(3)).
//
// Permission to dereference or read (readlink(2)) this symbolic
// link is governed by a ptrace access mode
// PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

use std::path::PathBuf;
define_struct! {
    pub struct Cwd(PathBuf);
}

pub fn cwd_of(pid: u32) -> Result<Cwd, crate::ProcErr> {
    let path = std::fs::read_link(format!("/proc/{}/cwd", pid))?;
    Ok(Cwd(path))
}

pub fn cwd_self() -> Result<Cwd, crate::ProcErr> {
    let path = std::fs::read_link("/proc/self/cwd")?;
    Ok(Cwd(path))
}

pub fn cwd_of_of(pid: u32, tid: u32) -> Result<Cwd, crate::ProcErr> {
    let path = std::fs::read_link(format!("/proc/{}/task/{}/cwd", pid, tid))?;
    Ok(Cwd(path))
}

pub fn cwd_self_of(tid: u32) -> Result<Cwd, crate::ProcErr> {
    let path = std::fs::read_link(format!("/proc/self/task/{}/cwd", tid))?;
    Ok(Cwd(path))
}

pub fn cwd_self_self() -> Result<Cwd, crate::ProcErr> {
    let path = std::fs::read_link("/proc/thread-self/cwd")?;
    Ok(Cwd(path))
}
