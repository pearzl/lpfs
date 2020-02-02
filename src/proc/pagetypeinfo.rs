// cat /proc/pagetypeinfo
// 
// Page block order: 9
// Pages per block:  512
// Free pages count per migrate type at order       0      1      2      3      4      5      6      7      8      9     10
// Node    0, zone      DMA, type    Unmovable      0      0      0      1      1      1      1      1      1      1      0
// Node    0, zone      DMA, type  Reclaimable      0      0      0      0      0      0      0      0      0      0      0
// Node    0, zone      DMA, type      Movable      1      1      2      1      2      1      1      0      1      0      2
// Node    0, zone      DMA, type      Reserve      0      0      0      0      0      0      0      0      0      1      0
// Node    0, zone      DMA, type      Isolate      0      0      0      0      0      0      0      0      0      0      0
// Node    0, zone    DMA32, type    Unmovable    103     54     77      1      1      1     11      8      7      1      9
// Node    0, zone    DMA32, type  Reclaimable      0      0      2      1      0      0      0      0      1      0      0
// Node    0, zone    DMA32, type      Movable    169    152    113     91     77     54     39     13      6      1    452
// Node    0, zone    DMA32, type      Reserve      1      2      2      2      2      0      1      1      1      1      0
// Node    0, zone    DMA32, type      Isolate      0      0      0      0      0      0      0      0      0      0      0
// Number of blocks type     Unmovable  Reclaimable      Movable      Reserve      Isolate
// Node 0, zone      DMA            2            0            5            1            0
// Node 0, zone    DMA32           41            6          967            2            0
//
// Fragmentation avoidance in the kernel works by grouping pages of different
// migrate types into the same contiguous regions of memory called page blocks.
// A page block is typically the size of the default hugepage size e.g. 2MB on
// X86-64. By keeping pages grouped based on their ability to move, the kernel
// can reclaim pages within a page block to satisfy a high-order allocation.
// The pagetypinfo begins with information on the size of a page block. It
// then gives the same type of information as buddyinfo except broken down
// by migrate-type and finishes with details on how many page blocks of each
// type exist.
// If min_free_kbytes has been tuned correctly (recommendations made by hugeadm
// from libhugetlbfs http://sourceforge.net/projects/libhugetlbfs/), one can
// make an estimate of the likely number of huge pages that can be allocated
// at a given point in time. All the "Movable" blocks should be allocatable
// unless memory has been mlock()'d. Some of the Reclaimable blocks should
// also be allocatable although a lot of filesystem metadata may have to be
// reclaimed to achieve this.
// 
// -- https://android.googlesource.com/kernel/msm/+/android-wear-5.1.1_r0.6/Documentation/filesystems/proc.txt?autodive=0%2F%2F%2F#716

define_struct! {
    /// Represent the content of /proc/pagetypeinfo, returned by [`pagetypeinfo()`](fn.pagetypeinfo.html)
    /// 
    /// Reference to [`mm/vmstat.c`](https://github.com/torvalds/linux/blob/master/mm/vmstat.c#L1372)
    /// 
    /// See [`free_pages()`](struct.PageTypeInfo.html#method.free_pages), 
    /// [`blocks_type_number()`](struct.PageTypeInfo.html#method.blocks_type_number) for details.
    pub struct PageTypeInfo {
        page_block_order: i32,
        pages_per_block: u64,
        /// (node, zone, type, counts)
        free_pages: Vec<(i32, String, String, [u64;11])>,
        /// (node, zone, counts)
        blocks_type_number: Vec<(i32, String, [u64;6])>,
    }
}

#[derive(Debug, PartialEq)]
struct FreePage {
    node: i32,
    zone: String,
    migrate: String,
    counts: [u64;11]
}

impl FreePage {
    fn into_tuple(self) -> (i32, String, String, [u64;11]) {
        (self.node, self.zone, self.migrate, self.counts)
    }
}

use std::str::FromStr;
impl FromStr for FreePage {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<FreePage, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 17 {
            return Err("no enough fileds to parse a free page".into())
        }

        let node = columns[1].trim_end_matches(',').parse::<i32>()?;
        let zone = columns[3].trim_end_matches(',').to_string();
        let migrate = columns[5].to_string();
        let mut counts = [0;11];
        for (c, v) in counts.iter_mut().zip(columns[6..].iter()) {
            *c = v.parse::<u64>()?;
        }

        Ok(FreePage{
            node, zone, migrate, counts
        })
    }
}

#[derive(Debug, PartialEq)]
struct BlockTypeNumber {
    node: i32,
    zone: String,
    counts: [u64; 6]
}

impl BlockTypeNumber {
    fn into_tuple(self) -> (i32, String, [u64;6]) {
        (self.node, self.zone, self.counts)
    }
}

impl FromStr for BlockTypeNumber {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<BlockTypeNumber, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 10 {
            return Err("no enough fields to parse blocks type number".into())
        }

        let node = columns[1].trim_end_matches(',').parse::<i32>()?;
        let zone = columns[3].to_string();
        let mut counts = [0;6];
        for (c, v) in counts.iter_mut().zip(columns[4..].iter()) {
            *c = v.parse::<u64>()?;
        }

        Ok(BlockTypeNumber{
            node, zone, counts
        })
    }
}

impl FromStr for PageTypeInfo {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<PageTypeInfo, crate::ProcErr> {
        let blocks: Vec<&str> = s.split("\n\n").collect();
        if blocks.len() != 3 {
            println!("{:?}", blocks);
            return Err("pagetypeinfo should have 3 blocks".into())
        }

        let b1: Vec<&str> = blocks[0].split_ascii_whitespace().collect();
        if b1.len() != 8 {
            return Err("first block of pagetypeinfo should 8 items".into())
        }
        let page_block_order = b1[3].parse::<i32>()?;
        let pages_per_block = b1[7].parse::<u64>()?;

        let mut free_pages = vec![];
        for line in blocks[1].lines().skip(1) {
            let l = line.parse::<FreePage>()?;
            free_pages.push(l.into_tuple());
        }

        let mut blocks_type_number = vec![];
        for line in blocks[2].lines().skip(1) {
            let l = line.parse::<BlockTypeNumber>()?;
            blocks_type_number.push(l.into_tuple())
        }

        Ok(PageTypeInfo{
            page_block_order, pages_per_block,
            free_pages, blocks_type_number
        })
    }
}

instance_impl! {
    pagetypeinfo, "/proc/pagetypeinfo", PageTypeInfo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_freepage() {
        let source = "Node    0, zone      DMA, type    Unmovable      2     10     13      8      4      1      2      2      0      0      0";
        let correct = FreePage {
            node: 0, 
            zone: String::from("DMA"), 
            migrate: String::from("Unmovable"),
            counts: [2,10,13,8,4,1,2,2,0,0,0]
        };
        assert_eq!(correct, source.parse::<FreePage>().unwrap());
    }

    #[test]
    fn test_parse_blocktypenumber() {
        let source = "Node 0, zone      DMA            2            1            5            0            0            0";
        let correct = BlockTypeNumber{
            node: 0,
            zone: String::from("DMA"),
            counts: [2,1,5,0,0,0]
        };
        assert_eq!(correct, source.parse::<BlockTypeNumber>().unwrap());
    }

    #[test]
    fn test_parse_pagetypeinfo() {
        let source = {
"Page block order: 9
Pages per block:  512

Free pages count per migrate type at order       0      1      2      3      4      5      6      7      8      9     10 
Node    0, zone      DMA, type    Unmovable      2     10     13      8      4      1      2      2      0      0      0 
Node    0, zone      DMA, type  Reclaimable      3      1      2      2      1      0      1      1      1      0      0 
Node    0, zone      DMA, type      Movable      2      1      1      0      1      1      0      1      2      0      0 
Node    0, zone      DMA, type      Reserve      0      0      0      0      0      0      0      0      0      0      0 
Node    0, zone      DMA, type          CMA      0      0      0      0      0      0      0      0      0      0      0 
Node    0, zone      DMA, type      Isolate      0      0      0      0      0      0      0      0      0      0      0 
Node    0, zone    DMA32, type    Unmovable    148     77    109    136     20      7      0      0      0      0      0 
Node    0, zone    DMA32, type  Reclaimable     31    135    247    159     31     11      3      0      0      0      0 
Node    0, zone    DMA32, type      Movable    452    198    246   1628    565     17     18     16     11      1      0 
Node    0, zone    DMA32, type      Reserve      0      0      0      0      0      0      0      0      0      0      0 
Node    0, zone    DMA32, type          CMA      0      0      0      0      0      0      0      0      0      0      0 
Node    0, zone    DMA32, type      Isolate      0      0      0      0      0      0      0      0      0      0      0 

Number of blocks type     Unmovable  Reclaimable      Movable      Reserve          CMA      Isolate 
Node 0, zone      DMA            2            1            5            0            0            0 
Node 0, zone    DMA32           74           42          756            0            0            0 "
        };
        let correct = PageTypeInfo {
            page_block_order: 9,
            pages_per_block: 512,
            free_pages: vec![
                (0,   String::from("DMA"),   String::from("Unmovable"), [  2,  10,  13,    8,   4,  1,  2,  2,  0, 0, 0]), 
                (0,   String::from("DMA"), String::from("Reclaimable"), [  3,   1,   2,    2,   1,  0,  1,  1,  1, 0, 0]), 
                (0,   String::from("DMA"),     String::from("Movable"), [  2,   1,   1,    0,   1,  1,  0,  1,  2, 0, 0]), 
                (0,   String::from("DMA"),     String::from("Reserve"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
                (0,   String::from("DMA"),         String::from("CMA"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
                (0,   String::from("DMA"),     String::from("Isolate"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
                (0, String::from("DMA32"),   String::from("Unmovable"), [148,  77, 109,  136,  20,  7,  0,  0,  0, 0, 0]), 
                (0, String::from("DMA32"), String::from("Reclaimable"), [ 31, 135, 247,  159,  31, 11,  3,  0,  0, 0, 0]), 
                (0, String::from("DMA32"),     String::from("Movable"), [452, 198, 246, 1628, 565, 17, 18, 16, 11, 1, 0]), 
                (0, String::from("DMA32"),     String::from("Reserve"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
                (0, String::from("DMA32"),         String::from("CMA"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
                (0, String::from("DMA32"),     String::from("Isolate"), [  0,   0,   0,    0,   0,  0,  0,  0,  0, 0, 0]), 
            ],
            blocks_type_number: vec![
                (0,   String::from("DMA"), [ 2,  1,   5, 0, 0, 0]), 
                (0, String::from("DMA32"), [74, 42, 756, 0, 0, 0]), 
            ]
        };
        assert_eq!(correct, source.parse::<PageTypeInfo>().unwrap());
    }
}