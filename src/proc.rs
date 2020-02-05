//! Contains the top-level files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!

pub mod apm;
pub mod buddyinfo;
pub mod cmdline;
pub mod consoles;
pub mod cpuinfo;
pub mod crypto;
pub mod devices;
pub mod dma;
pub mod execdomains;
pub mod fb;
pub mod filesystems;
pub mod interrupts;
pub mod iomem;
pub mod ioports;
pub mod kcore;
pub mod loadavg;
pub mod locks;
pub mod mdstat;
pub mod meminfo;
pub mod misc;
pub mod modules;
pub mod mounts;
pub mod mtrr;
pub mod pagetypeinfo;
pub mod partitions;
pub mod stat;
pub mod swaps;
pub mod uptime;
pub mod version;
