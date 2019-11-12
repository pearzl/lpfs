use crate::{Error, Result};
use std::collections::HashMap;
use std::convert::From;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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

default_read! {cmdline, "/proc/cmdline"}

#[derive(Debug)]
pub struct CpuInfo(HashMap<String, String>);

impl FromStr for CpuInfo {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut ret = HashMap::new();
        for line in value.trim().lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err(Error::BadFormat);
            }
            ret.insert(columns[0].trim().to_string(), columns[1].trim().to_string());
        }
        Ok(CpuInfo(ret))
    }
}

#[inline(always)]
fn to_cpuinfo(block: &str) -> Result<CpuInfo> {
    CpuInfo::from_str(block)
}

default_list! {
    cpuinfo, "/proc/cpuinfo", CpuInfo, to_cpuinfo, "\n\n"
}

#[derive(Debug)]
pub struct Crypto(HashMap<String, String>);

impl FromStr for Crypto {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut ret = HashMap::new();
        for line in value.trim().lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err(Error::BadFormat);
            }
            ret.insert(columns[0].to_string(), columns[1].to_string());
        }
        Ok(Crypto(ret))
    }
}

#[inline(always)]
fn to_crypto(block: &str) -> Result<Crypto> {
    Crypto::from_str(block)
}

default_list! {
    crypto, "/proc/crypto", Crypto, to_crypto, "\n\n"
}

#[derive(Debug)]
pub struct Dma {
    channel: usize,
    driver: String,
}

impl FromStr for Dma {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split(':').collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Dma {
            channel: columns[0].trim().parse::<usize>()?,
            driver: columns[1].trim().to_string(),
        })
    }
}

#[inline(always)]
fn to_dma(line: &str) -> Result<Dma> {
    Dma::from_str(line)
}

default_list! {
    dma, "/proc/dma", Dma, to_dma
}

default_read! {execdomains, "/proc/execdomains"}

#[derive(Debug)]
pub struct Device {
    major_number: usize,
    name: String,
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Device {
            major_number: columns[0].parse::<usize>()?,
            name: columns[1].to_string(),
        })
    }
}

pub fn devices() -> Result<(Vec<Device>, Vec<Device>)> {
    let content = std::fs::read_to_string("/proc/devices")?;
    let areas: Vec<&str> = content.split("\n\n").collect();
    if areas.len() != 2 {
        return Err(Error::BadFormat);
    }

    let characters: Vec<&str> = areas[0].trim().lines().collect();
    let mut character_devices = vec![];
    for s in characters.iter().skip(1) {
        let t = Device::from_str(s)?;
        character_devices.push(t);
    }

    let blocks: Vec<&str> = areas[1].trim().lines().collect();
    let mut block_devices = vec![];
    for s in blocks.iter().skip(1) {
        let t = Device::from_str(s)?;
        block_devices.push(t);
    }

    Ok((character_devices, block_devices))
}

#[derive(Debug)]
pub struct Fb {
    device: usize,
    driver: String,
}

impl FromStr for Fb {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut columns = value.split_ascii_whitespace();
        let device = columns.next().ok_or(Error::BadFormat)?.parse::<usize>()?;
        let driver: String = columns.collect();
        Ok(Fb { device, driver })
    }
}

#[inline(always)]
fn to_fb(line: &str) -> Result<Fb> {
    Fb::from_str(line)
}

default_list! {
    fb, "/proc/fb", Fb, to_fb
}

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

#[derive(Debug)]
/// returned by [`interrupts`](fn.interrupts.html)
pub enum Interrupt {
    Internal {
        name: String,
        counts: Vec<usize>,
        detail: String,
    },
    Device {
        irq_number: usize,
        counts: Vec<usize>,
        type_of: String,
        device_name: String,
    },
}

/// > Since Linux 2.6.24, for the i386 and x86-64 architec‐tures,
/// > at least, this also includes interrupts internal to the system.
/// >
/// > http://man7.org/linux/man-pages/man5/proc.5.html
///
/// For the reasons mentioned above, interrupt is represent by enum, Interrupt.
/// There are two variant, Internal and Device,
/// describe the internal interrupt and io device interrupt respectively.
///
/// Both variants contain counts filed, which represent the number of interrupts.
/// The length of Vector equals the CPU numbers.
/// First element is for CPU0, second for CPU1, and so on.
pub fn interrupts() -> Result<Vec<Interrupt>> {
    let content = std::fs::read_to_string("/proc/interrupts")?;
    let mut ret = vec![];
    let mut line_iter = content.trim_end().lines();

    let cpu_line = line_iter.next().ok_or(Error::BadFormat)?;
    let mut cpu_num = 0;
    for _ in cpu_line.trim().split_ascii_whitespace() {
        cpu_num += 1;
    }

    for line in line_iter {
        let mut columns: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        let column1 = columns[0].trim_end_matches(':');
        let mut counts = Vec::with_capacity(cpu_num);

        if let Ok(irq) = column1.parse::<usize>() {
            for item in columns.iter().take(cpu_num + 1).skip(1) {
                let c = item.trim().parse::<usize>()?;
                counts.push(c);
            }

            let device_name = columns.pop().ok_or(Error::BadFormat)?.to_string();
            let type_of: String = columns[1 + cpu_num..].iter().copied().collect();

            ret.push(Interrupt::Device {
                irq_number: irq,
                counts,
                type_of,
                device_name,
            });
        } else if column1 == "ERR" {
            let c = columns[1].trim().parse::<usize>()?;
            counts.push(c);
            ret.push(Interrupt::Internal {
                name: "ERR".to_string(),
                counts,
                detail: "".to_string(),
            });
        } else if column1 == "MIS" {
            let c = columns[1].trim().parse::<usize>()?;
            counts.push(c);
            ret.push(Interrupt::Internal {
                name: "MIS".to_string(),
                counts,
                detail: "".to_string(),
            });
        } else {
            let name = column1.to_string();

            for item in columns.iter().take(cpu_num + 1).skip(1) {
                let c = item.trim().parse::<usize>()?;
                counts.push(c);
            }

            let detail: String = columns[1 + cpu_num..].iter().copied().collect();

            ret.push(Interrupt::Internal {
                name,
                counts,
                detail,
            })
        }
    }

    Ok(ret)
}

#[derive(Debug)]
pub struct IoMem {
    start: usize,
    end: usize,
    kind: String,
}

impl IoMem {
    getter_gen! {
        start: usize,
        end: usize,
        kind: String
    }
}

impl FromStr for IoMem {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = value.split(|c| c == '-' || c == ':').collect();
        if items.len() != 3 {
            return Err(Error::BadFormat);
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let kind = items[2].to_string();

        Ok(IoMem { start, end, kind })
    }
}

#[inline(always)]
fn to_iomem(line: &str) -> Result<IoMem> {
    IoMem::from_str(line)
}

default_list! {
    iomem, "/proc/iomem", IoMem, to_iomem
}

#[derive(Debug)]
pub struct IoPort {
    start: usize,
    end: usize,
    device: String,
}

impl IoPort {
    getter_gen! {
        start: usize,
        end: usize,
        device: String
    }
}

impl FromStr for IoPort {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = value.split(|c| c == '-' || c == ':').collect();
        if items.len() != 3 {
            return Err(Error::BadFormat);
        }

        let start = usize::from_str_radix(items[0], 16)?;
        let end = usize::from_str_radix(items[1], 16)?;
        let device = items[2].to_string();

        Ok(IoPort { start, end, device })
    }
}

#[inline(always)]
fn to_ioports(line: &str) -> Result<IoPort> {
    IoPort::from_str(line)
}

default_list! {
    ioports, "/proc/ioports", IoPort, to_ioports
}

/// return the size of /proc/kcore in bytes.
///
/// `kcore()` is not exist in this crate, because it's content is not human readable.
///
/// Note:
/// > This value is given in bytes and is equal to the size of the physical memory (RAM) used plus 4 KB.
/// >
/// > https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/s2-proc-kcore
pub fn kcore_size() -> Result<u64> {
    let md = std::fs::metadata("/proc/kcore")?;
    Ok(md.len())
}

/// returned by [`loadavg()`](fn.loadavg.html)
#[derive(Debug)]
pub struct LoadAvg {
    one: f32,
    five: f32,
    fifteen: f32,
    cur_num: usize,
    total_num: usize,
    last_pid: usize,
}

impl LoadAvg {
    getter_gen! {
        one: f32,
        five: f32,
        fifteen: f32,
        cur_num: usize,
        total_num: usize,
        last_pid: usize
    }
}

impl FromStr for LoadAvg {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut column_iter = value.trim().split_ascii_whitespace();
        let one = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let five = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let fifteen = column_iter.next().ok_or(Error::BadFormat)?.parse::<f32>()?;
        let pnum: Vec<&str> = column_iter
            .next()
            .ok_or(Error::BadFormat)?
            .split('/')
            .collect();
        let cur_num = pnum.get(0).ok_or(Error::BadFormat)?.parse::<usize>()?;
        let total_num = pnum.get(1).ok_or(Error::BadFormat)?.parse::<usize>()?;
        let last_pid = column_iter
            .next()
            .ok_or(Error::BadFormat)?
            .parse::<usize>()?;
        Ok(LoadAvg {
            one,
            five,
            fifteen,
            cur_num,
            total_num,
            last_pid,
        })
    }
}

/// The content is parsed to a LoadAvg.
/// ```
/// use linux_proc::proc::loadavg::*;
/// fn main() {
///     let la = loadavg().unwrap();
///     println!("one minute load: {}", la.one());
///     println!("five minutes load: {}", la.five());
///     println!("fifteen minutes laod: {}", la.fifteen());
///     println!("current process number: {}", la.cur_num());
///     println!("total process number: {}", la.total_num());
///     println!("last process id: {}", la.last_pid());
/// }
/// ```
pub fn loadavg() -> Result<LoadAvg> {
    let content = std::fs::read_to_string("/proc/loadavg")?;
    LoadAvg::from_str(&content)
}

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

// TODO: detail the fileds
#[derive(Debug)]
pub struct MemInfo {
    entry: HashMap<String, usize>,
}

impl FromStr for MemInfo {
    type Err = Error;

    fn from_str(mi: &str) -> Result<Self> {
        let mut map = HashMap::new();
        for line in mi.lines() {
            let columns: Vec<&str> = line.split_ascii_whitespace().collect();
            if columns.len() != 3 {
                return Err(Error::BadFormat);
            }
            let key = columns[0].trim_end_matches(':').to_string();
            let value = columns[1].parse::<usize>()?;
            map.insert(key, value);
        }
        Ok(MemInfo { entry: map })
    }
}

pub fn meminfo() -> Result<MemInfo> {
    let content = std::fs::read_to_string("/proc/meminfo")?;
    MemInfo::from_str(&content)
}

#[derive(Debug)]
pub struct Misc {
    device: usize,
    driver: String,
}

impl FromStr for Misc {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.trim().split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        Ok(Misc {
            device: columns[0].parse::<usize>()?,
            driver: columns[1].to_string(),
        })
    }
}

#[inline(always)]
fn to_misc(line: &str) -> Result<Misc> {
    Misc::from_str(line)
}

default_list! { misc, "/proc/misc", Misc, to_misc }

#[derive(Debug)]
pub struct Module {
    name: String,
    mem_size: usize,
    instance_nums: usize,
    deps: Vec<String>,
    state: State,
    offset: usize,
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }
        let name = columns[0].to_string();
        let mem_size = columns[1].parse::<usize>()?;
        let instance_nums = columns[2].parse::<usize>()?;
        let deps: Vec<String> = if columns[3] == "-" {
            Vec::new()
        } else {
            columns[3].split(',').map(|s| s.to_string()).collect()
        };
        let state = State::from_str(columns[4])?;
        let offset = usize::from_str_radix(columns[5].trim_start_matches("0x"), 16)?;
        Ok(Module {
            name,
            mem_size,
            instance_nums,
            deps,
            state,
            offset,
        })
    }
}

#[derive(Debug)]
pub enum State {
    Live,
    Loading,
    Unloading,
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "Live" {
            Ok(State::Live)
        } else if s == "Loading" {
            Ok(State::Loading)
        } else if s == "Unloading" {
            Ok(State::Unloading)
        } else {
            Err(Error::BadFormat)
        }
    }
}

#[inline(always)]
fn to_modules(line: &str) -> Result<Module> {
    Module::from_str(line)
}

default_list! {
    modules, "/proc/modules", Module, to_modules
}

/// Unimplemented now.
pub fn kmsg() -> super::Result<String> {
    unimplemented!()
}

default_read! {mdstat, "/proc/mdstat"}

default_read! {consoles, "/proc/consoles"}

#[derive(Debug)]
pub struct Mount {
    device: String,
    mount_point: PathBuf,
    fs_type: String,
    mode: String,
    dummy1: String,
    dummy2: String,
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

#[derive(Debug)]
pub struct Partition {
    major: usize,
    minor: usize,
    blocks: usize,
    name: String,
}

impl FromStr for Partition {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        if columns.len() != 4 {
            return Err(Error::BadFormat);
        }
        let major = columns[0].parse::<usize>()?;
        let minor = columns[1].parse::<usize>()?;
        let blocks = columns[2].parse::<usize>()?;
        let name = columns[4].to_string();
        Ok(Partition {
            major,
            minor,
            blocks,
            name,
        })
    }
}

#[inline(always)]
fn to_partition(line: &str) -> Result<Partition> {
    Partition::from_str(line)
}

default_list! {
    partitions, "/proc/partitions", Partition, to_partition, '\n', 1
}

default_read! {mtrr, "/proc/consoles"}

#[derive(Debug)]
pub struct Object {
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

impl FromStr for Object {
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

        Ok(Object {
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
fn to_slabinfo(line: &str) -> Result<Object> {
    Object::from_str(line)
}

default_list! {
    slabinfo, "/proc/slabinfo", Object, to_slabinfo, '\n', 2
}

#[derive(Debug)]
pub struct Stat {
    cpu: Cpu,
    cpu_n: Vec<Cpu>,
    intr: Vec<usize>,
    ctxt: usize,
    btime: usize,
    procs_running: usize,
    processes: usize,
    procs_blocked: usize,
    softirq: Vec<usize>,
}

#[derive(Debug)]
pub struct Cpu {
    user: usize,
    nice: usize,
    system: usize,
    idle: usize,
    iowait: usize,
    irq: usize,
    softirq: usize,
    steal: usize,
    guest: usize,
    guest_nice: usize,
}

impl Cpu {
    getter_gen! {
        user: usize,
        nice: usize,
        system: usize,
        idle: usize,
        iowait: usize,
        irq: usize,
        softirq: usize,
        steal: usize,
        guest: usize,
        guest_nice: usize
    }
}

impl From<[usize; 10]> for Cpu {
    fn from(x: [usize; 10]) -> Cpu {
        Cpu {
            user: x[0],
            nice: x[1],
            system: x[2],
            idle: x[3],
            iowait: x[4],
            irq: x[5],
            softirq: x[6],
            steal: x[7],
            guest: x[8],
            guest_nice: x[9],
        }
    }
}

impl FromStr for Stat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Stat> {
        let lines: Vec<&str> = s.trim().lines().collect();
        let line_num = lines.len();
        if line_num < 8 {
            return Err(Error::BadFormat);
        }

        let mut cpu_arr = [0; 10];
        let cpu_columns: Vec<&str> = lines[0].split_ascii_whitespace().collect();
        if cpu_columns.len() != 11 || cpu_columns[0] != "cpu" {
            return Err(Error::BadFormat);
        }
        for (i, v) in cpu_columns[1..].iter().enumerate() {
            let n = v.parse::<usize>()?;
            cpu_arr[i] = n;
        }
        let cpu = Cpu::from(cpu_arr);

        let mut last_n = line_num - 1;

        let softirq_columns: Vec<&str> = lines[last_n].split_ascii_whitespace().collect();
        if softirq_columns[0] != "softirq" {
            return Err(Error::BadFormat);
        }
        let mut softirq: Vec<usize> = vec![];
        for s in &softirq_columns[1..] {
            let n = s.parse::<usize>()?;
            softirq.push(n);
        }
        last_n -= 1;

        let procs_blocked = lines[last_n]
            .trim_start_matches("procs_blocked")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let procs_running = lines[last_n]
            .trim_start_matches("procs_running")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let processes = lines[last_n]
            .trim_start_matches("processes")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let btime = lines[last_n]
            .trim_start_matches("btime")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let ctxt = lines[last_n]
            .trim_start_matches("ctxt")
            .trim()
            .parse::<usize>()?;
        last_n -= 1;

        let mut intr: Vec<usize> = vec![];
        for s in lines[last_n]
            .trim_start_matches("intr")
            .split_ascii_whitespace()
        {
            let n = s.parse::<usize>()?;
            intr.push(n);
        }
        last_n -= 1;

        let mut cpu_n = Vec::with_capacity(last_n);

        for i in 0..last_n {
            let mut cpu_arr = [0; 10];
            let columns: Vec<&str> = lines[i + 1]
                .trim_start_matches("cpu")
                .split_ascii_whitespace()
                .collect();
            if columns.len() != 11 || columns[0] != i.to_string() {
                return Err(Error::BadFormat);
            }
            for (k, v) in columns[1..].iter().enumerate() {
                cpu_arr[k] = v.parse::<usize>()?;
            }
            cpu_n.push(Cpu::from(cpu_arr));
        }

        Ok(Stat {
            cpu,
            cpu_n,
            intr,
            ctxt,
            btime,
            processes,
            procs_running,
            procs_blocked,
            softirq,
        })
    }
}

pub fn stat() -> Result<Stat> {
    let content = std::fs::read_to_string("/proc/stat")?;
    Stat::from_str(&content)
}

#[derive(Debug)]
pub struct Swap {
    filename: String,
    r#type: String,
    size: usize,
    used: usize,
    priority: isize,
}

impl FromStr for Swap {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let columns: Vec<&str> = value.split_ascii_whitespace().collect();
        if columns.len() != 5 {
            return Err(Error::BadFormat);
        }
        let filename = columns[0].to_string();
        let r#type = columns[1].to_string();
        let size = columns[2].parse::<usize>()?;
        let used = columns[3].parse::<usize>()?;
        let priority = columns[4].parse::<isize>()?;
        Ok(Swap {
            filename,
            r#type,
            size,
            used,
            priority,
        })
    }
}

#[inline(always)]
fn to_swaps(line: &str) -> Result<Swap> {
    Swap::from_str(line)
}

default_list! {
    swaps, "/proc/swaps", Swap, to_swaps, '\n', 1
}

#[derive(Debug)]
pub struct Uptime {
    total: f64,
    idle: f64,
}

impl FromStr for Uptime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let columns: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err(Error::BadFormat);
        }
        let total = columns[0].parse::<f64>()?;
        let idle = columns[1].parse::<f64>()?;
        Ok(Uptime { total, idle })
    }
}

pub fn uptime() -> Result<Uptime> {
    let content = std::fs::read_to_string("/proc/uptime")?;
    Uptime::from_str(&content)
}

default_read! {version, "/proc/version"}

pub fn proc_self() -> Result<PathBuf> {
    Ok(std::fs::read_link("/proc/self")?)
}

pub mod acpi;
pub mod driver;
pub mod pid;
