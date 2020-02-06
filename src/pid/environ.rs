// /proc/[pid]/environ
// This file contains the initial environment that was set when
// the currently executing program was started via execve(2).
// The entries are separated by null bytes ('\0'), and there may
// be a null byte at the end.  Thus, to print out the environment
// of process 1, you would do:
//
//     $ cat /proc/1/environ | tr '\000' '\n'
//
// If, after an execve(2), the process modifies its environment
// (e.g., by calling functions such as putenv(3) or modifying the
// environ(7) variable directly), this file will not reflect
// those changes.
//
// Furthermore, a process may change the memory location that
// this file refers via prctl(2) operations such as
// PR_SET_MM_ENV_START.
//
// Permission to access this file is governed by a ptrace access
// mode PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    pub struct Environ(Vec<(String, String)>);
}

impl std::str::FromStr for Environ {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = vec![];
        for entry in s.trim_end_matches('\0').split('\0') {
            let mut kv = entry.splitn(2, '=');
            let key: &str = kv
                .next()
                .ok_or_else(|| crate::ProcErr::from("key not found"))?;
            let value: &str = kv
                .next()
                .ok_or_else(|| crate::ProcErr::from("value not found"))?;
            entries.push((key.to_string(), value.to_string()))
        }
        Ok(Environ(entries))
    }
}

pid_instance_impl! {
    environ_of, "environ", Environ,
    environ_self, environ_of_of, environ_self_of, environ_self_self
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "VERBOSE_LOGGING=true\0_=/bin/cat\0";
        let correct = Environ(vec![
            ("VERBOSE_LOGGING".to_string(), "true".to_string()),
            ("_".to_string(), "/bin/cat".to_string()),
        ]);
        assert_eq!(correct, source.parse::<Environ>().unwrap());
    }
}
