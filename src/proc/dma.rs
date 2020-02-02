// 
// This is a list of the registered ISA DMA (direct memory access) channels in use.
// 
// -- https://www.unix.com/man-page/suse/5/proc/
// 

define_struct!{
    /// Represent the an entry in /proc/dma, returned by [`dma()`](fn.dma.html).
    /// 
    /// Reference to [`kernel/dma.c`](https://github.com/torvalds/linux/blob/master/kernel/dma.c).
    pub struct Dma {
        channel: u8,
        device: String,
    }
}

use std::str::FromStr;
impl FromStr for Dma {
    type Err = crate::ProcErr;

    fn from_str(value: &str) -> Result<Self, crate::ProcErr> {
        let columns: Vec<&str> = value.split(':').collect();
        if columns.len() != 2 {
            return Err("dma consists of two parts".into());
        }
        Ok(Dma {
            channel: columns[0].trim().parse::<u8>()?,
            device: columns[1].trim().to_string(),
        })
    }
}

list_impl! {
    dma, "/proc/dma", Dma, "\n", 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dma_parse() {
        let source = " 4: cascade";
        let correct = Dma{ channel: 4, device: "cascade".to_string()};
        assert_eq!(correct, source.parse().unwrap());
    }
}