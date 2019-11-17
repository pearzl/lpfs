use crate::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Map {
    addr_begin: usize,
    addr_end: usize,
    permission: String,
    offset: usize,
    device_major: usize,
    device_minor: usize,
    inode: usize,
    path: PathBuf,
}

impl Map {
    getter_gen! {
        addr_begin: usize,
        addr_end:   usize,
        permission: String,
        offset:     usize,
        device_major: usize,
        device_minor: usize,
        inode: usize,
        path: PathBuf
    }

    pub fn addr(&self) -> (usize, usize) {
        (self.addr_begin, self.addr_end)
    }

    pub fn device(&self) -> String {
        format!("{}:{}", self.device_major, self.device_minor)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }

        let addr: Vec<&str> = columns[0].split('-').collect();
        if addr.len() != 2 {
            return Err(Error::BadFormat);
        }
        let addr_begin = usize::from_str_radix(addr[0], 16)?;
        let addr_end = usize::from_str_radix(addr[1], 16)?;

        let permission = columns[1].to_string();

        let offset = usize::from_str_radix(columns[2], 16)?;

        let device: Vec<&str> = columns[3].split(':').collect();
        if device.len() != 2 {
            return Err(Error::BadFormat);
        }
        let device_major = usize::from_str_radix(device[0], 16)?;
        let device_minor = usize::from_str_radix(device[1], 16)?;

        let inode = columns[4].parse::<usize>()?;

        let path = PathBuf::from(columns[5]);

        Ok(Map {
            addr_begin,
            addr_end,
            permission,
            offset,
            device_major,
            device_minor,
            inode,
            path,
        })
    }
}

pub fn maps_of(pid: u32) -> Result<Vec<Map>> {
    let content = std::fs::read_to_string(pid_path!(pid, "maps"))?;
    let mut ret = vec![];
    for line in content.trim().lines() {
        ret.push(Map::from_str(line)?)
    }
    Ok(ret)
}
