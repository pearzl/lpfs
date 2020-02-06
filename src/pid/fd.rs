// This is a subdirectory containing one entry for each file
// which the process has open, named by its file descriptor, and
// which is a symbolic link to the actual file.  Thus, 0 is stan‐
// dard input, 1 standard output, 2 standard error, and so on.
//
// For file descriptors for pipes and sockets, the entries will
// be symbolic links whose content is the file type with the
// inode.  A readlink(2) call on this file returns a string in
// the format:
//
//     type:[inode]
//
// For example, socket:[2248868] will be a socket and its inode
// is 2248868.  For sockets, that inode can be used to find more
// information in one of the files under /proc/net/.
//
// For file descriptors that have no corresponding inode (e.g.,
// file descriptors produced by bpf(2), epoll_create(2),
// eventfd(2), inotify_init(2), perf_event_open(2), signalfd(2),
// timerfd_create(2), and userfaultfd(2)), the entry will be a
// symbolic link with contents of the form
//
//     anon_inode:<file-type>
//
// In many cases (but not all), the file-type is surrounded by
// square brackets.
//
// For example, an epoll file descriptor will have a symbolic
// link whose content is the string anon_inode:[eventpoll].
//
// In a multithreaded process, the contents of this directory are
// not available if the main thread has already terminated (typi‐
// cally by calling pthread_exit(3)).
//
// Programs that take a filename as a command-line argument, but
// don't take input from standard input if no argument is sup‐
// plied, and programs that write to a file named as a command-
// line argument, but don't send their output to standard output
// if no argument is supplied, can nevertheless be made to use
// standard input or standard output by using /proc/[pid]/fd
// files as command-line arguments.  For example, assuming that
// -i is the flag designating an input file and -o is the flag
// designating an output file:
//
//     $ foobar -i /proc/self/fd/0 -o /proc/self/fd/1 ...
//
// and you have a working filter.
//
// /proc/self/fd/N is approximately the same as /dev/fd/N in some
// UNIX and UNIX-like systems.  Most Linux MAKEDEV scripts sym‐
// bolically link /dev/fd to /proc/self/fd, in fact.
//
// Most systems provide symbolic links /dev/stdin, /dev/stdout,
// and /dev/stderr, which respectively link to the files 0, 1,
// and 2 in /proc/self/fd.  Thus the example command above could
// be written as:
//
//     $ foobar -i /dev/stdin -o /dev/stdout ...
//
// Permission to dereference or read (readlink(2)) the symbolic
// links in this directory is governed by a ptrace access mode
// PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
//
// Note that for file descriptors referring to inodes (pipes and
// sockets, see above), those inodes still have permission bits
// and ownership information distinct from those of the
// /proc/[pid]/fd entry, and that the owner may differ from the
// user and group IDs of the process.  An unprivileged process
// may lack permissions to open them, as in this example:
//
//     $ echo test | sudo -u nobody cat
//     test
//     $ echo test | sudo -u nobody cat /proc/self/fd/0
//     cat: /proc/self/fd/0: Permission denied
//
// File descriptor 0 refers to the pipe created by the shell and
// owned by that shell's user, which is not nobody, so cat does
// not have permission to create a new file descriptor to read
// from that inode, even though it can still read from its exist‐
// ing file descriptor 0.
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html
//
// fd — A directory containing all of the file descriptors for a particular process. These are given in numbered links:
// total 0
// lrwx------    1 root     root           64 May  8 11:31 0 -> /dev/null
// lrwx------    1 root     root           64 May  8 11:31 1 -> /dev/null
// lrwx------    1 root     root           64 May  8 11:31 2 -> /dev/null
// lrwx------    1 root     root           64 May  8 11:31 3 -> /dev/ptmx
// lrwx------    1 root     root           64 May  8 11:31 4 -> socket:[7774817]
// lrwx------    1 root     root           64 May  8 11:31 5 -> /dev/ptmx
// lrwx------    1 root     root           64 May  8 11:31 6 -> socket:[7774829]
// lrwx------    1 root     root           64 May  8 11:31 7 -> /dev/ptmx
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-directories#s2-proc-processdirs

use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct Fd(PathBuf);

impl Fd {
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }
}

macro_rules! fd_impl {
    ($path: expr) => {
        let dir_entries = std::fs::read_dir($path)?;
        let mut ret = vec![];

        for task_dir in dir_entries {
            let fd = task_dir?.path();
            ret.push(Fd(fd));
        }

        Ok(ret)
    };
}

pub fn fd_of(pid: u32) -> Result<Vec<Fd>, crate::ProcErr> {
    fd_impl! {format!("/proc/{}/fd", pid)}
}

pub fn fd_self() -> Result<Vec<Fd>, crate::ProcErr> {
    fd_impl! {"/proc/self/fd"}
}

pub fn fd_of_of(pid: u32, tid: u32) -> Result<Vec<Fd>, crate::ProcErr> {
    fd_impl! {format!("/proc/{}/task/{}/fd", pid, tid)}
}

pub fn fd_self_of(tid: u32) -> Result<Vec<Fd>, crate::ProcErr> {
    fd_impl! {format!("/proc/self/task/{}/fd", tid)}
}

pub fn fd_self_self() -> Result<Vec<Fd>, crate::ProcErr> {
    fd_impl! {"/proc/thread-self/fd"}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fd() {
        let ret = fd_self().unwrap();
        assert!(ret.contains(&Fd(std::path::PathBuf::from("/proc/self/fd/0"))));
        assert!(ret.contains(&Fd(std::path::PathBuf::from("/proc/self/fd/1"))));
        assert!(ret.contains(&Fd(std::path::PathBuf::from("/proc/self/fd/2"))));
    }
}
