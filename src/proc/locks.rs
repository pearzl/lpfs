use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/locks
///
/// There are two method to access the Lock:
/// 1. by filed name, these method has the same name as the filed name.
/// 2. by column index, correct index if from 0 to 7, wrong index make a panic.
///
/// Note: index by column always return `String` type.
/// However filed name have different type to return and are not group by column.
///
/// Note: access last column by filed is an Option, None stand for EOF.
/// The last column always exist.
/// ```
/// use linux_proc::proc::locks::*;
/// fn main() {
///     for lock in locks().unwrap() {
///         assert_eq!(lock.class(), &lock.column(1));
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Lock {
    id: usize,
    class: String,
    mode: String,
    rw: String,
    pid: usize,
    major: usize,
    minor: usize,
    inode: usize,
    start: usize,
    end: Option<usize>,
}

impl Lock {
    getter_gen! {
        id: usize,
        class: String,
        mode: String,
        rw: String,
        pid: usize,
        major: usize,
        minor: usize,
        inode: usize,
        start: usize,
        end: Option<usize>
    }

    #[allow(clippy::useless_format)]
    pub fn column(&self, index: usize) -> String {
        match index {
            0 => format!("{}", self.id),
            1 => format!("{}", self.class),
            2 => format!("{}", self.mode),
            3 => format!("{}", self.rw),
            4 => format!("{}", self.pid),
            5 => format!("{:02x}:{:02x}:{}", self.major, self.minor, self.inode),
            6 => format!("{}", self.start),
            7 => {
                if let Some(e) = self.end {
                    format!("{}", e)
                } else {
                    format!("{}", "EOF")
                }
            }
            _ => panic!("out of range"),
        }
    }
}

impl FromStr for Lock {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.trim().split_ascii_whitespace().collect();
        if columns.len() != 8 {
            return Err(Error::BadFormat);
        }

        let id = columns[0].trim_end_matches(':').parse::<usize>()?;
        let class = columns[1].to_string();
        let mode = columns[2].to_string();
        let rw = columns[3].to_string();
        let pid = columns[4].parse::<usize>()?;
        let file: Vec<&str> = columns[5].split(':').collect();
        if file.len() != 3 {
            return Err(Error::BadFormat);
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
            id,
            class,
            mode,
            rw,
            pid,
            major,
            minor,
            inode,
            start,
            end,
        })
    }
}

#[inline(always)]
fn to_locks(line: &str) -> Result<Lock> {
    Lock::from_str(line)
}

default_list! {
    locks, "/proc/locks", Lock, to_locks
}
