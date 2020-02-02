// /proc/mdstat
// 
// > 5.2.18.  /proc/mdstat
// > This file contains the current information for multiple-disk, RAID configurations. If the system does not contain such a configuration, then /proc/mdstat looks similar to the following:
// > Personalities :  read_ahead not set unused devices: <none>
// > This file remains in the same state as seen above unless a software RAID or md device is present. In that case, view /proc/mdstat to find the current status of mdX RAID devices.
// > The /proc/mdstat file below shows a system with its md0 configured as a RAID 1 device, while it is currently re-syncing the disks:
// > Personalities : [linear] [raid1] read_ahead 1024 sectors
// > md0: active raid1 sda2[1] sdb2[0] 9940 blocks [2/2] [UU] resync=1% finish=12.3min algorithm 2 [3/3] [UUU]
// > unused devices: <none>
// 
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-mdstat
// 

define_struct!{
    pub struct MdStat(String);
}

use std::str::FromStr;
impl FromStr for MdStat {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MdStat(s.to_string()))
    }
}

instance_impl!{
    mdstat, "/proc/mdstat", MdStat
}
