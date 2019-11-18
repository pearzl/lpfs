// top-level files
#[doc(inline)]
pub mod buddyinfo;
pub mod cmdline;
pub mod consoles;
pub mod cpuinfo;
pub mod crypto;
pub mod devices;
pub mod diskstats;
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
pub mod partitions;
pub mod slabinfo;
pub mod stat;
pub mod swaps;
pub mod uptime;
pub mod version;

pub mod _self;

// Process Directories

/// functions in this module has two differences compare to the functions reside in other moduels:
///
/// 1. receive an argument of u32, which represent the process pid number.
/// 2. append "_of" suffix to the function.
pub mod pid {

    macro_rules! pid_path {
        ($pid: expr, $fname: expr) => {
            format!("/proc/{}/{}", $pid, $fname)
        };
    }

    pub mod cmdline;
    pub mod cwd;
    pub mod environ;
    pub mod exe;
    pub mod maps;
    pub mod root;
    pub mod stat;
    pub mod statm;
    pub mod status;
    pub mod task;
}

// other subdirectories
pub mod acpi {
    pub mod wakeup;
}
pub mod driver {
    pub mod rtc;
}
pub mod net {
    pub mod arp;
    pub mod dev;
    pub mod dev_mcast;
    pub mod ip_tables_names;
    pub mod netstat;
    pub mod route;
}
