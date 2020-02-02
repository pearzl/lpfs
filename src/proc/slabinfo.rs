//! > 5.2.26.  /proc/slabinfo
//! > This file gives full information about memory usage on the slab level. Linux kernels greater than version 2.2 use slab pools to manage memory above the page level. Commonly used objects have their own slab pools.
//! > Instead of parsing the highly verbose /proc/slabinfo file manually, the /usr/bin/slabtop program displays kernel slab cache information in real time. This program allows for custom configurations, including column sorting and screen refreshing.
//! > A sample screen shot of /usr/bin/slabtop usually looks like the following example:
//! > Active / Total Objects (% used)    : 133629 / 147300 (90.7%)
//! > Active / Total Slabs (% used)      : 11492 / 11493 (100.0%)
//! > Active / Total Caches (% used)     : 77 / 121 (63.6%)
//! > Active / Total Size (% used)       : 41739.83K / 44081.89K (94.7%)
//! > Minimum / Average / Maximum Object : 0.01K / 0.30K / 128.00K
//! > OBJS   ACTIVE USE      OBJ   SIZE     SLABS OBJ/SLAB CACHE SIZE NAME
//! > 44814  43159  96%    0.62K   7469      6     29876K ext3_inode_cache
//! > 36900  34614  93%    0.05K    492     75      1968K buffer_head
//! > 35213  33124  94%    0.16K   1531     23      6124K dentry_cache
//! > 7364   6463  87%    0.27K    526      14      2104K radix_tree_node
//! > 2585   1781  68%    0.08K     55      47       220K vm_area_struct
//! > 2263   2116  93%    0.12K     73      31       292K size-128
//! > 1904   1125  59%    0.03K     16      119        64K size-32
//! > 1666    768  46%    0.03K     14      119        56K anon_vma
//! > 1512   1482  98%    0.44K    168       9       672K inode_cache
//! > 1464   1040  71%    0.06K     24      61        96K size-64
//! > 1320    820  62%    0.19K     66      20       264K filp
//! > 678    587  86%    0.02K      3      226        12K dm_io
//! > 678    587  86%    0.02K      3      226        12K dm_tio
//! > 576    574  99%    0.47K     72        8       288K proc_inode_cache
//! > 528    514  97%    0.50K     66        8       264K size-512
//! > 492    372  75%    0.09K     12       41        48K bio
//! > 465    314  67%    0.25K     31       15       124K size-256
//! > 452    331  73%    0.02K      2      226         8K biovec-1
//! > 420    420 100%    0.19K     21       20        84K skbuff_head_cache
//! > 305    256  83%    0.06K      5       61        20K biovec-4
//! > 290      4   1%    0.01K      1      290         4K revoke_table
//! > 264    264 100%    4.00K    264        1      1056K size-4096
//! > 260    256  98%    0.19K     13       20        52K biovec-16
//! > 260    256  98%    0.75K     52        5       208K biovec-64
//! > Some of the more commonly used statistics in /proc/slabinfo that are included into /usr/bin/slabtop include:
//! > OBJS — The total number of objects (memory blocks), including those in use (allocated), and some spares not in use.
//! > ACTIVE — The number of objects (memory blocks) that are in use (allocated).
//! > USE — Percentage of total objects that are active. ((ACTIVE/OBJS)(100))
//! > OBJ SIZE — The size of the objects.
//! > SLABS — The total number of slabs.
//! > OBJ/SLAB — The number of objects that fit into a slab.
//! > CACHE SIZE — The cache size of the slab.
//! > NAME — The name of the slab.
//! > For more information on the /usr/bin/slabtop program, refer to the slabtop man page.