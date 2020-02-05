// /proc/[pid]/cmdline
// This read-only file holds the complete command line for the
// process, unless the process is a zombie.  In the latter case,
// there is nothing in this file: that is, a read on this file
// will return 0 characters.  The command-line arguments appear
// in this file as a set of strings separated by null bytes
// ('\0'), with a further null byte after the last string.
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    pub struct Cmdline{
        cmd: String,
        args: Vec<String>,
    }
}

use std::str::FromStr;
impl FromStr for Cmdline {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Cmdline, crate::ProcErr> {
        let mut iter = s.split('\0');
        let cmd: String = iter.next().map_or_else(String::new, |s| s.to_string());
        let args: Vec<String> = iter.map(String::from).collect();
        Ok(Cmdline { cmd, args })
    }
}

pid_instance_impl! {
    cmdline_of, "cmdline", Cmdline,
    cmdline_self, cmdline_of_of, cmdline_self_of, cmdline_self_sellf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let source = String::new();
        let correct = Cmdline {
            cmd: "".to_string(),
            args: vec![],
        };
        assert_eq!(correct, source.parse::<Cmdline>().unwrap());

        let source = "abc";
        let correct = Cmdline {
            cmd: "abc".to_string(),
            args: vec![],
        };
        assert_eq!(correct, source.parse::<Cmdline>().unwrap());

        let source = "abc\01\02";
        let correct = Cmdline {
            cmd: "abc".to_string(),
            args: vec!["1".to_string(), "2".to_string()],
        };
        assert_eq!(correct, source.parse::<Cmdline>().unwrap());
    }
}
