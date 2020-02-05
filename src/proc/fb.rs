//
// 5.2.9.  /proc/fb
// This file contains a list of frame buffer devices, with the frame buffer device number and the driver that controls it. Typical output of /proc/fb for systems which contain frame buffer devices looks similar to the following:
// 0 VESA VGA
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-fb

define_struct! {
    /// represent an entry in /proc/fb
    pub struct Fb {
        device: usize,
        drivers: Vec<String>,
    }
}

use std::str::FromStr;
impl FromStr for Fb {
    type Err = crate::ProcErr;

    fn from_str(value: &str) -> Result<Self, crate::ProcErr> {
        let mut columns = value.split_ascii_whitespace();
        let device = columns
            .next()
            .ok_or_else(|| "device number not found")?
            .parse::<usize>()?;
        let drivers = columns.map(|s: &str| s.to_string()).collect();
        Ok(Fb { device, drivers })
    }
}

list_impl! {
    fb, "/proc/fb", Fb, '\n', 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "0 VESA VGA";
        let correct = Fb {
            device: 0,
            drivers: vec!["VESA".to_string(), "VGA".to_string()],
        };
        assert_eq!(correct, source.parse::<Fb>().unwrap());
    }
}
