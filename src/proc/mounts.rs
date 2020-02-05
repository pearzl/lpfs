// 5.2.22.  /proc/mounts
// This file provides a list of all mounts in use by the system:
// rootfs / rootfs rw 0 0
// /proc /proc proc rw,nodiratime 0 0 none
// /dev ramfs rw 0 0
// /dev/mapper/VolGroup00-LogVol00 / ext3 rw 0 0
// none /dev ramfs rw 0 0
// /proc /proc proc rw,nodiratime 0 0
// /sys /sys sysfs rw 0 0
// none /dev/pts devpts rw 0 0
// usbdevfs /proc/bus/usb usbdevfs rw 0 0
// /dev/hda1 /boot ext3 rw 0 0
// none /dev/shm tmpfs rw 0 0
// none /proc/sys/fs/binfmt_misc binfmt_misc rw 0 0
// sunrpc /var/lib/nfs/rpc_pipefs rpc_pipefs rw 0 0
// The output found here is similar to the contents of /etc/mtab, except that /proc/mount is more up-to-date.
// The first column specifies the device that is mounted, the second column reveals the mount point, and the third column tells the file system type, and the fourth column tells you if it is mounted read-only (ro) or read-write (rw). The fifth and sixth columns are dummy values designed to match the format used in /etc/mtab.
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-mounts
//

use std::path::PathBuf;
define_struct! {
    pub struct Mount {
        device: String,
        mount_point: PathBuf,
        fs_type: String,
        mode: String,
        dummy1: String,
        dummy2: String,
    }
}

use std::str::FromStr;
impl FromStr for Mount {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err("require 6 fields to prase a Mount".into());
        }
        let device = columns[0].to_string();
        let mount_point: PathBuf = columns[1].to_string().into();
        let fs_type = columns[2].to_string();
        let mode = columns[3].to_string();
        let dummy1 = columns[4].to_string();
        let dummy2 = columns[5].to_string();
        Ok(Mount {
            device,
            mount_point,
            fs_type,
            mode,
            dummy1,
            dummy2,
        })
    }
}

list_impl! {
    mounts, "/proc/mounts", Mount, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mount() {
        let source =
            "cgroup /sys/fs/cgroup/cpuset cgroup rw,nosuid,nodev,noexec,relatime,cpuset 0 0";
        let correct = Mount {
            device: "cgroup".to_string(),
            mount_point: "/sys/fs/cgroup/cpuset".to_string().into(),
            fs_type: "cgroup".to_string(),
            mode: "rw,nosuid,nodev,noexec,relatime,cpuset".to_string(),
            dummy1: "0".to_string(),
            dummy2: "0".to_string(),
        };
        assert_eq!(correct, source.parse::<Mount>().unwrap());
    }
}
