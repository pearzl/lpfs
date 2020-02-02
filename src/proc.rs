//! Contains the top-level files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!
//! Tese submodules is re-expored.
//!

define_modules! {
    apm;
    cpuinfo;
    stat;
    loadavg;
    kcore;
    uptime;
    buddyinfo;
    pagetypeinfo;
    crypto;
    version;
    cmdline;
    consoles;
    devices;
    dma;
    execdomains;
    fb;
    filesystems;
    interrupts;
    iomem;
    ioports;
    locks;
    mdstat;
    meminfo;
    misc;
    modules;
    mounts;
    mtrr;
    partitions;
}
