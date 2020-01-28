//! > <pre>
//! > 5.2.17.  /proc/locks
//! > This file displays the files currently locked by the kernel. The contents of this file contain internal kernel debugging data and can vary tremendously, depending on the use of the system. A sample /proc/locks file for a lightly loaded system looks similar to the following:
//! > 1: POSIX  ADVISORY  WRITE 3568 fd:00:2531452 0 EOF
//! > 2: FLOCK  ADVISORY  WRITE 3517 fd:00:2531448 0 EOF
//! > 3: POSIX  ADVISORY  WRITE 3452 fd:00:2531442 0 EOF
//! > 4: POSIX  ADVISORY  WRITE 3443 fd:00:2531440 0 EOF
//! > 5: POSIX  ADVISORY  WRITE 3326 fd:00:2531430 0 EOF
//! > 6: POSIX  ADVISORY  WRITE 3175 fd:00:2531425 0 EOF
//! > 7: POSIX  ADVISORY  WRITE 3056 fd:00:2548663 0 EOF
//! > Each lock has its own line which starts with a unique number. The second column refers to the class of lock used, with FLOCK signifying the older-style UNIX file locks from a flock system call and POSIX representing the newer POSIX locks from the lockf system call.
//! > The third column can have two values: ADVISORY or MANDATORY. ADVISORY means that the lock does not prevent other people from accessing the data; it only prevents other attempts to lock it. MANDATORY means that no other access to the data is permitted while the lock is held. The fourth column reveals whether the lock is allowing the holder READ or WRITE access to the file. The fifth column shows the ID of the process holding the lock. The sixth column shows the ID of the file being locked, in the format of MAJOR-DEVICE:MINOR-DEVICE:INODE-NUMBER . The seventh and eighth column shows the start and end of the file's locked region.
//! </pre>
//! 
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-kmsg

define_struct! {
    pub struct Lock{
        id: usize,
        class: String,
        mode: String,
        rw: String,
        pid: usize,
        major: usize,
        minor: usize,
        inode: usize,
        start: usize,
        /// None means "EOF"
        end: Option<usize>,
    }
}

use std::str::FromStr;
impl FromStr for Lock {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Lock, Self::Err> {
        let columns: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        if columns.len() != 8 {
            return Err("lock need 8 fields".into());
        }

        let id = columns[0].trim_end_matches(':').parse::<usize>()?;
        let class = columns[1].to_string();
        let mode = columns[2].to_string();
        let rw = columns[3].to_string();
        let pid = columns[4].parse::<usize>()?;
        let file: Vec<&str> = columns[5].split(':').collect();
        if file.len() != 3 {
            return Err("file need three items".into());
        }
        let major = usize::from_str_radix(file[0], 16)?;
        let minor = usize::from_str_radix(file[1], 16)?;
        let inode = usize::from_str_radix(file[2], 10)?;
        let start = columns[6].parse::<usize>()?;
        let end = if "EOF" == columns[7] {
            None
        } else {
            Some(columns[7].parse::<usize>()?)
        };

        Ok(Lock {
            id, class, mode, rw, pid,
            major, minor, inode, start, end,
        })
    }
}

list_impl! {
    locks, "/proc/locks", Lock, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lock() {
        let source = "1: FLOCK  ADVISORY  WRITE 16861 fc:01:1313050 0 EOF";
        let correct = Lock {
            id: 1, 
            class: "FLOCK".to_string(), 
            mode: "ADVISORY".to_string(),
            rw: "WRITE".to_string(),
            pid: 16861,
            major: 0xfc,
            minor: 1,
            inode: 1313050,
            start: 0,
            end: None
        };
        assert_eq!(correct, source.parse::<Lock>().unwrap());
    }
}