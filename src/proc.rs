use super::{Error, Result};
use std::collections::HashMap;

pub fn buddyinfo() -> Result<HashMap<(usize, String), [usize; 10]>> {
    let content = std::fs::read_to_string("/proc/buddyinfo")?;
    let mut ret = HashMap::new();

    for line in content.lines() {
        let mut column = line.split_ascii_whitespace();

        let _ = column.next();
        let node = column.next().ok_or(Error::BadFormat)?;
        let node = node
            .trim()
            .trim_end_matches(',')
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let _ = column.next();
        let zone = column.next().ok_or(Error::BadFormat)?;

        let mut list = [0; 10];
        for i in 0..10 {
            let num = column.next().ok_or(Error::BadFormat)?;
            let num = num.parse::<usize>().map_err(|_| Error::BadFormat)?;
            list[i] = num;
        }

        ret.insert((node, zone.to_string()), list);
    }

    Ok(ret)
}

pub mod acpi;
pub mod driver;

default_read! {cmdline, "/proc/cmdline"}

default_read! {consoles, "/proc/consoles"}

default_pairs! {cpuinfo, "/proc/cpuinfo"}

default_pairs! {crypto, "/proc/crypto"}

pub fn devices() -> Result<(Vec<(usize, String)>, Vec<(usize, String)>)> {
    let content = std::fs::read_to_string("/proc/devices")?;
    let mut block = content.split("\n\n");

    let character_devices_block = block.next().ok_or(Error::BadFormat)?;
    let mut character_devices = vec![];
    let mut line_iter = character_devices_block.lines();
    let _ = line_iter.next();
    for dev in line_iter {
        let mut item = dev.trim().split(" ");
        let major_number = item.next().ok_or(Error::BadFormat)?;
        let name = item.next().ok_or(Error::BadFormat)?;

        let major_number = major_number
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;
        character_devices.push((major_number, name.to_string()));
    }

    let block_devices_block = block.next().ok_or(Error::BadFormat)?;
    let mut block_devices = vec![];
    let mut line_iter = block_devices_block.lines();
    let _ = line_iter.next();
    for dev in line_iter {
        let mut item = dev.trim().split(" ");
        let major_number = item.next().ok_or(Error::BadFormat)?;
        let name = item.next().ok_or(Error::BadFormat)?;

        let major_number = major_number
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;
        block_devices.push((major_number, name.to_string()));
    }

    Ok((character_devices, block_devices))
}

#[derive(Debug)]
pub struct Disk {
    pub major_number: usize,
    pub minor_number: usize,
    pub device_name: String,
    pub reads_completed_successfully: usize,
    pub reads_merged: usize,
    pub sectors_read: usize,
    pub time_spent_reading: usize,
    pub writes_completed: usize,
    pub writes_merged: usize,
    pub sectors_written: usize,
    pub time_spent_writing: usize,
    pub ios_currently_in_progress: usize,
    pub time_spent_doing_ios: usize,
    pub weighted_time_spent_doing_ios: usize,
    pub discards_completed_successfully: usize,
    pub discards_merged: usize,
    pub sectors_discarded: usize,
    pub time_spent_discarding: usize,
}

impl std::ops::Index<usize> for Disk {
    type Output = usize;

    fn index(&self, index: usize) -> &usize {
        if index == 1 {
            &self.major_number
        } else if index == 2 {
            &self.minor_number
        } else if index == 4 {
            &self.reads_completed_successfully
        } else if index == 5 {
            &self.reads_merged
        } else if index == 6 {
            &self.sectors_read
        } else if index == 7 {
            &self.time_spent_reading
        } else if index == 8 {
            &self.writes_completed
        } else if index == 9 {
            &self.writes_merged
        } else if index == 10 {
            &self.sectors_written
        } else if index == 11 {
            &self.time_spent_writing
        } else if index == 12 {
            &self.ios_currently_in_progress
        } else if index == 13 {
            &self.time_spent_doing_ios
        } else if index == 14 {
            &self.weighted_time_spent_doing_ios
        } else if index == 15 {
            &self.discards_completed_successfully
        } else if index == 16 {
            &self.discards_merged
        } else if index == 17 {
            &self.sectors_discarded
        } else if index == 18 {
            &self.time_spent_discarding
        } else {
            panic!("index undefined field")
        }
    }
}

pub fn diskstats() -> Result<Vec<Disk>> {
    let content = std::fs::read_to_string("/proc/diskstats")?;
    let mut ret = vec![];

    for disk in content.lines() {
        let mut item_iter = disk.trim().split_ascii_whitespace();

        let major_number = item_iter.next().ok_or(Error::BadFormat)?;
        let major_number = major_number
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let minor_number = item_iter.next().ok_or(Error::BadFormat)?;
        let minor_number = minor_number
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let device_name = item_iter
            .next()
            .map(|s| s.to_string())
            .ok_or(Error::BadFormat)?;

        let reads_completed_successfully = item_iter.next().ok_or(Error::BadFormat)?;
        let reads_completed_successfully = reads_completed_successfully
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let reads_merged = item_iter.next().ok_or(Error::BadFormat)?;
        let reads_merged = reads_merged
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let sectors_read = item_iter.next().ok_or(Error::BadFormat)?;
        let sectors_read = sectors_read
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let time_spent_reading = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_reading = time_spent_reading
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let writes_completed = item_iter.next().ok_or(Error::BadFormat)?;
        let writes_completed = writes_completed
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let writes_merged = item_iter.next().ok_or(Error::BadFormat)?;
        let writes_merged = writes_merged
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let sectors_written = item_iter.next().ok_or(Error::BadFormat)?;
        let sectors_written = sectors_written
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let time_spent_writing = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_writing = time_spent_writing
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let ios_currently_in_progress = item_iter.next().ok_or(Error::BadFormat)?;
        let ios_currently_in_progress = ios_currently_in_progress
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let time_spent_doing_ios = item_iter.next().ok_or(Error::BadFormat)?;
        let time_spent_doing_ios = time_spent_doing_ios
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let weighted_time_spent_doing_ios = item_iter.next().ok_or(Error::BadFormat)?;
        let weighted_time_spent_doing_ios = weighted_time_spent_doing_ios
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let discards_completed_successfully = item_iter.next().unwrap_or("0");
        let discards_completed_successfully = discards_completed_successfully
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let discards_merged = item_iter.next().unwrap_or("0");
        let discards_merged = discards_merged
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let sectors_discarded = item_iter.next().unwrap_or("0");
        let sectors_discarded = sectors_discarded
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let time_spent_discarding = item_iter.next().unwrap_or("0");
        let time_spent_discarding = time_spent_discarding
            .parse::<usize>()
            .map_err(|_| Error::BadFormat)?;

        let disk = Disk {
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
        };
        ret.push(disk);
    }
    Ok(ret)
}
