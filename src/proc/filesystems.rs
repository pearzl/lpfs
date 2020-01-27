//！> 5.2.10.  /proc/filesystems
//！> This file displays a list of the file system types currently supported by the kernel. Sample output from a generic /proc/filesystems file looks similar to the following:
//！> nodev   sysfs
//！> nodev   rootfs
//！> nodev   bdev
//！> nodev   proc
//！> nodev   sockfs
//！> nodev   binfmt_misc
//！> nodev   usbfs
//！> nodev   usbdevfs
//！> nodev   futexfs
//！> nodev   tmpfs
//！> nodev   pipefs
//！> nodev   eventpollfs
//！> nodev   devpts
//！> 	ext2
//！> nodev   ramfs
//！> nodev   hugetlbfs
//！> 	iso9660
//！> nodev   mqueue
//！> 	ext3
//！> nodev   rpc_pipefs
//！> nodev   autofs
//！> The first column signifies whether the file system is mounted on a block device. Those beginning with nodev are not mounted on a device. The second column lists the names of the file systems supported.
//！> The mount command cycles through the file systems listed here when one is not specified as an argument.
//! >
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-filesystems

define_struct! {
    pub struct FileSystem {
        nodev: bool,
        fs_type: String,
    }
}

use std::str::FromStr;
impl FromStr for FileSystem {
    type Err = crate::ProcErr;

    fn from_str(value: &str) -> Result<Self, crate::ProcErr> {
        let nodev = value.starts_with("nodev");
        let fs_type = if nodev {
            value.trim_start_matches("nodev").trim().to_string()
        }else {
            value.trim().to_string()
        };
        Ok(FileSystem{
            nodev, fs_type
        })
    }
}

list_impl! {
    filesystems, "/proc/filesystems", FileSystem, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "nodev   sysfs";
        let correct = FileSystem {
            nodev: true,
            fs_type: "sysfs".to_string()
        };
        assert_eq!(correct, source.parse().unwrap());

        let source = "sysfs";
        let correct = FileSystem {
            nodev: false,
            fs_type: "sysfs".to_string()
        };
        assert_eq!(correct, source.parse().unwrap());

        let source = "   sysfs";
        let correct = FileSystem {
            nodev: false,
            fs_type: "sysfs".to_string()
        };
        assert_eq!(correct, source.parse().unwrap());
    }
}
