use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Dev{
    name: String,
    receive: [usize;8],
    transmit: [usize; 8]
}

impl Dev {
    getter_gen! {
        name: String,
        receive: [usize;8],
        transmit: [usize; 8]
    }

    pub fn bytes_rev(&self) -> usize{
        self.receive[0]
    }

    pub fn packets_rev(&self) -> usize {
        self.receive[1]
    }

    pub fn errs_rev(&self) -> usize {
        self.receive[2]
    }

    pub fn drops_rev(&self) -> usize {
        self. receive[3]
    }

    pub fn fifo_rev(&self) -> usize {
        self.receive[4]
    }

    pub fn frame(&self) -> usize {
        self.receive[5]
    }

    pub fn compressed_rev(&self) -> usize {
        self.receive[6]
    }

    pub fn multicast(&self) -> usize {
        self.receive[7]
    }
    
    pub fn bytes_trs(&self) -> usize {
        self.transmit[0]
    }

    pub fn packets_trs(&self) -> usize {
        self.transmit[1]
    }

    pub fn errs_trs(&self) -> usize {
        self.transmit[2]
    }

    pub fn drop_trs(&self) -> usize {
        self.transmit[3]
    }

    pub fn fifo_trs(&self) -> usize {
        self.transmit[4]
    }

    pub fn colls(&self) -> usize {
        self.transmit[5]
    }

    pub fn carrier(&self) -> usize {
        self.transmit[6]
    }

    pub fn compressed_trs(&self) -> usize {
        self.transmit[7]
    }
}

impl FromStr for Dev {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 17 {
            return Err(Error::BadFormat)
        }
        
        let name = columns[0].trim_end_matches(':').to_string();
        
        let mut receive = [0;8];
        for (rev, item) in receive.iter_mut().zip(columns[1..9].iter()) {
            *rev = item.parse::<usize>()?;
        }

        let mut transmit = [0;8];
        for (trs, item) in transmit.iter_mut().zip(columns[1..9].iter()) {
            *trs = item.parse::<usize>()?;
        }

        Ok(Dev{
            name, receive, transmit
        })
    }
}

#[inline(always)]
fn to_dev(line: &str) -> Result<Dev> {
    Dev::from_str(line)
}

default_list! {
    dev, "/proc/net/dev", Dev, to_dev, '\n', 2
}

