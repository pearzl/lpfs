use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Route {
    iface: String,
    destination: u32,
    getway: u32,
    flags: String,
    ref_cnt: usize,
    use_: usize,
    metric: usize,
    mask: u32,
    mtu: usize,
    window: usize,
    irtt: usize,
}

impl Route {
    getter_gen! {
        iface: String,
        destination: u32,
        getway: u32,
        flags: String,
        ref_cnt: usize,
        use_: usize,
        metric: usize,
        mask: u32,
        mtu: usize,
        window: usize,
        irtt: usize
    }
}

impl FromStr for Route {
    type Err = Error;

    fn from_str(s: &str) -> Result<Route> {
        let columns: Vec<&str> = s.split_ascii_whitespace().collect();
        if columns.len() != 11 {
            return Err(Error::BadFormat);
        }

        let iface = columns[0].to_string();
        let destination = u32::from_str_radix(columns[1], 16)?;
        let getway = u32::from_str_radix(columns[2], 16)?;
        let flags = columns[3].to_string();
        let ref_cnt = columns[4].parse::<usize>()?;
        let use_ = columns[5].parse::<usize>()?;
        let metric = columns[6].parse::<usize>()?;
        let mask = u32::from_str_radix(columns[7], 16)?;
        let mtu = columns[8].parse::<usize>()?;
        let window = columns[9].parse::<usize>()?;
        let irtt = columns[10].parse::<usize>()?;

        Ok(Route {
            iface,
            destination,
            getway,
            flags,
            ref_cnt,
            use_,
            metric,
            mask,
            mtu,
            window,
            irtt,
        })
    }
}

#[inline(always)]
fn to_route(line: &str) -> Result<Route> {
    Route::from_str(line)
}

default_list! {
    route, "/proc/net/route", Route, to_route, '\n', 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "eth0    000000E0        00000000        0000    0       0       256     000000F0        0       0       0";
        let r = Route {
            iface: String::from("eth0"),
            destination: 224,
            getway: 0,
            flags: String::from("0000"),
            ref_cnt: 0,
            use_: 0,
            metric: 256,
            mask: 240,
            mtu: 0,
            window: 0,
            irtt: 0,
        };
        assert_eq!(r, source.parse::<Route>().unwrap());
    }
}
