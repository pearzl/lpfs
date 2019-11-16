use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/buddyinfo
///
/// ```
/// use proc_getter::buddyinfo::*;
///
/// let bis = buddyinfo().unwrap();
/// for bi in bis {
///     let chunk_0= bi.page_nums()[0];
///     println!("There are {} chunks of memory avaliable in zone {}", chunk_0, bi.zone());
/// }
/// ```
#[derive(Debug)]
pub struct BuddyInfo {
    node: usize,
    zone: String,
    page_nums: [usize; 11],
}

impl BuddyInfo {
    getter_gen! {
        node: usize,
        zone: String,
        page_nums: [usize; 11]
    }
}

impl FromStr for BuddyInfo {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.trim().split_ascii_whitespace().collect();
        if columns.len() != 15 {
            return Err(Error::BadFormat);
        }

        let node = columns[1].trim_end_matches(',').parse::<usize>()?;
        let zone = columns[3].to_string();
        let mut page_nums = [0; 11];
        for (n, s) in page_nums.iter_mut().zip(&columns[4..]) {
            *n = s.parse::<usize>()?;
        }

        Ok(BuddyInfo {
            node,
            zone,
            page_nums,
        })
    }
}

#[inline(always)]
fn to_buddyinfo(line: &str) -> Result<BuddyInfo> {
    BuddyInfo::from_str(line)
}

default_list! {
   buddyinfo, "/proc/buddyinfo", BuddyInfo, to_buddyinfo
}
