//! Contains the top-level files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!

pub mod apm;
pub mod cpuinfo;
pub mod stat;
pub mod loadavg;
pub mod kcore;
pub mod uptime;
pub mod buddyinfo;
pub mod pagetypeinfo;
pub mod crypto;
pub mod version;
pub mod cmdline;
pub mod consoles;
pub mod devices;
pub mod dma;
pub mod execdomains;
pub mod fb;
pub mod filesystems;
pub mod interrupts;
pub mod iomem;
pub mod ioports;
pub mod locks;
pub mod mdstat;
pub mod meminfo;
pub mod misc;
pub mod modules;
pub mod mounts;
pub mod mtrr;
pub mod partitions;
pub mod swaps;
