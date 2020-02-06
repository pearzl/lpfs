// /proc/[pid]/maps
// A file containing the currently mapped memory regions and
// their access permissions.  See mmap(2) for some further infor‐
// mation about memory mappings.
//
// Permission to access this file is governed by a ptrace access
// mode PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
//
// The format of the file is:
//
// address           perms offset  dev   inode       pathname
// 00400000-00452000 r-xp 00000000 08:02 173521      /usr/bin/dbus-daemon
// 00651000-00652000 r--p 00051000 08:02 173521      /usr/bin/dbus-daemon
// 00652000-00655000 rw-p 00052000 08:02 173521      /usr/bin/dbus-daemon
// 00e03000-00e24000 rw-p 00000000 00:00 0           [heap]
// 00e24000-011f7000 rw-p 00000000 00:00 0           [heap]
// ...
// 35b1800000-35b1820000 r-xp 00000000 08:02 135522  /usr/lib64/ld-2.15.so
// 35b1a1f000-35b1a20000 r--p 0001f000 08:02 135522  /usr/lib64/ld-2.15.so
// 35b1a20000-35b1a21000 rw-p 00020000 08:02 135522  /usr/lib64/ld-2.15.so
// 35b1a21000-35b1a22000 rw-p 00000000 00:00 0
// 35b1c00000-35b1dac000 r-xp 00000000 08:02 135870  /usr/lib64/libc-2.15.so
// 35b1dac000-35b1fac000 ---p 001ac000 08:02 135870  /usr/lib64/libc-2.15.so
// 35b1fac000-35b1fb0000 r--p 001ac000 08:02 135870  /usr/lib64/libc-2.15.so
// 35b1fb0000-35b1fb2000 rw-p 001b0000 08:02 135870  /usr/lib64/libc-2.15.so
// ...
// f2c6ff8c000-7f2c7078c000 rw-p 00000000 00:00 0    [stack:986]
// ...
// 7fffb2c0d000-7fffb2c2e000 rw-p 00000000 00:00 0   [stack]
// 7fffb2d48000-7fffb2d49000 r-xp 00000000 00:00 0   [vdso]
//
// The address field is the address space in the process that the
// mapping occupies.  The perms field is a set of permissions:
//
//     r = read
//     w = write
//     x = execute
//     s = shared
//     p = private (copy on write)
//
// The offset field is the offset into the file/whatever; dev is
// the device (major:minor); inode is the inode on that device.
// 0 indicates that no inode is associated with the memory
// region, as would be the case with BSS (uninitialized data).
//
// The pathname field will usually be the file that is backing
// the mapping.  For ELF files, you can easily coordinate with
// the offset field by looking at the Offset field in the ELF
// program headers (readelf -l).
//
// There are additional helpful pseudo-paths:
//
//      [stack]
//             The initial process's (also known as the main
//             thread's) stack.
//
//      [stack:<tid>] (from Linux 3.4 to 4.4)
//             A thread's stack (where the <tid> is a thread ID).
//             It corresponds to the /proc/[pid]/task/[tid]/
//             path.  This field was removed in Linux 4.5, since
//             providing this information for a process with
//             large numbers of threads is expensive.
//
//      [vdso] The virtual dynamically linked shared object.  See
//             vdso(7).
//
//      [heap] The process's heap.
//
// If the pathname field is blank, this is an anonymous mapping
// as obtained via mmap(2).  There is no easy way to coordinate
// this back to a process's source, short of running it through
// gdb(1), strace(1), or similar.
//
// pathname is shown unescaped except for newline characters,
// which are replaced with an octal escape sequence.  As a
// result, it is not possible to determine whether the original
// pathname contained a newline character or the literal \e012
// character sequence.
//
// If the mapping is file-backed and the file has been deleted,
// the string " (deleted)" is appended to the pathname.  Note
// that this is ambiguous too.
//
// Under Linux 2.0, there is no field giving pathname.
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    pub struct Maps(Vec<Map>);
}

impl std::str::FromStr for Maps {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];
        for line in s.lines() {
            let map = line.parse::<Map>()?;
            v.push(map);
        }
        Ok(Maps(v))
    }
}

pid_instance_impl! {
    maps_of, "maps", Maps,
    maps_self, maps_of_of, maps_self_of, maps_self_self
}

define_struct! {
    pub struct Map {
        address: (usize, usize),
        perms: &'static str,
        offset: usize,
        dev: (usize, usize),
        inode: usize,
        pathname: PathName,
        deleted: bool,
    }
}

impl Map {
    const PERMS_STR: [&'static str; 16] = [
        "rwxp", "rwx-", "rw-p", "rw--", "r-xp", "r-x-", "r--p", "r---", "-wxp", "-wx-", "-w-p",
        "-w--", "--xp", "--x-", "---p", "----",
    ];

    pub fn perrms_r(&self) -> bool {
        self.perms.as_bytes()[0] == b'-'
    }
    pub fn perrms_w(&self) -> bool {
        self.perms.as_bytes()[1] == b'-'
    }
    pub fn perrms_x(&self) -> bool {
        self.perms.as_bytes()[2] == b'-'
    }
    pub fn perrms_p(&self) -> bool {
        self.perms.as_bytes()[3] == b'-'
    }
}

impl std::str::FromStr for Map {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = s.split_ascii_whitespace();

        let addrs: Vec<&str> = columns
            .next()
            .ok_or_else(|| crate::ProcErr::from("address not found"))?
            .split('-')
            .collect();
        if addrs.len() != 2 {
            return Err("invalid address".into());
        }
        let start = usize::from_str_radix(&addrs[0], 16)?;
        let end = usize::from_str_radix(&addrs[1], 16)?;
        let address = (start, end);

        let perms_str = columns
            .next()
            .ok_or_else(|| crate::ProcErr::from("perms not found"))?;
        let mut perms = "";
        for ps in Map::PERMS_STR.iter() {
            if perms_str == *ps {
                perms = ps;
                break;
            }
        }
        if perms == "" {
            return Err("invalid perms".into());
        }

        let offset_str = columns
            .next()
            .ok_or_else(|| crate::ProcErr::from("offset not found"))?;
        let offset = usize::from_str_radix(offset_str, 16)?;

        let addrs: Vec<&str> = columns
            .next()
            .ok_or_else(|| crate::ProcErr::from("dev not found"))?
            .split(':')
            .collect();
        if addrs.len() != 2 {
            return Err("invalid dev".into());
        }
        let major = usize::from_str_radix(&addrs[0], 16)?;
        let minor = usize::from_str_radix(&addrs[1], 16)?;
        let dev = (major, minor);

        let inode = columns
            .next()
            .ok_or_else(|| crate::ProcErr::from("inode not found"))?
            .parse::<usize>()?;

        let pathname = if let Some(pn) = columns.next() {
            pn.parse::<PathName>()?
        } else {
            PathName::Empty
        };

        let deleted = if let Some(dt) = columns.next() {
            if dt == "(deleted)" {
                true
            } else {
                return Err("unknow field".into());
            }
        } else {
            false
        };

        Ok(Map {
            address,
            perms,
            offset,
            dev,
            inode,
            pathname,
            deleted,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PathName {
    Path(std::path::PathBuf),
    Stack,
    StackTid(u32),
    Vdso,
    Heap,
    Vsyscall,
    Vvar,
    Empty,
}

impl std::str::FromStr for PathName {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "[vvar]" {
            Ok(Self::Vvar)
        } else if s == "[vdso]" {
            Ok(Self::Vdso)
        } else if s == "[vsyscall]" {
            Ok(Self::Vsyscall)
        } else if s == "[heap]" {
            Ok(Self::Heap)
        } else if s == "[stack]" {
            Ok(Self::Stack)
        } else if s.starts_with('/') {
            Ok(Self::Path(s.into()))
        } else if s.starts_with("[stack:") {
            let tid_str = &s[6..s.len() - 1];
            let tid = tid_str.parse::<u32>()?;
            Ok(Self::StackTid(tid))
        } else {
            Err(format!("unknow pathname: {}", s).into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse1() {
        let source = "7f0f79376000-7f0f79377000 rw-p 00000000 00:00 0 ";
        let correct = Map {
            address: (0x7f0f79376000, 0x7f0f79377000),
            perms: "rw-p",
            offset: 0,
            dev: (0, 0),
            inode: 0,
            pathname: PathName::Empty,
            deleted: false,
        };
        assert_eq!(correct, source.parse::<Map>().unwrap());
    }

    #[test]
    fn test_parse2() {
        let source = "7f0f79147000-7f0f79149000 rw-p 001eb000 fc:01 131498                     /lib/x86_64-linux-gnu/libc-2.27.so";
        let correct = Map {
            address: (0x7f0f79147000, 0x7f0f79149000),
            perms: "rw-p",
            offset: 0x001eb000,
            dev: (0xfc, 0x01),
            inode: 131498,
            pathname: PathName::Path("/lib/x86_64-linux-gnu/libc-2.27.so".into()),
            deleted: false,
        };
        assert_eq!(correct, source.parse::<Map>().unwrap());
    }
}
