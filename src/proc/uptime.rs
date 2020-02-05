// 5.2.30.  /proc/uptime
// This file contains information detailing how long the system has been on since its last restart.
// The output of /proc/uptime is quite minimal:
// 350735.47 234388.90
// The first number is the total number of seconds the system has been up.
// The second number is how much of that time the machine has spent idle, in seconds.
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-kcore
//

define_struct! {
    /// Represent the content of /proc/uptime, returned by [`uptime()`](fn.uptime.html)
    pub struct Uptime {
        total: f64,
        idle: f64,
    }
}

impl Uptime {
    /// Return the system usage from last restart.
    pub fn usage(&self) -> f64 {
        self.idle / self.total
    }
}

use std::str::FromStr;
impl FromStr for Uptime {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Uptime, crate::ProcErr> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 2 {
            return Err("not uptime format".into());
        }

        let total = columns[0].parse::<f64>()?;
        let idle = columns[1].parse::<f64>()?;

        Ok(Uptime { total, idle })
    }
}

instance_impl! {
    uptime, "/proc/uptime", Uptime
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_uptime() {
        let source = "2935986.36 5816188.13";
        let correct = Uptime {
            total: 2935986.36,
            idle: 5816188.13,
        };
        assert_eq!(source.parse::<Uptime>().unwrap(), correct);
    }
}
