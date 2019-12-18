
define_struct!{
    /// Represent the content of /proc/consoles, returned by [`consoles()`](fn.consoles.html)
    pub struct Consoles{
        consoles: String,
    }
}

use std::str::FromStr;
impl FromStr for Consoles {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Consoles, crate::ProcErr> {
        let consoles = s.trim().to_string();
        Ok(Consoles{consoles})
    }
}


instance_impl!(
    consoles, "/proc/consoles", Consoles
);