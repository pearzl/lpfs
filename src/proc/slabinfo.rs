use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/slabinfo
#[derive(Debug)]
pub struct SlabInfo {
    name: String,
    active_objs: usize,
    num_objs: usize,
    objsize: usize,
    objperslab: usize,
    pagesperslab: usize,
    limit: usize,
    batchcount: usize,
    sharedfactor: usize,
    active_slabs: usize,
    num_slabs: usize,
    sharedavail: usize,
}

impl SlabInfo {
    getter_gen! {
        name: String,
        active_objs: usize,
        num_objs: usize,
        objsize: usize,
        objperslab: usize,
        pagesperslab: usize,
        limit: usize,
        batchcount: usize,
        sharedfactor: usize,
        active_slabs: usize,
        num_slabs: usize,
        sharedavail: usize
    }
}

impl FromStr for SlabInfo {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 16 {
            return Err(Error::BadFormat);
        }

        let name = columns[0].to_string();
        let active_objs = columns[1].parse::<usize>()?;
        let num_objs = columns[2].parse::<usize>()?;
        let objsize = columns[3].parse::<usize>()?;
        let objperslab = columns[4].parse::<usize>()?;
        let pagesperslab = columns[5].parse::<usize>()?;
        let limit = columns[8].parse::<usize>()?;
        let batchcount = columns[9].parse::<usize>()?;
        let sharedfactor = columns[10].parse::<usize>()?;
        let active_slabs = columns[13].parse::<usize>()?;
        let num_slabs = columns[14].parse::<usize>()?;
        let sharedavail = columns[15].parse::<usize>()?;

        Ok(SlabInfo {
            name,
            active_objs,
            num_objs,
            objsize,
            objperslab,
            pagesperslab,
            limit,
            batchcount,
            sharedfactor,
            active_slabs,
            num_slabs,
            sharedavail,
        })
    }
}

#[inline(always)]
fn to_slabinfo(line: &str) -> Result<SlabInfo> {
    SlabInfo::from_str(line)
}

default_list! {
    slabinfo, "/proc/slabinfo", SlabInfo, to_slabinfo, '\n', 2
}