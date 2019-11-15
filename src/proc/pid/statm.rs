use crate::{Error, Result};
use std::str::FromStr;

pub struct Statm {
    total: usize,
    portions: usize,
    shared: usize,
    code: usize,
    data: usize,
    library: usize,
    dirty: usize,
}

impl Statm {
    getter_gen! {
        total: usize,
        portions: usize,
        shared: usize,
        code: usize,
        data: usize,
        library: usize,
        dirty: usize
    }
}

impl FromStr for Statm {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 7 {
            return Err(Error::BadFormat);
        }

        let total = columns[0].parse::<usize>()?;
        let portions = columns[1].parse::<usize>()?;
        let shared = columns[2].parse::<usize>()?;
        let code = columns[3].parse::<usize>()?;
        let data = columns[4].parse::<usize>()?;
        let library = columns[5].parse::<usize>()?;
        let dirty = columns[6].parse::<usize>()?;

        Ok(Statm {
            total,
            portions,
            shared,
            code,
            data,
            library,
            dirty,
        })
    }
}

pub fn statm_of(pid: u32) -> Result<Statm> {
    let content = std::fs::read_to_string(pid_path!(pid, "statm"))?;
    Statm::from_str(content.trim())
}
