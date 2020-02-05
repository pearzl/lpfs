// /proc/version
// This string identifies the kernel version that is currently
// running.  It includes the contents of /proc/sys/kernel/ostype,
// /proc/sys/kernel/osrelease and /proc/sys/kernel/version.  For
// example:
//
// Linux version 1.0.9 (quinlan@phaze) #1 Sat May 14 01:51:54 EDT 1994
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    /// Represent the content of /proc/version
    ///
    /// The fields of this struc reference to
    /// [`scripts/mkcompile_h`](https://github.com/torvalds/linux/blob/master/scripts/mkcompile_h) and
    /// [`init/version.c`](https://github.com/torvalds/linux/blob/master/init/version.c).
    ///
    /// ```text
    /// Linux version ${uts_release} (${linux_compiler_by}@${linux_compiler_host}) (${linux_compiler}) #${uts_version} ${config_flag} ${timestamp}
    /// ```
    pub struct Version {
        uts_release: String,
        linux_compiler_by: String,
        linux_compiler_host: String,
        linux_compiler: String,
        uts_version: String,
        config_flag: String,
        timestamp: String,
    }
}

use std::str::FromStr;
impl FromStr for Version {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Version, crate::ProcErr> {
        if let Some(p1) = s[14..].find('(') {
            let p1 = 14 + p1;
            if let Some(p2) = s[p1 + 1..].find('@') {
                let p2 = p2 + p1 + 1;
                if let Some(p3) = s[p2 + 1..].find(')') {
                    let p3 = p2 + p3 + 1;
                    if let Some(p4) = s[p3 + 3..].rfind(')') {
                        let p4 = p3 + p4 + 3;
                        if let Some(p5) = s[p4 + 2..].find(' ') {
                            let p5 = p4 + p5 + 2;
                            if let Some(p6) = s[p5 + 1..].find(' ') {
                                let p6 = p5 + p6 + 1;
                                let uts_release = s[14..p1 - 1].to_string();
                                let linux_compiler_by = s[p1 + 1..p2].to_string();
                                let linux_compiler_host = s[p2 + 1..p3].to_string();
                                let linux_compiler = s[p3 + 3..p4].to_string();
                                let uts_version = s[p4 + 3..p5].to_string();
                                let config_flag = s[p5 + 1..p6].to_string();
                                let timestamp = s[p6 + 1..].to_string();
                                return Ok(Version {
                                    uts_release,
                                    linux_compiler_by,
                                    linux_compiler_host,
                                    linux_compiler,
                                    uts_version,
                                    config_flag,
                                    timestamp,
                                });
                            }
                        }
                    }
                }
            }
        }

        Err("unknow format".into())
    }
}

instance_impl! {
    version, "/proc/version", Version
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_version() {
        let source = "Linux version 3.10.0-1062.1.1.el7.x86_64 (mockbuild@kbuilder.bsys.centos.org) (gcc version 4.8.5 20150623 (Red Hat 4.8.5-39) (GCC) ) #1 SMP Fri Sep 13 22:55:44 UTC 2019";
        let correct = Version {
            uts_release: String::from("3.10.0-1062.1.1.el7.x86_64"),
            linux_compiler_by: String::from("mockbuild"),
            linux_compiler_host: String::from("kbuilder.bsys.centos.org"),
            linux_compiler: String::from("gcc version 4.8.5 20150623 (Red Hat 4.8.5-39) (GCC) "),
            uts_version: String::from("1"),
            config_flag: String::from("SMP"),
            timestamp: String::from("Fri Sep 13 22:55:44 UTC 2019"),
        };
        assert_eq!(correct, source.parse::<Version>().unwrap())
    }
}
