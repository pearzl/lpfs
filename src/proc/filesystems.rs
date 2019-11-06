use crate::proc::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct FileSystem {
    nodev: bool,
    fs_type: String,
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split('\t').collect();
        if columns.len() < 2 {
            return Err(Error::BadFormat);
        }
        Ok(FileSystem {
            nodev: columns[0] != "nodev",
            fs_type: columns[1].trim().to_string(),
        })
    }
}

#[inline(always)]
fn to_filesystems(line: &str) -> Result<FileSystem> {
    FileSystem::from_str(line)
}

default_list! {
    filesystems, "/proc/filesystems", FileSystem, to_filesystems
}
