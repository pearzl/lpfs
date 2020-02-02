
define_struct!{
    /// Represent the content of /proc/consoles, returned by [`consoles()`](fn.consoles.html)
    pub struct Consoles(String);
}

use std::str::FromStr;
impl FromStr for Consoles {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Consoles, crate::ProcErr> {
        Ok(Consoles(s.to_string()))
    }
}


instance_impl!(
    consoles, "/proc/consoles", Consoles
);