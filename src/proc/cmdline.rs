
define_struct!{
    pub struct Cmdline{
        cmdline: String,
    }
}

use std::str::FromStr;
impl FromStr for Cmdline {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Cmdline, crate::ProcErr> {
        let cmdline = s.trim().to_string();
        Ok(Cmdline{cmdline})
    }
}


instance_impl!(
    cmdline, "/proc/cmdline", Cmdline
);