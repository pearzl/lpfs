//! > ### 5.2.2.  `/proc/buddyinfo`
//! > 
//! > <a id="idm139745916909296" class="indexterm" href=""></a>
//! > 
//! > This file is used primarily for diagnosing memory fragmentation issues. 
//! > Using the buddy algorithm, each column represents the number of pages of a certain order (a certain size) that are available at any given time. 
//! > For example, for zone DMA (direct memory access), there are 90 of 2^(0*PAGE_SIZE) chunks of memory. 
//! > Similarly, there are 6 of 2^(1*PAGE_SIZE) chunks, and 2 of 2^(2*PAGE_SIZE) chunks of memory available.
//! > 
//! > The `DMA` row references the first 16 MB on a system, the `HighMem` row references all memory greater than 4 GB on a system, and the `Normal` row references all memory in between.
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-buddyinfo
//! >
//! >
//! > 
//! > 
//! > Node 0, zone      DMA      0      4      5      4      4      3 ...
//! > Node 0, zone   Normal      1      0      0      1    101      8 ...
//! > Node 0, zone  HighMem      2      0      0      1      1      0 ...
//! > 
//! > External fragmentation is a problem under some workloads, and buddyinfo is a
//! > useful tool for helping diagnose these problems.  Buddyinfo will give you a 
//! > clue as to how big an area you can safely allocate, or why a previous
//! > allocation failed.
//! > 
//! > Each column represents the number of pages of a certain order which are 
//! > available.  In this case, there are 0 chunks of 2^0*PAGE_SIZE available in 
//! > ZONE_DMA, 4 chunks of 2^1*PAGE_SIZE in ZONE_DMA, 101 chunks of 2^4*PAGE_SIZE 
//! > available in ZONE_NORMAL, etc... 
//! > 
//! > More information relevant to external fragmentation can be found in pagetypeinfo.
//! >
//! > -- https://www.kernel.org/doc/Documentation/filesystems/proc.txt
//! >
//! >
//! >
//! > /proc/buddyinfo
//! > This file contains information which is used for diagnosing memory fragmentation issues.
//! > Each line starts with the identification of the node and the name of the zone which together identify a memory region 
//! > This is then followed by the count of available chunks of a certain order in which these zones are split. 
//! > The size in bytes of a certain order is given by the formula:
//! > (2^order) * PAGE_SIZE
//! 
//! > The binary buddy allocator algorithm inside the kernel will split one chunk into two chunks of a smaller order 
//! > (thus with half the size) or combine two contiguous chunks into one larger chunk of a higher order 
//! > (thus with double the size) to satisfy allocation requests and to counter memory fragmentation. 
//! > The order matches the column number, when starting to count at zero.
//! > For example on an x86-64 system:
//! > ```text
//! > Node 0, zone     DMA     1    1    1    0    2    1    1    0    1    1    3
//! > Node 0, zone   DMA32    65   47    4   81   52   28   13   10    5    1  404
//! > Node 0, zone  Normal   216   55  189  101   84   38   37   27    5    3  587
//! > ``` 
//! > 
//! > In this example, there is one node containing three zones and there are 11 different chunk sizes. 
//! > If the page size is 4 kilobytes, then the first zone called DMA (on x86 the first 16 megabyte of memory)
//! > has 1 chunk of 4 kilobytes (order 0) available and has 3 chunks of 4 megabytes (order 10) available.
//! > If the memory is heavily fragmented, the counters for higher order chunks will be zero 
//! > and allocation of large contiguous areas will fail.
//! > Further information about the zones can be found in /proc/zoneinfo.
//! >
//! > -- https://manpages.debian.org/testing/manpages/procfs.5.en.html
//! 


define_struct! {
    /// Represent the an entry of /proc/buddyinfo
    /// 
    /// The fields of this struct reference to [`mm/vmstat.c`]
    /// (https://github.com/torvalds/linux/blob/89d57dddd7d319ded00415790a0bb3c954b7e386/mm/vmstat.c#L1356).
    pub struct BuddyInfo {
        node: i32, 
        zone: String, 
        free_areas: [u64;11],
    }
}

use std::str::FromStr;
impl FromStr for BuddyInfo {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<BuddyInfo, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 15 {
            return Err(bfe!("no enough fields to parse a Page".to_string()))
        }
        
        let node = columns[1].trim_end_matches(",").parse::<i32>()?;
        let zone = columns[3].to_string();
        let mut free_areas = [0;11];
        for (fa, v) in free_areas.iter_mut().zip(columns[4..].iter()) {
            *fa = v.parse::<u64>()?;
        }

        Ok(BuddyInfo{
            node, zone, free_areas
        })
    }
}

list_impl! {
    buddyinfo, "/proc/buddyinfo", BuddyInfo, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_page() {
        let source = "Node 0, zone      DMA      7      3     11     11      6      1      4      4      3      0      0";
        let correct = BuddyInfo {
            node: 0, zone: String::from("DMA"), free_areas: [7,3,11,11,6,1,4,4,3,0,0]
        };
        assert_eq!(correct, source.parse::<BuddyInfo>().unwrap());
    }
}