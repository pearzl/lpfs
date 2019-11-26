//! > 5.2.14.  /proc/kcore
//! > This file represents the physical memory of the system and is stored in the core file format. 
//! > Unlike most /proc/ files, kcore displays a size. 
//! > This value is given in bytes and is equal to the size of the physical memory (RAM) used plus 4 KB.
//! > The contents of this file are designed to be examined by a debugger, such as gdb, and is not human readable.
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-kcore
//! 

/// Return the size of /proc/kcore in bytes.
pub fn kcore_size() -> Result<u64, crate::ProcErr> {
    let md = std::fs::metadata("/proc/kcore")?;
    Ok(md.len())
}

/// Return physical memory used sizes in bytes.
pub fn kcore_ram() -> Result<u64, crate::ProcErr> {
    Ok(kcore_size()? - 4 * 1024)
}