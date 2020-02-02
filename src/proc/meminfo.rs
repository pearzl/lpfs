// > 5.2.19.  /proc/meminfo
// > This is one of the more commonly used files in the /proc/ directory, as it reports a large amount of valuable information about the systems RAM usage.
// > The following sample /proc/meminfo virtual file is from a system with 256 MB of RAM and 512 MB of swap space:
// > MemTotal:       255908 kB
// > MemFree:         69936 kB
// > Buffers:         15812 kB
// > Cached:         115124 kB
// > SwapCached:          0 kB
// > Active:          92700 kB
// > Inactive:        63792 kB
// > HighTotal:           0 kB
// > HighFree:            0 kB
// > LowTotal:       255908 kB
// > LowFree:         69936 kB
// > SwapTotal:      524280 kB
// > SwapFree:       524280 kB
// > Dirty:               4 kB
// > Writeback:           0 kB
// > Mapped:          42236 kB
// > Slab:            25912 kB
// > Committed_AS:   118680 kB
// > PageTables:       1236 kB
// > VmallocTotal:  3874808 kB
// > VmallocUsed:      1416 kB
// > VmallocChunk:  3872908 kB
// > HugePages_Total:     0
// > HugePages_Free:      0
// > Hugepagesize:     4096 kB
// > Much of the information here is used by the free, top, and ps commands. In fact, the output of the free command is similar in appearance to the contents and structure of /proc/meminfo. But by looking directly at /proc/meminfo, more details are revealed:
// > MemTotal — Total amount of physical RAM, in kilobytes.
// > MemFree — The amount of physical RAM, in kilobytes, left unused by the system.
// > Buffers — The amount of physical RAM, in kilobytes, used for file buffers.
// > Cached — The amount of physical RAM, in kilobytes, used as cache memory.
// > SwapCached — The amount of swap, in kilobytes, used as cache memory.
// > Active — The total amount of buffer or page cache memory, in kilobytes, that is in active use. This is memory that has been recently used and is usually not reclaimed for other purposes.
// > Inactive — The total amount of buffer or page cache memory, in kilobytes, that are free and available. This is memory that has not been recently used and can be reclaimed for other purposes.
// > HighTotal and HighFree — The total and free amount of memory, in kilobytes, that is not directly mapped into kernel space. The HighTotal value can vary based on the type of kernel used.
// > LowTotal and LowFree — The total and free amount of memory, in kilobytes, that is directly mapped into kernel space. The LowTotal value can vary based on the type of kernel used.
// > SwapTotal — The total amount of swap available, in kilobytes.
// > SwapFree — The total amount of swap free, in kilobytes.
// > Dirty — The total amount of memory, in kilobytes, waiting to be written back to the disk.
// > Writeback — The total amount of memory, in kilobytes, actively being written back to the disk.
// > Mapped — The total amount of memory, in kilobytes, which have been used to map devices, files, or libraries using the mmap command.
// > Slab — The total amount of memory, in kilobytes, used by the kernel to cache data structures for its own use.
// > Committed_AS — The total amount of memory, in kilobytes, estimated to complete the workload. This value represents the worst case scenario value, and also includes swap memory.
// > PageTables — The total amount of memory, in kilobytes, dedicated to the lowest page table level.
// > VMallocTotal — The total amount of memory, in kilobytes, of total allocated virtual address space.
// > VMallocUsed — The total amount of memory, in kilobytes, of used virtual address space.
// > VMallocChunk — The largest contiguous block of memory, in kilobytes, of available virtual address space.
// > HugePages_Total — The total number of hugepages for the system. The number is derived by dividing Hugepagesize by the megabytes set aside for hugepages specified in /proc/sys/vm/hugetlb_pool. This statistic only appears on the x86, Itanium, and AMD64 architectures.
// > HugePages_Free — The total number of hugepages available for the system. This statistic only appears on the x86, Itanium, and AMD64 architectures.
// > Hugepagesize — The size for each hugepages unit in kilobytes. By default, the value is 4096 KB on uniprocessor kernels for 32 bit architectures. For SMP, hugemem kernels, and AMD64, the default is 2048 KB. For Itanium architectures, the default is 262144 KB. This statistic only appears on the x86, Itanium, and AMD64 architectures.
// 
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-meminfo
// 

use std::collections::HashMap;

/// Returnd by [`meminfo()`](fn.meminfo.html)
/// 
/// MemInfo can be derefed to a HashMap, so any item can be retrived by the map.
/// 
/// ```no_run
/// use lpfs::proc::meminfo::*;
/// let mi = meminfo().unwrap();
/// let total_memory1 = mi.get("MemTotal").unwrap();
/// let total_memory2 = mi.mem_total();
/// assert_eq!(*total_memory1, total_memory2);
/// ```
/// 

#[derive(Debug, PartialEq, Clone)]
pub struct MemInfo(HashMap<String, usize>);

impl std::ops::Deref for MemInfo {
    type Target = HashMap<String, usize>;

    fn deref(&self) -> &HashMap<String, usize> {
        &self.0
    }
}

macro_rules! get_fields {
    ($(
        $method: ident, $key: expr
    );*) => {
        impl MemInfo{$(
            #[doc="Retrive`"]
            #[doc=$key]
            #[doc="`. This will panic if it is not exist."]
            pub fn $method(&self) -> usize {
                *self.0.get($key).expect(concat!($key, " is not exists in meminfo"))
            }
        )*}
    };
}

get_fields! {
    mem_total, "MemTotal";
    mem_free, "MemFree";
    mem_avilablle, "MemAvailable";
    buffers, "Buffers";
    cached, "Cached";
    swap_cached, "SwapCached";
    active, "Active";
    inactive, "Inactive";
    swap_total, "SwapTotal";
    swap_free, "SwapFree";
    dirty, "Dirty";
    writeback, "Writeback";
    mapped, "Mapped";
    slab, "Slab";
    committed_as, "Committed_AS";
    page_tables, "PageTables";
    vm_alloc_total, "VMallocTotal";
    vm_alloc_used, "VMallocUsed";
    vm_alloc_chunk, "VMallocChunk";
    huge_page_total, "HugePages_Total";
    huge_pages_free, "HugePages_Free ";
    hugepagesize, "Hugepagesize";
    direct_map_4k, "DirectMap4k";
    direct_map_2m, "DirectMap2M";
    direct_map_1g, "DirectMap1G"
}


use std::str::FromStr;
impl FromStr for MemInfo {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for line in s.lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err("not k-v pair".into());
            }
            let key = columns[0].trim().to_string();
            let value = columns[1]
                .trim()
                .trim_end_matches("kB")
                .trim()
                .parse::<usize>()?;
            map.insert(key, value);
        }
        Ok(MemInfo(map))
    }
}

instance_impl! {
    meminfo, "/proc/meminfo", MemInfo
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_parse_meminfo() {
        let source = "VmallocChunk:  3872908 kB\nHugePages_Total:     0";
        let mut map = HashMap::new();
        map.insert("VmallocChunk".into(), 3872908);
        map.insert("HugePages_Total".into(), 0);
        assert_eq!(MemInfo(map), source.parse().unwrap());
    }
} 