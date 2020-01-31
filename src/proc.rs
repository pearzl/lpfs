//! Contains the top-level files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!
//! Tese submodules is re-expored.
//!

define_modules! {
    apm "apm";
    cpuinfo "cpuinfo";
    stat "stat";
    loadavg "loadavg";
    kcore "kcore";
    uptime "uptime";
    buddyinfo "buddyinfo";
    pagetypeinfo "pagetypeinfo";
    crypto "crypto";
    version "version";
    cmdline "cmdline";
    consoles "consoles";
    devices "devices";
    dma "dma";
    execdomains "execdomains";
    fb "fb";
    filesystems "filesystems";
    interrupts "interrupts";
    iomem "iomem";
    ioports "ioports";
    locks "locks";
    mdstat "mdstat";
    meminfo "meminfo";
    misc "misc";
    modules "modules";
    mounts "mounts";
}
