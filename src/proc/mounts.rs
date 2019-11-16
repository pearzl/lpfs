use crate::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// represent an entry in /proc/mounts
#[derive(Debug)]
pub struct Mount {
    device: String,
    mount_point: PathBuf,
    fs_type: String,
    mode: String,
    dummy1: String,
    dummy2: String,
}

impl Mount {
    getter_gen! {
        device: String,
        mount_point: PathBuf,
        fs_type: String,
        mode: String,
        dummy1: String,
        dummy2: String
    }
}

impl FromStr for Mount {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }
        let device = columns[0].to_string();
        let mount_point = Path::new(columns[1]).to_path_buf();
        let fs_type = columns[2].to_string();
        let mode = columns[3].to_string();
        let dummy1 = columns[4].to_string();
        let dummy2 = columns[5].to_string();
        Ok(Mount {
            device,
            mount_point,
            fs_type,
            mode,
            dummy1,
            dummy2,
        })
    }
}

#[inline(always)]
fn to_mounts(line: &str) -> Result<Mount> {
    Mount::from_str(line)
}

default_list! {
    mounts, "/proc/mounts", Mount, to_mounts
}
