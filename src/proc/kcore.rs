use crate::Result;

/// return the size of /proc/kcore in bytes.
///
/// `kcore()` is not exist in this crate, because it's content is not human readable.
///
/// Note:
/// > This value is given in bytes and is equal to the size of the physical memory (RAM) used plus 4 KB.
/// >
/// > https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/s2-proc-kcore
pub fn kcore_size() -> Result<u64> {
    let md = std::fs::metadata("/proc/kcore")?;
    Ok(md.len())
}
