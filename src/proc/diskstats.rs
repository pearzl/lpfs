use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/diskstats
///
/// if kernel version is blow 4.18, the last four filed is not exist in file, then they will be 0.
/// 
/// ```
/// let disks = diskstats().unwrap();
/// for disk in disks {
///     println!("disk name: {}", disk.device_name());
/// }
/// ```
#[derive(Debug)]
pub struct DiskStat {
    major_number: usize,
    minor_number: usize,
    device_name: String,
    reads_completed_successfully: usize,
    reads_merged: usize,
    sectors_read: usize,
    time_spent_reading: usize,
    writes_completed: usize,
    writes_merged: usize,
    sectors_written: usize,
    time_spent_writing: usize,
    ios_currently_in_progress: usize,
    time_spent_doing_ios: usize,
    weighted_time_spent_doing_ios: usize,
    discards_completed_successfully: usize,
    discards_merged: usize,
    sectors_discarded: usize,
    time_spent_discarding: usize,
}

impl DiskStat {
    getter_gen! {
        major_number: usize,
        minor_number: usize,
        device_name: String,
        reads_completed_successfully: usize,
        reads_merged: usize,
        sectors_read: usize,
        time_spent_reading: usize,
        writes_completed: usize,
        writes_merged: usize,
        sectors_written: usize,
        time_spent_writing: usize,
        ios_currently_in_progress: usize,
        time_spent_doing_ios: usize,
        weighted_time_spent_doing_ios: usize,
        discards_completed_successfully: usize,
        discards_merged: usize,
        sectors_discarded: usize,
        time_spent_discarding: usize
    }
}

impl FromStr for DiskStat {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut item_iter = value.trim().split_ascii_whitespace();

        let major_number = item_iter.next().ok_or(Error::BadFormat)?;
        let major_number = major_number.parse::<usize>()?;

        let minor_number = item_iter.next().ok_or(Error::BadFormat)?;
        let minor_number = minor_number.parse::<usize>()?;

        let device_name = item_iter
            .next()
            .map(|s| s.to_string())
            .ok_or(Error::BadFormat)?;

        let reads_completed_successfully = item_iter.next().ok_or(Error::BadFormat)?;
        let reads_completed_successfully = reads_completed_successfully.parse::<usize>()?;

        let reads_merged = item_iter.next().ok_or(Error::BadFormat)?;
        let reads_merged = reads_merged.parse::<usize>()?;

        let sectors_read = item_iter.next().ok_or(Error::BadFormat)?;
        let sectors_read = sectors_read.parse::<usize>()?;

        let time_spent_reading = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_reading = time_spent_reading.parse::<usize>()?;

        let writes_completed = item_iter.next().ok_or(Error::BadFormat)?;
        let writes_completed = writes_completed.parse::<usize>()?;

        let writes_merged = item_iter.next().ok_or(Error::BadFormat)?;
        let writes_merged = writes_merged.parse::<usize>()?;

        let sectors_written = item_iter.next().ok_or(Error::BadFormat)?;
        let sectors_written = sectors_written.parse::<usize>()?;

        let time_spent_writing = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_writing = time_spent_writing.parse::<usize>()?;

        let ios_currently_in_progress = item_iter.next().ok_or(Error::BadFormat)?;
        let ios_currently_in_progress = ios_currently_in_progress.parse::<usize>()?;

        let time_spent_doing_ios = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_doing_ios = time_spent_doing_ios.parse::<usize>()?;

        let weighted_time_spent_doing_ios = item_iter.next().ok_or(Error::BadFormat)?;
        let weighted_time_spent_doing_ios = weighted_time_spent_doing_ios.parse::<usize>()?;

        let discards_completed_successfully = item_iter.next().unwrap_or("0");
        let discards_completed_successfully = discards_completed_successfully.parse::<usize>()?;

        let discards_merged = item_iter.next().unwrap_or("0");
        let discards_merged = discards_merged.parse::<usize>()?;

        let sectors_discarded = item_iter.next().unwrap_or("0");
        let sectors_discarded = sectors_discarded.parse::<usize>()?;

        let time_spent_discarding = item_iter.next().unwrap_or("0");
        let time_spent_discarding = time_spent_discarding.parse::<usize>()?;

        Ok(DiskStat {
            major_number,
            minor_number,
            device_name,
            reads_completed_successfully,
            reads_merged,
            sectors_read,
            time_spent_reading,
            writes_completed,
            writes_merged,
            sectors_written,
            time_spent_writing,
            ios_currently_in_progress,
            time_spent_doing_ios,
            weighted_time_spent_doing_ios,
            discards_completed_successfully,
            discards_merged,
            sectors_discarded,
            time_spent_discarding,
        })
    }
}

#[inline(always)]
fn to_diskstats(line: &str) -> Result<DiskStat> {
    DiskStat::from_str(line)
}

default_list! {
    diskstats, "/proc/diskstats", DiskStat, to_diskstats
}
